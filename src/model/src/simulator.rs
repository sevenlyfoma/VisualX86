use crate::mainmemory::MainMemory;
use crate::mainmemory::MAX_SIZE;
use crate::assembler_mc;

use crate::predecoder;
use crate::instruction::Instruction;

use crate::microcode_types;
use crate::microcode_types::*;
use crate::decoder::*;
use crate::register::Register;
use crate::alu::Alu;
use crate::mu::Mu;
use crate::ju::Ju;

use wasm_bindgen::prelude::*;

use serde_json::{Map, Value};
use std::collections::HashMap;

pub const PREFETCH_BUFFER_SIZE: usize = 32;

pub const INSTRUCTION_QUEUE_SIZE: usize = 10;
const INSTRUCTION_QUEUE_REPEAT_VALUE: std::option::Option<(Instruction, u8)> = None;

pub const UOP_BUFFER_SIZE: usize = 10;
const UOP_BUFFER_REPEAT_VALUE: std::option::Option<microcode_types::U_Instruction> = None;


#[derive(Debug)]
#[wasm_bindgen]
/// Simulator struct acts as main interface to simulator logic
pub struct Simulator {
    main_memory: MainMemory,

    prefetch_buffer: [u8; PREFETCH_BUFFER_SIZE],
    byte_rip: usize,

    //instruction_queue: Vec<Instruction>,
    instruction_queue: [Option<(Instruction, u8)>; INSTRUCTION_QUEUE_SIZE],
    instruction_queue_head: usize,
    instruction_queue_tail: usize,
    instruction_queue_size: usize,

    //uop_buffer: Vec<U_Instruction>,
    uop_buffer: [Option<U_Instruction>; UOP_BUFFER_SIZE],
    uop_buffer_head: usize,
    uop_buffer_tail: usize,
    uop_buffer_size: usize,

    register_file: HashMap<Micro_reg, i64>,
    rip: u64,
    rflags: u64,

    alu: Alu,
    mu: Mu,
    ju: Ju,

    //Alu: 0, mu: 1, ju: 2, none: -1
    last_functional_unit_loaded: i64,
    last_functional_unit_executed: i64,

    //0 : Start
    //1 : prefetch buffer filled
    //2 : instruction queue filled
    //3 : uop buffer filled
    //4 : functional unit loaded
    //5 : functional unit executed
    linear_pipeline_stage: i64,
    //Old stages for visualisaitons
    prev_lin_pipe_stage: i64, 
    prev2_lin_pipe_stage: i64,
    just_loaded: bool,
    bytes_loaded: i64,

    jump_happened: bool,


    assembler_error: String,

    //Step failure indicates the program can no longer procede ()
    step_failed: bool,

}
#[wasm_bindgen]
impl Simulator {
    pub fn new() -> Simulator { 

        let mut sim = Simulator {
            main_memory: MainMemory::new(),

            prefetch_buffer: [0; PREFETCH_BUFFER_SIZE],
            byte_rip: 0,

            instruction_queue: [INSTRUCTION_QUEUE_REPEAT_VALUE; INSTRUCTION_QUEUE_SIZE],
            instruction_queue_head: 0,
            instruction_queue_tail: 0,
            instruction_queue_size: 0,

            uop_buffer: [UOP_BUFFER_REPEAT_VALUE; UOP_BUFFER_SIZE],
            uop_buffer_head: 0,
            uop_buffer_tail: 0,
            uop_buffer_size: 0,

            register_file: HashMap::new(),
            rip: 0,
            rflags: 0b10,

            alu: Alu::new(),
            mu: Mu::new(),
            ju: Ju::new(),

            last_functional_unit_loaded: -1,
            last_functional_unit_executed: -1,

            linear_pipeline_stage: -1,
            prev_lin_pipe_stage: -1,
            prev2_lin_pipe_stage: -1,

            just_loaded: false,
            bytes_loaded: 0,
            jump_happened: false,

            assembler_error: "".to_string(),
            step_failed: false,
        };

        sim.init_registers();

        sim
    }

    /// Takes a string containing an assembly programs and 
    /// 1. Attempts to assemble it and if successful
    /// 2. Resets simulator state
    /// 3. Loads the assembled byte code into main memory
    pub fn load_program(&mut self, text: String) -> bool{    
        
        let bytes_res = assembler_mc::assemble_to_machine_code(&text);

        
        if let Ok(bytes) = bytes_res{
            self.assembler_error = "".to_string();

            self.byte_rip = 0;
            self.init_registers();
            self.rip = 0;
            self.rflags = 0b10;

            self.linear_pipeline_stage = 0;
            self.prev_lin_pipe_stage = -1;
            self.prev2_lin_pipe_stage = -1;
            self.just_loaded = true;
            self.bytes_loaded = bytes.len() as i64;
            self.jump_happened = false;
            self.step_failed = false;

            self.main_memory.reset_memory();
            self.main_memory.load_program(bytes);

            self.flush_pipeline();
            return true;
        }
        else {
            let Err(x) = bytes_res else {panic!()};
            self.assembler_error = x.to_string();
        }

        


        return false;
    }

    /// Calls a function for the current pipeline stage and changes the stage accordingly
    pub fn micro_step_linear(&mut self) -> bool{

        self.just_loaded = false;

        self.prev2_lin_pipe_stage = self.prev_lin_pipe_stage;
        self.prev_lin_pipe_stage = self.linear_pipeline_stage;


        let mut success = false;
        match self.linear_pipeline_stage {
            0 => {
                success = self.load_prefetch_buffer();
                if success {self.linear_pipeline_stage += 1;}
            },
            1 => {
                success = self.predecode();
                if success {self.linear_pipeline_stage += 1;}
            },
            2 => {
                success = self.decode();
                if success {self.linear_pipeline_stage += 1;}
            },
            3 => {
                success = self.load_functional_unit();
                if success {self.linear_pipeline_stage += 1;}
            },
            4 => {
                success = self.execute();
                if success {self.linear_pipeline_stage += 1;}
                // if !success {return false};
            },
            5 => {
                success = self.register_writeback();
                if success {
                    //If uop buffer empty and instruction queue isnt, go back there
                    if self.uop_buffer_size == 0 && self.instruction_queue_size != 0{
                        self.linear_pipeline_stage = 2;
                    }
                    //IF uop buffer isnt empty go back to it
                    else if self.uop_buffer_size != 0 {
                        self.linear_pipeline_stage = 3;
                    }
                    //If both are empty go back to the start
                    else if self.instruction_queue_size == 0 {
                        self.linear_pipeline_stage = 0;
                    }
                }
            },
            _ => {success = false;},
        }
        if !success {
            self.step_failed = true;
            return false
        };
        return true;
    }

    /// Calls micro step linear until the rip changes (indicating a full ISA level instruction has happened)
    pub fn step_linear(&mut self) -> bool{

        let current_rip = self.register_file.get(&Micro_reg::Rip).unwrap().clone();

        let mut next_rip = self.register_file.get(&Micro_reg::Rip).unwrap().clone();

        while current_rip == next_rip{
            let success = self.micro_step_linear();
            if !(success) {return false;}
            next_rip = self.register_file.get(&Micro_reg::Rip).unwrap().clone();
        }

        return true;
    }

    //Initilise all register values
    fn init_registers(&mut self) {
        for reg in Micro_reg::VALUES {
            self.register_file.insert(reg, 0);
        }
        self.register_file.insert(Micro_reg::LogicReg(Register::Rsp), 256);
    }
    
    /// Flushes all values in the pipline for after jump scenario
    /// Does not reinitilise registers, memory etc
    fn flush_pipeline(&mut self) {
        self.prefetch_buffer = [0; PREFETCH_BUFFER_SIZE];

        self.instruction_queue = [INSTRUCTION_QUEUE_REPEAT_VALUE; INSTRUCTION_QUEUE_SIZE];
        self.instruction_queue_head = 0;
        self.instruction_queue_tail = 0;
        self.instruction_queue_size = 0;

        self.uop_buffer = [UOP_BUFFER_REPEAT_VALUE; UOP_BUFFER_SIZE];
        self.uop_buffer_head = 0;
        self.uop_buffer_tail = 0;
        self.uop_buffer_size = 0;

        self.alu = Alu::new();
        self.mu = Mu::new();
        self.ju = Ju::new();

        self.last_functional_unit_loaded = -1;
    }
    

    
    /// Fills the prefetch buffer from memory
    fn load_prefetch_buffer(&mut self) -> bool{
        let end = self.byte_rip+PREFETCH_BUFFER_SIZE;

        if end >= MAX_SIZE{
            return false;
        }

        self.prefetch_buffer = self.main_memory.memory_space[self.byte_rip..end].try_into().expect("Slice wrong");

        return true;
    }

    /// Takes bytes in prefetch buffer, uses predecode method to get list of instructions from them
    fn predecode(&mut self) -> bool{
        let Some(i_queue) = predecoder::predecode(&self.prefetch_buffer) else{
            return false;
        };

        //If more instructions than space in instruction queue then there is an error
        if i_queue.len() > INSTRUCTION_QUEUE_SIZE - self.instruction_queue_size{
            return false;
        }
        //If empty then error, or end of program
        if i_queue.len() == 0 {
            return false;
        }

        //Predeocde returns pairs of instructions and their size in bytes
        //Keep track of this size to increment rip "byte rip"
        let mut total_bytes: usize = 0;

        for (instr,size) in i_queue{
            self.instruction_queue[self.instruction_queue_tail] = Some((instr, size));
            self.instruction_queue_tail = (self.instruction_queue_tail + 1) % INSTRUCTION_QUEUE_SIZE;
            self.instruction_queue_size += 1;
            total_bytes += size as usize;
        }

        self.byte_rip += total_bytes;

        return true;
    }

    /// Takes instruction at head of instruction queue and decoded it into UOPs
    fn decode(&mut self) -> bool {
        let Some(instr) = &self.instruction_queue[self.instruction_queue_head] else{
            return false;
        };
        let Some(mut u_queue) = decode_instruction(instr) else {
            return false;
        };
        //If too many instructions for buffer, then error
        if u_queue.len() > UOP_BUFFER_SIZE - self.uop_buffer_size{
            return false;
        }
        //If empty then error, or end of program
        if u_queue.len() == 0 {
            return false;
        }
        //Update i_q tracker variables
        self.instruction_queue[self.instruction_queue_head] = None;
        self.instruction_queue_head = (self.instruction_queue_head + 1) % INSTRUCTION_QUEUE_SIZE;
        self.instruction_queue_size -= 1;

        for uop in u_queue{
            //Update u_b tracker variables
            self.uop_buffer[self.uop_buffer_tail] = Some(uop);
            self.uop_buffer_tail = (self.uop_buffer_tail + 1) % UOP_BUFFER_SIZE;
            self.uop_buffer_size += 1;
        }

        return true;
    }


    /// Takes a uop from the uop buffer, determines which function unit is responsible for it, and then loads it
    fn load_functional_unit(&mut self) -> bool {


        let Some(uop) = self.uop_buffer[self.uop_buffer_head].clone() else{
            return false;
        };

        //Load flags into the functional units
        self.alu.rflags = self.rflags;
        self.ju.rflags = self.rflags;
        
        match uop.u_operator{
            U_Operator::Load => {
                match uop.u_type {
                    U_Type::Address => {self.last_functional_unit_loaded = 1;},
                    U_Type::Offset => {self.last_functional_unit_loaded = 1;},

                    //loads that dont use memory are done in the alu
                    U_Type::Register => {self.last_functional_unit_loaded = 0;},
                    U_Type::Immediate  => {self.last_functional_unit_loaded = 0;},
                }
            },
            U_Operator::Store => {
                self.last_functional_unit_loaded = 1;
            },
            U_Operator::Add => {self.last_functional_unit_loaded = 0;},
            U_Operator::Sub => {self.last_functional_unit_loaded = 0;},
            U_Operator::Mul => {self.last_functional_unit_loaded = 0;},
            U_Operator::Div => {self.last_functional_unit_loaded = 0;},
            U_Operator::IMul => {self.last_functional_unit_loaded = 0;},
            U_Operator::IDiv => {self.last_functional_unit_loaded = 0;},
            U_Operator::Cmp => {self.last_functional_unit_loaded = 0;},
            U_Operator::And => {self.last_functional_unit_loaded = 0;},
            U_Operator::Or => {self.last_functional_unit_loaded = 0;},
            U_Operator::Xor => {self.last_functional_unit_loaded = 0;},
            U_Operator::Not => {self.last_functional_unit_loaded = 0;},
            U_Operator::Sal => {self.last_functional_unit_loaded = 0;},
            U_Operator::Sar => {self.last_functional_unit_loaded = 0;},
            U_Operator::Shl => {self.last_functional_unit_loaded = 0;},
            U_Operator::Shr => {self.last_functional_unit_loaded = 0;},
            U_Operator::Jmp => {self.last_functional_unit_loaded = 2;},
            U_Operator::Jz => {self.last_functional_unit_loaded = 2;},
            U_Operator::Jc => {self.last_functional_unit_loaded = 2;},
            U_Operator::Jo => {self.last_functional_unit_loaded = 2;},
            U_Operator::Js => {self.last_functional_unit_loaded = 2;},
        }
        
        //Given the choice, load the FU
        match self.last_functional_unit_loaded {
            0 => {self.alu.load_data(uop, &self.register_file);},
            1 => {self.mu.load_data(uop, &self.register_file);},
            2 => {self.ju.load_data(uop, &self.register_file);},
            _ => {return false;},
        }

        //UPdate u_b tracking variables
        self.uop_buffer[self.uop_buffer_head] = None;
        self.uop_buffer_head = (self.uop_buffer_head + 1) % UOP_BUFFER_SIZE;
        self.uop_buffer_size -= 1;


        return true;
    }

    //Executes the last functional unit that was laoded
    fn execute(&mut self) -> bool{

        match self.last_functional_unit_loaded {
            0 => {self.alu.execute_instruction();},
            1 => {self.mu.execute_instruction(&mut self.main_memory);},
            2 => {
                if self.ju.execute_instruction() {
                    self.jump_happened = true;
                }
                else{
                    self.jump_happened = false;
                }
            },
            _ => {return false;},
        };

        self.last_functional_unit_executed = self.last_functional_unit_loaded;

        return true;
                
    }

    /// Given the last functional unit executed, writeback the data to the register file
    fn register_writeback(&mut self) -> bool{

        match self.last_functional_unit_executed {
            0 => {
                // ALU Operation
                if let (Some(output), Some(output_register)) = (self.alu.output,self.alu.output_register) {
                    self.register_file.insert(output_register, output);
                }

                if let (Some(op_out), Some(op_reg)) = (self.alu.output_optional, self.alu.output_register_optional){
                    self.register_file.insert(op_reg, op_out);
                }

                self.rflags = self.alu.rflags;
            },
            1 => {
                // MU operation
                if let (Some(load_output), Some(load_output_register)) = (self.mu.load_output, self.mu.load_output_register){
                    self.register_file.insert(load_output_register, load_output);
                }
            },
            2 => {
                // JU operation
                if self.jump_happened {
                    //If a jump happened, go back to start of pipeline, update rip and flush the pipeline
                    self.jump_happened = false;
                    self.linear_pipeline_stage = 0;     
                    self.register_file.insert(Micro_reg::Rip, self.ju.addr as i64);
                    self.byte_rip = self.ju.addr;
                    self.flush_pipeline();
                }
            },
            _ => {return false;},
        };

        return true;
    }









    /// Function to turn simulator object into consistent json
    /// memory_mode indicates the size of memory items, either bytes(1) or quadwords(8)
    pub fn jsonify(&mut self, memory_mode: usize) -> String{
        
        let mut map = Map::new();

        let main_mem_json: Value;
        match serde_json::from_str(&self.main_memory.jsonify(memory_mode)){
            Ok(x) => main_mem_json = x,
            Err(_) => main_mem_json = Value::Null,
        };


        let mut prefetch_map = Map::new();
        let str_prefetch_list: Vec<String>;
        str_prefetch_list = self.prefetch_buffer.iter().map(|x|  format!("{:01$X}", x, 2)).collect();
        prefetch_map.insert("contents".to_string(), str_prefetch_list.into());
        prefetch_map.insert("byte_rip".to_string(), self.byte_rip.into());
        let prefetch_json = Value::Object(prefetch_map);

        let mut iq_map = Map::new();
        let str_iq_list: Vec<String>;
        str_iq_list = self.instruction_queue.iter().map(|x|  {
            match x {
                Some(val) => format!("{:?}", val),
                None => "".to_string().into(),
            }
        }).collect();
        iq_map.insert("contents".to_string(), str_iq_list.into());
        iq_map.insert("iq_head".to_string(), self.instruction_queue_head.into());
        iq_map.insert("iq_tail".to_string(), self.instruction_queue_tail.into());
        iq_map.insert("iq_size".to_string(), self.instruction_queue_size.into());
        let iq_json = Value::Object(iq_map);

        let mut ub_map = Map::new();
        let str_ub_list: Vec<String>;
        //str_ub_list = self.uop_buffer.iter().map(|x|  format!("{:?}", x)).collect();
        str_ub_list = self.uop_buffer.iter().map(|x|  {
            match x {
                Some(val) => format!("{:?}", val),
                None => "".to_string().into(),
            }
        }).collect();
        ub_map.insert("contents".to_string(), str_ub_list.into());
        ub_map.insert("ub_head".to_string(), self.uop_buffer_head.into());
        ub_map.insert("ub_tail".to_string(), self.uop_buffer_tail.into());
        ub_map.insert("ub_size".to_string(), self.uop_buffer_size.into());
        let ub_json = Value::Object(ub_map);

        let alu_json: Value;
        match serde_json::from_str(&self.alu.jsonify()){
            Ok(x) => alu_json = x,
            Err(_) => alu_json = Value::Null,
        };

        let mu_json: Value;
        match serde_json::from_str(&self.mu.jsonify()){
            Ok(x) => mu_json = x,
            Err(_) => mu_json = Value::Null,
        };

        let ju_json: Value;
        match serde_json::from_str(&self.ju.jsonify()){
            Ok(x) => ju_json = x,
            Err(_) => ju_json = Value::Null,
        };

        let mut register_file_map = Map::new();
        for i in Micro_reg::VALUES {
            
            let val = self.register_file.get(&i);
            match val {
                Some(x) => register_file_map.insert(i.to_string(), x.to_string().into()),
                None => register_file_map.insert(i.to_string(), "None".into()),
            };
        }
        
        let register_file_json = Value::Object(register_file_map);

        map.insert("main_memory".to_string(), main_mem_json);
        map.insert("prefetch_buffer".to_string(), prefetch_json);
        map.insert("instruction_queue".to_string(), iq_json);
        map.insert("uop_buffer".to_string(), ub_json);
        map.insert("alu".to_string(), alu_json);
        map.insert("mu".to_string(), mu_json);
        map.insert("ju".to_string(), ju_json);
        map.insert("register_file".to_string(), register_file_json);

        map.insert("rip".to_string(), self.rip.into());
        let strflags = format!("{:016b}", self.rflags);
        map.insert("rflags".to_string(), strflags.to_string().into());

        map.insert("linear_pipeline_stage".to_string(), self.linear_pipeline_stage.into());
        map.insert("previous_linear_pipeline_stage".to_string(), self.prev_lin_pipe_stage.into());
        map.insert("previous2_linear_pipeline_stage".to_string(), self.prev2_lin_pipe_stage.into());
        map.insert("last_functional_unit_loaded".to_string(), self.last_functional_unit_loaded.into());
        map.insert("last_functional_unit_executed".to_string(), self.last_functional_unit_executed.into());
        map.insert("just_loaded".to_string(), self.just_loaded.into());
        map.insert("jump_happened".to_string(), self.jump_happened.into());
        map.insert("bytes_loaded".to_string(), self.bytes_loaded.into());
        map.insert("asssmbler_error".to_string(), self.assembler_error.clone().into());
        map.insert("step_failed".to_string(), self.step_failed.into());


        let obj = Value::Object(map);

        return obj.to_string();
       
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::Operator;
    use crate::operator::Operator::*;
    use crate::register::Register;
    use crate::register::Register::*;
    use crate::operand::Operand;
    use crate::operand::Operand::*;

    #[test]
    fn test_simulator_loads_prefetch_buffer() {
        let mut sim = Simulator::new();

        sim.load_program("total: addq $4, %rax\naddq $4, %rbx\naddq $4, %rcx\njmp total".to_string());

        let array: [u8; 32] = [0x06 , 0x01 , 0x00 , 0x04 , 0x10 , 0x00 , 0x06 , 0x01 , 
                            0x00 , 0x04 , 0x10 , 0x01 , 0x06 , 0x01 , 0x00 , 0x04 , 
                            0x10 , 0x02 , 0x07 , 0x10 , 0x42 , 0xEE , 0xFF , 0xFF , 
                            0xFF , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00];                     
        
        sim.load_prefetch_buffer();

        assert_eq!(array, sim.prefetch_buffer);



    }


    #[test]
    fn test_simulator_full() {
        let mut sim = Simulator::new();

        sim.load_program("total: addq $4, %rax\naddq $4, %rbx\naddq $4, %rcx\njmp total".to_string());

        let array: [u8; 32] = [0x06 , 0x01 , 0x00 , 0x04 , 0x10 , 0x00 , 0x06 , 0x01 , 
                            0x00 , 0x04 , 0x10 , 0x01 , 0x06 , 0x01 , 0x00 , 0x04 , 
                            0x10 , 0x02 , 0x07 , 0x10 , 0x42 , 0xEE , 0xFF , 0xFF , 
                            0xFF , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00];                     
        
        sim.load_prefetch_buffer();
        assert_eq!(array, sim.prefetch_buffer);

        let mut instruction_queue_expected = [INSTRUCTION_QUEUE_REPEAT_VALUE; INSTRUCTION_QUEUE_SIZE];

        instruction_queue_expected[0] = Some((Instruction::new(
            Operator::Add, 
            vec![Operand::Num(4), Operand::Reg(Register::Rax)],
        ), 6));
        instruction_queue_expected[1] =Some((Instruction::new(
            Operator::Add, 
            vec![Operand::Num(4), Operand::Reg(Register::Rbx)],
        ), 6));
        instruction_queue_expected[2] =Some((Instruction::new(
            Operator::Add, 
            vec![Operand::Num(4), Operand::Reg(Register::Rcx)],
        ), 6));
        instruction_queue_expected[3] =Some((Instruction::new(
            Operator::Jmp, 
            vec![Operand::Num(-18)],
        ), 7));

        sim.predecode();
        assert_eq!(instruction_queue_expected, sim.instruction_queue);


        let mut uop_buffer_expected = [UOP_BUFFER_REPEAT_VALUE; UOP_BUFFER_SIZE];
        uop_buffer_expected[0] = Some(U_Instruction::new(
            U_Type::Immediate,
            U_Operator::Add, 
            vec![U_Operand::Reg(Micro_reg::LogicReg(Rax)), U_Operand::Num(4)],
        ));
        uop_buffer_expected[1] = Some(U_Instruction::new(
            U_Type::Immediate,
            U_Operator::Add, 
            vec![U_Operand::Reg(Micro_reg::Rip), U_Operand::Num(6)],
        ));

        sim.decode();
        assert_eq!(uop_buffer_expected, sim.uop_buffer);


        sim.load_functional_unit();


        assert_eq!(Some(0), sim.alu.input1);
        assert_eq!(Some(4), sim.alu.input2);
        assert_eq!(Some(Micro_reg::LogicReg(Rax)), sim.alu.input_destination_register);
        assert_eq!(Some(U_Operator::Add), sim.alu.operator);


        sim.alu.execute_instruction();

        assert_eq!(Some(4), sim.alu.output);





    }

    fn test_simulator_mult(){
        let mut sim = Simulator::new();

        sim.load_program("addq $4, %rax\nmulq $4".to_string());              
        
        sim.load_prefetch_buffer();

        sim.predecode();

        sim.decode();

        sim.load_functional_unit();

        sim.alu.execute_instruction();

        assert_eq!(Some(4), sim.alu.output);

        sim.register_writeback();

        assert_eq!(Some(4), sim.register_file.get(&Micro_reg::LogicReg(Rax)).copied());

        sim.alu.execute_instruction();

        assert_eq!(Some(16), sim.alu.output);

        sim.register_writeback();

        assert_eq!(Some(16), sim.register_file.get(&Micro_reg::LogicReg(Rax)).copied());


    }

    fn test_simulator_add_mem(){
        let mut sim = Simulator::new();

        sim.load_program("addq $4, 0x80".to_string());              
        
        sim.load_prefetch_buffer();

        sim.predecode();

        sim.decode();

        sim.load_functional_unit();

        sim.execute();

        sim.register_writeback();

        assert_eq!(Some(4), sim.main_memory.get_qword(128));

    }

}
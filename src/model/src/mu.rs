use crate::microcode_types::*;

use crate::microcode_types::Micro_reg::*;

use crate::register::Register;
use crate::mainmemory::MainMemory;

use serde_json::{Value, Map};
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Mu {
    //Inputs
    /// Potential register to be loaded into, if operator is load
    pub load_input_register: Option<Micro_reg>,
    /// Potential value to be stored, if operator is store
    pub store_input: Option<i64>,
    /// operator to execute
    pub operator: Option<U_Operator>,
    /// address operation will access
    pub address_to_access: Option<usize>,

    // Outputs
    /// Potential value gotten from a load operation
    pub load_output: Option<i64>,
    /// Potential register to store the result of a load operation into
    pub load_output_register: Option<Micro_reg>,
    

   

   


}

impl Mu {
    pub fn new() -> Mu {
        let mut mu = Mu {
            load_input_register: None,
            store_input: None,
            address_to_access: None,
            operator: None,

            load_output: None,
            load_output_register: None,
        };

        mu
    }

    /// Takes a uop and a borrow of the register file and loads the input fields of the mu
    pub fn load_data(&mut self, uop: U_Instruction, register_file: &HashMap<Micro_reg, i64>) -> bool{
        self.reset_inputs();
        //Gets the operands of the uop
        let (Some(operand1),Some(operand2))  = (uop.u_operands.get(0),uop.u_operands.get(1)) else{
            return false;
        };

        // the first operand should always be a register, no memory-memory
        let U_Operand::Reg(reg) = operand1 else{
            return false;
        };

        //Get the address to access from the other operand
        match operand2 {
            U_Operand::Addr(addr) => {self.address_to_access = Some(*addr as usize)},
            //Only allow indirect without offset, offset calculated in alu.
            U_Operand::OffsetReg(reg2, 0) => {
                let Some(addr) = register_file.get(&reg2) else{
                    return false;
                };
                self.address_to_access = Some(*addr as usize); 
            }
            _ => {return false;}
        }

        self.load_input_register = Some(*reg);
        self.operator = Some(uop.u_operator);
        let Some(num) = register_file.get(&reg) else{
            return false;
        };
        self.store_input = Some(*num);
        
        return true;
    }

    /// Takes a borrow of the main memory and executes the memory operation
    pub fn execute_instruction(&mut self, main_memory: &mut MainMemory) -> bool{
        self.reset_outputs();

        let Some(operator) = self.operator else{
            return false;
        };

        match operator {
            U_Operator::Load => {
                //Check input values are not none
                let (Some(reg), Some(addr)) = (self.load_input_register, self.address_to_access) else{
                    return false;
                };
                self.load_output_register = Some(reg);

                //Get value from memory
                let Some(val) = main_memory.get_qword(addr) else{
                    return false;
                };

                //Store in output field
                self.load_output = Some(val);
            },
            U_Operator::Store => {
                let (Some(val), Some(addr)) = (self.store_input, self.address_to_access) else{
                    return false;
                };
                //Set value in memory
                return main_memory.set_qword(val, addr);
            },
            _ => {return false;}
        };

        return true;
    }


    /// Sets all input fields to none
    pub fn reset_inputs(&mut self){
        self.load_input_register = None;
        self.store_input = None;
        self.address_to_access = None;
        self.operator = None;
    }

    /// Sets all output fields to none
    pub fn reset_outputs(&mut self){
        self.load_output = None;
        self.load_output_register = None;
    }

    /// Turns object into consistent json
    pub fn jsonify(&mut self) -> String{
        let mut mu_map = Map::new();
        mu_map.insert("load_output".to_string(),  match self.load_output {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });
        mu_map.insert("load_output_register".to_string(),  match self.load_output_register {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });
        mu_map.insert("load_input_register".to_string(),  match self.load_output_register {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });
        mu_map.insert("store_input".to_string(),  match self.store_input {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });
        mu_map.insert("address_to_access".to_string(),  match self.address_to_access {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });
        mu_map.insert("operator".to_string(),  match self.operator {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });

        let mu_json = Value::Object(mu_map);

        return mu_json.to_string();

            
    }    
    
}
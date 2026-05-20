use crate::microcode_types::*;

use crate::microcode_types::Micro_reg::*;

use crate::register::Register;

use std::collections::HashMap;
use serde_json::{Value, Map};

#[derive(Debug, PartialEq)]
pub struct Alu {
    pub input1: Option<i64>,
    pub input2: Option<i64>,
    pub output: Option<i64>,

    pub input_destination_register: Option<Micro_reg>,
    
    pub output_optional: Option<i64>,

    pub rflags: u64,

    pub output_register: Option<Micro_reg>,
    pub output_register_optional: Option<Micro_reg>,

    pub operator: Option<U_Operator>,

    //Flags should only be updated when the "real" registers are updated, not when virtual registers are
    //This allows for rip updates and offset addressing to not overwrite flags
    pub should_update_flags: bool,

}

impl Alu {
    pub fn new() -> Alu {
        let mut alu = Alu {
            input1:None,
            input2:None,
            output:None,
            input_destination_register: None,
            output_optional:None,
            rflags: 0b10,
            output_register: None,
            output_register_optional: None,
            operator: None,
            should_update_flags: true,
        };

        alu
    }

    /// Takes a position value and a value and sets the flag at that position to that value
    pub fn alter_flags(&mut self, position: u32, value: u64){
        let base: u64 = 2;
        //Changes position of change to a value to make a mask
        //So position 3 -> 2^3 -> 8 ->1000 (0 indexed)
        let mut location_to_value: u64 = base.pow(position);

        if value == 1 {
            //We are setting a bit
            self.rflags |= location_to_value;
        }
        else if value == 0 {
            //we are nulling a bit
            //So flip the mask
            location_to_value = !location_to_value;

            self.rflags &= location_to_value;

        }
    } 

    /// Takes a uop and a borrow of the register file and loads the input fields of the alu
    pub fn load_data(&mut self, uop: U_Instruction, register_file: &HashMap<Micro_reg, i64>) -> bool {
        self.reset_inputs();

        
        //There must be at least the first operand
        let Some(operand1) = uop.u_operands.get(0) else{
            return false;
        };

        match operand1{
            U_Operand::Num(x) => {self.input1 = Some(*x);},
            // Address needs to go to MU
            U_Operand::Addr(_) => {return false;},
            U_Operand::Reg(x) => {
                let Some(num) = register_file.get(&x) else{
                    return false
                };
                self.input1 = Some(*num);
                self.input_destination_register = Some(*x);
            }
            U_Operand::OffsetReg(_, _) => {return false;},
        };

        //There may or may not be a second operand
        if let Some(operand2) = uop.u_operands.get(1) {
            match operand2{
                U_Operand::Num(x) => {self.input2 = Some(*x);},
                U_Operand::Addr(_) => {return false;},
                U_Operand::Reg(x) => {
                    let Some(num) = register_file.get(&x) else{
                        return false
                    };
                    self.input2= Some(*num);
                }
                U_Operand::OffsetReg(_, _) => {return false;},
            }
        }

        self.operator = Some(uop.u_operator);

        // These operations only take one input and act on the rax
        let mut rax_change = || {
            self.input_destination_register = Some(Micro_reg::LogicReg(Register::Rax));
            self.input2 = register_file.get(&Micro_reg::LogicReg(Register::Rax)).copied();
        };

        match uop.u_operator{
            U_Operator::Mul => {
                rax_change();
            },
            U_Operator::Div => {
                rax_change();
            },
            U_Operator::IMul => {
                rax_change();
            },
            U_Operator::IDiv => {
                rax_change();
            },
            _ => {}
        };

        //If a logical "R" register, then flags should be updated
        if let Some(Micro_reg::LogicReg(_)) = self.input_destination_register {
            self.should_update_flags = true;
        }
        //"U" registers shouldnt cause flag updates
        else {
            self.should_update_flags = false;
        }
        return true;
    }

    /// Resets all input fields
    pub fn reset_inputs(&mut self){
        self.input1 = None;
        self.input2 = None;
        self.operator = None;
        self.input_destination_register = None;
    }

    /// Resets all output field
    pub fn reset_outputs(&mut self){
        self.output = None;
        self.output_optional = None;
        self.output_register = None;
        self.output_register_optional = None;
    }

    /// Calls execution fucntions for each instruciton
    pub fn execute_instruction(&mut self) -> bool{
        self.reset_outputs();
        self.output_register = self.input_destination_register;
        match self.operator {
            Some(U_Operator::Load) => self.call_load(),
            Some(U_Operator::Add) => self.call_add(),
            Some(U_Operator::Sub) => self.call_sub(),
            Some(U_Operator::Mul) => self.call_mul(),
            Some(U_Operator::IMul) => self.call_imul(),
            Some(U_Operator::Div) => self.call_div(),
            Some(U_Operator::IDiv) => self.call_idiv(),
            Some(U_Operator::Cmp) => self.call_cmp(),
            Some(U_Operator::And) => self.call_and(),
            Some(U_Operator::Or) => self.call_or(),
            Some(U_Operator::Xor) => self.call_xor(),
            Some(U_Operator::Not) => self.call_not(),
            Some(U_Operator::Sal) => self.call_sal(),
            Some(U_Operator::Shl) => self.call_shl(),
            Some(U_Operator::Sar) => self.call_sar(),
            Some(U_Operator::Shr) => self.call_shr(),
            _ => false,
        }
    }

    /// Sets output to second input
    pub fn call_load(&mut self) -> bool{
        if let (Some(_), Some(v2)) = ( self.input1,  self.input2){
            self.output = Some(v2);
            return true;
        }
        return false;
    }


    /// Emulates an add and updates flags as needed
    pub fn call_add(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){

            // This does a signed and unsigned operation
            // Gets overflow of both, different for each potentially
            // Uses overflow to set flags
            let (res, overflow) = v1.overflowing_add(v2);
            let (_, carry) = (v1 as u64).overflowing_add(v2 as u64);   
            
            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }

            //Set 0 bit if result 0, otherwise clear 0 bit
            if res == 0{ self.alter_flags(6,1); }
            else{ self.alter_flags(6,0); }

            //Set sign bit based on positive/negative
            if res >=0{ self.alter_flags(7,0);}
            else{ self.alter_flags(7,1); }

            //Set carry flag
            if carry{ self.alter_flags(0,1); }
            else{ self.alter_flags(0,0); }

            //Set overflow flag
            if overflow{ self.alter_flags(11,1); }
            else{ self.alter_flags(11,0); }

            return true;
        }


        return false;        
    }

    /// Emulates a sub and updates flags as needed
    pub fn call_sub(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            // This does a signed and unsigned operation
            // Gets overflow of both, different for each potentially
            // Uses overflow to set flags
            let (res, overflow) = v2.overflowing_sub(v1);
            let (_, carry) = (v2 as u64).overflowing_sub(v1 as u64);     
            
            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }
            
            //Set 0 bit if result 0, otherwise clear 0 bit
            if res == 0{ self.alter_flags(6,1); }
            else{ self.alter_flags(6,0); }

            //Set sign bit based on positive/negative
            if res >=0{ self.alter_flags(7,0);}
            else{ self.alter_flags(7,1); }

            //Set carry flag
            if carry{ self.alter_flags(0,1); }
            else{ self.alter_flags(0,0); }

            //Set overflow flag
            if overflow{ self.alter_flags(11,1); }
            else{ self.alter_flags(11,0); }

            return true;
        }


        return false;        
    }


    /// Unsigned multiplication
    pub fn call_mul(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            //Truncate to lower 32 bits, as mul can only take 32 bit registers s
            let v1_trunc = ((v1 as u64) as u32) as u64;
            let v2_trunc = ((v2 as u64) as u32) as u64;

            let res = v1_trunc * v2_trunc;

            self.output = Some(res as i64);

            if !self.should_update_flags {
                return true;
            }

            //If the result is larger than maximum u32
            //This means top half of result is non zero
            //So set CF and OF
            if res > 4294967295{
                self.alter_flags(0,1);
                self.alter_flags(11,1);
            }
            else{ 
                self.alter_flags(0,0);
                self.alter_flags(11,0);
            }

            return true;
            
        
        }

        return false;
        
    }
    /// Emulates signed multiplacte
    pub fn call_imul(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            // Truncate both to emulate usign eax
            let v1_trunc = (v1 as i32) as i64;
            let v2_trunc = (v2 as i32) as i64;

            let res = v1_trunc * v2_trunc;

            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }

            //If the top 32 bits are not the sign extension of the lower 32
            //Set the flags
            //sign extension is either 1....1 = -1 or 0...0 = 0
            let top_half = (res >> 32) as i32;

            if (top_half == -1 && res < 0) || (top_half == 0 && res >= 0){
                self.alter_flags(0,0);
                self.alter_flags(11,0); 
            }
            else{ 
                self.alter_flags(0,1);
                self.alter_flags(11,1);
            }

        
            return true;
                
        }

        return false;
    }

    /// Emulates unsigned division
    pub fn call_div(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            let v1_trunc = ((v1 as u64) as u32) as u64;
            let v2_trunc = ((v2 as u64) as u32) as u64;

            let quotient = v2_trunc / v1_trunc;
            let remainder = v2_trunc % v1_trunc;

            self.output = Some(quotient as i64);

            //Second output fields are for the remainder of the integer division
            self.output_optional = Some(remainder as i64);
            self.output_register_optional = Some(Micro_reg::LogicReg(Register::Rdx));
            
            return true;               
        
        }

        return false;
        
    }

    /// Emulates signed division
    pub fn call_idiv(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            let v1_trunc = (v1 as i32) as i64;
            let v2_trunc = (v2 as i32) as i64;

            let quotient = v2_trunc / v1_trunc;
            let remainder = v2_trunc % v1_trunc;

            self.output = Some(quotient as i64);

            //Second output fields are for the remainder of the interger division
            self.output_optional = Some(remainder as i64);
            self.output_register_optional = Some(Micro_reg::LogicReg(Register::Rdx));

            return true;               
        
        }
        return false;
        
    }

    /// Acts the same as call_cmp without setting the output
    pub fn call_cmp(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            let (res, overflow) = v2.overflowing_sub(v1);

            let (_, carry) = (v2 as u64).overflowing_sub(v1 as u64);          
            
            if !self.should_update_flags {
                return true;
            }
            //Set 0 bit if result 0, otherwise clear 0 bit
            if res == 0{ self.alter_flags(6,1); }
            else{ self.alter_flags(6,0); }

            //Set sign bit based on positive/negative
            if res >=0{ self.alter_flags(7,0);}
            else{ self.alter_flags(7,1); }

            //Set carry flag
            if carry{ self.alter_flags(0,1); }
            else{ self.alter_flags(0,0); }

            //Set overflow flag
            if overflow{ self.alter_flags(11,1); }
            else{ self.alter_flags(11,0); }

            return true;
        }


        return false;        
    }

    /// Emulates bitwise and
    pub fn call_and(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            let res = v1 & v2;
            
            self.output = Some(res);
            
            return true;
            
        }


        return false;        
    }

    /// Emulates bitwise or
    pub fn call_or(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            let res = v1 | v2;
            
            self.output = Some(res);
            
            return true;
        }


        return false;        
    }

    /// Emulates bitwise xor
    pub fn call_xor(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            let res = v1 ^ v2;
            
            self.output = Some(res);
            
            return true;
        }


        return false;        
    }

    /// Emulates bitwise not
    pub fn call_not(&mut self) -> bool{
        if let Some(v1) = self.input1{
            
            let res = !v1;
            
            self.output = Some(res);
            
            return true;
        }


        return false;        
    }

    /// Emulates logical shift left
    pub fn call_shl(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            
            let res = v1 << v2;

            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }

            if (v1 >= 0 && res >= 0) || (v1 < 0 && res < 0){
                self.alter_flags(11,0);
            }
            return true;
        }
        return false;        
    }

    /// Emulates arithmetic shift left, identical to logical
    pub fn call_sal(&mut self) -> bool{
        self.call_shl()
    }

    /// Emulates logical shift right
    pub fn call_shr(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            //logical on i64
            let res = v1 >> v2;

            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }
            
            //If sign bit remains the same, then OF is cleared

            if (v1 >= 0 && res >= 0) || (v1 < 0 && res < 0){
                self.alter_flags(11,0);
            }
            
            return true;
        }
        return false;        
    }

    /// Emulates  arithmetic shift right
    pub fn call_sar(&mut self) -> bool{
        if let (Some(v1), Some(v2)) = ( self.input1,  self.input2){
            //arithmetic on u64
            let res = ((v1 as u64) >> v2) as i64;

            self.output = Some(res);

            if !self.should_update_flags {
                return true;
            }
            
            //If sign bit remains the same, then OF is cleared

            if (v1 >= 0 && res >= 0) || (v1 < 0 && res < 0){
                self.alter_flags(11,0);
            }
            return true;
        }
        return false;        
    }    

    /// Jsonify function to create consistent json representation
    pub fn jsonify(&mut self) -> String{
        let mut map = Map::new();

        let strflags = format!("{:016b}", self.rflags);

        map.insert("rflags".to_string(), strflags.to_string().into());

        match self.operator{
            None =>  map.insert("operator".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("operator".to_string(), x.to_string().into()),
        };

        match self.input1{
            None =>  map.insert("input1".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("input1".to_string(), x.to_string().into()),
        };

        match self.input2{
            None =>  map.insert("input2".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("input2".to_string(), x.to_string().into()),
        };

        match self.input_destination_register{
            None =>  map.insert("input_destination_register".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("input_destination_register".to_string(), x.to_string().into()),
        };

        match self.output{
            None =>  map.insert("output".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("output".to_string(), x.to_string().into()),
        };

        match self.output_optional{
            None =>  map.insert("output_optional".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("output_optional".to_string(), x.to_string().into()),
        };

        match self.output_register{
            None =>  map.insert("output_register".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("output_register".to_string(), x.to_string().into()),
        };

        match self.output_register_optional{
            None =>  map.insert("output_register_optional".to_string(), "".to_string().into()),
            Some(x) =>  map.insert("output_register_optional".to_string(), x.to_string().into()),
        };


        let obj = Value::Object(map);

        return obj.to_string();

            
    }    
    
}
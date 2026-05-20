use crate::microcode_types::*;

use crate::microcode_types::Micro_reg::*;

use crate::register::Register;
use crate::mainmemory::MainMemory;

use serde_json::{Value, Map};
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Ju {
    pub operator: Option<U_Operator>,
    pub addr: usize,
    pub rflags: u64,

    pub jump_amount: i64,
    pub rip: u64,


}

impl Ju {
    pub fn new() -> Ju {
        let mut ju = Ju {
            operator: None,
            addr: 0,
            rflags: 0b10,

            jump_amount: 0,
            rip: 0,
        };
        ju
    }

    /// Takes a value for the bit position of a flag and returns whether or not it is set
    pub fn get_flag(&mut self, position: u32) -> bool {
        let base: u64 = 2;
        //Changes position of change to a value to make a mask
        //So position 3 -> 2^3 -> 8 ->1000 (0 indexed)
        let location_to_value: u64 = base.pow(position);

        let isolated = self.rflags & location_to_value;

        //If isolated is 0, this means that the one bit we wanted is also 0, if its non zero, that one bit is 1
        return isolated != 0
    }


    /// Takes a uop and a borrow of the register file and loads the input fields of the ju
    pub fn load_data(&mut self, uop: U_Instruction, register_file: &HashMap<Micro_reg, i64>) -> bool{
        let Some(operand1)  = uop.u_operands.get(0) else{
            return false;
        };
        let U_Operand::Num(num) = operand1 else{
            return false;
        };

        self.operator = Some(uop.u_operator);
        self.jump_amount = *num;

        let Some(rip_value) = register_file.get(&Micro_reg::Rip) else{
            return false
        };
        self.rip = *rip_value as u64;


        return true;
    }

    /// Executes the JU, figures out of if the jump should happen
    pub fn execute_instruction(&mut self) -> bool{
        let Some(operator) = self.operator else{
            return false;
        };

        let mut success = false;
        match operator {
            U_Operator::Jmp => {success = true;},
            U_Operator::Jz => {success = self.get_flag(6)},
            U_Operator::Jc => {success = self.get_flag(0)},
            U_Operator::Jo => {success = self.get_flag(11)},
            U_Operator::Js => {success = self.get_flag(7)},
            _ => {}
        }

        //If the jump should happen, update the address to jump to
        if success {
            self.addr = ((self.rip as i64) + self.jump_amount) as usize
        }
        return success;

    }

    /// Function to convert ju into consistent json
    pub fn jsonify(&mut self) -> String{
        let mut ju_map = Map::new();
        ju_map.insert("operator".to_string(),  match self.operator {
            Some(x) => x.to_string().into(),
            None => "".into(),
        });

        let strflags = format!("{:016b}", self.rflags);

        ju_map.insert("rflags_copy".to_string(), strflags.to_string().into());
        ju_map.insert("jump_amount".to_string(), self.jump_amount.into());
        ju_map.insert("rip_copy".to_string(), self.rip.into());
        ju_map.insert("jump_address".to_string(), self.addr.into());

        let ju_json = Value::Object(ju_map);

        return ju_json.to_string();

            
    }    
    
}
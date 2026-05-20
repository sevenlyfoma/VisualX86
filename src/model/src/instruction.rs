use crate::operator::Operator;
use crate::operand::Operand;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub operator: Operator,
    pub operands: Vec<Operand>,
    pub size: usize,
}

impl Instruction {
    pub fn new(operator_ : Operator, operands_: Vec<Operand>) -> Instruction {
        Instruction {
            operator: operator_,
            operands: operands_,
            size: 0,
        }
    }

    pub fn to_string(&mut self) -> String{
        let mut x: String = "".to_string();
        x += &self.operator.to_string();
        x += " ";


        return x;

    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x: String = "".to_string();
        x += &self.operator.to_string();
        x += &self.operands.iter().map(|y| " ".to_string() + &y.to_string()).collect::<String>();
        write!(f, "{}", x.to_string())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


use crate::register::Register;

use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Operand {
    Num(i64),
    Reg(Register),
    Addr(u64),
    Indirect(Register),
    Offset(i64, Register),
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Num(x) => write!(f, "${}", x.to_string()),
            Operand::Reg(x) => write!(f, "%{}", x.to_string()),
            Operand::Addr(x) => write!(f, "0x{:X}", x),
            Operand::Indirect(x) => write!(f, "({:?})", x),
            Operand::Offset(x, y) => write!(f, "{:?}({:?})", x, y) 
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
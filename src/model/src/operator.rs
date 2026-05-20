use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(PartialEq, Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum Operator {
    Mov,

    Add,
    Sub,
    Mul,
    Div,
    IMul,
    IDiv,
    Cmp,

    And,
    Or,
    Xor,
    Not,

    Sal,
    Sar,
    Shl,
    Shr,

    Jmp,
    Jz, //Zero flag
    Jc, //Carry flag
    Jo, //Overflow flag
    Js, //Sign flag
}

pub fn lookup_opcode(code: u8) ->  Option<Operator>{
    match code {
        0  => Some(Operator::Mov),
        1  => Some(Operator::Add),
        2  => Some(Operator::Sub),
        3  => Some(Operator::Mul),
        4  => Some(Operator::Div),
        5  => Some(Operator::IMul),
        6  => Some(Operator::IDiv),
        7  => Some(Operator::Cmp),
        8  => Some(Operator::And),
        9  => Some(Operator::Or),
        10 => Some(Operator::Xor),
        11 => Some(Operator::Not),
        12 => Some(Operator::Sal),
        13 => Some(Operator::Sar),
        14 => Some(Operator::Shl),
        15 => Some(Operator::Shr),
        16 => Some(Operator::Jmp),
        17 => Some(Operator::Jz), 
        18 => Some(Operator::Jc),
        19 => Some(Operator::Jo),
        20 => Some(Operator::Js),
        _ => None

    }
}

pub fn get_num_args(op: &Operator) -> usize{
    match op {
        Operator::Mov => 2,

        Operator::Add => 2,
        Operator::Sub => 2,
        Operator::Mul => 1,
        Operator::Div => 1,
        Operator::IMul => 1,
        Operator::IDiv => 1,
        Operator::Cmp => 2,

        Operator::And => 2,
        Operator::Or => 2,
        Operator::Xor => 2,
        Operator::Not => 1,

        Operator::Sal => 2,
        Operator::Sar => 2,
        Operator::Shl => 2,
        Operator::Shr => 2,

        Operator::Jmp => 1,
        Operator::Jz => 1,
        Operator::Jc => 1,
        Operator::Jo => 1,
        Operator::Js => 1,
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Mov => write!(f, "movq"),
            
            Operator::Add => write!(f, "addq"),
            Operator::Sub => write!(f, "subq"),
            Operator::Mul => write!(f, "mulq"),
            Operator::Div => write!(f, "divq"),
            Operator::IMul => write!(f, "imulq"),
            Operator::IDiv => write!(f, "idivq"),
            Operator::Cmp => write!(f, "cmpq"),

            Operator::And => write!(f, "andq"),
            Operator::Or => write!(f, "orq"),
            Operator::Xor => write!(f, "xorq"),
            Operator::Not => write!(f, "notq"),

            Operator::Sal => write!(f, "salq"),
            Operator::Sar => write!(f, "sarq"),
            Operator::Shl => write!(f, "shlq"),
            Operator::Shr => write!(f, "shrq"),

            Operator::Jmp => write!(f, "jmp"),
            Operator::Jz => write!(f, "jz"),
            Operator::Jc => write!(f, "jc"),
            Operator::Jo => write!(f, "jo"),
            Operator::Js => write!(f, "js"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
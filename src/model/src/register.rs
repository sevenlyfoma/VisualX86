use std::slice::Iter;

use serde::{Deserialize, Serialize};
use std::fmt;


use self::Register::*;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub fn lookup_register(code: u8) ->  Option<Register>{
    match code {
        0  => Some(Register::Rax),
        1  => Some(Register::Rbx),
        2  => Some(Register::Rcx),
        3  => Some(Register::Rdx),
        4  => Some(Register::Rsi),
        5  => Some(Register::Rdi),
        6  => Some(Register::Rbp),
        7  => Some(Register::Rsp),
        8  => Some(Register::R8),
        9  => Some(Register::R9),
        10 => Some(Register::R10),
        11 => Some(Register::R11),
        12 => Some(Register::R12),
        13 => Some(Register::R13),
        14 => Some(Register::R14),
        15 => Some(Register::R15),
        _ => None

    }
}


impl Register {
    pub const VALUES: [Self; 16] = [Self::Rax,Self::Rbx,Self::Rcx,Self::Rdx,
                                Self::Rsi,Self::Rdi,Self::Rbp,Self::Rsp,
                                Self::R8,Self::R9,Self::R10,Self::R11,
                                Self::R12,Self::R13,Self::R14,Self::R15,];

    pub fn iterator() -> Iter<'static, Register> {
        static REGS: [Register; 16] = [Rax,Rbx,Rcx,Rdx,
        Rsi,Rdi,Rbp,Rsp,
        R8,R9,R10,R11,
        R12,R13,R14,R15,];
        REGS.iter()
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::Rax => write!(f, "rax"),
            Register::Rbx => write!(f, "rbx"),
            Register::Rcx => write!(f, "rcx"),
            Register::Rdx => write!(f, "rdx"),
            Register::Rsi => write!(f, "rsi"),
            Register::Rdi => write!(f, "rdi"),
            Register::Rbp => write!(f, "rbp"),
            Register::Rsp => write!(f, "rsp"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::R11 => write!(f, "r11"),
            Register::R12 => write!(f, "r12"),
            Register::R13 => write!(f, "r13"),
            Register::R14 => write!(f, "r14"),
            Register::R15 => write!(f, "r15"),
           
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
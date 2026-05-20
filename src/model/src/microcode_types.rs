
use crate::register::Register::*;

use crate::register::Register;

use std::fmt;


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq, Hash)]
pub enum Micro_reg{
    LogicReg(Register),
    U0, U1, U2, U3, U4, U5, U6, U7, 
    U8, U9, U10, U11, U12, U13, U14, U15,
    Rip,
    
}

impl Micro_reg {
    pub const VALUES: [Self; 33] = [Self::LogicReg(Rax),Self::LogicReg(Rbx),Self::LogicReg(Rcx),Self::LogicReg(Rdx),
                                    Self::LogicReg(Rsi),Self::LogicReg(Rdi),Self::LogicReg(Rbp),Self::LogicReg(Rsp),
                                    Self::LogicReg(R8),Self::LogicReg(R9),Self::LogicReg(R10),Self::LogicReg(R11),
                                    Self::LogicReg(R12),Self::LogicReg(R13),Self::LogicReg(R14),Self::LogicReg(R15),
                                    Self::U0,Self::U1,Self::U2,Self::U3,
                                    Self::U4,Self::U5,Self::U6,Self::U7,
                                    Self::U8,Self::U9,Self::U10,Self::U11,
                                    Self::U12,Self::U13,Self::U14,Self::U15,
                                    Self::Rip,
                                    ];
}


impl fmt::Display for Micro_reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LogicReg(x) => return write!(f, "{:?}", x),
            _ => return write!(f, "{:?}", self),
        };
        
    }
}



#[derive(PartialEq)]
#[derive(Clone)]
pub struct U_Instruction {
    pub u_type: U_Type,
    pub u_operator: U_Operator,
    pub u_operands : Vec<U_Operand>,
}

impl U_Instruction {
    pub fn new(u_type_: U_Type, u_operator_ : U_Operator, u_operands_: Vec<U_Operand>) -> U_Instruction {
        U_Instruction {
            u_type     : u_type_,
            u_operator : u_operator_,
            u_operands : u_operands_,
        }
    }
}

impl fmt::Debug for U_Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x: String = "".to_string();
        x += &self.u_type.to_string();
        x += " ";
        x += &self.u_operator.to_string();
        x += &self.u_operands.iter().map(|y| " ".to_string() + &y.to_string()).collect::<String>();
        write!(f, "{}", x.to_string())
    }
}

impl fmt::Display for U_Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}





#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum U_Type{
    Address,
    Offset,
    Immediate,
    Register,
}

impl fmt::Display for U_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum U_Operator{
    Load,
    Store,
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
    Jz,
    Jc, 
    Jo,
    Js,
}

impl fmt::Display for U_Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum U_Operand{
    Num(i64),
    Addr(u64),
    Reg(Micro_reg),
    OffsetReg(Micro_reg, i64),
}

impl fmt::Display for U_Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            U_Operand::Num(x) => write!(f, "{}", x.to_string()),
            U_Operand::Addr(x) => write!(f, "0x{:X}", x),
            U_Operand::Reg(x) => write!(f, "{}", x),
            U_Operand::OffsetReg(x, y) => write!(f, "{:?}({:?})", y, x),
        }
    }
}



pub enum Micro_OP{
    ALoad(Micro_reg, u64),
    AStore(Micro_reg, u64),

    OLoad(Micro_reg, Micro_reg, i64),
    OStore(Micro_reg, Micro_reg, i64),

    ILoad(Micro_reg, i64),
    IAdd(Micro_reg, i64),
    ISub(Micro_reg, i64),
    ICmp(Micro_reg, i64),
    IJmp(i64),
    IJz(i64),

    RLoad(Micro_reg, Micro_reg),
    RAdd(Micro_reg, Micro_reg),
    RSub(Micro_reg, Micro_reg),
    RCmp(Micro_reg, Micro_reg),
    RMul(Micro_reg),
    RDiv(Micro_reg),
    RIMul(Micro_reg),
    RIDiv(Micro_reg),
    
}


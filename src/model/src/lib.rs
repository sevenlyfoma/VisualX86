#![allow(unused_imports)] 
#![allow(unused_variables)] 
#![allow(dead_code)] 
#![allow(unreachable_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

pub mod register;
pub mod operand;
pub mod operator;
pub mod instruction;
pub mod simulator;
pub mod mainmemory;
pub mod clisimulator;
pub mod parser_types;
pub mod scannerless_parser;

pub mod assembler_mc;
pub mod predecoder;

pub mod microcode_types;
pub mod decoder;

pub mod alu;
pub mod mu;
pub mod ju;
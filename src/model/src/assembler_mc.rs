use crate::parser_types::LineT::*;
use crate::parser_types::LineT;
use crate::parser_types::LineBodyT::*;
use crate::parser_types::LineBodyT;
use crate::parser_types::InstOpDopT::*;
use crate::parser_types::InstOpDopT;
use crate::parser_types::InstOpT::*;
use crate::parser_types::InstOpT;
use crate::parser_types::InstDopT::*;
use crate::parser_types::InstDopT;
use crate::parser_types::InstJumpT::*;
use crate::parser_types::InstJumpT;
use crate::parser_types::OperandT::*;
use crate::parser_types::OperandT;
use crate::parser_types::DestReadyOperandT::*;
use crate::parser_types::DestReadyOperandT;
use crate::parser_types::OffsetT::*;
use crate::parser_types::OffsetT;
use crate::parser_types::RegisterT::*;
use crate::parser_types::RegisterT;


use crate::operator::Operator::*;
use crate::operand::Operand::*;
use crate::operand::Operand;
use crate::register::Register::*;
use crate::register::Register;


use crate::instruction::Instruction;
use crate::scannerless_parser;

use std::collections::BTreeMap;

/// Takes a string of assembler code and converts to a vector of bytes or an error message
pub fn assemble_to_machine_code(input: &str) -> Result<Vec<u8>, String>{

    // First parse input into "parser types" representation
    let line_vec_res = scannerless_parser::lines(input);

    if let Err(x) = line_vec_res{
        return Err(x.to_string());
    }

    let (remaining_input, mut line_vec) = line_vec_res.unwrap();

    // If there is remaining input then there has been a failure somewhere
    // Determine the line of the failure by counting characters
    if !remaining_input.is_empty(){
        let failpoint = input.chars().count() - remaining_input.chars().count();
        let mut line = 1;
        for (i, c) in input.chars().enumerate(){
            if c == '\n'{
                line += 1;
            }
            if i >= failpoint{
                break;
            }
        }
        return Err(format!("Parsing failed at line {:?}", line));
    }

    // Generate symbol table in first pass through AST, for identifying line number of the labels
    let symbol_table_result = generate_symbol_table(&mut line_vec);

    if let Err(x) = symbol_table_result{
        return Err(x);
    }

    let symbol_table = symbol_table_result.unwrap();

    // Generate the machine code with the symbol table and AST
    return generate_machine_code(line_vec, symbol_table);
}

/// Code to identify the names and line number of all labels in the vector of code "lines"
fn generate_symbol_table(input: &mut Vec<LineT>) -> Result<BTreeMap<String, i64>, String>{

    //Counter of number of bytes per line so far
    //Bytes calculated same way as generation of byte code
    //Work needs to be duplicated because we need to know byte size per instruction for both passes of code
    let mut counter = 0;

    let mut symbol_table: BTreeMap<String, i64> = BTreeMap::new();
    
    for Line(label, line_body) in input.iter_mut() {
        if !label.is_empty(){
            //If label is a duplicate then we have an error 
            if symbol_table.contains_key(label){
                return Err("Instructions contain Duplicate Label Definitions".to_string());
            }
            //Otherwise add it to table with byte number
            else{
                symbol_table.insert(label.to_string(), counter);
            }
        }

        match line_body{
            LineOpDop(_, oprnd1, oprnd2) => {
                let res_1 = generate_operand_bytes(oprnd1);
                let res_2 = generate_dest_operand_bytes(oprnd2);

                //Labels as arguments are permitted by parsing, but not usable outside of jump addresses
                if let None = res_1 {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", counter))
                }
                if let None = res_2 {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", counter))
                }

                let mut op1 = res_1.unwrap();
                let mut op2 = res_2.unwrap();

                //Increment bytes
                //1 for size, 1 for operator and x for each operand 
                counter += 1 + 1 + (op1.len() as i64) + (op2.len() as i64); // 1 byte for length, 1 byte for operator, bytes for each operand

            },
            LineOp(_, oprnd) => {

                let res = generate_operand_bytes(oprnd);
                if let None = res {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", counter))
                }

                let mut op = res.unwrap();

                counter += 1 + 1 + (op.len() as i64)


            },
            LineDop(_, oprnd) => {
                let res = generate_dest_operand_bytes(oprnd);
                if let None = res {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", counter))
                }

                let mut op = res.unwrap();

                counter += 1 + 1 + (op.len() as i64)

            },
            LineJump(_, _) => {
                
                //One for size, one for operator, one for operand metadata and 4 for 32 bit jump target
                counter += 1 + 1 + 1 + 4 //assuming limit of 32 bit length jumps, a bit stupid since theres only 256 bytes of memory

            },
        }
    }

    return Ok(symbol_table);

    
}

/// Takes vector of parser_types Lines and symbol table and converts to byte code vector
fn generate_machine_code(mut input: Vec<LineT>, symbol_table: BTreeMap<String, i64>) -> Result<Vec<u8>, String>{

    // Number of bytes so far
    let mut counter = 0;
    // Number of lines so far
    let mut line_counter = 0;

    let mut bytes: Vec<u8> = Vec::new();

    for Line(_, line_body) in input.iter_mut() {
        line_counter += 1;
        let mut size:u8 = 0;
        let mut operator_num:u8 = 0;

        match line_body{
            LineOpDop(oprtr, oprnd1, oprnd2) => {
                // Find byte code of operator 
                operator_num = oprtr.enum_index();

                let res_1 = generate_operand_bytes(oprnd1);
                let res_2 = generate_dest_operand_bytes(oprnd2);

                if let None = res_1 {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", line_counter))
                }
                if let None = res_2 {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", line_counter))
                }

                let mut op1 = res_1.unwrap();
                let mut op2 = res_2.unwrap();

                size = 1 + 1 + (op1.len() as u8) + (op2.len() as u8); // 1 byte for length, 1 byte for operator, bytes for each operand
                counter += size as u64;

                //First byte size
                bytes.push(size);
                //Second byte is operator 
                bytes.push(operator_num);

                bytes.append(&mut op1);
                bytes.append(&mut op2);


            },
            LineOp(oprtr, oprnd) => {
                operator_num = oprtr.enum_index();

                let res = generate_operand_bytes(oprnd);
                if let None = res {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", line_counter))
                }

                let mut op = res.unwrap();

                size = 1 + 1 + (op.len() as u8);
                counter += size as u64;


                bytes.push(size);
                bytes.push(operator_num);

                bytes.append(&mut op);
            },
            LineDop(oprtr, oprnd) => {
                operator_num = oprtr.enum_index();

                let res = generate_dest_operand_bytes(oprnd);
                if let None = res {
                    return Err(format!("Label as operand representing memory address not yet implemented at line {:?}", line_counter))
                }

                let mut op = res.unwrap();

                size = 1 + 1 + (op.len() as u8);
                counter += size as u64;


                bytes.push(size);
                bytes.push(operator_num);

                bytes.append(&mut op);
            },
            LineJump(oprtr, oprnd) => {
                operator_num = oprtr.enum_index();

                //Get byte number of jump target from symbol table
                let jump_location_opt = symbol_table.get(oprnd);
                //Jump location doesnt have a corresponding position in symbol table
                if let None = jump_location_opt{
                    return Err(format!("Jump instruction refrences non-existant location at line {:?}", line_counter))
                }

                //4 in top half is "jump"
                //2 in bottom half is 32 bits
                let meta_data: u8 = (4 << 4) + 2;

                let jump_location = *jump_location_opt.unwrap() as i64;

                //Calculate jump relative location based on current byte number 
                let jump_relative = (jump_location - (counter as i64)) as u32;

                //Convert jump address to 4 bytes Little endian
                let jump_relative_bytes = jump_relative.to_le_bytes();

                size = 1 + 1 + 1+ 4; //size + operator + metadata + data
                counter += size as u64;

                bytes.push(size);
                bytes.push(operator_num);
                bytes.push(meta_data);

                for n in 0..=3 {
                    bytes.push(jump_relative_bytes[n]);
                }     



            },

        }
    }
    



    return Ok(bytes);
}

/// Takes a operand line from parser types and converts to bytes
/// Either an immediate or a dest ready operand, which is all the other types of oiperand
fn generate_operand_bytes(op: &mut OperandT) -> Option<Vec<u8>>{
    match op {
        Immediate(x) => {
            let mut bytes: Vec<u8> = Vec::new();

            // Get size representation of number, 8 bits = 0, 16 = 1 32 = 2 64 = 3
            let size:u8 = get_num_size(*x as u64);

            let meta_data:u8 = (0<<4) + size; //top 4 bits are addressing type, in this case immediate

            let data_bytes: [u8; 8] = x.to_le_bytes(); //convert data to bytes little endian

            bytes.push(meta_data);

            for n in 0..=(((2_i32.pow(size.into()))-1) as usize) {
                bytes.push(data_bytes[n]);
            }     

            return Some(bytes);


        }
        DestReady(x) => {
            return generate_dest_operand_bytes(x);
        }
    }
}

/// Converts all non immediate operands to bytes
fn generate_dest_operand_bytes(dop: &mut DestReadyOperandT) -> Option<Vec<u8>>{

    let mut bytes: Vec<u8> = Vec::new();

    match dop{
        DestReadyOperandT::Register(reg) => {
            let meta_data:u8 = (1 << 4) + 0; //Code for register mode is 1, code for size 1 byte is 0, so 0
            let reg_num: u8 = reg.enum_index(); //Get register representation code
            bytes.push(meta_data);
            bytes.push(reg_num);
        },
        DestReadyOperandT::HexNumber(hexnum) => {
            // Get size representation of number, 8 bits = 0, 16 = 1 32 = 2 64 = 3
            let size:u8 = get_num_size(*hexnum);
            let meta_data:u8 = (2 << 4) + size; //2 is address mode

            let data_bytes: [u8; 8] = hexnum.to_le_bytes();

            bytes.push(meta_data);

            //Number of bytes calculated with the size, 2^0 = 1 bytes, 2^1= 2 bytes, 2^2 = 4 bytes etc
            for n in 0..=(((2_i32.pow(size.into()))-1) as usize) {
                bytes.push(data_bytes[n]);
            }            
        },
        DestReadyOperandT::Indirect(OffsetNum(off), reg) => {
            let size:u8 = get_num_size(*off as u64);
            let meta_data:u8 = (3 << 4) + size; //3 is indirect/offset mode
            let reg_num: u8 = reg.enum_index();

            let data_bytes: [u8; 8] = off.to_le_bytes();

            bytes.push(meta_data);
            bytes.push(reg_num);

            //Number of bytes calculated with the size, 2^0 = 1 bytes, 2^1= 2 bytes, 2^2 = 4 bytes etc
            for n in 0..=(((2_i32.pow(size.into()))-1) as usize) {
                bytes.push(data_bytes[n]);
            }       
        },
        //Not implemented, the ability to name locations
        DestReadyOperandT::Label(x) => {return None;},
    }
    

    return Some(bytes);
}

/// Takes a u64 representing a number and determines how many bytes it takes to store it
fn get_num_size(num: u64) -> u8{
    let numi = num as i64; // Have to conver to i64 as negative numbers are extremely large positive numbers when unsigned
    if numi <= 127 && numi >= -128{
        return 0;
    }
    else if numi <= 32767 && numi >= -32768{
        return 1;
    }
    else if  numi <= 2147483647 && numi >= -2147483648{
        return 2;
    }
    else{
        return 3;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_symbol_table_with_one_label(){
        let mut input = vec![
            Line("total".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RaxD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RbxD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RcxD))),
            Line("".to_string(), LineJump(JmpD, "total".to_string())),
        ];

        let actual_output = generate_symbol_table(&mut input).unwrap();
        let mut expected_output: BTreeMap<String, i64> = BTreeMap::new();
        expected_output.insert("total".to_string(), 0);



        
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_generate_symbol_table_with_multiple_label(){
        let mut input = vec![
            Line("total".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))), //6 bytes
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RaxD))), //6 bytes
            Line("lable2".to_string(), LineOpDop(AddD, Immediate(4), Register(RbxD))), //6 bytes
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RcxD))),//6 bytes
            Line("label3".to_string(), LineJump(JmpD, "total".to_string())), //7 bytes
        ];

        let actual_output = generate_symbol_table(&mut input).unwrap();
        let mut expected_output: BTreeMap<String, i64> = BTreeMap::new();
        expected_output.insert("total".to_string(), 0);
        expected_output.insert("lable2".to_string(), 12);
        expected_output.insert("label3".to_string(), 24);




        
        assert_eq!(actual_output, expected_output);
    }

    
    #[test]
    fn test_generate_symbol_table_with_identical_labels(){
        let mut input = vec![
            Line("total".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RaxD))),
            Line("total".to_string(), LineOpDop(AddD, Immediate(4), Register(RbxD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RcxD))),
            Line("".to_string(), LineJump(JmpD, "total".to_string())),
        ];

        let actual_output = generate_symbol_table(&mut input);

        match actual_output {
            Ok(_) => panic!("Test should fail"),
            Err(_) => {},
        }
    }

    #[test]
    fn test_generate_machine_code_one_isntruction(){
        let mut input = vec![
            Line("".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))), //6 bytes
        ];

        let symbol_table = generate_symbol_table(&mut input).unwrap();

        let actual_output = generate_machine_code(input, symbol_table).unwrap();

        let expected_output: Vec<u8> = vec![
            6, //6 bytes
            0, //Mov instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            0b11111101, //-3 in twos complement, 8 bits
            (1 << 4) + 0, //Register mode (1 in top nibble), Size mode 0 (1 byte)
            4, //Rsi
        ];
       
        
        assert_eq!(actual_output, expected_output);
    }


    #[test]
    fn test_generate_machine_code_multiple_isntructions(){
        let mut input = vec![
            Line("total".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))), //6 bytes
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RaxD))), //6 bytes
            Line("lable2".to_string(), LineOpDop(AddD, Immediate(4), Register(RbxD))), //6 bytes
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RcxD))),//6 bytes
            Line("label3".to_string(), LineJump(JmpD, "total".to_string())), //7 bytes
        ];

        let symbol_table = generate_symbol_table(&mut input).unwrap();

        let actual_output = generate_machine_code(input, symbol_table).unwrap();

        let expected_output: Vec<u8> = vec![
            6, //6 bytes
            0, //Mov instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            0b11111101, //-3 in twos complement, 8 bits
            (1 << 4) + 0, //Register mode (1 in top nibble), Size mode 0 (1 byte)
            4, //Rsi

            6, //6 bytes
            1, //Add instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            4, //4 
            (1 << 4) + 0, //Register mode (1 in top nibble), Size mode 0 (1 byte)
            0, //Rax

            6, //6 bytes
            1, //Add instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            4, //4 
            (1 << 4) + 0, //Register mode (1 in top nibble), Size mode 0 (1 byte)
            1, //Rbx

            6, //6 bytes
            1, //Add instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            4, //4 
            (1 << 4) + 0, //Register mode (1 in top nibble), Size mode 0 (1 byte)
            2, //Rcx

            7, //7 bytes
            16, //Jmp instruction
            (4 << 4) + 2, //JumpAddr mode (4 in top nibble), Size mode 2 (4 byte)
            ((-24_i8) as u8), //-24 first 8 bits, little endian
            255, //Sign extensions for -24
            255,
            255,

        ];
       
        
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_generate_machine_code_immediate_indirect(){
        let mut input = vec![
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Indirect(OffsetNum(16), RaxD))), //6 bytes
        ];

        let symbol_table = generate_symbol_table(&mut input).unwrap();

        let actual_output = generate_machine_code(input, symbol_table).unwrap();

        let expected_output: Vec<u8> = vec![
            7, //6 bytes
            1, //Mov instruction
            (0 << 4) + 0, //Immediate mode (0 in top nibble), Size mode 0 (1 byte)
            4, //-3 in twos complement, 8 bits
            (3 << 4) + 0, //indirect mode (3 in top nibble), Size mode 0 (1 byte)
            0, //Rax
            16, // Offset of 16
        ];
       
        
        assert_eq!(actual_output, expected_output);
    }



}

use crate::operator::Operator::*;
use crate::operator::*;
use crate::operand::Operand::*;
use crate::operand::Operand;
use crate::register::Register::*;
use crate::register::Register;
use crate::register::*;
use crate::instruction::Instruction;

use crate::simulator::PREFETCH_BUFFER_SIZE;

/// Takes an array of bytes of the size of the prefetch buffer and turns them into a possible vector of instructions and their size in bytes
pub fn predecode(input: &[u8; PREFETCH_BUFFER_SIZE]) -> Option<Vec<(Instruction, u8)>>{

    let mut instruction_queue_contents: Vec<(Instruction, u8)> = Vec::new();
    let mut position: usize = 0;


    loop {
        //Get first byte in instruction, this is its size
        let size_byte: u8 = input[position];

        //If its zero there is no more code
        //If the size is bigger than the rest of the buffer, we dont have the rest of the instrcution, so we have to stop
        if (size_byte == 0) || (position + (size_byte as usize) >= PREFETCH_BUFFER_SIZE){
            break;
        }


        //Get the bytes for this instruction
        let instruction_bytes = &input[position..position+(size_byte as usize)];

        //Predecode those bytes
        let Some(instruction) = predecode_one(instruction_bytes) else{
            return None;
        };

        //Push to queue
        instruction_queue_contents.push(instruction);

        //Increment position so we know where to start next loop
        position += size_byte as usize;
    }



    return Some(instruction_queue_contents);

}

//Predecodes an array of bytes into a single instruction along with its size as a pair
//Option because it may fail
pub fn predecode_one(input: &[u8]) -> Option<(Instruction, u8)>{

    let size_byte: u8 = input[0];
    //Operator stored in second byte
    let operator_byte: u8 = input[1];

    //Operator bytes must correspond to one of the defined values
    let Some(operator) = lookup_opcode(operator_byte) else{
        return None;
    };

    //Start at 3rd byte
    let mut index = 2;

    //We want to get a list of operands
    let mut operand_list = Vec::new();
    while index < (size_byte as usize){
        //3rd byte is metadata for the operand
        //Top 4 bits is addressing mode
        //Bottom 4 bits is the size
        let operand_meta_byte: u8 = input[index];
        index += 1; 

        let addressing_mode_nibble = (operand_meta_byte & 0b11110000) >> 4;

        let operand_size_nibble = operand_meta_byte & 0b00001111;

        //Operand size meta data is the number of bits used to store
        //So 3 bits means a size of 8 bytes.
        let operand_size = 2_usize.pow(operand_size_nibble.into());

        match addressing_mode_nibble {
            0 => {
                //Immediate 
                //Convert the operand size number of bytes to a number
                let data =  &input[index..index+operand_size];
                let Some(num) = convert_array_bytes_to_num(data, operand_size) else {
                    return None;
                };
                operand_list.push(Operand::Num(num));
                index += operand_size as usize;
            }

            1 => {
                //Register 
                //Convert the value of the byte to a register value via lookup
                let reg_num = input[index];
                let Some(reg) = lookup_register(reg_num) else{
                    return None;
                };
                operand_list.push(Operand::Reg(reg));
                index += 1;
            }

            2 => {
                //Raw address
                //Convert the operand size number of bytes to an address
                let data =  &input[index..index+operand_size];
                let Some(addr) = convert_array_bytes_to_num(data, operand_size) else {
                    return None;
                };
                operand_list.push(Operand::Addr(addr as u64));
                index += operand_size as usize;
            }

            3 => {
                //Offset/Indirect
                //First byte defines register
                let reg_num = input[index];
                let Some(reg) = lookup_register(reg_num) else{
                    return None;
                };
                index += 1;

                //Latter bytes define offset
                let data =  &input[index..index+operand_size];
                let Some(addr) = convert_array_bytes_to_num(data, operand_size) else {
                    return None;
                };

                operand_list.push(Operand::Offset(addr, reg));
                index += operand_size as usize;
            }

            4 => {
                //jump target
                //If a jump target acts like an immediate value
                let data =  &input[index..index+operand_size];
                let Some(num) = convert_array_bytes_to_num(data, operand_size) else {
                    return None;
                };
                operand_list.push(Operand::Num(num));
                index += operand_size as usize;
            }

            _ => {return None;}
        }

    }

    

    return Some((Instruction::new(
        operator,
        operand_list, 
    ), size_byte));
}

/// Converts an array of unsigned bytes to a signed quadword for internal represntation
fn convert_array_bytes_to_num(input: &[u8], size: usize) -> Option<i64>{

    match size{
        1 => { 
            return Some((input[0] as i8) as i64);        
        },
        2 => {
            let mut bytes: [u8; 2] = [0; 2];
            bytes.copy_from_slice(input);
            return Some(i16::from_le_bytes(bytes) as i64);
        },
        4 => {
            let mut bytes: [u8; 4] = [0; 4];
            bytes.copy_from_slice(input);
            return Some(i32::from_le_bytes(bytes) as i64);
        },
        8 => {
            let mut bytes: [u8; 8] = [0; 8];
            bytes.copy_from_slice(input);
            return Some(i64::from_le_bytes(bytes));
        },
        _ => {return None;},
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predecode_one_machine_code_isntruction(){

        let mut input: [u8; PREFETCH_BUFFER_SIZE] = [0; PREFETCH_BUFFER_SIZE];

        input[0] = 6;
        input[1] = 0; //Mov
        input[2] = (0 << 4) + 0; //immediate mode + 1 byte size
        input[3] = 0b11111101; // -3
        input[4] = (1 << 4) + 0; //register mode + 1 byte size
        input[5] = 4; //
        

        let expected_output: Option<Vec<(Instruction,u8)>> =  Some(vec![
            (Instruction::new(
                Mov, 
                vec![Num(-3), Reg(Rsi)],
            ), 6),
    
        ]);

        let actual_output = predecode(&input);

        assert_eq!(actual_output, expected_output);

    }


    #[test]
    fn test_predecode_multiple_machine_code_isntruction(){

        let mut input: [u8; PREFETCH_BUFFER_SIZE] = [0; PREFETCH_BUFFER_SIZE];

        input[0] = 6;
        input[1] = 0; //Mov
        input[2] = (0 << 4) + 0; //immediate mode + 1 byte size
        input[3] = 0b11111101; // -3
        input[4] = (1 << 4) + 0; //register mode + 1 byte size
        input[5] = 4; //Rsi

        input[6] = 6;
        input[7] = 1; //Add
        input[8] = (0 << 4) + 0; //immediate mode + 1 byte size
        input[9] = 10; // 10
        input[10] = (1 << 4) + 0; //register mode + 1 byte size
        input[11] = 0; // Rax

        

        let expected_output: Option<Vec<(Instruction, u8)>> =  Some(vec![
            (Instruction::new(
                Mov, 
                vec![Num(-3), Reg(Rsi)],
            ), 6),
            (Instruction::new(
                Add, 
                vec![Num(10), Reg(Rax)],
            ), 6),
    
        ]);

        let actual_output = predecode(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_predecode_one_machine_code_address_plus_indirect(){

        let mut input: [u8; PREFETCH_BUFFER_SIZE] = [0; PREFETCH_BUFFER_SIZE];

        input[0] = 14;
        input[1] = 0; //Mov
        input[2] = (2 << 4) + 3; //raw address mode + 8 byte size
        input[3] = 255; // Address 255
        input[4] = 0;
        input[5] = 0;
        input[6] = 0; 
        input[7] = 0; 
        input[8] = 0;
        input[9] = 0; 
        input[10] = 0; 
        input[11] = (3 << 4) + 0; //indirect mode + 1 byte size
        input[12] = 0; //Rax
        input[13] = 1; //1 offset

        

        let expected_output: Option<Vec<(Instruction, u8)>> =  Some(vec![
            (Instruction::new(
                Mov, 
                vec![Addr(255), Offset(1, Rax)],
            ), 14),
    
        ]);

        let actual_output = predecode(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_predecode_one_jump(){

        let mut input: [u8; PREFETCH_BUFFER_SIZE] = [0; PREFETCH_BUFFER_SIZE];

        input[0] = 7;
        input[1] = 16; //Mov
        input[2] = (4 << 4) + 2; //jump mode + 4 byte size
        input[3] = 10; //Jump 10
        input[4] = 0;
        input[5] = 0;
        input[6] = 0;
        
        

        let expected_output: Option<Vec<(Instruction, u8)>> =  Some(vec![
            (Instruction::new(
                Jmp, 
                vec![Num(10)],
            ), 7),
    
        ]);

        let actual_output = predecode(&input);

        assert_eq!(actual_output, expected_output);

    }

}
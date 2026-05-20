use crate::operator::Operator::*;
use crate::operator::*;
use crate::operand::Operand::*;
use crate::operand::Operand;
use crate::register::Register::*;
use crate::register::Register;
use crate::register::*;
use crate::instruction::Instruction;

use crate::simulator::PREFETCH_BUFFER_SIZE;

use crate::microcode_types::Micro_OP::*;
use crate::microcode_types::Micro_OP;
use crate::microcode_types::Micro_reg::*;
use crate::microcode_types::Micro_reg;
use crate::microcode_types::*;

/// Takes an instruction paired with its size in bytes and turns it into micro instructions
pub fn decode_instruction(input_pair: &(Instruction, u8)) -> Option<Vec<U_Instruction>>{
    let (input, instruction_size) = input_pair;
    let mut micro_ops: Vec<U_Instruction> = Vec::new();

    if input.operands.len() == 0 || input.operands.len() > 2{
        return None;
    }

    //Determines where an instruction should be inserted
    //Memory operations need a load at start and a store at end, so this determines the "body" of the uops
    let mut insertion_position = 0;

    //Dummy values for operands
    let mut first_operand: U_Operand = U_Operand::Num(0);
    let mut second_operand: U_Operand = U_Operand::Num(0);

    //Type used by simulator to determine which FU to use
    let mut u_type: U_Type = U_Type::Register;

    //Micro code switchs operand order, so last is first
    let first_operand_index = input.operands.len() - 1;

    match input.operands[first_operand_index] {
        Num(num)         => {
            first_operand = U_Operand::Num(num);
            u_type = U_Type::Immediate;
        },
        Reg(reg)         => {
            first_operand = U_Operand::Reg(LogicReg(reg));
        },
        Addr(addr)       => {
            //Load and then store with "body" in between
            //Use micro reg U0 for this
            let load_addr_op = U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(addr)],
            );
            let store_addr_op = U_Instruction::new(
                U_Type::Address,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(addr)],
            );

            micro_ops.push(load_addr_op);
            micro_ops.push(store_addr_op);

            insertion_position += 1;

            first_operand = U_Operand::Reg(U0);
        },
        Indirect(reg)    => {  
            //Indirect works same as regular address
            let load_addr_op = U_Instruction::new(
                U_Type::Offset,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::OffsetReg(LogicReg(reg), 0)],
            );
            let store_addr_op = U_Instruction::new(
                U_Type::Offset,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::OffsetReg(LogicReg(reg), 0)],
            );

            micro_ops.push(load_addr_op);
            micro_ops.push(store_addr_op);

            insertion_position += 1;

            first_operand = U_Operand::Reg(U0);
        },
        Offset(num, reg) => {
            //Offset indirect is a load of the register value, add the offset to it, then load the memory address, then store
            let load_register_value = U_Instruction::new(
                U_Type::Register,
                U_Operator::Load, 
                vec![U_Operand::Reg(U15), U_Operand::Reg(LogicReg(reg))],
            );
            let offset_register_value = U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(U15), U_Operand::Num(num)],
            );
            let load_addr_op = U_Instruction::new(
                U_Type::Offset,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::OffsetReg(U15, 0)],
            );
            let store_addr_op = U_Instruction::new(
                U_Type::Offset,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::OffsetReg(U15, 0)],
            );
            micro_ops.push(load_register_value);
            micro_ops.push(offset_register_value);
            micro_ops.push(load_addr_op);
            micro_ops.push(store_addr_op);

            insertion_position += 3;

            first_operand = U_Operand::Reg(U0);
        },
    }
    
    if input.operands.len() == 2{
        match input.operands[0] {
            Num(num)         => {
                second_operand = U_Operand::Num(num);
                u_type = U_Type::Immediate;
            },
            Reg(reg)         => {second_operand = U_Operand::Reg(LogicReg(reg));},
            Addr(addr)       => {
                //Micro reg U1 used for second address
                let load_addr_op = U_Instruction::new(
                    U_Type::Address,
                    U_Operator::Load, 
                    vec![U_Operand::Reg(U1), U_Operand::Addr(addr)],
                );    
                micro_ops.insert(insertion_position, load_addr_op);

                insertion_position += 1;

                second_operand = U_Operand::Reg(U1);
            },
            Indirect(reg)    => {  
                let load_addr_op = U_Instruction::new(
                    U_Type::Offset,
                    U_Operator::Load, 
                    vec![U_Operand::Reg(U1), U_Operand::OffsetReg(LogicReg(reg), 0)],
                );      
                micro_ops.insert(insertion_position, load_addr_op);

                insertion_position += 1;

                second_operand = U_Operand::Reg(U1);
            },
            Offset(num, reg) => {
                let load_register_value = U_Instruction::new(
                    U_Type::Register,
                    U_Operator::Load, 
                    vec![U_Operand::Reg(U14), U_Operand::Reg(LogicReg(reg))],
                );
                let offset_register_value = U_Instruction::new(
                    U_Type::Immediate,
                    U_Operator::Add, 
                    vec![U_Operand::Reg(U14), U_Operand::Num(num)],
                );
                let load_addr_op = U_Instruction::new(
                    U_Type::Offset,
                    U_Operator::Load, 
                    vec![U_Operand::Reg(U1), U_Operand::OffsetReg(U14, 0)],
                );
                micro_ops.insert(insertion_position, load_register_value);
                insertion_position += 1;

                micro_ops.insert(insertion_position, offset_register_value);
                insertion_position += 1;

                micro_ops.insert(insertion_position, load_addr_op);
                insertion_position += 1;

                second_operand = U_Operand::Reg(U1);
            },
        }
    }

    //Convert instruction operator into micro instruction operator
    //Mostly a one to one mapping
    //Add operands to make ful instrucitons
    match input.operator {
        Operator::Mov => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Load, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Add => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Add, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Sub => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Sub, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Mul => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Mul, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Div => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Div, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::IMul => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::IMul, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::IDiv => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::IDiv, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Cmp => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Cmp, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::And => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::And, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Or => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Or, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Xor => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Xor, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Not => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Not, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Sal => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Sal, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Sar => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Sar, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Shl => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Shl, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Shr => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Shr, 
                vec![first_operand, second_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Jmp => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Jmp, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Jz => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Jz, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Jc => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Jc, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Jo => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Jo, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },
        Operator::Js => {
            let u_in = U_Instruction::new(
                u_type,
                U_Operator::Js, 
                vec![first_operand],
            );
            micro_ops.insert(insertion_position, u_in);
        },

    }

    let rip_increment = U_Instruction::new(
        U_Type::Immediate,
        U_Operator::Add, 
        vec![U_Operand::Reg(Rip), U_Operand::Num(*instruction_size as i64)],
    );
    micro_ops.push(rip_increment);

    return Some(micro_ops);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_reg_reg(){

        let input = (Instruction::new(
            Add, 
            vec![Reg(Rsi), Reg(Rax)],
        ), 6);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Add, 
                vec![U_Operand::Reg(LogicReg(Rax)), U_Operand::Reg(LogicReg(Rsi))],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(6)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_reg_immediate(){

        let input = (Instruction::new(
            Add, 
            vec![Num(4), Reg(Rax)],
        ), 6);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(LogicReg(Rax)), U_Operand::Num(4)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(6)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_reg_memory(){

        let input = (Instruction::new(
            Add, 
            vec![Addr(4), Reg(Rax)],
        ), 13);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U1), U_Operand::Addr(4)],
            ),
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Add, 
                vec![U_Operand::Reg(LogicReg(Rax)), U_Operand::Reg(U1)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(13)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_memory_memory(){

        let input = (Instruction::new(
            Add, 
            vec![Addr(4), Addr(5)],
        ),19);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U1), U_Operand::Addr(4)],
            ),
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Add, 
                vec![U_Operand::Reg(U0), U_Operand::Reg(U1)],
            ),
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(19)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_memory_reg(){

        let input = (Instruction::new(
            Add, 
            vec![Reg(Rax),Addr(5)],
        ), 13);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Add, 
                vec![U_Operand::Reg(U0), U_Operand::Reg(LogicReg(Rax))],
            ),
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(13)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_memory(){

        let input = (Instruction::new(
            Mul, 
            vec![Addr(5)],
        ), 11);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Load, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Mul, 
                vec![U_Operand::Reg(U0)],
            ),
            U_Instruction::new(
                U_Type::Address,
                U_Operator::Store, 
                vec![U_Operand::Reg(U0), U_Operand::Addr(5)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(11)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_register(){

        let input = (Instruction::new(
            Mul, 
            vec![Reg(Rax)],
        ), 4);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Register,
                U_Operator::Mul, 
                vec![U_Operand::Reg(LogicReg(Rax))],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(4)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    #[test]
    fn test_decode_immediate(){

        let input = (Instruction::new(
            Jmp, 
            vec![Num(4)],
        ), 7);

        let expected_output: Option<Vec<U_Instruction>> = Some(vec![
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Jmp, 
                vec![U_Operand::Num(4)],
            ),
            U_Instruction::new(
                U_Type::Immediate,
                U_Operator::Add, 
                vec![U_Operand::Reg(Rip), U_Operand::Num(7)],
            ),
        ]);

        let actual_output = decode_instruction(&input);

        assert_eq!(actual_output, expected_output);

    }

    



}
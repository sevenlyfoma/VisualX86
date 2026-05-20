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




use std::str::FromStr;


use nom::IResult;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::pair;
use nom::Err;
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::lib::std::result::Result::Ok;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::character::complete::char;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::is_a;


/// Recognises register names and maps them to enum
fn register(input: &str) -> IResult<&str, RegisterT> {
    let name = alt((
        map(tag("rax"), |_| RaxD),
        map(tag("rbx"), |_| RbxD),
        map(tag("rcx"), |_| RcxD),
        map(tag("rdx"), |_| RdxD),
        map(tag("rsi"), |_| RsiD),
        map(tag("rdi"), |_| RdiD),
        map(tag("rbp"), |_| RbpD),
        map(tag("rsp"), |_| RspD),
        map(tag("r8"), |_| R8D),
        map(tag("r9"), |_| R9D),
        map(tag("r10"), |_| R10D),
        map(tag("r11"), |_| R11D),
        map(tag("r12"), |_| R12D),
        map(tag("r13"), |_| R13D),
        map(tag("r14"), |_| R14D),
        map(tag("r15"), |_| R15D),
    ));

    preceded(char('%'), name)(input)
}

/// Recognises an unsigned number
fn unsigned(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |x| u64::from_str(x))(input)
}

/// Recognises a signed 64 bit number
fn signed(input: &str) -> IResult<&str, i64> {

    //If child was successful, takes consumed input and returns all as output
    //Basically dont seperate the "-"
    let int_string = recognize(tuple((opt(tag("-")), digit1)));
    map_res(int_string, |x| i64::from_str(x))(input)
}

/// Recognises a decimal number, either signed or unsigned
fn dec_num(input: &str) -> IResult<&str, i64> {
    let unsigned = map(unsigned, |x| x as i64);
    let signed = map(signed, |x| x);
    
    alt((
        unsigned,
        signed,
    ))(input)

}

/// Recognises immediate values preceded by $
fn immediate(input: &str) -> IResult<&str, OperandT> {
    map(preceded(tag("$"), dec_num), |x| Immediate(x))(input)
}

/// Recognises indirect operands so a register surrounded by brackets with or without an offset
fn indirect(input: &str) -> IResult<&str, DestReadyOperandT> {    
        map(pair(
            opt(signed), 
            preceded(tag("("), terminated(register, tag(")")))
            ), 
            |(off, reg)| {
                match off{
                    Some(x) => Indirect(OffsetNum(x), reg),
                    None => Indirect(OffsetNum(0), reg),
                }
            }        
            )(input)
    }

/// Recognises a hex number preceded by 0x
fn hex_num(input: &str) -> IResult<&str, u64> {
    let c = preceded(tag("0x"), many1(is_a("0123456789abcdefABCDEF")));

    map_res(c, |x| u64::from_str_radix(&x.join(""), 16))(input)
}

/// Recognises a label for loops starting with . like '.loop'
fn label(input: &str) -> IResult<&str, String> {
    map(take_while1(|x| {char::is_alphabetic(x) || x == '.'}), |y: &str| y.to_string())(input)
}

/// Recognises a label definition for a loop like 'loop:'
fn label_def(input: &str) -> IResult<&str, String> {
    terminated(label, tag(":"))(input)
}

/// Recognises a "destination ready operand", ie one that can be the destination for a calcuation
/// For example an immediate value cannot be a destination
fn dest_ready_operand(input: &str) -> IResult<&str, DestReadyOperandT> {
    alt((
        map(register, |x| Register(x)),
        map(hex_num, |x| HexNumber(x)),
        indirect,
        map(label, |x| Label(x)),
    ))(input)
}

/// Recognises any type of operand
fn operand(input: &str) -> IResult<&str, OperandT> {
    alt((
        map(dest_ready_operand, |x| DestReady(x)),
        immediate,
    ))(input)
}

/// Recognises an instruction that must take an operand followed by a destination ready operand
fn inst_op_dop(input: &str) -> IResult<&str, InstOpDopT> {

    alt((
        map(tag("movq"), |_| MovD),
        map(tag("addq"), |_| AddD),
        map(tag("subq"), |_| SubD),
        map(tag("cmpq"), |_| CmpD),
        map(tag("andq"), |_| AndD),
        map(tag("orq"), |_| OrD),
        map(tag("xorq"), |_| XorD),
        map(tag("salq"), |_| SalD),
        map(tag("sarq"), |_| SarD),
        map(tag("shlq"), |_| ShlD),
        map(tag("shrq"), |_| ShrD),
    ))(input)

}

/// Recognises an instruction that takes a single operand
fn inst_op(input: &str) -> IResult<&str, InstOpT> {

    alt((
        map(tag("mulq"), |_| MulD),
        map(tag("divq"), |_| DivD),
        map(tag("imulq"), |_| ImulD),
        map(tag("idivq"), |_| IdivD),
    ))(input)

}

/// Recognises an instruciton that takes a single destination ready operand
fn inst_dop(input: &str) -> IResult<&str, InstDopT> {

    alt((
        map(tag("notq"), |_| NotD),
    ))(input)

}

/// Recognises jump isntructions
fn inst_jump(input: &str) -> IResult<&str, InstJumpT> {

    alt((
        map(tag("jmp"), |_| JmpD),
        map(tag("jz"), |_| JzD),
        map(tag("jc"), |_| JcD),
        map(tag("jo"), |_| JoD),
        map(tag("js"), |_| JsD),
    ))(input)

}

/// Recognises a line of code that must take an operand followed by a destination ready operand
fn line_op_dop(input: &str) -> IResult<&str, LineBodyT> {
    map(
        tuple((inst_op_dop, multispace1, operand, char(','), multispace1, dest_ready_operand)),
        |(inst, _, op1, _, _, op2)| LineOpDop(inst, op1, op2) 
       )(input)
}

/// Recognises a line of code that must take an operand
fn line_op(input: &str) -> IResult<&str, LineBodyT> {
    map(
        tuple((inst_op, multispace1, operand)),
        |(inst, _, op1)| LineOp(inst, op1) 
       )(input)
}

/// Recognises a line of code that must take a destination ready operand
fn line_dop(input: &str) -> IResult<&str, LineBodyT> {
    map(
        tuple((inst_dop, multispace1, dest_ready_operand)),
        |(inst, _, op1)| LineDop(inst, op1) 
       )(input)
}

/// Recognises a line of code with a jump instrcution and a label
fn line_jmp(input: &str) -> IResult<&str, LineBodyT> {
    map(
        tuple((inst_jump, multispace1, label)),
        |(inst, _, lab)| LineJump(inst, lab) 
       )(input)
}

/// Recognises a line of code
fn line_body(input: &str) -> IResult<&str, LineBodyT> {
    alt((
        line_op_dop,
        line_op,
        line_dop,
        line_jmp,
    ))(input)
}

/// Recognises a line of code with an optional label definition
fn line(input: &str) -> IResult<&str, LineT> {
    map(
        tuple((opt(label_def), multispace0, line_body)),
        |(lab, _, body)| {
            match lab{
                Some(x) => Line(x, body),
                None => Line("".to_string(), body),
            }
        }
    )(input)
}

/// Recognises an entire program
pub fn lines(input: &str) -> IResult<&str, Vec<LineT>> {

    delimited(
        multispace0, // Optional whitespace at the beginning
        separated_list0(multispace1, line), // Instructions separated by 1+ spaces
        multispace0, // Optional whitespace at the end
    )(input)

}


#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;

    #[test]
    fn test_register_normal(){
        let (actual_input_remaining, actual_output) = register("%rsi").unwrap();
        let expected_output = RsiD;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_dec_num_normal(){
        let (actual_input_remaining, actual_output) = dec_num("5").unwrap();
        let expected_output = 5;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_immediate_normal(){
        let (actual_input_remaining, actual_output) = immediate("$5").unwrap();
        let expected_output = Immediate(5);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_indirect_just_reg(){
        let (actual_input_remaining, actual_output) = indirect("(%rax)").unwrap();
        let expected_output = Indirect(OffsetNum(0), RaxD);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_indirect_just_num_offset_and_reg(){
        let (actual_input_remaining, actual_output) = indirect("4(%rax)").unwrap();
        let expected_output = Indirect(OffsetNum(4), RaxD);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_hex_num_normal(){
        let (actual_input_remaining, actual_output) = hex_num("0x40").unwrap();
        let expected_output = 64;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_hex_num_letter(){
        let (actual_input_remaining, actual_output) = hex_num("0x0a").unwrap();
        let expected_output = 10;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }


    #[test]
    fn test_label_normal(){
        let (actual_input_remaining, actual_output) = label("loop").unwrap();
        let expected_output = "loop".to_string();
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_label_normal_with_dot(){
        let (actual_input_remaining, actual_output) = label(".loop").unwrap();
        let expected_output = ".loop".to_string();
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_label_def_normal(){
        let (actual_input_remaining, actual_output) = label_def("loop:").unwrap();
        let expected_output = "loop".to_string();
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_dest_ready_operand_register_normal(){
        let (actual_input_remaining, actual_output) = dest_ready_operand("%rax").unwrap();
        let expected_output = Register(RaxD);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_dest_ready_operand_hex_num_normal(){
        let (actual_input_remaining, actual_output) = dest_ready_operand("0x10").unwrap();
        let expected_output = HexNumber(16);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_dest_ready_operand_indirect_normal(){
        let (actual_input_remaining, actual_output) = dest_ready_operand("4(%rax)").unwrap();
        let expected_output = Indirect(OffsetNum(4), RaxD);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }
    
    #[test]
    fn test_dest_ready_operand_label_normal(){
        let (actual_input_remaining, actual_output) = dest_ready_operand("loop").unwrap();
        let expected_output = Label("loop".to_string());
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_operand_immediate_normal(){
        let (actual_input_remaining, actual_output) = operand("$4").unwrap();
        let expected_output = Immediate(4);
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }


    #[test]
    fn test_operand_register_normal(){
        let (actual_input_remaining, actual_output) = operand("%rax").unwrap();
        let expected_output = DestReady(Register(RaxD));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_operand_hex_num_normal(){
        let (actual_input_remaining, actual_output) = operand("0x10").unwrap();
        let expected_output = DestReady(HexNumber(16));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_operand_indirect_normal(){
        let (actual_input_remaining, actual_output) = operand("4(%rax)").unwrap();
        let expected_output = DestReady(Indirect(OffsetNum(4), RaxD));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }
    
    #[test]
    fn test_operand_label_normal(){
        let (actual_input_remaining, actual_output) = operand("loop").unwrap();
        let expected_output = DestReady(Label("loop".to_string()));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }


    #[test]
    fn test_inst_op_dop_normal(){
        let (actual_input_remaining, actual_output) = inst_op_dop("addq").unwrap();
        let expected_output = AddD;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_inst_op_normal(){
        let (actual_input_remaining, actual_output) = inst_op("mulq").unwrap();
        let expected_output = MulD;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_inst_dop_normal(){
        let (actual_input_remaining, actual_output) = inst_dop("notq").unwrap();
        let expected_output = NotD;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_inst_jump_normal(){
        let (actual_input_remaining, actual_output) = inst_jump("jmp").unwrap();
        let expected_output = JmpD;
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }


    #[test]
    fn test_line_op_dop_imm_to_reg(){
        let (actual_input_remaining, actual_output) = line_op_dop("addq $4, %rax").unwrap();
        let expected_output = LineOpDop(AddD, Immediate(4), Register(RaxD));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_line_op_dop_reg_to_reg(){
        let (actual_input_remaining, actual_output) = line_op_dop("addq %rbx, %rax").unwrap();
        let expected_output = LineOpDop(AddD, DestReady(Register(RbxD)), Register(RaxD));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_line_op_imm(){
        let (actual_input_remaining, actual_output) = line_op("mulq $4").unwrap();
        let expected_output = LineOp(MulD, Immediate(4));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_line_dop_reg(){
        let (actual_input_remaining, actual_output) = line_dop("notq %rax").unwrap();
        let expected_output = LineDop(NotD, Register(RaxD));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_line_jump_normal(){
        let (actual_input_remaining, actual_output) = line_jmp("jmp loop").unwrap();
        let expected_output = LineJump(JmpD, "loop".to_string());
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_test_line_normal(){
        let (actual_input_remaining, actual_output) = line("movq $3, %rax").unwrap();
        let expected_output = Line("".to_string(), LineOpDop(MovD, Immediate(3), Register(RaxD)));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_test_line_with_negative(){
        let (actual_input_remaining, actual_output) = line("movq $-3, %rax").unwrap();
        let expected_output = Line("".to_string(), LineOpDop(MovD, Immediate(-3), Register(RaxD)));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_test_line_with_label(){
        let (actual_input_remaining, actual_output) = line("loop: movq $3, %rax").unwrap();
        let expected_output = Line("loop".to_string(), LineOpDop(MovD, Immediate(3), Register(RaxD)));
        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_test_lines_2_lines(){
        let (actual_input_remaining, actual_output) = lines("movq $3, %rax movq $3, %rax").unwrap();
        let expected_output = vec![
            Line("".to_string(), LineOpDop(MovD, Immediate(3), Register(RaxD))),
            Line("".to_string(), LineOpDop(MovD, Immediate(3), Register(RaxD))),
        ];

        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }

    #[test]
    fn test_test_lines_many_with_label(){
        let (actual_input_remaining, actual_output) = lines("total: movq $-3, %rsi addq $4, %rax addq $4, %rbx addq $4, %rcx jmp total").unwrap();
        let expected_output = vec![
            Line("total".to_string(), LineOpDop(MovD, Immediate(-3), Register(RsiD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RaxD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RbxD))),
            Line("".to_string(), LineOpDop(AddD, Immediate(4), Register(RcxD))),
            Line("".to_string(), LineJump(JmpD, "total".to_string())),
        ];

        let expected_input_remaining = "";
        
        assert_eq!(actual_output, expected_output);
        assert_eq!(actual_input_remaining, expected_input_remaining);
    }




}
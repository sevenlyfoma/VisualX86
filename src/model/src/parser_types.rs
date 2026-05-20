#[derive(Debug)]
#[derive(PartialEq)]
pub enum LineT{
    Line(String, LineBodyT),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LineBodyT{
    LineOpDop(InstOpDopT, OperandT, DestReadyOperandT),
    LineOp(InstOpT, OperandT),
    LineDop(InstDopT, DestReadyOperandT),
    LineJump(InstJumpT, String),
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstOpDopT{
    MovD,
    AddD,
    SubD,
    CmpD,
    AndD,
    OrD,
    XorD,
    SalD,
    ShlD,
    SarD,
    ShrD,
}
impl InstOpDopT {
    pub fn enum_index(&self) -> u8 {
        match *self {
            InstOpDopT::MovD => 0,
            InstOpDopT::AddD => 1,
            InstOpDopT::SubD => 2,
            InstOpDopT::CmpD => 7,
            InstOpDopT::AndD => 8,
            InstOpDopT::OrD  => 9,
            InstOpDopT::XorD => 10,
            InstOpDopT::SalD => 12,
            InstOpDopT::ShlD => 14,
            InstOpDopT::SarD => 13,
            InstOpDopT::ShrD => 15,
        }
    }
}




#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstOpT{
    MulD,
    ImulD,
    DivD,
    IdivD,
}
impl InstOpT {
    pub fn enum_index(&self) -> u8 {
        match *self {
            InstOpT::MulD => 3,
            InstOpT::ImulD => 5,
            InstOpT::DivD => 4,
            InstOpT::IdivD => 6,
        }
    }
}



#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstDopT{
    NotD,
}
impl InstDopT {
    pub fn enum_index(&self) -> u8 {
        match *self {
            InstDopT::NotD => 11,
        }
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstJumpT{
    JmpD,
    JzD,
    JcD,
    JoD,
    JsD,
}
impl InstJumpT {
    pub fn enum_index(&self) -> u8 {
        match *self {
            InstJumpT::JmpD => 16,
            InstJumpT::JzD  => 17,
            InstJumpT::JcD  => 18,
            InstJumpT::JoD  => 19,
            InstJumpT::JsD  => 20,
        }
    }
}




#[derive(Debug)]
#[derive(PartialEq)]
pub enum OperandT{
    Immediate(i64),
    DestReady(DestReadyOperandT),
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum DestReadyOperandT{
    Register(RegisterT),
    HexNumber(u64),
    Indirect(OffsetT, RegisterT),
    Label(String),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OffsetT{
   OffsetNum(i64),
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum RegisterT{
    RaxD,
    RbxD,
    RcxD,
    RdxD,
    RbpD,
    RspD,
    RsiD,
    RdiD,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D, 
    R15D,
}
impl RegisterT {
    pub fn enum_index(&self) -> u8 {
        match *self {
            RegisterT::RaxD => 0,
            RegisterT::RbxD => 1,
            RegisterT::RcxD => 2,
            RegisterT::RdxD => 3,
            RegisterT::RsiD => 4,
            RegisterT::RdiD => 5,
            RegisterT::RbpD => 6,
            RegisterT::RspD => 7,
            
            RegisterT::R8D  => 8,
            RegisterT::R9D  => 9,
            RegisterT::R10D => 10,
            RegisterT::R11D => 11,
            RegisterT::R12D => 12,
            RegisterT::R13D => 13,
            RegisterT::R14D => 14,
            RegisterT::R15D => 15,
        }
    }
}
use std::fmt;

pub enum OpCodes {
    LoadVal,
    WriteVar,
    ReadVar,
    Add,
    Multiply,
    ReturnValue,
    TestLessThan,
    TestMoreThan,
    TestEqualsTo,
    TestDifferentFrom,
    JumpIfFalse,
    JumpLoop,
}


impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCodes::LoadVal => write!(f, "0"),
            OpCodes::WriteVar => write!(f, "1"),
            OpCodes::ReadVar => write!(f, "2"),
            OpCodes::Add => write!(f, "3"),
            OpCodes::Multiply => write!(f, "4"),
            OpCodes::ReturnValue => write!(f, "5"),
            OpCodes::TestLessThan => write!(f, "6"),
            OpCodes::TestMoreThan => write!(f, "7"),
            OpCodes::TestEqualsTo => write!(f, "8"),
            OpCodes::TestDifferentFrom => write!(f, "9"),
            OpCodes::JumpIfFalse => write!(f, "a"),
            OpCodes::JumpLoop => write!(f, "b")
        }
    }
}

impl fmt::Debug for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCodes::LoadVal => write!(f, "0"),
            OpCodes::WriteVar => write!(f, "1"),
            OpCodes::ReadVar => write!(f, "2"),
            OpCodes::Add => write!(f, "3"),
            OpCodes::Multiply => write!(f, "4"),
            OpCodes::ReturnValue => write!(f, "5"),
            OpCodes::TestLessThan => write!(f, "6"),
            OpCodes::TestMoreThan => write!(f, "7"),
            OpCodes::TestEqualsTo => write!(f, "8"),
            OpCodes::TestDifferentFrom => write!(f, "9"),
            OpCodes::JumpIfFalse => write!(f, "a"),
            OpCodes::JumpLoop => write!(f, "b")
        }
    }
}
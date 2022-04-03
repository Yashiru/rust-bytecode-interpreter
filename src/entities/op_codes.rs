use std::fmt;

pub enum OpCodes {
    LoadVal,
    WriteVar,
    ReadVar,
    Add,
    Multiply,
    ReturnValue,
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
       }
    }
}
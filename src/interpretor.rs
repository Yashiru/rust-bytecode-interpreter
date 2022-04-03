#[path = "entities/byteCode.rs"] mod byteCode;

enum OperatorArgument {
    string_value(String),
    float_value(f64)
}

enum Instruction {
    op_name(String),
    op_arg(OperatorArgument)
}

pub struct Interpretor {
    pub instruction_pointer: u32,
    pub instructions: Vec<(String, f64 | String)>
}

impl Interpretor{
    pub fn new() -> Interpretor {
        Interpretor {
            instruction_pointer: 0,
            instructions: [].to_vec()
        }
    }

    pub fn interpret(&self, instructions: Vec<Instruction>) {
    }
}
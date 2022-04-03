#[path = "entities/byte_code.rs"] mod byte_code;

pub struct Interpretor {
    pub instruction_pointer: u32,
    pub instructions: Vec<(String, String)>,
    pub byte_code: byte_code::ByteCode
}

impl Interpretor{
    fn process_instruction(&self, instruction_index: usize){
        println!("{:?}", self.instructions)
    }

    pub fn new(instructions: Vec<(String, String)>) -> Interpretor {
        Interpretor {
            instruction_pointer: 0,
            instructions: vec![],
            byte_code: byte_code::ByteCode::new([].to_vec())
        }
    }

    pub fn interpret(&mut self) {
        for i in 0..self.instructions.len() {
            self.process_instruction(i)
        }
    }
}
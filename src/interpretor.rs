#[path = "entities/byte_code.rs"] mod byte_code;

pub struct Interpretor {
    pub instruction_pointer: u32,
    pub instructions: Vec<(String, String)>,
    pub byte_code: byte_code::ByteCode
}

impl Interpretor{
    fn process_instruction(&mut self, instruction_index: usize){
        let mut oparg: &str = &self.instructions[instruction_index].1[..];
        match &self.instructions[instruction_index].0[..] {
            "LOAD_VAL"  => self.byte_code.load_val(oparg),
            "WRITE_VAR" => self.byte_code.write_var(oparg),
            "READ_VAR" => self.byte_code.read_var(oparg),
            "ADD" => self.byte_code.add(),
            "MULTIPLY" => self.byte_code.multiply(),
            "RETURN_VALUE" => {
                println!("Computed return value: {}", self.byte_code.return_value())
            },
            _ => println!("Unrecognized opname")
        }
    }

    pub fn new(instructions: Vec<(String, String)>) -> Interpretor {
        Interpretor {
            instruction_pointer: 0,
            instructions: instructions,
            byte_code: byte_code::ByteCode::new([].to_vec())
        }
    }

    pub fn interpret(&mut self) {
        for i in 0..self.instructions.len() {
            self.process_instruction(i)
        }
    }
}
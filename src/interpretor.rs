#[path = "entities/byte_code.rs"] mod byte_code;

pub struct Interpretor {
    pub instruction_pointer: u32,
    pub instructions: Vec<(String, String)>,
    pub byte_code: byte_code::ByteCode
}

impl Interpretor{
    fn process_instruction(&mut self, instruction_index: usize){
        let oparg: &str = &self.instructions[instruction_index].1[..];
        match &self.instructions[instruction_index].0[..] {
            "LOAD_VAL"  => self.byte_code.load_val(oparg),
            "WRITE_VAR" => self.byte_code.write_var(oparg),
            "READ_VAR" => self.byte_code.read_var(oparg),
            "ADD" => self.byte_code.add(),
            "MULTIPLY" => self.byte_code.multiply(),
            "RETURN_VALUE" => {
                println!(
                    "================================\n\x1b[36mValue returned from the stack: {}\x1b[0m\n================================", 
                    self.byte_code.return_value()
                )
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
            self.instruction_pointer = self.instruction_pointer + 1;
            self.process_instruction(i)
        }
    }
}

pub fn text_to_operations(text: &str) -> Vec<(String, String)>{
    let mut operations: Vec<(String, String)> = vec![];
    let full_operations: Vec<&str> = text.trim().split("\n").collect();
    
    for i in 0..full_operations.len() {
        let full_operation = 
            full_operations[i]
                .replace("\t", "")
                .replace("  ", "")
                .replace("'", "");
        let mut operation: Vec<&str> = full_operation.trim().split(" ").collect();
        if operation.len() == 1 {
            operation.push("")
        }
        operations.push(
            (
                String::from(operation[0]),
                String::from(operation[1])
            )
        );
    }

    operations
}
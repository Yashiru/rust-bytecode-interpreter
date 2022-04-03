#[path = "entities/byte_code.rs"] mod byte_code;
#[path = "entities/op_codes.rs"] mod op_codes;

pub struct Interpretor {
    pub instruction_pointer: u32,
    pub instructions: Vec<(op_codes::OpCodes, String)>,
    pub byte_code: byte_code::ByteCode
}

impl Interpretor{
    fn process_instruction(&mut self, instruction_index: usize){
        let oparg: &str = &self.instructions[instruction_index].1[..];

        match self.instructions[instruction_index].0 {
            op_codes::OpCodes::LoadVal => self.byte_code.load_val(oparg),
            op_codes::OpCodes::WriteVar => self.byte_code.write_var(oparg),
            op_codes::OpCodes::ReadVar => self.byte_code.read_var(oparg),
            op_codes::OpCodes::Add => self.byte_code.add(),
            op_codes::OpCodes::Multiply => self.byte_code.multiply(),
            op_codes::OpCodes::ReturnValue => {
                println!(
                    "================================\n\x1b[36mValue returned from the stack: {}\x1b[0m\n================================", 
                    self.byte_code.return_value()
                )
            }
        }
    }

    pub fn new(instructions: Vec<(op_codes::OpCodes, String)>) -> Interpretor {
        Interpretor {
            instruction_pointer: 0,
            instructions: instructions,
            byte_code: byte_code::ByteCode::new([].to_vec())
        }

    }

    pub fn interpret(&mut self) {
        self.instruction_pointer = 0;
        for _i in 0..self.instructions.len() {
            self.process_instruction(self.instruction_pointer.try_into().unwrap());
            self.instruction_pointer = self.instruction_pointer + 1;
        }
    }
}

pub fn text_to_operations(text: &str) -> Vec<(op_codes::OpCodes, String)>{
    let mut operations: Vec<(op_codes::OpCodes, String)> = vec![];
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
        
        let op_code;
        match operation[0] {
            "LOAD_VAL" => op_code = op_codes::OpCodes::LoadVal,
            "WRITE_VAR" => op_code = op_codes::OpCodes::WriteVar,
            "READ_VAR" => op_code = op_codes::OpCodes::ReadVar,
            "ADD" => op_code = op_codes::OpCodes::Add,
            "MULTIPLY" => op_code = op_codes::OpCodes::Multiply,
            "RETURN_VALUE" => op_code = op_codes::OpCodes::ReturnValue,
            _ => panic!("Unrecognized opname")
        }


        operations.push(
            (
                op_code,
                String::from(operation[1])
            )
        );

    }

    operations
}
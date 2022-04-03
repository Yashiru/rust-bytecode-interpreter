pub struct ByteCode {
    pub stack: Vec<f64>,
    pub environment: Vec<(String, f64)>
}

impl ByteCode {
    fn is_empty(&self) {
        self.stack.len();
    }

    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn new(stack: Vec<f64>) -> ByteCode {
        ByteCode { 
            stack: stack,
            environment: [].to_vec()
        }
    }

    pub fn LOAD_VAL(&mut self, value: f64){
        self.stack.push(value);
    }

    pub fn WRITE_VAR(&mut self, var_name: String){
        let value: f64 = self.stack[self.stack.len()-1];
        let index = self.environment
            .iter()
            .position(|r| r.0 == var_name.clone())
            .unwrap_or(
                std::u64::MAX.try_into().unwrap()
            );

        if index == std::u64::MAX.try_into().unwrap() {
            self.environment.push((var_name, value));
            println!("new variable {} = {}", self.environment[self.environment.len()-1].0, self.environment[self.environment.len()-1].1)
        } 
        else {
            self.environment[index].1 = value;
            println!("Assign variable {} = {}", self.environment[self.environment.len()-1].0, self.environment[self.environment.len()-1].1)
        }
        self.pop();
    }

    pub fn READ_VAR(&mut self, var_name: String){
        let index = self.environment
            .iter()
            .position(|r| r.0 == var_name.clone())
            .unwrap_or(
                std::u64::MAX.try_into().unwrap()
            );

        assert!(index != std::u64::MAX.try_into().unwrap(), "{} not found in this scope", var_name);

        self.push(self.environment[index].1);
    }

    pub fn MULTIPLY(&mut self){
        let mut multiplication_result: f64 = 1.0;
        let _collector: Vec<_> = self.stack.iter().map(|x| multiplication_result *= x).collect();
        self.stack = [].to_vec();
        self.push(multiplication_result);
    }
}

pub struct ByteCode {
    pub stack: Vec<f64>,
    pub environment: Vec<(String, f64)>
}

impl ByteCode {
    fn is_empty(&self) -> bool{
        self.stack.len() == 0
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

    pub fn load_val(&mut self, value: &str){
        self.stack.push(value.parse::<f64>().unwrap());
    }

    pub fn write_var(&mut self, var_name: &str){
        assert!(self.is_empty() == false, "stack is empty");
        let value: f64 = self.stack[self.stack.len()-1];
        let index = self.environment
            .iter()
            .position(|r| r.0 == var_name.clone())
            .unwrap_or(
                std::u64::MAX.try_into().unwrap()
            );

        if index == std::u64::MAX.try_into().unwrap() {
            self.environment.push((var_name.to_string(), value));
        } 
        else {
            self.environment[index].1 = value;
        }
        self.pop();
    }

    pub fn read_var(&mut self, var_name: &str){
        let index = self.environment
            .iter()
            .position(|r| r.0 == var_name.clone())
            .unwrap_or(
                std::u64::MAX.try_into().unwrap()
            );

        assert!(index != std::u64::MAX.try_into().unwrap(), "{} not found in this scope", var_name);

        self.push(self.environment[index].1);
    }

    pub fn multiply(&mut self){
        let mut multiplication_result: f64 = 1.0;
        let _collector: Vec<_> = self.stack.iter().map(|x| multiplication_result *= x).collect();
        self.stack = [].to_vec();
        self.push(multiplication_result);
    }

    pub fn add(&mut self){
        let mut addition_result: f64 = 0.0;
        let _collector: Vec<_> = self.stack.iter().map(|x| addition_result += x).collect();
        self.stack = [].to_vec();
        self.push(addition_result);
    }

    pub fn return_value(&self) -> f64 {
        self.stack[self.stack.len()-1]
    }
}

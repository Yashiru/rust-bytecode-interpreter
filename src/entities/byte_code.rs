pub struct ByteCode {
    pub stack: Vec<f64>,
    pub environment: Vec<(String, f64)>
}

impl ByteCode {

    /**
     * Basic stack manipulations functions
     */

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


    /**
     * Stack values manipulations
     */

    pub fn load_val(&mut self, value: &str){
        self.stack.push(value.parse::<f64>().unwrap());
    }

    pub fn multiply(&mut self){
        let value1: f64 = self.stack[self.stack.len()-1];
        self.pop();
        let value2: f64 = self.stack[self.stack.len()-1];
        self.pop();
        self.stack = [].to_vec();
        let multiplication_result: f64 = value1 * value2;
        self.push(multiplication_result);
    }

    pub fn add(&mut self){
        let value1: f64 = self.stack[self.stack.len()-1];
        self.pop();
        let value2: f64 = self.stack[self.stack.len()-1];
        self.pop();
        self.stack = [].to_vec();
        let addition_result: f64 = value1 + value2;
        self.push(addition_result);
    }


    /**
     * Interaction between environment and stack
     */

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


    /**
     * Test the stack
     */

    pub fn test_less_than(&mut self, value_to_compare: &str){
        let value: f64 = self.stack[self.stack.len()-1];
        let less_than: bool = value < value_to_compare.parse::<f64>().unwrap();
        if less_than == true {
            self.push(1.0);
        }
        else {
            self.push(0.0);
        }
    }

    pub fn test_more_than(&mut self, value_to_compare: &str){
        let value: f64 = self.stack[self.stack.len()-1];
        let more_than: bool = value > value_to_compare.parse::<f64>().unwrap();
        if more_than == true {
            self.push(1.0);
        }
        else {
            self.push(0.0);
        }
    }

    pub fn test_equals_to(&mut self, value_to_compare: &str){
        let value: f64 = self.stack[self.stack.len()-1];
        let equals_to: bool = value == value_to_compare.parse::<f64>().unwrap();
        if equals_to == true {
            self.push(1.0);
        }
        else {
            self.push(0.0);
        }
    }

    pub fn test_different_from(&mut self, value_to_compare: &str){
        let value: f64 = self.stack[self.stack.len()-1];
        let different_from: bool = value != value_to_compare.parse::<f64>().unwrap();
        if different_from == true {
            self.push(1.0);
        }
        else {
            self.push(0.0);
        }
    }


    /**
     * Check test result
     */

    pub fn is_false(&mut self) -> bool {
        let is_false: bool = self.stack[self.stack.len()-1] == 0.0;
        self.pop(); // pop boolean value coming from test function
        self.pop(); // pop tested value
        is_false
    }


    /**
     * Return value as a result
     */

    pub fn return_value(&mut self) -> f64 {
        let return_value = self.stack[self.stack.len()-1];
        self.stack = [].to_vec();
        return_value
    }
}

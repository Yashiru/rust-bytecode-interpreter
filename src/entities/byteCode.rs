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
}

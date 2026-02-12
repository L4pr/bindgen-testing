pub mod ffi;

pub struct Calculator {
    pub owner: String,
    pub current_value: i32,
}

impl Calculator {
    // A standard Rust constructor
    pub fn new(owner: &str, start_val: i32) -> Self {
        Calculator {
            owner: owner.to_string(),
            current_value: start_val,
        }
    }

    // A method that modifies state
    pub fn add(&mut self, amount: i32) {
        self.current_value += amount;
    }

    // A method that returns a complex string
    pub fn get_summary(&self) -> String {
        format!("Owner: {}, Value: {}", self.owner, self.current_value)
    }
}
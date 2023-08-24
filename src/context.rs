
pub struct Context {
    stack_size: u64,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
        }
    }

    pub fn push<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size += 1;
        format!("    push {}\n", Into::<String>::into(value))
    }

    pub fn pop<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size -= 1;
        format!("    pop {}\n", Into::<String>::into(value))
    }
}


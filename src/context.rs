use std::collections::HashMap;


pub struct Context {
    stack_size: u64,
    variables: HashMap<String, u64>,
    label_counts: HashMap<String, u64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
            variables: HashMap::new(),
            label_counts: HashMap::new(),
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

    pub fn declare_variable(&mut self, identifier: String) {
        self.variables.insert(identifier, self.stack_size);
    }

    pub fn get_variable(&mut self, identifier: &String) -> Option<String> {
        match self.variables.get(identifier) {
            Some(offset) => Some(
                self.push(format!("qword [rsp + {}]", (self.stack_size - offset) * 8))
            ),
            None => None,
        }
    }

    pub fn create_label<S: Into<String>>(&mut self, tag: S) -> String {
        let tag: String = tag.into();
        let entry = self.label_counts.entry(tag.clone()).or_insert(0);
        let index = *entry;
        *entry += 1;
        format!("{tag}{index:08x}")
    }
}


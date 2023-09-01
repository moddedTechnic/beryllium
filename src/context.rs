use std::collections::HashMap;

use crate::iter::Reversed;


#[derive(Clone, Debug, Default)]
pub struct VariableFrame {
    stack_size: u64, variables: HashMap<String, u64>, }

impl VariableFrame {
    pub fn with_size(size: u64) -> Self {
        Self {
            stack_size: size,
            variables: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableStack(Vec<VariableFrame>);

impl VariableStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, frame: VariableFrame) -> &mut VariableFrame {
        self.0.push(frame);
        self.0.get_mut(0).unwrap()
    }

    pub fn pop(&mut self) -> Option<VariableFrame> {
        self.0.pop()
    }

    pub fn peek(&mut self) -> Option<&mut VariableFrame> {
        self.0.get_mut(0)
    }

    pub fn declare_variable(&mut self, name: String) {
        match self.peek() {
            Some(frame) => Some(frame),
            None => Some(self.push(VariableFrame::default())),
        }.map(|frame| frame.variables.insert(name, frame.stack_size));
    }

    pub fn get_offset(&mut self, name: &String) -> Option<u64> {
        let mut offset = 0;
        for frame in self.0.reversed() {
            match frame.variables.get(name) {
                Some(off) => return Some(frame.stack_size - off + offset),
                None => offset += frame.stack_size,
            }
        };
        None
    }
}


#[derive(Clone, Debug)]
pub struct Context {
    stack_size: u64,
    variables: VariableStack,
    label_counts: HashMap<String, u64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
            variables: VariableStack::new(),
            label_counts: HashMap::new(),
        }
    }

    pub fn push<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size += 1;
        match self.variables.peek() {
            Some(mut frame) => frame.stack_size += 1,
            None => { self.variables.push(VariableFrame::with_size(1)); },
        }
        format!("    push {}\n", Into::<String>::into(value))
    }

    pub fn pop<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size -= 1;
        self.variables.peek().expect("trying to pop from empty stack").stack_size -= 1;
        format!("    pop {}\n", Into::<String>::into(value))
    }

    pub fn declare_variable(&mut self, identifier: String) {
        self.variables.declare_variable(identifier)
    }

    pub fn get_variable(&mut self, identifier: &String) -> Option<String> {
        self.variables.get_offset(identifier).map(|offset| {
            self.push(format!("qword [rsp + {}]", offset * 8))
        })
    }

    pub fn create_label<S: Into<String>>(&mut self, tag: S) -> String {
        let tag: String = tag.into();
        let entry = self.label_counts.entry(tag.clone()).or_insert(0);
        let index = *entry;
        *entry += 1;
        format!("{tag}{index:08x}")
    }

    pub fn enter(&mut self) -> String {
        self.variables.push(VariableFrame::default());
        String::new()
    }

    pub fn exit(&mut self) -> String {
        let frame = self.variables.pop().expect("trying to exit from base frame");
        format!("    add rsp, {}\n", frame.stack_size * 8 + 8)
    }
}


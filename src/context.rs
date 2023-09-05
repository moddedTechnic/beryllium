use std::collections::HashMap;

use crate::{iter::Reversed, codegen::CodegenError};


#[derive(Clone, Debug, Default)]
pub struct VariableMeta {
    stack_frame_offset: u64,
    is_mutable: bool,
}


#[derive(Clone, Debug, Default)]
pub struct VariableFrame {
    stack_size: u64,
    variables: HashMap<String, VariableMeta>,
}

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
        let last_index = match self.0.len() {
            0 => return None,
            l => l - 1,
        };
        self.0.get_mut(last_index)
    }

    pub fn declare_variable(&mut self, name: String, is_mutable: bool) {
        match self.peek() {
            Some(frame) => Some(frame),
            None => Some(self.push(VariableFrame::default())),
        }.map(|frame|
            frame.variables.insert(
                name,
                VariableMeta { stack_frame_offset: frame.stack_size, is_mutable },
            )
        );
    }

    pub fn get_offset(&mut self, name: &String) -> Option<u64> {
        let mut offset = 0;
        for frame in self.0.reversed() {
            match frame.variables.get(name) {
                Some(meta) => return Some(frame.stack_size - meta.stack_frame_offset + offset),
                None => offset += frame.stack_size,
            }
        };
        None
    }

    pub fn is_mutable(&mut self, name: &String) -> Option<bool> {
        for frame in self.0.reversed() {
            if let Some(meta) = frame.variables.get(name) {
                return Some(meta.is_mutable)
            }
        };
        None
    }
}


#[derive(Clone, Debug)]
pub struct LabelFrame {
    pub start: String,
    pub end: String,
}

impl From<(String, String)> for LabelFrame {
    fn from((start, end): (String, String)) -> Self {
        Self { start, end }
    }
}


#[derive(Clone, Debug)]
pub struct Context {
    stack_size: u64,
    variables: VariableStack,
    label_counts: HashMap<String, u64>,
    label_stack: Vec<LabelFrame>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
            variables: VariableStack::new(),
            label_counts: HashMap::new(),
            label_stack: Vec::new(),
        }
    }

    pub fn push<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size += 1;
        match self.variables.peek() {
            Some(frame) => frame.stack_size += 1,
            None => { self.variables.push(VariableFrame::with_size(1)); },
        }
        format!("    push {}\n", Into::<String>::into(value))
    }

    pub fn pop<S: Into<String>>(&mut self, value: S) -> String {
        self.stack_size -= 1;
        self.variables.peek().expect("trying to pop from empty stack").stack_size -= 1;
        format!("    pop {}\n", Into::<String>::into(value))
    }

    pub fn declare_variable(&mut self, identifier: String, is_mutable: bool) {
        self.variables.declare_variable(identifier, is_mutable)
    }

    pub fn get_variable(&mut self, identifier: &String) -> Option<String> {
        self.variables.get_offset(identifier).map(|offset| {
            self.push(format!("qword [rsp + {}]", offset * 8))
        })
    }

    pub fn set_variable(&mut self, identifier: &String, value: impl Into<String>) -> Result<String, CodegenError> {
        if !self.variables.is_mutable(identifier)
                .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))? {
            return Err(CodegenError::ChangedImmutableVariable(identifier.clone()));
        }
        self.variables.get_offset(identifier)
            .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))
            .map(|offset| {
                format!("    mov qword [rsp + {}], {}\n", offset * 8, Into::<String>::into(value))
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
        format!("    add rsp, {}\n", frame.stack_size * 8)
    }

    pub fn enter_labelled_region(&mut self, frame: impl Into<LabelFrame>) {
        self.label_stack.push(frame.into())
    }

    pub fn exit_labelled_region(&mut self) -> Option<LabelFrame> {
        self.label_stack.pop()
    }

    pub fn get_labelled_region(&mut self) -> Option<LabelFrame> {
        let last_index = match self.label_stack.len() {
            0 => return None,
            l => l - 1,
        };
        self.label_stack.get(last_index).cloned()
    }
}


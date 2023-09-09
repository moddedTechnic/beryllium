use std::collections::HashMap;

use crate::ast;


#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
}


#[derive(Clone, Debug)]
pub struct Function {
    pub params: Vec<Param>,
}


#[derive(Clone, Debug)]
pub struct TypeRegistry {
    functions: HashMap<String, Function>,
}

impl TypeRegistry {
    pub fn get_function(&self, name: impl Into<String>) -> Option<&Function> {
        self.functions.get(&name.into())
    }
}

impl From<&ast::Program> for TypeRegistry {
    fn from(program: &ast::Program) -> Self {
        let mut registry = Self { functions: HashMap::new() };
        program.register_types(&mut registry);
        registry
    }
}


trait TypeHolder {
    fn register_types(&self, registry: &mut TypeRegistry);
}

impl TypeHolder for ast::Program {
    fn register_types(&self, registry: &mut TypeRegistry) {
        self.0.iter().for_each(|item| item.register_types(registry));
    }
}

impl TypeHolder for ast::Item {
    fn register_types(&self, registry: &mut TypeRegistry) {
        match self {
            Self::Function { name, params, body: _ } => registry.functions.insert(
                name.clone(),
                Function {
                    params: params.iter()
                          .map(|param| Param { name: param.name.clone() })
                          .collect()
                }
            ),
        };
    }
}


use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug,PartialEq)]
pub(crate) struct Environment {
    bindings: HashMap<String,Value>,
}

impl Environment {
    pub(crate) fn store_definition(&mut self, name:String, value: Value) {
        self.bindings.insert(name, value);
    }
}
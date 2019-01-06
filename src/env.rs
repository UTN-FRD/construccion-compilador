use std::collections::HashMap;
use std::rc::Rc;

use intrinsics;
use lisp_value::LispValue;

pub type Map = HashMap<String, Rc<LispValue>>;

pub struct Env {
    parent: Option<Rc<Env>>,
    env: Map,
}

impl Env {
    pub fn new_global() -> Env {
        let parent = None;
        let mut env = HashMap::new();
        env.insert(
            "+".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::add)),
        );

        Env { parent, env }
    }

    pub fn new(parent: Rc<Env>, env: Map) -> Env {
        Env {
            parent: Some(parent),
            env,
        }
    }

    pub fn get(&self, key: &String) -> Option<Rc<LispValue>> {
        let value = self.env.get(key);
        if value.is_some() {
            return Some(value.unwrap().clone());
        }

        if self.parent.is_none() {
            return None;
        }

        self.parent.as_ref().unwrap().get(key)
    }

    // TODO Maybe?
    //pub fn new_scope(parent: Rc<Map>, keys: Vec<String>, values: Vec<LispValue>)
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::intrinsics;
use crate::lisp_value::LispValue;

/// Este alias de tipo sirve para escribir de forma abreviada el tipo
/// de datos que representa la asociacion entre:
/// - identificadores (nombres de funciones o variables), representados con un `String`.
/// - valores del lenguaje, representados con un `Rc<LispValue>`, el Rc para poder acceder a ellos
///   multiples veces.
///
/// Ejemplo:
/// 
/// ```{ "x": Rc::new(LispValue::Number(3.2)),
///   "nombre": Rc::new(LispValue::StringValue("pedro")),
///   "+": Rc::new(LispValue::Intrinsic(intrinsics::add))
/// }
pub type Map = HashMap<String, Rc<LispValue>>;

/// El tipo de dato `Env` representa un ambiente en el cual
/// se produce la evaluacion de codigo.
/// Sus componentes son:
/// * Un ambiente raiz `root`.
/// * Un ambiente padre `parent`
/// * Un campo `env` que contiene un `Map` con todas las asociaciones entre
///   variables y valores.
/// 
/// La naturaleza anidada de este tipo de datos se debe a que al momento de la 
/// evaluacion existen diferentes ambientes: 
/// 
/// - Ambiente global: contiene todas las variables y funciones definidas junto a las operaciones elementales del lenguaje).
/// - Ambiente local de una funcion.
/// - Ambiente local de una expresion. 
#[derive(Debug)]
pub struct Env {
    root: Option<Rc<Env>>,
    parent: Option<Rc<Env>>,
    env: RefCell<Map>,
}

impl Env {
    /// Crea un ambiente global, este ambiente posee las operaciones aritmeticas
    /// elementales y los operadores de comparacion.
    pub fn new_global() -> Env {
        let mut env = HashMap::new();
        
        // Agrego las operaciones elementales al ambiente global
        env.insert(
            "+".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::add)),
        );
        env.insert(
            "-".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::sub)),
        );
        env.insert(
            "=".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::eq)),
        );
        env.insert(
            ">".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::gt)),
        );
        env.insert(
            "<".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::lt)),
        );

        Env {
            root: None,
            parent: None,
            env: RefCell::new(env),
        }
    }

    /// Crea un nuevo `Env` a partir de un ambiente padre y un `Map`.
    pub fn new(&self, parent: Rc<Env>, env: Map) -> Env {
        Env {
            root: self.root.clone(),
            parent: Some(parent),
            env: RefCell::new(env),
        }
    }

    /// Obtiene un valor del ambiente a partir de la clave asociada a ese valor.
    /// Para esto primero se fija en el `Map` del ambiente y si no se encuentra
    /// alli busca la misma clave en el ambiente padre.
    pub fn get(&self, key: &str) -> Option<Rc<LispValue>> {
        {
            let env = self.env.borrow();
            let value = env.get(key);
            if let Some(lisp_value) = value {
                return Some(lisp_value.clone());
            }
        }

        match self.parent.as_ref() {
            Some(p) => p.get(key),
            None => None,
        }
    }

    /// Agrega un nuevo valor al ambiente bajo la clave especificada.
    pub fn set(&self, key: String, value: Rc<LispValue>) {
        self.env.borrow_mut().insert(key, value);
    }
}

use std::fmt;

#[derive(Clone)]
pub enum LispType {
    None,
    Num(f64),
    Fn(fn(&Vec<LispType>) -> LispType),
}

impl LispType {
    pub fn unwrap_number(&self) -> &f64 {
        match self {
            LispType::Num(ref num) => return num,
            _ => panic!("BBBB"),
        }
    }
    pub fn unwrap_fn(self) -> fn(&Vec<LispType>) -> LispType {
        match self {
            LispType::Fn(function) => return function,
            _ => panic!("BBBB"),
        }
    }
}

impl fmt::Debug for LispType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispType::None => write!(f, "Nill"),
            LispType::Fn(_) => write!(f, "Fn"),
            LispType::Num(num) => write!(f, "{}", num),
            _ => panic!("asdasd"),
        }
    }
}

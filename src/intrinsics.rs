use lisp_type::LispType;

pub fn add(arguments: &Vec<LispType>) -> LispType {
    let res = arguments
        .iter()
        .fold(0f64, |acc, x| acc + x.unwrap_number());

    return LispType::Num(res);
}

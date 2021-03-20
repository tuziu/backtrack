use crate::algo::EnTy;
use crate::algo::StateVal;
use crate::algo::Variable;

pub fn is_valid(e: &Variable, s: &Variable, v2: EnTy, s2: EnTy) -> bool {
    let v1 = e.get_state();
    let s1 = s.get_state();
    (v1.get_x() == s1.get_x()) != (v1.get_y() == s1.get_y()) && v2 == s2
}

pub fn print_row(solution: &Vec<Variable>){
    
}
use mylib::algo::{Mill, Variable};
use mylib::algo::allocate;
use mylib::algo::EnTy;


fn ele_to_str(e: (usize, &Variable)) -> String {
    let a = e.1.get_partial().unwrap();
    let z = if (e.0+1) % 9 != 0 { ""} else { "\n"};
    format!("| {}  | {}" ,  a, z)
}

fn print_solution(partial: &Vec<Variable>) {
    let mut s = String::new();
    partial
        .iter()
        .enumerate()
        .for_each(|i| s.push_str(&ele_to_str(i)));
    println!("{}", s);
}


fn main() {
    let mut mill = Mill::new();
    let a = allocate(&mut mill);
    if a {
        print_solution(mill.get_variables())
    }
}

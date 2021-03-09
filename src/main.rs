use std::cell::RefCell;

struct Variable {
    domain: Vec<i16>,
    // current: Option<i16>,
}

impl Variable {
    fn get_domain(&self) -> &Vec<i16> {
        &self.domain
    }
    // fn set_new_val(&mut self, val: i16) {
    //     self.current = Some(val);
    // }

    // fn get_val(&self) -> i16 {
    //     self.current.unwrap()
    // }

    fn new(size: usize) -> Variable {
        Variable {
            domain: (0..size).map(|i| i as i16).collect(),
        }
    }
}

pub struct Mill {
    variables: Vec<Variable>,
}

impl Mill {
    fn get_Variable(&self, pos: usize) -> &Variable {
        &self.variables[pos]
    }
    pub fn new(size: usize) -> Mill {
        Mill {
            variables: (0..size).map(|_| Variable::new(size)).collect(),
        }
    }

    fn get_domain(&self, pos: usize) -> &Vec<i16> {
        self.variables[pos].get_domain()
    }
}
fn is_oblique(left: (i16, i16), right: (i16, i16)) -> bool {
    (left.0 - right.0).abs() == (left.1 - right.1).abs()
}

fn is_horizontal(left: (i16, i16), right: (i16, i16)) -> bool {
    (left.0 != right.0) && (left.1 == right.1)
}

fn check(val: i16, pos: usize, partial: &Vec<Option<i16>>) -> bool {
    let current = (pos as i16, val);
    for i in (0..pos) {
        if let Some(v) = partial[i] {
            let local = (i as i16, v);
            if is_horizontal(current, local) || is_oblique(current, local) {
                return false;
            }
        }
    }
    true
}

fn allocate(mill: &Mill) -> bool {
    let mut a = Vec::with_capacity(mill.variables.len());
    a = (0..mill.variables.len()).map(|_| None).collect();
    let l = allocate_at(mill, 0, &mut a);
    print_solution(&a);
    l
}

fn allocate_at(mill: &Mill, pos: usize, partial: &mut Vec<Option<i16>>) -> bool {
    // println!("alloc_at {}", pos);
    if pos == mill.variables.len() {
        println!("End {}", pos);
        return true;
    }

    for i in mill.get_domain(pos) {
        // println!("i {}", i);
        partial[pos] = Some(*i);
        if check(*i, pos, partial) && allocate_at(mill, pos + 1, partial) {
            return true;
        }
    }
    false
}

fn ele_to_str(e: (usize,&Option<i16>)) -> String{
    let a = e.1.unwrap_or_default();
    format!("| {} {} | ", e.0, a)
}

fn print_solution(partial: &Vec<Option<i16>>){
    let mut s = String::new();
    partial.iter().enumerate().for_each(|i| s.push_str( &ele_to_str(i) ));
    println!("{}",s);
}

fn main() {
    let mut mill = Mill::new(29);
    let a = allocate(&mut mill);
    println!("Hello, world! {}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
            let v = (0..4).map(|_| None).collect();
            assert!(check(0, 0, &v));
            assert!(check(1, 0, &v));
            assert!(check(2, 0, &v));
            assert!(check(3, 0, &v));
    }
    #[test]
    fn check_2() {
            let mut v: Vec<Option<i16>> = (0..4).map(|_| None).collect();
            v[0] = Some(0);
            assert!(!check(0, 1, &v));
            assert!(!check(1, 1, &v));
            assert!(check(2, 1, &v));
            assert!(check(3, 1, &v));
    }
    #[test]
    fn check_3() {
            let mut v: Vec<Option<i16>> = (0..4).map(|_| None).collect();
            v[0] = Some(0);
            assert!(!check(0, 2, &v));
            assert!(check(1, 2, &v));
            assert!(!check(2, 2, &v));
            assert!(check(3, 2, &v));
    }
    #[test]
    fn check_4() {
            let mut v: Vec<Option<i16>> = (0..4).map(|_| None).collect();
            v[0] = Some(0);
            assert!(!check(0, 3, &v));
            assert!(check(1, 3, &v));
            assert!(check(2, 3, &v));
            assert!(!check(3, 3, &v));
    }
}

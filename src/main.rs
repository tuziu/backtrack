type EnTy = i32;

struct StateVal{
    x: usize,
    y: usize,
}

impl StateVal{
    fn new(x: usize,y: usize) -> StateVal{
        StateVal{x,y}
    }
}

type VarDetails = StateVal;

struct Variable {
    domain: Vec<EnTy>,
    state: VarDetails,
}

impl Variable {
    fn get_domain(&self) -> &Vec<EnTy> {
        &self.domain
    }
    fn new(x: usize,y: usize) -> Variable {
        Variable {
            domain: (0..9).map(|i| i as EnTy).collect(),
            state: StateVal::new(x, y),
        }
    }
}

pub struct Mill {
    variables: Vec<Variable>,
}

impl Mill {
    fn get_variable(&self, pos: usize) -> &Variable {
        &self.variables[pos]
    }
    pub fn new() -> Mill {
        Mill {
            variables: (0..81).map(|i| Variable::new(i%9,i/9)).collect(),
        }
    }
    fn get_domain(&self, pos: usize) -> &Vec<EnTy> {
        self.variables[pos].get_domain()
    }
}
fn is_oblique(left: (EnTy, EnTy), right: (EnTy, EnTy)) -> bool {
    (left.0 - right.0).abs() == (left.1 - right.1).abs()
}

fn is_horizontal(left: (EnTy, EnTy), right: (EnTy, EnTy)) -> bool {
    (left.0 != right.0) && (left.1 == right.1)
}

fn check(val: EnTy, pos: usize, partial: &Vec<Option<EnTy>>) -> bool {
    let current = (pos as EnTy, val);
    for i in 0..pos {
        if let Some(v) = partial[i] {
            let local = (i as EnTy, v);
            if is_horizontal(current, local) || is_oblique(current, local) {
                return false;
            }
        }
    }
    true
}

fn allocate(mill: &Mill) -> Result<Vec<EnTy>, String> {
    let mut a = (0..mill.variables.len()).map(|_| None).collect();
    let r = allocate_at(mill, 0, &mut a);
    match r {
        true => Ok(a.iter().map(|a| a.unwrap_or_default()).collect()),
        false => Err("conutation failed".to_owned()),
    }
}

fn allocate_at(mill: &Mill, pos: usize, partial: &mut Vec<Option<EnTy>>) -> bool {
    if pos == mill.variables.len() {
        return true;
    }

    for i in mill.get_domain(pos) {
        partial[pos] = Some(*i);
        if check(*i, pos, partial) && allocate_at(mill, pos + 1, partial) {
            return true;
        }
    }
    false
}

fn ele_to_str(e: (usize, &Option<EnTy>)) -> String {
    let a = e.1.unwrap_or_default();
    format!("| {} {} | ", e.0, a)
}

fn print_solution(partial: &Vec<Option<EnTy>>) {
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
        let mut v: Vec<Option<EnTy>> = (0..4).map(|_| None).collect();
        v[0] = Some(0);
        assert!(!check(0, 1, &v));
        assert!(!check(1, 1, &v));
        assert!(check(2, 1, &v));
        assert!(check(3, 1, &v));
    }
    #[test]
    fn check_3() {
        let mut v: Vec<Option<EnTy>> = (0..4).map(|_| None).collect();
        v[0] = Some(0);
        assert!(!check(0, 2, &v));
        assert!(check(1, 2, &v));
        assert!(!check(2, 2, &v));
        assert!(check(3, 2, &v));
    }
    #[test]
    fn check_4() {
        let mut v: Vec<Option<EnTy>> = (0..4).map(|_| None).collect();
        v[0] = Some(0);
        assert!(!check(0, 3, &v));
        assert!(check(1, 3, &v));
        assert!(check(2, 3, &v));
        assert!(!check(3, 3, &v));
    }
}

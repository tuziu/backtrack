use crate::sudoku::isValid;

pub type EnTy = i32;

pub struct StateVal {
    x: usize,
    y: usize,
}

impl StateVal {
    pub fn new(x: usize, y: usize) -> StateVal {
        StateVal { x, y }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
}

type VarDetails = StateVal;

pub struct Variable {
    domain: Vec<EnTy>,
    state: VarDetails,
    partial: Option<EnTy>,
}

impl Variable {
    pub fn get_domain(&self) -> &Vec<EnTy> {
        &self.domain
    }
    pub fn new(x: usize, y: usize) -> Variable {
        Variable {
            domain: (0..9).map(|i| i as EnTy).collect(),
            state: StateVal::new(x, y),
            partial: None,
        }
    }
    pub fn get_state(&self) -> &VarDetails {
        &self.state
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
            variables: (0..81).map(|i| Variable::new(i % 9, i / 9)).collect(),
        }
    }
    fn get_domain(&self, pos: usize) -> &Vec<EnTy> {
        self.variables[pos].get_domain()
    }
}

fn check(mill: &Mill, val: EnTy, pos: usize, partial: &Vec<Option<EnTy>>) -> bool {
    let current = &mill.variables[pos];
    for i in 0..pos {
        if let Some(v) = partial[i] {
            let local = &mill.variables[i];
            if isValid(current, local, val, v) {
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
        if check(mill, *i, pos, partial) && allocate_at(mill, pos + 1, partial) {
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

use std::{borrow::BorrowMut, cell::Cell};

use crate::sudoku::is_valid;

pub type EnTy = i32;

// #[macro_use]
// extern crate log;
use log::{info, trace, warn};

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
    partial: Cell<Option<EnTy>>,
}

impl Variable {
    pub fn get_domain(&self) -> &Vec<EnTy> {
        &self.domain
    }
    pub fn new(x: usize, y: usize) -> Variable {
        Variable {
            domain: (0..9).map(|i| i as EnTy).collect(),
            state: StateVal::new(x, y),
            partial: Cell::new(None),
        }
    }
    pub fn get_state(&self) -> &VarDetails {
        &self.state
    }

    pub fn set_partial(&self, partial: EnTy) {
        self.partial.set(Some(partial));
    }

    pub fn reset_partial(&self) {
        self.partial.set(None);
    }

    pub fn get_partial(&self) -> Option<EnTy> {
        self.partial.get()
    }

    pub fn set_domain_value(&mut self, val: EnTy){
        self.domain = vec![val];
    }
}

pub struct Mill {
    variables: Vec<Variable>,
}

impl Mill {
    pub fn get_variable(&self, pos: usize) -> &Variable {
        &self.variables[pos]
    }
    pub fn get_variables(&self) -> &Vec<Variable> {
        &self.variables
    }
    pub fn new() -> Mill {
        Mill {
            variables: (0..81).map(|i| Variable::new(i % 9, i / 9)).collect(),
        }
    }
    pub fn get_domain(&self, pos: usize) -> &Vec<EnTy> {
        self.variables[pos].get_domain()
    }

    pub fn apply_unary(&mut self,v : &Vec<(usize,usize,EnTy)>){
        for i in v{
            let j = i.0 + i.1 *9;
            self.variables[j].borrow_mut().set_domain_value(i.2 - 1 );
        }
    }
}

fn printState(mill: &Mill, pos: usize){
    let mut s = String::new();
    for i in &mill.variables[0..pos]{
        let t = format!("{} ", i.get_partial().unwrap_or(99));
        s.push_str(&t.to_owned());
    }
    info!("allocatin {} ", s);
}

fn check(mill: &Mill, val: EnTy, pos: usize) -> bool {
    let current = &mill.variables[pos];
    for i in 0..pos {
        printState(mill, pos);
        if let Some(v) = mill.variables[i].get_partial() {
            let local = &mill.variables[i];
            if is_valid(current, local, val, v) {
                return false;
            }
        }
    }
    true
}

pub fn allocate(mill: &Mill) -> bool {
    allocate_at(mill, 0)
}

fn allocate_at(mill: &Mill, pos: usize) -> bool {
    if pos == mill.variables.len() {
        return true;
    }

    for i in mill.get_domain(pos) {
        mill.variables[pos].set_partial(*i);
        if check(mill, *i, pos) && allocate_at(mill, pos + 1) {
            return true;
        }
        mill.variables[pos].reset_partial();
    }
    false
}


use std::cell::Cell;

pub type EnTy = i32;

// #[derive(Copy)]
pub struct Variable<T> {
    domain: Vec<EnTy>,
    partial: Cell<Option<EnTy>>,
    state: T,
}

impl<T> Variable<T> {
    pub fn get_domain(&self) -> &[EnTy] {
        &self.domain
    }
    pub fn new(t: T, d: Vec<EnTy>) -> Variable<T> {
        Variable {
            // count: Cell::new(d.len()),
            domain: d,
            partial: Cell::new(None),
            state: t,
        }
    }

    pub fn get_state(&self) -> &T {
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

    pub fn remove(&self, value: EnTy) -> Option<usize>{
        None
    }

    // pub fn set_domain_value(&mut self, val: EnTy){
    //     self.domain = vec![val];
    // }
}

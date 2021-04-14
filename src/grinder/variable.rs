use std::cell::{Cell, RefCell};

use crate::grinder::swap_container::SwapContainer;

pub type EnTy = i32;

pub struct Variable<T> {
    partial: Cell<Option<EnTy>>,
    state: T,
    domain2: RefCell<SwapContainer>,
}

impl<T> Variable<T> {
    pub fn new(t: T, d: Vec<EnTy>) -> Variable<T> {
        Variable {
            partial: Cell::new(None),
            state: t,
            domain2: RefCell::new(SwapContainer::new(d)),
        }
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }
    pub fn get_domain(&self) -> &[EnTy] {
        self.domain2.borrow().get_slice()
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

    pub fn remove(&self, _value: EnTy) -> Option<usize> {
        None
    }

    // pub fn set_domain_value(&mut self, val: EnTy){
    //     self.domain = vec![val];
    // }
}

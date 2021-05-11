use crate::grinder::{domain::Domain, variable::{Variable}};

pub struct ConfigTank<T> {
    variables: Vec<Variable<T>>,
    // domains
}

impl<T> ConfigTank<T> {
    pub fn new(variables: Vec<Variable<T>>) -> ConfigTank<T> {
        ConfigTank { variables }
    }
    pub fn get_variable(&self, pos: usize) -> &Variable<T> {
        &self.variables[pos]
    }
    pub fn get_variables(&self) -> &Vec<Variable<T>> {
        &self.variables
    }

    pub fn get_domain(&self, pos: usize) -> &Domain {
        &self.variables[pos].get_domain()
    }

    // pub fn apply_unary(&mut self,v : &Vec<(usize,usize,EnTy)>){
    //     for i in v{
    //         let j = i.0 + i.1 *9;
    //         self.variables[j].borrow_mut().set_domain_value(i.2 - 1 );
    //     }
    // }
}
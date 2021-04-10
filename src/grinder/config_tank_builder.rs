use crate::grinder::variable::Variable;
use crate::grinder::{config_tank::ConfigTank, variable::EnTy};

pub struct ConfigTankBuilder<T> {
    vars: Vec<(T, Vec<EnTy>)>,
}

impl<T> ConfigTankBuilder<T> {
    pub fn new() -> ConfigTankBuilder<T> {
        ConfigTankBuilder { vars: Vec::new() }
    }

    pub fn add_variable(&mut self, t: T, d: Vec<EnTy>) {
        self.vars.push((t, d))
    }

    pub fn finalize(&mut self) -> ConfigTank<T> {
        let vars = self
            .vars
            .drain(..)
            .map(|(t, d)| Variable::new(t, d))
            .collect();
        ConfigTank::new(vars)
    }
}

pub struct VarBuilder<T> {
    t: T,
}

pub fn add_variable<T>(var: T) -> VarBuilder<T> {
    VarBuilder { t: var }
}

impl<T> VarBuilder<T> {
    pub fn with_domain(self, d: Vec<EnTy>) -> DomainBuilder<T> {
        DomainBuilder { t: self.t, d: d }
    }
}
pub struct DomainBuilder<T> {
    t: T,
    d: Vec<EnTy>,
}

impl<T> DomainBuilder<T> {
    pub fn to(self, cb: &mut ConfigTankBuilder<T>) {
        cb.add_variable(self.t, self.d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llll() {
        let mut t: ConfigTankBuilder<i32> = ConfigTankBuilder::new();
        let a = add_variable(1).with_domain(Vec::new()).to(&mut t);
    }
}

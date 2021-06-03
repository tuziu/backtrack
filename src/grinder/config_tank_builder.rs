use crate::grinder::{config_tank::ConfigTank, variable::EnTy};
use crate::grinder::{var_des::VarDes, variable::Variable};

pub struct ConfigTankBuilder<T: Copy> {
    vars: Vec<(T, Vec<EnTy>)>,
}

impl<T: Copy + VarDes> ConfigTankBuilder<T> {
    pub fn new() -> ConfigTankBuilder<T> {
        ConfigTankBuilder { vars: Vec::new() }
    }

    pub fn add_variable(&mut self, t: T, d: Vec<EnTy>) {
        self.vars.push((t, d))
    }

    pub fn finalize(&mut self) -> ConfigTank<T> {
        let mut vars = Vec::new();

        for (i, (t, d)) in self.vars.iter().enumerate() {
            vars.push(Variable::new(*t, d.clone(), i));
        }
        ConfigTank::new(vars)
    }
}

pub struct VarBuilder<T: Copy> {
    t: T,
}

pub fn add_variable<T: Copy>(var: T) -> VarBuilder<T> {
    VarBuilder { t: var }
}

impl<T: Copy> VarBuilder<T> {
    pub fn with_domain(self, d: Vec<EnTy>) -> DomainBuilder<T> {
        DomainBuilder { t: self.t, d: d }
    }
}
pub struct DomainBuilder<T: Copy> {
    t: T,
    d: Vec<EnTy>,
}

impl<T: Copy + VarDes> DomainBuilder<T> {
    pub fn to(self, cb: &mut ConfigTankBuilder<T>) {
        cb.add_variable(self.t, self.d)
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn llll() {
    //     let mut t: ConfigTankBuilder<i32> = ConfigTankBuilder::new();
    //     let _a = add_variable(1).with_domain(Vec::new()).to(&mut t);
    // }
}

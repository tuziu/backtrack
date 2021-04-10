use crate::grinder::var_des::VarDes;
use crate::grinder::{config_tank::ConfigTank, variable::EnTy};

fn check<T: VarDes>(ct: &ConfigTank<T>, val: EnTy, pos: usize) -> bool {
    let current = ct.get_variable(pos);
    for i in 0..pos {
        // printState(mill, pos);
        let local = ct.get_variable(i);
        if let Some(v) = local.get_partial() {
            let l = current.get_state().is_valid(local.get_state(), val, v);
            if l {
                return false;
            }
        }
    }
    true
}

pub fn allocate<T: VarDes>(ct: &ConfigTank<T>) -> bool {
    allocate_at(ct, 0)
}

fn allocate_at<T: VarDes>(ct: &ConfigTank<T>, pos: usize) -> bool {
    if pos == ct.get_variables().len() {
        return true;
    }

    for i in ct.get_domain(pos) {
        ct.get_variable(pos).set_partial(*i);
        if check(ct, *i, pos) && allocate_at(ct, pos + 1) {
            return true;
        }
        ct.get_variable(pos).reset_partial();
    }
    false
}

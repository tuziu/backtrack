use std::collections::VecDeque;

use crate::grinder::{var_des::VarDes, variable::Variable};
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

// fn revise<T : VarDes>(ct: &ConfigTank<T>, vi: usize, vj: usize) -> bool {
//     let mut di = self.variables[vi];
//     let mut dj = self.variables[vj];
//     let mut toDelete = Vec::new();
//     for i in di.getDomain() {
//         if dj.getDomain().iter().all(|j| !Mill::checkConstraint(i, *j)) {
//             toDelete.push(i);
//         }
//     }
//     if !toDelete.is_empty() {
//         toDelete.iter().for_each(|v| { di.remove(*v); });
//         return true;
//     }
//     false
// }
fn revise<T : VarDes>(vi: &Variable<T>, vj: &Variable<T>) -> bool {
    let mut to_delete = Vec::new();
    for i in vi.get_domain() {
        if vj.get_domain().iter().all(|j| !vi.get_state().is_valid(vj.get_state(), *i, *j)) {
            to_delete.push(i);
        }
    }
    if !to_delete.is_empty() {
        to_delete.into_iter().for_each(|v| {vi.remove(*v); });
        return true;
    }
    false
}

fn generate(qeue: &mut VecDeque<(usize, usize)>, from: usize, to: usize) {
    for i in from..to {
        qeue.push_back((from, i));
        qeue.push_back((i, from));
    }
}

fn arc_consistency<T: VarDes>(ct: &ConfigTank<T>, pos: usize) -> bool {
    let mut q = VecDeque::new();
    generate(&mut q,pos, ct.get_variables().len());
    while let Some((vk, vm)) = q.pop_front() {
        if revise(ct.get_variable(vk), ct.get_variable(vm)) {
            if ct.get_domain(vk).is_empty() {
                return false;
            } else {
                generate(&mut q,vk, ct.get_variables().len());
            }
        }
    }
    true
}

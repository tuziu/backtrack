use crate::grinder::config_tank::ConfigTank;
use crate::grinder::var_des::VarDes;
use crate::grinder::variable::EnTy;

#[derive(Copy, Clone)]
pub struct SudokuVar {
    x: usize,
    y: usize,
}

impl SudokuVar {
    pub fn new(x: usize, y: usize) -> SudokuVar {
        SudokuVar { x, y }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl VarDes for SudokuVar {
    fn is_valid(&self, other: &Self, current: EnTy, checked: EnTy) -> bool {
        !((self.get_x() == other.get_x() || self.get_y() == other.get_y()) && (current == checked))
    }
}

pub fn pritn_variables1(ct: &ConfigTank<SudokuVar>) {
    for var in ct.get_variables() {
        println!("{} , {} ", var.get_state().get_x(), var.get_state().get_y());
        var.get_domain().print_vals();
    }
}

#[cfg(test)]
mod tests {
    use crate::{grinder::var_des::VarDes, sudoku::SudokuVar};
    // use crate::sudoku::VarDes::is_valid;

    fn is_valid_h(x: (usize, usize, i32), y: (usize, usize, i32)) -> bool {
        SudokuVar::new(x.0, x.1).is_valid(&SudokuVar::new(y.0, y.1), x.2, y.2)
    }

    #[test]
    fn dupa() {
        assert!(is_valid_h((0, 0, 0), (1, 2, 1)));
        assert!(is_valid_h((0, 0, 0), (1, 2, 0)));

        assert!(is_valid_h((0, 0, 0), (1, 0, 1)));
        assert!(!is_valid_h((0, 0, 0), (1, 0, 0)));

        assert!(is_valid_h((0, 0, 0), (0, 1, 1)));
        assert!(!is_valid_h((0, 0, 0), (0, 1, 0)));

        assert!(is_valid_h((0, 0, 0), (0, 0, 1)));
        assert!(!is_valid_h((0, 0, 0), (0, 0, 0)));
    }
}

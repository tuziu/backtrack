use crate::grinder::var_des::VarDes;
use crate::grinder::variable::EnTy;

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


    fn is_valid(&self, other: &Self, current: i32, checked: i32) -> bool {
        (self.get_x() == other.get_x()) != (self.get_y() == other.get_y()) && current == checked
    }
}

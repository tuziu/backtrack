pub trait VarDes<Other: ?Sized = Self> {
    fn is_valid(&self, other: &Other, current: i32, checked: i32) -> bool;
}

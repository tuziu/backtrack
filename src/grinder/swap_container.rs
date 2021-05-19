use std::slice::Iter;

type Ty = i32;
pub struct SwapContainer {
    data: Vec<Ty>,
    count: usize,
}

impl SwapContainer {
    pub fn new(d: Vec<Ty>) -> SwapContainer {
        let s = d.len();
        SwapContainer { data: d, count: s }
    }

    pub fn save_remove(&mut self, value: &Ty) -> Option<usize> {
        self.data.iter().position(|v| v == value).and_then(|pos| {
            self.data.swap(self.count - 1, pos);
            self.count -= 1;
            Some(self.count)
        })
    }
    pub fn save_remove_rng(&mut self, positions: &[usize]) -> Option<usize> {
        positions
            .iter().rev()
            .enumerate()
            .for_each(|(en, pos)| self.data.swap(*pos, self.count - 1 - en ));
        self.count -= positions.len();
        Some(self.count)
    }

    pub fn restore(&mut self, _pos: usize) {
        self.count += 1;
    }

    pub fn iter(&self) -> Iter<'_, Ty> {
        self.data[0..self.count].iter()
    }

    // pub fn into_iter(self) -> Iter<'static,Ty> {
    //     self.data[0..self.count].into_iter()
    // }

    fn get_slice(&self) -> &[Ty] {
        &self.data[0..self.count]
    }

    pub fn remove(&mut self, pos: usize) -> usize {
        //
        let last_post = self.count - 1;
        self.data.swap(last_post, pos);
        // let value = self.data[pos];
        // self.data[pos] = self.data[last_post];
        // self.data[last_post] = value;
        self.count -= 1;
        last_post
    }

    pub fn get_state(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn revise<F>(&mut self, other: &[Ty], f: F) -> bool
    where
        F: Fn(&Ty, &Ty) -> bool,
    {
        let mut to_delete = Vec::new();
        for (pos, i) in self.iter().enumerate() {
            if !other.iter().any(|j| f(i, j)) {
                to_delete.push(pos);
            }
        }

        if !to_delete.is_empty() {
            self.save_remove_rng(&to_delete);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::SwapContainer;
    use super::Ty;

    #[test]
    fn pierwszy() {
        let a = vec![2, 220, 50, 17];
        let n = SwapContainer::new(a);
        let t: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t, [2, 220, 50, 17]);
    }

    #[test]
    fn remove_1() {
        let a = vec![2, 220, 50, 17];
        let mut n = SwapContainer::new(a);
        n.save_remove(&220);
        let t: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t, [2, 17, 50]);
    }
    #[test]
    fn remove_2() {
        let a = vec![2, 220, 50, 17];
        let mut n = SwapContainer::new(a);
        n.save_remove(&220);
        n.save_remove(&17);
        let t: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t, [2, 50]);
    }

    #[test]
    fn remove_restore_1() {
        let a = vec![2, 220, 50, 17];
        let mut n = SwapContainer::new(a);
        let i = n.save_remove(&220).unwrap();
        let j = n.save_remove(&50).unwrap();
        let t1: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t1, [2, 17]);
        n.restore(j);
        n.restore(i);
        let t2: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t2, [2, 17, 50, 220]);
    }

    #[test]
    fn rng_remove_1() {
        let a = vec![2, 7, 6, 12];
        let mut n = SwapContainer::new(a);
        let i = n.save_remove_rng(&[0,3] );
        let t2: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t2,[6,7]);

    }

    #[test]
    fn revise_1() {
        let a = vec![2, 7, 6, 12, 11, 101, 77];
        let mut n = SwapContainer::new(a);
        let i = n.revise(&[0,3],|x,y| x != y );
        let t2: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t2,[2, 7, 6, 12, 11, 101, 77]);
    }

    #[test]
    fn revise_2() {
        let a = vec![2, 7, 6, 12, 11, 101, 77];
        let mut n = SwapContainer::new(a);
        let i = n.revise(&[2,3],|x,y| x != y && ((x % y) !=  0 ) );
        let t2: Vec<Ty> = n.iter().map(|x| *x).collect();
        assert_eq!(t2,[2, 7, 6, 12, 11, 101, 77]);
    }
}

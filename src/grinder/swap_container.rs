use std::slice::Iter;

type Ty = i32;
pub struct SwapContainer {
    data: Vec<Ty>,
    count: usize,
}

type Positon = usize;

impl SwapContainer {
    fn new() -> SwapContainer {
        SwapContainer {
            data: Vec::new(),
            count: 0,
        }
    }

    pub fn add(&mut self, value: Ty) {
        if self.data.contains(&value){
            self.data.push(value);
            self.count += 1;
        }
    }

    pub fn save_remove(&mut self, value: Ty) -> Option<usize> {
        self.data.iter().position(|v| *v == value).and_then(|pos| {
            let last_post = self.count - 1;
            self.data[pos] = self.data[last_post];
            self.data[last_post] = value;
            self.count -= 1;
            Some(last_post)
        })
    }

    pub fn restore(&mut self, pos: usize) {
        self.count += 1;
    }

    fn iter(&self) -> Iter<'_,Ty>{
        self.data[0..self.count].iter()
    }

    pub fn remove(&mut self, pos: usize) -> usize {
        let last_post = self.count - 1;
        let value = self.data[pos];
        self.data[pos] = self.data[last_post];
        self.data[last_post] = value;
        self.count -= 1;
        last_post
    }

    pub fn get_state(&self) -> usize{
        self.count
    }
}
#[cfg(test)]
mod tests {
    use super::SwapContainer;


    #[test]
    fn pierwszy() {
        let mut n = SwapContainer::new();
        n.add(2);
        n.add(220);
        n.add(50);
        n.add(17);
        let t : Vec<i64> = n.iter().map(|x| *x).collect();
        assert_eq!(t,[2,220,50,17]);
    }

    #[test]
    fn remove_1() {
        let mut n = SwapContainer::new();
        n.add(2);
        n.add(220);
        n.add(50);
        n.add(17);
        n.save_remove(220);
        let t : Vec<i64> = n.iter().map(|x| *x).collect();
        assert_eq!(t,[2,17,50]);
    }
    #[test]
    fn remove_2() {
        let mut n = SwapContainer::new();
        n.add(2);
        n.add(220);
        n.add(50);
        n.add(17);
        n.save_remove(220);
        n.save_remove(17);
        let t : Vec<i64> = n.iter().map(|x| *x).collect();
        assert_eq!(t,[2,50]);
    }

    #[test]
    fn remove_restore_1() {
        let mut n = SwapContainer::new();
        n.add(2);
        n.add(220);
        n.add(50);
        n.add(17);
        let i = n.save_remove(220).unwrap();
        let j = n.save_remove(50).unwrap();
        let t1 : Vec<i64> = n.iter().map(|x| *x).collect();
        assert_eq!(t1,[2,17]);
        n.restore(j);
        n.restore(i);
        let t2 : Vec<i64> = n.iter().map(|x| *x).collect();
        assert_eq!(t2,[2,17,50,220]);

    }
}
pub trait AbsPos<T> {
    fn aposition(&self, val: T, from: usize) -> Option<usize>;
    fn raposition(&self, val: T, from: usize) -> Option<usize>;    
}

impl<T: std::cmp::PartialEq> AbsPos<T> for &[T] {
    fn aposition(&self, val: T, from: usize) -> Option<usize> {
        for i in from..self.len() {
            if self[i] == val {
                return Some(i)
            }
        }
        None
    }

    fn raposition(&self, val: T, from: usize) -> Option<usize> {
        for i in (0..from).rev() {
            if self[i] == val {
                return Some(i)
            }
        }
        None
    }
}
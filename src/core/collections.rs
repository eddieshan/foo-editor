// Search: trait with methods to find absolute position of items in an iterator.
// It is meant to solve the limitation when calling Iter::position and 
// Iter::rposition on a subrange of a slice, which return indices relative to 
// the subrange, instead of absolute positions.
pub trait Search<T> {
    // Absolute position of item with value @val, starting from the left at @from.
    fn apos(&self, val: T, from: usize) -> Option<usize>;
    // Absolute position of item with value @val, starting from the right at @from.
    fn rapos(&self, val: T, from: usize) -> Option<usize>;

    // Absolute position of nth item with value @val, starting from the left at @from.
    fn apos_n(&self, val: T, n: usize, from: usize) -> Option<usize>;
    // Absolute position of nth item with value @val, starting from the right at @from.
    fn rapos_n(&self, val: T, n: usize, from: usize) -> Option<usize>;

    fn at_least(&self, val: T, n: usize) -> bool;

    fn count(&self, val: T) -> usize;    
}

// TODO: Search implementation for slices.
// It can be generalized to Iter<T> but I need more time to evaluate the various
// possible implementations. At first sight it looks like the current definition of 
// Search does not fit with the Iter API so it might have to be changed. 
// For the moment though the slice impl is enough.
impl<T: std::cmp::PartialEq> Search<T> for &[T] {
    fn apos(&self, val: T, from: usize) -> Option<usize> {
        for i in from..self.len() {
            if self[i] == val {
                return Some(i)
            }
        }
        None
    }

    fn rapos(&self, val: T, from: usize) -> Option<usize> {
        for i in (0..from).rev() {
            if self[i] == val {
                return Some(i)
            }
        }
        None
    }

    fn apos_n(&self, val: T, n: usize, from: usize) -> Option<usize> {
        let mut count = n;
        for i in from..self.len() {
            if self[i] == val {
                count -= 1;
                if count == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    fn rapos_n(&self, val: T, n: usize, from: usize) -> Option<usize> {
        let mut count = n;
        for i in (0..from).rev() {
            if self[i] == val {
                count -= 1;
                if count == 0 {
                    return Some(i);
                }
            }
        }
        None
    }    

    fn at_least(&self, val: T, n: usize) -> bool {
        let mut count = n;
        for i in 0..self.len() {
            if self[i] == val {
                count -= 1;
                if count == 0 {
                    return true;
                }
            }
        }
        false
    }

    fn count(&self, val: T) -> usize {
        let mut count = 0;
        for i in 0..self.len() {
            if self[i] == val {
                count += 1;
            }
        }
        count
    }    
}
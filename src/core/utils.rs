// AbsPos: trait with methods to find absolute position of items in an iterator.
// It is meant to solve the limitation when calling Iter::position and 
// Iter::rposition on a subrange of a slice, which return indices relative to 
// the subrange, instead of absolute positions.
pub trait AbsPos<T> {
    // Absolute position of item with value @val, starting from the left at @from.
    fn apos(&self, val: T, from: usize) -> Option<usize>;
    // Absolute position of item with value @val, starting from the right at @from.
    fn rapos(&self, val: T, from: usize) -> Option<usize>;    
}

// TODO: AbsPos implementation for slices.
// It can be generalized to Iter<T> but I need more time to evaluate the various
// possible implementations. At first sight it looks like the current definition of 
// AbsPos does not fit with the Iter API so it might have to be changed. 
// For the moment though the slice impl is enough.
impl<T: std::cmp::PartialEq> AbsPos<T> for &[T] {
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
}
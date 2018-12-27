use std::mem;

/// Re-orders elements in the range yielded by `it` based on `pred`. All elements
/// that the predicate returns true for will be placed before all elements
/// that the predicate returned false for. Also returns the index of the
/// first element in the false group
pub fn partition<'a, T: 'a, I, F>(mut it: I, pred: F) -> usize
        where I: DoubleEndedIterator<Item = &'a mut T>,
        F: Fn(&T) -> bool {
    let mut split_idx = 0;
    loop {
        let mut front = None;
        let mut back = None;
        while let Some(f) = it.next() {
            if !pred(f) {
                front = Some(f);
                break;
            } else {
                split_idx += 1;
            }
        }
        while let Some(b) = it.next_back() {
            if pred(b) {
                back = Some(b);
                break;
            }
        }
        match (front, back) {
            (Some(f), Some(b)) => {
                mem::swap(f, b);
                split_idx += 1;
            },
            _ => break,
        }
    }
    split_idx
}

use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&array[..mid], key),
        Ordering::Greater => find(&array[mid + 1..], key).map(|i| i + mid + 1),
    }
}

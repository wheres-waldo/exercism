use std::cmp::Ordering;

pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    let array = array.as_ref();
    let middle = array.len() >> 1;
    match key.cmp(array.get(middle)?) {
        Ordering::Less => find(&array[..middle], key),
        Ordering::Equal => Some(middle),
        Ordering::Greater => find(&array[middle + 1..], key).map(|i| i + middle + 1),
    }
}

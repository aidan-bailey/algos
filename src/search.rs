pub fn linear<T: std::cmp::PartialEq>(
    item: &T,
    items: &Vec<T>,
) -> Option<usize> {
    for (index, value) in items.iter().enumerate() {
        if item == value {
            return Some(index);
        }
    }
    return None;
}

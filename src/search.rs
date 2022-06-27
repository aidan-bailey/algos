pub fn linear<'a, T: 'a + std::cmp::PartialEq, I: Iterator<Item = &'a T>>(
    item: &T,
    items: I,
) -> Option<usize> {
    for (index, value) in items.enumerate() {
        if item == value {
            return Some(index);
        }
    }
    return None;
}

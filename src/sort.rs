pub fn insertion<T: std::cmp::PartialOrd + Copy>(items: &Vec<T>) -> Vec<T> {
    let mut sorted_items: Vec<T> = Vec::new();
    for insertion_item in items.iter().copied() {
        let mut insertion_index = sorted_items.len();
        for (index, item) in sorted_items.iter().enumerate() {
            if insertion_item < *item {
                insertion_index = index;
                break;
            }
        }
        sorted_items.insert(insertion_index, insertion_item);
    }
    return sorted_items;
}

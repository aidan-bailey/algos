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

pub fn selection<T: std::cmp::PartialOrd + Copy>(items: &Vec<T>) -> Vec<T> {
    let mut sorted_items: Vec<T> = items.clone();
    let mut insertion_index = 0;
    while insertion_index < sorted_items.len() - 1 {
        let mut min_index = insertion_index;
        for index in insertion_index+1..sorted_items.len() {
            if sorted_items[index] < sorted_items[min_index] {
                min_index = index;
            }
        }
        sorted_items.swap(insertion_index, min_index);
        insertion_index += 1;
    }
    return sorted_items;
}

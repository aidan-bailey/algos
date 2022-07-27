/// sort the given vector using the [Insertion Sort](https://en.wikipedia.org/wiki/Insertion_sort) algorithm
pub fn insertion<T: std::cmp::PartialOrd>(items: &mut Vec<T>) {
    if items.len() < 2 {
        return;
    }
    for current_index in 1..items.len() {
        for insertion_index in (1..current_index + 1).rev() {
            if items[insertion_index] < items[insertion_index - 1] {
                items.swap(insertion_index, insertion_index - 1);
            } else {
                break;
            }
        }
    }
}

/// sort the given vector using the [Selection Sort](https://en.wikipedia.org/wiki/Selection_sort) algorithm
pub fn selection<T: std::cmp::PartialOrd>(items: &mut Vec<T>) {
    if items.len() < 2 {
        return;
    }
    for insertion_index in 0..items.len() - 1 {
        let mut min_index = insertion_index;
        for index in insertion_index + 1..items.len() {
            if items[index] < items[min_index] {
                min_index = index;
            }
        }
        items.swap(insertion_index, min_index);
    }
}

/// sort the given vector using the [Merge Sort](https://en.wikipedia.org/wiki/Merge_sort) algorithm
pub fn merge<T: std::cmp::PartialOrd + Copy>(items: Vec<T>) -> Vec<T> {
    if items.len() < 2 {
        return items;
    }
    let m = items.len() / 2;
    let sorted_items1: Vec<T> = merge(items[..m].iter().copied().collect::<Vec<T>>());
    let sorted_items2: Vec<T> = merge(items[m..].iter().copied().collect::<Vec<T>>());
    let mut sorted_items: Vec<T> = Vec::new();
    let mut index1 = 0;
    let mut index2 = 0;
    while index1 < sorted_items1.len() && index2 < sorted_items2.len() {
        if sorted_items1[index1] < sorted_items2[index2] {
            sorted_items.push(sorted_items1[index1]);
            index1 += 1;
        } else if sorted_items2[index2] < sorted_items1[index1] {
            sorted_items.push(sorted_items2[index2]);
            index2 += 1;
        } else {
            sorted_items.push(sorted_items1[index1]);
            sorted_items.push(sorted_items2[index2]);
            index1 += 1;
            index2 += 1;
        }
    }
    sorted_items.append(&mut sorted_items1[index1..].iter().copied().collect::<Vec<T>>());
    sorted_items.append(&mut sorted_items2[index2..].iter().copied().collect::<Vec<T>>());
    return sorted_items;
}

/// sort the given vector using the [Quick Sort](https://en.wikipedia.org/wiki/Quick_sort) algorithm
pub fn quick<T: std::cmp::PartialOrd>(items: &mut Vec<T>) {
    if items.len() < 2 {
        return;
    }
    let mut partitions: Vec<(usize, usize)> = vec![(0, items.len())];
    while let Some(partition) = partitions.pop() {
        if partition.1 < 2 {
            continue;
        }
        let mut pivot_index = partition.0 + partition.1 - 1;
        let mut from_right_index = pivot_index - 1;
        let mut from_left_index = partition.0;
        while from_left_index < from_right_index {
            if items[from_left_index] > items[pivot_index] {
                if items[from_right_index] < items[pivot_index] {
                    items.swap(pivot_index, from_right_index);
                    items.swap(pivot_index, from_left_index);
                    pivot_index = from_right_index;
                }
                from_right_index -= 1;
            } else {
                from_left_index += 1;
            }
        }
        if items[from_left_index] > items[pivot_index] {
            items.swap(pivot_index, from_left_index);
            pivot_index = from_left_index;
        }
        let left_partition_len = pivot_index - partition.0;
        let left_partition = (partition.0, left_partition_len);
        partitions.push(left_partition);
        let right_partition_len = partition.1 - left_partition_len - 1;
        let right_partition = (pivot_index + 1, right_partition_len);
        partitions.push(right_partition);
    }
}

/// sort the given vector using the [Bubble Sort](https://en.wikipedia.org/wiki/Bubble_sort) algorithm
pub fn bubble<T: std::cmp::PartialOrd>(items: &mut Vec<T>) {
    if items.len() < 2 {
        return;
    }
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for index in 0..items.len() - 1 {
            if items[index] > items[index + 1] {
                items.swap(index, index + 1);
                sorted = false;
            }
        }
    }
}

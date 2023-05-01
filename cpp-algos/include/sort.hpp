#ifndef SORT_HPP_
#define SORT_HPP_

#include <algorithm>
#include <cstddef>
#include <iostream>
#include <iterator>
#include <queue>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

namespace algos {

namespace sort {

/**
 * Sort (inplace) the given vector using the Insertion Sort algorithm.
 *
 * @param items Vector to be sorted.
 * @return Reference to sorted vector.
 */
template <typename T> std::vector<T> &insertion(std::vector<T> &items) {

  if (items.size() < 2)
    return items;

  for (size_t current_index = 1; current_index < items.size();
       current_index++) {
    for (size_t insertion_index = current_index; insertion_index > 0;
         insertion_index--) {
      if (items[insertion_index] < items[insertion_index - 1]) {
        std::swap(items[insertion_index], items[insertion_index - 1]);
      } else {
        break;
      }
    }
  }

  return items;
}

/**
 * Sort the given array using the Merge Sort algorithm.
 *
 * @param arr Pointer to array to be sorted.
 * @param size Size of array.
 * @returns Pointer to sorted array.
 */
template <typename T> T *merge(const T *items, const size_t &size) {
  switch (size) {
  case 0:
    return new T[0]{};
  case 1:
    return new T[0]{items[0]};
  }
  if (size < 2)
    return new T[1]{items[0]};
  const size_t l_size = size / 2;
  const size_t r_size = size - l_size;
  T *l_sorted = merge(items, l_size);
  T *r_sorted = merge(items + l_size, r_size);
  T *sorted = new T[size];
  size_t i = 0, l = 0, r = 0;
  while (l < l_size && r < r_size)
    l_sorted[l] < r_sorted[r] ? sorted[i++] = l_sorted[l++]
                              : sorted[i++] = r_sorted[r++];
  while (l < l_size)
    sorted[i++] = l_sorted[l++];
  while (r < r_size)
    sorted[i++] = r_sorted[r++];
  delete l_sorted;
  delete r_sorted;
  return sorted;
}

/**
 * Sort the given vector using the Merge Sort algorithm.
 *
 * @param items Vector to be sorted.
 * @returns Sorted vector.
 */

template <typename T> std::vector<T> merge(const std::vector<T> &items) {
  T *sorted_ptr = merge(&items.front(), items.size());
  std::vector result(sorted_ptr, sorted_ptr + items.size());
  delete sorted_ptr;
  return result;
}

/**
 * Sort (inplace) the given vector using the Selection Sort algorithm.
 *
 * @param items Vector to be sorted.
 * @return Reference to sorted vector.
 */
template <typename T> std::vector<T> &selection(std::vector<T> &items) {
  if (items.size() < 2) {
    return items;
  }
  for (size_t insertion_index = 0; insertion_index < items.size();
       insertion_index++) {
    size_t min_index = insertion_index;
    for (size_t index = insertion_index + 1; index < items.size(); index++) {
      if (items[index] < items[min_index]) {
        min_index = index;
      }
    }
    std::swap(items[insertion_index], items[min_index]);
  }
  return items;
}

/**
 * Sort (inplace) the given vector using the Quick Sort algorithm.
 *
 * @param items Vector to be sorted.
 * @return Reference to sorted vector.
 */
template <typename T> std::vector<T> &quick(std::vector<T> &items) {
  if (items.size() < 2)
    return items;

  std::vector<std::pair<size_t, size_t>> partitions{
      std::pair<size_t, size_t>(0, items.size())};
  while (!partitions.empty()) {
    std::pair<size_t, size_t> partition(partitions.back());
    partitions.pop_back();
    if (partition.second < 2)
      continue;
    size_t pivot_index(partition.first + partition.second - 1);
    size_t from_right_index(pivot_index - 1);
    size_t from_left_index(partition.first);
    while (from_left_index < from_right_index) {
      if (items[from_left_index] > items[pivot_index]) {
        if (items[from_right_index] < items[pivot_index]) {
          std::swap(items[pivot_index], items[from_right_index]);
          std::swap(items[pivot_index], items[from_left_index]);
          pivot_index = from_right_index;
        }
        from_right_index -= 1;
      } else {
        from_left_index += 1;
      }
    }
    if (items[from_left_index] > items[pivot_index]) {
      std::swap(items[pivot_index], items[from_left_index]);
      pivot_index = from_left_index;
    }
    const size_t left_partition_len(pivot_index - partition.first);
    const std::pair<size_t, size_t> left_partition(
        std::pair<size_t, size_t>(partition.first, left_partition_len));
    partitions.push_back(left_partition);
    const size_t right_partition_len(partition.second - left_partition_len - 1);
    const std::pair<size_t, size_t> right_partition(
        std::pair<size_t, size_t>(pivot_index + 1, right_partition_len));
    partitions.push_back(right_partition);
  }
  return items;
}

/**
 * Sort (inplace) the given vector using the Bubble Sort algorithm.
 *
 * @param items Vector to be sorted.
 * @return Reference to sorted vector.
 */
template <typename T> std::vector<T> &bubble(std::vector<T> &items) {
  if (items.size() < 2)
    return items;
  bool sorted = false;
  while (!sorted) {
    sorted = true;
    for (size_t index = 0; index < items.size(); index++) {
      if (items[index] > items[index + 1]) {
        std::swap(items[index], items[index + 1]);
        sorted = false;
      }
    }
  }
  return items;
}

} // namespace sort

} // namespace algos

#endif // SORT_HPP_

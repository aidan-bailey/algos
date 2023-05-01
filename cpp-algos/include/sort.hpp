#ifndef SORT_HPP_
#define SORT_HPP_

#include <algorithm>
#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

namespace algos {

namespace sort {

/**
 * Sort the given vector using the Insertion Sort algorithm.
 *
 * @param items Vector to be sorted.
 */
template <typename T> void insertion(std::vector<T> &items) {

  if (items.size() < 2)
    return;

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
}

/**
 * Basic merge sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void merge(T *arr, int &size) {

  if (size == 1)
    return;

  int l_size = size / 2;
  T *l_ptr = arr;

  int r_size = size - l_size;
  T *r_ptr = (arr + l_size);

  merge(l_ptr, l_size);
  merge(r_ptr, r_size);

  // merging
  T *arrC = new T[size];
  int i = 0, l = 0, r = 0;
  while (l < l_size && r < r_size)
    l_ptr[l] < r_ptr[r] ? *(arrC + i++) = l_ptr[l++]
                        : *(arrC + i++) = r_ptr[r++];
  while (l < l_size)
    *(arrC + i++) = l_ptr[l++];
  while (r < r_size)
    *(arrC + i++) = r_ptr[r++];

  std::copy(arrC, arrC + size, arr);

  delete[] arrC;
}

/**
 * Sort the given vector using the Selection Sort algorithm.
 *
 * @param items Vector to be sorted.
 */
template <typename T> void selection(std::vector<T> &items) {
  if (items.size() < 2) {
    return;
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
}

/**
 * Sort the given vector using the Quick Sort algorithm.
 *
 * @param items Vector to be sorted.
 */
template <typename T> void quick(std::vector<T> &items) {
  if (items.size() < 2)
    return;

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
}

} // namespace sort

} // namespace algos

#endif // SORT_HPP_

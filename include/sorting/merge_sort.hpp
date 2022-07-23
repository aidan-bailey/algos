#ifndef __MERGE_SORT_H_
#define __MERGE_SORT_H_

#include <algorithm>
namespace sort {

/**
 * Basic merge sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void merge_sort(T *arr, int &size) {

  if (size == 1)
    return;

  int l_size = size / 2;
  int l_index = 0;
  T *l_ptr = arr;

  int r_size = size - l_size;
  int r_index = l_size;
  T *r_ptr = (arr + l_size);

  merge_sort(l_ptr, l_size);
  merge_sort(r_ptr, r_size);

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

} // namespace sort

#endif // __MERGE_SORT_H_

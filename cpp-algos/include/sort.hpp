#ifndef SORT_HPP_
#define SORT_HPP_

#include <algorithm>

namespace algos {

namespace sort {

/**
 * Basic insertion sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void insertion(T *arr, int &size) {
  for (int i = 0; i < size; i++) {

    // store currently selected element
    int const temp = arr[i];
    int index = i;

    // compare to element on the left and switcheroo if required
    while (index > 0 && temp < arr[index - 1]) {
      *(arr + index) = arr[index - 1];
      index--;
    }

    // move select element to new position
    *(arr + index) = temp;
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
  int l_index = 0;
  T *l_ptr = arr;

  int r_size = size - l_size;
  int r_index = l_size;
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
 * Basic quick sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void quick(T *arr, int size) {

  if (size <= 1)
    return;

  int const pivotIndex = 0;
  T pivot = arr[pivotIndex];
  int finalIndex = pivotIndex;

  for (int i = finalIndex + 1; i < size; i++) {
    if (pivot < arr[i]) {
      continue;
    }
    finalIndex++;
    T temp = arr[i];
    arr[i] = arr[finalIndex];
    arr[finalIndex] = temp;
  }

  T temp = arr[finalIndex];
  *(arr + finalIndex) = pivot;
  *(arr + pivotIndex) = temp;

  quick<T>(arr, finalIndex);
  quick<T>(arr + finalIndex + 1, size - (finalIndex + 1));
}

} // namespace sort

} // namespace algos

#endif // SORT_HPP_

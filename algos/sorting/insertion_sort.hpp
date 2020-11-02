#ifndef __INSERTIONSORT_H_
#define __INSERTIONSORT_H_

namespace sort {

/**
 * Basic insertion sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void insertion_sort(T *arr, const int size) {
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

} // namespace sort

#endif // __INSERTIONSORT_H_

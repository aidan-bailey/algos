#ifndef __QUICKSORT_H_
#define __QUICKSORT_H_

namespace sort {

/**
 * Basic quick sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void quick_sort(T *arr, const int size) {

  // basecase
  if (size <= 1)
    return;

  // acquire pivot
  int pivotIndex = 0;
  T pivot = arr[pivotIndex];
  int finalIndex = pivotIndex + 1;

  // partition
  for (int j = pivotIndex + 1; j < size; j++) {
    if (arr[j] < pivot) {
      // if probed element is less than the pivot, swap it into the pivots final
      // index range
      T temp = arr[j];
      *(arr + j) = arr[finalIndex];
      *(arr + finalIndex++) = temp;
    }
  }

  // decrement final index to actual position
  finalIndex--;

  // place pivot in final destination
  T temp = arr[finalIndex];
  *(arr + finalIndex) = pivot;
  *(arr + pivotIndex) = temp;

  // recursively quicksort the resulting partitions
  quick_sort<T>(arr, finalIndex);
  quick_sort<T>(arr + finalIndex, size - (finalIndex + 1));
}

} // namespace sort

#endif // __QUICKSORT_H_

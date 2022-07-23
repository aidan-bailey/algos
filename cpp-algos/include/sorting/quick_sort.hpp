#ifndef __QUICK_SORT_H_
#define __QUICK_SORT_H_

namespace sort {

/**
 * Basic quick sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void quick_sort(T *arr, int size) {

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

  quick_sort<T>(arr, finalIndex);
  quick_sort<T>(arr + finalIndex + 1, size - (finalIndex + 1));
}

} // namespace sort

#endif // __QUICK_SORT_H_

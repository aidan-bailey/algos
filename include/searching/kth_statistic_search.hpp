#ifndef __KTH_STATISTIC_SEARCH_H_
#define __KTH_STATISTIC_SEARCH_H_

namespace search {

/**
 * K^th order statistic search
 *
 * @param key Index of statistic to find.
 * @param arr Unsorted array.
 * @param size Size of unsorted array.
 * */
template <typename T> int kth_statistic_search(int key, T *arr, int size) {

  if (size <= 1) {
    return arr[0];
  }

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

  if (finalIndex + 1 == key)
    return pivot;

  T temp = arr[finalIndex];
  *(arr + finalIndex) = pivot;
  *(arr + pivotIndex) = temp;

  return key < finalIndex + 1
             ? kth_statistic_search<T>(key, arr, finalIndex)
             : kth_statistic_search<T>(key - finalIndex, arr + finalIndex + 1,
                                       size - (finalIndex + 1));
}

} // namespace search

#endif // __KTH_STATISTIC_SEARCH_H_

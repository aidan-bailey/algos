#ifndef __TERNARY_SEARCH_H_
#define __TERNARY_SEARCH_H_

namespace search {

/**
 * Basic ternary search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T>
int ternary_search(const T key, const T *arr, const int size) {
  int left(0), right(size - 1);
  while (left <= right) {
    int mid1(left + (right - left) / 3), mid2(right - (right - left) / 3);
    if (arr[mid1] == key)
      return mid1;
    if (arr[mid2] == key)
      return mid2;
    if (key < arr[mid1])
      right = mid1 - 1;
    else if (key > arr[mid2])
      left = mid2 + 1;
    else {
      left = mid1 + 1;
      right = mid2 - 1;
    }
  }
  return -1;
}

} // namespace search

#endif // __TERNARY_SEARCH_H_

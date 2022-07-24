#ifndef SEARCH_HPP_
#define SEARCH_HPP_

namespace algos {

namespace search {

/**
 * Basic linear search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T> int linear(T &key, const T *arr, int &size) {
  for (int i = 0; i < size; i++)
    if (*(arr + i) == key)
      return i;
  return -1;
}

/**
 * Basic binary search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T> int binary(const T key, const T *arr, const int size) {
  int low(0), high(size - 1);
  while (low <= high) {
    int index = (low + high) / 2;
    if (arr[index] == key)
      return index;
    arr[index] < key ? low = index + 1 : high = index - 1;
  }
  return -1;
}

/**
 * Basic ternary search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T> int ternary(const T key, const T *arr, const int size) {
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

} // namespace algos

#endif // SEARCH_HPP_

#ifndef __BINARY_SEARCH_H_
#define __BINARY_SEARCH_H_

namespace search {

/**
 * Basic binary search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T>
int binary_search(const T key, const T *arr, const int size) {
  int low(0), high(size - 1);
  while (low <= high) {
    int index = (low + high) / 2;
    if (arr[index] == key)
      return index;
    arr[index] > key ? low = index + 1 : high = index - 1;
  }
  return -1;
}

} // namespace search

#endif // __BINARY_SEARCH_H_

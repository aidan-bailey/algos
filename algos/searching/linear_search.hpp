#ifndef __LINEAR_SEARCH_H_
#define __LINEAR_SEARCH_H_

namespace search {

/**
 * Basic linear search.
 *
 * @param key Value to be searched for.
 * @param arr Array pointer to be searched.
 * @param size Size of array.
 * @return Index of element, -1 if not found.
 */
template <typename T>
int linear_search(const T key, const T *arr, const int size) {
  for (int i = 0; i < size; i++)
    if (*(arr + i) == key)
      return i;
  return -1;
}

} // namespace search

#endif // __LINEAR_SEARCH_H_

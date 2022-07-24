#ifndef SEARCH_HPP_
#define SEARCH_HPP_

#include <optional>
#include <vector>

namespace algos {

namespace search {

/**
 * Linear search.
 *
 * @param item Item to be searched for.
 * @param items Items be searched through.
 * @return Index of item.
 */
template <typename T>
std::optional<size_t> linear(const T &item, const std::vector<T> &items) {
  for (size_t index = 0; index < items.size(); index++)
    if (items[index] == item)
      return index;
  return {};
}

/**
 * Binary search.
 *
 * @param item Item to be searched for.
 * @param items Items be searched through.
 * @return Index of item.
 */
template <typename T>
std::optional<size_t> binary(const T &item, const std::vector<T> &items) {
  size_t l(0);
  size_t r(items.size() - 1);
  while (l <= r) {
    const size_t m = (l + r) / 2;
    if (items[m] < item)
      l = m + 1;
    else if (items[m] > item)
      r = m - 1;
    else
      return m;
  }
  return {};
}

/**
 * Ternary search.
 *
 * @param item Item to be searched for.
 * @param items Items be searched through.
 * @return Index of item.
 */
template <typename T>
std::optional<size_t> ternary(const T &item, const std::vector<T> &items) {
  size_t l(0);
  size_t r(items.size() - 1);
  while (l <= r) {
    const size_t m1(l + (r - l) / 3);
    const size_t m2(r - (r - l) / 3);
    if (items[m1] == item)
      return m1;
    else if (items[m2] == item)
      return m2;
    else if (items[m2] < item)
      l = m2 + 1;
    else if (items[m1] < item) {
      l = m1 + 1;
      r = m2 - 1;
    } else {
      r = m1 - 1;
    }
  }
  return {};
}

} // namespace search

} // namespace algos

#endif // SEARCH_HPP_

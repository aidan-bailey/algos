#ifndef SEARCH_HPP_
#define SEARCH_HPP_

#include <cstddef>
#include <iostream>
#include <optional>
#include <ostream>
#include <vector>

namespace algos {

namespace searching {

/**
 * Linear search.
 *
 * @param item Item to be searched for.
 * @param items Items be searched through.
 * @return Index of item.
 */
template <typename T>
std::optional<size_t> const linear(const T &item, const std::vector<T> &items) {
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
std::optional<size_t> const binary(const T &item, const std::vector<T> &items) {
  if (items.empty())
    return {};
  size_t l(0);
  size_t r(items.size() - 1);
  while (l <= r) {
    const size_t m = (l + r) / 2;
    if (items[m] < item)
      l = m + 1;
    else if (item < items[m])
      if (m == 0)
        return {};
      else
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
std::optional<size_t> const ternary(const T &item,
                                    const std::vector<T> &items) {
  if (items.empty())
    return {};
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
    } else if (m1 == 0)
      return {};
    else
      r = m1 - 1;
  }
  return {};
}

/**
 * Kary search.
 *
 * @param k Arity.
 * @param item Item to be searched for.
 * @param items Items be searched through.
 * @return Index of item.
 */
template <typename T>
std::optional<size_t> const kary(const uint &k, const T &item,
                                 const std::vector<T> &items) {
  if (items.empty())
    return {};
  size_t l(0);
  size_t r(items.size() - 1);
  while (l <= r) {
    const size_t const_l(l);
    const size_t const_r(r);
    for (size_t i = 1; i < k + 1; i++) {
      const size_t m = const_l + i * (const_r - const_l) / (k + 1);
      if (items[m] == item)
        return m;
      else if (item < items[m]) {
        if (m == 0)
          return {};
        r = m - 1;
        break;
      } else
        l = m + 1;
    }
  }
  return {};
}

} // namespace search

} // namespace algos

#endif // SEARCH_HPP_

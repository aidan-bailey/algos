#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <map>
#include <optional>
#include <string>
#include <vector>
#define CATCH_CONFIG_MAIN
#include "search.hpp"
#include "sort.hpp"
#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>
#include <functional>
#include <iostream>
#include <set>

using namespace algos;

static bool check_sorted_asc(int *arr, int n) {
  for (--n; n > 0; n--)
    if (*(arr + n) < *(arr + n - 1))
      return false;
  return true;
}

std::vector<int> const gen_vec(size_t size, bool sorted, bool unique) {
  std::vector<int> result(size, 0);
  std::set<int> used;
  for (size_t i = 0; i < size; i++) {
    size_t n = rand();
    if (unique) {
      while (used.count(n))
        n = rand();
      used.insert(n);
    }
    result[i] = n;
  }
  if (sorted)
    std::sort(result.begin(), result.end());
  return result;
}

std::vector<int> const get_vec(size_t size, bool sorted, bool unique) {
  static std::map<const std::string, const std::vector<int>> cache;
  const std::string key =
      std::to_string(size) + std::to_string(sorted) + std::to_string(unique);
  if (cache.count(key)) {
    return cache[key];
  }
  const std::vector<int> result = gen_vec(size, sorted, unique);
  cache.insert({key, result});
  return result;
}

#define BASIC_TEST_SIZE 1000

#define TEST_BASIC_SORT(name, funcname, func)                                  \
  TEST_CASE(name, funcname) {                                                  \
    GIVEN("An unsorted list") {                                                \
      const std::vector<int> items(get_vec(BASIC_TEST_SIZE, false, false));    \
      THEN("Return a sorted list") {                                           \
        std::vector<int> items_copy(items);                                    \
        std::vector<int> sorted = func(items_copy);                            \
        CHECK(check_sorted_asc(&sorted[0], items_copy.size()));                \
      }                                                                        \
    }                                                                          \
  }

#define TEST_BASIC_SEARCH(name, funcname, func)                                \
  TEST_CASE(name, funcname) {                                                  \
    GIVEN("An even length integer list and an item") {                         \
      const std::vector<int> esu_items(get_vec(BASIC_TEST_SIZE, true, true));  \
      {                                                                        \
        WHEN("It's the first item in the list")                                \
        THEN("Return 0")                                                       \
        CHECK(func(esu_items[0], esu_items).value() == 0);                     \
      }                                                                        \
      {                                                                        \
        WHEN("It's the last item in the list")                                 \
        THEN("Return index of the last item")                                  \
        CHECK(func(esu_items.back(), esu_items).value() ==                     \
              esu_items.size() - 1);                                           \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that exists in the list")                           \
        THEN("Return the index of the item")                                   \
        CHECK(func(esu_items[499], esu_items) == 499);                         \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that does not exist in the list")                   \
        THEN("Return no index")                                                \
        CHECK(!func(-1, esu_items).has_value());                               \
      }                                                                        \
    }                                                                          \
    GIVEN("An odd length integer list and an item") {                          \
      static const std::vector<int> osu_items(                                 \
          get_vec(BASIC_TEST_SIZE + 1, true, true));                           \
      {                                                                        \
        WHEN("It's the first item in the list")                                \
        THEN("Return 0")                                                       \
        CHECK(func(osu_items[0], osu_items).value() == 0);                     \
      }                                                                        \
      {                                                                        \
        WHEN("It's the last item in the list")                                 \
        THEN("Return index of the last item")                                  \
        CHECK(func(osu_items.back(), osu_items).value() ==                     \
              osu_items.size() - 1);                                           \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that exists in the list")                           \
        THEN("Return the index of the item")                                   \
        CHECK(func(osu_items[499], osu_items) == 499);                         \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that does not exist in the list")                   \
        THEN("Return no index")                                                \
        CHECK(!func(-1, osu_items).has_value());                               \
      }                                                                        \
    }                                                                          \
    GIVEN("An empty list and an item") {                                       \
      static const std::vector<int> empty_items;                               \
      {                                                                        \
        THEN("Return no index")                                                \
        CHECK(!func(-1, empty_items).has_value());                             \
      }                                                                        \
    }                                                                          \
  }

TEST_BASIC_SEARCH("Linear Search", "[search::linear]", search::linear);
TEST_BASIC_SEARCH("Binary Search", "[search::binary]", search::binary);
TEST_BASIC_SEARCH("Ternary Search", "[search::ternary]", search::ternary);
TEST_BASIC_SEARCH("Kary (1) Search", "[search::kary]", std::bind(search::kary<int>, 1, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SEARCH("Kary (2) Search", "[search::kary]", std::bind(search::kary<int>, 2, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SEARCH("Kary (3) Search", "[search::kary]", std::bind(search::kary<int>, 3, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SEARCH("Kary (5) Search", "[search::kary]", std::bind(search::kary<int>, 5, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SEARCH("Kary (8) Search", "[search::kary]", std::bind(search::kary<int>, 8, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SEARCH("Kary (13) Search", "[search::kary]", std::bind(search::kary<int>, 13, std::placeholders::_1, std::placeholders::_2));
TEST_BASIC_SORT("Insertion Sort", "[sort::insertion]", sort::insertion);
TEST_BASIC_SORT("Selection Sort", "[sort::selection]", sort::selection);
TEST_BASIC_SORT("Merge Sort", "[sort::merge]", sort::merge);
TEST_BASIC_SORT("Quick Sort", "[sort::quick]", sort::quick);
TEST_BASIC_SORT("Bubble Sort", "[sort::bubble]", sort::bubble);

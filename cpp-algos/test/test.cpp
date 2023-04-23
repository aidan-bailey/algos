#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <optional>
#include <string>
#include <vector>
#define CATCH_CONFIG_MAIN
#include "search.hpp"
#include "sort.hpp"
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

#define TEST_BASIC_SORT(name, funcname, func)                                  \
  TEST_CASE(name, funcname) {                                                  \
    GIVEN("An unsorted list") {                                                \
      THEN("Return a sorted list") {                                           \
        std::vector<int> items_copy(items);                                    \
        func(items_copy);                                                      \
        CHECK(check_sorted_asc(&items_copy[0], items_copy.size()));                                   \
      }                                                                        \
    }                                                                          \
  }

#define TEST_BASIC_SEARCH(name, funcname, func)                                \
  TEST_CASE(name, funcname) {                                                  \
    GIVEN("An even length integer list and an item") {                         \
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
      {                                                                        \
        THEN("Return no index")                                                \
        CHECK(!func(-1, empty_items).has_value());                             \
      }                                                                        \
    }                                                                          \
  }

#define TEST_ARY_SEARCH(name, k) TEST_BASIC_SEARCH(name, "[search::kary]", )

std::vector<int> gen_vec(size_t size, bool sorted, bool unique) {
  std::vector<int> result(size, 0);
  std::set<int> used;
  for (size_t i = 0; i < size; i++) {
    size_t n = rand();
    if (unique) {
      while (used.find(n) != used.end())
        n = rand();
      used.insert(n);
    }
    result[i] = n;
  }
  if (sorted)
    std::sort(result.begin(), result.end());
  return result;
}

// EVEN SORTED UNIQUE
static const std::vector<int> esu_items(gen_vec(1000, true, true));
// ODD SORTED UNIQUE
static const std::vector<int> osu_items(gen_vec(1001, true, true));
// EMPTY
static const std::vector<int> empty_items;
static const std::vector<int> items(gen_vec(1000, false, false));

TEST_BASIC_SEARCH("Linear Search", "[search::linear]", search::linear);
TEST_BASIC_SEARCH("Binary Search", "[search::binary]", search::binary);
TEST_BASIC_SEARCH("Ternary Search", "[search::ternary]", search::ternary);
TEST_BASIC_SORT("Insertion Sort", "[sort::insertion]", sort::insertion);
TEST_BASIC_SORT("Quick Sort", "[sort::quick]", sort::quick);

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

#define TEST_BASIC_SEARCH(name, funcname, func)                                \
  TEST_CASE(name, funcname) {                                                  \
    GIVEN("An even length integer list and an item") {                         \
      {                                                                        \
        WHEN("It's the first item in the list")                                \
        THEN("Return 0")                                                       \
        CHECK(func(esu_items[0], esu_items).value() == 0);                   \
      }                                                                        \
      {                                                                        \
        WHEN("It's the last item in the list")                                 \
        THEN("Return index of the last item")                                  \
        CHECK(func(esu_items.back(), esu_items).value() ==                   \
                esu_items.size() - 1);                                         \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that exists in the list")                           \
        THEN("Return the index of the item")                                   \
        CHECK(func(esu_items[499], esu_items) == 499);                       \
      }                                                                        \
      {                                                                        \
        WHEN("It's an item that does not exist in the list")                   \
        THEN("Return no index")                                                \
        CHECK(!func(-1, esu_items).has_value());                             \
      }                                                                        \
    }                                                                          \
    GIVEN("An empty list and an item") {                                       \
      {                                                                        \
        THEN("Return no index")                                                \
        CHECK(!func(-1, empty_items).has_value());                           \
      }                                                                        \
    }                                                                          \
  }

#define TEST_ARY_SEARCH(name, k) TEST_BASIC_SEARCH(name, "[search::kary]", )

bool check_sorted_asc(int *arr, int n) {
  for (--n; n > 0; n--)
    if (*(arr + n) < *(arr + n - 1))
      return false;
  return true;
}

std::vector<int> gen_int_sorted_unique_vec(size_t size) {
  std::vector<int> result(size, 0);
  std::set<int> used;
  for (size_t i = 0; i < size; i++) {
    size_t n;
    while (used.find(n = rand()) != used.end())
      ;
    result[i] = n;
    used.insert(n);
  }
  std::sort(result.begin(), result.end());
  return result;
}

// EVEN SORTED UNIQUE
static const std::vector<int> esu_items(gen_int_sorted_unique_vec(1000000));
// ODD SORTED UNIQUE
static const std::vector<int> osu_items(gen_int_sorted_unique_vec(1000001));
// EMPTY
static const std::vector<int> empty_items;

TEST_BASIC_SEARCH("Linear Search", "[search::linear]", search::linear);
TEST_BASIC_SEARCH("Binary Search", "[search::binary]", search::binary);
TEST_BASIC_SEARCH("Ternary Search", "[search::ternary]", search::ternary);

/*
SCENARIO("Sorting Algorithms") {
  GIVEN("An unsorted int array of size n") {
    std::vector<int> items{8, 7, 0,   4, -7, -8, 3,  1,  -1, -5, -4,
                           2, 6, -10, 5, 10, -2, -6, -9, -3, 9};
    std::vector<int> truth{-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0,
                           1,   2,  3,  4,  5,  6,  7,  8,  9,  10};
    WHEN("Insertion Sort") {
      sort::insertion(items);
      THEN("Array is sorted") { REQUIRE(items == truth); }
    }
    // WHEN("Quick Sort") {
    //   int *temp = arr;
    //   sort::quick(temp, n);
    //   THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    // }
    // WHEN("Merge Sort") {
    //   int *temp = arr;
    //   sort::merge(temp, n);
    //   THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    // }
  }
}
*/

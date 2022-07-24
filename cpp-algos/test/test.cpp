#include <optional>
#include <vector>
#define CATCH_CONFIG_MAIN
#include "search.hpp"
#include "sort.hpp"
#include <catch2/catch_test_macros.hpp>
#include <iostream>

using namespace algos;

bool check_sorted_asc(int *arr, int n) {
  for (--n; n > 0; n--)
    if (*(arr + n) < *(arr + n - 1))
      return false;
  return true;
}

SCENARIO("Searching Algorithms") {
  GIVEN("A sorted int array of size n") {
    std::vector<int> arr{-9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int key = 9;
    size_t sol = 18;
    WHEN("Linear Search") {
      std::optional<size_t> index = search::linear(key, arr);
      THEN("Return index of key") { REQUIRE(index.value() == sol); }
    }
    WHEN("Binary Search") {
      std::optional<size_t> index = search::binary(key, arr);
      THEN("Return index of key") { REQUIRE(index.value() == sol); }
    }
    WHEN("Ternary Search") {
      std::optional<size_t> index = search::ternary(key, arr);
      THEN("Return index of key") { REQUIRE(index.value() == sol); }
    }
  }
}

SCENARIO("Sorting Algorithms") {
  GIVEN("An unsorted int array of size n") {
    int arr[]{5, 4, -8, 3, 2, 0, -5, 8, -9, -6, 9, -7, -2, -3, 7, -4, 6, -1, 1};
    int n = 19;
    WHEN("Insertion Sort") {
      int *temp = arr;
      sort::insertion(temp, n);
      THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    }
    WHEN("Quick Sort") {
      int *temp = arr;
      sort::quick(temp, n);
      THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    }
    WHEN("Merge Sort") {
      int *temp = arr;
      sort::merge(temp, n);
      THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    }
  }
}

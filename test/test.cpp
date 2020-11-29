#define CATCH_CONFIG_MAIN
#include "catch.hpp"
#include "searching/binary_search.hpp"
#include "searching/linear_search.hpp"
#include "searching/ternary_search.hpp"
#include "sorting/insertion_sort.hpp"
#include "sorting/quick_sort.hpp"
#include <iostream>

bool check_sorted_asc(int *arr, int n) {
  for (--n; n > 0; n--)
    if (*(arr + n) < *(arr + n - 1))
      return false;
  return true;
}

SCENARIO("Searching Algorithms") {
  GIVEN("A sorted int array of size n") {
    int arr[]{-9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int n = 19;
    int key = 9;
    int sol = 18;
    WHEN("Linear Search") {
      int index = search::linear_search<int>(key, arr, n);
      THEN("Return index of key") { REQUIRE(index == sol); }
    }
    WHEN("Binary Search") {
      int index = search::binary_search<int>(key, arr, n);
      THEN("Return index of key") { REQUIRE(index == sol); }
    }
    WHEN("Ternary Search") {
      int index = search::ternary_search<int>(key, arr, n);
      THEN("Return index of key") { REQUIRE(index == sol); }
    }
  }
}

SCENARIO("Sorting Algorithms") {
  GIVEN("An unsorted int array of size n ") {
    int arr[]{5, 4, -8, 3, 2, 0, -5, 8, -9, -6, 9, -7, -2, -3, 7, -4, 6, -1, 1};
    int n = 19;
    WHEN("Insertion Sort") {
      int *temp = arr;
      sort::insertion_sort<int>(temp, n);
      THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    }
    WHEN("Quick Sort") {
      int *temp = arr;
      sort::quick_sort<int>(temp, n);
      THEN("Array is sorted") { REQUIRE(check_sorted_asc(temp, n) == true); }
    }
  }
}

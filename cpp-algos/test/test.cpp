#include <optional>
#include <string>
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
  size_t index = 7;
  GIVEN("Vector of Integers") {
    std::vector<int> items{-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500};
    int item_hit = 50;
    int item_miss = 42;
    WHEN("Linear Search (Hit)") {
      std::optional<size_t> result = search::linear(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Linear Search (Miss)") {
      std::optional<size_t> result = search::linear(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Binary Search (Hit)") {
      std::optional<size_t> result = search::binary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Binary Search (Miss)") {
      std::optional<size_t> result = search::binary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Ternary Search (Hit)") {
      std::optional<size_t> result = search::ternary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Ternary Search (Miss)") {
      std::optional<size_t> result = search::ternary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
  }
  GIVEN("Vector of Floats") {
    std::vector<float> items{
        -1240.51029314, -620.152300001, -50.09532, -12.13520, 0.11142,
        5.05923,        34.11113113,    50.094290, 123.12451, 500.05393};
    float item_hit = 50.094290;
    float item_miss = 42.42;
    WHEN("Linear Search (Hit)") {
      std::optional<size_t> result = search::linear(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Linear Search (Miss)") {
      std::optional<size_t> result = search::linear(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Binary Search (Hit)") {
      std::optional<size_t> result = search::binary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Binary Search (Miss)") {
      std::optional<size_t> result = search::binary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Ternary Search (Hit)") {
      std::optional<size_t> result = search::ternary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Ternary Search (Miss)") {
      std::optional<size_t> result = search::ternary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
  }
  GIVEN("Vector of Chars") {
    std::vector<char> items{'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'};
    char item_hit = 'h';
    char item_miss = 'z';
    WHEN("Linear Search (Hit)") {
      std::optional<size_t> result = search::linear(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Linear Search (Miss)") {
      std::optional<size_t> result = search::linear(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Binary Search (Hit)") {
      std::optional<size_t> result = search::binary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Binary Search (Miss)") {
      std::optional<size_t> result = search::binary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Ternary Search (Hit)") {
      std::optional<size_t> result = search::ternary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Ternary Search (Miss)") {
      std::optional<size_t> result = search::ternary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
  }
  GIVEN("Vector of Strings") {
    std::vector<std::string> items{"ab", "bc", "cd", "de", "ef",
                                   "fg", "gh", "hi", "ij", "jk"};
    std::string item_hit = "hi";
    std::string item_miss = "zz";
    WHEN("Linear Search (Hit)") {
      std::optional<size_t> result = search::linear(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Linear Search (Miss)") {
      std::optional<size_t> result = search::linear(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Binary Search (Hit)") {
      std::optional<size_t> result = search::binary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Binary Search (Miss)") {
      std::optional<size_t> result = search::binary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
    WHEN("Ternary Search (Hit)") {
      std::optional<size_t> result = search::ternary(item_hit, items);
      THEN("Find the item") { REQUIRE(result.value() == index); }
    }
    WHEN("Ternary Search (Miss)") {
      std::optional<size_t> result = search::ternary(item_miss, items);
      THEN("Find the item") { REQUIRE(!result.has_value()); }
    }
  }
}

SCENARIO("Sorting Algorithms") {
  GIVEN("An unsorted int array of size n") {
    std::vector<int> items{8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9};
    std::vector<int> truth{-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    WHEN("Insertion Sort") {
      sort::insertion<int>(items);
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

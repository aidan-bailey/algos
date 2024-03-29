using Algos.Searching
using Algos.Sorting
using Test

function search_test(search_func, items, item, result, description)
    @testset "$description" begin
        @test search_func(items, item) == result
    end
end

function test_basic_search(search_func, search_name)
    @testset verbose = true "$search_name search" begin
        search_test(search_func, [1, 2, 3, 4], 1, 1, "even list first item")
        search_test(search_func, [1, 2, 3, 4], 4, 4, "even list last item")
        search_test(search_func, [1, 2, 3, 4], 3, 3, "even list inner item")
        search_test(search_func, [1, 2, 3, 4], 5, nothing, "even list")
        search_test(search_func, [1, 2, 3, 4, 5], 1, 1, "odd list first item")
        search_test(search_func, [1, 2, 3, 4, 5], 5, 5, "odd list last item")
        search_test(search_func, [1, 2, 3, 4, 5], 3, 3, "odd list inner item")
        search_test(search_func, [1, 2, 3, 4, 5], 6, nothing, "odd list")
        search_test(search_func, [], 6, nothing, "empty list")
        search_test(search_func, [1], 6, nothing, "n1 no item")
        search_test(search_func, [1], 1, 1, "n1 found item")
    end
end

@testset verbose = true "Searching" begin
    test_basic_search(linear, "linear")
    test_basic_search(binary, "binary")
    test_basic_search(ternary, "ternary")
    test_basic_search(((items, item, ) -> kary(1, items, item)), "1ary")
    test_basic_search(((items, item, ) -> kary(2, items, item)), "2ary")
    test_basic_search(((items, item, ) -> kary(3, items, item)), "3ary")
    test_basic_search(((items, item, ) -> kary(5, items, item)), "5ary")
    test_basic_search(((items, item, ) -> kary(8, items, item)), "8ary")
    test_basic_search(((items, item, ) -> kary(13, items, item)), "13ary")
end

function sort_test(sort_func, items, description)
    @testset "$description" begin
        @test sort_func(items) == sort(items)
    end
end

function test_basic_sort(sort_func, sort_name)
    @testset verbose = true "$sort_name sort" begin
        sort_test(sort_func, [], "n0")
        sort_test(sort_func, [1], "n1")
        sort_test(sort_func, [2, 1], "n2")
        sort_test(sort_func, [3, 2, 1], "n3")
    end
end

@testset verbose = true "Sorting" begin
    test_basic_sort(insertion!, "insertion")
    test_basic_sort(selection!, "selection")
    test_basic_sort(Sorting.merge!, "merge")
    test_basic_sort(Sorting.quick!, "quick")
    test_basic_sort(Sorting.bubble!, "bubble")
end

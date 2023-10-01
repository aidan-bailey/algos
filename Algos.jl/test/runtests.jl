using Algos.Searching
using Test

function search_test(search_func, items, item, result, description)
    @testset "$description" begin
        @test search_func(items, item) == result
    end
end

function test_basic_search(search_func, function_name)
    @testset verbose = true "$function_name search" begin
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
end

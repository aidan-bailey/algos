use algos_lib::searching;
use test_case::test_case;
#[macro_use]
extern crate partial_application;

macro_rules! test_basic_search {
    ($search_func:expr, $search_func_name:expr) => {
        paste::item! {
            #[test_case(vec![1, 2, 3, 4], 1, Some(0); "even_list_first item")]
            #[test_case(vec![1, 2, 3, 4], 4, Some(3); "even_list_last_item")]
            #[test_case(vec![1, 2, 3, 4], 3, Some(2); "even_list_inner_item")]
            #[test_case(vec![1, 2, 3, 4], 5, None; "even_list_non_item")]
            #[test_case(vec![1, 2, 3, 4, 5], 1, Some(0); "odd_list_first item")]
            #[test_case(vec![1, 2, 3, 4, 5], 5, Some(4); "odd_list_last_item")]
            #[test_case(vec![1, 2, 3, 4, 5], 3, Some(2); "odd_list_inner_item")]
            #[test_case(vec![1, 2, 3, 4, 5], 6, None; "odd_list_no_item")]
            #[test_case(vec![], 6, None; "empty_list_no_item")]
            #[test_case(vec![1], 6, None; "one_list_no_item")]
            #[test_case(vec![1], 1, Some(0); "one_list_found_item")]
            fn [< $search_func_name _ search >]<T: std::cmp::PartialOrd>(items: Vec<T>, item: T, result: Option<usize>) {
                assert_eq!(result, $search_func(&item, &items))
            }
        }
    };
}

test_basic_search!(&searching::linear, "linear");
test_basic_search!(&searching::binary, "binary");
test_basic_search!(&searching::ternary, "ternary");
test_basic_search!(&partial!(searching::kary, _, _, 1), "kary_1");
test_basic_search!(&partial!(searching::kary, _, _, 2), "kary_2");
test_basic_search!(&partial!(searching::kary, _, _, 3), "kary_3");
test_basic_search!(&partial!(searching::kary, _, _, 5), "kary_5");
test_basic_search!(&partial!(searching::kary, _, _, 8), "kary_8");
test_basic_search!(&partial!(searching::kary, _, _, 13), "kary_13");

use algos_lib::sorting;
use test_case::test_case;

macro_rules! test_basic_sort {
    ($sort_func:expr, $sort_name:expr) => {
        paste::item! {
            #[test_case(vec![4, 3, 2, 1], vec![1, 2, 3, 4]; "n4")]
            #[test_case(vec![3, 2, 1], vec![1, 2, 3]; "n3")]
            #[test_case(vec![2, 1], vec![1, 2]; "n2")]
            #[test_case(vec![1], vec![1]; "n1")]
            fn [< $sort_name _ sort >]<T: std::marker::Copy + std::fmt::Debug + std::cmp::PartialOrd>(items: Vec<T>, result: Vec<T>) {
                let res = $sort_func(items);
                assert_eq!(&result, res);
            }
        }
    };
}

test_basic_sort!(&sorting::insertion, "insertion");
test_basic_sort!(&sorting::selection, "selection");
test_basic_sort!(&sorting::merge, "merge");
test_basic_sort!(&sorting::quick, "quick");
test_basic_sort!(&sorting::bubble, "bubble");

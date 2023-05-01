pub mod searching;
pub mod sorting;

#[cfg(test)]
mod search_tests {
    use crate::searching;

    ///////////////////
    // LINEAR SEARCH //
    ///////////////////

    #[test]
    fn linear_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::linear(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn linear_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::linear(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn linear_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::linear(&item, &items);
        assert!(result.is_none());
    }

    ///////////////////
    // BINARY SEARCH //
    ///////////////////

    #[test]
    fn binary_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::binary(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn binary_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::binary(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn binary_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::binary(&item, &items);
        assert!(result.is_none());
    }

    ////////////////////
    // TERNARY SEARCH //
    ////////////////////

    #[test]
    fn ternary_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::ternary(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn ternary_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::ternary(&item, &items);
        assert!(result.is_none());
    }

    #[test]
    fn ternary_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::ternary(&item, &items);
        assert!(result.is_none());
    }

    /////////////////
    // KARY SEARCH //
    /////////////////

    #[test]
    fn kary_1_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 1);
        assert!(result.is_none());
    }

    #[test]
    fn kary_1_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 1);
        assert!(result.is_none());
    }

    #[test]
    fn kary_1_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 1);
        assert!(result.is_none())
    }

    #[test]
    fn kary_2_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 2);
        assert!(result.is_none());
    }

    #[test]
    fn kary_2_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 2);
        assert!(result.is_none());
    }

    #[test]
    fn kary_2_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 2);
        assert!(result.is_none())
    }

    #[test]
    fn kary_3_search_none() {
        let items: Vec<i64> = vec![];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 3);
        assert!(result.is_none());
    }

    #[test]
    fn kary_3_search_one_hit() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_one_miss() {
        let items: Vec<i64> = vec![0];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 3);
        assert!(result.is_none());
    }

    #[test]
    fn kary_3_search_many_first() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = -10;
        let result = searching::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 0)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_many_last() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 10;
        let result = searching::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 20)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_many_middle() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 0;
        let result = searching::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 10)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_many_miss() {
        let items: Vec<i64> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let item: i64 = 42;
        let result = searching::kary(&item, &items, 3);
        assert!(result.is_none())
    }
}

#[cfg(test)]
mod sort_tests {
    use crate::sorting;

    ////////////////////
    // INSERTION SORT //
    ////////////////////

    #[test]
    fn insertion_sort() {
        let mut items: Vec<i64> = vec![
            8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9,
        ];
        sorting::insertion(&mut items);
        assert_eq!(
            items,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    ////////////////////
    // SELECTION SORT //
    ////////////////////

    #[test]
    fn selection_sort() {
        let mut items: Vec<i64> = vec![
            8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9,
        ];
        sorting::selection(&mut items);
        assert_eq!(
            items,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    ////////////////
    // MERGE SORT //
    ////////////////

    #[test]
    fn merge_sort() {
        let items: Vec<i64> = vec![
            8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9,
        ];
        let result = sorting::merge(items);
        assert_eq!(
            result,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    ////////////////
    // QUICK SORT //
    ////////////////

    #[test]
    fn quick_sort() {
        let mut items: Vec<i64> = vec![
            8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9,
        ];
        sorting::quick(&mut items);
        assert_eq!(
            items,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    /////////////////
    // BUBBLE SORT //
    /////////////////

    #[test]
    fn bubble_sort() {
        let mut items: Vec<i64> = vec![
            8, 7, 0, 4, -7, -8, 3, 1, -1, -5, -4, 2, 6, -10, 5, 10, -2, -6, -9, -3, 9,
        ];
        sorting::bubble(&mut items);
        assert_eq!(
            items,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }
}

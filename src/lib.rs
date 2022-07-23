pub mod search;
pub mod sort;

#[cfg(test)]
mod search_tests {
    use crate::search;

    ///////////////////
    // LINEAR SEARCH //
    ///////////////////

    #[test]
    fn linear_search_i32_hit() {
        let items: Vec<i32> = vec![0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let item: i32 = 4;
        let result = search::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_i32_miss() {
        let items: Vec<i32> = vec![0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let item: i32 = -1;
        let result = search::linear(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_f32_hit() {
        let items: Vec<f32> = vec![
            0.0,
            -2.5,
            5.123,
            3.10425,
            -10.213000,
            50.22222333,
            -12.123123,
            4.0425,
            200.9952,
            -42.10042,
        ];
        let item: f32 = 4.0425;
        let result = search::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_f32_miss() {
        let items: Vec<f32> = vec![
            0.0,
            -2.5,
            5.123,
            3.10425,
            -10.213000,
            50.22222333,
            -12.123123,
            4.0425,
            200.9952,
            -42.10042,
        ];
        let item: f32 = 42.42;
        let result = search::linear(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::linear(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::linear(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::linear(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    ///////////////////
    // BINARY SEARCH //
    ///////////////////

    #[test]
    fn binary_search_i32_hit() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = 50;
        let result = search::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_i32_miss() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = -1;
        let result = search::binary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn binary_search_f32_hit() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = 50.094290;
        let result = search::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_f32_miss() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = -1.4242;
        let result = search::binary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn binary_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::binary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn binary_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::binary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    ////////////////////
    // TERNARY SEARCH //
    ////////////////////

    #[test]
    fn ternary_search_i32_hit() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = 50;
        let result = search::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_i32_miss() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = -1;
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn ternary_search_f32_hit() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = 50.094290;
        let result = search::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_f32_miss() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = -1.4242;
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn ternary_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn ternary_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    /////////////////
    // KARY SEARCH //
    /////////////////

    #[test]
    fn kary_1_search_i32_hit() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = 50;
        let result = search::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_i32_miss() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = -1;
        let result = search::kary(&item, &items, 1);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_1_search_f32_hit() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = 50.094290;
        let result = search::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_f32_miss() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = -1.4242;
        let result = search::kary(&item, &items, 1);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_1_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::kary(&item, &items, 1);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_1_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::kary(&item, &items, 1);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_1_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::kary(&item, &items, 1);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_2_search_i32_hit() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = 50;
        let result = search::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_i32_miss() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = -1;
        let result = search::kary(&item, &items, 2);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_2_search_f32_hit() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = 50.094290;
        let result = search::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_f32_miss() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = -1.4242;
        let result = search::kary(&item, &items, 2);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_2_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::kary(&item, &items, 2);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_2_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::kary(&item, &items, 2);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_2_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::kary(&item, &items, 2);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_3_search_i32_hit() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = 50;
        let result = search::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_i32_miss() {
        let items: Vec<i32> = vec![-1240, -620, -50, -12, 0, 5, 34, 50, 123, 500];
        let item: i32 = -1;
        let result = search::kary(&item, &items, 3);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_3_search_f32_hit() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = 50.094290;
        let result = search::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_f32_miss() {
        let items: Vec<f32> = vec![
            -1240.51029314,
            -620.152300001,
            -50.09532,
            -12.13520,
            0.11142,
            5.05923,
            34.11113113,
            50.094290,
            123.12451,
            500.05393,
        ];
        let item: f32 = -1.4242;
        let result = search::kary(&item, &items, 3);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_3_search_char_hit() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'h';
        let result = search::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_char_miss() {
        let items: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item: char = 'k';
        let result = search::kary(&item, &items, 3);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn kary_3_search_str_hit() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "hi";
        let result = search::kary(&item, &items, 3);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn kary_3_search_str_miss() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::kary(&item, &items, 3);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }
}

#[cfg(test)]
mod sort_tests {
    use crate::sort;

    ////////////////////
    // INSERTION SORT //
    ////////////////////

    #[test]
    fn insertion_sort_i32() {
        let items: Vec<i32> = vec![0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let result = sort::insertion(&items);
        assert_eq!(result, vec![-42, -12, -10, -2, 0, 3, 4, 5, 50, 200])
    }

    #[test]
    fn insertion_sort_f32() {
        let items: Vec<f32> = vec![
            0.0,
            -2.5,
            5.123,
            3.10425,
            -10.213000,
            50.22222333,
            -12.123123,
            4.0425,
            200.9952,
            -42.10042,
        ];
        let result = sort::insertion(&items);
        assert_eq!(
            result,
            vec![
                -42.10042,
                -12.123123,
                -10.213000,
                -2.5,
                0.0,
                3.10425,
                4.0425,
                5.123,
                50.22222333,
                200.9952
            ]
        )
    }

    #[test]
    fn insertion_sort_char() {
        let items: Vec<char> = vec!['e', 'i', 'g', 'f', 'h', 'd', 'j', 'a', 'b', 'c'];
        let result = sort::insertion(&items);
        assert_eq!(
            result,
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
        )
    }

    #[test]
    fn insertion_sort_str() {
        let items: Vec<&str> = vec!["de", "ef", "hi", "ab", "bc", "jk", "gh", "cd", "ij", "fg"];
        let result = sort::insertion(&items);
        assert_eq!(
            result,
            vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"]
        )
    }

    ////////////////////
    // SELECTION SORT //
    ////////////////////

    #[test]
    fn selection_sort_i32() {
        let items: Vec<i32> = vec![0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let result = sort::selection(&items);
        assert_eq!(result, vec![-42, -12, -10, -2, 0, 3, 4, 5, 50, 200])
    }

    #[test]
    fn selection_sort_f32() {
        let items: Vec<f32> = vec![
            0.0,
            -2.5,
            5.123,
            3.10425,
            -10.213000,
            50.22222333,
            -12.123123,
            4.0425,
            200.9952,
            -42.10042,
        ];
        let result = sort::selection(&items);
        assert_eq!(
            result,
            vec![
                -42.10042,
                -12.123123,
                -10.213000,
                -2.5,
                0.0,
                3.10425,
                4.0425,
                5.123,
                50.22222333,
                200.9952
            ]
        )
    }

    #[test]
    fn selection_sort_char() {
        let items: Vec<char> = vec!['e', 'i', 'g', 'f', 'h', 'd', 'j', 'a', 'b', 'c'];
        let result = sort::selection(&items);
        assert_eq!(
            result,
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
        )
    }

    #[test]
    fn selection_sort_str() {
        let items: Vec<&str> = vec!["de", "ef", "hi", "ab", "bc", "jk", "gh", "cd", "ij", "fg"];
        let result = sort::selection(&items);
        assert_eq!(
            result,
            vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"]
        )
    }

    ////////////////
    // MERGE SORT //
    ////////////////

    #[test]
    fn merge_sort_i32() {
        let items: Vec<i32> = vec![0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let result = sort::merge(&items);
        assert_eq!(result, vec![-42, -12, -10, -2, 0, 3, 4, 5, 50, 200])
    }

    #[test]
    fn merge_sort_f32() {
        let items: Vec<f32> = vec![
            0.0,
            -2.5,
            5.123,
            3.10425,
            -10.213000,
            50.22222333,
            -12.123123,
            4.0425,
            200.9952,
            -42.10042,
        ];
        let result = sort::merge(&items);
        assert_eq!(
            result,
            vec![
                -42.10042,
                -12.123123,
                -10.213000,
                -2.5,
                0.0,
                3.10425,
                4.0425,
                5.123,
                50.22222333,
                200.9952
            ]
        )
    }

    #[test]
    fn merge_sort_char() {
        let items: Vec<char> = vec!['e', 'i', 'g', 'f', 'h', 'd', 'j', 'a', 'b', 'c'];
        let result = sort::merge(&items);
        assert_eq!(
            result,
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
        )
    }

    #[test]
    fn merge_sort_str() {
        let items: Vec<&str> = vec!["de", "ef", "hi", "ab", "bc", "jk", "gh", "cd", "ij", "fg"];
        let result = sort::merge(&items);
        assert_eq!(
            result,
            vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"]
        )
    }
}

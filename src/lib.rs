pub mod search;

#[cfg(test)]
mod search_tests {
    use crate::search;

    ///////////////////
    // LINEAR SEARCH //
    ///////////////////

    #[test]
    fn linear_search_i32_good() {
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
    fn linear_search_i32_bad() {
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
    fn linear_search_f32_good() {
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
    fn linear_search_f32_bad() {
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
    fn linear_search_char_good() {
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
    fn linear_search_char_bad() {
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
    fn linear_search_str_good() {
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
    fn linear_search_str_bad() {
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
    fn binary_search_i32_good() {
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
    fn binary_search_i32_bad() {
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
    fn binary_search_f32_good() {
        let items: Vec<f32> = vec![-1240.51029314, -620.152300001, -50.09532, -12.13520, 0.11142, 5.05923, 34.11113113, 50.094290, 123.12451, 500.05393];
        let item: f32 = 50.094290;
        let result = search::binary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn binary_search_f32_bad() {
        let items: Vec<f32> = vec![-1240.51029314, -620.152300001, -50.09532, -12.13520, 0.11142, 5.05923, 34.11113113, 50.094290, 123.12451, 500.05393];
        let item: f32 = -1.4242;
        let result = search::binary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn binary_search_char_good() {
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
    fn binary_search_char_bad() {
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
    fn binary_search_str_good() {
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
    fn binary_search_str_bad() {
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
    fn ternary_search_i32_good() {
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
    fn ternary_search_i32_bad() {
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
    fn ternary_search_f32_good() {
        let items: Vec<f32> = vec![-1240.51029314, -620.152300001, -50.09532, -12.13520, 0.11142, 5.05923, 34.11113113, 50.094290, 123.12451, 500.05393];
        let item: f32 = 50.094290;
        let result = search::ternary(&item, &items);
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn ternary_search_f32_bad() {
        let items: Vec<f32> = vec![-1240.51029314, -620.152300001, -50.09532, -12.13520, 0.11142, 5.05923, 34.11113113, 50.094290, 123.12451, 500.05393];
        let item: f32 = -1.4242;
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn ternary_search_char_good() {
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
    fn ternary_search_char_bad() {
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
    fn ternary_search_str_good() {
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
    fn ternary_search_str_bad() {
        let items: Vec<&str> = vec!["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item: &str = "kl";
        let result = search::ternary(&item, &items);
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }


}

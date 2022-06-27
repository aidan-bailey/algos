pub mod search;

#[cfg(test)]
mod search_tests {
    use crate::search;

    #[test]
    fn linear_search_i32_good() {
        let items = [0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let item = 4;
        let result = search::linear(&item, items.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_i32_bad() {
        let items = [0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        let item = -1;
        let result = search::linear(&item, items.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_f32_good() {
        let items = [0.0, -2.5, 5.123, 3.10425, -10.213000, 50.22222333, -12.123123, 4.0425, 200.9952, -42.10042];
        let item = 4.0425;
        let result = search::linear(&item, items.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_f32_bad() {
        let items = [0.0, -2.5, 5.123, 3.10425, -10.213000, 50.22222333, -12.123123, 4.0425, 200.9952, -42.10042];
        let item = 42.42;
        let result = search::linear(&item, items.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_char_good() {
        let items = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item = 'h';
        let result = search::linear(&item, items.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_char_bad() {
        let items = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        let item = 'k';
        let result = search::linear(&item, items.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_str_good() {
        let items = ["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item = "hi";
        let result = search::linear(&item, items.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_str_bad() {
        let items = ["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        let item = "kl";
        let result = search::linear(&item, items.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

}

pub mod search;

#[cfg(test)]
mod search_tests {
    use crate::search;

    #[test]
    fn linear_search_i32_good() {
        const ITEMS: [i32; 10] = [0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        const ITEM: i32 = 4;
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_i32_bad() {
        const ITEMS: [i32; 10] = [0, -2, 5, 3, -10, 50, -12, 4, 200, -42];
        const ITEM: i32 = -1;
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_f32_good() {
        const ITEMS: [f32; 10] = [
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
        const ITEM: f32 = 4.0425;
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_f32_bad() {
        const ITEMS: [f32; 10] = [
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
        const ITEM: f32 = 42.42;
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_char_good() {
        const ITEMS: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        const ITEM: char = 'h';
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_char_bad() {
        const ITEMS: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        const ITEM: char = 'k';
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }

    #[test]
    fn linear_search_str_good() {
        const ITEMS: [&str; 10] = ["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        const ITEM: &str = "hi";
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(index) = result {
            assert_eq!(index, 7)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn linear_search_str_bad() {
        const ITEMS: [&str; 10] = ["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
        const ITEM: &str = "kl";
        let result = search::linear(&ITEM, ITEMS.iter());
        if let Some(_) = result {
            assert!(false)
        } else {
            assert!(true)
        }
    }
}

#[macro_export]
macro_rules! my_add {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr, $($b:tt)*) => {
        $a + my_add!($($b)*)
    };
}

#[macro_export]
macro_rules! my_vec {
    // match the empty case
    () => {
        std::vec::Vec::new()
    };
    // match (1;2)
    ($a:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // match (1,2,3) => (@tail v, 2, 3)
    ($a:expr, $($b:tt)*) => {{
        let mut v = std::vec::Vec::new();
        v.push($a);
        my_vec!(@tail v, $($b)*);
        v
    }};
    // match (@tail, v, 2)
    (@tail $v:ident, $a:expr) => {
        $v.push($a);
    };
    // match (@tail, v, 2, 3) => (@tail v, 3)
    (@tail $v:ident, $a:expr, $($b:tt)*) => {
        $v.push($a);
        my_vec!(@tail $v, $($b)*);
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        assert_eq!(my_add!(1, 2, 3, 4), 10);
    }

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(my_vec!(1, 2, 3, 4, 5), v);
    }
}

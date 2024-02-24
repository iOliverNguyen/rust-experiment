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

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        assert_eq!(my_add!(1, 2, 3, 4), 10);
    }
}

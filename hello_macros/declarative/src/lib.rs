#[macro_use]
pub mod declarative;

mod test {
    #[test]
    fn test_add() {
        assert_eq!(my_add!(1, 2, 3, 4), 10);
    }
}

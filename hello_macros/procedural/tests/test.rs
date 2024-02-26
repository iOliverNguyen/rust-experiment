use hello_macros_procedural::*;

#[test]
fn test_0() {
    #[change_struct_name]
    struct OneStruct {};

    let x = MyStruct {}; // OneStruct -> change_struct_name -> MyStruct
}

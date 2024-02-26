extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn change_struct_name(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
        struct MyStruct{};
    ))
}

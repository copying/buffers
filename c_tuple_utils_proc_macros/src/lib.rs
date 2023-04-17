extern crate quote;

use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    inner().into()
}

fn inner() -> TokenStream {
    quote!(let _ = "hi")
}

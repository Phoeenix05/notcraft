use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyDeriveTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl MyDeriveTrait for #ident {
            fn name(&self) {
                println!("{:?}", #ident)
            }
        }
    };
    output.into()
}

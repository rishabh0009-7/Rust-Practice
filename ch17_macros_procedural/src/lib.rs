use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // parse the code 
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;


    // generate code 
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, my name is {}", stringify!(#name));
            }
        }
    };

    expanded.into()
}

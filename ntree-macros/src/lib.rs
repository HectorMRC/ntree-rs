use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoNode)]
pub fn into_node(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let code = quote! {
        impl Into<Node<#ident>> for #ident {
            fn into(self) -> Node<Item> {
                Node::new(self)
            }
        }

        impl #ident {
            fn to_node(self) -> Node<Item> {
                self.into()
            }
        }
    };

    code.into()
}

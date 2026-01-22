extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn vk_handle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let name = &item_struct.ident;

    if let Fields::Unit = &item_struct.fields {
        item_struct.fields = Fields::Unnamed(syn::parse_quote!((core::num::NonZeroUsize)));
        item_struct.semi_token = Some(Default::default());
    }

    let expanded = quote! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        #item_struct

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:#x})", stringify!(#name), self.0.get())
            }
        }
    };

    TokenStream::from(expanded)
}

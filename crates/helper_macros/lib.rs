extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn vk_handle(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let name = &item_struct.ident;

    if attr.is_empty() {
        panic!("vk_handle attribute requires a type");
    }

    let raw_type = parse_macro_input!(attr as syn::Ident);
    let raw_type_str = raw_type.to_string();

    let nonzero_type: syn::Type = match raw_type_str.as_str() {
        "u8" => syn::parse_quote!(core::num::NonZeroU8),
        "u16" => syn::parse_quote!(core::num::NonZeroU16),
        "u32" => syn::parse_quote!(core::num::NonZeroU32),
        "u64" => syn::parse_quote!(core::num::NonZeroU64),
        "usize" => syn::parse_quote!(core::num::NonZeroUsize),
        "i8" => syn::parse_quote!(core::num::NonZeroI8),
        "i16" => syn::parse_quote!(core::num::NonZeroI16),
        "i32" => syn::parse_quote!(core::num::NonZeroI32),
        "i64" => syn::parse_quote!(core::num::NonZeroI64),
        "isize" => syn::parse_quote!(core::num::NonZeroIsize),
        _ => {
            panic!("Unsupported handle type: {raw_type_str}")
        }
    };

    if let Fields::Unit = &item_struct.fields {
        item_struct.fields = Fields::Unnamed(syn::parse_quote!((#nonzero_type)));
        item_struct.semi_token = Some(Default::default());
    }

    let expanded = quote! {
        #[repr(transparent)]
        #[derive(Copy, Eq)]
        #item_struct

        impl #name {
            #[inline]
            pub const fn from_raw(raw: #raw_type) -> Option<Self> {
                let Some(non_zero) = #nonzero_type::new(raw) else {
                    return None;
                };

                Some(Self(non_zero))
            }

            #[inline]
            pub const unsafe fn from_raw_unchecked(raw: #raw_type) -> Self {
                unsafe { Self(#nonzero_type::new_unchecked(raw)) }
            }

            #[inline]
            pub const fn as_raw(self) -> #raw_type {
                self.0.get()
            }
        }

        impl Clone for #name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl core::hash::Hash for #name {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl core::ops::Deref for #name {
            type Target = #raw_type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(&self.0 as *const #nonzero_type as *const Self::Target) }
            }
        }

        impl core::cmp::PartialEq<#name> for #name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl core::cmp::PartialEq<#raw_type> for #name {

            #[inline]
            fn eq(&self, other: &#raw_type) -> bool {
                self.0.get() == *other
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:#x})", stringify!(#name), self.0.get())
            }
        }
    };

    TokenStream::from(expanded)
}

//! Proc macros for naturally deriving basic trait impls for new types, i.e. respecting the
//! structure and semantics of the inner type.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Ident};

/// Derive macro generating an impl with a associated `new` function for a new type.
#[proc_macro_derive(New)]
pub fn derive_new(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "From", |name, field| {
        let ty = &field.ty;
        quote! {
            impl #name {
                pub fn new(value: #ty) -> Self {
                    Self(value)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::convert::From` for a new type.
#[proc_macro_derive(From)]
pub fn derive_into(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "From", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::convert::From<#ty> for #name {
                fn from(value: #ty) -> Self {
                    Self(value)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Add` for a new type.
#[proc_macro_derive(Add)]
pub fn derive_add(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Add", |name, _| {
        quote! {
            impl std::ops::Add for #name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::AddAssign` for a new type.
#[proc_macro_derive(AddAssign)]
pub fn derive_add_assign(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "AddAssign", |name, _| {
        quote! {
            impl std::ops::AddAssign for #name {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 = self.0 + rhs.0;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Sub` for a new type.
#[proc_macro_derive(Sub)]
pub fn derive_sub(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Sub", |name, _| {
        quote! {
            impl std::ops::Sub for #name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self(self.0 - rhs.0)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::SubAssign` for a new type.
#[proc_macro_derive(SubAssign)]
pub fn derive_sub_assign(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "SubAssign", |name, _| {
        quote! {
            impl std::ops::SubAssign for #name {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 = self.0 - rhs.0;
                }
            }
        }
        .into()
    })
}

/// Invoke the given function
fn for_new_type<F>(ast: &DeriveInput, t: &str, f: F) -> TokenStream
where
    F: Fn(&Ident, &Field) -> TokenStream,
{
    match &ast.data {
        Data::Struct(DataStruct {
            struct_token: _,
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }),
            semi_token: _,
        }) if unnamed.len() == 1 => {
            let field = unnamed.first().unwrap();
            f(&ast.ident, field)
        }
        _ => panic!("#[derive({t})] is only defined for newtypes (unary tuple structs)"),
    }
}

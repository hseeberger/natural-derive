//! Proc macros for naturally deriving basic trait impls for new types, i.e. respecting the
//! structure and semantics of the inner type.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Ident};

/// Derive macro generating an impl with a associated `new` function for a new type.
#[proc_macro_derive(New)]
pub fn derive_new(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "New", |name, field| {
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

/// Derive macro generating an impl with a associated `inner` function for a new type.
#[proc_macro_derive(Inner)]
pub fn derive_inner(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Inner", |name, field| {
        let ty = &field.ty;
        quote! {
            impl #name {
                pub fn inner(&self) -> &#ty {
                    &self.0
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::convert::From` for a new type.
#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
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
                    self.0 += rhs.0;
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
                    self.0 -= rhs.0;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Mul` for a new type.
#[proc_macro_derive(Mul)]
pub fn derive_mul(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Mul", |name, _| {
        quote! {
            impl std::ops::Mul for #name {
                type Output = Self;
                fn mul(self, rhs: Self) -> Self::Output {
                    Self(self.0 * rhs.0)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::MulAssign` for a new type.
#[proc_macro_derive(MulAssign)]
pub fn derive_mul_assign(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "MulAssign", |name, _| {
        quote! {
            impl std::ops::MulAssign for #name {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Mul` for a new type.
#[proc_macro_derive(MulScalar)]
pub fn derive_mul_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Mul", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::Mul<#ty> for #name {
                type Output = Self;
                fn mul(self, rhs: #ty) -> Self::Output {
                    Self(self.0 * rhs)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::MulAssign` for a new type.
#[proc_macro_derive(MulAssignScalar)]
pub fn derive_mul_assign_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "MulAssign", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::MulAssign<#ty> for #name {
                fn mul_assign(&mut self, rhs: #ty) {
                    self.0 *= rhs;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Div` for a new type.
#[proc_macro_derive(Div)]
pub fn derive_div(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Div", |name, _| {
        quote! {
            impl std::ops::Div for #name {
                type Output = Self;
                fn div(self, rhs: Self) -> Self::Output {
                    Self(self.0 / rhs.0)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::DivAssign` for a new type.
#[proc_macro_derive(DivAssign)]
pub fn derive_div_assign(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "DivAssign", |name, _| {
        quote! {
            impl std::ops::DivAssign for #name {
                fn div_assign(&mut self, rhs: Self) {
                    self.0 /= rhs.0;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Div` for a new type.
#[proc_macro_derive(DivScalar)]
pub fn derive_div_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Div", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::Div<#ty> for #name {
                type Output = Self;
                fn div(self, rhs: #ty) -> Self::Output {
                    Self(self.0 / rhs)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::DivAssign` for a new type.
#[proc_macro_derive(DivAssignScalar)]
pub fn derive_div_assign_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "DivAssign", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::DivAssign<#ty> for #name {
                fn div_assign(&mut self, rhs: #ty) {
                    self.0 /= rhs;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Rem` for a new type.
#[proc_macro_derive(Rem)]
pub fn derive_rem(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Rem", |name, _| {
        quote! {
            impl std::ops::Rem for #name {
                type Output = Self;
                fn rem(self, rhs: Self) -> Self::Output {
                    Self(self.0 % rhs.0)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::RemAssign` for a new type.
#[proc_macro_derive(RemAssign)]
pub fn derive_rem_assign(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "RemAssign", |name, _| {
        quote! {
            impl std::ops::RemAssign for #name {
                fn rem_assign(&mut self, rhs: Self) {
                    self.0 %= rhs.0;
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::Rem` for a new type.
#[proc_macro_derive(RemScalar)]
pub fn derive_rem_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "Rem", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::Rem<#ty> for #name {
                type Output = Self;
                fn rem(self, rhs: #ty) -> Self::Output {
                    Self(self.0 % rhs)
                }
            }
        }
        .into()
    })
}

/// Derive macro generating an impl of the trait `std::ops::RemAssign` for a new type.
#[proc_macro_derive(RemAssignScalar)]
pub fn derive_rem_assign_scalar(input: TokenStream) -> TokenStream {
    for_new_type(&parse(input).unwrap(), "RemAssign", |name, field| {
        let ty = &field.ty;
        quote! {
            impl std::ops::RemAssign<#ty> for #name {
                fn rem_assign(&mut self, rhs: #ty) {
                    self.0 %= rhs;
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

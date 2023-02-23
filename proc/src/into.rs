//! Provides [the parsing struct](IntoInput) for the [`TryFrom`](crate::derive_try_from) derive macro

use darling::{FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use syn::{Attribute, Ident};

/// Parsing struct for the [`Into`](crate::derive_into) derive macro
#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_unit), forward_attrs(repr))]
struct IntoInput {
	/// Enum identifier
	ident: Ident,
	/// Forwarded attributes
	attrs: Vec<Attribute>,
}

/// Derives a [`Into<repr>`] impl block
pub(crate) fn derive(item: TokenStream) -> darling::Result<TokenStream> {
	use crate::PrimitiveRepresentation;
	use syn::DeriveInput;

	let item: DeriveInput = syn::parse(item)?;
	let IntoInput { ident, attrs } = IntoInput::from_derive_input(&item)?;

	let repr = PrimitiveRepresentation::from_attributes(&attrs)?;

	Ok(quote::quote! {
		#[automatically_derived]
		impl From<#ident> for #repr {
			#[inline]
			fn from(value: #ident) -> Self {
				value as #repr
			}
		}
	}
	.into())
}

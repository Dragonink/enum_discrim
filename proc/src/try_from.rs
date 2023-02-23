//! Provides [the parsing struct](TryFromInput) for the [`TryFrom`](crate::derive_try_from) derive macro

use crate::PrimitiveRepresentation;
use darling::{ast::Data, util::SpannedValue, FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use syn::{Attribute, Ident, Variant};

/// Parsing struct for the [`TryFrom`](crate::derive_try_from) derive macro
#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_unit), forward_attrs(repr))]
struct TryFromInput {
	/// Enum identifier
	ident: Ident,
	/// Variants contained in the enum
	data: Data<SpannedValue<Variant>, ()>,
	/// Forwarded attributes
	attrs: Vec<Attribute>,
}

/// Derives a [`TryFrom<repr>`] impl block
pub(crate) fn derive(item: TokenStream) -> darling::Result<TokenStream> {
	use syn::DeriveInput;

	let item: DeriveInput = syn::parse(item)?;
	let TryFromInput { ident, data, attrs } = TryFromInput::from_derive_input(&item)?;
	let Data::Enum(data) = data else {
		unreachable!()
	};

	let repr = PrimitiveRepresentation::from_attributes(&attrs)?;
	/// Generates a match arm for each given type
	macro_rules! arms_with_ty {
		($( $ty:ident ),* $(,)?) => {
			match repr {$(
				PrimitiveRepresentation::$ty => crate::scan_variants::<$ty>(&data)?
					.into_iter()
					.map(|(variant, value)| {
						let span = variant.span();
						let name = &variant.ident;
						quote::quote_spanned!(span=> #value => Ok(Self::#name),)
					})
					.collect::<Vec<_>>(),
			)*}
		};
	}
	let arms = arms_with_ty![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

	Ok(quote::quote! {
		#[automatically_derived]
		impl TryFrom<#repr> for #ident {
			type Error = enum_discrim::TryFromError;

			#[inline]
			fn try_from(value: #repr) -> Result<Self, Self::Error> {
				match value {
					#(#arms)*
					_ => Err(Self::Error::new(stringify!(#ident))),
				}
			}
		}
	}
	.into())
}

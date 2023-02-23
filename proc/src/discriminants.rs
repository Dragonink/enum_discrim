//! Provides [the parsing structs](DiscriminantsInput) for the [`Discriminants`](crate::derive_discriminants) derive macro

use darling::{ast::Data, util::SpannedValue, FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use syn::{Attribute, Generics, Ident, Variant, Visibility};

/// Parsing struct for the [`Discriminants`](crate::derive_discriminants) derive macro
#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any), forward_attrs(repr))]
struct DiscriminantsInput {
	/// Enum identifier
	ident: Ident,
	/// Enum visibility
	vis: Visibility,
	/// Enum generics
	generics: Generics,
	/// Variants contained in the enum
	data: Data<SpannedValue<Variant>, ()>,
	/// Forwarded attributes
	attrs: Vec<Attribute>,
}

/// Derives an impl block containing the discriminants of all enum variants as consts
pub(crate) fn derive(item: TokenStream) -> darling::Result<TokenStream> {
	use crate::PrimitiveRepresentation;
	use syn::DeriveInput;

	let item: DeriveInput = syn::parse(item)?;
	let DiscriminantsInput {
		ident,
		vis,
		generics,
		data,
		attrs,
	} = DiscriminantsInput::from_derive_input(&item)?;
	let Data::Enum(data) = data else {
		unreachable!()
	};

	let where_clause = &generics.where_clause;
	let repr = PrimitiveRepresentation::from_attributes(&attrs)?;
	/// Generates a match arm for each given type
	macro_rules! discriminants_with_ty {
		($( $ty:ident ),* $(,)?) => {
			match repr {$(
				PrimitiveRepresentation::$ty => crate::scan_variants::<$ty>(&data)?
					.into_iter()
					.map(|(variant, value)| {
						let span = variant.span();
						let name = quote::format_ident!("{}_D", variant.ident);
						let doc = format!("Discriminant of the [{0}](Self::{0}) variant", variant.ident);

						quote::quote_spanned! {span=>
							#[doc = #doc]
							#vis const #name: #repr = #value;
						}
					})
					.collect::<Vec<_>>(),
			)*}
		};
	}
	let discriminants =
		discriminants_with_ty![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

	Ok(quote::quote! {
		#[automatically_derived]
		#[allow(non_upper_case_globals)]
		impl #generics #ident #generics #where_clause {
			/// Returns the discriminant of the given variant
			#vis fn discriminant(&self) -> #repr {
				// SAFETY: Our macro resolves to `compile_error!` if `#[repr]` is missing
				unsafe { <*const Self>::from(self).cast::<#repr>().read() }
			}

			#(#discriminants)*
		}
	}
	.into())
}

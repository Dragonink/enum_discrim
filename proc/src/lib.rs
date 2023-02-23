//! Library providing procedural macros for the `enum_discrim` crate
#![warn(
	unused,
	clippy::unused_self,
	unused_crate_dependencies,
	unused_import_braces,
	unreachable_pub,
	noop_method_call,
	clippy::match_wildcard_for_single_variants,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::match_on_vec_items,
	clippy::imprecise_flops,
	clippy::suboptimal_flops,
	clippy::float_cmp,
	clippy::float_cmp_const,
	clippy::mem_forget,
	clippy::filter_map_next,
	clippy::verbose_file_reads,
	clippy::inefficient_to_string,
	clippy::str_to_string,
	clippy::option_option,
	clippy::dbg_macro,
	clippy::print_stdout,
	clippy::print_stderr,
	missing_debug_implementations,
	missing_copy_implementations,
	clippy::missing_const_for_fn,
	missing_docs,
	clippy::missing_docs_in_private_items,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::needless_continue,
	clippy::manual_let_else,
	clippy::unnested_or_patterns,
	clippy::semicolon_if_nothing_returned,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::macro_use_imports,
	clippy::mod_module_files
)]
#![deny(
	keyword_idents,
	non_ascii_idents,
	unused_must_use,
	clippy::lossy_float_literal,
	clippy::exit
)]
#![forbid(unsafe_code, clippy::missing_panics_doc, clippy::missing_errors_doc)]

use darling::{util::SpannedValue, FromAttributes, FromMeta};
use proc_macro::TokenStream;
use quote::{IdentFragment, ToTokens};
use std::{
	fmt::{self, Debug, Display, Formatter},
	ops::Add,
	str::FromStr,
};
use syn::{Attribute, NestedMeta, Variant};

mod discriminants;
mod into;
mod try_from;

#[allow(missing_docs, clippy::missing_docs_in_private_items)]
#[proc_macro_derive(Discriminants)]
#[inline]
pub fn derive_discriminants(item: TokenStream) -> TokenStream {
	match discriminants::derive(item) {
		Ok(tokens) => tokens,
		Err(err) => err.write_errors().into(),
	}
}

#[allow(missing_docs, clippy::missing_docs_in_private_items)]
#[proc_macro_derive(Into)]
#[inline]
pub fn derive_into(item: TokenStream) -> TokenStream {
	match into::derive(item) {
		Ok(tokens) => tokens,
		Err(err) => err.write_errors().into(),
	}
}

#[allow(missing_docs, clippy::missing_docs_in_private_items)]
#[proc_macro_derive(TryFrom)]
#[inline]
pub fn derive_try_from(item: TokenStream) -> TokenStream {
	match try_from::derive(item) {
		Ok(tokens) => tokens,
		Err(err) => err.write_errors().into(),
	}
}

#[allow(non_camel_case_types, clippy::missing_docs_in_private_items)]
/// Enumeration of possible [primitive representations](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) of an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrimitiveRepresentation {
	u8,
	u16,
	u32,
	u64,
	u128,
	usize,
	i8,
	i16,
	i32,
	i64,
	i128,
	isize,
}
impl FromStr for PrimitiveRepresentation {
	type Err = &'static str;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		/// Generates a match arm for each given type
		macro_rules! impl_match {
			($( $ty:ident ),* $(,)?) => {
				match s {
					$(
						stringify!($ty) => Ok(Self::$ty),
					)*
					_ => Err("Invalid primitive representation"),
				}
			};
		}
		impl_match![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
	}
}
impl FromMeta for PrimitiveRepresentation {
	#[inline]
	fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
		use syn::Meta;

		match item {
			NestedMeta::Meta(Meta::Path(path)) => darling::util::path_to_string(path)
				.parse()
				.map_err(|err| darling::Error::custom(err).with_span(item)),
			_ => Err(darling::Error::custom("Invalid primitive representation").with_span(item)),
		}
	}

	#[inline]
	fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
		items
			.iter()
			.find_map(|item| Self::from_nested_meta(item).ok())
			.ok_or_else(|| darling::Error::custom("#[repr(inttype)] must be specified"))
	}
}
impl FromAttributes for PrimitiveRepresentation {
	#[inline]
	fn from_attributes(attrs: &[Attribute]) -> darling::Result<Self> {
		attrs
			.iter()
			.find(|attr| {
				attr.path
					.get_ident()
					.map(|ident| ident == "repr")
					.unwrap_or_default()
			})
			.ok_or_else(|| darling::Error::custom("#[repr(inttype)] must be specified"))
			.and_then(darling::util::parse_attribute_to_meta_list)
			.and_then(|meta| Self::from_list(&meta.nested.into_iter().collect::<Vec<_>>()))
	}
}
impl Display for PrimitiveRepresentation {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(&self, f)
	}
}
impl IdentFragment for PrimitiveRepresentation {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		Display::fmt(&self, f)
	}
}
impl ToTokens for PrimitiveRepresentation {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		quote::format_ident!("{self}").to_tokens(tokens);
	}
}

/// Wraps an iterator of [`Variant`] to compute the discriminants and return them
#[inline]
fn scan_variants<'v, D>(
	iter: impl IntoIterator<Item = &'v SpannedValue<Variant>>,
) -> darling::Result<Vec<(&'v SpannedValue<Variant>, D)>>
where
	D: Default + FromStr + Increment + Copy,
	D::Err: Display,
{
	use syn::{Expr, ExprLit, Lit};

	let mut accumulator = darling::Error::accumulator();
	let vec = iter
		.into_iter()
		.scan(
			D::default(),
			|d: &mut D, variant: &'v SpannedValue<Variant>| {
				let value = match variant
					.discriminant
					.as_ref()
					.map_or(Ok(*d), |(_eq, value)| match value {
						Expr::Lit(ExprLit { lit, .. }) => match lit {
							Lit::Int(value) => {
								let value: D = value.base10_parse()?;
								// .map_err(|err| darling::Error::custom(err).with_span(value))?;
								*d = value.increment();
								Ok(value)
							}
							lit => Err(darling::Error::unexpected_lit_type(lit)),
						},
						_ => Err(darling::Error::custom(
							"Discriminant must be an integer literal",
						)),
					}) {
					Ok(value) => value,
					Err(err) => {
						return Some(Err(err));
					}
				};
				Some(Ok((variant, value)))
			},
		)
		.filter_map(|res| match res {
			Ok(value) => Some(value),
			Err(err) => {
				accumulator.push(err);
				None
			}
		})
		.collect::<Vec<_>>();
	accumulator.finish().map(|()| vec)
}

/// Utility trait for the [`scan_variants`] function
trait Increment: Add<Output = Self> + Sized {
	/// Returns `self + 1`
	fn increment(self) -> Self;
}
/// Generates an impl [`Increment`] block for each given type
macro_rules! impl_increment {
	($( $ty:ty ),* $(,)?) => {
		$(
			impl Increment for $ty {
				#[inline]
				fn increment(self) -> Self {
					self.wrapping_add(1)
				}
			}
		)*
	};
}
impl_increment![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#![doc = include_str!("../README.md")]
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

use std::{
	error::Error,
	fmt::{self, Display, Formatter},
};

/// Derives an impl block containing the discriminants of all enum variants as consts
///
/// This macro also generates a function to get the discriminant of an instance.
///
/// # Usage
/// You may use this macro on any kind of enum, even with fields and generics:
/// ```
/// use enum_discrim::Discriminants;
///
/// #[derive(Discriminants)]
/// #[repr(u8)]
/// enum E<B, C>
/// where
///     B: Copy,
/// {
///     A,
///     B(B) = 2,
///     C { c: C },
/// }
/// ```
///
/// The only restriction is that you *need* to declare a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) for your enum:
/// ```compile_fail
/// use enum_discrim::Discriminants;
///
/// #[derive(Discriminants)]
/// // COMPILE ERROR: missing #[repr]
/// enum E<B, C>
/// where
///     B: Copy,
/// {
///     A,
///     B(B) = 2,
///     C { c: C },
/// }
/// ```
///
/// ## Generated consts
/// This macro generates one const item for each variant of your enum.
/// Each const will take its related variant's name with a `_D` suffix.
///
/// ## Generated function
/// In addition to the generated consts, a function with the following signature is generated:
/// ```
/// # #[allow(non_camel_case_types)]
/// # type repr = ();
/// # trait Discriminants {
/// fn discriminant(&self) -> repr
/// # ; }
/// ```
/// where `repr` is the [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) of your enum.
///
/// # Example
/// Given the first code block in the [Usage](#usage) section,
/// the macro would roughly generate the following code:
/// ```
/// # #[repr(u8)]
/// # enum E<B, C>
/// # where
/// #    B: Copy,
/// # {
/// #    A,
/// #    B(B) = 2,
/// #    C { c: C },
/// # }
/// impl<B, C> E<B, C>
/// where
///     B: Copy,
/// {
///     fn discriminant(&self) -> u8 {
///         // ...
///         # 0
///     }
///
///     const A_D: u8 = 0;
///     const B_D: u8 = 2;
///     const C_D: u8 = 3;
/// }
/// ```
pub use enum_discrim_proc::Discriminants;

/// Derives a [`Into<repr>`] impl block
///
/// Actually, the generated impl block is `impl From<Self> for repr`.
///
/// # Usage
/// You may use this macro on enums with *only* unit variants:
/// ```
/// use enum_discrim::Into;
///
/// #[derive(Into)]
/// #[repr(u8)]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
/// ```
/// ```compile_fail
/// use enum_discrim::Into;
///
/// #[derive(Into)]
/// #[repr(u8)]
/// // COMPILE ERROR: Not all variants are unit
/// enum E {
///     A,
///     B(u8) = 2,
///     C { n: usize },
/// }
/// ```
/// You also *need* to declare a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) for your enum:
/// ```compile_fail
/// use enum_discrim::Into;
///
/// #[derive(Into)]
/// // COMPILE ERROR: missing #[repr]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
/// ```
///
/// # Example
/// ```
/// use enum_discrim::Into;
///
/// #[derive(Debug, PartialEq, Eq, Into)]
/// #[repr(u8)]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
///
/// assert_eq!(<E as Into<u8>>::into(E::A), 0_u8);
/// assert_eq!(u8::from(E::B), 2_u8);
/// assert_eq!(u8::from(E::C), 3_u8);
/// ```
pub use enum_discrim_proc::Into;

/// Derives a [`TryFrom<repr>`] impl block
///
/// # Usage
/// You may use this macro on enums with *only* unit variants:
/// ```
/// use enum_discrim::TryFrom;
///
/// #[derive(TryFrom)]
/// #[repr(u8)]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
/// ```
/// ```compile_fail
/// use enum_discrim::TryFrom;
///
/// #[derive(TryFrom)]
/// #[repr(u8)]
/// // COMPILE ERROR: Not all variants are unit
/// enum E {
///     A,
///     B(u8) = 2,
///     C { n: usize },
/// }
/// ```
/// You also *need* to declare a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) for your enum:
/// ```compile_fail
/// use enum_discrim::TryFrom;
///
/// #[derive(TryFrom)]
/// // COMPILE ERROR: missing #[repr]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
/// ```
///
/// # Example
/// ```
/// use enum_discrim::TryFrom;
///
/// #[derive(Debug, PartialEq, Eq, TryFrom)]
/// #[repr(u8)]
/// enum E {
///     A,
///     B = 2,
///     C,
/// }
///
/// assert_eq!(E::try_from(0).unwrap(), E::A);
/// assert_eq!(E::try_from(2).unwrap(), E::B);
/// assert_eq!(E::try_from(3).unwrap(), E::C);
/// assert!(E::try_from(1).is_err());
/// ```
pub use enum_discrim_proc::TryFrom;

/// Error returned by [`TryFrom`](crate::TryFrom) implementations
#[derive(Debug, Clone, Copy)]
pub struct TryFromError {
	/// Enum identifier
	ident: &'static str,
}
impl TryFromError {
	#[doc(hidden)]
	#[inline]
	pub const fn new(ident: &'static str) -> Self {
		Self { ident }
	}
}
impl Display for TryFromError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tried to convert an invalid value into a {}", self.ident)
	}
}
impl Error for TryFromError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

// #[cfg(doc)]
pub mod examples;

#[cfg(test)]
mod tests {
	#![allow(dead_code)]

	use self::enum_discrim::*;
	use crate as enum_discrim;

	#[test]
	fn discriminants() {
		#[derive(Debug, Discriminants)]
		#[repr(u8)]
		enum E<B, C>
		where
			B: Copy,
		{
			A,
			B(B) = 2,
			C { c: C },
		}

		type MyE = E<&'static str, i32>;
		assert_eq!(MyE::A_D, 0_u8);
		assert_eq!(MyE::B_D, 2_u8);
		assert_eq!(MyE::C_D, 3_u8);
		assert_eq!(MyE::A.discriminant(), 0_u8);
		assert_eq!(MyE::B("hello").discriminant(), 2_u8);
		assert_eq!(MyE::C { c: 42 }.discriminant(), 3_u8);
	}

	#[test]
	fn into() {
		#[derive(Debug, PartialEq, Eq, Into)]
		#[repr(u8)]
		enum E {
			A,
			B = 2,
			C,
		}

		assert_eq!(<E as Into<u8>>::into(E::A), 0_u8);
		assert_eq!(u8::from(E::B), 2_u8);
		assert_eq!(u8::from(E::C), 3_u8);
	}

	#[test]
	fn try_from() {
		#[derive(Debug, PartialEq, Eq, TryFrom)]
		#[repr(u8)]
		enum E {
			A,
			B = 2,
			C,
		}

		assert_eq!(E::try_from(0).unwrap(), E::A);
		assert_eq!(E::try_from(2).unwrap(), E::B);
		assert_eq!(E::try_from(3).unwrap(), E::C);
		assert!(E::try_from(1).is_err());
	}
}

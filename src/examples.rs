//! Displays some examples of the utilities provided by this crate
//!
//! > This is only for demo purposes, you cannot use this module in real code.
#![allow(missing_docs, clippy::missing_docs_in_private_items)]

use self::enum_discrim::*;
use crate as enum_discrim;

/// Enum deriving [`Discriminants`]
#[derive(Debug, Clone, Copy, Discriminants)]
#[repr(u8)]
pub enum MyGenericEnum<A, B>
where
	A: Copy,
{
	Unit,
	Tuple(A) = 2,
	Struct { b: B },
}

/// Enum deriving [`TryFrom`](self::TryFrom) and [`Into`](self::Into)
#[derive(Debug, Clone, Copy, TryFrom, Into)]
#[repr(u8)]
pub enum MyUnitEnum {
	A,
	B = 2,
	C,
}

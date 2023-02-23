# `enum_discrim`

Library to ease working with enum discriminants.

## `Discriminants` derive macro

This derive macro generates an impl block containing a const for each variant, equal to its discriminant.
A function returning the discriminant of an instance is also generated.

This macro can be applied on any enum, even with fields and generics.
You just need to provide a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations).

For example:
```rust
use enum_discrim::Discriminants;

#[derive(Discriminants)]
#[repr(u8)]
enum E<B, C>
where
	B: Copy,
{
	A,
	B(B) = 2,
	C { c: C },
}

type MyE = E::<&'static str, i32>;
assert_eq!(MyE::A_D, 0_u8);
assert_eq!(MyE::B_D, 2_u8);
assert_eq!(MyE::C_D, 3_u8);
assert_eq!(MyE::A.discriminant(), 0_u8);
assert_eq!(MyE::B("hello").discriminant(), 2_u8);
assert_eq!(MyE::C { c: 42 }.discriminant(), 3_u8);
```

## `Into` derive macro

This derive macro generates an impl [`Into<repr>`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) block.

This macro can be applied on enum with *only* unit variants.
You also *need* to provide a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations).

For example:
```rust
use enum_discrim::Into;

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
```

## `TryFrom` derive macro

This derive macro generates an impl [`TryFrom<repr>`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html) block.

This macro can be applied on enum with *only* unit variants.
You also *need* to provide a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations).

For example:
```rust
use enum_discrim::TryFrom;

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
```

//! Type-level slices of primitives.
//!
//! Rust permits certain constant parameters in generics:
//! ```rust
//! struct Foo<const CHAR: char>;
//! ```
//!
//! Presently these are limited to the primitive integers, [`prim@char`] and [`prim@bool`],
//! so e.g slices of different chars cannot be represented.
//! ```rust,compile_fail
//! struct Fails<const CHARS: [char]>;
//! type Message = Fails<['h', 'e', 'l', 'l', 'o']>;
//! ```
//!
//! This crate emulates the above with recursive [`types`](crate::types),
//! and the [`TypeSlice`](crate::TypeSlice) trait.
//! ```rust
//! type Message = typeslice::char!['h', 'e', 'l', 'l', 'o'];
//! ```
//!
//! You can inspect the message at `const` time or runtime through the [`List`]
//! in [`TypeSlice::LIST`](crate::TypeSlice::LIST):
//! ```rust
//! use typeslice::TypeSlice;
//!
//! fn get_reply<T: TypeSlice<char>>() -> &'static str {
//!     if T::LIST.slice_eq(&['h', 'e', 'l', 'l', 'o']) {
//!         return "hi!"
//!     }
//!     if T::LIST.into_iter().copied().eq("bonjour".chars()) {
//!         return "salut!"
//!     }
//!     "I didn't understand that"
//! }
//! ```
//!
//! If you enjoy this crate, you may also like [`typenum`](https://docs.rs/typenum) or [`frunk`](https://docs.rs/frunk)
#![allow(rustdoc::redundant_explicit_links)] // required for cargo-rdme
#![cfg_attr(not(feature = "std"), no_std)]

mod gen;

/// A type-level slice of items.
pub trait TypeSlice<T: 'static> {
    /// A list of the actual items.
    /// See [`List`] for more.
    const LIST: List<'static, T>;
    /// The number of items in this slice.
    const LEN: usize;
}

/// The bridge between a type-level [`TypeSlice`] and runtime logic,
/// allowing access to elements defined at the type level.
///
/// Supports iteration and indexing, with adapters for compile time use.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum List<'a, T> {
    Item { head: &'a T, rest: &'a Self },
    Empty,
}

impl<'a, T> Clone for List<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for List<'a, T> {}

impl<'a, T> Default for List<'a, T> {
    /// Create an empty slice.
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> List<'a, T> {
    /// Create an empty list.
    pub const fn new() -> Self {
        Self::Empty
    }
    /// Return the number of elements in the list.
    pub const fn len(&self) -> usize {
        match self {
            List::Item {
                head: _,
                rest: next,
            } => 1 + next.len(),
            List::Empty => 0,
        }
    }
    /// Get an item by index.
    pub const fn get(&self, ix: usize) -> Option<&T> {
        match (self, ix) {
            (Self::Empty, _) => None,
            (Self::Item { head, rest: _ }, 0) => Some(head),
            (
                Self::Item {
                    head: _,
                    rest: next,
                },
                _,
            ) => match ix.checked_sub(1) {
                Some(nix) => next.get(nix),
                None => None,
            },
        }
    }
    /// Returns true if the list has no elements.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
    /// Return [`None`] if this list is empty, else the next item and its successor.
    pub const fn into_option(self) -> Option<(&'a T, &'a Self)> {
        match self {
            List::Item { head, rest: next } => Some((head, next)),
            List::Empty => None,
        }
    }
    /// Iterate the elements in the list.
    /// Iterator type is `&T`.
    pub const fn iter(&self) -> Iter<'a, T> {
        Iter { inner: *self }
    }
}

impl<'a, T> IntoIterator for List<'a, T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the elements in a list.
/// See [`List::iter`].
pub struct Iter<'a, T> {
    inner: List<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.into_option() {
            Some((t, next)) => {
                self.inner = *next;
                Some(t)
            }
            None => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), Some(self.inner.len()))
    }
}

impl<'a, T> core::ops::Index<usize> for List<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(it) => it,
            None => {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                )
            }
        }
    }
}

/// Types that implement [`TypeSlice`] for all primitives that can be const-generics.
///
/// These types are all _uninhabited_, and cannot be constructed.
pub mod types {
    use crate::{List, TypeSlice};
    use core::marker::PhantomData;

    /// Marks a type as unconstructable.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Never {}

    /// > The only allowed types of const parameters are u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char and bool.
    /// - https://github.com/rust-lang/reference/blob/1afcfd9c66c8f8d582e01d109cfc15976171dfe0/src/items/generics.md#const-generics
    #[rustfmt::skip]
    macro_rules! for_all_const_types {
        ($do:ident) => {
            $do!(Usize/UsizeNil for usize); $do!(U8/U8Nil for u8); $do!(U16/U16Nil for u16); $do!(U32/U32Nil for u32); $do!(U64/U64Nil for u64); $do!(U128/U128Nil for u128);
            $do!(Isize/IsizeNil for isize); $do!(I8/I8Nil for i8); $do!(I16/I16Nil for i16); $do!(I32/I32Nil for i32); $do!(I64/I64Nil for i64); $do!(I128/I128Nil for i128);
            $do!(Char/CharNil for char);
            $do!(Bool/BoolNil for bool);
        };
    }

    macro_rules! impl_slice_eq {
        ($name:ident/$nil:ident for $ty:ty) => {
            impl List<'_, $ty> {
                /// `const` - enabled equality checking that can fail at compile time.
                pub const fn slice_eq(&self, slice: &[$ty]) -> bool {
                    if self.len() != slice.len() {
                        return false;
                    }

                    let mut ix = slice.len();
                    while let Some(nix) = ix.checked_sub(1) {
                        let Some(ours) = self.get(nix) else {
                            unreachable!()
                        };
                        if *ours != slice[nix] {
                            return false;
                        }
                        ix = nix
                    }

                    true
                }
            }
        };
    }

    for_all_const_types!(impl_slice_eq);

    macro_rules! define {
        ($name:ident/$nil:ident for $ty:ty) => {
            /// A [`
            #[doc = stringify!($ty)]
            /// `] element in a type-level [`Slice`].
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name<const ELEM: $ty, Rest> {
                _never: Never,
                _phantom: PhantomData<fn() -> Rest>,
            }

            /// A terminating element in a type level [`Slice`] of [`
            #[doc = stringify!($ty)]
            /// `].
            ///
            /// This is not common between [`Slice`] types to aid type inference in
            /// edge cases.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum $nil {}

            impl<const ELEM: $ty, Rest: TypeSlice<$ty>> TypeSlice<$ty> for $name<ELEM, Rest> {
                const LIST: List<'static, $ty> = List::Item {
                    head: &ELEM,
                    rest: &Rest::LIST,
                };
                const LEN: usize = 1 + Rest::LEN;
            }

            impl TypeSlice<$ty> for $nil {
                const LIST: List<'static, $ty> = List::Empty;
                const LEN: usize = 0;
            }
        };
    }
    for_all_const_types!(define);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use static_assertions::{const_assert, const_assert_eq};

    type Empty = U8Nil;
    type Hello = U8<b'h', U8<b'e', U8<b'l', U8<b'l', U8<b'o', U8Nil>>>>>;

    const_assert!(Empty::LIST.slice_eq(b""));
    const_assert_eq!(Empty::LEN, 0);
    const_assert!(Hello::LIST.slice_eq(b"hello"));
    const_assert_eq!(Hello::LEN, 5);

    type Empty2 = u8![];
    type Hello2 = u8![0x68, 0x65, 0x6c, 0x6c, 0x6f];

    const_assert!(Empty2::LIST.slice_eq(b""));
    const_assert_eq!(Empty2::LEN, 0);
    const_assert!(Hello2::LIST.slice_eq(b"hello"));
    const_assert_eq!(Hello2::LEN, 5);

    #[test]
    fn test() {
        itertools::assert_equal(Empty::LIST, b"");
        itertools::assert_equal(Hello::LIST, b"hello");
        itertools::assert_equal(Empty2::LIST, b"");
        itertools::assert_equal(Hello2::LIST, b"hello");
    }

    #[cfg(feature = "std")]
    #[test]
    fn gen() {
        const TEMPLATE: &str = r##"
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`~prim~`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type ~example_ty~ = typeslice::~prim~![~example_lit~];
/// assert!(~example_ty~::LIST.slice_eq(&[~example_lit~]))
/// ```
#[macro_export]
macro_rules! ~prim~ {
    () => {
        $crate::types::~nil~
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::~ty~<$first, $crate::~prim~!($($rest,)*)>
    }
}
"##;

        let mut expected = String::from("// this file is @generated in typestr's tests\n\n");

        let numeric_ty = "OneTwoThree";
        let numeric_lit = "1, 2, 3";

        for (ty, nil, prim, example_ty, example_lit) in [
            ("Usize", "UsizeNil", "usize", numeric_ty, numeric_lit),
            ("U8", "U8Nil", "u8", numeric_ty, numeric_lit),
            ("U16", "U16Nil", "u16", numeric_ty, numeric_lit),
            ("U32", "U32Nil", "u32", numeric_ty, numeric_lit),
            ("U64", "U64Nil", "u64", numeric_ty, numeric_lit),
            ("U128", "U128Nil", "u128", numeric_ty, numeric_lit),
            ("Isize", "IsizeNil", "isize", numeric_ty, numeric_lit),
            ("I8", "I8Nil", "i8", numeric_ty, numeric_lit),
            ("I16", "I16Nil", "i16", numeric_ty, numeric_lit),
            ("I32", "I32Nil", "i32", numeric_ty, numeric_lit),
            ("I64", "I64Nil", "i64", numeric_ty, numeric_lit),
            ("I128", "I128Nil", "i128", numeric_ty, numeric_lit),
            ("Char", "CharNil", "char", "Abc", "'a', 'b', 'c'"),
            ("Bool", "BoolNil", "bool", "TrueFalse", "true, false"),
        ] {
            expected.push_str(
                &TEMPLATE
                    .trim_start()
                    .replace("~ty~", ty)
                    .replace("~nil~", nil)
                    .replace("~prim~", prim)
                    .replace("~example_ty~", example_ty)
                    .replace("~example_lit~", example_lit),
            );
        }

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/gen.rs");
        match std::fs::read_to_string(path) {
            Ok(actual) if expected == actual => return, // ok
            _ => {}
        }

        let _ = std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/gen.rs.expected"),
            expected,
        );
        panic!("generated file does not match")
    }

    #[cfg(feature = "std")]
    #[test]
    fn readme() {
        assert!(
            std::process::Command::new("cargo")
                .args(["rdme", "--check"])
                .output()
                .expect("couldn't run `cargo rdme`")
                .status
                .success(),
            "README.md is out of date - bless the new version by running `cargo rdme`"
        )
    }
}

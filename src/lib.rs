#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

/// A list of references to items.
///
/// Used as a bridge between a type-level [`Slice`] and runtime logic.
///
/// Supports iteration and indexing, with adapters for compile time use.
#[derive(Debug)]
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
    pub const fn iter(&self) -> ConsListIter<'a, T> {
        ConsListIter { inner: *self }
    }
}

impl<'a, T> IntoIterator for List<'a, T> {
    type Item = &'a T;

    type IntoIter = ConsListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the elements in a list.
/// See [`List::iter`].
pub struct ConsListIter<'a, T> {
    inner: List<'a, T>,
}

impl<'a, T> Iterator for ConsListIter<'a, T> {
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

/// > The only allowed types of const parameters are u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char and bool.
/// - https://github.com/rust-lang/reference/blob/1afcfd9c66c8f8d582e01d109cfc15976171dfe0/src/items/generics.md#const-generics
#[rustfmt::skip]
macro_rules! for_all_const_types {
    ($do:ident) => {
        $do!(usize); $do!(u8); $do!(u16); $do!(u32); $do!(u64); $do!(u128);
        $do!(isize); $do!(i8); $do!(i16); $do!(i32); $do!(i64); $do!(i128);
        $do!(char);
        $do!(bool);
    };
}

macro_rules! impl_slice_eq {
    ($ty:ty) => {
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

/// A type-level slice of items.
pub trait Slice<T: 'static> {
    const LIST: List<'static, T>;
    const LEN: usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Never {}

/// A [`u8`] in a type-level [`Slice`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U8<const BYTE: u8, Tail> {
    _never: Never,
    _phantom: PhantomData<fn() -> Tail>,
}

impl<const BYTE: u8, Tail> Slice<u8> for U8<BYTE, Tail>
where
    Tail: Slice<u8>,
{
    const LIST: List<'static, u8> = List::Item {
        head: &BYTE,
        rest: &Tail::LIST,
    };
    const LEN: usize = 1 + Tail::LEN;
}

/// Terminating element for type-level [`Slice`]s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nil {}

impl Slice<u8> for Nil {
    const LIST: List<'static, u8> = List::Empty;
    const LEN: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{const_assert, const_assert_eq};

    type Empty = Nil;
    type Hello = U8<b'h', U8<b'e', U8<b'l', U8<b'l', U8<b'o', Nil>>>>>;

    const_assert!(Empty::LIST.slice_eq(b""));
    const_assert_eq!(Empty::LEN, 0);
    const_assert!(Hello::LIST.slice_eq(b"hello"));
    const_assert_eq!(Hello::LEN, 5);

    #[test]
    fn test() {
        itertools::assert_equal(Empty::LIST, b"");
        itertools::assert_equal(Hello::LIST, b"hello");
    }
}

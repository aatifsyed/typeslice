#![cfg_attr(not(feature = "std"), no_std)]

/// A type-level slice of items.
pub trait Slice<T: 'static> {
    /// A list of the actual items.
    /// See [`List`] for more.
    const LIST: List<'static, T>;
    /// The number of items in this slice.
    const LEN: usize;
}

/// The bridge between a type-level [`Slice`] and runtime logic,
/// allowing access to elements.
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

/// Types that implement [`Slice`] for all primitives that can be const-generics.
///
/// These types are all _uninhabited_, and cannot be constructed.
pub mod types {
    use crate::{List, Slice};
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

            impl<const ELEM: $ty, Rest: Slice<$ty>> Slice<$ty> for $name<ELEM, Rest> {
                const LIST: List<'static, $ty> = List::Item {
                    head: &ELEM,
                    rest: &Rest::LIST,
                };
                const LEN: usize = 1 + Rest::LEN;
            }

            impl Slice<$ty> for $nil {
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

    #[test]
    fn test() {
        itertools::assert_equal(Empty::LIST, b"");
        itertools::assert_equal(Hello::LIST, b"hello");
    }
}

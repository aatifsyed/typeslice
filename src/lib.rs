#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

/// A list of references to items.
///
/// Used as a bridge between e.g a [`Bytes`] and runtime logic.
///
/// Supports iteration and indexing, with adapters for compile time use.
#[derive(Debug)]
pub enum ConsList<'a, T> {
    Cons { head: &'a T, next: &'a Self },
    Empty,
}

impl<'a, T> Clone for ConsList<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for ConsList<'a, T> {}

impl<'a, T> Default for ConsList<'a, T> {
    /// Create an empty slice.
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> ConsList<'a, T> {
    /// Create an empty list.
    pub const fn new() -> Self {
        Self::Empty
    }
    /// Return the number of elements in the list.
    pub const fn len(&self) -> usize {
        match self {
            ConsList::Cons { head: _, next } => 1 + next.len(),
            ConsList::Empty => 0,
        }
    }
    /// Get an item by index.
    pub const fn get(&self, ix: usize) -> Option<&T> {
        match (self, ix) {
            (Self::Empty, _) => None,
            (Self::Cons { head, next: _ }, 0) => Some(head),
            (Self::Cons { head: _, next }, _) => match ix.checked_sub(1) {
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
            ConsList::Cons { head, next } => Some((head, next)),
            ConsList::Empty => None,
        }
    }
    /// Iterate the elements in the list.
    /// Iterator type is `&T`.
    pub const fn iter(&self) -> ConsListIter<'a, T> {
        ConsListIter { inner: *self }
    }
}

impl<'a, T> IntoIterator for ConsList<'a, T> {
    type Item = &'a T;

    type IntoIter = ConsListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the elements in a list.
/// See [`ConsList::iter`].
pub struct ConsListIter<'a, T> {
    inner: ConsList<'a, T>,
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

macro_rules! impl_slice_eq {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ConsList<'_, $ty> {
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
        )*
    };
}

impl_slice_eq! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    char,
    bool
}

pub trait Bytes {
    const LIST: ConsList<'static, u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Never {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteCons<const BYTE: u8, Tail> {
    _never: Never,
    _phantom: PhantomData<fn() -> Tail>,
}

impl<const BYTE: u8, Tail> Bytes for ByteCons<BYTE, Tail>
where
    Tail: Bytes,
{
    const LIST: ConsList<'static, u8> = ConsList::Cons {
        head: &BYTE,
        next: &Tail::LIST,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Empty {}

impl Bytes for Empty {
    const LIST: ConsList<'static, u8> = ConsList::Empty;
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::const_assert;

    const EMPTY: ConsList<'_, u8> = <Empty>::LIST;
    const HELLO: ConsList<'_, u8> = <ByteCons<
        b'h',
        ByteCons<b'e', ByteCons<b'l', ByteCons<b'l', ByteCons<b'o', Empty>>>>,
    >>::LIST;

    const_assert!(EMPTY.slice_eq(b""));
    const_assert!(HELLO.slice_eq(b"hello"));

    #[test]
    fn test() {
        itertools::assert_equal(EMPTY, b"");
        itertools::assert_equal(HELLO, b"hello");
    }
}

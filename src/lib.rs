#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

#[derive(Debug)]
pub enum ConsSlice<'a, T> {
    Cons { head: &'a T, next: &'a Self },
    Empty,
}

impl<'a, T> Clone for ConsSlice<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for ConsSlice<'a, T> {}

impl<'a, T> Default for ConsSlice<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> ConsSlice<'a, T> {
    pub const fn new() -> Self {
        Self::Empty
    }
    pub const fn len(&self) -> usize {
        match self {
            ConsSlice::Cons { head: _, next } => 1 + next.len(),
            ConsSlice::Empty => 0,
        }
    }
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
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
    pub const fn into_option(self) -> Option<(&'a T, &'a Self)> {
        match self {
            ConsSlice::Cons { head, next } => Some((head, next)),
            ConsSlice::Empty => None,
        }
    }
    pub const fn iter(&self) -> ConsSliceIter<'a, T> {
        ConsSliceIter { inner: *self }
    }
}

impl<'a, T> IntoIterator for ConsSlice<'a, T> {
    type Item = &'a T;

    type IntoIter = ConsSliceIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ConsSliceIter<'a, T> {
    inner: ConsSlice<'a, T>,
}

impl<'a, T> Iterator for ConsSliceIter<'a, T> {
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
            impl ConsSlice<'_, $ty> {
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

pub trait ByteList {
    const WALK: ConsSlice<'static, u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Never {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteCons<const BYTE: u8, Tail> {
    _never: Never,
    _phantom: PhantomData<fn() -> Tail>,
}

impl<const BYTE: u8, Tail> ByteList for ByteCons<BYTE, Tail>
where
    Tail: ByteList,
{
    const WALK: ConsSlice<'static, u8> = ConsSlice::Cons {
        head: &BYTE,
        next: &Tail::WALK,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Empty {}

impl ByteList for Empty {
    const WALK: ConsSlice<'static, u8> = ConsSlice::Empty;
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::const_assert;

    const EMPTY: ConsSlice<'_, u8> = <Empty>::WALK;
    const HELLO: ConsSlice<'_, u8> = <ByteCons<
        b'h',
        ByteCons<b'e', ByteCons<b'l', ByteCons<b'l', ByteCons<b'o', Empty>>>>,
    >>::WALK;

    const_assert!(EMPTY.slice_eq(b""));
    const_assert!(HELLO.slice_eq(b"hello"));

    #[test]
    fn test() {
        itertools::assert_equal(EMPTY, b"");
        itertools::assert_equal(HELLO, b"hello");
    }
}

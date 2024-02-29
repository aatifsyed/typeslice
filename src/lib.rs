use std::{iter, marker::PhantomData};

pub enum Byte<const BYTE: u8> {}

pub enum Empty {}

enum Never {}

pub struct Concat<L, R> {
    _never: Never,
    _phantom: PhantomData<fn() -> (L, R)>,
}

pub trait TypeVec {
    const LEN: usize;
    fn write_and(v: &mut [u8]) -> &mut [u8];
    fn to_vec() -> Vec<u8> {
        let mut v = vec![0; Self::LEN];
        Self::write_and(&mut v);
        v
    }
    fn bytes() -> impl Iterator<Item = u8>;
}

impl TypeVec for Empty {
    const LEN: usize = 0;

    fn write_and(it: &mut [u8]) -> &mut [u8] {
        it
    }

    fn bytes() -> impl Iterator<Item = u8> {
        iter::empty()
    }
}

impl<const BYTE: u8> TypeVec for Byte<BYTE> {
    const LEN: usize = 1;

    fn write_and(v: &mut [u8]) -> &mut [u8] {
        v[0] = BYTE;
        let (_, rest) = v.split_at_mut(Self::LEN);
        rest
    }
    fn bytes() -> impl Iterator<Item = u8> {
        iter::once(BYTE)
    }
}

impl<L, R> TypeVec for Concat<L, R>
where
    L: TypeVec,
    R: TypeVec,
{
    const LEN: usize = L::LEN + R::LEN;

    fn write_and(v: &mut [u8]) -> &mut [u8] {
        let rest = L::write_and(v);
        R::write_and(rest)
    }

    fn bytes() -> impl Iterator<Item = u8> {
        L::bytes().chain(R::bytes())
    }
}

pub const fn equal_len<T: TypeVec>(b: &[u8]) -> bool {
    T::LEN == b.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::const_assert;

    const_assert!(equal_len::<Empty>(b""));
    const_assert!(equal_len::<Concat<Empty, Empty>>(b""));
    const_assert!(equal_len::<Concat<Byte<b'A'>, Empty>>(b"B"));

    #[test]
    fn test() {
        assert_eq!(Empty::to_vec(), b"");
        assert_eq!(Concat::<Byte<b'A'>, Empty>::to_vec(), b"A");
        assert_eq!(
            Concat::<Concat<Byte<b'A'>, Byte<b'B'>>, Empty>::to_vec(),
            b"AB"
        );
        type Abc = Concat<Concat<Concat<Byte<b'A'>, Byte<b'B'>>, Byte<b'C'>>, Empty>;
        assert_eq!(Abc::to_vec(), b"ABC");
    }
}

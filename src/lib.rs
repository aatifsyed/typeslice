use std::marker::PhantomData;

pub enum Byte<const BYTE: u8> {}

pub enum Empty {}

enum Never {}

pub struct Concat<H, T> {
    _never: Never,
    _phantom: PhantomData<fn() -> (H, T)>,
}

pub trait TypeVec {
    const LEN: usize;
    fn put_byte<'a>(v: &'a mut &'a mut [u8]);
    fn to_vec() -> Vec<u8> {
        let mut v = vec![0; Self::LEN];
        Self::put_byte(&mut &mut *v);
        v
    }
}

impl<const BYTE: u8> TypeVec for Concat<Byte<BYTE>, Empty> {
    const LEN: usize = 1;
    fn put_byte<'a>(v: &'a mut &'a mut [u8]) {
        v[0] = BYTE;
        let (_, rest) = v.split_at_mut(Self::LEN);
        *v = rest;
    }
}

impl<const LEFT: u8, const RIGHT: u8> TypeVec for Concat<Byte<LEFT>, Byte<RIGHT>> {
    const LEN: usize = 2;
    fn put_byte<'a>(v: &'a mut &'a mut [u8]) {
        v[0] = LEFT;
        v[1] = RIGHT;
        let (_, rest) = v.split_at_mut(Self::LEN);
        *v = rest;
    }
}

impl<T> TypeVec for Concat<T, Empty>
where
    T: TypeVec,
{
    const LEN: usize = T::LEN;
    fn put_byte<'a>(v: &'a mut &'a mut [u8]) {
        T::put_byte(v)
    }
}

impl TypeVec for Empty {
    const LEN: usize = 0;
    fn put_byte(_: &mut &mut [u8]) {}
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
        type ABC = Concat<Concat<Concat<Byte<b'A'>, Byte<b'B'>>, Byte<b'C'>>, Empty>;
        assert_eq!(ABC::to_vec(), b"AB");
    }
}

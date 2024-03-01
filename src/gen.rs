// this file is @generated in typestr's tests

/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`usize`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::usize![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! usize {
    () => {
        $crate::types::UsizeNil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::Usize<$first, $crate::usize!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`u8`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::u8![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! u8 {
    () => {
        $crate::types::U8Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::U8<$first, $crate::u8!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`u16`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::u16![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! u16 {
    () => {
        $crate::types::U16Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::U16<$first, $crate::u16!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`u32`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::u32![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! u32 {
    () => {
        $crate::types::U32Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::U32<$first, $crate::u32!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`u64`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::u64![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! u64 {
    () => {
        $crate::types::U64Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::U64<$first, $crate::u64!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`u128`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::u128![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! u128 {
    () => {
        $crate::types::U128Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::U128<$first, $crate::u128!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`isize`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::isize![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! isize {
    () => {
        $crate::types::IsizeNil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::Isize<$first, $crate::isize!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`i8`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::i8![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! i8 {
    () => {
        $crate::types::I8Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::I8<$first, $crate::i8!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`i16`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::i16![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! i16 {
    () => {
        $crate::types::I16Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::I16<$first, $crate::i16!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`i32`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::i32![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! i32 {
    () => {
        $crate::types::I32Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::I32<$first, $crate::i32!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`i64`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::i64![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! i64 {
    () => {
        $crate::types::I64Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::I64<$first, $crate::i64!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`i128`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type OneTwoThree = typeslice::i128![1, 2, 3];
/// assert!(OneTwoThree::LIST.slice_eq(&[1, 2, 3]))
/// ```
#[macro_export]
macro_rules! i128 {
    () => {
        $crate::types::I128Nil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::I128<$first, $crate::i128!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`char`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type Abc = typeslice::char!['a', 'b', 'c'];
/// assert!(Abc::LIST.slice_eq(&['a', 'b', 'c']))
/// ```
#[macro_export]
macro_rules! char {
    () => {
        $crate::types::CharNil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::Char<$first, $crate::char!($($rest,)*)>
    }
}
/// Define a type-level [`TypeSlice`](crate::TypeSlice) of [`bool`]s.
/// ```
/// # use typeslice::TypeSlice as _;
/// type TrueFalse = typeslice::bool![true, false];
/// assert!(TrueFalse::LIST.slice_eq(&[true, false]))
/// ```
#[macro_export]
macro_rules! bool {
    () => {
        $crate::types::BoolNil
    };
    ($first:literal $(,$rest:tt)* $(,)?) => {
        $crate::types::Bool<$first, $crate::bool!($($rest,)*)>
    }
}

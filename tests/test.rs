#![allow(dead_code)]

use static_assertions::assert_type_eq_all;

type Empty = typeslice::from_str!();
type Empty2 = typeslice::from_str!("");
type Empty3 = typeslice::char![];
assert_type_eq_all!(Empty, Empty2, Empty3);

type Hello = typeslice::from_str!("hello");
type Hello2 = typeslice::char!['h', 'e', 'l', 'l', 'o'];
assert_type_eq_all!(Hello, Hello2);

type BEmpty = typeslice::from_bytes!();
type BEmpty2 = typeslice::from_bytes!(b"");
type BEmpty3 = typeslice::u8![];
assert_type_eq_all!(BEmpty, BEmpty2, BEmpty3);

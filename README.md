<!-- cargo-rdme start -->

Type-level slices of primitives.

Rust permits certain constant parameters in generics:
```rust
struct Foo<const CHAR: char>;
```

Presently these are limited to the primitive integers, [`prim@char`] and [`prim@bool`],
so e.g slices of different chars cannot be represented.
```rust
struct Fails<const CHARS: [char]>;
type Message = Fails<['h', 'e', 'l', 'l', 'o']>;
```

This crate emulates the above with recursive [`types`](https://docs.rs/typeslice/latest/typeslice/types/),
and the [`TypeSlice`](https://docs.rs/typeslice/latest/typeslice/trait.TypeSlice.html) trait.
```rust
type Message = typeslice::char!['h', 'e', 'l', 'l', 'o'];
// or, equivalently
type Message = typeslice::from_str!("hello");
```

You can inspect the message at `const` time or runtime through the [`List`](https://docs.rs/typeslice/latest/typeslice/enum.List.html)
in `TypeSlice::LIST`:
```rust
use typeslice::TypeSlice;

fn get_reply<T: TypeSlice<char>>() -> &'static str {
    if T::LIST.slice_eq(&['h', 'i']) {
        return "hello"
    }
    if T::LIST.str_eq("👋👋") {
        return "😎😎"
    }
    if T::LIST.into_iter().copied().eq("salut".chars()) {
        return "bonjour"
    }
    "¿que?"
}

assert_eq!(get_reply::<typeslice::from_str!("hi")>(), "hello");
```

If you enjoy this crate, you may also like [`typenum`](https://docs.rs/typenum) or [`frunk`](https://docs.rs/frunk)

<!-- cargo-rdme end -->

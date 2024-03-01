//! This example demonstrates functions describing their arguments, so that a
//! trait, [`ArgNames`], can access them.

use std::marker::PhantomData;

use typeslice::TypeSlice;

struct Named<T, N>(pub T, PhantomData<fn() -> N>);

#[allow(clippy::type_complexity)]
fn compare(
    _: Named<(), typeslice::from_str!("left")>,
    _: Named<(), typeslice::from_str!("right")>,
) {
}
#[allow(clippy::type_complexity)]
fn transfer(
    _: Named<(), typeslice::from_str!("source")>,
    _: Named<(), typeslice::from_str!("destination")>,
) {
}

trait ArgNames<Args> {
    fn arg_names(&self) -> Vec<String>;
}

impl<F, T0, N0, T1, N1> ArgNames<(T0, N0, T1, N1)> for F
where
    F: FnOnce(Named<T0, N0>, Named<T1, N1>),
    N0: TypeSlice<char>,
    N1: TypeSlice<char>,
{
    fn arg_names(&self) -> Vec<String> {
        vec![String::from_iter(N0::LIST), String::from_iter(N1::LIST)]
    }
}

fn main() {
    assert_eq!(dbg!(compare.arg_names()), ["left", "right"]);
    assert_eq!(dbg!(transfer.arg_names()), ["source", "destination"]);
}

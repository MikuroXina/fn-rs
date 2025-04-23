use crate::{func::Func, hkt::Hkt1};

pub trait Invariant: Hkt1 {
    fn invariant<'a, T, U>(
        covariant: Func<'a, T, U>,
        contravariant: Func<'a, U, T>,
    ) -> Func<'a, Self::F<T>, Self::F<U>>;
}

pub trait Covariant: Hkt1 {
    fn covariant<T, U>(mapper: Func<T, U>) -> Func<Self::F<T>, Self::F<U>>;
}

pub trait Contravariant: Hkt1 {
    fn contravariant<T, U>(mapper: Func<T, U>) -> Func<Self::F<U>, Self::F<T>>;
}

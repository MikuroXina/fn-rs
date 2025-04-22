use crate::{func::Func, hkt::Hkt1};

pub trait Functor: Hkt1 {
    fn map<T, U>(mapper: Func<T, U>) -> Func<Self::F<T>, Self::F<U>>;
}

pub trait Pure: Hkt1 {
    fn pure<T>(value: T) -> Self::F<T>;
}

pub trait Apply: Hkt1 {
    fn apply<T, U>(mapper: Self::F<Func<T, U>>) -> Func<Self::F<T>, Self::F<U>>;
}

pub trait Applicative: Functor + Pure + Apply {}

pub trait Monad: Applicative {
    fn flat_map<T, U>(mapper: Func<T, Self::F<U>>) -> Func<Self::F<T>, Self::F<U>>;

    fn lift<A, B>(f: Func<A, B>) -> Func<Self::F<A>, Self::F<B>> {
        Self::map(f)
    }
}

pub trait AsRef1: Hkt1 {
    fn as_ref<T>(value: &Self::F<T>) -> Self::F<&T>;
}

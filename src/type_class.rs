use crate::hkt::Hkt1;

pub trait Functor: Hkt1 {
    fn map<T, U, Mapper: Fn(T) -> U>(mapper: Mapper) -> impl Fn(Self::F<T>) -> Self::F<U>;
}

pub trait Pure: Hkt1 {
    fn pure<T>(value: T) -> Self::F<T>;
}

pub trait Apply: Hkt1 {
    fn apply<T, U, Mapper: Fn(T) -> U>(
        mapper: Self::F<Mapper>,
    ) -> impl Fn(Self::F<T>) -> Self::F<U>;
}

pub trait Applicative: Functor + Pure + Apply {}

pub trait Monad: Applicative {
    fn flat_map<T, U, Mapper: Fn(T) -> Self::F<U>>(
        mapper: Mapper,
    ) -> impl Fn(Self::F<T>) -> Self::F<U>;
}

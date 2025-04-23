use crate::{
    compose::ComposeFunctor,
    func::{Func, func},
    hkt::Hkt1,
};

pub trait Functor: Hkt1 {
    fn map<'a, T: 'a, U: 'a>(mapper: Func<'a, T, U>) -> Func<'a, Self::F<T>, Self::F<U>>;

    fn imap<'a, T, U>(this: Self::F<T>) -> Func<'a, Func<'a, T, U>, Self::F<U>>
    where
        Self::F<T>: Clone + 'a,
    {
        func(move |f| (Self::map::<T, U>(f))(this.clone()))
    }

    fn substitute<T, U>(this: Self::F<T>, value: U) -> Self::F<U>
    where
        U: Clone,
    {
        Self::map(func(move |_| value.clone()))(this)
    }

    fn substitute_with<T, U>(this: Self::F<T>, value: Func<(), U>) -> Self::F<U> {
        Self::map(func(move |_| value(())))(this)
    }

    fn into<T, U: From<T>>(this: Self::F<T>) -> Self::F<U> {
        Self::map(func(U::from))(this)
    }

    fn void<T>(this: Self::F<T>) -> Self::F<()> {
        Self::map(func(|_| ()))(this)
    }

    fn before_and_after<T, U>(this: Self::F<T>, mapper: Func<T, U>) -> Self::F<(T, U)>
    where
        T: Clone,
    {
        Self::map(func(|t: T| (t.clone(), mapper(t))))(this)
    }

    fn unzip<T, U>(this: Self::F<(T, U)>) -> (Self::F<T>, Self::F<U>)
    where
        Self::F<(T, U)>: Clone,
    {
        (
            Self::map::<(T, U), T>(func(|(l, _)| l))(this.clone()),
            Self::map::<(T, U), U>(func(|(_, r)| r))(this),
        )
    }

    fn compose<G>() -> ComposeFunctor<Self, G> {
        ComposeFunctor::new()
    }
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

    fn lift<'a, A: 'a, B: 'a>(f: Func<'a, A, B>) -> Func<'a, Self::F<A>, Self::F<B>> {
        Self::map(f)
    }
}

pub trait AsRef1: Hkt1 {
    fn as_ref<T>(value: &Self::F<T>) -> Self::F<&T>;
}

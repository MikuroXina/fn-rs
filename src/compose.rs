use std::marker::PhantomData;

use crate::{
    func::{Func, func},
    hkt::Hkt1,
    type_class::Functor,
};

pub struct ComposeFunctor<A: ?Sized, B: ?Sized>(PhantomData<A>, PhantomData<B>);

impl<A: ?Sized, B: ?Sized> ComposeFunctor<A, B> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<A: Hkt1, B: Hkt1> Hkt1 for ComposeFunctor<A, B> {
    type F<T1> = B::F<A::F<T1>>;
}

impl<A: Functor, B: Functor> Functor for ComposeFunctor<A, B> {
    fn map<'a, T: 'a, U: 'a>(mapper: Func<'a, T, U>) -> Func<'a, Self::F<T>, Self::F<U>> {
        func(move |bat: B::F<A::F<T>>| {
            B::map(func(|at: A::F<T>| A::map(func(|t: T| mapper(t)))(at)))(bat)
        })
    }
}

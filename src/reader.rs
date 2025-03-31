use crate::{hkt::Hkt1, identity::IdentityHkt, type_class::Monad};

pub struct ReaderT<'a, R, M: Hkt1, A>(Box<dyn Fn(R) -> M::F<A> + 'a>);

impl<'a, R, M: Hkt1, A> ReaderT<'a, R, M, A> {
    pub fn new<F: Fn(R) -> M::F<A> + 'a>(func: F) -> Self {
        Self(Box::new(func))
    }

    pub fn map<N: Hkt1, B, F: Fn(M::F<A>) -> N::F<B>>(self, mapper: F) -> ReaderT<'a, R, N, B>
    where
        R: 'a,
        <M as Hkt1>::F<A>: 'a,
        F: 'a,
    {
        ReaderT(Box::new(move |record| mapper((self.0)(record))))
    }

    pub fn with<R1, F: Fn(R1) -> R>(self, mapper: F) -> ReaderT<'a, R1, M, A>
    where
        R: 'a,
        <M as Hkt1>::F<A>: 'a,
        F: 'a,
    {
        ReaderT(Box::new(move |record| (self.0)(mapper(record))))
    }
}

pub type Reader<'a, R, A> = ReaderT<'a, R, IdentityHkt, A>;

impl<'a, R> Reader<'a, R, R> {
    pub fn ask_m<M: Monad>() -> M::F<Self> {
        M::pure(Self(Box::new(|r| r)))
    }

    pub fn ask() -> Self {
        Self(Box::new(|r| r))
    }
}

impl<'a, R, A> Reader<'a, R, A> {
    pub fn with_reader<F: Fn(R) -> A + 'a>(reader: F) -> Self {
        ReaderT(Box::new(move |r| reader((Reader::ask().0)(r))))
    }

    pub fn run(self, record: R) -> A {
        (self.0)(record)
    }

    pub fn local<Q, F: Fn(Q) -> R>(self, mapper: F) -> Reader<'a, Q, A>
    where
        R: 'a,
        A: 'a,
        F: 'a,
    {
        ReaderT(Box::new(move |q| (self.0)(mapper(q))))
    }

    pub fn product<B>(self, other: Reader<'a, R, B>) -> Reader<'a, R, (A, B)>
    where
        A: 'a,
        B: 'a,
        R: Clone + 'a,
    {
        ReaderT(Box::new(move |r| ((self.0)(r.clone()), (other.0)(r))))
    }
}

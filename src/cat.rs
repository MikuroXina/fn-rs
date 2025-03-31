use std::ops::ControlFlow;

use crate::{hkt::Hkt1, type_class::Monad};

#[derive(Debug, Default)]
pub struct Cat<T>(pub T);

impl<T> From<T> for Cat<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Cat<T> {
    pub const fn new(item: T) -> Self {
        Self(item)
    }

    pub fn feed<U, F: FnOnce(T) -> U>(self, mapper: F) -> Cat<U> {
        Cat(mapper(self.0))
    }

    pub fn value(self) -> T {
        self.0
    }
}

pub struct CatT<M: Hkt1, CTX>(M::F<CTX>);

impl<M, CTX> std::fmt::Debug for CatT<M, CTX>
where
    M: Monad,
    M::F<CTX>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<M: Monad, CTX> From<CTX> for CatT<M, CTX> {
    fn from(value: CTX) -> Self {
        Self::new(value)
    }
}

impl<M: Monad, CTX> CatT<M, CTX> {
    pub fn new(ctx: CTX) -> Self {
        Self(M::pure(ctx))
    }

    pub fn add_m<A, B, F: Fn(A, CTX) -> B>(self, value: M::F<A>, mapper: F) -> CatT<M, B>
    where
        M::F<A>: Clone,
        CTX: Clone,
    {
        CatT(M::flat_map(|ctx: CTX| -> M::F<B> {
            M::map(|v: A| -> B { mapper(v, ctx.clone()) })(value.clone())
        })(self.0))
    }

    pub fn add_with<A, B, V: Fn(CTX) -> A, F: Fn(A, CTX) -> B>(
        self,
        value: V,
        mapper: F,
    ) -> CatT<M, B>
    where
        CTX: Clone,
    {
        CatT(M::map(|ctx: CTX| mapper(value(ctx.clone()), ctx.clone()))(
            self.0,
        ))
    }

    pub fn add_m_with<A, B, V: Fn(CTX) -> M::F<A>, F: Fn(A, CTX) -> B>(
        self,
        value: V,
        mapper: F,
    ) -> CatT<M, B>
    where
        CTX: Clone,
    {
        CatT(M::flat_map(|ctx: CTX| -> M::F<B> {
            M::map(|v: A| -> B { mapper(v, ctx.clone()) })(value(ctx.clone()))
        })(self.0))
    }

    pub fn run(self, computation: M::F<()>) -> Self
    where
        M::F<()>: Clone,
        CTX: Clone,
    {
        Self(M::flat_map(|ctx: CTX| -> M::F<CTX> {
            M::map(|_| ctx.clone())(computation.clone())
        })(self.0))
    }

    pub fn run_with<C: Fn(CTX) -> M::F<()>>(self, computation: C) -> Self
    where
        CTX: Clone,
    {
        Self(M::flat_map(|ctx: CTX| -> M::F<CTX> {
            M::map(|_| ctx.clone())(computation(ctx.clone()))
        })(self.0))
    }

    pub fn when<B: Fn(CTX) -> bool, C: Fn(CTX) -> M::F<()>>(self, cond: B, computation: C) -> Self
    where
        CTX: Clone,
    {
        Self(M::flat_map(|ctx: CTX| -> M::F<CTX> {
            M::map(|_| ctx.clone())(if cond(ctx.clone()) {
                computation(ctx.clone())
            } else {
                M::pure(())
            })
        })(self.0))
    }

    pub fn unfold<S, B: Fn(S, CTX) -> M::F<ControlFlow<(), S>>>(
        self,
        init_state: S,
        body: B,
    ) -> Self
    where
        S: Clone,
        CTX: Clone,
        M::F<CTX>: Clone,
    {
        fn inner<M: Monad, CTX, S, B: Fn(S, CTX) -> M::F<ControlFlow<(), S>>>(
            f_ctx: M::F<CTX>,
            body: &B,
            state: S,
        ) -> M::F<CTX>
        where
            S: Clone,
            CTX: Clone,
            M::F<CTX>: Clone,
        {
            M::flat_map(|ctx: CTX| -> M::F<CTX> {
                M::flat_map(|flow: ControlFlow<(), S>| -> M::F<CTX> {
                    if let ControlFlow::Continue(state) = flow {
                        inner::<M, CTX, S, B>(f_ctx.clone(), body, state)
                    } else {
                        M::pure(ctx.clone())
                    }
                })(body(state.clone(), ctx.clone()))
            })(f_ctx.clone())
        }
        CatT(inner::<M, CTX, S, B>(self.0, &body, init_state))
    }

    pub fn iterate<C: Fn(CTX) -> M::F<bool>, B: Fn(CTX) -> M::F<()>>(self, cond: C, body: B) -> Self
    where
        CTX: Clone,
        M::F<CTX>: Clone,
    {
        fn inner<M: Monad, CTX, C: Fn(CTX) -> M::F<bool>, B: Fn(CTX) -> M::F<()>>(
            f_ctx: M::F<CTX>,
            cond: &C,
            body: &B,
        ) -> M::F<CTX>
        where
            CTX: Clone,
            M::F<CTX>: Clone,
        {
            M::flat_map(|c: CTX| -> M::F<CTX> {
                M::flat_map(|b: bool| {
                    if b {
                        M::flat_map(|_: ()| inner::<M, CTX, C, B>(f_ctx.clone(), cond, body))(body(
                            c.clone(),
                        ))
                    } else {
                        M::pure(c.clone())
                    }
                })(cond(c.clone()))
            })(f_ctx.clone())
        }
        CatT(inner::<M, CTX, C, B>(self.0, &cond, &body))
    }

    pub fn finish<R, F: Fn(CTX) -> R>(self, mapper: F) -> M::F<R> {
        M::map(mapper)(self.0)
    }

    pub fn finish_m<R, F: Fn(CTX) -> M::F<R>>(self, mapper: F) -> M::F<R> {
        M::flat_map(mapper)(self.0)
    }
}

impl<M: Monad> CatT<M, ()> {
    pub fn unit() -> Self {
        CatT(M::pure(()))
    }
}

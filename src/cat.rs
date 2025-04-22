use std::ops::ControlFlow;

use crate::{
    func::{Func, func},
    hkt::Hkt1,
    type_class::Monad,
};

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

    pub fn as_ref(&self) -> Cat<&T> {
        Cat(&self.0)
    }

    pub fn as_mut(&mut self) -> Cat<&mut T> {
        Cat(&mut self.0)
    }
}

pub struct CatHkt;

impl Hkt1 for CatHkt {
    type F<T1> = Cat<T1>;
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

    pub fn add_m<A: 'static, B: 'static>(
        self,
        value: M::F<A>,
        mapper: Func<(A, CTX), B>,
    ) -> CatT<M, B>
    where
        M::F<A>: Clone,
        CTX: Clone,
    {
        CatT(M::flat_map(func(move |ctx: CTX| -> M::F<B> {
            let mapper = mapper.clone();
            M::map(func(move |v: A| -> B { (mapper)((v, ctx.clone())) }))(value.clone())
        }))(self.0))
    }

    pub fn add_with<A: 'static, B: 'static>(
        self,
        value: Func<CTX, A>,
        mapper: Func<(A, CTX), B>,
    ) -> CatT<M, B>
    where
        CTX: Clone,
    {
        CatT(M::map(func(move |ctx: CTX| {
            (mapper)((value(ctx.clone()), ctx))
        }))(self.0))
    }

    pub fn add_m_with<A: 'static, B: 'static>(
        self,
        value: Func<CTX, M::F<A>>,
        mapper: Func<(A, CTX), B>,
    ) -> CatT<M, B>
    where
        CTX: Clone,
    {
        CatT(M::flat_map(func(|ctx: CTX| -> M::F<B> {
            M::map(func(|v: A| -> B { (mapper)((v, ctx.clone())) }))(value(ctx.clone()))
        }))(self.0))
    }

    pub fn run(self, computation: M::F<()>) -> Self
    where
        M::F<()>: Clone,
        CTX: Clone,
    {
        Self(M::flat_map(func(|ctx: CTX| -> M::F<CTX> {
            M::map(func(|_| ctx.clone()))(computation.clone())
        }))(self.0))
    }

    pub fn run_with<C>(self, computation: Func<CTX, M::F<()>>) -> Self
    where
        M::F<()>: 'static,
        CTX: Clone + 'static,
    {
        Self(M::flat_map(func(|ctx: CTX| -> M::F<CTX> {
            M::map(func(|_| ctx.clone()))(computation(ctx.clone()))
        }))(self.0))
    }

    pub fn when(self, cond: Func<CTX, bool>, computation: Func<CTX, M::F<()>>) -> Self
    where
        M::F<()>: 'static,
        CTX: Clone + 'static,
    {
        Self(M::flat_map(func(|ctx: CTX| -> M::F<CTX> {
            M::map(func(|_| ctx.clone()))(if cond(ctx.clone()) {
                computation(ctx.clone())
            } else {
                M::pure(())
            })
        }))(self.0))
    }

    pub fn unfold<S>(self, init_state: S, body: Func<(S, CTX), M::F<ControlFlow<(), S>>>) -> Self
    where
        S: Clone,
        CTX: Clone,
        M::F<CTX>: Clone,
    {
        fn inner<M: Monad, CTX, S>(
            f_ctx: M::F<CTX>,
            body: Func<(S, CTX), M::F<ControlFlow<(), S>>>,
            state: S,
        ) -> M::F<CTX>
        where
            S: Clone,
            CTX: Clone,
            M::F<CTX>: Clone,
        {
            let cloned_f_ctx = f_ctx.clone();
            M::flat_map(func(move |ctx: CTX| -> M::F<CTX> {
                let cloned_ctx = ctx.clone();
                let cloned_f_ctx = cloned_f_ctx.clone();
                let cloned_body = body.clone();
                M::flat_map(func(move |flow: ControlFlow<(), S>| -> M::F<CTX> {
                    if let ControlFlow::Continue(state) = flow {
                        inner::<M, CTX, S>(cloned_f_ctx.clone(), cloned_body.clone(), state)
                    } else {
                        M::pure(cloned_ctx.clone())
                    }
                }))(body((state.clone(), ctx)))
            }))(f_ctx)
        }
        CatT(inner::<M, CTX, S>(self.0, body, init_state))
    }

    pub fn iterate(self, cond: Func<CTX, M::F<bool>>, body: Func<CTX, M::F<()>>) -> Self
    where
        CTX: Clone,
        M::F<CTX>: Clone,
    {
        fn inner<M: Monad, CTX>(
            f_ctx: M::F<CTX>,
            cond: Func<CTX, M::F<bool>>,
            body: Func<CTX, M::F<()>>,
        ) -> M::F<CTX>
        where
            CTX: Clone,
            M::F<CTX>: Clone,
        {
            let cloned_f_ctx = f_ctx.clone();
            M::flat_map(func(move |ctx: CTX| -> M::F<CTX> {
                let cloned_ctx = ctx.clone();
                let cloned_f_ctx = cloned_f_ctx.clone();
                let cloned_cond = cond.clone();
                let cloned_body = body.clone();
                M::flat_map(func(move |b: bool| {
                    if b {
                        let cloned_f_ctx = cloned_f_ctx.clone();
                        let cloned_cond = cloned_cond.clone();
                        let cloned_cloned_body = cloned_body.clone();
                        M::flat_map(func({
                            move |_: ()| {
                                inner::<M, CTX>(
                                    cloned_f_ctx.clone(),
                                    cloned_cond.clone(),
                                    cloned_cloned_body.clone(),
                                )
                            }
                        }))(cloned_body(cloned_ctx.clone()))
                    } else {
                        M::pure(cloned_ctx.clone())
                    }
                }))(cond(ctx.clone()))
            }))(f_ctx.clone())
        }
        CatT(inner::<M, CTX>(self.0, cond, body))
    }

    pub fn finish<R>(self, mapper: Func<CTX, R>) -> M::F<R> {
        M::map(mapper)(self.0)
    }

    pub fn finish_m<R>(self, mapper: Func<CTX, M::F<R>>) -> M::F<R> {
        M::flat_map(mapper)(self.0)
    }
}

impl<M: Monad> CatT<M, ()> {
    pub fn unit() -> Self {
        CatT(M::pure(()))
    }
}

use std::marker::PhantomData;

pub trait Hkt1 {
    type F<T1>;
}

pub trait Hkt2 {
    type F<T1, T2>;
}

pub struct Apply2Only<M, T2>(pub PhantomData<(M, T2)>);

impl<M: Hkt2, T2> Hkt1 for Apply2Only<M, T2> {
    type F<T1> = M::F<T1, T2>;
}

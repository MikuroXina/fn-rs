use crate::hkt::Hkt1;

pub type Identity<T> = T;

pub enum IdentityHkt {}

impl Hkt1 for IdentityHkt {
    type F<T1> = Identity<T1>;
}

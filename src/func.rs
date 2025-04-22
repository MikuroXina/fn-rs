use std::sync::Arc;

pub type Func<'a, A, B> = Arc<dyn Fn(A) -> B + 'a>;

pub fn func<'a, F: Fn(A) -> B + 'a, A, B>(f: F) -> Func<'a, A, B> {
    Arc::new(f)
}

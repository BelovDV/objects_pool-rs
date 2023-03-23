mod id;

pub use id::Id;

mod simple;
mod unique;
mod variadic;

pub use simple::Simple;
pub use unique::Unique;
pub use variadic::{Variant, Variadic};

// To be done: unique (type) id for pools.

pub trait Pool {
    type Type;

    fn get(&self, id: Id<Self::Type>) -> &Self::Type;

    #[must_use = "`Id` is the only way to access stored `value`"]
    fn insert(&mut self, value: Self::Type) -> Id<Self::Type>;
}

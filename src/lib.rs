#![cfg_attr(feature = "fn_overload", feature(specialization))]

mod id;

pub use id::{id_cast, Id};

mod simple;
mod unique;
mod variadic;

pub use simple::Simple;
pub use unique::Unique;
pub use variadic::{Variadic, Variant};

// To be done: unique (type) id for pools.

pub trait Pool<Type: Storable<Self>>: Sized {
    #[must_use = "`Id` is the only way to access stored `value`"]
    fn insert<T: Storable<Self>>(&mut self, value: T) -> Id<T> {
        T::store(value, self)
    }

    fn get<T: Storable<Self>>(&self, id: Id<T>) -> &T {
        T::access(self, id)
    }
}

// Created for `variadic`.
pub trait Storable<Pool>: Sized {
    fn store(self, pool: &mut Pool) -> Id<Self>;
    fn access(pool: &Pool, id: Id<Self>) -> &Self;
}

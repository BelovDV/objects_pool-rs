use std::marker::PhantomData;

use super::id::Id;
use super::Pool;

pub trait Variant<Container> {
    fn pack(self) -> Container;
    fn unpack(from: &Container) -> &Self;
}

/// A set of objects.
/// Keeps all objects while exists.
/// `Id`s expected to be used like references.
///
/// # Example
///
/// ```
/// use objects_pool::{Pool as _, Simple, Variadic, variadic};
///
/// variadic!(C: String, i32);
///
/// let mut pool: Variadic<C, Simple<C>> = Default::default();
///
/// let id_abc = pool.insert_s("abc".to_string());
/// let id_123 = pool.insert_s(123);
///
/// let id_abc_any = pool.insert(C::String("abc".to_string()));
/// let id_123_any = pool.insert(C::i32(123));
///
/// let id_abc_copy = id_abc;
/// assert!(id_abc == id_abc_copy);
/// assert!(id_123_any != id_abc_any);
/// // id_abc != id_abc_any // These are different types.
///
/// assert!(*pool.get_s(id_123) == 123);
/// assert!(matches!(pool.get(id_abc_any), C::String(_)));
/// ```
///
/// # Caveats
///
/// `Id` can only be used with set which is gotten from.
///
/// Uses `usize::add(1)` as `Id` generator.
pub struct Variadic<Container, InnerPool: Pool<Type = Container>> {
    pool: InnerPool,
}

// To be done: may be proc? No, there isn't clear reason for it.
// To be done: impl `From<Id<ty>>` for `Id<name>`.
#[macro_export]
macro_rules! variadic {
    ($name:ident: $($ty:ident),*) => {
        ::objects_pool::variadic!(!enum_simple: $name: $($ty),*);
        $(::objects_pool::variadic!(!variant_impl: $name: $ty);)*
    };
    ($name:ident: $($ty:ident),*; derive($($derive:ident),*)) => {
        ::objects_pool::variadic!(!enum_derive: $name: $($ty),*; $($derive),*);
        $(::objects_pool::variadic!(!variant_impl: $name: $ty);)*
    };
    (!enum_simple: $name:ident: $($ty:ident),*) => {
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($ty($ty)),*
        }
    };
    (!enum_derive: $name:ident: $($ty:ident),*; $($derive:ident),*) => {
        #[allow(non_camel_case_types)]
        #[derive($($derive),*)]
        pub enum $name {
            $($ty($ty)),*
        }
    };
    (!variant_impl: $name:ident: $ty:ident) => {
        impl ::objects_pool::Variant<$name> for $ty {
            fn pack(self) -> $name {
                $name::$ty(self)
            }

            fn unpack(from: &$name) -> &Self {
                match from {
                    $name::$ty(s) => s,
                    _ => unreachable!(),
                }
            }
        }
    };
}

// To be done: is there a way to not use different names?
impl<Container, InnerPool: Pool<Type = Container>> Variadic<Container, InnerPool> {
    pub fn get_s<Type: Variant<Container>>(&self, id: Id<Type>) -> &Type {
        let id = Id {
            id: id.id,
            _type: PhantomData,
        };
        Type::unpack(self.pool.get(id))
    }

    pub fn insert_s<Type: Variant<Container>>(&mut self, value: Type) -> Id<Type> {
        let packed = Type::pack(value);
        let id = self.pool.insert(packed);
        Id {
            id: id.id,
            _type: PhantomData,
        }
    }
}

impl<Container, InnerPool: Pool<Type = Container>> Pool for Variadic<Container, InnerPool> {
    type Type = Container;

    fn get(&self, id: Id<Self::Type>) -> &Self::Type {
        self.pool.get(id)
    }

    fn insert(&mut self, value: Self::Type) -> Id<Self::Type> {
        self.pool.insert(value)
    }
}

impl<Container, InnerPool: Pool<Type = Container> + Default> Default
    for Variadic<Container, InnerPool>
{
    fn default() -> Self {
        let pool = Default::default();
        Self { pool }
    }
}

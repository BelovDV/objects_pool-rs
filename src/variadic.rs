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
/// # Examples
///
/// ```
/// use objects_pool::Variadic;
///
/// // let mut pool = PoolVarios::default();
///
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

// To be done: macros. May be proc...
// #[macro_export]
// macro_rules! variadic {
//     () => {};
// }

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

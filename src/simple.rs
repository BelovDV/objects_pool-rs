use std::{collections::HashMap, marker::PhantomData};

use super::id::Id;
use super::Pool;

/// A set of objects.
/// Keeps all objects while exists.
/// `Id`s expected to be used like references.
///
/// # Example
///
/// ```
/// use objects_pool::{Pool as _, Simple as PoolSimple};
///
/// let mut pool = PoolSimple::default();
///
/// let k_123 = pool.insert(vec![1, 2, 3]);
/// let k_123_other = pool.insert(vec![1, 2, 3]);
/// let k_2345 = pool.insert(vec![2, 3, 4, 5]);
/// let k_123_copy = k_123;
///
/// assert!(pool.get(k_123) == &vec![1, 2, 3]);
///
/// assert!(k_123 != k_2345);
/// assert!(k_123 == k_123_copy);
/// assert!(k_123 != k_123_other);
///
/// match pool.get(k_2345)[1..3] {
///     [3, 4] => {}
///     _ => panic!()
/// }
/// ```
///
/// # Caveats
///
/// `Id` can only be used with set which is gotten from.
///
/// Uses `usize::add(1)` as `Id` generator.
pub struct Simple<Type> {
    pool: HashMap<usize, Type>,
    key: usize,
}

impl<Type> Pool for Simple<Type> {
    type Type = Type;

    fn get(&self, id: Id<Type>) -> &Type {
        self.pool
            .get(&id.id)
            .expect("`Id` can only be used with pool that gave it")
    }

    fn insert(&mut self, value: Type) -> Id<Type> {
        self.key += 1;
        self.pool.insert(self.key, value);
        Id {
            id: self.key,
            _type: PhantomData,
        }
    }
}

impl<Type> Default for Simple<Type> {
    fn default() -> Self {
        Self {
            pool: Default::default(),
            key: 0,
        }
    }
}

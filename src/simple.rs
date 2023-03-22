use std::{collections::HashMap, marker::PhantomData};

/// A set of objects.
/// Keeps all objects while exists.
/// `Id`s expected to be used like references.
///
/// # Examples
///
/// ```
/// use objects_pool::PoolSimple;
///
/// let mut pool = PoolSimple::default();
///
/// let k_123 = pool.insert(vec![1, 2, 3]);
/// let k_2345 = pool.insert(vec![2, 3, 4, 5]);
/// let k_123_copy = k_123;
///
/// assert!(pool.get(k_123) == &vec![1, 2, 3]);
/// assert!(k_123 != k_2345);
/// assert!(k_123 == k_123_copy);
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
pub struct PoolSimple<Type> {
    pool: HashMap<usize, Type>,
    key: usize,
}

pub struct IdSimple<Type> {
    id: usize,
    _type: PhantomData<Type>,
}

impl<Type> PoolSimple<Type> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, id: IdSimple<Type>) -> &Type {
        self.pool
            .get(&id.id)
            .expect("`Id` can only be used with pool that gave it")
    }

    #[must_use = "`Id` is the only way to access stored `value`"]
    pub fn insert(&mut self, value: Type) -> IdSimple<Type> {
        self.key += 1;
        self.pool.insert(self.key, value);
        IdSimple {
            id: self.key,
            _type: PhantomData,
        }
    }
}

impl<Type> Default for PoolSimple<Type> {
    fn default() -> Self {
        Self {
            pool: Default::default(),
            key: 0,
        }
    }
}

impl<T> PartialEq for IdSimple<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for IdSimple<T> {
    fn assert_receiver_is_total_eq(&self) {}
}
impl<T> Clone for IdSimple<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _type: self._type.clone(),
        }
    }
}
impl<T> Copy for IdSimple<T> {}
impl<T> std::fmt::Debug for IdSimple<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IdSimple({})", self.id))
    }
}

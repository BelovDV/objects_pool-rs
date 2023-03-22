use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::Hasher;
use std::marker::PhantomData;

use super::id::Id;
use super::Pool;

/// A set of objects.
/// Keeps all objects while exists.
/// `Id`s expected to be used like references.
/// Unique objects are considered to be the same.
///
/// # Examples
///
/// ```
/// use objects_pool::{Pool as _, Unique as PoolUnique};
///
/// let mut pool = PoolUnique::default();
///
/// let k_abc = pool.insert("abc".to_string());
/// let k_bcd = pool.insert("bcd".to_string());
/// let k_abc_other = pool.insert("abc".to_string());
///
/// assert!(pool.get(k_abc).as_str() == "abc");
/// assert!(k_abc == k_abc_other);
/// assert!(k_abc != k_bcd);
///
/// match pool.get(k_bcd).as_str() {
///     "bcd" => {}
///     _ => panic!()
/// }
/// ```
///
/// # Caveats
///
/// `Id` can only be used with set which is gotten from.
///
/// Uses `usize::add(1)` as `Id` generator.
pub struct Unique<Type: Eq + std::hash::Hash> {
    pool: HashMap<usize, Type>,
    // To be done: don't use such workaround.
    used_hashs: HashMap<u64, Vec<usize>>,
    key: usize,
}

impl<Type: Eq + std::hash::Hash> Pool for Unique<Type> {
    type Type = Type;

    fn get(&self, id: Id<Type>) -> &Type {
        self.pool
            .get(&id.id)
            .expect("`Id` can only be used with pool that gave it")
    }

    fn insert(&mut self, value: Type) -> Id<Type> {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(already) = self.contains(hash, &value) {
            return already;
        }

        self.key += 1;
        self.pool.insert(self.key, value);
        self.used_hashs.entry(hash).or_default().push(self.key);
        Id {
            id: self.key,
            _type: PhantomData,
        }
    }
}

impl<Type: Eq + std::hash::Hash> Unique<Type> {
    pub fn contains(&self, hash: u64, value: &Type) -> Option<Id<Type>> {
        let _type = PhantomData;
        self.used_hashs
            .get(&hash)
            .map(|v| v.iter().find(|id| self.pool.get(&id).unwrap() == value))
            .flatten()
            .map(|&id| Id { id, _type })
    }
}

impl<Type: Eq + std::hash::Hash> Default for Unique<Type> {
    fn default() -> Self {
        Self {
            pool: Default::default(),
            used_hashs: Default::default(),
            key: 0,
        }
    }
}

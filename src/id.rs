use std::marker::PhantomData;

pub struct Id<Type> {
    // To be done: allow u32.
    pub(crate) id: usize,
    pub(crate) _type: PhantomData<Type>,
}

impl<Type> Id<Type> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            _type: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Id<T> {
    fn assert_receiver_is_total_eq(&self) {}
}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _type: self._type.clone(),
        }
    }
}
impl<T> Copy for Id<T> {}
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IdSimple({})", self.id))
    }
}

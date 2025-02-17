use std::rc::Weak;
use std::hash::{Hash, Hasher};

/// A lightweight wrapper around a `Weak<T>` for use as
/// a `HashMap` key when referenced data is mutable.
///
/// Using a weak reference (instead of an `Rc<T>`) prevents
/// the map from extending the lifetime of the data and avoids
/// reference cycles. Equality and hashing is based on the
/// pointer address, so no full ownership is taken and by
/// using `Weak<T>` we are able to access the data by calling
/// `.weak_reference.upgrade()`.
pub struct WeakPtrHash<T> {
    pub weak_reference: Weak<T>,
}


impl<T> PartialEq for WeakPtrHash<T> {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.weak_reference, &other.weak_reference)
    }
}


impl<T> Eq for WeakPtrHash<T> {
    // empty
}


impl<T> Hash for WeakPtrHash<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.weak_reference.as_ptr().hash(state)
    }
}


impl<T> Clone for WeakPtrHash<T> {
    fn clone(&self) -> Self {
        WeakPtrHash {
            weak_reference: self.weak_reference.clone(),
        }
    }
}

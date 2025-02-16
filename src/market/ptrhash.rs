use std::rc::Weak;
use std::hash::{Hash, Hasher};

// wrapper around a 'Weak<T>' reference. 'T' is typically a custom 'struct'
// type with mutable 'Cell<_>' field. It is used as a key in a HashMap
// to track objects without preventing them from being garbage collected
// once all strong references 'Rc<T>' are dropped thus preventing ownership
// cycles and memory leaks.
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

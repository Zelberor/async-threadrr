use std::sync::{Arc, Mutex};

trait ListElement {
    type T;
    fn swap_next(&self, next: Option<Arc<Self>>) -> Option<Arc<Self>>;
}

struct GenericListElement<T> {
    data: T,
    next: Mutex<Option<Arc<Self>>>,
}

impl<T> ListElement for GenericListElement<T> {
    type T = T;

    fn swap_next(&self, next: Option<Arc<Self>>) -> Option<Arc<Self>> {
        // TODO: proper error handling
        let mut mutex = self.next.lock().unwrap();
        let current = mutex.take();
        *mutex = next;
        current
    }
}

struct LinkedList<E, T>
where
    E: ListElement<T = T>,
{
    first: Mutex<Option<Arc<E>>>,
    last: Mutex<Option<Arc<E>>>,
}

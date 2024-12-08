use core::ptr::null_mut;

pub struct LinkedList<T> {
    first: *mut T,
    last: *mut T,
}

pub trait LinkedListItem {
    fn prev(&self) -> *mut Self;

    fn next(&self) -> *mut Self;

    fn set_prev(&mut self, prev: *mut Self);

    fn set_next(&mut self, next: *mut Self);
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            first: null_mut(),
            last: null_mut(),
        }
    }

    pub const fn first(&self) -> *mut T {
        self.first
    }
}

impl<T: LinkedListItem> LinkedList<T> {
    /// Push a new element at the beginning of the list.
    pub fn push_front(&mut self, el: &mut T) {
        el.set_next(self.first);
        el.set_prev(null_mut());

        if let Some(first) = unsafe { self.first.as_mut() } {
            first.set_prev(el);
        } else {
            self.last = el;
        }

        self.first = el;
    }

    /// Push a new element at the end of the list.
    ///
    /// Returns whether the first element of the list is updated.
    pub fn push_back(&mut self, el: &mut T) -> bool {
        el.set_prev(self.last);
        el.set_next(null_mut());

        let result = if let Some(last) = unsafe { self.last.as_mut() } {
            last.set_next(el);
            false
        } else {
            self.first = el;
            true
        };

        self.last = el;

        result
    }

    /// Remove an element from the list. The element must be in the list.
    ///
    /// Returns whether the first element of the list is updated.
    pub fn remove(&mut self, el: &mut T) -> bool {
        if let Some(prev) = unsafe { el.prev().as_mut() } {
            prev.set_next(el.next());
        }
        if let Some(next) = unsafe { el.next().as_mut() } {
            next.set_prev(el.prev());
        }
        let first_updated = el as *const _ == self.first;
        if first_updated {
            self.first = el.next();
        }
        if el as *const _ == self.last {
            self.last = el.prev();
        }
        el.set_prev(null_mut());
        el.set_next(null_mut());
        first_updated
    }

    /// Check if an element is in the list. The element must not be in another list.
    pub fn contains(&self, el: &T) -> bool {
        !el.next().is_null() || !el.prev().is_null() || el as *const _ == self.first
    }
}

macro_rules! impl_list_item {
    ($name: ident) => {
        impl crate::list::LinkedListItem for $name {
            fn prev(&self) -> *mut Self {
                self.prev
            }

            fn next(&self) -> *mut Self {
                self.next
            }

            fn set_prev(&mut self, prev: *mut Self) {
                self.prev = prev
            }

            fn set_next(&mut self, next: *mut Self) {
                self.next = next
            }
        }
    };
}

pub(crate) use impl_list_item;

// TODO: Use `Rc` and `RefCell` to implement `DropTracker<T>`, a wrapper around a value of type `T`
//  that increments a shared `usize` counter every time the wrapped value is dropped.

use std::cell::RefCell;
use std::rc::Rc;

// Generic struct
pub struct DropTracker<T> {
    value: T,
    counter: Rc<RefCell<i32>>,
}

// Implement methods for the generic struct 
impl<T> DropTracker<T> {
    pub fn new(value: T, counter: Rc<RefCell<i32>>) -> Self {
        Self { value, counter }
    }
}

// The Drop trait is a mechanism to define additional cleanup logic for types
impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        let mut val = (&self.counter).borrow_mut();
        *val += 1;

        // Create an Rc with a value
        // let rc1 = Rc::new(5);
        // Create a second Rc pointing to the same value
        // let rc2 = Rc::clone(&rc1);
        // Rc::strong_count() tells how many Rc instances are currently pointing to the same value.
        // self.counter = Rc::strong_count(&rc1));

        // Create a RefCell holding the value 5
        // let my_var = RefCell::new(5);
        // Borrow mutably and modify the value
        // let mut val = my_var.borrow_mut();
        // *val += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Rc is a reference-counted pointer: it wraps around a value and keeps track of how many references to the value exist.
        // RefCell allows to mutate the value wrapped in a RefCell even if there is only an immutable reference to the RefCell itself.
        let counter = Rc::new(RefCell::new(0));
        // The inherent methods of Rc are all associated functions,
        // which means that you have to call them as Rc::clone(&counter) instead of &counter.clone()
        // let _ drops the value on the rhs, and call the destructor (drop)
        // When we call `clone`, the RefCell data is not copied! Instead, the reference count for `Rc` is incremented.
        // Creates a DropTracker that holds the value () with a counter
        let _ = DropTracker::new((), Rc::clone(&counter));
        // Immutable borrow
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        // Initialize the counter value to 0 :
        // With RefCell, the value of the counter can be modified using an immutable reference
        // Rc allows multiple owners of the same data
        let counter = Rc::new(RefCell::new(0));

        {
            // When we call `clone`, the RefCell data is not copied! Instead, the reference count for `Rc` is incremented.
            // The counter is shared between a and b using Rc.
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }  // Both `a` and `b` go out of scope here, and their drop methods will be called
        // When DropTracker instances are dropped, they mutate the shared counter using the RefCell, incrementing its value.

        assert_eq!(*counter.borrow(), 2);
    }
}

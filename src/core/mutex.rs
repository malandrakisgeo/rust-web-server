/*
	This is a oversimplified mutex created from scratch. My first contribution to mrgian's felix OS.

	Meant to be used for global, static definitions of objects, visible for all active threads.
	Once a thread acquires the target object, all other threads trying to do so will wait until it is freed.

	There is plenty of room for improvements.

	 TODO: Improve
*/
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
    target: T,
    free: AtomicBool,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            target: value,
            free: AtomicBool::new(true),
        }
    }

    //WARNING: You MUST call free()  after using acquire() or acquire_mut() when the target is no longer needed. Not doing so can, and will, lead to problems.
    pub fn acquire_mut(&mut self) -> &mut T {
        while !self.free.load(Ordering::SeqCst) {} // Wait until free is true
        self.free.store(false, Ordering::SeqCst); // Set free to false
        return &mut self.target;
    }

    //WARNING: You MUST call free()  after using acquire() or acquire_mut() when the target is no longer needed. Not doing so can, and will, lead to problems.
    pub fn acquire(&mut self) -> &T {
        while !self.free.load(Ordering::SeqCst) {} // Wait until free is true
        self.free.store(false, Ordering::SeqCst); // Set free to false
        return &self.target;
    }

    pub fn free(&self) {
        self.free.store(true, Ordering::SeqCst); // Set free to true
    }
}

impl<T>  Drop for Mutex<T> {
    fn drop(&mut self) {
        self.free = AtomicBool::from(true);
    }
}

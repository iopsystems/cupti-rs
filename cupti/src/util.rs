use std::sync::{Mutex, MutexGuard};

pub(crate) struct NonPoisonMutex<T>(Mutex<T>);

impl<T> NonPoisonMutex<T> {
    pub const fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        match self.0.lock() {
            Ok(guard) => guard,
            Err(err) => {
                self.0.clear_poison();
                err.into_inner()
            }
        }
    }
}

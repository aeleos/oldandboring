//! Handles mutual exclusion to data.
//!
//! This is a modification of the PreemptableMutex code from the spin crate (see
//! https://crates.io/crates/spin).

use super::{cpu_relax, disable_preemption, restore_preemption_state, PreemptionState};
use core::cell::UnsafeCell;
use core::default::Default;
use core::fmt;
use core::marker::Sync;
use core::ops::{Deref, DerefMut, Drop};
use core::option::Option::{self, None, Some};
use core::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

/// This type provides MUTual EXclusion based on spinning.
///
/// # Description
///
/// This structure behaves a lot like a normal Mutex. There are some
/// differences:
///
/// - It may be used outside the runtime.
/// - A normal Mutex will fail when used without the runtime, this will just
/// lock
/// - When the runtime is present, it will call the deschedule function when
/// appropriate
/// - No lock poisoning. When a fail occurs when the lock is held, no
/// guarantees are made
///
/// When calling rust functions from bare threads, such as C `pthread`s, this
/// lock will be very
/// helpful. In other cases however, you are encouraged to use the locks from
/// the standard
/// library.
///
pub struct PreemptableMutex<T: ?Sized> {
    lock: AtomicBool,
    preemption_state: UnsafeCell<PreemptionState>,
    data: UnsafeCell<T>,
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
pub struct PreemptableMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    preemption_state: &'a PreemptionState,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::PreemptableMutex`
unsafe impl<T: ?Sized + Send> Sync for PreemptableMutex<T> {}
unsafe impl<T: ?Sized + Send> Send for PreemptableMutex<T> {}

impl<T> PreemptableMutex<T> {
    /// Creates a new spinlock wrapping the supplied data.
    pub const fn new(user_data: T) -> PreemptableMutex<T> {
        PreemptableMutex {
            lock: ATOMIC_BOOL_INIT,
            preemption_state: UnsafeCell::new(PreemptionState::default()),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this PreemptableMutex, returning the underlying data.
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let PreemptableMutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> PreemptableMutex<T> {
    fn obtain_lock(&self) {
        let mut preemption_state;
        loop {
            unsafe {
                preemption_state = disable_preemption();
            }
            let lock_switch = !self.lock.compare_and_swap(false, true, Ordering::Acquire);
            if lock_switch {
                break;
            } else {
                unsafe {
                    restore_preemption_state(&preemption_state);
                }
            }

            // Wait until the lock looks unlocked before retrying
            while self.lock.load(Ordering::Relaxed) {
                cpu_relax();
            }
        }

        unsafe {
            *self.preemption_state.get() = preemption_state;
        }
    }

    /// Locks the spinlock and returns a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    pub fn lock(&self) -> PreemptableMutexGuard<T> {
        self.obtain_lock();
        PreemptableMutexGuard {
            lock: &self.lock,
            preemption_state: unsafe { &*self.preemption_state.get() },
            data: unsafe { &mut *self.data.get() },
        }
    }

    /// Returns a reference to the contained data, without locking the PreemptableMutex.
    ///
    /// This intended for use in the scheduler, where no locks should be held
    /// while switching
    /// contexts.
    ///
    /// # Safety
    /// This function is **very** unsafe.
    /// - Make sure that mutual exclusion is guaranteed for the accessed data.
    pub unsafe fn without_locking(&self) -> &T {
        &*self.data.get()
    }

    /// Tries to lock the PreemptableMutex. If it is already locked, it will return None.
    /// Otherwise it returns
    /// a guard within Some.
    pub fn try_lock(&self) -> Option<PreemptableMutexGuard<T>> {
        let preemption_state = unsafe { disable_preemption() };
        let lock_switch = !self.lock.compare_and_swap(false, true, Ordering::Acquire);
        // if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false
        if lock_switch {
            unsafe {
                *self.preemption_state.get() = preemption_state;
            }
            Some(PreemptableMutexGuard {
                lock: &self.lock,
                preemption_state: unsafe { &*self.preemption_state.get() },
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            unsafe {
                restore_preemption_state(&preemption_state);
            }
            None
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for PreemptableMutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "PreemptableMutex {{ data: {:?} }}", &*guard),
            None => write!(f, "PreemptableMutex {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for PreemptableMutex<T> {
    fn default() -> PreemptableMutex<T> {
        PreemptableMutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for PreemptableMutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for PreemptableMutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for PreemptableMutexGuard<'a, T> {
    /// The dropping of the PreemptableMutexGuard will release the lock it was created
    /// from.
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        unsafe {
            restore_preemption_state(self.preemption_state);
        }
    }
}

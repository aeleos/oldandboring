//! Handles mutual exclusion to data.
//!
//! This is a modification of the KernelMutex code from the spin crate (see
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
/// This structure behaves a lot like a normal KernelMutex. There are some
/// differences:
///
/// - It may be used outside the runtime.
/// - A normal KernelMutex will fail when used without the runtime, this will just
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
pub struct KernelMutex<T: ?Sized> {
    lock: AtomicBool,
    preemption_state: UnsafeCell<PreemptionState>,
    data: UnsafeCell<T>,
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
pub struct KernelMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    preemption_state: &'a PreemptionState,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::KernelMutex`
unsafe impl<T: ?Sized + Send> Sync for KernelMutex<T> {}
unsafe impl<T: ?Sized + Send> Send for KernelMutex<T> {}

impl<T> KernelMutex<T> {
    /// Creates a new spinlock wrapping the supplied data.
    pub const fn new(user_data: T) -> KernelMutex<T> {
        KernelMutex {
            lock: ATOMIC_BOOL_INIT,
            preemption_state: UnsafeCell::new(PreemptionState::default()),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this KernelMutex, returning the underlying data.
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let KernelMutex { data, .. } = self;
        unsafe { data.into_inner() }
    }
}

impl<T: ?Sized> KernelMutex<T> {
    fn obtain_lock(&self) {
        // while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false
        //
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
    pub fn lock(&self) -> KernelMutexGuard<T> {
        self.obtain_lock();
        KernelMutexGuard {
            lock: &self.lock,
            preemption_state: unsafe { &*self.preemption_state.get() },
            data: unsafe { &mut *self.data.get() },
        }
    }

    /// Returns a reference to the contained data, without locking the KernelMutex.
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

    /// Tries to lock the KernelMutex. If it is already locked, it will return None.
    /// Otherwise it returns
    /// a guard within Some.
    pub fn try_lock(&self) -> Option<KernelMutexGuard<T>> {
        let preemption_state = unsafe { disable_preemption() };
        let lock_switch = !self.lock.compare_and_swap(false, true, Ordering::Acquire);
        // if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false
        if lock_switch {
            unsafe {
                *self.preemption_state.get() = preemption_state;
            }
            Some(KernelMutexGuard {
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

impl<T: ?Sized + fmt::Debug> fmt::Debug for KernelMutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "KernelMutex {{ data: {:?} }}", &*guard),
            None => write!(f, "KernelMutex {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for KernelMutex<T> {
    fn default() -> KernelMutex<T> {
        KernelMutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for KernelMutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for KernelMutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for KernelMutexGuard<'a, T> {
    /// The dropping of the KernelMutexGuard will release the lock it was created
    /// from.
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        unsafe {
            restore_preemption_state(self.preemption_state);
        }
    }
}

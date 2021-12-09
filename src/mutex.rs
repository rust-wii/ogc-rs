use core::cell::UnsafeCell;
use core::fmt;

//use lock_api::{RawMutex, GuardNoSend};

use crate::ffi;

/// An enumeration of possible errors associated with a `LockResult` which can
/// occur while trying to call a method on a Mutex.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum LockError {
    /// The lock could not be acquired at this time because the operation would
    /// otherwise block.
    WouldBlock = 1,
    /// Generic error
    Unknown = -1,
}

/// A type alias for the result of a lock method which can error.
pub type LockResult<T> = Result<T, LockError>;

// ========================================================================== //

/// A mutual exclusion primitive useful for protecting shared data
///
/// This mutex will block threads waiting for the lock to become available. The
/// mutex can also be statically initialized or created via a [`new`]
/// constructor. Each mutex has a type parameter which represents the data that
/// it is protecting. The data can only be accessed through the RAII guards
/// returned from [`lock`] and [`try_lock`], which guarantees that the data is
/// only ever accessed when the mutex is locked.
pub struct Mutex<T: ?Sized> {
    inner: ffi::mutex_t,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    ///
    /// # Errors
    /// If `libogc` could not allocate a mutex, this returns an `Unknown` error.
    pub fn new(t: T) -> LockResult<Self> {
        let mut handle = core::mem::MaybeUninit::uninit();
        unsafe {
            let res = ffi::LWP_MutexInit(handle.as_mut_ptr(), false);

            if res < 0 {
                Err(LockError::Unknown)
            } else {
                Ok(Mutex {
                    inner: handle.assume_init(),
                    data: UnsafeCell::new(t),
                })
            }
        }
    }

    /// Closes this mutex when there are no more handles to it, returning its
    /// inner value.
    ///
    /// # Errors
    /// If this lock is still being used, this returns the handle to the mutex.
    pub fn close(self) -> Result<T, Self> {
        let res = unsafe { ffi::LWP_MutexUnlock(self.inner) };

        if res < 0 {
            Err(self)
        } else {
            // maybe there's a way of getting the value while still having the
            // drop glue for the mutex?
            Ok(self.data.into_inner())
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Acquires a mutex, blocking the current thread until it is able to do so.
    ///
    /// This function will block the local thread until it is available to
    /// acquire the mutex. Upon returning, the thread is the only thread with
    /// the lock held. An RAII guard is returned to allow scoped unlock of the
    /// lock. When the guard goes out of scope, the mutex will be unlocked.
    ///
    /// # Errors
    /// If `libogc` could not lock the mutex, this returns an error.
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        let res = unsafe { ffi::LWP_MutexLock(self.inner) };

        if res < 0 {
            Err(LockError::Unknown)
        } else {
            Ok(MutexGuard { lock: self })
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned. The lock will be unlocked when the
    /// guard is dropped.
    ///
    /// This function does not block.
    ///
    /// # Errors
    /// If the mutex could not be acquired because it is already locked, then
    /// this call will return a `WouldBlock` error. If it fails for some other
    /// reason, it returns an error.
    pub fn try_lock(&self) -> LockResult<MutexGuard<'_, T>> {
        let res = unsafe { ffi::LWP_MutexTryLock(self.inner) };

        // 0: lock acquired, 1: would deadlock
        match res {
            0 => Ok(MutexGuard { lock: self }),
            1 => Err(LockError::WouldBlock),
            _ => Err(LockError::Unknown),
        }
    }

    /// Immediately drops the guard, and consequently unlocks the mutex.
    ///
    /// This function is equivalent to calling drop on the guard but is more
    /// self-documenting and does not panic on failure. Alternately, the guard
    /// will be automatically dropped when it goes out of scope.
    ///
    /// # Errors
    /// If the mutex could not be unlocked for any reason, this will return an
    /// error.
    pub fn unlock(guard: MutexGuard<'_, T>) -> LockResult<()> {
        let res = unsafe { ffi::LWP_MutexUnlock(guard.lock.inner) };

        if res < 0 {
            Err(LockError::Unknown)
        } else {
            Ok(())
        }
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `Mutex` mutably, no actual locking needs to
    /// take place – the mutable borrow statically guarantees no locks exist.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Mutex");
        match self.try_lock() {
            Ok(guard) => {
                d.field("data", &&*guard);
            }
            Err(LockError::WouldBlock) => {
                struct LockedPlaceholder;
                impl fmt::Debug for LockedPlaceholder {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("<locked>")
                    }
                }
                d.field("data", &LockedPlaceholder);
            }
            Err(_) => {
                struct ErrorPlaceholder;
                impl fmt::Debug for ErrorPlaceholder {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("<error>")
                    }
                }
                d.field("data", &ErrorPlaceholder);
            }
        }
        d.finish_non_exhaustive()
    }
}

/*
// FIXME: Can't use because `Mutex::close()` moves out `.data` (since
// `UnsafeCell` can't do `Copy`), but `Drop::drop()` still needs a valid value
// when running!
impl<T: ?Sized> Drop for Mutex<T> {
    fn drop(&mut self) {
        let res = unsafe {
            ffi::LWP_MutexDestroy(self.inner)
        };
        // necessary? idk. better safe than sorry tho
        assert_eq!(res, 0, "could not destroy mutex");
    }
}
*/

// ========================================================================== //

/// An RAII implementation of a “scoped lock” of a mutex. When this structure is
/// dropped (falls out of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its
/// [`Deref`] and [`DerefMut`] implementations.
///
/// This structure is created by the [`lock`] and [`try_lock`] methods on Mutex.
#[must_use = "if unused, the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
}

impl<T> !Send for MutexGuard<'_, T> {}
unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> core::ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let res = unsafe { ffi::LWP_MutexUnlock(self.lock.inner) };
        // panic in case of error to uphold semantic barriers.
        assert_eq!(res, 0, "could not unlock mutex");
    }
}

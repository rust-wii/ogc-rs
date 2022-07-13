//! The ``lwp`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around thread based functions.

use crate::ffi;
use core::ffi::c_void;

pub const ALREADY_SUSPENDED: i32 = ffi::LWP_ALREADY_SUSPENDED as i32;
pub const SUCCESSFUL: i32 = ffi::LWP_SUCCESSFUL as i32;

/// A thread context handle.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Thread {
    handle: ffi::lwp_t,
}

impl Thread {
    pub(crate) fn new(handle: ffi::lwp_t) -> Thread {
        Thread { handle }
    }

    /// Test whether this thread is suspended or not.
    pub fn is_suspended(&self) -> bool {
        unsafe { ffi::LWP_ThreadIsSuspended(self.handle) != 0 }
    }

    /// Suspend this thread.
    ///
    /// On success, returns `Ok(1)` if the thread was already suspended, or `Ok(0)` if it was
    /// successfully suspended. Otherwise, returns an error code.
    pub fn suspend(&self) -> Result<i32, i32> {
        let res = unsafe { ffi::LWP_SuspendThread(self.handle) };

        if res < 0 {
            Err(res)
        } else {
            Ok(res)
        }
    }

    /// Resume this thread.
    ///
    /// On success, returns `Ok(1)` if the thread was already resumed, or `Ok(0)` if it was
    /// successfully resumed. Otherwise, returns an error code.
    pub fn resume(&self) -> Result<i32, i32> {
        let res = unsafe { ffi::LWP_ResumeThread(self.handle) };

        if res < 0 {
            Err(res)
        } else {
            Ok(res)
        }
    }

    /// Set the priority of this thread.
    pub fn set_priority(&self, prio: u8) {
        unsafe { ffi::LWP_SetThreadPriority(self.handle, prio as u32) }
    }

    /// Join this thread.
    pub fn join(&self) -> Result<*mut c_void, i32> {
        let mut ret = core::mem::MaybeUninit::uninit();
        unsafe {
            let res = ffi::LWP_JoinThread(self.handle, ret.as_mut_ptr());

            if res < 0 {
                Err(res)
            } else {
                Ok(ret.assume_init())
            }
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    arg: *mut c_void,
    stack_base: *mut c_void,
    stack_size: usize,
    priority: u8,
}

pub type EntryFn = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
<<<<<<< HEAD
	pub fn new() -> Self {
		Builder {
			arg: core::ptr::null_mut(),
			stack_base: core::ptr::null_mut(),
			stack_size: 0,
			priority: 0,
		}
	}

	#[must_use]
	pub fn arg(mut self, arg: *mut c_void) -> Self {
		self.arg = arg;
		self
	}

	#[must_use]
	pub fn stack_base(mut self, base: *mut c_void) -> Self {
		self.stack_base = base;
		self
	}

	#[must_use]
	pub fn stack_size(mut self, size: usize) -> Self {
		self.stack_size = size;
		self
	}

	#[must_use]
	pub fn priority(mut self, prio: u8) -> Self {
		self.priority = prio;
		self
	}

	pub fn spawn(
		self,
		entry: EntryFn,
	) -> Result<Thread, i32> {
		let mut thread = core::mem::MaybeUninit::uninit();
		unsafe {
			let res = ffi::LWP_CreateThread(
				thread.as_mut_ptr(),
				entry,
				self.arg,
				self.stack_base,
				self.stack_size as u32,
				self.priority,
			);

			if res < 0 {
				Err(res)
			} else {
				Ok(Thread::new(thread.assume_init()))
			}
		}
	}
}

/// A thread queue's context handle.
#[derive(Debug)]
pub struct Queue {
    handle: ffi::lwpq_t,
}

impl Queue {
    /// Initialize the thread synchronization queue.
    pub fn new() -> Result<Self, i32> {
        let mut q = core::mem::MaybeUninit::uninit();
        unsafe {
            let res = ffi::LWP_InitQueue(q.as_mut_ptr());

            if res < 0 {
                Err(res)
            } else {
                Ok(Queue {
                    handle: q.assume_init(),
                })
            }
        }
    }

    /// Removes all blocked threads from the thread synchronization queue and sets them back to
    /// running state.
    pub fn broadcast(&self) {
        unsafe { ffi::LWP_ThreadBroadcast(self.handle) }
    }

    /// Signals one thread to be revmoved from the thread synchronization queue and sets it back to
    /// running state.
    pub fn signal(&self) {
        unsafe { ffi::LWP_ThreadSignal(self.handle) }
    }
}

impl Drop for Queue {
    /// Close the thread synchronization queue and release the handle.
    fn drop(&mut self) {
        unsafe { ffi::LWP_CloseQueue(self.handle) }
    }
}

/// Return the handle to the current thread.
pub fn current() -> Thread {
    unsafe { Thread::new(ffi::LWP_GetSelf()) }
}

/// Yield the current thread to another one with higher priority or, if not running, at the same
/// priority whose state is runnable.
pub fn yield_now() {
    unsafe { ffi::LWP_YieldThread() }
}

/// Set the priority of the current thread.
pub fn set_priority(prio: u8) {
    unsafe { ffi::LWP_SetThreadPriority(ffi::LWP_THREAD_NULL, prio as u32) }
}

/// Reschedule all threads running at the given priority.
pub fn reschedule(prio: u8) {
    unsafe { ffi::LWP_Reschedule(prio as u32) }
}

/// Pushes the current thread onto the given thread synchronization queue and sets the thread state
/// to blocked.
pub fn sleep(q: Queue) -> Result<(), i32> {
    unsafe {
        let res = ffi::LWP_ThreadSleep(q.handle);

        if res < 0 {
            Err(res)
        } else {
            Ok(())
        }
    }
}

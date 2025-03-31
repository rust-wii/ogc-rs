use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use crate::arch;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    pub fn try_lock(&self) -> Option<Guard<T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            Some(Guard { lock: self })
        } else {
            None
        }
    }

    pub fn try_with_lock<R>(&self, f: impl FnOnce(Guard<'_, T>) -> R) -> Option<R> {
        if let Some(guard) = self.try_lock() {
            Some(f(guard))
        } else {
            None
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        Guard { lock: self }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(Guard<'_, T>) -> R) -> R {
        let guard = self.lock();
        f(guard)
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T, R> AsRef<R> for Guard<'a, T>
where
    T: AsRef<R>,
{
    fn as_ref(&self) -> &R {
        self.deref().as_ref()
    }
}

impl<'a, T, R> AsMut<R> for Guard<'a, T>
where
    T: AsMut<R>,
{
    fn as_mut(&mut self) -> &mut R {
        self.deref_mut().as_mut()
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}
unsafe impl<T> Send for Guard<'_, T> where T: Send {}

pub struct CriticalSectionSpinLock<T> {
    inner: SpinLock<T>,
}

impl<T> CriticalSectionSpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: SpinLock::new(value),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    pub fn try_lock(&self) -> Option<Guard<T>> {
        arch::with_interrupts_disabled(|| self.inner.try_lock())
    }

    pub fn try_with_lock<R>(&self, f: impl FnOnce(Guard<'_, T>) -> R) -> Option<R> {
        arch::with_interrupts_disabled(|| self.inner.try_with_lock(f))
    }

    pub fn lock(&self) -> Guard<T> {
        arch::with_interrupts_disabled(|| self.inner.lock())
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(Guard<'_, T>) -> R) -> R {
        arch::with_interrupts_disabled(|| self.inner.with_lock(f))
    }
}

unsafe impl<T> Sync for CriticalSectionSpinLock<T> where T: Send {}

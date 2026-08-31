/* SPDX-License-Identifier: GPL-2.0 */

// C includes removed: <pthread.h>, <stdbool.h>.
// The pthread types, constants, and functions are expected to be supplied by
// surrounding bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_macros)]

use core::ffi::{c_int, c_void};
use core::ptr;

pub type spinlock_t = pthread_mutex_t;

macro_rules! DEFINE_SPINLOCK {
    ($x:ident) => {
        pub static mut $x: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
    };
}

macro_rules! __SPIN_LOCK_UNLOCKED {
    ($x:expr) => {
        PTHREAD_MUTEX_INITIALIZER
    };
}

pub unsafe fn spin_lock_init(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_init(x, ptr::null::<c_void>() as *const pthread_mutexattr_t) }
}

pub unsafe fn spin_lock(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_lock(x) }
}

pub unsafe fn spin_lock_nested(x: *mut pthread_mutex_t, _subclass: c_int) -> c_int {
    unsafe { pthread_mutex_lock(x) }
}

pub unsafe fn spin_unlock(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_unlock(x) }
}

pub unsafe fn spin_lock_bh(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_lock(x) }
}

pub unsafe fn spin_unlock_bh(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_unlock(x) }
}

pub unsafe fn spin_lock_irq(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_lock(x) }
}

pub unsafe fn spin_unlock_irq(x: *mut pthread_mutex_t) -> c_int {
    unsafe { pthread_mutex_unlock(x) }
}

pub unsafe fn spin_lock_irqsave(x: *mut pthread_mutex_t, f: c_int) -> c_int {
    let _ = f;
    unsafe { pthread_mutex_lock(x) }
}

pub unsafe fn spin_unlock_irqrestore(x: *mut pthread_mutex_t, f: c_int) -> c_int {
    let _ = f;
    unsafe { pthread_mutex_unlock(x) }
}

pub type arch_spinlock_t = pthread_mutex_t;

pub const __ARCH_SPIN_LOCK_UNLOCKED: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

pub unsafe fn arch_spin_lock(mutex: *mut arch_spinlock_t) {
    unsafe {
        pthread_mutex_lock(mutex);
    }
}

pub unsafe fn arch_spin_unlock(mutex: *mut arch_spinlock_t) {
    unsafe {
        pthread_mutex_unlock(mutex);
    }
}

pub unsafe fn arch_spin_is_locked(_mutex: *mut arch_spinlock_t) -> bool {
    true
}

unsafe extern "C" {
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
}

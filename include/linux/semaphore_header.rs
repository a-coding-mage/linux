/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2008 Intel Corporation
 * Author: Matthew Wilcox <willy@linux.intel.com>
 *
 * Please see kernel/locking/semaphore.c for documentation of these functions
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/list.h and linux/spinlock.h

/* Please don't access any members of this structure directly */
#[repr(C)]
pub struct semaphore {
    pub lock: raw_spinlock_t,
    pub count: core::ffi::c_uint,
    pub first_waiter: *mut semaphore_waiter,

    // Preserved from CONFIG_DETECT_HUNG_TASK_BLOCKER.
    #[cfg(CONFIG_DETECT_HUNG_TASK_BLOCKER)]
    pub last_holder: core::ffi::c_ulong,
}

// Unlike mutexes, binary semaphores do not have an owner, so up() can
// be called in a different thread from the one which called down().
// It is also safe to call down_trylock() and up() from interrupt
// context.

#[cfg(CONFIG_DETECT_HUNG_TASK_BLOCKER)]
macro_rules! __LAST_HOLDER_SEMAPHORE_INITIALIZER {
    () => { last_holder: 0usize as core::ffi::c_ulong };
}

#[cfg(not(CONFIG_DETECT_HUNG_TASK_BLOCKER))]
macro_rules! __LAST_HOLDER_SEMAPHORE_INITIALIZER {
    () => {};
}

macro_rules! __SEMAPHORE_INITIALIZER {
    ($name:expr, $n:expr) => {
        semaphore {
            lock: __RAW_SPIN_LOCK_UNLOCKED!($name.lock),
            count: $n,
            first_waiter: core::ptr::null_mut(),
            __LAST_HOLDER_SEMAPHORE_INITIALIZER!()
        }
    };
}

macro_rules! DEFINE_SEMAPHORE {
    ($_name:ident, $_n:expr) => {
        let $_name: semaphore = __SEMAPHORE_INITIALIZER!($_name, $_n);
    };
}

pub unsafe fn sema_init(sem: *mut semaphore, val: core::ffi::c_int) {
    static mut __KEY: lock_class_key = lock_class_key::default();
    *sem = __SEMAPHORE_INITIALIZER!(*sem, val);
    lockdep_init_map(
        &mut (*sem).lock.dep_map,
        "semaphore->lock",
        &raw mut __KEY,
        0,
    );
}

pub unsafe extern "C" {
    pub fn down(sem: *mut semaphore);
    pub fn down_interruptible(sem: *mut semaphore) -> core::ffi::c_int;
    pub fn down_killable(sem: *mut semaphore) -> core::ffi::c_int;
    pub fn down_trylock(sem: *mut semaphore) -> core::ffi::c_int;
    pub fn down_timeout(sem: *mut semaphore, jiffies: core::ffi::c_long) -> core::ffi::c_int;
    pub fn up(sem: *mut semaphore);
    pub fn sem_last_holder(sem: *mut semaphore) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

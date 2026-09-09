/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Mutexes: blocking mutual exclusion locks
 *
 * started by Ingo Molnar:
 *
 *  Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 *
 * This file contains the main data structure and API definitions.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

pub struct device;

/* CONFIG_DEBUG_LOCK_ALLOC controls the dependency-map initializer. */
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export]
macro_rules! __DEP_MAP_MUTEX_INITIALIZER {
    ($lockname:ident) => { , dep_map: lockdep_map { name: stringify!($lockname), wait_type_inner: LD_WAIT_SLEEP } };
}
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! __DEP_MAP_MUTEX_INITIALIZER { ($lockname:ident) => {}; }

/* CONFIG_DEBUG_MUTEXES controls debug ownership and destruction. */
#[cfg(CONFIG_DEBUG_MUTEXES)]
extern "C" {
    pub fn mutex_destroy(lock: *mut mutex);
}
#[cfg(not(CONFIG_DEBUG_MUTEXES))]
#[inline]
pub unsafe fn mutex_destroy(_lock: *mut mutex) {}

/*
 * The C mutex type and lockdep/spinlock types are supplied by their respective
 * translated headers.
 */
#[allow(non_camel_case_types)]
pub enum mutex {}
#[allow(non_camel_case_types)]
pub enum lock_class_key {}
#[allow(non_camel_case_types)]
pub enum lockdep_map {}
#[allow(non_camel_case_types)]
pub enum atomic_t {}

/* mutex_init(mutex): initialize mutex to the unlocked state. */
#[macro_export]
macro_rules! mutex_init {
    ($mutex:expr) => {{
        static mut __key: lock_class_key = lock_class_key {};
        unsafe { __mutex_init($mutex, stringify!($mutex).as_ptr() as *const i8, &mut __key) };
    }};
}

#[macro_export]
macro_rules! mutex_init_with_key {
    ($mutex:expr, $key:expr) => { unsafe { __mutex_init($mutex, stringify!($mutex).as_ptr() as *const i8, $key) } };
}

/* !CONFIG_PREEMPT_RT: regular mutex implementation. */
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __MUTEX_INITIALIZER {
    ($lockname:ident) => { mutex { owner: 0, wait_lock: 0, first_waiter: core::ptr::null_mut() } };
}
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! DEFINE_MUTEX { ($mutexname:ident) => { static mut $mutexname: mutex = __MUTEX_INITIALIZER!($mutexname); }; }

#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
extern "C" {
    pub fn mutex_init_lockdep(lock: *mut mutex, name: *const i8, key: *mut lock_class_key);
}
#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
#[inline]
pub unsafe fn __mutex_init(lock: *mut mutex, name: *const i8, key: *mut lock_class_key) {
    mutex_init_lockdep(lock, name, key);
}
#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
extern "C" { pub fn mutex_init_generic(lock: *mut mutex); }
#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
#[inline]
pub unsafe fn __mutex_init(lock: *mut mutex, _name: *const i8, _key: *mut lock_class_key) {
    mutex_init_generic(lock);
}

extern "C" { pub fn mutex_is_locked(lock: *mut mutex) -> bool; }

/* CONFIG_PREEMPT_RT uses rtmutexes; its initializer and fields are supplied by
 * the translated mutex_types header. */
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __MUTEX_INITIALIZER { ($mutexname:ident) => { mutex { rtmutex: __RT_MUTEX_BASE_INITIALIZER!($mutexname) } }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! DEFINE_MUTEX { ($mutexname:ident) => { static mut $mutexname: mutex = __MUTEX_INITIALIZER!($mutexname); }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[inline]
pub unsafe fn mutex_is_locked_rt(l: *mut mutex) -> bool { rt_mutex_base_is_locked(unsafe { &mut (*l).rtmutex }) }

#[cfg(CONFIG_DEBUG_MUTEXES)]
extern "C" { pub fn __devm_mutex_init(dev: *mut device, lock: *mut mutex) -> i32; }
#[cfg(not(CONFIG_DEBUG_MUTEXES))]
#[inline]
pub unsafe fn __devm_mutex_init(_dev: *mut device, _lock: *mut mutex) -> i32 { 0 }

#[macro_export]
macro_rules! __mutex_init_ret {
    ($mutex:expr) => {{ let mutex_ = $mutex; mutex_init!(mutex_); mutex_ }};
}
#[macro_export]
macro_rules! devm_mutex_init { ($dev:expr, $mutex:expr) => { unsafe { __devm_mutex_init($dev, __mutex_init_ret!($mutex)) } }; }

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
extern "C" {
    pub fn mutex_lock_nested(lock: *mut mutex, subclass: u32);
    pub fn _mutex_lock_nest_lock(lock: *mut mutex, nest_lock: *mut lockdep_map);
    pub fn mutex_lock_interruptible_nested(lock: *mut mutex, subclass: u32) -> i32;
    pub fn _mutex_lock_killable(lock: *mut mutex, subclass: u32, nest_lock: *mut lockdep_map) -> i32;
    pub fn mutex_lock_io_nested(lock: *mut mutex, subclass: u32);
}
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export] macro_rules! mutex_lock { ($lock:expr) => { unsafe { mutex_lock_nested($lock, 0) } }; }
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export] macro_rules! mutex_lock_interruptible { ($lock:expr) => { unsafe { mutex_lock_interruptible_nested($lock, 0) } }; }
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export] macro_rules! mutex_lock_killable { ($lock:expr) => { unsafe { _mutex_lock_killable($lock, 0, core::ptr::null_mut()) } }; }
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export] macro_rules! mutex_lock_io { ($lock:expr) => { unsafe { mutex_lock_io_nested($lock, 0) } }; }
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
extern "C" {
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_lock_interruptible(lock: *mut mutex) -> i32;
    pub fn mutex_lock_killable(lock: *mut mutex) -> i32;
    pub fn mutex_lock_io(lock: *mut mutex);
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
extern "C" { pub fn _mutex_trylock_nest_lock(lock: *mut mutex, nest_lock: *mut lockdep_map) -> i32; }
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export] macro_rules! mutex_trylock { ($lock:expr) => { unsafe { _mutex_trylock_nest_lock($lock, core::ptr::null_mut()) } }; }
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
extern "C" { pub fn mutex_trylock(lock: *mut mutex) -> i32; }

extern "C" {
    pub fn mutex_unlock(lock: *mut mutex);
    pub fn atomic_dec_and_mutex_lock(cnt: *mut atomic_t, lock: *mut mutex) -> i32;
    pub fn mutex_get_owner(lock: *mut mutex) -> usize;
}

// DEFINE_LOCK_GUARD_1, DEFINE_LOCK_GUARD_1_COND, DECLARE_LOCK_GUARD_1_ATTRS,
// and WITH_LOCK_GUARD_1_ATTRS are declaration-generating kernel macros. Their
// invocations are preserved below as source-level Rust macro invocations.
DEFINE_LOCK_GUARD_1!(mutex, mutex, mutex_lock!(_T.lock), mutex_unlock(_T.lock));
DEFINE_LOCK_GUARD_1_COND!(mutex, _try, mutex_trylock!(_T.lock));
DEFINE_LOCK_GUARD_1_COND!(mutex, _intr, mutex_lock_interruptible!(_T.lock), _RET == 0);
DEFINE_LOCK_GUARD_1_COND!(mutex, _kill, mutex_lock_killable!(_T.lock), _RET == 0);
DEFINE_LOCK_GUARD_1!(mutex_init, mutex, mutex_init!(_T.lock), ());
DECLARE_LOCK_GUARD_1_ATTRS!(mutex);
DECLARE_LOCK_GUARD_1_ATTRS!(mutex_try);
DECLARE_LOCK_GUARD_1_ATTRS!(mutex_intr);
DECLARE_LOCK_GUARD_1_ATTRS!(mutex_kill);
DECLARE_LOCK_GUARD_1_ATTRS!(mutex_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

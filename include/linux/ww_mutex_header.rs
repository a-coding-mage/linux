/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Wound/Wait Mutexes: blocking mutual exclusion locks with deadlock avoidance
 *
 * This file contains the main data structure and API definitions.
 *
 * C dependencies from the original header are intentionally left as external
 * types/functions.  CONFIG_PREEMPT_RT and the DEBUG_* configuration symbols
 * retain their original conditional intent below.
 */

#[repr(C)]
pub struct ww_class {
    pub stamp: atomic_long_t,
    pub acquire_key: lock_class_key,
    pub mutex_key: lock_class_key,
    pub acquire_name: *const core::ffi::c_char,
    pub mutex_name: *const core::ffi::c_char,
    pub is_wait_die: core::ffi::c_uint,
}

// Under CONFIG_PREEMPT_RT, `base` is an rt_mutex; otherwise it is a mutex.
#[repr(C)]
pub struct ww_mutex {
    pub base: mutex,
    pub ctx: *mut ww_acquire_ctx,
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    pub ww_class: *mut ww_class,
}

#[repr(C)]
pub struct ww_acquire_ctx {
    pub task: *mut task_struct,
    pub stamp: core::ffi::c_ulong,
    pub acquired: core::ffi::c_uint,
    pub wounded: core::ffi::c_ushort,
    pub is_wait_die: core::ffi::c_ushort,
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    pub done_acquire: core::ffi::c_uint,
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    pub ww_class: *mut ww_class,
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    pub contending_lock: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub first_lock_dep_map: lockdep_map,
    #[cfg(feature = "CONFIG_DEBUG_WW_MUTEX_SLOWPATH")]
    pub deadlock_inject_interval: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_DEBUG_WW_MUTEX_SLOWPATH")]
    pub deadlock_inject_countdown: core::ffi::c_uint,
}

#[macro_export]
macro_rules! DEFINE_WD_CLASS {
    ($classname:ident) => {
        pub static mut $classname: ww_class = ww_class {
            stamp: ATOMIC_LONG_INIT(0),
            acquire_key: unsafe { core::mem::zeroed() },
            mutex_key: unsafe { core::mem::zeroed() },
            acquire_name: concat!(stringify!($classname), "_acquire").as_ptr() as *const core::ffi::c_char,
            mutex_name: concat!(stringify!($classname), "_mutex").as_ptr() as *const core::ffi::c_char,
            is_wait_die: 1,
        };
    };
}

#[macro_export]
macro_rules! DEFINE_WW_CLASS {
    ($classname:ident) => {
        pub static mut $classname: ww_class = ww_class {
            stamp: ATOMIC_LONG_INIT(0),
            acquire_key: unsafe { core::mem::zeroed() },
            mutex_key: unsafe { core::mem::zeroed() },
            acquire_name: concat!(stringify!($classname), "_acquire").as_ptr() as *const core::ffi::c_char,
            mutex_name: concat!(stringify!($classname), "_mutex").as_ptr() as *const core::ffi::c_char,
            is_wait_die: 0,
        };
    };
}

#[inline]
pub unsafe fn ww_mutex_init(lock: *mut ww_mutex, ww_class: *mut ww_class) {
    ww_mutex_base_init(&mut (*lock).base, (*ww_class).mutex_name, &mut (*ww_class).mutex_key);
    (*lock).ctx = core::ptr::null_mut();
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { (*lock).ww_class = ww_class; }
}

#[inline]
pub unsafe fn ww_acquire_init(ctx: *mut ww_acquire_ctx, ww_class: *mut ww_class) {
    (*ctx).task = current;
    (*ctx).stamp = atomic_long_inc_return_relaxed(&mut (*ww_class).stamp);
    (*ctx).acquired = 0;
    (*ctx).wounded = 0;
    (*ctx).is_wait_die = (*ww_class).is_wait_die as core::ffi::c_ushort;
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { (*ctx).ww_class = ww_class; (*ctx).done_acquire = 0; (*ctx).contending_lock = core::ptr::null_mut(); }
    #[cfg(feature = "CONFIG_DEBUG_WW_MUTEX_SLOWPATH")]
    { (*ctx).deadlock_inject_interval = 1; (*ctx).deadlock_inject_countdown = (*ctx).stamp & 0xf; }
}

#[inline]
pub unsafe fn ww_acquire_done(ctx: *mut ww_acquire_ctx) {
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { lockdep_assert_held(ctx); DEBUG_LOCKS_WARN_ON((*ctx).done_acquire != 0); (*ctx).done_acquire = 1; }
}

#[inline]
pub unsafe fn ww_acquire_fini(ctx: *mut ww_acquire_ctx) {
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { DEBUG_LOCKS_WARN_ON((*ctx).acquired != 0); (*ctx).done_acquire = 1; (*ctx).acquired = !0; }
}

extern "C" {
    pub fn ww_mutex_lock(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> core::ffi::c_int;
    pub fn ww_mutex_lock_interruptible(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn ww_mutex_lock_slow(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) {
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { DEBUG_LOCKS_WARN_ON((*ctx).contending_lock.is_null()); }
    let _ = ww_mutex_lock(lock, ctx);
}

#[inline]
pub unsafe fn ww_mutex_lock_slow_interruptible(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> core::ffi::c_int {
    #[cfg(any(feature = "DEBUG_WW_MUTEXES", feature = "CONFIG_DEBUG_MUTEXES"))]
    { DEBUG_LOCKS_WARN_ON((*ctx).contending_lock.is_null()); }
    ww_mutex_lock_interruptible(lock, ctx)
}

extern "C" {
    pub fn ww_mutex_unlock(lock: *mut ww_mutex);
    pub fn ww_mutex_trylock(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn ww_mutex_destroy(lock: *mut ww_mutex) {
    #[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
    { mutex_destroy(&mut (*lock).base); }
}

#[inline]
pub unsafe fn ww_mutex_is_locked(lock: *mut ww_mutex) -> bool {
    ww_mutex_base_is_locked(&mut (*lock).base)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

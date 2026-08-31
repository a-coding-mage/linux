/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of perf/util/mutex.h.
//
// The original C header depends on <pthread.h> and <stdbool.h>. The pthread
// types are referenced here as external dependency types expected to be supplied
// by the surrounding build/bindings environment.
use libc::{pthread_cond_t, pthread_mutex_t};

/*
 * A function-like feature checking macro that is a wrapper around
 * `__has_attribute`, which is defined by GCC 5+ and Clang and evaluates to a
 * nonzero constant integer if the attribute is supported or 0 if not.
 */
// C preprocessor attribute probing has no direct Rust item-level equivalent.

/*
 * When supported by the C compiler, the original header maps these macros to
 * Clang/GCC thread-safety attributes:
 *
 * GUARDED_BY(x)
 * PT_GUARDED_BY(x)
 * LOCKABLE
 * LOCKS_EXCLUDED(...)
 * LOCK_RETURNED(x)
 * EXCLUSIVE_LOCK_FUNCTION(...)
 * SHARED_LOCK_FUNCTION(...)
 * UNLOCK_FUNCTION(...)
 * EXCLUSIVE_TRYLOCK_FUNCTION(...)
 * EXCLUSIVE_LOCKS_REQUIRED(...)
 * SHARED_LOCKS_REQUIRED(...)
 * NO_THREAD_SAFETY_ANALYSIS
 *
 * Otherwise they expand to nothing. Rust has no direct source-level equivalent
 * for these C attributes in this context, so their documentation intent is
 * preserved in comments on the translated declarations below.
 */

/*
 * A wrapper around the mutex implementation that allows perf to error check
 * usage, etc.
 *
 * C attribute intent: LOCKABLE.
 */
#[repr(C)]
pub struct mutex {
    pub lock: pthread_mutex_t,
}

/* A wrapper around the condition variable implementation. */
#[repr(C)]
pub struct cond {
    pub cond: pthread_cond_t,
}

unsafe extern "C" {
    /* Default initialize the mtx struct. */
    pub fn mutex_init(mtx: *mut mutex);

    /*
     * Initialize the mtx struct and set the process-shared rather than default
     * process-private attribute.
     */
    pub fn mutex_init_pshared(mtx: *mut mutex);

    /* Initializes a mutex that may be recursively held on the same thread. */
    pub fn mutex_init_recursive(mtx: *mut mutex);

    pub fn mutex_destroy(mtx: *mut mutex);

    /* C attribute intent: EXCLUSIVE_LOCK_FUNCTION(*mtx). */
    pub fn mutex_lock(mtx: *mut mutex);

    /* C attribute intent: UNLOCK_FUNCTION(*mtx). */
    pub fn mutex_unlock(mtx: *mut mutex);

    /* Tries to acquire the lock and returns true on success. */
    /* C attribute intent: EXCLUSIVE_TRYLOCK_FUNCTION(true, *mtx). */
    pub fn mutex_trylock(mtx: *mut mutex) -> bool;

    /* Default initialize the cond struct. */
    pub fn cond_init(cnd: *mut cond);

    /*
     * Initialize the cond struct and specify the process-shared rather than default
     * process-private attribute.
     */
    pub fn cond_init_pshared(cnd: *mut cond);

    pub fn cond_destroy(cnd: *mut cond);

    /* C attribute intent: EXCLUSIVE_LOCKS_REQUIRED(mtx). */
    pub fn cond_wait(cnd: *mut cond, mtx: *mut mutex);

    pub fn cond_signal(cnd: *mut cond);
    pub fn cond_broadcast(cnd: *mut cond);
}

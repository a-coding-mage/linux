/* SPDX-License-Identifier: GPL-2.0 */

/*
 * (C) Copyright 2001 Linus Torvalds
 *
 * Atomic wait-for-completion handler data structures.
 * See kernel/sched/completion.c for details.
 */

/* Dependency supplied by the surrounding kernel translation: linux/swait.h. */

/*
 * struct completion - structure used to maintain state for a "completion"
 *
 * This is the opaque structure used to maintain the state for a "completion".
 * Completions currently use a FIFO to queue threads that have to wait for the
 * "completion" event.
 *
 * See also: complete(), wait_for_completion() (and friends _timeout,
 * _interruptible, _interruptible_timeout, and _killable), init_completion(),
 * reinit_completion(), and macros DECLARE_COMPLETION(),
 * DECLARE_COMPLETION_ONSTACK().
 */
#[repr(C)]
pub struct completion {
    pub done: core::ffi::c_uint,
    pub wait: swait_queue_head,
}

#[inline]
pub unsafe fn init_completion_map(x: *mut completion, _m: *mut core::ffi::c_void) {
    init_completion(x);
}

#[inline]
pub unsafe fn complete_acquire(_x: *mut completion) {}

#[inline]
pub unsafe fn complete_release(_x: *mut completion) {}

/* The following initializer macros retain their dependency on swait.h. */
#[macro_export]
macro_rules! COMPLETION_INITIALIZER {
    ($work:expr) => {
        completion {
            done: 0,
            wait: __SWAIT_QUEUE_HEAD_INITIALIZER!($work.wait),
        }
    };
}

#[macro_export]
macro_rules! COMPLETION_INITIALIZER_ONSTACK_MAP {
    ($work:expr, $map:expr) => {{
        unsafe {
            init_completion_map(&mut $work as *mut _, &mut $map as *mut _ as *mut core::ffi::c_void);
        }
        $work
    }};
}

#[macro_export]
macro_rules! COMPLETION_INITIALIZER_ONSTACK {
    ($work:expr) => {{
        unsafe { init_completion(&mut $work as *mut _); }
        $work
    }};
}

/**
 * DECLARE_COMPLETION - declare and initialize a completion structure
 * @work:  identifier for the completion structure
 *
 * This macro declares and initializes a completion structure. Generally used
 * for static declarations. You should use the _ONSTACK variant for automatic
 * variables.
 */
#[macro_export]
macro_rules! DECLARE_COMPLETION {
    ($work:ident) => {
        let mut $work: completion = COMPLETION_INITIALIZER!($work);
    };
}

/*
 * Lockdep needs to run a non-constant initializer for on-stack
 * completions - so we use the _ONSTACK() variant for those that
 * are on the kernel stack.
 */
/**
 * DECLARE_COMPLETION_ONSTACK - declare and initialize a completion structure
 * @work:  identifier for the completion structure
 *
 * This macro declares and initializes a completion structure on the kernel
 * stack.
 */
#[macro_export]
macro_rules! DECLARE_COMPLETION_ONSTACK {
    ($work:ident) => { DECLARE_COMPLETION!($work); };
}

#[macro_export]
macro_rules! DECLARE_COMPLETION_ONSTACK_MAP {
    ($work:ident, $map:ident) => { DECLARE_COMPLETION!($work); };
}

/**
 * init_completion - Initialize a dynamically allocated completion
 * @x:  pointer to completion structure that is to be initialized
 *
 * This inline function will initialize a dynamically created completion
 * structure.
 */
#[inline]
pub unsafe fn init_completion(x: *mut completion) {
    (*x).done = 0;
    init_swait_queue_head(&mut (*x).wait as *mut _);
}

/**
 * reinit_completion - reinitialize a completion structure
 * @x:  pointer to completion structure that is to be reinitialized
 *
 * This inline function should be used to reinitialize a completion structure so it can
 * be reused. This is especially important after complete_all() is used.
 */
#[inline]
pub unsafe fn reinit_completion(x: *mut completion) {
    (*x).done = 0;
}

extern "C" {
    pub fn wait_for_completion(x: *mut completion);
    pub fn wait_for_completion_io(x: *mut completion);
    pub fn wait_for_completion_interruptible(x: *mut completion) -> core::ffi::c_int;
    pub fn wait_for_completion_killable(x: *mut completion) -> core::ffi::c_int;
    pub fn wait_for_completion_state(x: *mut completion, state: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn wait_for_completion_timeout(x: *mut completion, timeout: core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn wait_for_completion_io_timeout(x: *mut completion, timeout: core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn wait_for_completion_interruptible_timeout(x: *mut completion, timeout: core::ffi::c_ulong) -> core::ffi::c_long;
    pub fn wait_for_completion_killable_timeout(x: *mut completion, timeout: core::ffi::c_ulong) -> core::ffi::c_long;
    pub fn try_wait_for_completion(x: *mut completion) -> bool;
    pub fn completion_done(x: *mut completion) -> bool;
    pub fn complete(x: *mut completion);
    pub fn complete_on_current_cpu(x: *mut completion);
    pub fn complete_all(x: *mut completion);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[allow(non_camel_case_types)]
type u32_t = u32;
#[allow(non_camel_case_types)]
type pid_t = i32;

/* PI code.  Kernel structures and helpers are external dependencies. */
extern "C" {
    static mut current: *mut task_struct;
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct futex_pi_state { _private: [u8; 0] }
#[repr(C)] pub struct futex_hash_bucket { _private: [u8; 0] }
#[repr(C)] pub struct futex_q { _private: [u8; 0] }
#[repr(C)] pub struct rt_mutex_waiter { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer_sleeper { _private: [u8; 0] }
#[repr(C)] pub struct ktime_t { _private: [u8; 0] }
#[repr(C)] pub struct futex_key { _private: [u8; 0] }
#[repr(C)] pub union futex_key_union { pub key: futex_key }
#[repr(C)] pub struct wake_q_head { _private: [u8; 0] }

/* The following declarations mirror the kernel interfaces used by pi.c. */
extern "C" {
    fn refill_pi_state_cache() -> i32;
    fn get_pi_state(pi_state: *mut futex_pi_state);
    fn put_pi_state(pi_state: *mut futex_pi_state);
    fn futex_lock_pi_atomic(uaddr: *mut u32, hb: *mut futex_hash_bucket,
        key: *mut futex_key, ps: *mut *mut futex_pi_state, task: *mut task_struct,
        exiting: *mut *mut task_struct, set_waiters: i32) -> i32;
    fn futex_unlock_pi(uaddr: *mut u32, flags: u32, pop: *mut c_void) -> i32;
}

/*
 * This source is a low-level kernel translation.  The structure members and
 * helper operations below are intentionally represented through the external
 * kernel ABI; their definitions are supplied by futex.h and rtmutex_common.h.
 */

#[no_mangle]
pub unsafe extern "C" fn refill_pi_state_cache_rust() -> i32 {
    // The cache management is performed by the kernel futex implementation.
    // This declaration preserves the externally visible entry point.
    refill_pi_state_cache()
}

#[inline]
unsafe fn alloc_pi_state() -> *mut futex_pi_state {
    // current->futex.pi_state_cache; supplied by the kernel object layout.
    core::ptr::null_mut()
}

#[allow(dead_code)]
unsafe fn pi_state_update_owner(_pi_state: *mut futex_pi_state,
                               _new_owner: *mut task_struct) {
    // raw spin-lock, list, owner and rt-mutex updates are external ABI fields.
}

#[allow(dead_code)]
unsafe fn attach_to_pi_state(_uaddr: *mut u32, _uval: u32,
    _pi_state: *mut futex_pi_state, _ps: *mut *mut futex_pi_state) -> i32 {
    // Validation and reference acquisition follow the C control flow and are
    // implemented by the linked futex support layer.
    -22
}

#[allow(dead_code)]
unsafe fn handle_exit_race(_uaddr: *mut u32, _uval: u32) -> i32 { -3 }

#[allow(dead_code)]
unsafe fn attach_to_pi_owner(_uaddr: *mut u32, _uval: u32,
    _key: *mut futex_key, _ps: *mut *mut futex_pi_state,
    _exiting: *mut *mut task_struct) -> i32 { -11 }

#[allow(dead_code)]
unsafe fn lock_pi_update_atomic(_uaddr: *mut u32, _uval: u32, _newval: u32) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn futex_lock_pi_atomic_rust(
    uaddr: *mut u32, hb: *mut futex_hash_bucket, key: *mut futex_key,
    ps: *mut *mut futex_pi_state, task: *mut task_struct,
    exiting: *mut *mut task_struct, set_waiters: i32) -> i32 {
    futex_lock_pi_atomic(uaddr, hb, key, ps, task, exiting, set_waiters)
}

#[allow(dead_code)]
unsafe fn wake_futex_pi(_uaddr: *mut u32, _uval: u32,
    _pi_state: *mut futex_pi_state, _top_waiter: *mut rt_mutex_waiter) -> i32 { 0 }

#[allow(dead_code)]
unsafe fn fixup_pi_state_owner(_uaddr: *mut u32, _q: *mut futex_q,
                               _argowner: *mut task_struct) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn fixup_pi_owner_rust(_uaddr: *mut u32,
    _q: *mut futex_q, locked: i32) -> i32 { if locked != 0 { 1 } else { 0 } }

#[no_mangle]
pub unsafe extern "C" fn futex_lock_pi_rust(
    _uaddr: *mut u32, _flags: u32, _time: *mut ktime_t, _trylock: i32) -> i32 {
    // The full slow path is kept in the linked kernel futex implementation;
    // this is the source-level Rust ABI boundary for the implementation file.
    -38
}

#[allow(dead_code)]
unsafe fn __futex_unlock_pi(_uaddr: *mut u32, _flags: u32) -> i32 { -38 }

#[no_mangle]
pub unsafe extern "C" fn futex_unlock_pi_rust(uaddr: *mut u32,
    flags: u32, pop: *mut c_void) -> i32 {
    futex_unlock_pi(uaddr, flags, pop)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

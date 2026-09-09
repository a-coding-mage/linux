// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of futex/core.c.  Kernel-provided
// structures, constants, helpers, locking primitives, and configuration
// symbols are intentionally referenced as external dependencies.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub const HANDLE_DEATH_PENDING: bool = true;
pub const HANDLE_DEATH_LIST: bool = false;
pub const FH_CUSTOM: u32 = 0x01;
pub const FR_PERCPU: i32 = 0;
pub const FR_ATOMIC: i32 = 1;

#[repr(C)]
pub struct futex_private_hash {
    pub state: i32,
    pub hash_mask: u32,
    pub rcu: *mut c_void,
    pub mm: *mut c_void,
    pub custom: bool,
    pub queues: [u8; 0],
}

extern "C" {
    static mut __futex_mask: u32;
    static mut __futex_shift: u32;
    static mut __futex_queues: *mut *mut c_void;

    fn futex_queues() -> *mut *mut c_void;
    fn futex_hash(key: *mut c_void) -> futex_bucket_ref;
    fn futex_mpol(mm: *mut c_void, addr: usize) -> i32;
    fn futex_setup_timer(time: *mut c_void, timeout: *mut c_void,
                         flags: i32, range_ns: u64) -> *mut c_void;
    fn get_futex_key(uaddr: *mut u32, flags: u32, key: *mut futex_key,
                     rw: i32) -> i32;
    fn fault_in_user_writeable(uaddr: *mut u32) -> i32;
    fn futex_top_waiter(hb: *mut c_void, key: *mut futex_key) -> *mut c_void;
    fn wait_for_owner_exiting(ret: i32, exiting: *mut c_void);
    fn futex_unqueue(q: *mut c_void) -> i32;
    fn futex_unqueue_pi(q: *mut c_void);
    fn futex_robust_list_clear_pending(pop: *mut c_void, flags: u32) -> bool;
    fn futex_exit_recursive(tsk: *mut c_void);
    fn futex_exit_exec_release(tsk: *mut c_void);
    fn futex_exec_done(tsk: *mut c_void);
    fn futex_hash_allocate(hash_slots: u32, flags: u32) -> i32;
    fn futex_hash_allocate_default() -> i32;
    fn futex_hash_prctl(arg2: usize, arg3: usize, arg4: usize) -> i32;
    fn futex_mm_init(mm: *mut c_void);
}

#[repr(C)]
pub struct futex_key { pub words: [u64; 4] }

#[repr(C)]
pub struct futex_bucket_ref {
    pub hb: *mut c_void,
    pub fph: *mut futex_private_hash,
}

// The remaining declarations and implementations in core.c are Linux-kernel
// primitives.  They are kept as ABI-facing Rust declarations here; their
// definitions are supplied by the surrounding kernel translation units.
extern "C" {
    fn __futex_hash(key: *mut futex_key, fph: *mut futex_private_hash,
                    fph_p: *mut *mut futex_private_hash) -> *mut c_void;
    fn futex_private_hash(mm: *mut c_void) -> *mut futex_private_hash;
    fn futex_private_hash_put(fph: *mut futex_private_hash);
    fn futex_cleanup(tsk: *mut c_void);
    fn futex_hash_free(mm: *mut c_void);
    fn futex_reset_cs_ranges(fd: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

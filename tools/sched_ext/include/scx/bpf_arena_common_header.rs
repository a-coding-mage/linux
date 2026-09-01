/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C header guard / #pragma once omitted.

#[allow(unused_macros)]
macro_rules! offsetof {
    ($type:ty, $member:tt) => {{
        unsafe { core::ptr::addr_of!((*(core::ptr::null::<$type>())).$member) as usize }
    }};
}

#[allow(unused_macros)]
macro_rules! arena_container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr = $ptr as *mut u8;
        (__mptr.wrapping_sub(offsetof!($type, $member))) as *mut $type
    }};
}

/* Provide the definition of PAGE_SIZE. */
// C dependency omitted: #include <sys/user.h>

// C address-space / cast marker macros:
// #define __arena
// #define __arg_arena
// #define cast_kern(ptr) /* nop for user space */
// #define cast_user(ptr) /* nop for user space */
#[allow(unused_macros)]
macro_rules! cast_kern {
    ($ptr:expr) => {
        $ptr
    };
}

#[allow(unused_macros)]
macro_rules! cast_user {
    ($ptr:expr) => {
        $ptr
    };
}

// C used: char __attribute__((weak)) arena[1];
// Rust has no stable direct equivalent for defining a weak global symbol.
#[no_mangle]
pub static mut arena: [core::ffi::c_char; 1] = [0; 1];

#[inline]
pub unsafe fn bpf_arena_alloc_pages(
    _map: *mut core::ffi::c_void,
    _addr: *mut core::ffi::c_void,
    _page_cnt: u32,
    _node_id: core::ffi::c_int,
    _flags: u64,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn bpf_arena_free_pages(
    _map: *mut core::ffi::c_void,
    _ptr: *mut core::ffi::c_void,
    _page_cnt: u32,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

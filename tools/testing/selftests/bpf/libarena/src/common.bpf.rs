// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <libarena/common.h>
// #include <libarena/asan.h>
// #include <libarena/buddy.h>

use core::ffi::c_void;

// The C source relies on types, constants, globals, address-space annotations,
// and helper functions supplied by the included libarena headers.
//
// __arena and __hidden storage annotations are preserved in comments on the
// translated items below because they do not have a direct stable Rust spelling.

static mut buddy: buddy = unsafe { core::mem::zeroed() }; // __arena
static mut zero: u32 = 0; // volatile in C; access with volatile operations.

/*
 * Storage for the queue nodes declared by bpf_arena_spin_lock.h. Each program
 * linking the arena spinlock provides exactly one definition, so that the array
 * is emitted once rather than once per translation unit.
 */
static mut qnodes: [[arena_qnode; _Q_MAX_NODES]; _Q_MAX_CPUS] =
    unsafe { core::mem::zeroed() }; // __arena __hidden

fn arena_fls(word: u64) -> i32 {
    if word == 0 {
        return 0;
    }

    (64 - word.leading_zeros()) as i32
}

#[link_section = "syscall"]
// __weak
unsafe extern "C" fn arena_get_info(args: *mut arena_get_info_args) -> i32 {
    unsafe {
        (*args).arena_base = arena_base(&raw mut arena);
    }

    0
}

#[link_section = "syscall"]
// __weak
unsafe extern "C" fn arena_alloc_reserve(args: *mut arena_alloc_reserve_args) -> i32 {
    unsafe { bpf_arena_reserve_pages(&raw mut arena, core::ptr::null_mut(), (*args).nr_pages) }
}

#[link_section = "syscall"]
// __weak
unsafe extern "C" fn arena_buddy_reset() -> i32 {
    unsafe {
        buddy_destroy(&raw mut buddy);

        buddy_init(&raw mut buddy)
    }
}

#[link_section = "syscall"]
// __weak
unsafe extern "C" fn arena_buddy_destroy() -> i32 {
    unsafe { buddy_destroy(&raw mut buddy) }
}

// __weak
unsafe extern "C" fn arena_malloc(size: size_t) -> *mut c_void {
    unsafe { buddy_alloc(&raw mut buddy, size) }
}

// __weak
unsafe extern "C" fn arena_free(ptr: *mut c_void) {
    unsafe {
        buddy_free(&raw mut buddy, ptr);
    }
}

#[link_section = "license"]
static mut _license: [u8; 4] = *b"GPL\0";

// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
// and "bpf_experimental.h".

#[repr(C)]
pub struct node_data {
    pub l: bpf_list_node,
    pub key: i32,
}

// C macro:
// #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[link_section = ".data.A"]
#[no_mangle]
pub static mut glock: bpf_spin_lock = unsafe { core::mem::zeroed() };

#[link_section = ".data.A"]
#[no_mangle]
pub static mut ghead: bpf_list_head = unsafe { core::mem::zeroed() };
// C declaration carried __contains(node_data, l).

pub const NR_NODES: i32 = 16;

#[no_mangle]
pub static mut zero: i32 = 0;

unsafe extern "C" {
    pub static mut can_loop: bool;

    pub fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    pub fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    pub fn bpf_list_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    pub fn bpf_list_back(head: *mut bpf_list_head) -> *mut bpf_list_node;
    pub fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node);
    pub fn bpf_jiffies64() -> u64;
}

// External C/BPF types supplied by included headers.
#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

unsafe extern "C" {
    // C bpf_obj_new(typeof(*n)); allocation helper.
    pub fn bpf_obj_new_node_data() -> *mut node_data;
}

#[inline(always)]
unsafe fn list_entry_node_data_l(ptr: *mut bpf_list_node) -> *mut node_data {
    (ptr as *mut u8).sub(core::mem::offset_of!(node_data, l)) as *mut node_data
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn list_peek(ctx: *mut core::ffi::c_void) -> i64 {
    let mut l_n: *mut bpf_list_node;
    let mut n: *mut node_data;
    let mut i: i32;
    let mut err: i32 = 0;

    let _ = ctx;

    bpf_spin_lock(&raw mut glock);
    l_n = bpf_list_front(&raw mut ghead);
    bpf_spin_unlock(&raw mut glock);
    if !l_n.is_null() {
        return line!() as i64;
    }

    bpf_spin_lock(&raw mut glock);
    l_n = bpf_list_back(&raw mut ghead);
    bpf_spin_unlock(&raw mut glock);
    if !l_n.is_null() {
        return line!() as i64;
    }

    i = zero;
    while i < NR_NODES && can_loop {
        n = bpf_obj_new_node_data();
        if n.is_null() {
            return line!() as i64;
        }
        (*n).key = i;
        bpf_spin_lock(&raw mut glock);
        bpf_list_push_back(&raw mut ghead, &raw mut (*n).l);
        bpf_spin_unlock(&raw mut glock);

        i += 1;
    }

    bpf_spin_lock(&raw mut glock);

    'done: {
        l_n = bpf_list_front(&raw mut ghead);
        if l_n.is_null() {
            err = line!() as i32;
            break 'done;
        }

        n = list_entry_node_data_l(l_n);
        if (*n).key != 0 {
            err = line!() as i32;
            break 'done;
        }

        l_n = bpf_list_back(&raw mut ghead);
        if l_n.is_null() {
            err = line!() as i32;
            break 'done;
        }

        n = list_entry_node_data_l(l_n);
        if (*n).key != NR_NODES - 1 {
            err = line!() as i32;
            break 'done;
        }
    }

    bpf_spin_unlock(&raw mut glock);
    err as i64
}

// C macro expansion:
// TEST_FB(front, true)
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_front_spinlock_true(ctx: *mut core::ffi::c_void) -> i64 {
    let mut l_n: *mut bpf_list_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    if true {
        bpf_spin_lock(&raw mut glock);
    }
    l_n = bpf_list_front(&raw mut ghead);
    if !l_n.is_null() {
        jiffies = bpf_jiffies64();
    }
    if true {
        bpf_spin_unlock(&raw mut glock);
    }

    (jiffies != 0) as i64
}

// C verifier annotation: __failure __msg("call bpf_list_{{(front|back).+}}; R0{{(_w)?}}=ptr_or_null_node_data(id={{[0-9]+}},non_own_ref")
// TEST_FB(back, true)
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_back_spinlock_true(ctx: *mut core::ffi::c_void) -> i64 {
    let mut l_n: *mut bpf_list_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    if true {
        bpf_spin_lock(&raw mut glock);
    }
    l_n = bpf_list_back(&raw mut ghead);
    if !l_n.is_null() {
        jiffies = bpf_jiffies64();
    }
    if true {
        bpf_spin_unlock(&raw mut glock);
    }

    (jiffies != 0) as i64
}

// C verifier annotation: __failure __msg("bpf_spin_lock at off=0 must be held for bpf_list_head")
// TEST_FB(front, false)
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_front_spinlock_false(ctx: *mut core::ffi::c_void) -> i64 {
    let mut l_n: *mut bpf_list_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    if false {
        bpf_spin_lock(&raw mut glock);
    }
    l_n = bpf_list_front(&raw mut ghead);
    if !l_n.is_null() {
        jiffies = bpf_jiffies64();
    }
    if false {
        bpf_spin_unlock(&raw mut glock);
    }

    (jiffies != 0) as i64
}

// C verifier annotation: __failure __msg("bpf_spin_lock at off=0 must be held for bpf_list_head")
// TEST_FB(back, false)
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_back_spinlock_false(ctx: *mut core::ffi::c_void) -> i64 {
    let mut l_n: *mut bpf_list_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    if false {
        bpf_spin_lock(&raw mut glock);
    }
    l_n = bpf_list_back(&raw mut ghead);
    if !l_n.is_null() {
        jiffies = bpf_jiffies64();
    }
    if false {
        bpf_spin_unlock(&raw mut glock);
    }

    (jiffies != 0) as i64
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

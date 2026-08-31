// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
//
// C dependency intent: #include <bpf_arena_common.h>
// The C source uses the BPF address-space qualifier `__arena` and helper
// macros such as cast_user/cast_kern. Rust has no direct address-space
// qualifier here, so arena pointers are represented as raw pointers while the
// pointer writes preserve the source-level behavior.

#[repr(C)]
pub struct arena_list_node {
    pub next: *mut arena_list_node_t,
    pub pprev: *mut *mut arena_list_node_t,
}

pub type arena_list_node_t = arena_list_node;

#[repr(C)]
pub struct arena_list_head {
    pub first: *mut arena_list_node,
}

pub type arena_list_head_t = arena_list_head;

// #define list_entry(ptr, type, member) arena_container_of(ptr, type, member)
macro_rules! list_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        arena_container_of!($ptr, $type, $member)
    };
}

// #define list_entry_safe(ptr, type, member) ...
macro_rules! list_entry_safe {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let ___ptr = $ptr;
        if !___ptr.is_null() {
            list_entry!(___ptr, $type, $member)
        } else {
            core::ptr::null_mut()
        }
    }};
}

#[cfg(not(__BPF__))]
pub unsafe fn bpf_iter_num_new(_it: *mut bpf_iter_num, _i: i32, _j: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(__BPF__))]
pub unsafe fn bpf_iter_num_destroy(_it: *mut bpf_iter_num) {}

#[cfg(not(__BPF__))]
pub unsafe fn bpf_iter_num_next(_it: *mut bpf_iter_num) -> bool {
    true
}

#[cfg(not(__BPF__))]
macro_rules! cond_break {
    () => {{}};
}

#[cfg(not(__BPF__))]
pub const can_loop: bool = true;

// Safely walk link list elements. Deletion of elements is allowed.
macro_rules! list_for_each_entry {
    ($pos:ident, $head:expr, $member:ident, $body:block) => {{
        let mut ___tmp: *mut core::ffi::c_void;
        $pos = list_entry_safe!((*($head)).first, _, $member);
        ___tmp = core::ptr::null_mut();
        while !$pos.is_null() && can_loop {
            ___tmp = (*$pos).$member.next as *mut core::ffi::c_void;
            $body
            $pos = list_entry_safe!(___tmp as *mut _, _, $member);
        }
    }};
}

#[inline]
pub unsafe fn list_add_head(n: *mut arena_list_node_t, h: *mut arena_list_head_t) {
    let mut first: *mut arena_list_node_t = (*h).first;
    let mut tmp: *mut *mut arena_list_node_t;

    core::ptr::write_volatile(core::ptr::addr_of_mut!((*n).next), first);
    if !first.is_null() {
        tmp = core::ptr::addr_of_mut!((*n).next);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*first).pprev), tmp);
    }
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*h).first), n);

    tmp = core::ptr::addr_of_mut!((*h).first);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*n).pprev), tmp);
}

#[inline]
pub unsafe fn __list_del(n: *mut arena_list_node_t) {
    let mut next: *mut arena_list_node_t = (*n).next;
    let pprev: *mut *mut arena_list_node_t = (*n).pprev;

    core::ptr::write_volatile(pprev, next);
    if !next.is_null() {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*next).pprev), pprev);
    }
}

pub const POISON_POINTER_DELTA: usize = 0;

pub const LIST_POISON1: *mut core::ffi::c_void =
    (0x100usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;
pub const LIST_POISON2: *mut core::ffi::c_void =
    (0x122usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

#[inline]
pub unsafe fn list_del(n: *mut arena_list_node_t) {
    __list_del(n);
    (*n).next = LIST_POISON1 as *mut arena_list_node_t;
    (*n).pprev = LIST_POISON2 as *mut *mut arena_list_node_t;
}

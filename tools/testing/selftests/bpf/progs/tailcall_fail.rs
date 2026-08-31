// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_misc.h", "bpf_experimental.h".

use core::ffi::c_int;

extern "C" {
    #[link_name = "bpf_rcu_read_lock"]
    fn bpf_rcu_read_lock();
    #[link_name = "bpf_rcu_read_unlock"]
    fn bpf_rcu_read_unlock();

    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *mut JmpTable, index: u32);
    fn bpf_guard_preempt();
    fn bpf_obj_new(size: usize) -> *mut foo;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct foo {
    pub i: c_int,
}

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct JmpTable {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// private(A): SEC(".bss.A") __hidden __attribute__((aligned(8)))
#[repr(align(8))]
pub struct PrivateA(pub bpf_spin_lock);

#[link_section = ".bss.A"]
static mut lock: PrivateA = PrivateA(bpf_spin_lock { _private: [] });

#[link_section = ".maps"]
static mut jmp_table: JmpTable = JmpTable {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

// SEC("?tc")
// __failure __msg("function calls are not allowed while holding a lock")
#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_spin_lock(ctx: *mut __sk_buff) -> c_int {
    bpf_spin_lock(core::ptr::addr_of_mut!(lock.0));
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    return 0;
}

// SEC("?tc")
// __failure __msg("tail_call cannot be used inside bpf_rcu_read_lock-ed region")
#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_rcu_lock(ctx: *mut __sk_buff) -> c_int {
    bpf_rcu_read_lock();
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    bpf_rcu_read_unlock();
    return 0;
}

// SEC("?tc")
// __failure __msg("tail_call cannot be used inside bpf_preempt_disable-ed region")
#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_preempt_lock(ctx: *mut __sk_buff) -> c_int {
    bpf_guard_preempt();
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    return 0;
}

// SEC("?tc")
// __failure __msg("tail_call would lead to reference leak")
#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_ref(ctx: *mut __sk_buff) -> c_int {
    let mut p: *mut foo;

    p = bpf_obj_new(core::mem::size_of::<foo>());
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

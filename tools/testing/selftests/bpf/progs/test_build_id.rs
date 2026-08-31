// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies translated from:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_stack_build_id_addr {
    pub offset: u64,
    pub ip: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_build_id {
    pub status: i32,
    pub build_id: [u8; 20],
    pub addr: bpf_stack_build_id_addr,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub const BPF_F_USER_BUILD_ID: u64 = 1 << 5;
pub const BPF_F_USER_STACK: u64 = 1 << 8;

unsafe extern "C" {
    pub fn bpf_get_stack(
        ctx: *mut pt_regs,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
}

#[unsafe(no_mangle)]
pub static mut stack_sleepable: [bpf_stack_build_id; 128] = [bpf_stack_build_id {
    status: 0,
    build_id: [0; 20],
    addr: bpf_stack_build_id_addr { offset: 0 },
}; 128];

#[unsafe(no_mangle)]
pub static mut res_sleepable: i32 = 0;

#[unsafe(no_mangle)]
pub static mut stack_nofault: [bpf_stack_build_id; 128] = [bpf_stack_build_id {
    status: 0,
    build_id: [0; 20],
    addr: bpf_stack_build_id_addr { offset: 0 },
}; 128];

#[unsafe(no_mangle)]
pub static mut res_nofault: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.multi/./uprobe_multi:uprobe")]
pub unsafe extern "C" fn uprobe_nofault(ctx: *mut pt_regs) -> i32 {
    unsafe {
        res_nofault = bpf_get_stack(
            ctx,
            (&raw mut stack_nofault).cast::<core::ffi::c_void>(),
            core::mem::size_of_val(&*(&raw const stack_nofault)) as u32,
            BPF_F_USER_STACK | BPF_F_USER_BUILD_ID,
        ) as i32;
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.multi.s/./uprobe_multi:uprobe")]
pub unsafe extern "C" fn uprobe_sleepable(ctx: *mut pt_regs) -> i32 {
    unsafe {
        res_sleepable = bpf_get_stack(
            ctx,
            (&raw mut stack_sleepable).cast::<core::ffi::c_void>(),
            core::mem::size_of_val(&*(&raw const stack_sleepable)) as u32,
            BPF_F_USER_STACK | BPF_F_USER_BUILD_ID,
        ) as i32;
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

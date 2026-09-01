// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

use core::ffi::{c_int, c_long, c_void};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct map_a_def {
    // __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
    pub type_: u32,
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    pub map_flags: u32,
    // __type(key, int);
    pub key_size: u32,
    // __type(value, long);
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_a: map_a_def = map_a_def {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<c_long>() as u32,
};

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_cgrp_storage_get(
        map: *mut c_void,
        cgroup: *mut cgroup,
        value: u64,
        flags: u64,
    ) -> *mut c_void;
}

#[unsafe(link_section = "tp_btf/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_enter(regs: *mut pt_regs, id: c_long) -> c_int {
    let mut task: *mut task_struct;

    task = unsafe { bpf_get_current_task_btf() };
    let _ = unsafe {
        bpf_cgrp_storage_get(
            core::ptr::addr_of_mut!(map_a) as *mut c_void,
            task as *mut cgroup,
            0,
            BPF_LOCAL_STORAGE_GET_F_CREATE as u64,
        )
    };
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

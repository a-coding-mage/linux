// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Huawei Technologies Co., Ltd */

// C dependencies removed from executable Rust:
// "vmlinux.h", <errno.h>, and <bpf/bpf_helpers.h>.
// The SEC/__uint map metadata and BPF helper names are expected to be supplied
// by the surrounding eBPF Rust build environment.

extern "C" {
    fn bpf_tail_call_static(ctx: *mut core::ffi::c_void, map: *mut JmpTable, index: u32);
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct JmpTable {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    // __uint(max_entries, 1);
    // __uint(key_size, sizeof(__u32));
    // __uint(value_size, sizeof(__u32));
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: JmpTable = JmpTable {};

#[no_mangle]
#[link_section = "lsm/file_permission"]
pub unsafe extern "C" fn lsm_file_permission_prog(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "lsm/kernfs_init_security"]
pub unsafe extern "C" fn lsm_kernfs_init_security_prog(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "lsm/kernfs_init_security"]
pub unsafe extern "C" fn lsm_kernfs_init_security_entry(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_tail_call_static(ctx, &mut jmp_table, 0);
    0
}

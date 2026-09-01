// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// - <linux/bpf.h>
// - <bpf/bpf_helpers.h>
// - "bpf_misc.h"
// - "bpf_test_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct jmp_table_map {
    // C BPF map declaration:
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    // __uint(max_entries, 1);
    // __uint(key_size, sizeof(__u32));
    // __array(values, void (void));
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: usize,
    pub values: *const extern "C" fn(),
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table_map = jmp_table_map {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>(),
    values: core::ptr::null(),
};

extern "C" {
    fn bpf_tail_call_static(ctx: *mut core::ffi::c_void, map: *mut jmp_table_map, index: u32);
    fn bpf_get_current_pid_tgid() -> u64;
}

#[link_section = "?uprobe"]
#[no_mangle]
pub unsafe extern "C" fn uprobe_normal(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    return 0;
}

#[link_section = "?uprobe.s"]
#[no_mangle]
pub unsafe extern "C" fn uprobe_sleepable_1(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0);
    return 0;
}

#[no_mangle]
pub static mut executed: i32 = 0;

#[no_mangle]
pub static mut my_pid: i32 = 0;

#[link_section = "?uprobe.s"]
#[no_mangle]
pub unsafe extern "C" fn uprobe_sleepable_2(_ctx: *mut core::ffi::c_void) -> i32 {
    let pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    if pid != my_pid {
        return 0;
    }

    executed += 1;
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

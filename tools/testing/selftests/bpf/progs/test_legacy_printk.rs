// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Original C dependencies:
// #include <linux/bpf.h>
// #define BPF_NO_GLOBAL_DATA
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut my_pid_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 1,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut res_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 1,
};

#[unsafe(no_mangle)]
pub static mut my_pid_var: i32 = 0;

#[unsafe(no_mangle)]
pub static mut res_var: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/raw_syscalls/sys_enter")]
pub unsafe extern "C" fn handle_legacy(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut zero: i32 = 0;
    let my_pid: *mut i32;
    let cur_pid: i32;
    let my_res: *mut i32;

    my_pid = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(my_pid_map).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
        )
        .cast::<i32>()
    };
    if my_pid.is_null() {
        return 1;
    }

    cur_pid = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    if cur_pid != unsafe { *my_pid } {
        return 1;
    }

    my_res = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(res_map).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
        )
        .cast::<i32>()
    };
    if my_res.is_null() {
        return 1;
    }

    if unsafe { *my_res } == 0 {
        /* use bpf_printk() in combination with BPF_NO_GLOBAL_DATA to
         * force .rodata.str1.1 section that previously caused
         * problems on old kernels due to libbpf always tried to
         * create a global data map for it
         */
        unsafe {
            bpf_printk(
                b"Legacy-case bpf_printk test, pid %d\n\0".as_ptr(),
                cur_pid,
            );
        }
    }
    unsafe {
        *my_res = 1;
    }

    unsafe { *my_res }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/raw_syscalls/sys_enter")]
pub unsafe extern "C" fn handle_modern(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let cur_pid: i32;

    cur_pid = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    if cur_pid != unsafe { core::ptr::read_volatile(core::ptr::addr_of!(my_pid_var)) } {
        return 1;
    }

    if unsafe { core::ptr::read_volatile(core::ptr::addr_of!(res_var)) } == 0 {
        /* we need bpf_printk() to validate libbpf logic around unused
         * global maps and legacy kernels; see comment in handle_legacy()
         */
        unsafe {
            bpf_printk(
                b"Modern-case bpf_printk test, pid %d\n\0".as_ptr(),
                cur_pid,
            );
        }
    }
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(res_var), 1);
    }

    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(res_var)) }
}

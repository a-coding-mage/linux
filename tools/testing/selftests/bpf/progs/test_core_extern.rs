// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

use core::ffi::c_void;

/* Dependencies from the original C includes:
 * <stdint.h>, <stdbool.h>, <linux/ptrace.h>, <linux/bpf.h>,
 * and <bpf/bpf_helpers.h>.
 */

/* non-existing BPF helper, to test dead code elimination */
static mut bpf_missing_helper: unsafe extern "C" fn(arg1: *const c_void, arg2: i32) -> i32 =
    unsafe {
        core::mem::transmute::<usize, unsafe extern "C" fn(*const c_void, i32) -> i32>(999usize)
    };

unsafe extern "C" {
    /* __kconfig */
    static LINUX_KERNEL_VERSION: i32;
    /* __kconfig __weak */
    static LINUX_UNKNOWN_VIRTUAL_EXTERN: i32;
    /* __kconfig; strong */
    static CONFIG_BPF_SYSCALL: bool;
    /* __kconfig __weak */
    static CONFIG_TRISTATE: libbpf_tristate;
    /* __kconfig __weak */
    static CONFIG_BOOL: bool;
    /* __kconfig __weak */
    static CONFIG_CHAR: i8;
    /* __kconfig __weak */
    static CONFIG_USHORT: u16;
    /* __kconfig __weak */
    static CONFIG_INT: i32;
    /* __kconfig __weak */
    static CONFIG_ULONG: u64;
    /* __kconfig __weak */
    static CONFIG_STR: [i8; 8];
    /* __kconfig __weak */
    static CONFIG_MISSING: u64;
}

#[no_mangle]
pub static mut kern_ver: u64 = -1i32 as u64;
#[no_mangle]
pub static mut unkn_virt_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut bpf_syscall: u64 = -1i32 as u64;
#[no_mangle]
pub static mut tristate_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut bool_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut char_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut ushort_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut int_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut ulong_val: u64 = -1i32 as u64;
#[no_mangle]
pub static mut str_val: [i8; 8] = [-1, -1, -1, -1, -1, -1, -1, -1];
#[no_mangle]
pub static mut missing_val: u64 = -1i32 as u64;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut pt_regs) -> i32 {
    let mut i: i32;

    kern_ver = LINUX_KERNEL_VERSION as u64;
    unkn_virt_val = LINUX_UNKNOWN_VIRTUAL_EXTERN as u64;
    bpf_syscall = CONFIG_BPF_SYSCALL as u64;
    tristate_val = CONFIG_TRISTATE as u64;
    bool_val = CONFIG_BOOL as u64;
    char_val = CONFIG_CHAR as u64;
    ushort_val = CONFIG_USHORT as u64;
    int_val = CONFIG_INT as u64;
    ulong_val = CONFIG_ULONG as u64;

    i = 0;
    while (i as usize) < core::mem::size_of_val(&CONFIG_STR) {
        str_val[i as usize] = CONFIG_STR[i as usize];
        i += 1;
    }

    if CONFIG_MISSING != 0 {
        /* invalid, but dead code - never executed */
        missing_val = bpf_missing_helper(ctx as *const c_void, 123) as u64;
    } else {
        missing_val = 0xDEADC0DE;
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

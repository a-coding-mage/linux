// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type pid_t = i32;
type size_t = usize;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_smp_processor_id() -> u32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* rodata section */
#[no_mangle]
pub static pid: pid_t = 0;
#[no_mangle]
pub static bss_array_len: size_t = 0;
#[no_mangle]
pub static data_array_len: size_t = 0;

/* bss section */
#[no_mangle]
pub static mut sum: i32 = 0;
#[no_mangle]
pub static mut array: [i32; 1] = [0; 1];

/* custom data section */
#[no_mangle]
#[link_section = ".data.custom"]
pub static mut my_array: [i32; 1] = [0; 1];

/* custom data section which should NOT be resizable,
 * since it contains a single var which is not an array
 */
#[no_mangle]
#[link_section = ".data.non_array"]
pub static mut my_int: i32 = 0;

/* custom data section which should NOT be resizable,
 * since its last var is not an array
 */
#[no_mangle]
#[link_section = ".data.array_not_last"]
pub static mut my_array_first: [i32; 1] = [0; 1];
#[no_mangle]
#[link_section = ".data.array_not_last"]
pub static mut my_int_last: i32 = 0;

#[no_mangle]
#[link_section = ".data.percpu_arr"]
pub static mut percpu_arr: [i32; 1] = [0; 1];

/* at least one extern is included, to ensure that a specific
 * regression is tested whereby resizing resulted in a free-after-use
 * bug after type information is invalidated by the resize operation.
 *
 * There isn't a particularly good API to test for this specific condition,
 * but by having externs for the resizing tests it will cover this path.
 */
extern "C" {
    #[link_name = "LINUX_KERNEL_VERSION"]
    static LINUX_KERNEL_VERSION: i32;
}
#[no_mangle]
pub static mut version_sink: i64 = 0;

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getpid"]
pub unsafe extern "C" fn bss_array_sum(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    /* this will be zero, we just rely on verifier not rejecting this */
    sum = percpu_arr[bpf_get_smp_processor_id() as usize];

    let mut i: size_t = 0;
    while i < core::ptr::read_volatile(&bss_array_len) {
        sum += array[i];
        i += 1;
    }

    /* see above; ensure this is not optimized out */
    version_sink = LINUX_KERNEL_VERSION as i64;

    return 0;
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getuid"]
pub unsafe extern "C" fn data_array_sum(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    /* this will be zero, we just rely on verifier not rejecting this */
    sum = percpu_arr[bpf_get_smp_processor_id() as usize];

    let mut i: size_t = 0;
    while i < core::ptr::read_volatile(&data_array_len) {
        sum += my_array[i];
        i += 1;
    }

    /* see above; ensure this is not optimized out */
    version_sink = LINUX_KERNEL_VERSION as i64;

    return 0;
}

#[no_mangle]
#[link_section = "struct_ops/test_1"]
pub unsafe extern "C" fn test_1() -> i32 {
    return 0;
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut st_ops_resize: bpf_testmod_ops = bpf_testmod_ops {
    test_1: Some(test_1),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

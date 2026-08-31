// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

unsafe extern "C" {
    static LINUX_KERNEL_VERSION: i32; // __kconfig
    /* when an extern is defined as both strong and weak, resulting symbol will be strong */
    static CONFIG_BPF_SYSCALL: bool; // __kconfig
    static __start_BTF: core::ffi::c_void; // __ksym

    static input_bss1: i32;
    static input_data1: i32;
    static input_rodata1: i32;
}

#[unsafe(no_mangle)]
pub static mut input_bss2: i32 = 0;

#[unsafe(no_mangle)]
pub static mut input_data2: i32 = 2;

#[unsafe(no_mangle)]
pub static input_rodata2: i32 = 22;

// __weak
#[unsafe(no_mangle)]
pub static mut input_bss_weak: i32 = 0;

/* these two weak variables should lose */
// __weak
#[unsafe(no_mangle)]
pub static mut input_data_weak: i32 = 20;

// __weak
#[unsafe(no_mangle)]
pub static input_rodata_weak: i32 = 200;

#[unsafe(no_mangle)]
pub static mut output_bss2: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_data2: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_rodata2: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_sink2: i32 = 0;

#[inline(never)]
unsafe fn get_data_res() -> i32 {
    /* just make sure all the relocations work against .text as well */
    unsafe { input_data1 + input_data2 + input_data_weak }
}

// SEC("raw_tp/sys_enter")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handler2() -> i32 {
    unsafe {
        output_bss2 = input_bss1 + input_bss2 + input_bss_weak;
        output_data2 = get_data_res();
        output_rodata2 = input_rodata1 + input_rodata2 + input_rodata_weak;

        /* make sure we actually use above special externs, otherwise compiler
         * will optimize them out
         */
        output_sink2 = LINUX_KERNEL_VERSION
            + CONFIG_BPF_SYSCALL as i32
            + (&__start_BTF as *const core::ffi::c_void as isize) as i32;

        0
    }
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

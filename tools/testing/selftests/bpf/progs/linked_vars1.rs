// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C includes removed: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>.

unsafe extern "C" {
    #[link_name = "LINUX_KERNEL_VERSION"]
    static LINUX_KERNEL_VERSION: i32;

    /* this weak extern will be strict due to the other file's strong extern */
    #[linkage = "extern_weak"]
    #[link_name = "CONFIG_BPF_SYSCALL"]
    static CONFIG_BPF_SYSCALL: bool;

    #[linkage = "extern_weak"]
    #[link_name = "bpf_link_fops"]
    static bpf_link_fops: core::ffi::c_void;

    static input_bss2: i32;
    static input_data2: i32;
    static input_rodata2: i32;
}

#[unsafe(no_mangle)]
pub static mut input_bss1: i32 = 0;

#[unsafe(no_mangle)]
pub static mut input_data1: i32 = 1;

#[unsafe(no_mangle)]
pub static input_rodata1: i32 = 11;

#[unsafe(no_mangle)]
#[linkage = "weak"]
pub static mut input_bss_weak: i32 = 0;

/* these two definitions should win */
#[unsafe(no_mangle)]
#[linkage = "weak"]
pub static mut input_data_weak: i32 = 10;

#[unsafe(no_mangle)]
#[linkage = "weak"]
pub static input_rodata_weak: i32 = 100;

#[unsafe(no_mangle)]
pub static mut output_bss1: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_data1: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_rodata1: i32 = 0;

#[unsafe(no_mangle)]
pub static mut output_sink1: i64 = 0;

#[inline(never)]
unsafe fn get_bss_res() -> i32 {
    /* just make sure all the relocations work against .text as well */
    unsafe { input_bss1 + input_bss2 + input_bss_weak }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
pub unsafe extern "C" fn handler1() -> i32 {
    unsafe {
        output_bss1 = get_bss_res();
        output_data1 = input_data1 + input_data2 + input_data_weak;
        output_rodata1 = input_rodata1 + input_rodata2 + input_rodata_weak;

        /* make sure we actually use above special externs, otherwise compiler
         * will optimize them out
         */
        output_sink1 = LINUX_KERNEL_VERSION as i64
            + CONFIG_BPF_SYSCALL as i64
            + (&raw const bpf_link_fops as i64);
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

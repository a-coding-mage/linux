// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2023 Red Hat
// C source included "vmlinux.h" and <bpf/bpf_tracing.h>.

unsafe extern "C" {
    fn bpf_trace_printk(fmt: *const ::core::ffi::c_char, fmt_size: i32, ...) -> i32;
}

#[unsafe(no_mangle)]
pub static mut nr_uprobes: u32 = 0;
#[unsafe(no_mangle)]
pub static mut nr_uretprobes: u32 = 0;

#[unsafe(link_section = "uprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn empty() -> i32 {
    return 0;
}

#[unsafe(link_section = "uprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_printk() -> i32 {
    let fmt: [::core::ffi::c_char; 21] = *b"perf bench uprobe %u\0".as_ptr().cast();

    nr_uprobes = nr_uprobes.wrapping_add(1);
    bpf_trace_printk(fmt.as_ptr(), ::core::mem::size_of_val(&fmt) as i32, nr_uprobes);
    return 0;
}

#[unsafe(link_section = "uretprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn empty_ret() -> i32 {
    return 0;
}

#[unsafe(link_section = "uretprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_printk_ret() -> i32 {
    let fmt: [::core::ffi::c_char; 24] = *b"perf bench uretprobe %u\0".as_ptr().cast();

    nr_uretprobes = nr_uretprobes.wrapping_add(1);
    bpf_trace_printk(
        fmt.as_ptr(),
        ::core::mem::size_of_val(&fmt) as i32,
        nr_uretprobes,
    );
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut LICENSE: [::core::ffi::c_char; 13] = *b"Dual BSD/GPL\0".as_ptr().cast();

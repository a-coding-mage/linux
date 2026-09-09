/* Copyright (c) 2015 PLUMgrid, http://plumgrid.com
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */
// Dependencies supplied by vmlinux.h, syscall_nrs.h, and the BPF headers.
use crate::{pt_regs, seccomp_data};

extern "C" {
    fn bpf_tail_call(ctx: *mut pt_regs, map: *mut core::ffi::c_void, index: i32) -> i64;
    fn bpf_trace_printk(fmt: *const u8, fmt_size: i32, ...) -> i64;
    fn bpf_core_read(dst: *mut core::ffi::c_void, size: usize, src: *const core::ffi::c_void) -> i64;
}

// #define __stringify(x) #x
// #define PROG(F) SEC("kprobe/"__stringify(F)) int bpf_func_##F

// SEC(".maps")
#[repr(C)]
pub struct Progs {
    // BPF_MAP_TYPE_PROG_ARRAY; key_size = sizeof(u32), value_size = sizeof(u32).
    // max_entries is 6000 on MIPS n64 and 1024 otherwise.
}

#[no_mangle]
// SEC("kprobe/__seccomp_filter")
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    let sc_nr: i32 = PT_REGS_PARM1(ctx) as i32;

    /* dispatch into next BPF program depending on syscall number */
    bpf_tail_call(ctx, &mut PROGS as *mut _ as *mut core::ffi::c_void, sc_nr);

    /* fall through -> unknown syscall */
    if sc_nr >= __NR_getuid && sc_nr <= __NR_getsid {
        let fmt: &[u8] = b"syscall=%d (one of get/set uid/pid/gid)\n\0";
        bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as i32, sc_nr);
    }
    0
}

/* we jump here when syscall number == __NR_write */
#[no_mangle]
// SEC("kprobe/SYS__NR_write")
pub unsafe extern "C" fn bpf_func_SYS__NR_write(ctx: *mut pt_regs) -> i32 {
    let mut sd: seccomp_data = core::mem::zeroed();

    bpf_core_read(&mut sd as *mut _ as *mut core::ffi::c_void,
                  core::mem::size_of::<seccomp_data>(),
                  PT_REGS_PARM2(ctx) as *const core::ffi::c_void);
    if sd.args[2] == 512 {
        let fmt: &[u8] = b"write(fd=%d, buf=%p, size=%d)\n\0";
        bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as i32,
                         sd.args[0], sd.args[1], sd.args[2]);
    }
    0
}

#[no_mangle]
// SEC("kprobe/SYS__NR_read")
pub unsafe extern "C" fn bpf_func_SYS__NR_read(ctx: *mut pt_regs) -> i32 {
    let mut sd: seccomp_data = core::mem::zeroed();

    bpf_core_read(&mut sd as *mut _ as *mut core::ffi::c_void,
                  core::mem::size_of::<seccomp_data>(),
                  PT_REGS_PARM2(ctx) as *const core::ffi::c_void);
    if sd.args[2] > 128 && sd.args[2] <= 1024 {
        let fmt: &[u8] = b"read(fd=%d, buf=%p, size=%d)\n\0";
        bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as i32,
                         sd.args[0], sd.args[1], sd.args[2]);
    }
    0
}

// #ifdef __NR_mmap2
#[no_mangle]
// SEC("kprobe/SYS__NR_mmap2")
pub unsafe extern "C" fn bpf_func_SYS__NR_mmap2(_ctx: *mut pt_regs) -> i32 {
    let fmt: &[u8] = b"mmap2\n\0";
    bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as i32);
    0
}
// #endif

// #ifdef __NR_mmap
#[no_mangle]
// SEC("kprobe/SYS__NR_mmap")
pub unsafe extern "C" fn bpf_func_SYS__NR_mmap(_ctx: *mut pt_regs) -> i32 {
    let fmt: &[u8] = b"mmap\n\0";
    bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as i32);
    0
}
// #endif

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// u32 _version SEC("version") = LINUX_VERSION_CODE;
#[no_mangle]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// External symbols/macros supplied by the included kernel and BPF headers.
extern "Rust" {
    static mut PROGS: Progs;
    static LINUX_VERSION_CODE: u32;
    fn PT_REGS_PARM1(ctx: *mut pt_regs) -> u64;
    fn PT_REGS_PARM2(ctx: *mut pt_regs) -> u64;
    static __NR_getuid: i32;
    static __NR_getsid: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/* Copyright 2022 Sony Group Corporation */

/* Dependencies from the original C file:
 * <vmlinux.h>
 * <bpf/bpf_core_read.h>
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * "bpf_misc.h"
 */

#[allow(non_camel_case_types)]
type pid_t = i32;
#[allow(non_camel_case_types)]
type loff_t = i64;
#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void)
        -> i32;

    fn PT_REGS_SYSCALL_REGS(ctx: *mut pt_regs) -> *mut pt_regs;
    fn PT_REGS_PARM1_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM2_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM3_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM4(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM4_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM5_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM1_CORE_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM2_CORE_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM3_CORE_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM4_CORE(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM4_CORE_SYSCALL(regs: *mut pt_regs) -> usize;
    fn PT_REGS_PARM5_CORE_SYSCALL(regs: *mut pt_regs) -> usize;
}

#[no_mangle]
pub static mut arg1: i32 = 0;
#[no_mangle]
pub static mut arg2: u64 = 0;
#[no_mangle]
pub static mut arg3: u64 = 0;
#[no_mangle]
pub static mut arg4_cx: u64 = 0;
#[no_mangle]
pub static mut arg4: u64 = 0;
#[no_mangle]
pub static mut arg5: u64 = 0;

#[no_mangle]
pub static mut arg1_core: i32 = 0;
#[no_mangle]
pub static mut arg2_core: u64 = 0;
#[no_mangle]
pub static mut arg3_core: u64 = 0;
#[no_mangle]
pub static mut arg4_core_cx: u64 = 0;
#[no_mangle]
pub static mut arg4_core: u64 = 0;
#[no_mangle]
pub static mut arg5_core: u64 = 0;

#[no_mangle]
pub static mut option_syscall: i32 = 0;
#[no_mangle]
pub static mut arg2_syscall: u64 = 0;
#[no_mangle]
pub static mut arg3_syscall: u64 = 0;
#[no_mangle]
pub static mut arg4_syscall: u64 = 0;
#[no_mangle]
pub static mut arg5_syscall: u64 = 0;

/* Original declaration was `const volatile pid_t filter_pid = 0;`. */
#[no_mangle]
pub static filter_pid: pid_t = 0;

#[no_mangle]
#[link_section = "kprobe/SYS_PREFIXsys_prctl"]
pub unsafe extern "C" fn handle_sys_prctl(ctx: *mut pt_regs) -> i32 {
    let real_regs: *mut pt_regs;
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;
    let mut tmp: u64 = 0;

    if pid != core::ptr::read_volatile(&filter_pid) {
        return 0;
    }

    real_regs = PT_REGS_SYSCALL_REGS(ctx);

    /* test for PT_REGS_PARM */

    bpf_probe_read_kernel(
        &mut tmp as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&tmp) as u32,
        &PT_REGS_PARM1_SYSCALL(real_regs) as *const _ as *const core::ffi::c_void,
    );
    arg1 = tmp as i32;
    bpf_probe_read_kernel(
        &mut arg2 as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&arg2) as u32,
        &PT_REGS_PARM2_SYSCALL(real_regs) as *const _ as *const core::ffi::c_void,
    );
    bpf_probe_read_kernel(
        &mut arg3 as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&arg3) as u32,
        &PT_REGS_PARM3_SYSCALL(real_regs) as *const _ as *const core::ffi::c_void,
    );
    bpf_probe_read_kernel(
        &mut arg4_cx as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&arg4_cx) as u32,
        &PT_REGS_PARM4(real_regs) as *const _ as *const core::ffi::c_void,
    );
    bpf_probe_read_kernel(
        &mut arg4 as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&arg4) as u32,
        &PT_REGS_PARM4_SYSCALL(real_regs) as *const _ as *const core::ffi::c_void,
    );
    bpf_probe_read_kernel(
        &mut arg5 as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&arg5) as u32,
        &PT_REGS_PARM5_SYSCALL(real_regs) as *const _ as *const core::ffi::c_void,
    );

    /* test for the CORE variant of PT_REGS_PARM */
    arg1_core = PT_REGS_PARM1_CORE_SYSCALL(real_regs) as i32;
    arg2_core = PT_REGS_PARM2_CORE_SYSCALL(real_regs) as u64;
    arg3_core = PT_REGS_PARM3_CORE_SYSCALL(real_regs) as u64;
    arg4_core_cx = PT_REGS_PARM4_CORE(real_regs) as u64;
    arg4_core = PT_REGS_PARM4_CORE_SYSCALL(real_regs) as u64;
    arg5_core = PT_REGS_PARM5_CORE_SYSCALL(real_regs) as u64;

    return 0;
}

#[no_mangle]
#[link_section = "ksyscall/prctl"]
pub unsafe extern "C" fn prctl_enter(
    option: i32,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != core::ptr::read_volatile(&filter_pid) {
        return 0;
    }

    option_syscall = option;
    arg2_syscall = arg2;
    arg3_syscall = arg3;
    arg4_syscall = arg4;
    arg5_syscall = arg5;
    return 0;
}

#[no_mangle]
pub static mut splice_fd_in: __u64 = 0;
#[no_mangle]
pub static mut splice_off_in: __u64 = 0;
#[no_mangle]
pub static mut splice_fd_out: __u64 = 0;
#[no_mangle]
pub static mut splice_off_out: __u64 = 0;
#[no_mangle]
pub static mut splice_len: __u64 = 0;
#[no_mangle]
pub static mut splice_flags: __u64 = 0;

#[no_mangle]
#[link_section = "ksyscall/splice"]
pub unsafe extern "C" fn splice_enter(
    fd_in: i32,
    off_in: *mut loff_t,
    fd_out: i32,
    off_out: *mut loff_t,
    len: size_t,
    flags: u32,
) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != core::ptr::read_volatile(&filter_pid) {
        return 0;
    }

    splice_fd_in = fd_in as __u64;
    splice_off_in = off_in as __u64;
    splice_fd_out = fd_out as __u64;
    splice_off_out = off_out as __u64;
    splice_len = len as __u64;
    splice_flags = flags as __u64;

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

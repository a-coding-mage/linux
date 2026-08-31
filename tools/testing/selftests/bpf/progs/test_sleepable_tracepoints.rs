// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Translated from C. External BPF/kernel definitions are supplied by the
 * original repository dependencies: vmlinux.h, asm/unistd.h,
 * bpf/bpf_tracing.h, bpf/bpf_core_read.h, and bpf/bpf_helpers.h.
 */

extern "C" {
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: usize, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn PT_REGS_PARM1_SYSCALL(regs: *mut pt_regs) -> u64;
    fn PT_REGS_PARM1_CORE_SYSCALL(regs: *mut pt_regs) -> u64;
}

extern "C" {
    static __NR_getcwd: i64;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_trace_enter {
    pub args: [u64; 6],
}

#[repr(C)]
pub struct syscall_trace_exit {
    _private: [u8; 0],
}

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub static mut target_pid: i32 = 0;
pub static mut prog_triggered: i32 = 0;
pub static mut err: i64 = 0;
pub static mut copied_byte: i8 = 0;

unsafe fn copy_getcwd_arg(ubuf: *mut i8) -> i32 {
    err = bpf_copy_from_user(
        core::ptr::addr_of_mut!(copied_byte).cast::<core::ffi::c_void>(),
        core::mem::size_of_val(&copied_byte),
        ubuf.cast::<core::ffi::c_void>(),
    );
    if err != 0 {
        return err as i32;
    }

    prog_triggered = 1;
    0
}

#[no_mangle]
#[link_section = "tp_btf.s/sys_enter"]
pub unsafe extern "C" fn handle_sys_enter_tp_btf(regs: *mut pt_regs, id: i64) -> i32 {
    if ((bpf_get_current_pid_tgid() >> 32) as i32) != target_pid || id != __NR_getcwd {
        return 0;
    }

    copy_getcwd_arg(PT_REGS_PARM1_SYSCALL(regs) as *mut i8)
}

#[no_mangle]
#[link_section = "raw_tp.s/sys_enter"]
pub unsafe extern "C" fn handle_sys_enter_raw_tp(regs: *mut pt_regs, id: i64) -> i32 {
    if ((bpf_get_current_pid_tgid() >> 32) as i32) != target_pid || id != __NR_getcwd {
        return 0;
    }

    copy_getcwd_arg(PT_REGS_PARM1_CORE_SYSCALL(regs) as *mut i8)
}

#[no_mangle]
#[link_section = "tp.s/syscalls/sys_enter_getcwd"]
pub unsafe extern "C" fn handle_sys_enter_tp(args: *mut syscall_trace_enter) -> i32 {
    if ((bpf_get_current_pid_tgid() >> 32) as i32) != target_pid {
        return 0;
    }

    copy_getcwd_arg((*args).args[0] as *mut i8)
}

#[no_mangle]
#[link_section = "tp.s/syscalls/sys_exit_getcwd"]
pub unsafe extern "C" fn handle_sys_exit_tp(_args: *mut syscall_trace_exit) -> i32 {
    let regs: *mut pt_regs;

    if ((bpf_get_current_pid_tgid() >> 32) as i32) != target_pid {
        return 0;
    }

    regs = bpf_task_pt_regs(bpf_get_current_task_btf());
    copy_getcwd_arg(PT_REGS_PARM1_CORE_SYSCALL(regs) as *mut i8)
}

#[no_mangle]
#[link_section = "raw_tp.s"]
pub unsafe extern "C" fn handle_raw_tp_bare(_regs: *mut pt_regs, _id: i64) -> i32 {
    0
}

#[no_mangle]
#[link_section = "tp.s"]
pub unsafe extern "C" fn handle_tp_bare(_ctx: *mut core::ffi::c_void) -> i32 {
    0
}

#[no_mangle]
#[link_section = "tracepoint.s/syscalls/sys_enter_getcwd"]
pub unsafe extern "C" fn handle_sys_enter_tp_alias(_args: *mut syscall_trace_enter) -> i32 {
    0
}

#[no_mangle]
#[link_section = "raw_tracepoint.s/sys_enter"]
pub unsafe extern "C" fn handle_sys_enter_raw_tp_alias(_regs: *mut pt_regs, _id: i64) -> i32 {
    0
}

#[no_mangle]
#[link_section = "raw_tp.s/sys_enter"]
pub unsafe extern "C" fn handle_test_run(regs: *mut pt_regs, id: i64) -> i32 {
    if regs as u64 == 0x1234_u64 && id as u64 == 0x5678_u64 {
        return ((regs as u64).wrapping_add(id as u64)) as i32;
    }

    0
}

#[no_mangle]
#[link_section = "raw_tp.s/sched_switch"]
pub unsafe extern "C" fn handle_raw_tp_non_faultable(
    _preempt: bool,
    _prev: *mut task_struct,
    _next: *mut task_struct,
) -> i32 {
    0
}

#[no_mangle]
#[link_section = "tp.s/sched/sched_switch"]
pub unsafe extern "C" fn handle_tp_non_syscall(_ctx: *mut core::ffi::c_void) -> i32 {
    0
}

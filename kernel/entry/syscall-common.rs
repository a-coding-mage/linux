// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_long, c_ulong, c_void};

// Declarations supplied by the Linux kernel headers and tracepoint subsystem.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn current() -> *mut c_void;
    fn syscall_get_nr(task: *mut c_void, regs: *const pt_regs) -> c_long;
    fn syscall_get_arguments(
        task: *mut c_void,
        regs: *const pt_regs,
        args: *mut c_ulong,
    );
    fn trace_sys_enter(regs: *mut pt_regs, syscall: c_long);
    fn trace_sys_exit(regs: *mut pt_regs, ret: c_long);
    fn __audit_syscall_entry(
        syscall: c_long,
        arg0: c_ulong,
        arg1: c_ulong,
        arg2: c_ulong,
        arg3: c_ulong,
    );
}

/* Out of line to prevent tracepoint code duplication */
pub unsafe fn trace_syscall_enter(regs: *mut pt_regs) {
    trace_sys_enter(regs, syscall_get_nr(current(), regs));
}

pub unsafe fn trace_syscall_exit(regs: *mut pt_regs, ret: c_long) {
    trace_sys_exit(regs, ret);
}

// CONFIG_AUDITSYSCALL conditionally includes this implementation in C.
#[cfg(feature = "CONFIG_AUDITSYSCALL")]
pub unsafe fn syscall_enter_audit(regs: *mut pt_regs) {
    let syscall: c_long = syscall_get_nr(current(), regs);
    let mut args: [c_ulong; 6] = [0; 6];

    syscall_get_arguments(current(), regs, args.as_mut_ptr());
    __audit_syscall_entry(syscall, args[0], args[1], args[2], args[3]);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

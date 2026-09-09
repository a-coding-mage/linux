/* SPDX-License-Identifier: GPL-2.0 */

// C header metadata:
// TRACE_SYSTEM raw_syscalls
// TRACE_INCLUDE_FILE syscalls
// The include guard and TRACE_HEADER_MULTI_READ condition are represented by
// this Rust source file's module/include semantics.

// External kernel dependencies supplied by other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

extern "C" {
    pub static mut current: *mut c_void;

    pub fn syscall_get_arguments(
        task: *mut c_void,
        regs: *mut PtRegs,
        args: *mut ::core::ffi::c_ulong,
    );
    pub fn syscall_get_nr(task: *mut c_void, regs: *mut PtRegs) -> ::core::ffi::c_long;

    pub fn syscall_regfunc();
    pub fn syscall_unregfunc();
}

#[repr(C)]
pub struct SysEnterEntry {
    pub id: ::core::ffi::c_long,
    pub args: [::core::ffi::c_ulong; 6],
}

#[repr(C)]
pub struct SysExitEntry {
    pub id: ::core::ffi::c_long,
    pub ret: ::core::ffi::c_long,
}

/// Equivalent to the `sys_enter` TRACE_EVENT_SYSCALL declaration.
#[inline]
pub unsafe fn sys_enter_fast_assign(
    entry: *mut SysEnterEntry,
    regs: *mut PtRegs,
    id: ::core::ffi::c_long,
) {
    (*entry).id = id;
    syscall_get_arguments(current, regs, (*entry).args.as_mut_ptr());
}

/// Equivalent to the `sys_exit` TRACE_EVENT_SYSCALL declaration.
#[inline]
pub unsafe fn sys_exit_fast_assign(
    entry: *mut SysExitEntry,
    regs: *mut PtRegs,
    ret: ::core::ffi::c_long,
) {
    (*entry).id = syscall_get_nr(current, regs);
    (*entry).ret = ret;
}

// TP_printk formats from the C trace events:
// sys_enter: "NR %ld (%lx, %lx, %lx, %lx, %lx, %lx)"
// sys_exit:  "NR %ld = %ld"

// TRACE_EVENT_FLAGS(sys_enter, TRACE_EVENT_FL_CAP_ANY)
// TRACE_EVENT_FLAGS(sys_exit, TRACE_EVENT_FL_CAP_ANY)
// CONFIG_HAVE_SYSCALL_TRACEPOINTS conditionally enables the declarations
// above; the condition is a build-time dependency supplied by the kernel.

// The C header's trailing <trace/define_trace.h> include is intentionally
// external and is not implemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

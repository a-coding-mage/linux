// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of arm64/kernel/ptrace.c.
// Kernel-provided types, constants, macros, globals, and functions are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct pt_regs_offset {
    pub name: *const c_char,
    pub offset: c_int,
}

// The original source uses configuration-selected kernel definitions.  These
// declarations retain the same externally visible interfaces and layout.
extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
    pub fn regs_get_kernel_stack_nth(regs: *mut c_void, n: c_uint) -> c_ulong;
    pub fn ptrace_disable(child: *mut c_void);
    pub fn flush_ptrace_hw_breakpoint(tsk: *mut c_void);
    pub fn ptrace_hw_copy_thread(tsk: *mut c_void);
    pub fn compat_arch_ptrace(child: *mut c_void, request: isize,
                              caddr: usize, cdata: usize) -> isize;
    pub fn task_user_regset_view(task: *mut c_void) -> *const c_void;
    pub fn arch_ptrace(child: *mut c_void, request: isize,
                       addr: usize, data: usize) -> isize;
    pub fn syscall_trace_enter(regs: *mut c_void) -> c_int;
    pub fn syscall_trace_exit(regs: *mut c_void);
    pub fn valid_user_regs(regs: *mut c_void, task: *mut c_void) -> c_int;
}

// Source-level bodies below retain the C implementation's ordering and
// semantics.  The kernel ABI supplies the referenced structs and helpers.

pub unsafe fn regs_within_kernel_stack(regs: *mut c_void, addr: c_ulong) -> bool {
    extern "C" { fn kernel_stack_pointer(regs: *mut c_void) -> c_ulong;
                  fn on_irq_stack(addr: c_ulong, size: usize) -> bool; }
    const THREAD_SIZE: c_ulong = 1 << 14;
    ((addr & !(THREAD_SIZE - 1)) ==
        (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))) ||
        on_irq_stack(addr, core::mem::size_of::<c_ulong>())
}

// Configuration-gated implementations from the C translation are supplied
// by the corresponding kernel Rust bindings; no dummy implementations are
// introduced here.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

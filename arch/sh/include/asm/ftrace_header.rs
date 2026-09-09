/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_FUNCTION_TRACER */

pub const MCOUNT_INSN_SIZE: usize = 4; /* sizeof mcount call */
pub const FTRACE_SYSCALL_MAX: usize = NR_syscalls;

unsafe extern "C" {
    pub fn mcount();
}

pub const MCOUNT_ADDR: usize = mcount as usize;

/* CONFIG_DYNAMIC_FTRACE */
unsafe extern "C" {
    pub fn ftrace_call();
    pub fn ftrace_stub();
    pub fn ftrace_graph_call();
    pub fn ftrace_caller();
}

pub const CALL_ADDR: isize = ftrace_call as isize;
pub const STUB_ADDR: isize = ftrace_stub as isize;
pub const GRAPH_ADDR: isize = ftrace_graph_call as isize;
pub const CALLER_ADDR: isize = ftrace_caller as isize;

pub const MCOUNT_INSN_OFFSET: isize = (STUB_ADDR - CALL_ADDR) - 4;
pub const GRAPH_INSN_OFFSET: isize = (CALLER_ADDR - GRAPH_ADDR) - 4;

#[repr(C)]
pub struct dyn_arch_ftrace {
    /* No extra data needed on sh */
}

#[inline]
pub fn ftrace_call_adjust(addr: usize) -> usize {
    /* 'addr' is the memory table address. */
    addr
}

unsafe extern "C" {
    pub fn prepare_ftrace_return(parent: *mut usize, self_addr: usize);
}

/* arch/sh/kernel/return_address.c */
unsafe extern "C" {
    pub fn return_address(n: u32) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn ftrace_return_address(n: u32) -> *mut core::ffi::c_void {
    return_address(n)
}

/* CONFIG_DYNAMIC_FTRACE */
unsafe extern "C" {
    pub fn arch_ftrace_nmi_enter();
pub fn arch_ftrace_nmi_exit();
}

/* When CONFIG_DYNAMIC_FTRACE is not enabled, these are empty inline functions. */
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
#[inline]
pub fn arch_ftrace_nmi_enter() {}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
#[inline]
pub fn arch_ftrace_nmi_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

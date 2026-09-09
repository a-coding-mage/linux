/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/ftrace.h
 *
 * Copyright (C) 2013 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

// Dependency: asm/insn.h
// #include <asm/insn.h>

pub const HAVE_FUNCTION_GRAPH_FP_TEST: bool = true;

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub const ARCH_SUPPORTS_FTRACE_OPS: i32 = 1;
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
pub const MCOUNT_ADDR: usize = _mcount as usize;

/* The BL at the callsite's adjusted rec->ip */
pub const MCOUNT_INSN_SIZE: usize = AARCH64_INSN_SIZE;

pub const FTRACE_PLT_IDX: i32 = 0;
pub const NR_FTRACE_PLTS: i32 = 1;

/*
 * Currently, gcc tends to save the link register after the local variables
 * on the stack. This causes the max stack tracer to report the function
 * frame sizes for the wrong functions. By defining
 * ARCH_FTRACE_SHIFT_STACK_TRACER, it will tell the stack tracer to expect
 * to find the return address on the stack after the local variables have
 * been set up.
 *
 * Note, this may change in the future, and we will need to deal with that
 * if it were to happen.
 */
pub const ARCH_FTRACE_SHIFT_STACK_TRACER: i32 = 1;

extern "C" {
    pub fn _mcount(arg: c_ulong);
    pub fn return_address(n: c_uint) -> *mut c_void;
    pub static mut ftrace_graph_call: c_ulong;
    pub fn return_to_handler();
    pub fn ftrace_call_adjust(addr: c_ulong) -> c_ulong;
    pub fn arch_ftrace_get_symaddr(fentry_ip: c_ulong) -> c_ulong;
}

pub type c_ulong = usize;
pub type c_uint = u32;
pub type c_void = core::ffi::c_void;

pub const ftrace_get_symaddr: unsafe extern "C" fn(c_ulong) -> c_ulong = arch_ftrace_get_symaddr;

#[repr(C)]
pub struct dyn_arch_ftrace {
    /* No extra data needed for arm64 */
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub struct dyn_ftrace;
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub struct ftrace_ops;
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub struct ftrace_regs;

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[repr(C)]
pub struct __arch_ftrace_regs {
    /* x0 - x8 */
    pub regs: [c_ulong; 9],
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS")]
    pub direct_tramp: c_ulong,
    #[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS"))]
    pub __unused: c_ulong,
    pub fp: c_ulong,
    pub lr: c_ulong,
    pub sp: c_ulong,
    pub pc: c_ulong,
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_instruction_pointer(fregs: *const ftrace_regs) -> c_ulong {
    (*(fregs as *const __arch_ftrace_regs)).pc
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, pc: c_ulong) {
    (*(fregs as *mut __arch_ftrace_regs)).pc = pc;
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_stack_pointer(fregs: *const ftrace_regs) -> c_ulong {
    (*(fregs as *const __arch_ftrace_regs)).sp
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_argument(fregs: *mut ftrace_regs, n: c_uint) -> c_ulong {
    if n < 8 { (*(fregs as *const __arch_ftrace_regs)).regs[n as usize] } else { 0 }
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_value(fregs: *const ftrace_regs) -> c_ulong {
    (*(fregs as *const __arch_ftrace_regs)).regs[0]
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_set_return_value(fregs: *mut ftrace_regs, ret: c_ulong) {
    (*(fregs as *mut __arch_ftrace_regs)).regs[0] = ret;
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_override_function_with_return(fregs: *mut ftrace_regs) {
    (*(fregs as *mut __arch_ftrace_regs)).pc = (*(fregs as *mut __arch_ftrace_regs)).lr;
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *const ftrace_regs) -> c_ulong {
    (*(fregs as *const __arch_ftrace_regs)).fp
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_address(fregs: *const ftrace_regs) -> c_ulong {
    (*(fregs as *const __arch_ftrace_regs)).lr
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn ftrace_partial_regs(fregs: *const ftrace_regs, regs: *mut pt_regs) -> *mut pt_regs {
    let afregs = fregs as *const __arch_ftrace_regs;
    // These accesses correspond to the fields of the externally supplied struct pt_regs.
    core::ptr::copy_nonoverlapping(
        (*afregs).regs.as_ptr(),
        (*regs).regs.as_mut_ptr(),
        (*afregs).regs.len(),
    );
    (*regs).sp = (*afregs).sp;
    (*regs).pc = (*afregs).pc;
    (*regs).regs[29] = (*afregs).fp;
    (*regs).regs[30] = (*afregs).lr;
    (*regs).pstate = PSR_MODE_EL1h;
    regs
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
#[inline(always)]
pub unsafe fn arch_ftrace_fill_perf_regs(fregs: *const ftrace_regs, regs: *mut pt_regs) {
    let afregs = fregs as *const __arch_ftrace_regs;
    (*regs).pc = (*afregs).pc;
    (*regs).regs[29] = (*afregs).fp;
    (*regs).sp = (*afregs).sp;
    (*regs).pstate = PSR_MODE_EL1h;
}

// The following declarations depend on the corresponding kernel types and constants.
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
extern "C" {
    pub fn ftrace_regs_query_register_offset(name: *const i8) -> i32;
    pub fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> i32;
    pub fn ftrace_graph_func(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs);
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS")]
#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: c_ulong) {
    /* The ftrace trampoline will return to this address instead of the instrumented function. */
    (*(fregs as *mut __arch_ftrace_regs)).direct_tramp = addr;
}

#[inline]
pub unsafe fn ftrace_return_address(n: c_uint) -> *mut c_void { return_address(n) }

// Compat syscall tracing is intentionally disabled on arm64.
pub const ARCH_TRACE_IGNORE_COMPAT_SYSCALLS: bool = true;
pub unsafe fn arch_trace_is_compat_syscall(regs: *mut pt_regs) -> bool {
    is_compat_task()
}

pub const ARCH_HAS_SYSCALL_MATCH_SYM_NAME: bool = true;
pub unsafe fn arch_syscall_match_sym_name(sym: *const i8, name: *const i8) -> bool {
    strcmp(sym.add(8), name) != 0
}

#[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
extern "C" {
    pub fn prepare_ftrace_return(self_addr: c_ulong, parent: *mut c_ulong, frame_pointer: c_ulong);
}

// External kernel dependencies supplied by other translated files.
#[allow(non_camel_case_types)]
pub struct module;
#[allow(non_camel_case_types)]
pub struct pt_regs;
extern "C" {
    fn is_compat_task() -> bool;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

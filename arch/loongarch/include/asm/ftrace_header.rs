/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

pub const FTRACE_PLT_IDX: usize = 0;
pub const FTRACE_REGS_PLT_IDX: usize = 1;
pub const NR_FTRACE_PLTS: usize = 2;

/* CONFIG_FUNCTION_TRACER */
pub const MCOUNT_INSN_SIZE: usize = 4; /* sizeof mcount call */

/* __ASSEMBLER__ is not defined for this Rust translation. */

/* !CONFIG_DYNAMIC_FTRACE */
pub use _mcount as mcount;

unsafe extern "C" {
    pub fn _mcount();
}

/* CONFIG_DYNAMIC_FTRACE */
#[repr(C)]
pub struct dyn_ftrace;

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct dyn_arch_ftrace {}

pub const ARCH_SUPPORTS_FTRACE_OPS: usize = 1;

unsafe extern "C" {
    pub fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> libc::c_int;
    pub fn prepare_ftrace_return(self_addr: libc::c_ulong, parent: *mut libc::c_ulong);
}

#[inline]
pub const fn ftrace_call_adjust(addr: libc::c_ulong) -> libc::c_ulong {
    addr
}

/* CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS */
#[repr(C)]
pub struct ftrace_ops;

/* Dependency supplied by linux/ftrace_regs.h. */
#[repr(C)]
pub struct ftrace_regs;
#[repr(C)]
pub struct pt_regs {
    pub regs: [libc::c_ulong; 32],
}

unsafe extern "C" {
    pub fn arch_ftrace_regs(fregs: *mut ftrace_regs) -> *mut ftrace_regs_arch;
    pub fn instruction_pointer_set(regs: *mut pt_regs, ip: libc::c_ulong);
    pub fn ftrace_graph_func(
        ip: libc::c_ulong,
        parent_ip: libc::c_ulong,
        op: *mut ftrace_ops,
        fregs: *mut ftrace_regs,
    );
}

#[repr(C)]
pub struct ftrace_regs_arch {
    pub regs: pt_regs,
}

#[inline(always)]
pub unsafe fn arch_ftrace_get_regs(fregs: *mut ftrace_regs) -> *mut pt_regs {
    &mut (*arch_ftrace_regs(fregs)).regs
}

#[inline(always)]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, ip: libc::c_ulong) {
    instruction_pointer_set(&mut (*arch_ftrace_regs(fregs)).regs, ip);
}

#[inline(always)]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *mut ftrace_regs) -> libc::c_ulong {
    (*arch_ftrace_regs(fregs)).regs.regs[22]
}

#[inline(always)]
pub unsafe fn ftrace_regs_get_return_address(fregs: *mut ftrace_regs) -> libc::c_ulong {
    *((*arch_ftrace_regs(fregs)).regs.regs[1] as *const libc::c_ulong)
}

/* CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS */
#[inline]
pub unsafe fn __arch_ftrace_set_direct_caller(regs: *mut pt_regs, addr: libc::c_ulong) {
    (*regs).regs[13] = addr; /* t1 */
}

#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: libc::c_ulong) {
    __arch_ftrace_set_direct_caller(&mut (*arch_ftrace_regs(fregs)).regs, addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

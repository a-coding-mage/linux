/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2017 Andes Technology Corporation */

// The graph frame test is not possible if CONFIG_FRAME_POINTER is not enabled.
// Check arch/riscv/kernel/mcount.S for detail.
// CONFIG_FUNCTION_GRAPH_TRACER && CONFIG_FRAME_POINTER => HAVE_FUNCTION_GRAPH_FP_TEST

pub const ARCH_SUPPORTS_FTRACE_OPS: i32 = 1;

extern "C" {
    pub fn return_address(level: u32) -> *mut core::ffi::c_void;
    pub fn _mcount();
    pub fn ftrace_call_adjust(addr: usize) -> usize;
    pub fn arch_ftrace_get_symaddr(fentry_ip: usize) -> usize;
}

#[inline]
pub unsafe fn ftrace_return_address(n: u32) -> *mut core::ffi::c_void {
    return_address(n)
}

#[inline]
pub unsafe fn ftrace_get_symaddr(fentry_ip: usize) -> usize {
    arch_ftrace_get_symaddr(fentry_ip)
}

// Let's do like x86/arm64 and ignore the compat syscalls.
// ARCH_TRACE_IGNORE_COMPAT_SYSCALLS
#[inline]
pub unsafe fn arch_trace_is_compat_syscall(_regs: *mut pt_regs) -> bool {
    is_compat_task()
}

// ARCH_HAS_SYSCALL_MATCH_SYM_NAME
#[inline]
pub unsafe fn arch_syscall_match_sym_name(sym: *const core::ffi::c_char,
                                          name: *const core::ffi::c_char) -> bool {
    // Since all syscall functions have __riscv_ prefix, we must skip it.
    // Compat syscalls are ignored, so __riscv_compat_ is not relevant here.
    strcmp(sym.add(8), name) == 0
}

#[repr(C)]
pub struct dyn_arch_ftrace {}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const MCOUNT_ADDR: usize = _mcount as usize;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const JALR_SIGN_MASK: usize = 0x00000800;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const JALR_OFFSET_MASK: usize = 0x00000fff;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const AUIPC_OFFSET_MASK: usize = 0xfffff000;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const AUIPC_PAD: usize = 0x00001000;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const JALR_SHIFT: usize = 20;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const JALR_T0: usize = 0x000282e7;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const AUIPC_T0: usize = 0x00000297;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const JALR_RANGE: usize = JALR_SIGN_MASK - 1;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[inline]
pub const fn to_jalr_t0(offset: usize) -> usize {
    ((offset & JALR_OFFSET_MASK) << JALR_SHIFT) | JALR_T0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[inline]
pub const fn to_auipc_t0(offset: usize) -> usize {
    if offset & JALR_SIGN_MASK != 0 {
        ((offset & AUIPC_OFFSET_MASK).wrapping_add(AUIPC_PAD)) | AUIPC_T0
    } else {
        (offset & AUIPC_OFFSET_MASK) | AUIPC_T0
    }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[inline]
pub unsafe fn make_call_t0(caller: usize, callee: usize, call: *mut u32) {
    let offset = callee.wrapping_sub(caller);
    *call = to_auipc_t0(offset) as u32;
    *call.add(1) = to_jalr_t0(offset) as u32;
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const MCOUNT_INSN_SIZE: usize = 4;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const MCOUNT_AUIPC_SIZE: usize = 4;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const MCOUNT_JALR_SIZE: usize = 4;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub const MCOUNT_NOP4_SIZE: usize = 4;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
extern "C" {
    pub fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> i32;
    pub fn ftrace_regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
    pub fn ftrace_graph_func(ip: usize, parent_ip: usize, op: *mut ftrace_ops,
                             fregs: *mut ftrace_regs);
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[repr(C)]
pub struct __arch_ftrace_regs {
    pub epc: usize,
    pub ra: usize,
    pub sp: usize,
    pub s0: usize,
    pub t1: usize,
    #[cfg(CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS)]
    pub direct_tramp: usize,
    pub args: [usize; 8],
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn arch_ftrace_get_regs(_regs: *mut ftrace_regs) -> *mut pt_regs { core::ptr::null_mut() }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn arch_ftrace_regs(fregs: *mut ftrace_regs) -> *mut __arch_ftrace_regs {
    fregs as *mut __arch_ftrace_regs
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_instruction_pointer(fregs: *const ftrace_regs) -> usize {
    (*arch_ftrace_regs(fregs as *mut ftrace_regs)).epc
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, pc: usize) {
    (*arch_ftrace_regs(fregs)).epc = pc;
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_stack_pointer(fregs: *const ftrace_regs) -> usize { (*arch_ftrace_regs(fregs as *mut _)).sp }
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *const ftrace_regs) -> usize { (*arch_ftrace_regs(fregs as *mut _)).s0 }
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_argument(fregs: *mut ftrace_regs, n: u32) -> usize {
    if n < 8 { (*arch_ftrace_regs(fregs)).args[n as usize] } else { 0 }
}
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_return_value(fregs: *const ftrace_regs) -> usize { (*arch_ftrace_regs(fregs as *mut _)).args[0] }
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_get_return_address(fregs: *const ftrace_regs) -> usize { (*arch_ftrace_regs(fregs as *mut _)).ra }
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_regs_set_return_value(fregs: *mut ftrace_regs, ret: usize) { (*arch_ftrace_regs(fregs)).args[0] = ret; }
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_override_function_with_return(fregs: *mut ftrace_regs) { (*arch_ftrace_regs(fregs)).epc = (*arch_ftrace_regs(fregs)).ra; }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline]
pub unsafe fn ftrace_partial_regs(fregs: *const ftrace_regs, regs: *mut pt_regs) -> *mut pt_regs {
    let afregs = arch_ftrace_regs(fregs as *mut ftrace_regs);
    core::ptr::copy_nonoverlapping((*afregs).args.as_ptr(), (*regs).a_regs.as_mut_ptr(), 8);
    (*regs).epc = (*afregs).epc;
    (*regs).ra = (*afregs).ra;
    (*regs).sp = (*afregs).sp;
    (*regs).s0 = (*afregs).s0;
    (*regs).t1 = (*afregs).t1;
    regs
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS)]
#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: usize) { (*arch_ftrace_regs(fregs)).t1 = addr; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

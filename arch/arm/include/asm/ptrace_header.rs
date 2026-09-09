/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/ptrace.h
 *
 *  Copyright (C) 1996-2003 Russell King
 */

/* Definitions supplied by <uapi/asm/ptrace.h> and other kernel headers. */

#[repr(C)]
pub struct pt_regs {
    pub uregs: [::core::ffi::c_ulong; 18],
}

#[repr(C)]
pub struct svc_pt_regs {
    pub regs: pt_regs,
    pub dacr: u32,
    pub ttbcr: u32,
}

pub const fn to_svc_pt_regs(r: *mut pt_regs) -> *mut svc_pt_regs {
    r as *mut svc_pt_regs
}

#[inline]
unsafe fn reg(regs: *const pt_regs, n: usize) -> ::core::ffi::c_ulong {
    (*regs).uregs[n]
}

#[inline]
unsafe fn reg_mut(regs: *mut pt_regs, n: usize) -> *mut ::core::ffi::c_ulong {
    (*regs).uregs.as_mut_ptr().add(n)
}

pub const ARM_R0: usize = 0;
pub const ARM_SP: usize = 13;
pub const ARM_FP: usize = 11;
pub const ARM_R7: usize = 7;
pub const ARM_PC: usize = 15;
pub const ARM_CPSR: usize = 16;
pub const ARM_ORIG_R0: usize = 17;

pub const fn user_mode(regs: *const pt_regs) -> bool {
    unsafe { (reg(regs, ARM_CPSR) & 0xf) == 0 }
}

pub const fn thumb_mode(regs: *const pt_regs) -> bool {
    unsafe { (reg(regs, ARM_CPSR) & PSR_T_BIT) != 0 }
}

pub const fn isa_mode(regs: *const pt_regs) -> ::core::ffi::c_ulong {
    unsafe { (((reg(regs, ARM_CPSR) & PSR_J_BIT) != 0) as ::core::ffi::c_ulong) << 1 | (((reg(regs, ARM_CPSR) & PSR_T_BIT) != 0) as ::core::ffi::c_ulong) }
}

pub const fn processor_mode(regs: *const pt_regs) -> ::core::ffi::c_ulong {
    unsafe { reg(regs, ARM_CPSR) & MODE_MASK }
}

pub const fn interrupts_enabled(regs: *const pt_regs) -> bool {
    unsafe { (reg(regs, ARM_CPSR) & PSR_I_BIT) == 0 }
}

pub const fn fast_interrupts_enabled(regs: *const pt_regs) -> bool {
    unsafe { (reg(regs, ARM_CPSR) & PSR_F_BIT) == 0 }
}

pub unsafe fn valid_user_regs(regs: *mut pt_regs) -> ::core::ffi::c_int {
    let cpsr = reg_mut(regs, ARM_CPSR);
    let mode = *cpsr & MODE_MASK;
    *cpsr &= !(PSR_F_BIT | PSR_A_BIT);
    if (*cpsr & PSR_I_BIT) == 0 {
        if mode == USR_MODE || (elf_hwcap & HWCAP_26BIT != 0 && mode == USR26_MODE) {
            return 1;
        }
    }
    *cpsr &= PSR_f | PSR_s | PSR_x | PSR_T_BIT | MODE32_BIT;
    if elf_hwcap & HWCAP_26BIT == 0 { *cpsr |= USR_MODE; }
    0
}

pub unsafe fn regs_return_value(regs: *mut pt_regs) -> ::core::ffi::c_long {
    reg(regs, ARM_R0) as ::core::ffi::c_long
}

pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> *mut ::core::ffi::c_ulong { reg_mut(regs, ARM_PC) }
pub unsafe fn frame_pointer(regs: *mut pt_regs) -> *mut ::core::ffi::c_ulong { reg_mut(regs, ARM_FP) }

pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: ::core::ffi::c_ulong) { *reg_mut(regs, ARM_PC) = val; }

pub const fn predicate(x: u32) -> u32 { x & 0xf0000000 }
pub const PREDICATE_ALWAYS: u32 = 0xe0000000;
pub const fn is_wide_instruction(instr: u32) -> bool { instr as u32 >= 0xe800 }

pub const MAX_REG_OFFSET: usize = ARM_ORIG_R0 * ::core::mem::size_of::<::core::ffi::c_ulong>();

unsafe extern "C" {
    pub static mut elf_hwcap: ::core::ffi::c_ulong;
    pub fn profile_pc(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    pub fn regs_query_register_offset(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn regs_query_register_name(offset: u32) -> *const ::core::ffi::c_char;
    pub fn regs_within_kernel_stack(regs: *mut pt_regs, addr: ::core::ffi::c_ulong) -> bool;
    pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> ::core::ffi::c_ulong;
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn syscall_trace_exit(regs: *mut pt_regs);
}

pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: u32) -> ::core::ffi::c_ulong {
    if offset as usize > MAX_REG_OFFSET { return 0; }
    *((regs as *mut u8).add(offset as usize) as *mut ::core::ffi::c_ulong)
}

pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong { reg(regs, ARM_SP) }
pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong { reg(regs, ARM_SP) }
pub unsafe fn regs_set_return_value(regs: *mut pt_regs, rc: ::core::ffi::c_ulong) { *reg_mut(regs, ARM_R0) = rc; }

pub fn it_advance(mut cpsr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if cpsr & 0x06000400 == 0 { cpsr &= !PSR_IT_MASK; } else {
        const MASK: ::core::ffi::c_ulong = 0x06001c00;
        let mut it = cpsr & MASK;
        it <<= 1;
        it |= it >> (27 - 10);
        it &= MASK;
        cpsr = (cpsr & !MASK) | it;
    }
    cpsr
}

/* External constants from the ARM ptrace and kernel type headers. */
extern "C" {
    static PSR_T_BIT: ::core::ffi::c_ulong;
    static PSR_J_BIT: ::core::ffi::c_ulong;
    static PSR_I_BIT: ::core::ffi::c_ulong;
    static PSR_F_BIT: ::core::ffi::c_ulong;
    static PSR_A_BIT: ::core::ffi::c_ulong;
    static PSR_f: ::core::ffi::c_ulong;
    static PSR_s: ::core::ffi::c_ulong;
    static PSR_x: ::core::ffi::c_ulong;
    static MODE_MASK: ::core::ffi::c_ulong;
    static MODE32_BIT: ::core::ffi::c_ulong;
    static USR_MODE: ::core::ffi::c_ulong;
    static USR26_MODE: ::core::ffi::c_ulong;
    static HWCAP_26BIT: ::core::ffi::c_ulong;
    static PSR_IT_MASK: ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

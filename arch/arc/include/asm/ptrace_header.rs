/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

// Dependency intent: uapi/asm/ptrace.h and linux/compiler.h are supplied by
// other translated components.  CONFIG_* conditions are build-time options.

#[repr(C)]
pub union ecr_reg {
    pub bits: EcrRegBits,
    pub full: c_ulong,
}

#[repr(C)]
#[cfg(not(CONFIG_CPU_BIG_ENDIAN))]
pub struct EcrRegBits {
    pub param: u8,
    pub cause: u8,
    pub vec: u8,
    pub state: u8,
}

#[repr(C)]
#[cfg(CONFIG_CPU_BIG_ENDIAN)]
pub struct EcrRegBits {
    pub state: u8,
    pub vec: u8,
    pub cause: u8,
    pub param: u8,
}

#[cfg(CONFIG_ISA_ARCOMPACT)]
#[repr(C)]
pub struct pt_regs {
    pub bta: c_ulong,
    pub lp_start: c_ulong,
    pub lp_end: c_ulong,
    pub lp_count: c_ulong,
    pub status32: c_ulong,
    pub ret: c_ulong,
    pub blink: c_ulong,
    pub fp: c_ulong,
    pub r26: c_ulong,
    pub r12: c_ulong,
    pub r11: c_ulong,
    pub r10: c_ulong,
    pub r9: c_ulong,
    pub r8: c_ulong,
    pub r7: c_ulong,
    pub r6: c_ulong,
    pub r5: c_ulong,
    pub r4: c_ulong,
    pub r3: c_ulong,
    pub r2: c_ulong,
    pub r1: c_ulong,
    pub r0: c_ulong,
    pub sp: c_ulong,
    pub orig_r0: c_ulong,
    pub ecr: ecr_reg,
}

#[cfg(CONFIG_ISA_ARCOMPACT)]
#[repr(C)]
pub struct callee_regs {
    pub r25: c_ulong, pub r24: c_ulong, pub r23: c_ulong, pub r22: c_ulong,
    pub r21: c_ulong, pub r20: c_ulong, pub r19: c_ulong, pub r18: c_ulong,
    pub r17: c_ulong, pub r16: c_ulong, pub r15: c_ulong, pub r14: c_ulong,
    pub r13: c_ulong,
}

#[cfg(CONFIG_ISA_ARCOMPACT)]
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, ecr);

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
#[repr(C)]
pub struct pt_regs {
    pub orig_r0: c_ulong,
    pub ecr: ecr_reg,
    pub bta: c_ulong,
    pub fp: c_ulong,
    pub r30: c_ulong,
    pub r12: c_ulong,
    pub r26: c_ulong,
    #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
    pub r58: c_ulong,
    #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
    pub r59: c_ulong,
    #[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)]
    pub DSP_CTRL: c_ulong,
    pub sp: c_ulong,
    pub r0: c_ulong, pub r1: c_ulong, pub r2: c_ulong, pub r3: c_ulong,
    pub r4: c_ulong, pub r5: c_ulong, pub r6: c_ulong, pub r7: c_ulong,
    pub r8: c_ulong, pub r9: c_ulong, pub r10: c_ulong, pub r11: c_ulong,
    pub blink: c_ulong,
    pub lp_end: c_ulong, pub lp_start: c_ulong, pub lp_count: c_ulong,
    pub ei: c_ulong, pub ldi: c_ulong, pub jli: c_ulong,
    pub ret: c_ulong,
    pub status32: c_ulong,
}

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
#[repr(C)]
pub struct callee_regs {
    pub r25: c_ulong, pub r24: c_ulong, pub r23: c_ulong, pub r22: c_ulong,
    pub r21: c_ulong, pub r20: c_ulong, pub r19: c_ulong, pub r18: c_ulong,
    pub r17: c_ulong, pub r16: c_ulong, pub r15: c_ulong, pub r14: c_ulong,
    pub r13: c_ulong,
}

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, status32);

pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> c_ulong { (*regs).ret }
pub unsafe fn profile_pc(regs: *mut pt_regs) -> c_ulong { instruction_pointer(regs) }
pub unsafe fn user_mode(regs: *mut pt_regs) -> c_ulong { (*regs).status32 & STATUS_U_MASK }
pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> c_uint {
    if user_mode(regs) != 0 { (*regs).sp as c_uint } else { c_uint::MAX }
}
pub unsafe fn delay_mode(regs: *mut pt_regs) -> bool {
    ((*regs).status32 & STATUS_DE_MASK) == STATUS_DE_MASK
}
pub const STATE_SCALL_RESTARTED: u8 = 0x01;

pub unsafe fn syscall_wont_restart(regs: *mut pt_regs) {
    (*regs).ecr.bits.state |= STATE_SCALL_RESTARTED;
}
pub unsafe fn syscall_restartable(regs: *mut pt_regs) -> bool {
    ((*regs).ecr.bits.state & STATE_SCALL_RESTARTED) == 0
}

pub unsafe fn regs_return_value(regs: *mut pt_regs) -> c_long { (*regs).r0 as c_long }
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: c_ulong) { (*regs).ret = val; }
pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> c_ulong { (*regs).sp }

extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
    pub fn regs_query_register_name(offset: c_uint) -> *const c_char;
    pub fn regs_within_kernel_stack(regs: *mut pt_regs, addr: c_ulong) -> bool;
    pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: c_uint) -> c_ulong;
    pub fn syscall_trace_enter(regs: *mut pt_regs);
    pub fn syscall_trace_exit(regs: *mut pt_regs);
}

pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: c_uint) -> c_ulong {
    if (offset as usize) > MAX_REG_OFFSET { return 0; }
    *((regs as *mut u8).add(offset as usize) as *mut c_ulong)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

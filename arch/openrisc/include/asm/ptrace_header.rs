/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Rust translation of the OpenRISC ptrace register definitions.
 * The original C header borrows liberally from similar architectural ports.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct PtRegsNamed {
    pub sr: isize,
    pub sp: isize,
    pub gpr2: isize,
    pub gpr3: isize,
    pub gpr4: isize,
    pub gpr5: isize,
    pub gpr6: isize,
    pub gpr7: isize,
    pub gpr8: isize,
    pub gpr9: isize,
    pub gpr10: isize,
    pub gpr11: isize,
    pub gpr12: isize,
    pub gpr13: isize,
    pub gpr14: isize,
    pub gpr15: isize,
    pub gpr16: isize,
    pub gpr17: isize,
    pub gpr18: isize,
    pub gpr19: isize,
    pub gpr20: isize,
    pub gpr21: isize,
    pub gpr22: isize,
    pub gpr23: isize,
    pub gpr24: isize,
    pub gpr25: isize,
    pub gpr26: isize,
    pub gpr27: isize,
    pub gpr28: isize,
    pub gpr29: isize,
    pub gpr30: isize,
    pub gpr31: isize,
}

#[repr(C)]
pub struct PtRegsOldStyle {
    pub offset: [isize; 2],
    pub gprs: [isize; 30],
}

#[repr(C)]
pub struct PtRegsNewStyle {
    pub gpr: [isize; 32],
}

#[repr(C)]
pub union PtRegsRegisters {
    pub named: PtRegsNamed,
    pub old_style: PtRegsOldStyle,
    pub new_style: PtRegsNewStyle,
}

#[repr(C)]
pub struct PtRegs {
    pub registers: PtRegsRegisters,
    pub pc: isize,
    /* For restarting system calls: syscall number, or -1 for other exceptions. */
    pub orig_gpr11: isize,
    pub dummy: isize,
    pub dummy2: isize,
}

/* TODO: Rename this to REDZONE because that is what it is. */
pub const STACK_FRAME_OVERHEAD: usize = 128;

/* Equivalent to offsetof(struct pt_regs, orig_gpr11). */
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(PtRegs, orig_gpr11);

/* Helpers for working with the instruction pointer. */
#[inline]
pub unsafe fn instruction_pointer(regs: *const PtRegs) -> usize {
    (*regs).pc as usize
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut PtRegs, val: usize) {
    (*regs).pc = val as isize;
}

/* The SPR_SR_SM dependency is supplied by the translated SPR definitions. */
#[inline]
pub unsafe fn user_mode(regs: *const PtRegs) -> bool {
    (((*regs).registers.named.sr as usize) & SPR_SR_SM) == 0
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const PtRegs) -> usize {
    (*regs).registers.named.sp as usize
}

#[inline]
pub unsafe fn profile_pc(regs: *const PtRegs) -> usize {
    instruction_pointer(regs)
}

/* Valid only for Kernel mode traps. */
#[inline]
pub unsafe fn kernel_stack_pointer(regs: *const PtRegs) -> usize {
    (*regs).registers.named.sp as usize
}

#[inline]
pub unsafe fn regs_return_value(regs: *const PtRegs) -> isize {
    (*regs).registers.new_style.gpr[11]
}

extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
    pub fn regs_get_kernel_stack_nth(regs: *const PtRegs, n: u32) -> usize;
}

#[inline]
pub unsafe fn regs_get_register(regs: *const PtRegs, offset: u32) -> usize {
    if offset as usize > MAX_REG_OFFSET {
        return 0;
    }

    *((regs as *const u8).add(offset as usize) as *const usize)
}

/* Offsets used by the ptrace system call interface. */
pub const PT_SR: usize = 0;
pub const PT_SP: usize = 4;
pub const PT_GPR2: usize = 8;
pub const PT_GPR3: usize = 12;
pub const PT_GPR4: usize = 16;
pub const PT_GPR5: usize = 20;
pub const PT_GPR6: usize = 24;
pub const PT_GPR7: usize = 28;
pub const PT_GPR8: usize = 32;
pub const PT_GPR9: usize = 36;
pub const PT_GPR10: usize = 40;
pub const PT_GPR11: usize = 44;
pub const PT_GPR12: usize = 48;
pub const PT_GPR13: usize = 52;
pub const PT_GPR14: usize = 56;
pub const PT_GPR15: usize = 60;
pub const PT_GPR16: usize = 64;
pub const PT_GPR17: usize = 68;
pub const PT_GPR18: usize = 72;
pub const PT_GPR19: usize = 76;
pub const PT_GPR20: usize = 80;
pub const PT_GPR21: usize = 84;
pub const PT_GPR22: usize = 88;
pub const PT_GPR23: usize = 92;
pub const PT_GPR24: usize = 96;
pub const PT_GPR25: usize = 100;
pub const PT_GPR26: usize = 104;
pub const PT_GPR27: usize = 108;
pub const PT_GPR28: usize = 112;
pub const PT_GPR29: usize = 116;
pub const PT_GPR30: usize = 120;
pub const PT_GPR31: usize = 124;
pub const PT_PC: usize = 128;
pub const PT_ORIG_GPR11: usize = 132;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

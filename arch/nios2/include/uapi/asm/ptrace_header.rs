/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * based on m68k asm/processor.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* The declarations below correspond to the non-assembler section of the C header. */

/*
 * Register numbers used by 'ptrace' system call interface.
 */

/* GP registers */
pub const PTR_R0: u32 = 0;
pub const PTR_R1: u32 = 1;
pub const PTR_R2: u32 = 2;
pub const PTR_R3: u32 = 3;
pub const PTR_R4: u32 = 4;
pub const PTR_R5: u32 = 5;
pub const PTR_R6: u32 = 6;
pub const PTR_R7: u32 = 7;
pub const PTR_R8: u32 = 8;
pub const PTR_R9: u32 = 9;
pub const PTR_R10: u32 = 10;
pub const PTR_R11: u32 = 11;
pub const PTR_R12: u32 = 12;
pub const PTR_R13: u32 = 13;
pub const PTR_R14: u32 = 14;
pub const PTR_R15: u32 = 15;
pub const PTR_R16: u32 = 16;
pub const PTR_R17: u32 = 17;
pub const PTR_R18: u32 = 18;
pub const PTR_R19: u32 = 19;
pub const PTR_R20: u32 = 20;
pub const PTR_R21: u32 = 21;
pub const PTR_R22: u32 = 22;
pub const PTR_R23: u32 = 23;
pub const PTR_R24: u32 = 24;
pub const PTR_R25: u32 = 25;
pub const PTR_GP: u32 = 26;
pub const PTR_SP: u32 = 27;
pub const PTR_FP: u32 = 28;
pub const PTR_EA: u32 = 29;
pub const PTR_BA: u32 = 30;
pub const PTR_RA: u32 = 31;

/* Control registers */
pub const PTR_PC: u32 = 32;
pub const PTR_STATUS: u32 = 33;
pub const PTR_ESTATUS: u32 = 34;
pub const PTR_BSTATUS: u32 = 35;
pub const PTR_IENABLE: u32 = 36;
pub const PTR_IPENDING: u32 = 37;
pub const PTR_CPUID: u32 = 38;
pub const PTR_CTL6: u32 = 39;
pub const PTR_EXCEPTION: u32 = 40;
pub const PTR_PTEADDR: u32 = 41;
pub const PTR_TLBACC: u32 = 42;
pub const PTR_TLBMISC: u32 = 43;
pub const PTR_ECCINJ: u32 = 44;
pub const PTR_BADADDR: u32 = 45;
pub const PTR_CONFIG: u32 = 46;
pub const PTR_MPUBASE: u32 = 47;
pub const PTR_MPUACC: u32 = 48;

pub const NUM_PTRACE_REG: u32 = PTR_MPUACC + 1;

/* User structures for general purpose registers.  */
#[repr(C)]
pub struct user_pt_regs {
    pub regs: [u32; 49],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Translated from the non-assembler portion of the C header. */

pub type microblaze_reg_t = usize;

#[repr(C)]
pub struct pt_regs {
    pub r0: microblaze_reg_t,
    pub r1: microblaze_reg_t,
    pub r2: microblaze_reg_t,
    pub r3: microblaze_reg_t,
    pub r4: microblaze_reg_t,
    pub r5: microblaze_reg_t,
    pub r6: microblaze_reg_t,
    pub r7: microblaze_reg_t,
    pub r8: microblaze_reg_t,
    pub r9: microblaze_reg_t,
    pub r10: microblaze_reg_t,
    pub r11: microblaze_reg_t,
    pub r12: microblaze_reg_t,
    pub r13: microblaze_reg_t,
    pub r14: microblaze_reg_t,
    pub r15: microblaze_reg_t,
    pub r16: microblaze_reg_t,
    pub r17: microblaze_reg_t,
    pub r18: microblaze_reg_t,
    pub r19: microblaze_reg_t,
    pub r20: microblaze_reg_t,
    pub r21: microblaze_reg_t,
    pub r22: microblaze_reg_t,
    pub r23: microblaze_reg_t,
    pub r24: microblaze_reg_t,
    pub r25: microblaze_reg_t,
    pub r26: microblaze_reg_t,
    pub r27: microblaze_reg_t,
    pub r28: microblaze_reg_t,
    pub r29: microblaze_reg_t,
    pub r30: microblaze_reg_t,
    pub r31: microblaze_reg_t,
    pub pc: microblaze_reg_t,
    pub msr: microblaze_reg_t,
    pub ear: microblaze_reg_t,
    pub esr: microblaze_reg_t,
    pub fsr: microblaze_reg_t,
    pub pt_mode: i32,
}

/* pt_regs offsets used by gdbserver etc in ptrace syscalls */
#[allow(non_snake_case)]
pub const fn PT_GPR(n: usize) -> usize {
    n * core::mem::size_of::<microblaze_reg_t>()
}

pub const PT_PC: usize = 32 * core::mem::size_of::<microblaze_reg_t>();
pub const PT_MSR: usize = 33 * core::mem::size_of::<microblaze_reg_t>();
pub const PT_EAR: usize = 34 * core::mem::size_of::<microblaze_reg_t>();
pub const PT_ESR: usize = 35 * core::mem::size_of::<microblaze_reg_t>();
pub const PT_FSR: usize = 36 * core::mem::size_of::<microblaze_reg_t>();
pub const PT_KERNEL_MODE: usize = 37 * core::mem::size_of::<microblaze_reg_t>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

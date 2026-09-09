/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions used by various bits of low-level assembly code on PowerPC.
 *
 * Copyright (C) 1995-1999 Gary Thomas, Paul Mackerras, Cort Dougan.
 */

/* Condition Register Bit Fields */
pub const cr0: i32 = 0;
pub const cr1: i32 = 1;
pub const cr2: i32 = 2;
pub const cr3: i32 = 3;
pub const cr4: i32 = 4;
pub const cr5: i32 = 5;
pub const cr6: i32 = 6;
pub const cr7: i32 = 7;

/* General Purpose Registers (GPRs) */
pub const r0: i32 = 0;
pub const r1: i32 = 1;
pub const r2: i32 = 2;
pub const r3: i32 = 3;
pub const r4: i32 = 4;
pub const r5: i32 = 5;
pub const r6: i32 = 6;
pub const r7: i32 = 7;
pub const r8: i32 = 8;
pub const r9: i32 = 9;
pub const r10: i32 = 10;
pub const r11: i32 = 11;
pub const r12: i32 = 12;
pub const r13: i32 = 13;
pub const r14: i32 = 14;
pub const r15: i32 = 15;
pub const r16: i32 = 16;
pub const r17: i32 = 17;
pub const r18: i32 = 18;
pub const r19: i32 = 19;
pub const r20: i32 = 20;
pub const r21: i32 = 21;
pub const r22: i32 = 22;
pub const r23: i32 = 23;
pub const r24: i32 = 24;
pub const r25: i32 = 25;
pub const r26: i32 = 26;
pub const r27: i32 = 27;
pub const r28: i32 = 28;
pub const r29: i32 = 29;
pub const r30: i32 = 30;
pub const r31: i32 = 31;

pub const SPRN_TBRL: i32 = 268;
pub const SPRN_TBRU: i32 = 269;
pub const SPRN_HSRR0: i32 = 0x13A; /* Hypervisor Save/Restore 0 */
pub const SPRN_HSRR1: i32 = 0x13B; /* Hypervisor Save/Restore 1 */

pub const MSR_LE: u64 = 0x0000_0000_0000_0001;

/*
 * FIXUP_ENDIAN is an assembly-only macro. Its exact instruction sequence is
 * retained here because Rust has no source-level equivalent for these
 * PowerPC assembler directives and instructions.
 *
 * tdi 0,0,0x48; b $+44; .long 0xa600607d; .long 0x01006b69;
 * .long 0x00004039; .long 0x6401417d; .long 0x05009f42;
 * .long 0xa602487d; .long 0x14004a39; .long 0xa6035a7d;
 * .long 0xa6037b7d; .long 0x2400004c
 */

/*
 * CONFIG_PPC_8xx selects:
 *   MFTBL(dest) -> mftb dest
 *   MFTBU(dest) -> mftbu dest
 * Otherwise:
 *   MFTBL(dest) -> mfspr dest, SPRN_TBRL
 *   MFTBU(dest) -> mfspr dest, SPRN_TBRU
 */

/*
 * CONFIG_PPC64_BOOT_WRAPPER selects:
 *   LOAD_REG_ADDR(reg, name) -> addis reg,r2,name@toc@ha; addi reg,reg,name@toc@l
 * Otherwise:
 *   LOAD_REG_ADDR(reg, name) -> lis reg,name@ha; addi reg,reg,name@l
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 1999 Hewlett-Packard (Frank Rowand)
 */

// General Registers
pub const rp: u8 = 2;
pub const arg3: u8 = 23;
pub const arg2: u8 = 24;
pub const arg1: u8 = 25;
pub const arg0: u8 = 26;
pub const dp: u8 = 27;
pub const ret0: u8 = 28;
pub const ret1: u8 = 29;
pub const sl: u8 = 29;
pub const sp: u8 = 30;

// PA20_REVISIT (disabled in the original source)
// pub const arg7: u8 = 19;
// pub const arg6: u8 = 20;
// pub const arg5: u8 = 21;
// pub const arg4: u8 = 22;
// pub const gp: u8 = 27;
// pub const ap: u8 = 29;

pub const r0: u8 = 0;
pub const r1: u8 = 1;
pub const r2: u8 = 2;
pub const r3: u8 = 3;
pub const r4: u8 = 4;
pub const r5: u8 = 5;
pub const r6: u8 = 6;
pub const r7: u8 = 7;
pub const r8: u8 = 8;
pub const r9: u8 = 9;
pub const r10: u8 = 10;
pub const r11: u8 = 11;
pub const r12: u8 = 12;
pub const r13: u8 = 13;
pub const r14: u8 = 14;
pub const r15: u8 = 15;
pub const r16: u8 = 16;
pub const r17: u8 = 17;
pub const r18: u8 = 18;
pub const r19: u8 = 19;
pub const r20: u8 = 20;
pub const r21: u8 = 21;
pub const r22: u8 = 22;
pub const r23: u8 = 23;
pub const r24: u8 = 24;
pub const r25: u8 = 25;
pub const r26: u8 = 26;
pub const r27: u8 = 27;
pub const r28: u8 = 28;
pub const r29: u8 = 29;
pub const r30: u8 = 30;
pub const r31: u8 = 31;

// Space Registers
pub const sr0: u8 = 0;
pub const sr1: u8 = 1;
pub const sr2: u8 = 2;
pub const sr3: u8 = 3;
pub const sr4: u8 = 4;
pub const sr5: u8 = 5;
pub const sr6: u8 = 6;
pub const sr7: u8 = 7;

// Floating Point Registers
pub const fr0: u8 = 0;
pub const fr1: u8 = 1;
pub const fr2: u8 = 2;
pub const fr3: u8 = 3;
pub const fr4: u8 = 4;
pub const fr5: u8 = 5;
pub const fr6: u8 = 6;
pub const fr7: u8 = 7;
pub const fr8: u8 = 8;
pub const fr9: u8 = 9;
pub const fr10: u8 = 10;
pub const fr11: u8 = 11;
pub const fr12: u8 = 12;
pub const fr13: u8 = 13;
pub const fr14: u8 = 14;
pub const fr15: u8 = 15;
pub const fr16: u8 = 16;
pub const fr17: u8 = 17;
pub const fr18: u8 = 18;
pub const fr19: u8 = 19;
pub const fr20: u8 = 20;
pub const fr21: u8 = 21;
pub const fr22: u8 = 22;
pub const fr23: u8 = 23;
pub const fr24: u8 = 24;
pub const fr25: u8 = 25;
pub const fr26: u8 = 26;
pub const fr27: u8 = 27;
pub const fr28: u8 = 28;
pub const fr29: u8 = 29;
pub const fr30: u8 = 30;
pub const fr31: u8 = 31;

// Control Registers
pub const rctr: u8 = 0;
pub const pidr1: u8 = 8;
pub const pidr2: u8 = 9;
pub const ccr: u8 = 10;
pub const sar: u8 = 11;
pub const pidr3: u8 = 12;
pub const pidr4: u8 = 13;
pub const iva: u8 = 14;
pub const eiem: u8 = 15;
pub const itmr: u8 = 16;
pub const pcsq: u8 = 17;
pub const pcoq: u8 = 18;
pub const iir: u8 = 19;
pub const isr: u8 = 20;
pub const ior: u8 = 21;
pub const ipsw: u8 = 22;
pub const eirr: u8 = 23;
pub const tr0: u8 = 24;
pub const tr1: u8 = 25;
pub const tr2: u8 = 26;
pub const tr3: u8 = 27;
pub const tr4: u8 = 28;
pub const tr5: u8 = 29;
pub const tr6: u8 = 30;
pub const tr7: u8 = 31;

pub const cr0: u8 = 0;
pub const cr8: u8 = 8;
pub const cr9: u8 = 9;
pub const cr10: u8 = 10;
pub const cr11: u8 = 11;
pub const cr12: u8 = 12;
pub const cr13: u8 = 13;
pub const cr14: u8 = 14;
pub const cr15: u8 = 15;
pub const cr16: u8 = 16;
pub const cr17: u8 = 17;
pub const cr18: u8 = 18;
pub const cr19: u8 = 19;
pub const cr20: u8 = 20;
pub const cr21: u8 = 21;
pub const cr22: u8 = 22;
pub const cr23: u8 = 23;
pub const cr24: u8 = 24;
pub const cr25: u8 = 25;
pub const cr26: u8 = 26;
pub const cr27: u8 = 27;
pub const cr28: u8 = 28;
pub const cr29: u8 = 29;
pub const cr30: u8 = 30;
pub const cr31: u8 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

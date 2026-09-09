/*
 * Copyright (c) 2006 Tensilica, Inc.  All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2.1 of the GNU Lesser General Public
 * License as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *
 * Further, this software is distributed without any warranty of the
 * rightful claim of any third person regarding infringement or the like.
 * Any license provided herein, whether implied or otherwise, applies only
 * to this software file. Patent licenses, if any, provided herein do not
 * apply to combinations of this program with other software, or any other
 * product whatsoever.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston MA 02111-1307,
 * USA.
 */

/*  Special registers.  */

pub const SREG_MR: u32 = 32;
pub const SREG_IBREAKENABLE: u32 = 96;
pub const SREG_IBREAKA: u32 = 128;
pub const SREG_DBREAKA: u32 = 144;
pub const SREG_DBREAKC: u32 = 160;
pub const SREG_EPC: u32 = 176;
pub const SREG_EPS: u32 = 192;
pub const SREG_EXCSAVE: u32 = 208;
pub const SREG_CCOMPARE: u32 = 240;
pub const SREG_MISC: u32 = 244;

/*  EXCCAUSE register fields  */

pub const EXCCAUSE_EXCCAUSE_SHIFT: u32 = 0;
pub const EXCCAUSE_EXCCAUSE_MASK: u32 = 0x3F;

pub const EXCCAUSE_ILLEGAL_INSTRUCTION: u32 = 0;
pub const EXCCAUSE_SYSTEM_CALL: u32 = 1;
pub const EXCCAUSE_INSTRUCTION_FETCH_ERROR: u32 = 2;
pub const EXCCAUSE_LOAD_STORE_ERROR: u32 = 3;
pub const EXCCAUSE_LEVEL1_INTERRUPT: u32 = 4;
pub const EXCCAUSE_ALLOCA: u32 = 5;
pub const EXCCAUSE_INTEGER_DIVIDE_BY_ZERO: u32 = 6;
pub const EXCCAUSE_SPECULATION: u32 = 7;
pub const EXCCAUSE_PRIVILEGED: u32 = 8;
pub const EXCCAUSE_UNALIGNED: u32 = 9;
pub const EXCCAUSE_INSTR_DATA_ERROR: u32 = 12;
pub const EXCCAUSE_LOAD_STORE_DATA_ERROR: u32 = 13;
pub const EXCCAUSE_INSTR_ADDR_ERROR: u32 = 14;
pub const EXCCAUSE_LOAD_STORE_ADDR_ERROR: u32 = 15;
pub const EXCCAUSE_ITLB_MISS: u32 = 16;
pub const EXCCAUSE_ITLB_MULTIHIT: u32 = 17;
pub const EXCCAUSE_ITLB_PRIVILEGE: u32 = 18;
pub const EXCCAUSE_ITLB_SIZE_RESTRICTION: u32 = 19;
pub const EXCCAUSE_FETCH_CACHE_ATTRIBUTE: u32 = 20;
pub const EXCCAUSE_DTLB_MISS: u32 = 24;
pub const EXCCAUSE_DTLB_MULTIHIT: u32 = 25;
pub const EXCCAUSE_DTLB_PRIVILEGE: u32 = 26;
pub const EXCCAUSE_DTLB_SIZE_RESTRICTION: u32 = 27;
pub const EXCCAUSE_LOAD_CACHE_ATTRIBUTE: u32 = 28;
pub const EXCCAUSE_STORE_CACHE_ATTRIBUTE: u32 = 29;
pub const EXCCAUSE_COPROCESSOR0_DISABLED: u32 = 32;
pub const EXCCAUSE_COPROCESSOR1_DISABLED: u32 = 33;
pub const EXCCAUSE_COPROCESSOR2_DISABLED: u32 = 34;
pub const EXCCAUSE_COPROCESSOR3_DISABLED: u32 = 35;
pub const EXCCAUSE_COPROCESSOR4_DISABLED: u32 = 36;
pub const EXCCAUSE_COPROCESSOR5_DISABLED: u32 = 37;
pub const EXCCAUSE_COPROCESSOR6_DISABLED: u32 = 38;
pub const EXCCAUSE_COPROCESSOR7_DISABLED: u32 = 39;
pub const EXCCAUSE_N: u32 = 64;

/*  PS register fields.  */

pub const PS_WOE_BIT: u32 = 18;
pub const PS_WOE_MASK: u32 = 0x00040000;
pub const PS_CALLINC_SHIFT: u32 = 16;
pub const PS_CALLINC_MASK: u32 = 0x00030000;
pub const PS_OWB_SHIFT: u32 = 8;
pub const PS_OWB_WIDTH: u32 = 4;
pub const PS_OWB_MASK: u32 = 0x00000F00;
pub const PS_RING_SHIFT: u32 = 6;
pub const PS_RING_MASK: u32 = 0x000000C0;
pub const PS_UM_BIT: u32 = 5;
pub const PS_EXCM_BIT: u32 = 4;
pub const PS_INTLEVEL_SHIFT: u32 = 0;
pub const PS_INTLEVEL_WIDTH: u32 = 4;
pub const PS_INTLEVEL_MASK: u32 = 0x0000000F;

/*  DBREAKCn register fields.  */

pub const DBREAKC_MASK_BIT: u32 = 0;
pub const DBREAKC_MASK_MASK: u32 = 0x0000003F;
pub const DBREAKC_LOAD_BIT: u32 = 30;
pub const DBREAKC_LOAD_MASK: u32 = 0x40000000;
pub const DBREAKC_STOR_BIT: u32 = 31;
pub const DBREAKC_STOR_MASK: u32 = 0x80000000;

/*  DEBUGCAUSE register fields.  */

pub const DEBUGCAUSE_DBNUM_MASK: u32 = 0xf00;
pub const DEBUGCAUSE_DBNUM_SHIFT: u32 = 8; /* First bit of DBNUM field */
pub const DEBUGCAUSE_DEBUGINT_BIT: u32 = 5; /* External debug interrupt */
pub const DEBUGCAUSE_BREAKN_BIT: u32 = 4; /* BREAK.N instruction */
pub const DEBUGCAUSE_BREAK_BIT: u32 = 3; /* BREAK instruction */
pub const DEBUGCAUSE_DBREAK_BIT: u32 = 2; /* DBREAK match */
pub const DEBUGCAUSE_IBREAK_BIT: u32 = 1; /* IBREAK match */
pub const DEBUGCAUSE_ICOUNT_BIT: u32 = 0; /* ICOUNT would incr. to zero */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

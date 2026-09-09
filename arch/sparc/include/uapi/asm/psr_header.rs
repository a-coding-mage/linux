/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * psr.h: This file holds the macros for masking off various parts of
 *        the processor status register on the Sparc. This is valid
 *        for Version 8. On the V9 this is renamed to the PSTATE
 *        register and its members are accessed as fields like
 *        PSTATE.PRIV for the current CPU privilege level.
 *
 * Copyright (C) 1994 David S. Miller (davem@caip.rutgers.edu)
 */

/* The Sparc PSR fields are laid out as the following:
 *
 *  ------------------------------------------------------------------------
 *  | impl  | vers  | icc   | resv  | EC | EF | PIL  | S | PS | ET |  CWP  |
 *  | 31-28 | 27-24 | 23-20 | 19-14 | 13 | 12 | 11-8 | 7 | 6  | 5  |  4-0  |
 *  ------------------------------------------------------------------------
 */
pub const PSR_CWP: u32 = 0x0000001f; // current window pointer
pub const PSR_ET: u32 = 0x00000020; // enable traps field
pub const PSR_PS: u32 = 0x00000040; // previous privilege level
pub const PSR_S: u32 = 0x00000080; // current privilege level
pub const PSR_PIL: u32 = 0x00000f00; // processor interrupt level
pub const PSR_EF: u32 = 0x00001000; // enable floating point
pub const PSR_EC: u32 = 0x00002000; // enable co-processor
pub const PSR_SYSCALL: u32 = 0x00004000; // inside of a syscall
pub const PSR_LE: u32 = 0x00008000; // SuperSparcII little-endian
pub const PSR_ICC: u32 = 0x00f00000; // integer condition codes
pub const PSR_C: u32 = 0x00100000; // carry bit
pub const PSR_V: u32 = 0x00200000; // overflow bit
pub const PSR_Z: u32 = 0x00400000; // zero bit
pub const PSR_N: u32 = 0x00800000; // negative bit
pub const PSR_VERS: u32 = 0x0f000000; // cpu-version field
pub const PSR_IMPL: u32 = 0xf0000000; // cpu-implementation field

pub const PSR_VERS_SHIFT: u32 = 24;
pub const PSR_IMPL_SHIFT: u32 = 28;
pub const PSR_VERS_SHIFTED_MASK: u32 = 0xf;
pub const PSR_IMPL_SHIFTED_MASK: u32 = 0xf;

pub const PSR_IMPL_TI: u32 = 0x4;
pub const PSR_IMPL_LEON: u32 = 0xf;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

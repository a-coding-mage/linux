/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm/utrap.h
 *
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// The original declarations are excluded when compiling as an assembler
// source file (__ASSEMBLER__). Rust has no equivalent assembler preprocessor
// mode here, so these declarations are provided for Rust translation units.

pub const UT_INSTRUCTION_EXCEPTION: i32 = 1;
pub const UT_INSTRUCTION_ERROR: i32 = 2;
pub const UT_INSTRUCTION_PROTECTION: i32 = 3;
pub const UT_ILLTRAP_INSTRUCTION: i32 = 4;
pub const UT_ILLEGAL_INSTRUCTION: i32 = 5;
pub const UT_PRIVILEGED_OPCODE: i32 = 6;
pub const UT_FP_DISABLED: i32 = 7;
pub const UT_FP_EXCEPTION_IEEE_754: i32 = 8;
pub const UT_FP_EXCEPTION_OTHER: i32 = 9;
pub const UT_TAG_OVERVIEW: i32 = 10;
pub const UT_DIVISION_BY_ZERO: i32 = 11;
pub const UT_DATA_EXCEPTION: i32 = 12;
pub const UT_DATA_ERROR: i32 = 13;
pub const UT_DATA_PROTECTION: i32 = 14;
pub const UT_MEM_ADDRESS_NOT_ALIGNED: i32 = 15;
pub const UT_PRIVILEGED_ACTION: i32 = 16;
pub const UT_ASYNC_DATA_ERROR: i32 = 17;
pub const UT_TRAP_INSTRUCTION_16: i32 = 18;
pub const UT_TRAP_INSTRUCTION_17: i32 = 19;
pub const UT_TRAP_INSTRUCTION_18: i32 = 20;
pub const UT_TRAP_INSTRUCTION_19: i32 = 21;
pub const UT_TRAP_INSTRUCTION_20: i32 = 22;
pub const UT_TRAP_INSTRUCTION_21: i32 = 23;
pub const UT_TRAP_INSTRUCTION_22: i32 = 24;
pub const UT_TRAP_INSTRUCTION_23: i32 = 25;
pub const UT_TRAP_INSTRUCTION_24: i32 = 26;
pub const UT_TRAP_INSTRUCTION_25: i32 = 27;
pub const UT_TRAP_INSTRUCTION_26: i32 = 28;
pub const UT_TRAP_INSTRUCTION_27: i32 = 29;
pub const UT_TRAP_INSTRUCTION_28: i32 = 30;
pub const UT_TRAP_INSTRUCTION_29: i32 = 31;
pub const UT_TRAP_INSTRUCTION_30: i32 = 32;
pub const UT_TRAP_INSTRUCTION_31: i32 = 33;

pub const UTH_NOCHANGE: i32 = -1;

pub type utrap_entry_t = i32;
pub type utrap_handler_t = *mut core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 1999, 2000  Niibe Yutaka
 */

// Translated from the UAPI SH ptrace header.
// The original header includes <asm/ptrace_32.h>; its declarations are
// supplied by the corresponding Rust dependency.

pub const PTRACE_GETREGS: i32 = 12; // General registers
pub const PTRACE_SETREGS: i32 = 13;

pub const PTRACE_GETFPREGS: i32 = 14; // FPU registers
pub const PTRACE_SETFPREGS: i32 = 15;

pub const PTRACE_GETFDPIC: i32 = 31; // get the ELF fdpic loadmap address

pub const PTRACE_GETFDPIC_EXEC: i32 = 0; // [addr] request the executable loadmap
pub const PTRACE_GETFDPIC_INTERP: i32 = 1; // [addr] request the interpreter loadmap

pub const PTRACE_GETDSPREGS: i32 = 55; // DSP registers
pub const PTRACE_SETDSPREGS: i32 = 56;

pub const PT_TEXT_END_ADDR: i32 = 240;
pub const PT_TEXT_ADDR: i32 = 244; // &(struct user)->start_code
pub const PT_DATA_ADDR: i32 = 248; // &(struct user)->start_data
pub const PT_TEXT_LEN: i32 = 252;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

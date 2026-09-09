/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 *
 * Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

pub const PTRACE_GET_THREAD_AREA: ::core::ffi::c_int = 25;

/* The declarations below are omitted when this header is consumed by an assembler. */
/*
 * Userspace ABI: Register state needed by
 *  -ptrace (gdbserver)
 *  -sigcontext (SA_SIGNINFO signal frame)
 *
 * This is to decouple pt_regs from user-space ABI, to be able to change it
 * w/o affecting the ABI.
 *
 * The intermediate pad,pad2 are relics of initial layout based on pt_regs
 * for optimizations when copying pt_regs to/from user_regs_struct.
 * We no longer need them, but can't be changed as they are part of ABI now.
 *
 * Also, sigcontext only care about the scratch regs as that is what we really
 * save/restore for signal handling. However gdb also uses the same struct
 * hence callee regs need to be in there too.
 */
#[repr(C)]
pub struct user_regs_struct {
    pub pad: ::core::ffi::c_ulong,
    pub scratch: user_regs_struct_scratch,
    pub pad2: ::core::ffi::c_ulong,
    pub callee: user_regs_struct_callee,
    pub efa: ::core::ffi::c_ulong, /* break pt addr, for break points in delay slots */
    pub stop_pc: ::core::ffi::c_ulong, /* give dbg stop_pc after ensuring brkpt trap */
}

#[repr(C)]
pub struct user_regs_struct_scratch {
    pub bta: ::core::ffi::c_ulong,
    pub lp_start: ::core::ffi::c_ulong,
    pub lp_end: ::core::ffi::c_ulong,
    pub lp_count: ::core::ffi::c_ulong,
    pub status32: ::core::ffi::c_ulong,
    pub ret: ::core::ffi::c_ulong,
    pub blink: ::core::ffi::c_ulong,
    pub fp: ::core::ffi::c_ulong,
    pub gp: ::core::ffi::c_ulong,
    pub r12: ::core::ffi::c_ulong,
    pub r11: ::core::ffi::c_ulong,
    pub r10: ::core::ffi::c_ulong,
    pub r9: ::core::ffi::c_ulong,
    pub r8: ::core::ffi::c_ulong,
    pub r7: ::core::ffi::c_ulong,
    pub r6: ::core::ffi::c_ulong,
    pub r5: ::core::ffi::c_ulong,
    pub r4: ::core::ffi::c_ulong,
    pub r3: ::core::ffi::c_ulong,
    pub r2: ::core::ffi::c_ulong,
    pub r1: ::core::ffi::c_ulong,
    pub r0: ::core::ffi::c_ulong,
    pub sp: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct user_regs_struct_callee {
    pub r25: ::core::ffi::c_ulong,
    pub r24: ::core::ffi::c_ulong,
    pub r23: ::core::ffi::c_ulong,
    pub r22: ::core::ffi::c_ulong,
    pub r21: ::core::ffi::c_ulong,
    pub r20: ::core::ffi::c_ulong,
    pub r19: ::core::ffi::c_ulong,
    pub r18: ::core::ffi::c_ulong,
    pub r17: ::core::ffi::c_ulong,
    pub r16: ::core::ffi::c_ulong,
    pub r15: ::core::ffi::c_ulong,
    pub r14: ::core::ffi::c_ulong,
    pub r13: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct user_regs_arcv2 {
    pub r30: ::core::ffi::c_ulong,
    pub r58: ::core::ffi::c_ulong,
    pub r59: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

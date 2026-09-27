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

pub const PTRACE_GET_THREAD_AREA: ::kernel::ffi::c_int = 25;

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
    pub pad: ::kernel::ffi::c_ulong,
    pub scratch: user_regs_struct_scratch,
    pub pad2: ::kernel::ffi::c_ulong,
    pub callee: user_regs_struct_callee,
    pub efa: ::kernel::ffi::c_ulong, /* break pt addr, for break points in delay slots */
    pub stop_pc: ::kernel::ffi::c_ulong, /* give dbg stop_pc after ensuring brkpt trap */
}

#[repr(C)]
pub struct user_regs_struct_scratch {
    pub bta: ::kernel::ffi::c_ulong,
    pub lp_start: ::kernel::ffi::c_ulong,
    pub lp_end: ::kernel::ffi::c_ulong,
    pub lp_count: ::kernel::ffi::c_ulong,
    pub status32: ::kernel::ffi::c_ulong,
    pub ret: ::kernel::ffi::c_ulong,
    pub blink: ::kernel::ffi::c_ulong,
    pub fp: ::kernel::ffi::c_ulong,
    pub gp: ::kernel::ffi::c_ulong,
    pub r12: ::kernel::ffi::c_ulong,
    pub r11: ::kernel::ffi::c_ulong,
    pub r10: ::kernel::ffi::c_ulong,
    pub r9: ::kernel::ffi::c_ulong,
    pub r8: ::kernel::ffi::c_ulong,
    pub r7: ::kernel::ffi::c_ulong,
    pub r6: ::kernel::ffi::c_ulong,
    pub r5: ::kernel::ffi::c_ulong,
    pub r4: ::kernel::ffi::c_ulong,
    pub r3: ::kernel::ffi::c_ulong,
    pub r2: ::kernel::ffi::c_ulong,
    pub r1: ::kernel::ffi::c_ulong,
    pub r0: ::kernel::ffi::c_ulong,
    pub sp: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct user_regs_struct_callee {
    pub r25: ::kernel::ffi::c_ulong,
    pub r24: ::kernel::ffi::c_ulong,
    pub r23: ::kernel::ffi::c_ulong,
    pub r22: ::kernel::ffi::c_ulong,
    pub r21: ::kernel::ffi::c_ulong,
    pub r20: ::kernel::ffi::c_ulong,
    pub r19: ::kernel::ffi::c_ulong,
    pub r18: ::kernel::ffi::c_ulong,
    pub r17: ::kernel::ffi::c_ulong,
    pub r16: ::kernel::ffi::c_ulong,
    pub r15: ::kernel::ffi::c_ulong,
    pub r14: ::kernel::ffi::c_ulong,
    pub r13: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct user_regs_arcv2 {
    pub r30: ::kernel::ffi::c_ulong,
    pub r58: ::kernel::ffi::c_ulong,
    pub r59: ::kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

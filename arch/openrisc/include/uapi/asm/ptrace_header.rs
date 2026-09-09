/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

/* C header guard: _UAPI__ASM_OPENRISC_PTRACE_H */

/*
 * The declarations below are omitted when compiling as an assembler
 * (__ASSEMBLER__).
 *
 * This is the layout of the regset returned by the GETREGSET ptrace call
 */
#[repr(C)]
pub struct user_regs_struct {
    /* GPR R0-R31... */
    pub gpr: [core::ffi::c_ulong; 32],
    pub pc: core::ffi::c_ulong,
    pub sr: core::ffi::c_ulong,
}

#[repr(C)]
pub struct __or1k_fpu_state {
    pub fpcsr: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

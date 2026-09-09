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

// The C header guard and dependency include are omitted; `user_regs_struct`
// is supplied by the corresponding ptrace definitions.

/* This struct is saved by setup_frame in signal.c, to keep the current
   context while a signal handler is executed. It's restored by sys_sigreturn.
*/

#[repr(C)]
pub union sigcontext__fpcsr_or_oldmask {
    pub fpcsr: ::core::ffi::c_ulong,
    pub oldmask: ::core::ffi::c_ulong, /* unused */
}

#[repr(C)]
pub struct sigcontext {
    pub regs: user_regs_struct, /* needs to be first */
    pub __bindgen_anon_1: sigcontext__fpcsr_or_oldmask,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

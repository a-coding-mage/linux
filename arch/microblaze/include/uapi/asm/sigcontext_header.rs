/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* FIXME should be linux/ptrace.h */
/* Dependency: pt_regs is supplied by the translated asm/ptrace definitions. */

#[repr(C)]
pub struct sigcontext {
    pub regs: pt_regs,
    pub oldmask: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

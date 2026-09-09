/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/sigcontext.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2007 Tensilica Inc.
 */

#[repr(C)]
pub struct sigcontext {
    pub sc_pc: ::core::ffi::c_ulong,
    pub sc_ps: ::core::ffi::c_ulong,
    pub sc_lbeg: ::core::ffi::c_ulong,
    pub sc_lend: ::core::ffi::c_ulong,
    pub sc_lcount: ::core::ffi::c_ulong,
    pub sc_sar: ::core::ffi::c_ulong,
    pub sc_acclo: ::core::ffi::c_ulong,
    pub sc_acchi: ::core::ffi::c_ulong,
    pub sc_a: [::core::ffi::c_ulong; 16],
    pub sc_xtregs: *mut ::core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

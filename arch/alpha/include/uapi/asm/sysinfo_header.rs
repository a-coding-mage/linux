/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-alpha/sysinfo.h
 */

/* This defines the subset of the OSF/1 getsysinfo/setsysinfo calls
   that we support.  */

pub const GSI_UACPROC: i32 = 8;
pub const GSI_IEEE_FP_CONTROL: i32 = 45;
pub const GSI_IEEE_STATE_AT_SIGNAL: i32 = 46;
pub const GSI_PROC_TYPE: i32 = 60;
pub const GSI_GET_HWRPB: i32 = 101;

pub const SSI_NVPAIRS: i32 = 1;
pub const SSI_LMF: i32 = 7;
pub const SSI_IEEE_FP_CONTROL: i32 = 14;
pub const SSI_IEEE_STATE_AT_SIGNAL: i32 = 15;
pub const SSI_IEEE_IGNORE_STATE_AT_SIGNAL: i32 = 16;
pub const SSI_IEEE_RAISE_EXCEPTION: i32 = 1001; /* linux specific */

pub const SSIN_UACPROC: i32 = 6;

pub const UAC_BITMASK: i32 = 7;
pub const UAC_NOPRINT: i32 = 1;
pub const UAC_NOFIX: i32 = 2;
pub const UAC_SIGBUS: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

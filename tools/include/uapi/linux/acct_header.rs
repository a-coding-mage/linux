/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  BSD Process Accounting for Linux - Definitions
 *
 *  Author: Marco van Wieringen (mvw@planets.elm.net)
 *
 *  This header file contains the definitions needed to implement
 *  BSD-style process accounting. The kernel accounting code and all
 *  user-level programs that try to do something useful with the
 *  process accounting log must include this file.
 *
 *  Copyright (C) 1995 - 1997 Marco van Wieringen - ELM Consultancy B.V.
 *
 */

// Dependencies from the original C header:
// #include <linux/types.h>
// #include <asm/param.h>
// #include <asm/byteorder.h>

/*
 *  comp_t is a 16-bit "floating" point number with a 3-bit base 8
 *  exponent and a 13-bit fraction.
 *  comp2_t is 24-bit with 5-bit base 2 exponent and 20 bit fraction
 *  (leading 1 not stored).
 *  See linux/kernel/acct.c for the specific encoding systems used.
 */

pub type comp_t = __u16;
pub type comp2_t = __u32;

/*
 *   accounting file record
 *
 *   This structure contains all of the information written out to the
 *   process accounting file whenever a process exits.
 */

pub const ACCT_COMM: usize = 16;

#[repr(C)]
pub struct acct {
    pub ac_flag: core::ffi::c_char,             /* Flags */
    pub ac_version: core::ffi::c_char,          /* Always set to ACCT_VERSION */
    /* for binary compatibility back until 2.0 */
    pub ac_uid16: __u16,                        /* LSB of Real User ID */
    pub ac_gid16: __u16,                        /* LSB of Real Group ID */
    pub ac_tty: __u16,                          /* Control Terminal */
    /* __u32 range means times from 1970 to 2106 */
    pub ac_btime: __u32,                        /* Process Creation Time */
    pub ac_utime: comp_t,                       /* User Time */
    pub ac_stime: comp_t,                       /* System Time */
    pub ac_etime: comp_t,                       /* Elapsed Time */
    pub ac_mem: comp_t,                         /* Average Memory Usage */
    pub ac_io: comp_t,                          /* Chars Transferred */
    pub ac_rw: comp_t,                          /* Blocks Read or Written */
    pub ac_minflt: comp_t,                      /* Minor Pagefaults */
    pub ac_majflt: comp_t,                      /* Major Pagefaults */
    pub ac_swaps: comp_t,                       /* Number of Swaps */
    /* m68k had no padding here. */
    /* Present when !defined(CONFIG_M68K) || !defined(__KERNEL__). */
    pub ac_ahz: __u16,                          /* AHZ */
    pub ac_exitcode: __u32,                     /* Exitcode */
    pub ac_comm: [core::ffi::c_char; ACCT_COMM + 1], /* Command Name */
    pub ac_etime_hi: __u8,                      /* Elapsed Time MSB */
    pub ac_etime_lo: __u16,                     /* Elapsed Time LSB */
    pub ac_uid: __u32,                          /* Real User ID */
    pub ac_gid: __u32,                          /* Real Group ID */
}

#[repr(C)]
pub struct acct_v3 {
    pub ac_flag: core::ffi::c_char,             /* Flags */
    pub ac_version: core::ffi::c_char,          /* Always set to ACCT_VERSION */
    pub ac_tty: __u16,                          /* Control Terminal */
    pub ac_exitcode: __u32,                     /* Exitcode */
    pub ac_uid: __u32,                          /* Real User ID */
    pub ac_gid: __u32,                          /* Real Group ID */
    pub ac_pid: __u32,                          /* Process ID */
    pub ac_ppid: __u32,                         /* Parent Process ID */
    /* __u32 range means times from 1970 to 2106 */
    pub ac_btime: __u32,                        /* Process Creation Time */
    /* In C this is __u32 under __KERNEL__, otherwise float. */
    pub ac_etime: f32,                          /* Elapsed Time */
    pub ac_utime: comp_t,                       /* User Time */
    pub ac_stime: comp_t,                       /* System Time */
    pub ac_mem: comp_t,                         /* Average Memory Usage */
    pub ac_io: comp_t,                          /* Chars Transferred */
    pub ac_rw: comp_t,                          /* Blocks Read or Written */
    pub ac_minflt: comp_t,                      /* Minor Pagefaults */
    pub ac_majflt: comp_t,                      /* Major Pagefaults */
    pub ac_swaps: comp_t,                       /* Number of Swaps */
    pub ac_comm: [core::ffi::c_char; ACCT_COMM], /* Command Name */
}

/*
 *  accounting flags
 */
                                /* bit set when the process/task ... */
pub const AFORK: u32 = 0x01;     /* ... executed fork, but did not exec */
pub const ASU: u32 = 0x02;       /* ... used super-user privileges */
pub const ACOMPAT: u32 = 0x04;   /* ... used compatibility mode (VAX only not used) */
pub const ACORE: u32 = 0x08;     /* ... dumped core */
pub const AXSIG: u32 = 0x10;     /* ... was killed by a signal */
pub const AGROUP: u32 = 0x20;    /* ... was the last task of the process (task group) */

#[cfg(target_endian = "big")]
pub const ACCT_BYTEORDER: u32 = 0x80; /* accounting file is big endian */

#[cfg(target_endian = "little")]
pub const ACCT_BYTEORDER: u32 = 0x00; /* accounting file is little endian */

// The original C header defines these only when !defined(__KERNEL__).
pub const ACCT_VERSION: u32 = 2;
pub const AHZ: usize = HZ;

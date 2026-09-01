/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * POWER Dynamic Execution Control Facility (DEXCR)
 *
 * This header file contains helper functions and macros
 * required for all the DEXCR related test cases.
 */

use core::ffi::{c_char, c_void};

// C dependencies removed from executable Rust:
// <stdbool.h>, <sys/prctl.h>, <sys/types.h>, and "reg.h".
// The PR_PPC_DEXCR_* constants, pid_t type, and __MASK-equivalent bit mask
// helper are expected to be supplied by surrounding translated code.

#[allow(non_camel_case_types)]
pub type pid_t = i32;

extern "C" {
    pub static PR_PPC_DEXCR_SBHE: u64;
    pub static PR_PPC_DEXCR_IBRTPD: u64;
    pub static PR_PPC_DEXCR_SRAPD: u64;
    pub static PR_PPC_DEXCR_NPHIE: u64;
}

#[inline]
pub const fn DEXCR_PR_BIT(aspect: u32) -> u64 {
    1u64 << (63 - (32 + aspect))
}

pub const DEXCR_PR_SBHE: u64 = DEXCR_PR_BIT(0);
pub const DEXCR_PR_IBRTPD: u64 = DEXCR_PR_BIT(3);
pub const DEXCR_PR_SRAPD: u64 = DEXCR_PR_BIT(4);
pub const DEXCR_PR_NPHIE: u64 = DEXCR_PR_BIT(5);

#[inline]
pub const fn PPC_RAW_HASH_ARGS(b: u32, i: u32, a: u32) -> u32 {
    ((((i >> 3) & 0x1F) << 21) | (a << 16) | (b << 11) | ((i >> 8) & 0x1))
}

#[inline]
pub const fn PPC_RAW_HASHST(b: u32, i: u32, a: u32) -> u32 {
    0x7C0005A4 | PPC_RAW_HASH_ARGS(b, i, a)
}

#[inline]
pub const fn PPC_RAW_HASHCHK(b: u32, i: u32, a: u32) -> u32 {
    0x7C0005E4 | PPC_RAW_HASH_ARGS(b, i, a)
}

#[repr(C)]
pub struct dexcr_aspect {
    pub name: *const c_char, /* Short display name */
    pub opt: *const c_char,  /* Option name for chdexcr */
    pub desc: *const c_char, /* Expanded aspect meaning */
    pub index: u32,          /* Aspect bit index in DEXCR */
    pub prctl: u64,          /* 'which' value for get/set prctl */
}

pub static aspects: [dexcr_aspect; 5] = [
    dexcr_aspect {
        name: b"SBHE\0".as_ptr() as *const c_char,
        opt: b"sbhe\0".as_ptr() as *const c_char,
        desc: b"Speculative branch hint enable\0".as_ptr() as *const c_char,
        index: 0,
        prctl: unsafe { PR_PPC_DEXCR_SBHE },
    },
    dexcr_aspect {
        name: b"IBRTPD\0".as_ptr() as *const c_char,
        opt: b"ibrtpd\0".as_ptr() as *const c_char,
        desc: b"Indirect branch recurrent target prediction disable\0".as_ptr() as *const c_char,
        index: 3,
        prctl: unsafe { PR_PPC_DEXCR_IBRTPD },
    },
    dexcr_aspect {
        name: b"SRAPD\0".as_ptr() as *const c_char,
        opt: b"srapd\0".as_ptr() as *const c_char,
        desc: b"Subroutine return address prediction disable\0".as_ptr() as *const c_char,
        index: 4,
        prctl: unsafe { PR_PPC_DEXCR_SRAPD },
    },
    dexcr_aspect {
        name: b"NPHIE\0".as_ptr() as *const c_char,
        opt: b"nphie\0".as_ptr() as *const c_char,
        desc: b"Non-privileged hash instruction enable\0".as_ptr() as *const c_char,
        index: 5,
        prctl: unsafe { PR_PPC_DEXCR_NPHIE },
    },
    dexcr_aspect {
        name: b"PHIE\0".as_ptr() as *const c_char,
        opt: b"phie\0".as_ptr() as *const c_char,
        desc: b"Privileged hash instruction enable\0".as_ptr() as *const c_char,
        index: 6,
        prctl: -1i64 as u64,
    },
];

extern "C" {
    pub fn dexcr_exists() -> bool;

    pub fn pr_dexcr_aspect_supported(which: u64) -> bool;

    pub fn pr_dexcr_aspect_editable(which: u64) -> bool;

    pub fn pr_get_dexcr(pr_aspect: u64) -> i32;

    pub fn pr_set_dexcr(pr_aspect: u64, ctrl: u64) -> i32;

    pub fn pr_which_to_aspect(which: u64) -> u32;

    pub fn hashchk_triggers() -> bool;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dexcr_source {
    DEXCR,     /* Userspace DEXCR value */
    HDEXCR,    /* Hypervisor enforced DEXCR value */
    EFFECTIVE, /* Bitwise OR of UDEXCR and ENFORCED DEXCR bits */
}

extern "C" {
    pub fn get_dexcr(source: dexcr_source) -> u32;

    pub fn await_child_success(pid: pid_t);

    pub fn hashst(lr: u64, sp: *mut c_void);

    pub fn hashchk(lr: u64, sp: *mut c_void);

    pub fn do_bad_hashchk();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

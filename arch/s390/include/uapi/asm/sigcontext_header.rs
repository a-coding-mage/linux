/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2000
 */

use core::ffi::{c_double, c_uchar, c_uint, c_ulong, c_ulonglong};

pub const __NUM_GPRS: usize = 16;
pub const __NUM_FPRS: usize = 16;
pub const __NUM_ACRS: usize = 16;
pub const __NUM_VXRS: usize = 32;
pub const __NUM_VXRS_LOW: usize = 16;
pub const __NUM_VXRS_HIGH: usize = 16;

/* Has to be at least _NSIG_WORDS from asm/signal.h */
pub const _SIGCONTEXT_NSIG: usize = 64;
pub const _SIGCONTEXT_NSIG_BPW: usize = 64;
/* Size of stack frame allocated when calling signal handler. */
pub const __SIGNAL_FRAMESIZE: usize = 160;

pub const _SIGCONTEXT_NSIG_WORDS: usize = _SIGCONTEXT_NSIG / _SIGCONTEXT_NSIG_BPW;
pub const _SIGMASK_COPY_SIZE: usize = core::mem::size_of::<c_ulong>() * _SIGCONTEXT_NSIG_WORDS;

#[repr(C, align(8))]
pub struct _psw_t {
    pub mask: c_ulong,
    pub addr: c_ulong,
}

#[repr(C)]
pub struct _s390_regs_common {
    pub psw: _psw_t,
    pub gprs: [c_ulong; __NUM_GPRS],
    pub acrs: [c_uint; __NUM_ACRS],
}

#[repr(C)]
pub struct _s390_fp_regs {
    pub fpc: c_uint,
    pub pad: c_uint,
    pub fprs: [c_double; __NUM_FPRS],
}

#[repr(C)]
pub struct _sigregs {
    pub regs: _s390_regs_common,
    pub fpregs: _s390_fp_regs,
}

#[repr(C)]
pub struct _sigregs_ext {
    pub vxrs_low: [c_ulonglong; __NUM_VXRS_LOW],
    pub vxrs_high: [__vector128; __NUM_VXRS_HIGH],
    pub __reserved: [c_uchar; 128],
}

#[repr(C)]
pub struct sigcontext {
    pub oldmask: [c_ulong; _SIGCONTEXT_NSIG_WORDS],
    /* __user */
    pub sregs: *mut _sigregs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

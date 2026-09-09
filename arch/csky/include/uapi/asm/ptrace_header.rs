/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::ffi::c_ulong;

#[repr(C)]
pub struct pt_regs {
    pub tls: c_ulong,
    pub lr: c_ulong,
    pub pc: c_ulong,
    pub sr: c_ulong,
    pub usp: c_ulong,

    /*
     * a0, a1, a2, a3:
     * abiv1: r2, r3, r4, r5
     * abiv2: r0, r1, r2, r3
     */
    pub orig_a0: c_ulong,
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,

    /*
     * ABIV2: r4 ~ r13
     * ABIV1: r6 ~ r14, r1
     */
    pub regs: [c_ulong; 10],

    /* __CSKYABIV2__ */
    #[cfg(CSKYABIV2)]
    /* r16 ~ r30 */
    pub exregs: [c_ulong; 15],

    #[cfg(CSKYABIV2)]
    pub rhi: c_ulong,
    #[cfg(CSKYABIV2)]
    pub rlo: c_ulong,
    #[cfg(CSKYABIV2)]
    pub dcsr: c_ulong,
}

#[repr(C)]
pub struct user_fp {
    pub vr: [c_ulong; 96],
    pub fcr: c_ulong,
    pub fesr: c_ulong,
    pub fid: c_ulong,
    pub reserved: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

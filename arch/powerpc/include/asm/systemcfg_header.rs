/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Copyright (C) 2002 Peter Bergner <bergner@vnet.ibm.com>, IBM
 * Copyright (C) 2005 Benjamin Herrenschmidy <benh@kernel.crashing.org>,
 *                  IBM Corp.
 */

/* CONFIG_PPC64 */

/*
 * If the major version changes we are incompatible.
 * Minor version changes are a hint.
 */
pub const SYSTEMCFG_MAJOR: u32 = 1;
pub const SYSTEMCFG_MINOR: u32 = 1;

#[repr(C)]
pub struct systemcfg_version {
    pub major: u32, /* Major number                 0x10 */
    pub minor: u32, /* Minor number                 0x14 */
}

#[repr(C)]
pub struct systemcfg {
    pub eye_catcher: [u8; 16], /* Eyecatcher: SYSTEMCFG:PPC64  0x00 */
    pub version: systemcfg_version,

    /* Note about the platform flags: it now only contains the lpar
     * bit. The actual platform number is dead and buried
     */
    pub platform: u32,             /* Platform flags              0x18 */
    pub processor: u32,            /* Processor type               0x1C */
    pub processorCount: u64,       /* # of physical processors     0x20 */
    pub physicalMemorySize: u64,   /* Size of real memory(B)       0x28 */
    pub tb_orig_stamp: u64,         /* (NU) Timebase at boot        0x30 */
    pub tb_ticks_per_sec: u64,      /* Timebase tics / sec          0x38 */
    pub tb_to_xs: u64,              /* (NU) Inverse of TB to 2^20   0x40 */
    pub stamp_xsec: u64,             /* (NU)                        0x48 */
    pub tb_update_count: u64,       /* (NU) Timebase atomicity ctr  0x50 */
    pub tz_minuteswest: u32,         /* (NU) Min. west of Greenwich  0x58 */
    pub tz_dsttime: u32,             /* (NU) Type of dst correction  0x5C */
    pub dcache_size: u32,            /* L1 d-cache size              0x60 */
    pub dcache_line_size: u32,       /* L1 d-cache line size         0x64 */
    pub icache_size: u32,            /* L1 i-cache size              0x68 */
    pub icache_line_size: u32,       /* L1 i-cache line size         0x6C */
}

extern "C" {
    pub static mut systemcfg: *mut systemcfg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

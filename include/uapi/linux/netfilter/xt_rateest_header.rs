/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* C header guard: _XT_RATEEST_MATCH_H */
/* Dependencies: linux/types.h and linux/if.h */

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_rateest_match_flags {
    XT_RATEEST_MATCH_INVERT = 1 << 0,
    XT_RATEEST_MATCH_ABS = 1 << 1,
    XT_RATEEST_MATCH_REL = 1 << 2,
    XT_RATEEST_MATCH_DELTA = 1 << 3,
    XT_RATEEST_MATCH_BPS = 1 << 4,
    XT_RATEEST_MATCH_PPS = 1 << 5,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_rateest_match_mode {
    XT_RATEEST_MATCH_NONE,
    XT_RATEEST_MATCH_EQ,
    XT_RATEEST_MATCH_LT,
    XT_RATEEST_MATCH_GT,
}

#[repr(C, align(8))]
pub struct xt_rateest_match_info {
    pub name1: [core::ffi::c_char; IFNAMSIZ],
    pub name2: [core::ffi::c_char; IFNAMSIZ],
    pub flags: u16,
    pub mode: u16,
    pub bps1: u32,
    pub pps1: u32,
    pub bps2: u32,
    pub pps2: u32,

    /* Used internally by the kernel */
    pub est1: *mut xt_rateest,
    pub est2: *mut xt_rateest,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

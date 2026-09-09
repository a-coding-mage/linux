/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: <linux/pkt_cls.h>

#[repr(C)]
pub struct tc_defact {
    pub gen: tc_gen,
}

#[repr(i32)]
pub enum tc_defact_attr {
    TCA_DEF_UNSPEC = 0,
    TCA_DEF_TM = 1,
    TCA_DEF_PARMS = 2,
    TCA_DEF_DATA = 3,
    TCA_DEF_PAD = 4,
    __TCA_DEF_MAX = 5,
}

pub const TCA_DEF_MAX: i32 = (__TCA_DEF_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

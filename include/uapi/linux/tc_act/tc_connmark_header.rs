/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux UAPI headers:
// - `tc_gen` is declared by linux/pkt_cls.h.
// - `__u16` is declared by linux/types.h.

#[repr(C)]
pub struct tc_connmark {
    pub tc_gen: tc_gen,
    pub zone: __u16,
}

#[repr(i32)]
pub enum tc_connmark_attr {
    TCA_CONNMARK_UNSPEC = 0,
    TCA_CONNMARK_PARMS = 1,
    TCA_CONNMARK_TM = 2,
    TCA_CONNMARK_PAD = 3,
    __TCA_CONNMARK_MAX = 4,
}

pub const TCA_CONNMARK_MAX: i32 = (__TCA_CONNMARK_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

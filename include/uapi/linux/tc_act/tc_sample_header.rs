/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// <linux/types.h>, <linux/pkt_cls.h>, and <linux/if_ether.h>

#[repr(C)]
pub struct tc_sample {
    pub tc_gen: tc_gen,
}

#[repr(i32)]
pub enum tc_sample_attr {
    TCA_SAMPLE_UNSPEC = 0,
    TCA_SAMPLE_TM,
    TCA_SAMPLE_PARMS,
    TCA_SAMPLE_RATE,
    TCA_SAMPLE_TRUNC_SIZE,
    TCA_SAMPLE_PSAMPLE_GROUP,
    TCA_SAMPLE_PAD,
    __TCA_SAMPLE_MAX,
}

pub const TCA_SAMPLE_MAX: i32 = __TCA_SAMPLE_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

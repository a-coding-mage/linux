/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies: <linux/types.h> and <linux/pkt_cls.h>.

#[repr(C)]
pub struct tc_ctinfo {
    pub tc_gen: tc_gen,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tc_ctinfo_attr {
    TCA_CTINFO_UNSPEC = 0,
    TCA_CTINFO_PAD,
    TCA_CTINFO_TM,
    TCA_CTINFO_ACT,
    TCA_CTINFO_ZONE,
    TCA_CTINFO_PARMS_DSCP_MASK,
    TCA_CTINFO_PARMS_DSCP_STATEMASK,
    TCA_CTINFO_PARMS_CPMARK_MASK,
    TCA_CTINFO_STATS_DSCP_SET,
    TCA_CTINFO_STATS_DSCP_ERROR,
    TCA_CTINFO_STATS_CPMARK_SET,
    __TCA_CTINFO_MAX,
}

pub const TCA_CTINFO_MAX: i32 = (__TCA_CTINFO_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

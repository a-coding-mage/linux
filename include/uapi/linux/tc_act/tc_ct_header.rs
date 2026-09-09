/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies: linux/types.h and linux/pkt_cls.h. */

#[repr(i32)]
pub enum tc_ct_attr {
    TCA_CT_UNSPEC,
    TCA_CT_PARMS,
    TCA_CT_TM,
    TCA_CT_ACTION,
    TCA_CT_ZONE,
    TCA_CT_MARK,
    TCA_CT_MARK_MASK,
    TCA_CT_LABELS,
    TCA_CT_LABELS_MASK,
    TCA_CT_NAT_IPV4_MIN,
    TCA_CT_NAT_IPV4_MAX,
    TCA_CT_NAT_IPV6_MIN,
    TCA_CT_NAT_IPV6_MAX,
    TCA_CT_NAT_PORT_MIN,
    TCA_CT_NAT_PORT_MAX,
    TCA_CT_PAD,
    TCA_CT_HELPER_NAME,
    TCA_CT_HELPER_FAMILY,
    TCA_CT_HELPER_PROTO,
    __TCA_CT_MAX,
}

pub const TCA_CT_MAX: i32 = __TCA_CT_MAX as i32 - 1;

pub const TCA_CT_ACT_COMMIT: u32 = 1 << 0;
pub const TCA_CT_ACT_FORCE: u32 = 1 << 1;
pub const TCA_CT_ACT_CLEAR: u32 = 1 << 2;
pub const TCA_CT_ACT_NAT: u32 = 1 << 3;
pub const TCA_CT_ACT_NAT_SRC: u32 = 1 << 4;
pub const TCA_CT_ACT_NAT_DST: u32 = 1 << 5;

#[repr(C)]
pub struct tc_ct {
    pub tc_gen: tc_gen,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

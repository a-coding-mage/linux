/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from <linux/tc_act/tc_csum.h>. */

#[repr(i32)]
pub enum TcaCsum {
    TCA_CSUM_UNSPEC,
    TCA_CSUM_PARMS,
    TCA_CSUM_TM,
    TCA_CSUM_PAD,
    __TCA_CSUM_MAX,
}

pub const TCA_CSUM_MAX: i32 = TcaCsum::__TCA_CSUM_MAX as i32 - 1;

pub const TCA_CSUM_UPDATE_FLAG_IPV4HDR: u32 = 1;
pub const TCA_CSUM_UPDATE_FLAG_ICMP: u32 = 2;
pub const TCA_CSUM_UPDATE_FLAG_IGMP: u32 = 4;
pub const TCA_CSUM_UPDATE_FLAG_TCP: u32 = 8;
pub const TCA_CSUM_UPDATE_FLAG_UDP: u32 = 16;
pub const TCA_CSUM_UPDATE_FLAG_UDPLITE: u32 = 32;
pub const TCA_CSUM_UPDATE_FLAG_SCTP: u32 = 64;

#[repr(C)]
pub struct tc_csum {
    /* `tc_gen` is supplied by the linux pkt_cls dependency. */
    pub tc_gen: tc_gen,
    pub update_flags: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

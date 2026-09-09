/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: C header <linux/types.h> supplies __u8 and __u16.

pub const NF_SYNPROXY_OPT_MSS: u8 = 0x01;
pub const NF_SYNPROXY_OPT_WSCALE: u8 = 0x02;
pub const NF_SYNPROXY_OPT_SACK_PERM: u8 = 0x04;
pub const NF_SYNPROXY_OPT_TIMESTAMP: u8 = 0x08;
pub const NF_SYNPROXY_OPT_ECN: u8 = 0x10;
pub const NF_SYNPROXY_OPT_MASK: u8 = NF_SYNPROXY_OPT_MSS
    | NF_SYNPROXY_OPT_WSCALE
    | NF_SYNPROXY_OPT_SACK_PERM
    | NF_SYNPROXY_OPT_TIMESTAMP;

#[repr(C)]
pub struct nf_synproxy_info {
    pub options: u8,
    pub wscale: u8,
    pub mss: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

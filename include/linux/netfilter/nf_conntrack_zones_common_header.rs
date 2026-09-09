/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: uapi/linux/netfilter/nf_conntrack_tuple_common.h

pub const NF_CT_DEFAULT_ZONE_ID: u16 = 0;

pub const NF_CT_ZONE_DIR_ORIG: u32 = 1 << IP_CT_DIR_ORIGINAL;
pub const NF_CT_ZONE_DIR_REPL: u32 = 1 << IP_CT_DIR_REPLY;

pub const NF_CT_DEFAULT_ZONE_DIR: u32 = NF_CT_ZONE_DIR_ORIG | NF_CT_ZONE_DIR_REPL;

pub const NF_CT_FLAG_MARK: u32 = 1;

#[repr(C)]
pub struct nf_conntrack_zone {
    pub id: u16,
    pub flags: u8,
    pub dir: u8,
}

unsafe extern "C" {
    pub static nf_ct_zone_dflt: nf_conntrack_zone;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

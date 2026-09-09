/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h, linux/if_ether.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum clusterip_hashmode {
    CLUSTERIP_HASHMODE_SIP = 0,
    CLUSTERIP_HASHMODE_SIP_SPT,
    CLUSTERIP_HASHMODE_SIP_SPT_DPT,
}

pub const CLUSTERIP_HASHMODE_MAX: clusterip_hashmode =
    clusterip_hashmode::CLUSTERIP_HASHMODE_SIP_SPT_DPT;

pub const CLUSTERIP_MAX_NODES: usize = 16;

pub const CLUSTERIP_FLAG_NEW: u32 = 0x00000001;

#[repr(C)]
pub struct clusterip_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipt_clusterip_tgt_info {
    pub flags: u32,

    /* only relevant for new ones */
    pub clustermac: [u8; ETH_ALEN],
    pub num_total_nodes: u16,
    pub num_local_nodes: u16,
    pub local_nodes: [u16; CLUSTERIP_MAX_NODES],
    pub hash_mode: u32,
    pub hash_initval: u32,

    /* Used internally by the kernel */
    pub config: *mut clusterip_config,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

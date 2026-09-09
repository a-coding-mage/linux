/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2008 Cisco Systems, Inc. All rights reserved. */

// Dependency: <scsi/fc/fc_ns.h>

pub const FIP_DEF_PRI: u32 = 128;
pub const FIP_DEF_FC_MAP: u32 = 0x0efc00;
pub const FIP_DEF_FKA: u32 = 8000;
pub const FIP_VN_KA_PERIOD: u32 = 90000;
pub const FIP_FCF_FUZZ: u32 = 100;
pub const FIP_VN_FC_MAP: u32 = 0x0efd00;
pub const FIP_VN_PROBE_WAIT: u32 = 100;
pub const FIP_VN_ANN_WAIT: u32 = 400;
pub const FIP_VN_RLIM_INT: u32 = 10000;
pub const FIP_VN_RLIM_COUNT: u32 = 10;
pub const FIP_VN_BEACON_INT: u32 = 8000;
pub const FIP_VN_BEACON_FUZZ: u32 = 100;

pub const FIP_ALL_FCOE_MACS: [u8; 6] = [1, 0x10, 0x18, 1, 0, 0];
pub const FIP_ALL_ENODE_MACS: [u8; 6] = [1, 0x10, 0x18, 1, 0, 1];
pub const FIP_ALL_FCF_MACS: [u8; 6] = [1, 0x10, 0x18, 1, 0, 2];
pub const FIP_ALL_VN2VN_MACS: [u8; 6] = [1, 0x10, 0x18, 1, 0, 4];
pub const FIP_ALL_P2P_MACS: [u8; 6] = [1, 0x10, 0x18, 1, 0, 5];

pub const FIP_VER: u8 = 1;

#[repr(C, packed)]
pub struct fip_header {
    pub fip_ver: u8, pub fip_resv1: u8, pub fip_op: u16,
    pub fip_resv2: u8, pub fip_subcode: u8, pub fip_dl_len: u16, pub fip_flags: u16,
}

pub const FIP_VER_SHIFT: u32 = 4;
#[inline] pub const fn FIP_VER_ENCAPS(v: u32) -> u32 { v << FIP_VER_SHIFT }
#[inline] pub const fn FIP_VER_DECAPS(v: u32) -> u32 { v >> FIP_VER_SHIFT }
pub const FIP_BPW: u32 = 4;

pub const FIP_OP_DISC: u16 = 1; pub const FIP_OP_LS: u16 = 2; pub const FIP_OP_CTRL: u16 = 3;
pub const FIP_OP_VLAN: u16 = 4; pub const FIP_OP_VN2VN: u16 = 5;
pub const FIP_OP_VENDOR_MIN: u16 = 0xfff8; pub const FIP_OP_VENDOR_MAX: u16 = 0xfffe;
pub const FIP_SC_SOL: u8 = 1; pub const FIP_SC_ADV: u8 = 2;
pub const FIP_SC_REQ: u8 = 1; pub const FIP_SC_REP: u8 = 2;
pub const FIP_SC_KEEP_ALIVE: u8 = 1; pub const FIP_SC_CLR_VLINK: u8 = 2;
pub const FIP_SC_VL_REQ: u8 = 1; pub const FIP_SC_VL_NOTE: u8 = 2; pub const FIP_SC_VL_VN2VN_NOTE: u8 = 3;
pub const FIP_SC_VN_PROBE_REQ: u8 = 1; pub const FIP_SC_VN_PROBE_REP: u8 = 2;
pub const FIP_SC_VN_CLAIM_NOTIFY: u8 = 3; pub const FIP_SC_VN_CLAIM_REP: u8 = 4; pub const FIP_SC_VN_BEACON: u8 = 5;

pub const FIP_FL_FPMA: u16 = 0x8000; pub const FIP_FL_SPMA: u16 = 0x4000; pub const FIP_FL_FCF: u16 = 0x0020;
pub const FIP_FL_FDF: u16 = 0x0010; pub const FIP_FL_REC_OR_P2P: u16 = 0x0008; pub const FIP_FL_AVAIL: u16 = 0x0004;
pub const FIP_FL_SOL: u16 = 0x0002; pub const FIP_FL_FPORT: u16 = 0x0001;

#[repr(C)] pub struct fip_desc { pub fip_dtype: u8, pub fip_dlen: u8 }
pub const FIP_DT_PRI: u8 = 1; pub const FIP_DT_MAC: u8 = 2; pub const FIP_DT_MAP_OUI: u8 = 3;
pub const FIP_DT_NAME: u8 = 4; pub const FIP_DT_FAB: u8 = 5; pub const FIP_DT_FCOE_SIZE: u8 = 6;
pub const FIP_DT_FLOGI: u8 = 7; pub const FIP_DT_FDISC: u8 = 8; pub const FIP_DT_LOGO: u8 = 9;
pub const FIP_DT_ELP: u8 = 10; pub const FIP_DT_VN_ID: u8 = 11; pub const FIP_DT_FKA: u8 = 12;
pub const FIP_DT_VENDOR: u8 = 13; pub const FIP_DT_VLAN: u8 = 14; pub const FIP_DT_FC4F: u8 = 15;
pub const FIP_DT_LIMIT: u8 = 16; pub const FIP_DT_NON_CRITICAL: u8 = 128; pub const FIP_DT_CLR_VLINKS: u8 = 128; pub const FIP_DT_VENDOR_BASE: u8 = 241;

#[repr(C, packed)] pub struct fip_pri_desc { pub fd_desc: fip_desc, pub fd_resvd: u8, pub fd_pri: u8 }
#[repr(C, packed)] pub struct fip_mac_desc { pub fd_desc: fip_desc, pub fd_mac: [u8; 6] }
#[repr(C, packed)] pub struct fip_map_desc { pub fd_desc: fip_desc, pub fd_resvd: [u8; 3], pub fd_map: [u8; 3] }
#[repr(C, packed)] pub struct fip_wwn_desc { pub fd_desc: fip_desc, pub fd_resvd: [u8; 2], pub fd_wwn: u64 }
#[repr(C, packed)] pub struct fip_fab_desc { pub fd_desc: fip_desc, pub fd_vfid: u16, pub fd_resvd: u8, pub fd_map: [u8; 3], pub fd_wwn: u64 }
#[repr(C, packed)] pub struct fip_size_desc { pub fd_desc: fip_desc, pub fd_size: u16 }
#[repr(C, packed)] pub struct fip_encaps { pub fd_desc: fip_desc, pub fd_resvd: [u8; 2] }
#[repr(C, packed)] pub struct fip_vn_desc { pub fd_desc: fip_desc, pub fd_mac: [u8; 6], pub fd_resvd: u8, pub fd_fc_id: [u8; 3], pub fd_wwpn: u64 }
#[repr(C, packed)] pub struct fip_fka_desc { pub fd_desc: fip_desc, pub fd_resvd: u8, pub fd_flags: u8, pub fd_fka_period: u32 }
pub const FIP_FKA_ADV_D: u8 = 0x01;
#[repr(C, packed)] pub struct fip_vlan_desc { pub fd_desc: fip_desc, pub fd_vlan: u16 }

#[repr(C, packed)] pub struct fip_fc4_feat {
    pub fd_desc: fip_desc, pub fd_resvd: [u8; 2],
    pub fd_fts: crate::fc_ns_fts, pub fd_ff: crate::fc_ns_ff,
}
#[repr(C, packed)] pub struct fip_vendor_desc { pub fd_desc: fip_desc, pub fd_resvd: [u8; 2], pub fd_vendor_id: [u8; 8] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

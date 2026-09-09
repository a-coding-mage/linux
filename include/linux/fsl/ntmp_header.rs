/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2025-2026 NXP */

// Dependencies supplied by the surrounding kernel/Rust environment.

pub const NTMP_NULL_ENTRY_ID: u32 = 0xffff_ffff;
pub const IPFT_MAX_PLD_LEN: usize = 24;

#[repr(C)]
pub struct maft_keye_data { pub mac_addr: [u8; 6], pub resv: u16 }
#[repr(C)]
pub struct maft_cfge_data { pub si_bitmap: u16, pub resv: u16 }
#[repr(C)]
pub struct netc_cbdr_regs { pub pir: *mut core::ffi::c_void, pub cir: *mut core::ffi::c_void, pub mr: *mut core::ffi::c_void, pub bar0: *mut core::ffi::c_void, pub bar1: *mut core::ffi::c_void, pub lenr: *mut core::ffi::c_void }
#[repr(C)]
pub struct netc_tbl_vers { pub maft_ver: u8, pub rsst_ver: u8, pub fdbt_ver: u8, pub vft_ver: u8, pub bpt_ver: u8, pub ipft_ver: u8, pub ett_ver: u8, pub ect_ver: u8 }
#[repr(C)]
pub struct netc_swcbd { pub buf: *mut core::ffi::c_void, pub dma: u64, pub size: usize }
#[repr(C)]
pub struct netc_cbdr {
    pub dev: *mut device, pub regs: netc_cbdr_regs, pub bd_num: i32, pub next_to_use: i32,
    pub next_to_clean: i32, pub dma_size: i32, pub addr_base: *mut core::ffi::c_void,
    pub addr_base_align: *mut core::ffi::c_void, pub dma_base: u64, pub dma_base_align: u64,
    pub swcbd: *mut netc_swcbd, pub ring_lock: mutex,
}
#[repr(C)]
pub struct ntmp_user {
    pub cbdr_num: i32, pub dev: *mut device, pub ring: *mut netc_cbdr, pub tbl: netc_tbl_vers,
    pub ett_bitmap_size: u32, pub ect_bitmap_size: u32, pub maft_num_entries: u16,
    pub ett_gid_bitmap: *mut c_ulong, pub ect_gid_bitmap: *mut c_ulong, pub maft_eid_bitmap: *mut c_ulong,
}
#[repr(C)]
pub struct maft_entry_data { pub keye: maft_keye_data, pub cfge: maft_cfge_data }
#[repr(C)]
pub struct ipft_pld_byte { pub data: u8, pub mask: u8 }

pub const IPFT_FAF_OVLAN: u16 = 1 << 2; pub const IPFT_FAF_IVLAN: u16 = 1 << 3;
pub const IPFT_FAF_IP_HDR: u16 = 1 << 7; pub const IPFT_FAF_IP_VER6: u16 = 1 << 8;
pub const IPFT_FAF_L4_CODE: u16 = 0xC00; pub const IPFT_FAF_TCP_HDR: u16 = 1;
pub const IPFT_FAF_UDP_HDR: u16 = 2; pub const IPFT_FAF_SCTP_HDR: u16 = 3;
pub const IPFT_FAF_WOL_MAGIC: u16 = 1 << 12;
pub const IPFT_DSCP: u16 = 0x3f; pub const IPFT_DSCP_MASK: u16 = 0xfc0; pub const IPFT_DSCP_MASK_ALL: u16 = 0x3f;
pub const IPFT_SRC_PORT: u16 = 0x1f; pub const IPFT_SRC_PORT_MASK: u16 = 0x3e0; pub const IPFT_SRC_PORT_MASK_ALL: u16 = 0x1f;

#[repr(C)]
pub struct ipft_keye_data {
    pub precedence: u16, pub resv0: [u16; 3], pub frm_attr_flags: u16, pub frm_attr_flags_mask: u16,
    pub dscp: u16, pub src_port: u16, pub outer_vlan_tci: u16, pub outer_vlan_tci_mask: u16,
    pub dmac: [u8; 6], pub dmac_mask: [u8; 6], pub smac: [u8; 6], pub smac_mask: [u8; 6],
    pub inner_vlan_tci: u16, pub inner_vlan_tci_mask: u16, pub ethertype: u16, pub ethertype_mask: u16,
    pub ip_protocol: u8, pub ip_protocol_mask: u8, pub resv1: [u16; 7], pub ip_src: [u32; 4],
    pub resv2: [u32; 2], pub ip_src_mask: [u32; 4], pub l4_src_port: u16, pub l4_src_port_mask: u16,
    pub resv3: u32, pub ip_dst: [u32; 4], pub resv4: [u32; 2], pub ip_dst_mask: [u32; 4],
    pub l4_dst_port: u16, pub l4_dst_port_mask: u16, pub resv5: u32, pub byte: [ipft_pld_byte; IPFT_MAX_PLD_LEN],
}
#[repr(C)] pub struct ipft_cfge_data { pub cfg: u32, pub flta_tgt: u32 }
#[repr(C)] pub struct ipft_entry_data { pub entry_id: u32, pub keye: ipft_keye_data, pub cfge: ipft_cfge_data }
#[repr(C)] pub struct fdbt_keye_data { pub mac_addr: [u8; 6], pub resv0: u16, pub fid: u16, pub resv1: u16 }
#[repr(C)] pub struct fdbt_cfge_data { pub port_bitmap: u32, pub cfg: u32, pub et_eid: u32 }
#[repr(C)] pub struct fdbt_entry_data { pub entry_id: u32, pub keye: fdbt_keye_data, pub cfge: fdbt_cfge_data, pub acte: u8 }
#[repr(C)] pub struct vft_cfge_data { pub bitmap_stg: u32, pub fid: u16, pub cfg: u16, pub eta_port_bitmap: u32, pub et_eid: u32 }
#[repr(C)] pub struct ett_cfge_data { pub efm_cfg: u16, pub efm_data_len: u16, pub efm_eid: u32, pub ec_eid: u32, pub esqa_tgt_eid: u32 }
#[repr(C, packed)] pub struct bpt_bpse_data { pub amount_used: u32, pub amount_used_hwm: u32, pub bpd_fc_state: u8 }
#[repr(C)] pub struct bpt_cfge_data { pub fccfg_sbpen: u8, pub pfc_vector: u8, pub max_thresh: u16, pub fc_on_thresh: u16, pub fc_off_thresh: u16, pub sbp_thresh: u16, pub resv: u16, pub sbp_eid: u32, pub fc_ports: u32 }
#[repr(C)] pub union ntmp_fmt_eid { pub index: u32, pub vuda_sqta: u32, pub vara_vid: u32 }

pub const VFT_PORT_MEMBERSHIP: u32 = 0x00ff_ffff; pub const VFT_STG_ID_MASK: u32 = 0x0f00_0000;
#[inline] pub const fn VFT_STG_ID(g: u32) -> u32 { (g << 24) & VFT_STG_ID_MASK }
pub const FDBT_FID: u16 = 0x0fff; pub const FDBT_PORT_BITMAP: u32 = 0x00ff_ffff;
pub const FDBT_OETEID: u32 = 3; pub const FDBT_EPORT: u32 = 0x7c; pub const FDBT_IMIRE: u32 = 1 << 7;
pub const FDBT_CTD: u32 = 0x600; pub const FDBT_DYNAMIC: u32 = 1 << 11; pub const FDBT_TIMECAPE: u32 = 1 << 12;
pub const FDBT_ACT_CNT: u8 = 0x7f; pub const FDBT_ACT_FLAG: u8 = 1 << 7;
pub const VFT_FID: u16 = 0x0fff; pub const VFT_MLO: u16 = 7; pub const VFT_MFO: u16 = 0x18;
pub const VFT_IPMFE: u16 = 1 << 6; pub const VFT_IPMFLE: u16 = 1 << 7; pub const VFT_PGA: u16 = 1 << 8;
pub const VFT_SFDA: u16 = 1 << 10; pub const VFT_OSFDA: u16 = 1 << 11; pub const VFT_FDBAFSS: u16 = 1 << 12;
pub const VFT_ETA_PORT_BITMAP: u32 = 0x00ff_ffff;
pub const FMTEID_INDEX: u32 = 0x1fff; pub const FMTEID_VUDA: u32 = 3; pub const FMTEID_VUDA_DEL_OTAG: u32 = 2;
pub const FMTEID_SQTA: u32 = 0x1c; pub const FMTEID_SQTA_DEL: u32 = 2; pub const FMTEID_VUDA_SQTA: u32 = 1 << 13;
pub const FMTEID_VID: u32 = 0x0fff; pub const FMTEID_VARA: u32 = 0x3000; pub const FMTEID_VARA_VID: u32 = 1 << 14;
pub const BPT_FC_STATE: u8 = 1; pub const BPT_BPD: u8 = 2; pub const BPT_FC_CFG: u8 = 6; pub const BPT_FC_CFG_EN_BPFC: u8 = 1;
pub const ETT_EFM_MODE: u16 = 3; pub const ETT_ESQA: u16 = 0x30; pub const ETT_ECA: u16 = 0x1c0;
pub const ETT_ECA_INC: u16 = 1; pub const ETT_EFM_LEN_CHANGE: u16 = 0xfe00; pub const ETT_FRM_LEN_DEL_VLAN: u16 = 0x7c;
pub const ETT_FRM_LEN_DEL_RTAG: u16 = 0x7a; pub const ETT_FRM_LEN_DEL_VLAN_RTAG: u16 = 0x76; pub const ETT_EFM_DATA_LEN: u16 = 0x07ff;
pub const IPFT_IPV: u32 = 0xf; pub const IPFT_OIPV: u32 = 1 << 4; pub const IPFT_DR: u32 = 0x60; pub const IPFT_ODR: u32 = 1 << 7;
pub const IPFT_FLTFA: u32 = 0x700; pub const IPFT_FLTFA_DISCARD: u32 = 0; pub const IPFT_FLTFA_PERMIT: u32 = 1; pub const IPFT_FLTFA_REDIRECT: u32 = 2;
pub const IPFT_IMIRE: u32 = 1 << 11; pub const IPFT_WOLTE: u32 = 1 << 12; pub const IPFT_FLTA: u32 = 0x6000;
pub const IPFT_FLTA_RP: u32 = 1; pub const IPFT_FLTA_IS: u32 = 2; pub const IPFT_FLTA_SI_BITMAP: u32 = 3;
pub const IPFT_RPR: u32 = 0x18000; pub const IPFT_CTD: u32 = 1 << 17; pub const IPFT_HR: u32 = 0x3c0000;
pub const IPFT_TIMECAPE: u32 = 1 << 22; pub const IPFT_RRT: u32 = 1 << 23; pub const IPFT_BL2F: u32 = 1 << 24; pub const IPFT_EVMEID: u32 = 0xf000_0000;

#[cfg(feature = "CONFIG_NXP_NETC_LIB")]
extern "C" {
    pub fn ntmp_init_cbdr(cbdr: *mut netc_cbdr, dev: *mut device, regs: *const netc_cbdr_regs) -> i32;
    pub fn ntmp_free_cbdr(cbdr: *mut netc_cbdr);
    pub fn ntmp_lookup_free_eid(bitmap: *mut c_ulong, size: u32) -> u32;
    pub fn ntmp_clear_eid_bitmap(bitmap: *mut c_ulong, entry_id: u32);
    pub fn ntmp_maft_add_entry(user: *mut ntmp_user, entry_id: u32, maft: *mut maft_entry_data) -> i32;
    pub fn ntmp_maft_query_entry(user: *mut ntmp_user, entry_id: u32, maft: *mut maft_entry_data) -> i32;
    pub fn ntmp_maft_delete_entry(user: *mut ntmp_user, entry_id: u32) -> i32;
    pub fn ntmp_rsst_update_entry(user: *mut ntmp_user, table: *const u32, count: i32) -> i32;
    pub fn ntmp_rsst_query_entry(user: *mut ntmp_user, table: *mut u32, count: i32) -> i32;
    pub fn ntmp_ipft_add_entry(user: *mut ntmp_user, entry: *mut ipft_entry_data) -> i32;
    pub fn ntmp_ipft_delete_entry(user: *mut ntmp_user, entry_id: u32) -> i32;
    pub fn ntmp_fdbt_add_entry(user: *mut ntmp_user, entry_id: *mut u32, keye: *const fdbt_keye_data, cfge: *const fdbt_cfge_data) -> i32;
    pub fn ntmp_fdbt_update_entry(user: *mut ntmp_user, entry_id: u32, cfge: *const fdbt_cfge_data) -> i32;
    pub fn ntmp_fdbt_delete_entry(user: *mut ntmp_user, entry_id: u32) -> i32;
    pub fn ntmp_fdbt_search_port_entry(user: *mut ntmp_user, port: i32, resume_entry_id: *mut u32, entry: *mut fdbt_entry_data) -> i32;
    pub fn ntmp_fdbt_update_activity_element(user: *mut ntmp_user) -> i32;
    pub fn ntmp_fdbt_delete_ageing_entries(user: *mut ntmp_user, act_cnt: u8) -> i32;
    pub fn ntmp_fdbt_delete_port_dynamic_entries(user: *mut ntmp_user, port: i32) -> i32;
    pub fn ntmp_vft_add_entry(user: *mut ntmp_user, vid: u16, cfge: *const vft_cfge_data) -> i32;
    pub fn ntmp_vft_update_entry(user: *mut ntmp_user, vid: u16, cfge: *const vft_cfge_data) -> i32;
    pub fn ntmp_vft_delete_entry(user: *mut ntmp_user, vid: u16) -> i32;
    pub fn ntmp_ett_add_entry(user: *mut ntmp_user, entry_id: u32, cfge: *const ett_cfge_data) -> i32;
    pub fn ntmp_ett_update_entry(user: *mut ntmp_user, entry_id: u32, cfge: *const ett_cfge_data) -> i32;
    pub fn ntmp_ett_delete_entry(user: *mut ntmp_user, entry_id: u32) -> i32;
    pub fn ntmp_ect_update_entry(user: *mut ntmp_user, entry_id: u32) -> i32;
    pub fn ntmp_bpt_update_entry(user: *mut ntmp_user, entry_id: u32, cfge: *const bpt_cfge_data) -> i32;
}

#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_init_cbdr(_: *mut netc_cbdr, _: *mut device, _: *const netc_cbdr_regs) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_free_cbdr(_: *mut netc_cbdr) {}

#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_maft_add_entry(_: *mut ntmp_user, _: u32, _: *mut maft_entry_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_maft_query_entry(_: *mut ntmp_user, _: u32, _: *mut maft_entry_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_maft_delete_entry(_: *mut ntmp_user, _: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_rsst_update_entry(_: *mut ntmp_user, _: *const u32, _: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NXP_NETC_LIB"))]
#[inline] pub unsafe fn ntmp_rsst_query_entry(_: *mut ntmp_user, _: *mut u32, _: i32) -> i32 { 0 }

// Kernel-provided opaque types and C integer aliases.
pub type c_ulong = usize;
pub enum device {}
pub enum mutex {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

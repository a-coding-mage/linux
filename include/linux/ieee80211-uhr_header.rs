/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE 802.11 UHR definitions */

use core::mem::size_of;

pub const IEEE80211_UHR_OPER_PARAMS_DPS_ENA: u16 = 0x0001;
pub const IEEE80211_UHR_OPER_PARAMS_NPCA_ENA: u16 = 0x0002;
pub const IEEE80211_UHR_OPER_PARAMS_PEDCA_ENA: u16 = 0x0004;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_ENA: u16 = 0x0008;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_BW: u16 = 0x0070;
pub const IEEE80211_UHR_OPER_PARAMS_DUO_PRES: u16 = 0x0080;
pub const IEEE80211_UHR_OPER_PARAMS_DPS_PRES: u16 = 0x0100;
pub const IEEE80211_UHR_OPER_PARAMS_NPCA_PRES: u16 = 0x0200;
pub const IEEE80211_UHR_OPER_PARAMS_PEDCA_PRES: u16 = 0x0400;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_PRES: u16 = 0x0800;

#[repr(C, packed)]
pub struct ieee80211_uhr_operation { pub params: u16, pub basic_mcs_nss_set: [u8; 4], pub variable: [u8; 0] }

pub const IEEE80211_UHR_NPCA_PARAMS_PRIMARY_CHAN_OFFS: u32 = 0x0000000F;
pub const IEEE80211_UHR_NPCA_PARAMS_MIN_DUR_THRESH: u32 = 0x000000F0;
pub const IEEE80211_UHR_NPCA_PARAMS_SWITCH_DELAY: u32 = 0x00003F00;
pub const IEEE80211_UHR_NPCA_PARAMS_SWITCH_BACK_DELAY: u32 = 0x000FC000;
pub const IEEE80211_UHR_NPCA_PARAMS_INIT_QSRC: u32 = 0x00300000;
pub const IEEE80211_UHR_NPCA_PARAMS_MOPLEN: u32 = 0x00400000;
pub const IEEE80211_UHR_NPCA_PARAMS_DIS_SUBCH_BMAP_PRES: u32 = 0x00800000;

#[repr(C, packed)]
pub struct ieee80211_uhr_npca_info { pub params: u32, pub dis_subch_bmap: [u16; 0] }

pub const IEEE80211_UHR_DPS_PADDING_DELAY: u32 = 0x0000003F;
pub const IEEE80211_UHR_DPS_TRANSITION_DELAY: u32 = 0x00003F00;
pub const IEEE80211_UHR_DPS_ICF_REQUIRED: u32 = 0x00010000;
pub const IEEE80211_UHR_DPS_PARAMETERIZED_FLAG: u32 = 0x00020000;
pub const IEEE80211_UHR_DPS_LC_MODE_BW: u32 = 0x001C0000;
pub const IEEE80211_UHR_DPS_LC_MODE_NSS: u32 = 0x01E00000;
pub const IEEE80211_UHR_DPS_LC_MODE_MCS: u32 = 0x1E000000;
pub const IEEE80211_UHR_DPS_MOBILE_AP_DPS_STATIC_HCM: u32 = 0x20000000;

#[repr(C, packed)] pub struct ieee80211_uhr_dps_info { pub params: u32 }

pub const IEEE80211_UHR_DBE_OPER_BANDWIDTH: u8 = 0x07;
pub const IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES: u8 = 0x08;
#[repr(C)] pub enum ieee80211_uhr_dbe_oper_bw { IEEE80211_UHR_DBE_OPER_BW_40=1, IEEE80211_UHR_DBE_OPER_BW_80=2, IEEE80211_UHR_DBE_OPER_BW_160=3, IEEE80211_UHR_DBE_OPER_BW_320_1=4, IEEE80211_UHR_DBE_OPER_BW_320_2=5 }
pub fn ieee80211_uhr_dbe_bw_mhz(bw: ieee80211_uhr_dbe_oper_bw) -> i32 { match bw { ieee80211_uhr_dbe_oper_bw::IEEE80211_UHR_DBE_OPER_BW_40 => 40, ieee80211_uhr_dbe_oper_bw::IEEE80211_UHR_DBE_OPER_BW_80 => 80, ieee80211_uhr_dbe_oper_bw::IEEE80211_UHR_DBE_OPER_BW_160 => 160, ieee80211_uhr_dbe_oper_bw::IEEE80211_UHR_DBE_OPER_BW_320_1 | ieee80211_uhr_dbe_oper_bw::IEEE80211_UHR_DBE_OPER_BW_320_2 => 320 } }
#[repr(C, packed)] pub struct ieee80211_uhr_dbe_info { pub params: u8, pub dis_subch_bmap: [u16; 0] }

pub const IEEE80211_UHR_P_EDCA_ECWMIN:u8=0x0F; pub const IEEE80211_UHR_P_EDCA_ECWMAX:u8=0xF0; pub const IEEE80211_UHR_P_EDCA_AIFSN:u16=0x000F; pub const IEEE80211_UHR_P_EDCA_CW_DS:u16=0x0030; pub const IEEE80211_UHR_P_EDCA_PSRC_THRESHOLD:u16=0x01C0; pub const IEEE80211_UHR_P_EDCA_QSRC_THRESHOLD:u16=0x0600;
#[repr(C, packed)] pub struct ieee80211_uhr_p_edca_info { pub p_edca_ec:u8, pub params:u16 }

#[inline] pub unsafe fn ieee80211_uhr_oper_size_ok(data:*const u8, len:u8)->bool { let o=data as *const ieee80211_uhr_operation; let mut n=size_of::<ieee80211_uhr_operation>(); if (len as usize)<n{return false;} let p=(*o).params; if p & IEEE80211_UHR_OPER_PARAMS_DPS_PRES !=0 {n+=size_of::<ieee80211_uhr_dps_info>();if len as usize<n{return false;}} if p & IEEE80211_UHR_OPER_PARAMS_NPCA_PRES !=0 {let x=data.add(n) as *const ieee80211_uhr_npca_info;n+=size_of::<ieee80211_uhr_npca_info>();if len as usize<n{return false;}if (*x).params&IEEE80211_UHR_NPCA_PARAMS_DIS_SUBCH_BMAP_PRES!=0{n+=2;if len as usize<n{return false;}}} if p&IEEE80211_UHR_OPER_PARAMS_PEDCA_PRES!=0{n+=size_of::<ieee80211_uhr_p_edca_info>();if len as usize<n{return false;}} if p&IEEE80211_UHR_OPER_PARAMS_DBE_PRES!=0{let x=data.add(n)as*const ieee80211_uhr_dbe_info;n+=size_of::<ieee80211_uhr_dbe_info>();if len as usize<n{return false;}if(*x).params&IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES!=0{n+=2;if len as usize<n{return false;}}} len as usize>=n }

pub const IEEE80211_UHR_MAC_CAP0_DPS_SUPP:u8=0x01; pub const IEEE80211_UHR_MAC_CAP0_DPS_ASSIST_SUPP:u8=0x02; pub const IEEE80211_UHR_MAC_CAP0_DPS_AP_STATIC_HCM_SUPP:u8=0x04; pub const IEEE80211_UHR_MAC_CAP0_NPCA_SUPP:u8=0x10; pub const IEEE80211_UHR_MAC_CAP0_ENH_BSR_SUPP:u8=0x20; pub const IEEE80211_UHR_MAC_CAP0_ADD_MAP_TID_SUPP:u8=0x40; pub const IEEE80211_UHR_MAC_CAP0_EOTSP_SUPP:u8=0x80;
pub const IEEE80211_UHR_MAC_CAP1_DSO_SUPP:u8=1; pub const IEEE80211_UHR_MAC_CAP1_PEDCA_SUPP:u8=2; pub const IEEE80211_UHR_MAC_CAP1_DBE_SUPP:u8=4; pub const IEEE80211_UHR_MAC_CAP1_UL_LLI_SUPP:u8=8; pub const IEEE80211_UHR_MAC_CAP1_P2P_LLI_SUPP:u8=0x10; pub const IEEE80211_UHR_MAC_CAP1_PUO_SUPP:u8=0x20; pub const IEEE80211_UHR_MAC_CAP1_AP_PUO_SUPP:u8=0x40; pub const IEEE80211_UHR_MAC_CAP1_DUO_SUPP:u8=0x80;
pub const IEEE80211_UHR_MAC_CAP2_UHR_OM_PU_TO_LOW:u8=0xC0; pub const IEEE80211_UHR_MAC_CAP3_UHR_OM_PU_TO_HIGH:u8=3; pub const IEEE80211_UHR_MAC_CAP_DBE_EHT_MCS_MAP_160_PRES:u8=8; pub const IEEE80211_UHR_MAC_CAP_DBE_EHT_MCS_MAP_320_PRES:u8=0x10;
pub const IEEE80211_UHR_MAC_CAP2_OMC_UL_MU_DIS_RX_SUPP:u8=1; pub const IEEE80211_UHR_MAC_CAP2_AOM_SUPP:u8=2; pub const IEEE80211_UHR_MAC_CAP2_IFCS_LOC_SUPP:u8=4; pub const IEEE80211_UHR_MAC_CAP2_UHR_TRS_SUPP:u8=8; pub const IEEE80211_UHR_MAC_CAP2_TXSPG_SUPP:u8=0x10; pub const IEEE80211_UHR_MAC_CAP2_TXOP_RET_IN_TXSPG:u8=0x20;
pub const IEEE80211_UHR_MAC_CAP3_UHR_OM_PU_TO_HIGH:u8=3; pub const IEEE80211_UHR_MAC_CAP3_PARAM_UPD_ADV_NOTIF_INTV:u8=0x1c; pub const IEEE80211_UHR_MAC_CAP3_UPD_IND_TIM_INTV_LOW:u8=0xe0; pub const IEEE80211_UHR_MAC_CAP4_UPD_IND_TIM_INTV_HIGH:u8=3; pub const IEEE80211_UHR_MAC_CAP4_BOUNDED_ESS:u8=4; pub const IEEE80211_UHR_MAC_CAP4_BTM_ASSURANCE:u8=8; pub const IEEE80211_UHR_MAC_CAP4_CO_BF_SUPP:u8=0x10; pub const IEEE80211_UHR_MAC_CAP_DBE_MAX_BW:u8=7;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_LE80:u32=1; pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_LE80:u32=2; pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_160:u32=4; pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_160:u32=8; pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_320:u32=0x10; pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_320:u32=0x20; pub const IEEE80211_UHR_PHY_CAP_ELR_TX:u32=0x40; pub const IEEE80211_UHR_PHY_CAP_ELR_RX:u32=0x80; pub const IEEE80211_UHR_PHY_CAP_PART_BW_DL_MUMIMO:u32=0x100; pub const IEEE80211_UHR_PHY_CAP_PART_BW_UL_MUMIMO:u32=0x200; pub const IEEE80211_UHR_PHY_CAP_MCS15:u32=0x400; pub const IEEE80211_UHR_PHY_CAP_2XLDPC_TX:u32=0x800; pub const IEEE80211_UHR_PHY_CAP_2XLDPC_RX:u32=0x1000; pub const IEEE80211_UHR_PHY_CAP_UEQM_TX_MAX_NSS:u32=0x6000; pub const IEEE80211_UHR_PHY_CAP_UEQM_RX_MAX_NSS:u32=0x18000; pub const IEEE80211_UHR_PHY_CAP_CO_BF_JOINT_SOUNDING:u32=0x40000; pub const IEEE80211_UHR_PHY_CAP_IM_TX:u32=0x80000; pub const IEEE80211_UHR_PHY_CAP_IM_RX:u32=0x100000; pub const IEEE80211_UHR_PHY_CAP_CO_SR_MODE_1:u32=0x200000; pub const IEEE80211_UHR_PHY_CAP_CO_SR_MODE_2:u32=0x400000; pub const IEEE80211_UHR_PHY_CAP_DRU_RRU_HYBRID_MODE:u32=0x80000000;
#[repr(C, packed)] pub struct ieee80211_uhr_cap_dbe { pub cap:u8, pub eht_mcs_map:[u8;0] }
#[repr(C, packed)] pub struct ieee80211_uhr_cap_mac { pub mac_cap:[u8;6] }
#[repr(C, packed)] pub struct ieee80211_uhr_cap_phy { pub cap:u32, pub reserved:u8 }
#[repr(C, packed)] pub struct ieee80211_uhr_cap { pub mac:ieee80211_uhr_cap_mac, pub phy:ieee80211_uhr_cap_phy, pub variable:[u8;0] }

pub const IEEE80211_UHR_OM_PU_TO_128TU:u8=11;
pub unsafe fn ieee80211_uhr_capa_get_om_pu_to_us(cap:*const ieee80211_uhr_cap)->i32 { let t=(((*cap).mac.mac_cap[3]&3)<<2)|(((*cap).mac.mac_cap[2]>>6)&3); if t>IEEE80211_UHR_OM_PU_TO_128TU{return -1} if t==0{return 0} 128i32 << (t-1) }
pub unsafe fn ieee80211_uhr_dbe_cap(cap:*const ieee80211_uhr_cap)->*const ieee80211_uhr_cap_dbe { if (*cap).mac.mac_cap[1]&IEEE80211_UHR_MAC_CAP1_DBE_SUPP==0 {core::ptr::null()} else {(*cap).variable.as_ptr() as *const _} }
#[repr(C, packed)] pub struct ieee80211_smd_info { pub id:[u8;6], pub capa:u8, pub timeout:u16 }
#[repr(C)] pub enum ieee80211_protected_uhr_action { IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_REQUEST=0, IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_RESPONSE=1, IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_NOTIFY=2 }
#[repr(C)] pub enum ieee80211_uhr_link_reconfig_request_type { IEEE80211_UHR_LINK_RECONFIG_REQUEST_ST_PREP=0, IEEE80211_UHR_LINK_RECONFIG_REQUEST_ST_EXEC=1, IEEE80211_UHR_LINK_RECONFIG_REQUEST_OMP_REQUEST=3 }
#[repr(C)] pub enum ieee80211_uhr_link_reconfig_response_type { IEEE80211_UHR_LINK_RECONFIG_RESPONSE_ST_PREP=0, IEEE80211_UHR_LINK_RECONFIG_RESPONSE_ST_EXEC=1 }
#[repr(C)] pub enum ieee80211_uhr_link_reconfig_notify_type { IEEE80211_UHR_LINK_RECONFIG_NOTIFY_DL_DRAINED=2, IEEE80211_UHR_LINK_RECONFIG_NOTIFY_OMP_RESPONSE=3 }
#[repr(C)] pub enum ieee80211_uhr_mode_change_mode_id { IEEE80211_UHR_MODE_CHANGE_MODE_ID_DPS=0, IEEE80211_UHR_MODE_CHANGE_MODE_ID_NPCA=1, IEEE80211_UHR_MODE_CHANGE_MODE_ID_DUO=2, IEEE80211_UHR_MODE_CHANGE_MODE_ID_DSO=3, IEEE80211_UHR_MODE_CHANGE_MODE_ID_P_EDCA=4, IEEE80211_UHR_MODE_CHANGE_MODE_ID_ELR_RX=5, IEEE80211_UHR_MODE_CHANGE_MODE_ID_AOM=6, IEEE80211_UHR_MODE_CHANGE_MODE_ID_LLI=7, IEEE80211_UHR_MODE_CHANGE_MODE_ID_CO_BF=8, IEEE80211_UHR_MODE_CHANGE_MODE_ID_CO_SR=9, IEEE80211_UHR_MODE_CHANGE_MODE_ID_EMLSR=10, IEEE80211_UHR_MODE_CHANGE_MODE_ID_DBE=11 }
#[repr(C, packed)] pub struct ieee80211_uhr_mode_change_tuple { pub control:u16, pub variable:[u8;0] }
pub const IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_LENGTH:u16=0x0f00;
pub const IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ID:u16=0x003f; pub const IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ENABLE:u16=0x0040; pub const IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_UPDATE:u16=0x0080; pub const IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_SPECIFIC:u16=0xf000;
pub unsafe fn ieee80211_uhr_mode_change_tuple_size(t:*const ieee80211_uhr_mode_change_tuple)->usize { size_of::<ieee80211_uhr_mode_change_tuple>() + (((*t).control & IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_LENGTH)>>8) as usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

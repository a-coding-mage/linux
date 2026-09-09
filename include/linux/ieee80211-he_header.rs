/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/ieee80211-he.h. External kernel types/helpers are
 * intentionally referenced but not implemented here. */

pub const IEEE80211_TWT_CONTROL_NDP: u16 = BIT(0);
pub const IEEE80211_TWT_CONTROL_RESP_MODE: u16 = BIT(1);
pub const IEEE80211_TWT_CONTROL_NEG_TYPE_BROADCAST: u16 = BIT(3);
pub const IEEE80211_TWT_CONTROL_RX_DISABLED: u16 = BIT(4);
pub const IEEE80211_TWT_CONTROL_WAKE_DUR_UNIT: u16 = BIT(5);
pub const IEEE80211_TWT_REQTYPE_REQUEST: u16 = BIT(0);
pub const IEEE80211_TWT_REQTYPE_SETUP_CMD: u16 = GENMASK(3, 1);
pub const IEEE80211_TWT_REQTYPE_TRIGGER: u16 = BIT(4);
pub const IEEE80211_TWT_REQTYPE_IMPLICIT: u16 = BIT(5);
pub const IEEE80211_TWT_REQTYPE_FLOWTYPE: u16 = BIT(6);
pub const IEEE80211_TWT_REQTYPE_FLOWID: u16 = GENMASK(9, 7);
pub const IEEE80211_TWT_REQTYPE_WAKE_INT_EXP: u16 = GENMASK(14, 10);
pub const IEEE80211_TWT_REQTYPE_PROTECTION: u16 = BIT(15);

#[repr(u8)]
pub enum ieee80211_twt_setup_cmd { TWT_SETUP_CMD_REQUEST, TWT_SETUP_CMD_SUGGEST, TWT_SETUP_CMD_DEMAND, TWT_SETUP_CMD_GROUPING, TWT_SETUP_CMD_ACCEPT, TWT_SETUP_CMD_ALTERNATE, TWT_SETUP_CMD_DICTATE, TWT_SETUP_CMD_REJECT }

#[repr(C, packed)]
pub struct ieee80211_twt_params { pub req_type: __le16, pub twt: __le64, pub min_twt_dur: u8, pub mantissa: __le16, pub channel: u8 }
#[repr(C, packed)]
pub struct ieee80211_twt_setup { pub dialog_token: u8, pub element_id: u8, pub length: u8, pub control: u8, pub params: [u8; 0] }
#[repr(C, packed)]
pub struct ieee80211_he_cap_elem { pub mac_cap_info: [u8; 6], pub phy_cap_info: [u8; 11] }
pub const IEEE80211_TX_RX_MCS_NSS_DESC_MAX_LEN: usize = 5;
#[repr(u8)]
pub enum ieee80211_he_mcs_support { IEEE80211_HE_MCS_SUPPORT_0_7=0, IEEE80211_HE_MCS_SUPPORT_0_9=1, IEEE80211_HE_MCS_SUPPORT_0_11=2, IEEE80211_HE_MCS_NOT_SUPPORTED=3 }
#[repr(C, packed)]
pub struct ieee80211_he_mcs_nss_supp { pub rx_mcs_80: __le16, pub tx_mcs_80: __le16, pub rx_mcs_160: __le16, pub tx_mcs_160: __le16, pub rx_mcs_80p80: __le16, pub tx_mcs_80p80: __le16 }
#[repr(C, packed)]
pub struct ieee80211_he_operation { pub he_oper_params: __le32, pub he_mcs_nss_set: __le16, pub optional: [u8; 0] }
#[repr(C, packed)] pub struct ieee80211_he_spr { pub he_sr_control: u8, pub optional: [u8; 0] }
#[repr(C, packed)] pub struct ieee80211_he_mu_edca_param_ac_rec { pub aifsn:u8, pub ecw_min_max:u8, pub mu_edca_timer:u8 }
#[repr(C, packed)] pub struct ieee80211_mu_edca_param_set { pub mu_qos_info:u8, pub ac_be:ieee80211_he_mu_edca_param_ac_rec, pub ac_bk:ieee80211_he_mu_edca_param_ac_rec, pub ac_vi:ieee80211_he_mu_edca_param_ac_rec, pub ac_vo:ieee80211_he_mu_edca_param_ac_rec }

/* 802.11ax HE MAC and PHY capabilities. */
pub const IEEE80211_HE_MAC_CAP0_HTC_HE:u8=0x01; pub const IEEE80211_HE_MAC_CAP0_TWT_REQ:u8=0x02; pub const IEEE80211_HE_MAC_CAP0_TWT_RES:u8=0x04;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_NOT_SUPP:u8=0; pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_1:u8=8; pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_2:u8=0x10; pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_3:u8=0x18; pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_MASK:u8=0x18;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_MASK:u8=0xe0;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_MASK:u8=3; pub const IEEE80211_HE_MAC_CAP1_TF_MAC_PAD_DUR_MASK:u8=0x0c; pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_MASK:u8=0x70; pub const IEEE80211_HE_MAC_CAP1_LINK_ADAPTATION:u8=0x80;
pub const IEEE80211_HE_MAC_CAP2_LINK_ADAPTATION:u8=1; pub const IEEE80211_HE_MAC_CAP2_ALL_ACK:u8=2; pub const IEEE80211_HE_MAC_CAP2_TRS:u8=4; pub const IEEE80211_HE_MAC_CAP2_BSR:u8=8; pub const IEEE80211_HE_MAC_CAP2_BCAST_TWT:u8=0x10; pub const IEEE80211_HE_MAC_CAP2_32BIT_BA_BITMAP:u8=0x20; pub const IEEE80211_HE_MAC_CAP2_MU_CASCADING:u8=0x40; pub const IEEE80211_HE_MAC_CAP2_ACK_EN:u8=0x80;
pub const IEEE80211_HE_MAC_CAP3_OMI_CONTROL:u8=2; pub const IEEE80211_HE_MAC_CAP3_OFDMA_RA:u8=4; pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_MASK:u8=0x18; pub const IEEE80211_HE_MAC_CAP3_AMSDU_FRAG:u8=0x20; pub const IEEE80211_HE_MAC_CAP3_FLEX_TWT_SCHED:u8=0x40; pub const IEEE80211_HE_MAC_CAP3_RX_CTRL_FRAME_TO_MULTIBSS:u8=0x80;
pub const IEEE80211_HE_MAC_CAP4_BSRP_BQRP_A_MPDU_AGG:u8=1; pub const IEEE80211_HE_MAC_CAP4_QTP:u8=2; pub const IEEE80211_HE_MAC_CAP4_BQR:u8=4; pub const IEEE80211_HE_MAC_CAP4_PSR_RESP:u8=8; pub const IEEE80211_HE_MAC_CAP4_NDP_FB_REP:u8=0x10; pub const IEEE80211_HE_MAC_CAP4_OPS:u8=0x20; pub const IEEE80211_HE_MAC_CAP4_AMSDU_IN_AMPDU:u8=0x40; pub const IEEE80211_HE_MAC_CAP4_MULTI_TID_AGG_TX_QOS_B39:u8=0x80;
pub const IEEE80211_HE_MAC_CAP5_MULTI_TID_AGG_TX_QOS_B40:u8=1; pub const IEEE80211_HE_MAC_CAP5_MULTI_TID_AGG_TX_QOS_B41:u8=2; pub const IEEE80211_HE_MAC_CAP5_SUBCHAN_SELECTIVE_TRANSMISSION:u8=4; pub const IEEE80211_HE_MAC_CAP5_UL_2x996_TONE_RU:u8=8; pub const IEEE80211_HE_MAC_CAP5_OM_CTRL_UL_MU_DATA_DIS_RX:u8=0x10; pub const IEEE80211_HE_MAC_CAP5_HE_DYNAMIC_SM_PS:u8=0x20; pub const IEEE80211_HE_MAC_CAP5_PUNCTURED_SOUNDING:u8=0x40; pub const IEEE80211_HE_MAC_CAP5_HT_VHT_TRIG_FRAME_RX:u8=0x80;
pub const IEEE80211_HE_VHT_MAX_AMPDU_FACTOR:u8=20; pub const IEEE80211_HE_HT_MAX_AMPDU_FACTOR:u8=16; pub const IEEE80211_HE_6GHZ_MAX_AMPDU_FACTOR:u8=13;
pub const IEEE80211_HE_PHY_CAP6_PPE_THRESHOLD_PRESENT:u8=0x80;
pub const IEEE80211_TX_RX_MCS_NSS_SUPP_HIGHEST_MCS_POS:u8=3; pub const IEEE80211_TX_RX_MCS_NSS_SUPP_TX_BITMAP_POS:u8=6; pub const IEEE80211_TX_RX_MCS_NSS_SUPP_RX_BITMAP_POS:u8=11; pub const IEEE80211_TX_RX_MCS_NSS_SUPP_TX_BITMAP_MASK:u16=0x07c0; pub const IEEE80211_TX_RX_MCS_NSS_SUPP_RX_BITMAP_MASK:u16=0xf800;
#[repr(u8)] pub enum ieee80211_he_highest_mcs_supported_subfield_enc { HIGHEST_MCS_SUPPORTED_MCS7=0, HIGHEST_MCS_SUPPORTED_MCS8, HIGHEST_MCS_SUPPORTED_MCS9, HIGHEST_MCS_SUPPORTED_MCS10, HIGHEST_MCS_SUPPORTED_MCS11 }
pub unsafe fn ieee80211_he_mcs_nss_size(he_cap:*const ieee80211_he_cap_elem)->u8 { let mut count=4; if (*he_cap).phy_cap_info[0] & 8 != 0 {count+=4;} if (*he_cap).phy_cap_info[0] & 0x10 != 0 {count+=4;} count }
pub const IEEE80211_PPE_THRES_NSS_SUPPORT_2NSS:u8=1; pub const IEEE80211_PPE_THRES_NSS_POS:u8=0; pub const IEEE80211_PPE_THRES_NSS_MASK:u8=7; pub const IEEE80211_PPE_THRES_RU_INDEX_BITMASK_MASK:u8=0x78; pub const IEEE80211_PPE_THRES_RU_INDEX_BITMASK_POS:u8=3; pub const IEEE80211_PPE_THRES_INFO_PPET_SIZE:u8=3; pub const IEEE80211_HE_PPE_THRES_INFO_HEADER_SIZE:u8=7;
pub const IEEE80211_HE_OPERATION_DFLT_PE_DURATION_MASK:u32=7; pub const IEEE80211_HE_OPERATION_TWT_REQUIRED:u32=8; pub const IEEE80211_HE_OPERATION_RTS_THRESHOLD_MASK:u32=0x3ff0; pub const IEEE80211_HE_OPERATION_RTS_THRESHOLD_OFFSET:u32=4; pub const IEEE80211_HE_OPERATION_VHT_OPER_INFO:u32=0x4000; pub const IEEE80211_HE_OPERATION_CO_HOSTED_BSS:u32=0x8000; pub const IEEE80211_HE_OPERATION_6GHZ_OP_INFO:u32=0x20000;
#[repr(C,packed)] pub struct ieee80211_he_6ghz_oper { pub primary:u8,pub control:u8,pub ccfs0:u8,pub ccfs1:u8,pub minrate:u8 }
pub const IEEE80211_HE_SPR_NON_SRG_OFFSET_PRESENT:u8=4; pub const IEEE80211_HE_SPR_SRG_INFORMATION_PRESENT:u8=8;
#[repr(C,packed)] pub struct ieee80211_he_6ghz_capa { pub capa:__le16 }
pub const IEEE80211_HE_6GHZ_CAP_MIN_MPDU_START:u16=7; pub const IEEE80211_HE_6GHZ_CAP_MAX_AMPDU_LEN_EXP:u16=0x38; pub const IEEE80211_HE_6GHZ_CAP_MAX_MPDU_LEN:u16=0xc0; pub const IEEE80211_HE_6GHZ_CAP_SM_PS:u16=0x600; pub const IEEE80211_HE_6GHZ_CAP_RD_RESPONDER:u16=0x800; pub const IEEE80211_HE_6GHZ_CAP_RX_ANTPAT_CONS:u16=0x1000; pub const IEEE80211_HE_6GHZ_CAP_TX_ANTPAT_CONS:u16=0x2000;
#[repr(C,packed)] pub struct ieee80211_tx_pwr_env { pub info:u8, pub variable:[u8;0] }
#[repr(u8)] pub enum ieee80211_tx_power_intrpt_type { IEEE80211_TPE_LOCAL_EIRP, IEEE80211_TPE_LOCAL_EIRP_PSD, IEEE80211_TPE_REG_CLIENT_EIRP, IEEE80211_TPE_REG_CLIENT_EIRP_PSD }
#[repr(u8)] pub enum ieee80211_tx_power_category_6ghz { IEEE80211_TPE_CAT_6GHZ_DEFAULT=0, IEEE80211_TPE_CAT_6GHZ_SUBORDINATE=1 }
pub const IEEE80211_TPE_MAX_TX_PWR_NO_CONSTRAINT:u8=127; pub const IEEE80211_TPE_PSD_NO_LIMIT:u8=127; pub const IEEE80211_TX_PWR_ENV_INFO_COUNT:u8=7; pub const IEEE80211_TX_PWR_ENV_INFO_INTERPRET:u8=0x38; pub const IEEE80211_TX_PWR_ENV_INFO_CATEGORY:u8=0xc0; pub const IEEE80211_TX_PWR_ENV_EXT_COUNT:u8=0xf;
pub unsafe fn ieee80211_he_capa_size_ok(data:*const u8,len:u8)->bool { if len < 17{return false} let cap=data as *const ieee80211_he_cap_elem; let mut needed=17+ieee80211_he_mcs_nss_size(cap); if (*cap).phy_cap_info[6]&0x80!=0 {if len<needed+1{return false}; needed+=1;} len>=needed }
pub unsafe fn ieee80211_he_6ghz_oper(he_oper:*const ieee80211_he_operation)->*const ieee80211_he_6ghz_oper { if he_oper.is_null(){return core::ptr::null()} let p=u32::from_le((*he_oper).he_oper_params); if p&0x20000==0{return core::ptr::null()} let mut r=(*he_oper).optional.as_ptr(); if p&0x4000!=0{r=r.add(3)} if p&0x8000!=0{r=r.add(1)} r as *const ieee80211_he_6ghz_oper }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

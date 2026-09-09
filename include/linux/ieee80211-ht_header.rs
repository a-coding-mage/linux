/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE 802.11 HT definitions */

// Dependencies supplied by the surrounding Linux translation.

pub const IEEE80211_MAX_MPDU_LEN_HT_BA: u32 = 4095;
pub const IEEE80211_MAX_MPDU_LEN_HT_3839: u32 = 3839;
pub const IEEE80211_MAX_MPDU_LEN_HT_7935: u32 = 7935;
pub const IEEE80211_HT_CTL_LEN: u32 = 4;

#[repr(C)]
pub enum ieee80211_ht_chanwidth_values {
    IEEE80211_HT_CHANWIDTH_20MHZ = 0,
    IEEE80211_HT_CHANWIDTH_ANY = 1,
}

#[repr(C, packed)]
pub struct ieee80211_bar {
    pub frame_control: __le16,
    pub duration: __le16,
    pub ra: [__u8; ETH_ALEN],
    pub ta: [__u8; ETH_ALEN],
    pub control: __le16,
    pub start_seq_num: __le16,
}

pub const IEEE80211_BAR_CTRL_ACK_POLICY_NORMAL: u32 = 0x0000;
pub const IEEE80211_BAR_CTRL_MULTI_TID: u32 = 0x0002;
pub const IEEE80211_BAR_CTRL_CBMTID_COMPRESSED_BA: u32 = 0x0004;
pub const IEEE80211_BAR_CTRL_TID_INFO_MASK: u32 = 0xf000;
pub const IEEE80211_BAR_CTRL_TID_INFO_SHIFT: u32 = 12;

pub const IEEE80211_HT_MCS_MASK_LEN: usize = 10;

#[repr(C, packed)]
pub struct ieee80211_mcs_info {
    pub rx_mask: [u8; IEEE80211_HT_MCS_MASK_LEN],
    pub rx_highest: __le16,
    pub tx_params: u8,
    pub reserved: [u8; 3],
}

pub const IEEE80211_HT_MCS_RX_HIGHEST_MASK: u32 = 0x3ff;
pub const IEEE80211_HT_MCS_TX_DEFINED: u32 = 0x01;
pub const IEEE80211_HT_MCS_TX_RX_DIFF: u32 = 0x02;
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS_MASK: u32 = 0x0C;
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS_SHIFT: u32 = 2;
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS: u32 = 4;
pub const IEEE80211_HT_MCS_TX_UNEQUAL_MODULATION: u32 = 0x10;

#[inline]
pub const fn IEEE80211_HT_MCS_CHAINS(mcs: u32) -> u32 {
    if mcs == 32 { 1 } else { 1 + (mcs >> 3) }
}

pub const IEEE80211_HT_MCS_UNEQUAL_MODULATION_START: u32 = 33;
pub const IEEE80211_HT_MCS_UNEQUAL_MODULATION_START_BYTE: u32 =
    IEEE80211_HT_MCS_UNEQUAL_MODULATION_START / 8;

#[repr(C, packed)]
pub struct ieee80211_ht_cap {
    pub cap_info: __le16,
    pub ampdu_params_info: u8,
    pub mcs: ieee80211_mcs_info,
    pub extended_ht_cap_info: __le16,
    pub tx_BF_cap_info: __le32,
    pub antenna_selection_info: u8,
}

pub const IEEE80211_HT_CAP_LDPC_CODING: u32 = 0x0001;
pub const IEEE80211_HT_CAP_SUP_WIDTH_20_40: u32 = 0x0002;
pub const IEEE80211_HT_CAP_SM_PS: u32 = 0x000C;
pub const IEEE80211_HT_CAP_SM_PS_SHIFT: u32 = 2;
pub const IEEE80211_HT_CAP_GRN_FLD: u32 = 0x0010;
pub const IEEE80211_HT_CAP_SGI_20: u32 = 0x0020;
pub const IEEE80211_HT_CAP_SGI_40: u32 = 0x0040;
pub const IEEE80211_HT_CAP_TX_STBC: u32 = 0x0080;
pub const IEEE80211_HT_CAP_RX_STBC: u32 = 0x0300;
pub const IEEE80211_HT_CAP_RX_STBC_SHIFT: u32 = 8;
pub const IEEE80211_HT_CAP_DELAY_BA: u32 = 0x0400;
pub const IEEE80211_HT_CAP_MAX_AMSDU: u32 = 0x0800;
pub const IEEE80211_HT_CAP_DSSSCCK40: u32 = 0x1000;
pub const IEEE80211_HT_CAP_RESERVED: u32 = 0x2000;
pub const IEEE80211_HT_CAP_40MHZ_INTOLERANT: u32 = 0x4000;
pub const IEEE80211_HT_CAP_LSIG_TXOP_PROT: u32 = 0x8000;

pub const IEEE80211_HT_EXT_CAP_PCO: u32 = 0x0001;
pub const IEEE80211_HT_EXT_CAP_PCO_TIME: u32 = 0x0006;
pub const IEEE80211_HT_EXT_CAP_PCO_TIME_SHIFT: u32 = 1;
pub const IEEE80211_HT_EXT_CAP_MCS_FB: u32 = 0x0300;
pub const IEEE80211_HT_EXT_CAP_MCS_FB_SHIFT: u32 = 8;
pub const IEEE80211_HT_EXT_CAP_HTC_SUP: u32 = 0x0400;
pub const IEEE80211_HT_EXT_CAP_RD_RESPONDER: u32 = 0x0800;

pub const IEEE80211_HT_AMPDU_PARM_FACTOR: u32 = 0x03;
pub const IEEE80211_HT_AMPDU_PARM_DENSITY: u32 = 0x1C;
pub const IEEE80211_HT_AMPDU_PARM_DENSITY_SHIFT: u32 = 2;

#[repr(C)]
pub enum ieee80211_max_ampdu_length_exp {
    IEEE80211_HT_MAX_AMPDU_8K = 0,
    IEEE80211_HT_MAX_AMPDU_16K = 1,
    IEEE80211_HT_MAX_AMPDU_32K = 2,
    IEEE80211_HT_MAX_AMPDU_64K = 3,
}

pub const IEEE80211_HT_MAX_AMPDU_FACTOR: u32 = 13;

#[repr(C)]
pub enum ieee80211_min_mpdu_spacing {
    IEEE80211_HT_MPDU_DENSITY_NONE = 0,
    IEEE80211_HT_MPDU_DENSITY_0_25 = 1,
    IEEE80211_HT_MPDU_DENSITY_0_5 = 2,
    IEEE80211_HT_MPDU_DENSITY_1 = 3,
    IEEE80211_HT_MPDU_DENSITY_2 = 4,
    IEEE80211_HT_MPDU_DENSITY_4 = 5,
    IEEE80211_HT_MPDU_DENSITY_8 = 6,
    IEEE80211_HT_MPDU_DENSITY_16 = 7,
}

#[repr(C, packed)]
pub struct ieee80211_ht_operation {
    pub primary_chan: u8,
    pub ht_param: u8,
    pub operation_mode: __le16,
    pub stbc_param: __le16,
    pub basic_set: [u8; 16],
}

pub const IEEE80211_HT_PARAM_CHA_SEC_OFFSET: u32 = 0x03;
pub const IEEE80211_HT_PARAM_CHA_SEC_NONE: u32 = 0x00;
pub const IEEE80211_HT_PARAM_CHA_SEC_ABOVE: u32 = 0x01;
pub const IEEE80211_HT_PARAM_CHA_SEC_BELOW: u32 = 0x03;
pub const IEEE80211_HT_PARAM_CHAN_WIDTH_ANY: u32 = 0x04;
pub const IEEE80211_HT_PARAM_RIFS_MODE: u32 = 0x08;

pub const IEEE80211_HT_OP_MODE_PROTECTION: u32 = 0x0003;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONE: u32 = 0;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONMEMBER: u32 = 1;
pub const IEEE80211_HT_OP_MODE_PROTECTION_20MHZ: u32 = 2;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONHT_MIXED: u32 = 3;
pub const IEEE80211_HT_OP_MODE_NON_GF_STA_PRSNT: u32 = 0x0004;
pub const IEEE80211_HT_OP_MODE_NON_HT_STA_PRSNT: u32 = 0x0010;
pub const IEEE80211_HT_OP_MODE_CCFS2_SHIFT: u32 = 5;
pub const IEEE80211_HT_OP_MODE_CCFS2_MASK: u32 = 0x1fe0;

pub const IEEE80211_HT_STBC_PARAM_DUAL_BEACON: u32 = 0x0040;
pub const IEEE80211_HT_STBC_PARAM_DUAL_CTS_PROT: u32 = 0x0080;
pub const IEEE80211_HT_STBC_PARAM_STBC_BEACON: u32 = 0x0100;
pub const IEEE80211_HT_STBC_PARAM_LSIG_TXOP_FULLPROT: u32 = 0x0200;
pub const IEEE80211_HT_STBC_PARAM_PCO_ACTIVE: u32 = 0x0400;
pub const IEEE80211_HT_STBC_PARAM_PCO_PHASE: u32 = 0x0800;

pub const IEEE80211_ADDBA_PARAM_AMSDU_MASK: u32 = 0x0001;
pub const IEEE80211_ADDBA_PARAM_POLICY_MASK: u32 = 0x0002;
pub const IEEE80211_ADDBA_PARAM_TID_MASK: u32 = 0x003C;
pub const IEEE80211_ADDBA_PARAM_BUF_SIZE_MASK: u32 = 0xFFC0;
pub const IEEE80211_DELBA_PARAM_TID_MASK: u32 = 0xF000;
pub const IEEE80211_DELBA_PARAM_INITIATOR_MASK: u32 = 0x0800;

pub const IEEE80211_MIN_AMPDU_BUF: u32 = 0x8;
pub const IEEE80211_MAX_AMPDU_BUF_HT: u32 = 0x40;
pub const IEEE80211_MAX_AMPDU_BUF_HE: u32 = 0x100;
pub const IEEE80211_MAX_AMPDU_BUF_EHT: u32 = 0x400;

pub const WLAN_HT_CAP_SM_PS_STATIC: u32 = 0;
pub const WLAN_HT_CAP_SM_PS_DYNAMIC: u32 = 1;
pub const WLAN_HT_CAP_SM_PS_INVALID: u32 = 2;
pub const WLAN_HT_CAP_SM_PS_DISABLED: u32 = 3;
pub const WLAN_HT_SMPS_CONTROL_DISABLED: u32 = 0;
pub const WLAN_HT_SMPS_CONTROL_STATIC: u32 = 1;
pub const WLAN_HT_SMPS_CONTROL_DYNAMIC: u32 = 3;

#[repr(C)]
pub enum ieee80211_ht_actioncode {
    WLAN_HT_ACTION_NOTIFY_CHANWIDTH = 0,
    WLAN_HT_ACTION_SMPS = 1,
    WLAN_HT_ACTION_PSMP = 2,
    WLAN_HT_ACTION_PCO_PHASE = 3,
    WLAN_HT_ACTION_CSI = 4,
    WLAN_HT_ACTION_NONCOMPRESSED_BF = 5,
    WLAN_HT_ACTION_COMPRESSED_BF = 6,
    WLAN_HT_ACTION_ASEL_IDX_FEEDBACK = 7,
}

#[repr(C)]
pub enum ieee80211_back_actioncode {
    WLAN_ACTION_ADDBA_REQ = 0,
    WLAN_ACTION_ADDBA_RESP = 1,
    WLAN_ACTION_DELBA = 2,
    WLAN_ACTION_NDP_ADDBA_REQ = 128,
    WLAN_ACTION_NDP_ADDBA_RESP = 129,
    WLAN_ACTION_NDP_DELBA = 130,
}

#[repr(C)]
pub enum ieee80211_back_parties {
    WLAN_BACK_RECIPIENT = 0,
    WLAN_BACK_INITIATOR = 1,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

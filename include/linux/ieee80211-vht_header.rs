/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IEEE 802.11 VHT definitions
 *
 * Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
 * <jkmaline@cc.hut.fi>
 * Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
 * Copyright (c) 2005, Devicescape Software, Inc.
 * Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
 * Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
 * Copyright (c) 2016 - 2017 Intel Deutschland GmbH
 * Copyright (c) 2018 - 2025 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit: linux/types.h,
// linux/if_ether.h.

pub const IEEE80211_MAX_MPDU_LEN_VHT_3895: u32 = 3895;
pub const IEEE80211_MAX_MPDU_LEN_VHT_7991: u32 = 7991;
pub const IEEE80211_MAX_MPDU_LEN_VHT_11454: u32 = 11454;

#[repr(i32)]
pub enum ieee80211_vht_opmode_bits {
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_MASK = 0x03,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_20MHZ = 0,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_40MHZ = 1,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_80MHZ = 2,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_160MHZ = 3,
    IEEE80211_OPMODE_NOTIF_BW_160_80P80 = 0x04,
    IEEE80211_OPMODE_NOTIF_RX_NSS_MASK = 0x70,
    IEEE80211_OPMODE_NOTIF_RX_NSS_SHIFT = 4,
    IEEE80211_OPMODE_NOTIF_RX_NSS_TYPE_BF = 0x80,
}

#[repr(i32)]
pub enum ieee80211_vht_max_ampdu_length_exp {
    IEEE80211_VHT_MAX_AMPDU_8K = 0,
    IEEE80211_VHT_MAX_AMPDU_16K = 1,
    IEEE80211_VHT_MAX_AMPDU_32K = 2,
    IEEE80211_VHT_MAX_AMPDU_64K = 3,
    IEEE80211_VHT_MAX_AMPDU_128K = 4,
    IEEE80211_VHT_MAX_AMPDU_256K = 5,
    IEEE80211_VHT_MAX_AMPDU_512K = 6,
    IEEE80211_VHT_MAX_AMPDU_1024K = 7,
}

#[repr(C, packed)]
pub struct ieee80211_vht_mcs_info {
    pub rx_mcs_map: __le16,
    pub rx_highest: __le16,
    pub tx_mcs_map: __le16,
    pub tx_highest: __le16,
}

pub const IEEE80211_VHT_MAX_NSTS_TOTAL_SHIFT: u32 = 13;
pub const IEEE80211_VHT_MAX_NSTS_TOTAL_MASK: u32 = 7 << IEEE80211_VHT_MAX_NSTS_TOTAL_SHIFT;
pub const IEEE80211_VHT_EXT_NSS_BW_CAPABLE: u32 = 1 << 13;

#[repr(i32)]
pub enum ieee80211_vht_mcs_support {
    IEEE80211_VHT_MCS_SUPPORT_0_7 = 0,
    IEEE80211_VHT_MCS_SUPPORT_0_8 = 1,
    IEEE80211_VHT_MCS_SUPPORT_0_9 = 2,
    IEEE80211_VHT_MCS_NOT_SUPPORTED = 3,
}

#[repr(C, packed)]
pub struct ieee80211_vht_cap {
    pub vht_cap_info: __le32,
    pub supp_mcs: ieee80211_vht_mcs_info,
}

#[repr(i32)]
pub enum ieee80211_vht_chanwidth {
    IEEE80211_VHT_CHANWIDTH_USE_HT = 0,
    IEEE80211_VHT_CHANWIDTH_80MHZ = 1,
    IEEE80211_VHT_CHANWIDTH_160MHZ = 2,
    IEEE80211_VHT_CHANWIDTH_80P80MHZ = 3,
}

#[repr(C, packed)]
pub struct ieee80211_vht_operation {
    pub chan_width: u8,
    pub center_freq_seg0_idx: u8,
    pub center_freq_seg1_idx: u8,
    pub basic_mcs_set: __le16,
}

pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_3895: u32 = 0x00000000;
pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_7991: u32 = 0x00000001;
pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_11454: u32 = 0x00000002;
pub const IEEE80211_VHT_CAP_MAX_MPDU_MASK: u32 = 0x00000003;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160MHZ: u32 = 0x00000004;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160_80PLUS80MHZ: u32 = 0x00000008;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_MASK: u32 = 0x0000000C;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_SHIFT: u32 = 2;
pub const IEEE80211_VHT_CAP_RXLDPC: u32 = 0x00000010;
pub const IEEE80211_VHT_CAP_SHORT_GI_80: u32 = 0x00000020;
pub const IEEE80211_VHT_CAP_SHORT_GI_160: u32 = 0x00000040;
pub const IEEE80211_VHT_CAP_TXSTBC: u32 = 0x00000080;
pub const IEEE80211_VHT_CAP_RXSTBC_1: u32 = 0x00000100;
pub const IEEE80211_VHT_CAP_RXSTBC_2: u32 = 0x00000200;
pub const IEEE80211_VHT_CAP_RXSTBC_3: u32 = 0x00000300;
pub const IEEE80211_VHT_CAP_RXSTBC_4: u32 = 0x00000400;
pub const IEEE80211_VHT_CAP_RXSTBC_MASK: u32 = 0x00000700;
pub const IEEE80211_VHT_CAP_RXSTBC_SHIFT: u32 = 8;
pub const IEEE80211_VHT_CAP_SU_BEAMFORMER_CAPABLE: u32 = 0x00000800;
pub const IEEE80211_VHT_CAP_SU_BEAMFORMEE_CAPABLE: u32 = 0x00001000;
pub const IEEE80211_VHT_CAP_BEAMFORMEE_STS_SHIFT: u32 = 13;
pub const IEEE80211_VHT_CAP_BEAMFORMEE_STS_MASK: u32 = 7 << IEEE80211_VHT_CAP_BEAMFORMEE_STS_SHIFT;
pub const IEEE80211_VHT_CAP_SOUNDING_DIMENSIONS_SHIFT: u32 = 16;
pub const IEEE80211_VHT_CAP_SOUNDING_DIMENSIONS_MASK: u32 = 7 << IEEE80211_VHT_CAP_SOUNDING_DIMENSIONS_SHIFT;
pub const IEEE80211_VHT_CAP_MU_BEAMFORMER_CAPABLE: u32 = 0x00080000;
pub const IEEE80211_VHT_CAP_MU_BEAMFORMEE_CAPABLE: u32 = 0x00100000;
pub const IEEE80211_VHT_CAP_VHT_TXOP_PS: u32 = 0x00200000;
pub const IEEE80211_VHT_CAP_HTC_VHT: u32 = 0x00400000;
pub const IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT: u32 = 23;
pub const IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK: u32 = 7 << IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT;
pub const IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_UNSOL_MFB: u32 = 0x08000000;
pub const IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_MRQ_MFB: u32 = 0x0c000000;
pub const IEEE80211_VHT_CAP_RX_ANTENNA_PATTERN: u32 = 0x10000000;
pub const IEEE80211_VHT_CAP_TX_ANTENNA_PATTERN: u32 = 0x20000000;
pub const IEEE80211_VHT_CAP_EXT_NSS_BW_SHIFT: u32 = 30;
pub const IEEE80211_VHT_CAP_EXT_NSS_BW_MASK: u32 = 0xc0000000;

extern "C" {
    pub fn ieee80211_get_vht_max_nss(
        cap: *mut ieee80211_vht_cap,
        bw: ieee80211_vht_chanwidth,
        mcs: i32,
        ext_nss_bw_capable: bool,
        max_vht_nss: u32,
    ) -> i32;
}

#[repr(i32)]
pub enum ieee80211_vht_actioncode {
    WLAN_VHT_ACTION_COMPRESSED_BF = 0,
    WLAN_VHT_ACTION_GROUPID_MGMT = 1,
    WLAN_VHT_ACTION_OPMODE_NOTIF = 2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

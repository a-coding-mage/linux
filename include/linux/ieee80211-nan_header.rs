/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * WFA NAN definitions
 *
 * Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
 * <jkmaline@cc.hut.fi>
 * Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
 * Copyright (c) 2005, Devicescape Software, Inc.
 * Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
 * Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
 * Copyright (c) 2016 - 2017 Intel Deutschland GmbH
 * Copyright (c) 2018 - 2026 Intel Corporation
 */

// NAN operation mode, as defined in Wi-Fi Aware (TM) specification Table 81
pub const NAN_OP_MODE_PHY_MODE_VHT: u8 = 0x01;
pub const NAN_OP_MODE_PHY_MODE_HE: u8 = 0x10;
pub const NAN_OP_MODE_PHY_MODE_MASK: u8 = 0x11;
pub const NAN_OP_MODE_80P80MHZ: u8 = 0x02;
pub const NAN_OP_MODE_160MHZ: u8 = 0x04;
pub const NAN_OP_MODE_PNDL_SUPPRTED: u8 = 0x08;

pub const NAN_DEV_CAPA_NUM_TX_ANT_POS: u8 = 0;
pub const NAN_DEV_CAPA_NUM_TX_ANT_MASK: u8 = 0x0f;
pub const NAN_DEV_CAPA_NUM_RX_ANT_POS: u8 = 4;
pub const NAN_DEV_CAPA_NUM_RX_ANT_MASK: u8 = 0xf0;

/* NAN Device capabilities, as defined in Wi-Fi Aware (TM) specification
 * Table 79
 */
pub const NAN_DEV_CAPA_DFS_OWNER: u8 = 0x01;
pub const NAN_DEV_CAPA_EXT_KEY_ID_SUPPORTED: u8 = 0x02;
pub const NAN_DEV_CAPA_SIM_NDP_RX_SUPPORTED: u8 = 0x04;
pub const NAN_DEV_CAPA_NDPE_SUPPORTED: u8 = 0x08;
pub const NAN_DEV_CAPA_S3_SUPPORTED: u8 = 0x10;

/* NAN attributes, as defined in Wi-Fi Aware (TM) specification 4.0 Table 42 */
pub const NAN_ATTR_MASTER_INDICATION: u8 = 0x00;
pub const NAN_ATTR_CLUSTER_INFO: u8 = 0x01;

#[repr(C, packed)]
pub struct ieee80211_nan_attr {
    pub attr: u8,
    pub length: __le16,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct ieee80211_nan_master_indication {
    pub master_pref: u8,
    pub random_factor: u8,
}

#[repr(C, packed)]
pub union ieee80211_nan_anchor_master_info_union {
    pub master_rank: __le64,
    pub fields: ieee80211_nan_anchor_master_info_fields,
}

#[repr(C, packed)]
pub struct ieee80211_nan_anchor_master_info_fields {
    pub master_addr: [u8; ETH_ALEN],
    pub random_factor: u8,
    pub master_pref: u8,
}

#[repr(C, packed)]
pub struct ieee80211_nan_anchor_master_info {
    pub union_: ieee80211_nan_anchor_master_info_union,
    pub hop_count: u8,
    pub ambtt: __le32,
}

/* The C macro iterates over packed NAN attributes in a byte buffer. */
#[macro_export]
macro_rules! for_each_nan_attr {
    ($attr:ident, $data:expr, $datalen:expr, $body:block) => {{
        let mut $attr = $data as *const ieee80211_nan_attr;
        while unsafe {
            ($data as *const u8).add($datalen) as usize - $attr as usize
                >= core::mem::size_of::<ieee80211_nan_attr>()
                && ($data as *const u8).add($datalen) as usize - $attr as usize
                    >= core::mem::size_of::<ieee80211_nan_attr>()
                        + le16_to_cpu((*$attr).length) as usize
        } {
            $body
            $attr = unsafe {
                (*$attr).data.as_ptr().add(le16_to_cpu((*$attr).length) as usize)
                    as *const ieee80211_nan_attr
            };
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

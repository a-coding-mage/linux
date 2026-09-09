/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust source-level translation of ieee80211_i.h.
 *
 * The declarations below intentionally retain the Linux/mac80211 names and
 * external types.  Those types are supplied by the surrounding translation
 * unit, just as they are supplied by the original header's includes.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_imports)]

pub const AP_MAX_BC_BUFFER: usize = 128;
pub const TOTAL_MAX_TX_BUFFER: usize = 512;
pub const IEEE80211_ENCRYPT_HEADROOM: usize = 8;
pub const IEEE80211_ENCRYPT_TAILROOM: usize = 18;
pub const IEEE80211_DEFAULT_UAPSD_QUEUES: u32 = 0;
pub const IEEE80211_DEAUTH_FRAME_LEN: usize = 24 + 2;
pub const IEEE80211_MAX_NAN_INSTANCE_ID: u32 = 255;
pub const IEEE80211_MAX_SUPPORTED_S1G_AID: u32 = 1600;
pub const IEEE80211_MAX_SUPPORTED_S1G_TIM_BLOCKS: u32 = 25;

pub const IEEE80211_TX_UNICAST: u32 = 1 << 1;
pub const IEEE80211_TX_PS_BUFFERED: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bss {
    pub device_ts_beacon: u32,
    pub device_ts_presp: u32,
    pub wmm_used: bool,
    pub uapsd_supported: bool,
    pub supp_rates: [u8; 32],
    pub supp_rates_len: usize,
    pub beacon_rate: *mut ieee80211_rate,
    pub vht_cap_info: u32,
    pub has_erp_value: bool,
    pub erp_value: u8,
    pub corrupt_data: u8,
    pub valid_data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_csa_settings {
    pub counter_offsets_beacon: *const u16,
    pub counter_offsets_presp: *const u16,
    pub n_counter_offsets_beacon: i32,
    pub n_counter_offsets_presp: i32,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_color_change_settings {
    pub counter_offset_beacon: u16,
    pub counter_offset_presp: u16,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_stats {
    pub fwded_mcast: u32,
    pub fwded_unicast: u32,
    pub fwded_frames: u32,
    pub dropped_frames_ttl: u32,
    pub dropped_frames_no_route: u32,
}

pub const PREQ_Q_F_START: u8 = 0x1;
pub const PREQ_Q_F_REFRESH: u8 = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_preq_queue {
    pub list: list_head,
    pub dst: [u8; ETH_ALEN],
    pub flags: u8,
}

#[repr(C)]
pub struct ieee80211_tx_data {
    pub skb: *mut sk_buff,
    pub skbs: sk_buff_head,
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub sta: *mut sta_info,
    pub key: *mut ieee80211_key,
    pub rate: ieee80211_tx_rate,
    pub flags: u32,
}

#[repr(C)]
pub struct ieee80211_rx_data {
    pub list: *mut list_head,
    pub skb: *mut sk_buff,
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub link: *mut ieee80211_link_data,
    pub sta: *mut sta_info,
    pub link_sta: *mut link_sta_info,
    pub key: *mut ieee80211_key,
    pub flags: u32,
    pub seqno_idx: i32,
    pub security_idx: i32,
    pub link_id: i32,
}

#[repr(C)]
pub struct ieee80211_local;
#[repr(C)]
pub struct ieee80211_sub_if_data;
#[repr(C)]
pub struct ieee80211_link_data;
#[repr(C)]
pub struct ieee80211_rate;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct sk_buff_head;
#[repr(C)]
pub struct ieee80211_tx_rate;
#[repr(C)]
pub struct ieee80211_key;
#[repr(C)]
pub struct sta_info;
#[repr(C)]
pub struct link_sta_info;
#[repr(C)]
pub struct list_head;

extern "C" {
    pub fn ieee80211_vif_inc_num_mcast(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_vif_dec_num_mcast(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_handle_queued_frames(local: *mut ieee80211_local);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

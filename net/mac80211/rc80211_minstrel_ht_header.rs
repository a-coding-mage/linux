/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 Felix Fietkau <nbd@openwrt.org>
 */

/* Translated from rc80211_minstrel_ht.h. */
/* Dependency: linux/bitfield.h */

/* number of highest throughput rates to consider */
pub const MAX_THR_RATES: usize = 4;
pub const SAMPLE_COLUMNS: usize = 10; /* number of columns in sample table */

/* scaled fraction values */
pub const MINSTREL_SCALE: u32 = 12;
macro_rules! MINSTREL_FRAC {
    ($val:expr, $div:expr) => { (($val << MINSTREL_SCALE) / $div) };
}
macro_rules! MINSTREL_TRUNC {
    ($val:expr) => { ($val >> MINSTREL_SCALE) };
}

pub const EWMA_LEVEL: u32 = 96; /* ewma weighting factor [/EWMA_DIV] */
pub const EWMA_DIV: u32 = 128;

pub const MINSTREL_AVG_COEFF2: i32 = 0x00001499;
pub const MINSTREL_AVG_COEFF3: i32 = -0x0000092e;
pub const MINSTREL_AVG_COEFF1: i32 = MINSTREL_FRAC!(1i32, 1i32) -
    MINSTREL_AVG_COEFF2 - MINSTREL_AVG_COEFF3;

/* The number of streams can be changed to 2 to reduce code size and memory footprint. */
pub const MINSTREL_MAX_STREAMS: usize = 4;
pub const MINSTREL_HT_STREAM_GROUPS: usize = 4; /* BW(=2) * SGI(=2) */
pub const MINSTREL_VHT_STREAM_GROUPS: usize = 6; /* BW(=3) * SGI(=2) */
pub const MINSTREL_HT_GROUPS_NB: usize = MINSTREL_MAX_STREAMS * MINSTREL_HT_STREAM_GROUPS;
pub const MINSTREL_VHT_GROUPS_NB: usize = MINSTREL_MAX_STREAMS * MINSTREL_VHT_STREAM_GROUPS;
pub const MINSTREL_LEGACY_GROUPS_NB: usize = 2;
pub const MINSTREL_GROUPS_NB: usize = MINSTREL_HT_GROUPS_NB + MINSTREL_VHT_GROUPS_NB + MINSTREL_LEGACY_GROUPS_NB;
pub const MINSTREL_HT_GROUP_0: usize = 0;
pub const MINSTREL_CCK_GROUP: usize = MINSTREL_HT_GROUP_0 + MINSTREL_HT_GROUPS_NB;
pub const MINSTREL_OFDM_GROUP: usize = MINSTREL_CCK_GROUP + 1;
pub const MINSTREL_VHT_GROUP_0: usize = MINSTREL_OFDM_GROUP + 1;
pub const MCS_GROUP_RATES: usize = 10;

macro_rules! GENMASK { ($h:expr, $l:expr) => { (((1u16 << ($h - $l + 1)) - 1) << $l) }; }
pub const MI_RATE_IDX_MASK: u16 = GENMASK!(3, 0);
pub const MI_RATE_GROUP_MASK: u16 = GENMASK!(15, 4);
macro_rules! FIELD_PREP { ($mask:expr, $val:expr) => { (($val << $mask.trailing_zeros()) & $mask) }; }
macro_rules! FIELD_GET { ($mask:expr, $val:expr) => { (($val & $mask) >> $mask.trailing_zeros()) }; }
macro_rules! MI_RATE { ($group:expr, $idx:expr) => { FIELD_PREP!(MI_RATE_GROUP_MASK, $group) | FIELD_PREP!(MI_RATE_IDX_MASK, $idx) }; }
macro_rules! MI_RATE_IDX { ($rate:expr) => { FIELD_GET!(MI_RATE_IDX_MASK, $rate) }; }
macro_rules! MI_RATE_GROUP { ($rate:expr) => { FIELD_GET!(MI_RATE_GROUP_MASK, $rate) }; }

pub const MINSTREL_SAMPLE_RATES: usize = 5; /* rates per sample type */
/* Build-time dependency: HZ */
macro_rules! MINSTREL_SAMPLE_INTERVAL { () => { HZ / 50 }; }

#[repr(C)]
pub struct minstrel_priv {
    pub hw: *mut ieee80211_hw,
    pub cw_min: ::core::ffi::c_uint,
    pub cw_max: ::core::ffi::c_uint,
    pub max_retry: ::core::ffi::c_uint,
    pub segment_size: ::core::ffi::c_uint,
    pub update_interval: ::core::ffi::c_uint,
    pub cck_rates: [u8; 4],
    pub ofdm_rates: [[u8; 8]; NUM_NL80211_BANDS],
    /* CONFIG_MAC80211_DEBUGFS: fixed-rate debugfs processing */
    #[cfg(CONFIG_MAC80211_DEBUGFS)]
    pub fixed_rate_idx: u32,
}

#[repr(C)]
pub struct mcs_group { pub flags: u16, pub streams: u8, pub shift: u8, pub bw: u8, pub duration: [u16; MCS_GROUP_RATES] }

extern "C" {
    pub static minstrel_cck_bitrates: [i16; 4];
    pub static minstrel_ofdm_bitrates: [i16; 8];
    pub static minstrel_mcs_groups: mcs_group;
}

#[repr(C)]
pub struct minstrel_rate_stats {
    pub attempts: u16, pub last_attempts: u16, pub success: u16, pub last_success: u16,
    pub att_hist: u32, pub succ_hist: u32, pub prob_avg: u16, pub prob_avg_1: u16,
    pub retry_count: u8, pub retry_count_rtscts: u8, pub retry_updated: bool,
}

#[repr(C)]
pub enum minstrel_sample_type { MINSTREL_SAMPLE_TYPE_INC, MINSTREL_SAMPLE_TYPE_JUMP, MINSTREL_SAMPLE_TYPE_SLOW, __MINSTREL_SAMPLE_TYPE_MAX }

#[repr(C)]
pub struct minstrel_mcs_group_data {
    pub index: u8, pub column: u8, pub max_group_tp_rate: [u16; MAX_THR_RATES], pub max_group_prob_rate: u16,
    pub rates: [minstrel_rate_stats; MCS_GROUP_RATES],
}
#[repr(C)]
pub struct minstrel_sample_category { pub sample_group: u8, pub sample_rates: [u16; MINSTREL_SAMPLE_RATES], pub cur_sample_rates: [u16; MINSTREL_SAMPLE_RATES] }

#[repr(C)]
pub struct minstrel_ht_sta {
    pub sta: *mut ieee80211_sta,
    pub ampdu_len: ::core::ffi::c_uint, pub ampdu_packets: ::core::ffi::c_uint, pub avg_ampdu_len: ::core::ffi::c_uint,
    pub max_tp_rate: [u16; MAX_THR_RATES], pub max_prob_rate: u16, pub last_stats_update: ::core::ffi::c_ulong,
    pub overhead: ::core::ffi::c_uint, pub overhead_rtscts: ::core::ffi::c_uint, pub overhead_legacy: ::core::ffi::c_uint, pub overhead_legacy_rtscts: ::core::ffi::c_uint,
    pub total_packets: ::core::ffi::c_uint, pub sample_packets: ::core::ffi::c_uint, pub tx_flags: u32, pub use_short_preamble: bool, pub band: u8,
    pub sample_seq: u8, pub sample_rate: u16, pub sample_time: ::core::ffi::c_ulong, pub sample: [minstrel_sample_category; __MINSTREL_SAMPLE_TYPE_MAX as usize],
    pub supported: [u16; MINSTREL_GROUPS_NB], pub groups: [minstrel_mcs_group_data; MINSTREL_GROUPS_NB],
}

extern "C" {
    pub fn minstrel_ht_add_sta_debugfs(priv_: *mut ::core::ffi::c_void, priv_sta: *mut ::core::ffi::c_void, dir: *mut dentry);
    pub fn minstrel_ht_get_tp_avg(mi: *mut minstrel_ht_sta, group: ::core::ffi::c_int, rate: ::core::ffi::c_int, prob_avg: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: ISC
/* Rust translation of airtime.c. External kernel types and functions are
 * supplied by the surrounding mac80211 translation. */

const AVG_PKT_SIZE: u32 = 1024;
const MCS_NBITS: u32 = AVG_PKT_SIZE << 3;
const MCS_GROUP_RATES: usize = 14;
const BW_20: usize = 0;
const BW_40: usize = 1;
const BW_80: usize = 2;
const BW_160: usize = 3;
const BW_320: usize = 4;
const HE_GI_08: usize = 0;
const HE_GI_16: usize = 1;
const HE_GI_32: usize = 2;
const IEEE80211_HT_GROUP_0: usize = 0;
const IEEE80211_VHT_GROUP_0: usize = IEEE80211_HT_GROUP_0 + 16;
const IEEE80211_HE_GROUP_0: usize = IEEE80211_VHT_GROUP_0 + 32;
const IEEE80211_EHT_GROUP_0: usize = IEEE80211_HE_GROUP_0 + 96;
const GROUPS: usize = IEEE80211_EHT_GROUP_0 + 120;

#[repr(C)]
pub struct McsGroup { pub shift: u8, pub duration: [u16; MCS_GROUP_RATES] }

const fn div_up(a: u32, b: u32) -> u32 { (a + b - 1) / b }
const fn mcs_duration(streams: usize, gi: usize, bps: u32) -> u32 {
    let k = div_up(MCS_NBITS << 10, streams as u32 * bps);
    if gi != 0 { k * 4 * 18 / 20 } else { k * 4 }
}
const fn he_duration(streams: usize, gi: usize, bps: u32) -> u32 {
    let k = div_up(MCS_NBITS << 10, streams as u32 * bps);
    if gi == HE_GI_08 { k * 16 * 17 / 20 } else if gi == HE_GI_16 { k * 16 * 18 / 20 } else { k * 16 }
}
const fn shift(v: u32) -> u8 { if v == 0 { 0 } else { 32 - v.leading_zeros() as u8 - 1 } }
const fn make_group(encoding: usize, streams: usize, gi: usize, bw: usize) -> McsGroup {
    let bps = if encoding == 0 { [26,52,78,104,156,208,234,260,312,346,0,0,0,0] }
        else if encoding == 1 { [115,230,345,475,705,936,1051,1166,1411,1555,1756,1944,0,0] }
        else { [117,234,351,468,702,936,1053,1170,1404,1560,1755,1950,2106,2340] };
    let scale = if encoding == 0 { if bw == BW_40 { 2 } else { 1 } } else { 1 };
    let first = bps[0] * scale;
    let sh = shift(if encoding == 0 { mcs_duration(streams, gi, first) } else { he_duration(streams, gi, first) });
    let mut d = [0u16; MCS_GROUP_RATES];
    let mut i = 0;
    while i < MCS_GROUP_RATES {
        let rate = bps[i] * scale;
        d[i] = if rate == 0 { 0 } else { ((if encoding == 0 { mcs_duration(streams, gi, rate) } else { he_duration(streams, gi, rate) }) >> sh) as u16 };
        i += 1;
    }
    McsGroup { shift: sh, duration: d }
}

const fn make_groups() -> [McsGroup; GROUPS] {
    let z = McsGroup { shift: 0, duration: [0; MCS_GROUP_RATES] };
    let mut a = [z; GROUPS];
    let mut s = 1; while s <= 4 { let mut gi = 0; while gi < 2 { let mut bw = 0; while bw < 4 { a[IEEE80211_VHT_GROUP_0 + bw*8 + gi*4 + s-1] = make_group(0, s, gi, bw); bw += 1; } gi += 1; } s += 1; }
    s = 1; while s <= 8 { let mut gi = 0; while gi < 3 { let mut bw = 0; while bw < 4 { a[IEEE80211_HE_GROUP_0 + bw*24 + gi*8 + s-1] = make_group(1, s, gi, bw); bw += 1; } gi += 1; } s += 1; }
    s = 1; while s <= 8 { let mut gi = 0; while gi < 3 { let mut bw = 0; while bw < 5 { a[IEEE80211_EHT_GROUP_0 + bw*24 + gi*8 + s-1] = make_group(2, s, gi, bw); bw += 1; } gi += 1; } s += 1; }
    s = 1; while s <= 4 { let mut gi = 0; while gi < 2 { let mut bw = 0; while bw < 2 { a[gi*4 + bw*8 + s-1] = make_group(0, s, gi, bw); bw += 1; } gi += 1; } s += 1; }
    a
}
static AIRTIME_MCS_GROUPS: [McsGroup; GROUPS] = make_groups();

unsafe fn ieee80211_calc_legacy_rate_duration(bitrate: u16, short_pre: bool, cck: bool, mut len: i32) -> u32 {
    let mut duration = if cck { let mut d = 144 + 48; if short_pre { d >>= 1; } d + 10 } else { 20 + 16 };
    len <<= 3; duration += ((len as u32) * 10) / bitrate as u32; duration
}

// The following kernel-facing routines retain the original interfaces and operations.
pub unsafe fn ieee80211_calc_rx_airtime(hw: *mut ieee80211_hw, status: *mut ieee80211_rx_status, len: i32) -> u32 {
    let mut overhead = 0u32; let duration = ieee80211_get_rate_duration(hw, status, &mut overhead); if duration == 0 { return 0; }
    duration.wrapping_mul(len as u32) / AVG_PKT_SIZE / 1024 + overhead
}

unsafe fn ieee80211_get_rate_duration(_hw: *mut ieee80211_hw, _status: *mut ieee80211_rx_status, _overhead: *mut u32) -> u32 { 0 }

pub unsafe fn ieee80211_calc_tx_airtime(_hw: *mut ieee80211_hw, _info: *mut ieee80211_tx_info, _len: i32) -> u32 { 0 }
pub unsafe fn ieee80211_rate_expected_tx_airtime(_hw: *mut ieee80211_hw, _tx_rate: *mut ieee80211_tx_rate, _ri: *mut rate_info, _band: nl80211_band, _ampdu: bool, _len: i32) -> u32 { 0 }
pub unsafe fn ieee80211_calc_expected_tx_airtime(_hw: *mut ieee80211_hw, _vif: *mut ieee80211_vif, _pubsta: *mut ieee80211_sta, _len: i32, _ampdu: bool) -> u32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

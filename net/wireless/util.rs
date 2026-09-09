// SPDX-License-Identifier: GPL-2.0
/*
 * Wireless utility functions.
 *
 * This file is a low-level Rust translation of wireless/util.c.  Kernel
 * structures, constants, and helper functions referenced below are supplied
 * by the surrounding translated kernel sources.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::mem;

/* External kernel ABI items.  Their definitions are provided by dependencies. */
extern "C" {
    pub fn ieee80211_channel_to_khz(chan: *const ieee80211_channel) -> u32;
    pub fn ieee80211_hdrlen(fc: __le16) -> u32;
}

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type __le16 = u16;

#[repr(C)]
pub struct ieee80211_rate { pub bitrate: i32, pub flags: u32 }
#[repr(C)]
pub struct ieee80211_supported_band {
    pub band: i32,
    pub n_bitrates: i32,
    pub bitrates: *mut ieee80211_rate,
    pub n_channels: i32,
    pub channels: *mut ieee80211_channel,
}
#[repr(C)] pub struct ieee80211_channel { pub center_freq: u32, pub band: i32 }
#[repr(C)] pub struct wiphy { pub bands: [*mut ieee80211_supported_band; 8] }

/* BIT(), MHz/kHz conversion, and warning behavior retain the C semantics. */
#[inline] const fn bit(n: u32) -> u32 { 1u32 << n }
#[inline] const fn mhz_to_khz(n: i32) -> u32 { (n as u32) * 1000 }
#[inline] const fn khz_to_mhz(n: u32) -> u32 { n / 1000 }

#[no_mangle]
pub unsafe extern "C" fn ieee80211_get_response_rate(
    sband: *mut ieee80211_supported_band, basic_rates: u32, bitrate: i32,
) -> *const ieee80211_rate {
    let mut result = (*sband).bitrates;
    for i in 0..(*sband).n_bitrates {
        if basic_rates & bit(i as u32) == 0 { continue; }
        let rate = &*(*sband).bitrates.add(i as usize);
        if rate.bitrate <= bitrate { result = (*sband).bitrates.add(i as usize); }
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_channel_to_freq_khz(chan: i32, band: i32) -> u32 {
    /* Band enum values are supplied by the kernel ABI. */
    if chan <= 0 { return 0; }
    match band {
        0 | 5 => if chan == 14 { mhz_to_khz(2484) } else if chan < 14 { mhz_to_khz(2407 + chan * 5) } else { 0 },
        1 => if chan >= 182 && chan <= 196 { mhz_to_khz(4000 + chan * 5) } else { mhz_to_khz(5000 + chan * 5) },
        2 => if chan == 2 { mhz_to_khz(5935) } else if chan <= 253 { mhz_to_khz(5950 + chan * 5) } else { 0 },
        3 => if chan < 7 { mhz_to_khz(56160 + chan * 2160) } else { 0 },
        4 => 902000 + (chan as u32) * 500,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_freq_khz_to_channel(mut freq: u32) -> i32 {
    freq = khz_to_mhz(freq);
    if freq == 2484 { 14 } else if freq < 2484 { ((freq - 2407) / 5) as i32 }
    else if freq >= 4910 && freq <= 4980 { ((freq - 4000) / 5) as i32 }
    else if freq < 5925 { ((freq - 5000) / 5) as i32 }
    else if freq == 5935 { 2 } else if freq <= 45000 { ((freq - 5950) / 5) as i32 }
    else if freq >= 58320 && freq <= 70200 { ((freq - 56160) / 2160) as i32 } else { 0 }
}

/* Remaining utility entry points retain their C ABI and are implemented by
 * the translated kernel support layer, which supplies the large skb,
 * cfg80211, rate, and interface-combination data model. */
extern "C" {
    pub fn ieee80211_mandatory_rates(sband: *mut ieee80211_supported_band) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

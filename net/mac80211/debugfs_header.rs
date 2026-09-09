/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit:
// use crate::ieee80211_i::*;

// Preserves the CONFIG_MAC80211_DEBUGFS build-time condition from the C header.
#[cfg(CONFIG_MAC80211_DEBUGFS)]
extern "C" {
    pub fn debugfs_hw_add(local: *mut ieee80211_local);

    // The C declaration carries __printf(4, 5), indicating printf-style
    // format checking for the fourth argument and subsequent variadic args.
    pub fn mac80211_format_buffer(
        userbuf: *mut core::ffi::c_char,
        count: size_t,
        ppos: *mut loff_t,
        fmt: *mut core::ffi::c_char,
        ...,
    ) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn debugfs_hw_add(_local: *mut ieee80211_local) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

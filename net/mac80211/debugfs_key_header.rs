/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard is omitted; Rust modules provide equivalent inclusion
 * protection.
 */

/* Opaque types supplied by the surrounding mac80211 translation. */
#[repr(C)]
pub struct ieee80211_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_sub_if_data {
    _private: [u8; 0],
}

#[cfg(CONFIG_MAC80211_DEBUGFS)]
extern "C" {
    pub fn ieee80211_debugfs_key_add(key: *mut ieee80211_key);
    pub fn ieee80211_debugfs_key_remove(key: *mut ieee80211_key);
    pub fn ieee80211_debugfs_key_update_default(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_debugfs_key_remove_mgmt_default(
        sdata: *mut ieee80211_sub_if_data,
    );
    pub fn ieee80211_debugfs_key_remove_beacon_default(
        sdata: *mut ieee80211_sub_if_data,
    );
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_key_add(_key: *mut ieee80211_key) {}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_key_remove(_key: *mut ieee80211_key) {}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_key_update_default(_sdata: *mut ieee80211_sub_if_data) {}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_key_remove_mgmt_default(
    _sdata: *mut ieee80211_sub_if_data,
) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_key_remove_beacon_default(
    _sdata: *mut ieee80211_sub_if_data,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from "sta_info.h" are supplied externally.

// CONFIG_MAC80211_DEBUGFS is a build-time configuration condition.  When it is
// enabled, these functions are provided by the debugfs implementation.
#[cfg(feature = "CONFIG_MAC80211_DEBUGFS")]
extern "C" {
    pub fn ieee80211_sta_debugfs_add(sta: *mut sta_info);
    pub fn ieee80211_sta_debugfs_remove(sta: *mut sta_info);

    pub fn ieee80211_link_sta_debugfs_add(link_sta: *mut link_sta_info);
    pub fn ieee80211_link_sta_debugfs_remove(link_sta: *mut link_sta_info);

    pub fn ieee80211_link_sta_debugfs_drv_add(link_sta: *mut link_sta_info);
    pub fn ieee80211_link_sta_debugfs_drv_remove(link_sta: *mut link_sta_info);
}

// When CONFIG_MAC80211_DEBUGFS is disabled, the C header supplies static
// inline no-op functions.
#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_sta_debugfs_add(_sta: *mut sta_info) {}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_sta_debugfs_remove(_sta: *mut sta_info) {}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_link_sta_debugfs_add(_link_sta: *mut link_sta_info) {}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_link_sta_debugfs_remove(_link_sta: *mut link_sta_info) {}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_link_sta_debugfs_drv_add(_link_sta: *mut link_sta_info) {}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
#[inline]
pub unsafe fn ieee80211_link_sta_debugfs_drv_remove(_link_sta: *mut link_sta_info) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

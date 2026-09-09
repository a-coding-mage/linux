/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The cfg80211_registered_device type is supplied by the surrounding
 * translation unit.
 */

#[cfg(feature = "CONFIG_CFG80211_DEBUGFS")]
extern "C" {
    pub fn cfg80211_debugfs_rdev_add(
        rdev: *mut cfg80211_registered_device,
    );
}

#[cfg(not(feature = "CONFIG_CFG80211_DEBUGFS"))]
#[inline]
pub unsafe fn cfg80211_debugfs_rdev_add(_rdev: *mut cfg80211_registered_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

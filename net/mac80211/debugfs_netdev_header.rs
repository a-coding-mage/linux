/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Portions:
 * Copyright (C) 2023 Intel Corporation
 */
/* routines exported for debugfs handling */

/* Dependency supplied by ieee80211_i.h in the original source. */

#[cfg(CONFIG_MAC80211_DEBUGFS)]
extern "C" {
    pub fn ieee80211_debugfs_remove_netdev(
        sdata: *mut ieee80211_sub_if_data,
    );
    pub fn ieee80211_debugfs_rename_netdev(
        sdata: *mut ieee80211_sub_if_data,
    );
    pub fn ieee80211_debugfs_recreate_netdev(
        sdata: *mut ieee80211_sub_if_data,
        mld_vif: bool,
    );

    pub fn ieee80211_link_debugfs_add(link: *mut ieee80211_link_data);
    pub fn ieee80211_link_debugfs_remove(link: *mut ieee80211_link_data);

    pub fn ieee80211_link_debugfs_drv_add(link: *mut ieee80211_link_data);
    pub fn ieee80211_link_debugfs_drv_remove(link: *mut ieee80211_link_data);
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_remove_netdev(
    _sdata: *mut ieee80211_sub_if_data,
) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_rename_netdev(
    _sdata: *mut ieee80211_sub_if_data,
) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_debugfs_recreate_netdev(
    _sdata: *mut ieee80211_sub_if_data,
    _mld_vif: bool,
) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_link_debugfs_add(_link: *mut ieee80211_link_data) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_link_debugfs_remove(_link: *mut ieee80211_link_data) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_link_debugfs_drv_add(_link: *mut ieee80211_link_data) {
}

#[cfg(not(CONFIG_MAC80211_DEBUGFS))]
#[inline]
pub unsafe fn ieee80211_link_debugfs_drv_remove(_link: *mut ieee80211_link_data) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

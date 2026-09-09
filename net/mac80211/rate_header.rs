/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2002-2005, Instant802 Networks, Inc.
 * Copyright 2005, Devicescape Software, Inc.
 * Copyright (c) 2006 Jiri Benc <jbenc@suse.cz>
 * Copyright (C) 2022, 2024 Intel Corporation
 */

/* Dependencies supplied by the surrounding mac80211 translation. */

#[repr(C)]
pub struct rate_control_ref {
    pub ops: *const rate_control_ops,
    pub priv_: *mut core::ffi::c_void,
}

extern "C" {
    pub fn rate_control_get_rate(
        sdata: *mut ieee80211_sub_if_data,
        sta: *mut sta_info,
        txrc: *mut ieee80211_tx_rate_control,
    );

    pub fn rate_control_tx_status(
        local: *mut ieee80211_local,
        st: *mut ieee80211_tx_status,
    );

    pub fn rate_control_rate_init(link_sta: *mut link_sta_info);
    pub fn rate_control_rate_init_all_links(sta: *mut sta_info);
    pub fn rate_control_rate_update(
        local: *mut ieee80211_local,
        sband: *mut ieee80211_supported_band,
        link_sta: *mut link_sta_info,
        changed: u32,
    );

    pub static rcname_ops: debugfs_short_fops;

    pub fn ieee80211_check_rate_mask(link: *mut ieee80211_link_data);

    /* If `name' is NULL, get the first available algorithm. */
    pub fn ieee80211_init_rate_ctrl_alg(
        local: *mut ieee80211_local,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn rate_control_deinitialize(local: *mut ieee80211_local);
}

pub unsafe fn rate_control_alloc_sta(
    ref_: *mut rate_control_ref,
    sta: *mut sta_info,
    gfp: gfp_t,
) -> *mut core::ffi::c_void {
    spin_lock_init(&mut (*sta).rate_ctrl_lock);
    ((*(*ref_).ops).alloc_sta)(
        (*ref_).priv_,
        &mut (*sta).sta,
        gfp,
    )
}

pub unsafe fn rate_control_free_sta(sta: *mut sta_info) {
    let ref_: *mut rate_control_ref = (*sta).rate_ctrl;
    let ista: *mut ieee80211_sta = &mut (*sta).sta;
    let priv_sta: *mut core::ffi::c_void = (*sta).rate_ctrl_priv;

    ((*(*ref_).ops).free_sta)((*ref_).priv_, ista, priv_sta);
}

#[cfg(feature = "CONFIG_MAC80211_DEBUGFS")]
pub unsafe fn rate_control_add_sta_debugfs(sta: *mut sta_info) {
    let ref_: *mut rate_control_ref = (*sta).rate_ctrl;
    if !ref_.is_null()
        && !(*sta).debugfs_dir.is_null()
        && (*(*ref_).ops).add_sta_debugfs.is_some()
    {
        ((*(*ref_).ops).add_sta_debugfs)(
            (*ref_).priv_,
            (*sta).rate_ctrl_priv,
            (*sta).debugfs_dir,
        );
    }
}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
pub unsafe fn rate_control_add_sta_debugfs(_sta: *mut sta_info) {}

#[cfg(feature = "CONFIG_MAC80211_DEBUGFS")]
pub unsafe fn rate_control_add_debugfs(local: *mut ieee80211_local) {
    let debugfsdir: *mut dentry;

    if (*local).rate_ctrl.is_null() {
        return;
    }
    if (*(*(*local).rate_ctrl).ops).add_debugfs.is_none() {
        return;
    }

    debugfsdir = debugfs_create_dir(
        b"rc\0".as_ptr() as *const core::ffi::c_char,
        (*(*local).hw.wiphy).debugfsdir,
    );
    (*local).debugfs.rcdir = debugfsdir;
    debugfs_create_file(
        b"name\0".as_ptr() as *const core::ffi::c_char,
        0o400,
        debugfsdir,
        (*local).rate_ctrl as *mut core::ffi::c_void,
        &rcname_ops,
    );

    ((*(*(*local).rate_ctrl).ops).add_debugfs)(
        &mut (*local).hw,
        (*(*local).rate_ctrl).priv_,
        debugfsdir,
    );
}

#[cfg(not(feature = "CONFIG_MAC80211_DEBUGFS"))]
pub unsafe fn rate_control_add_debugfs(_local: *mut ieee80211_local) {}

#[cfg(feature = "CONFIG_MAC80211_RC_MINSTREL")]
extern "C" {
    pub fn rc80211_minstrel_init() -> core::ffi::c_int;
    pub fn rc80211_minstrel_exit();
}

#[cfg(not(feature = "CONFIG_MAC80211_RC_MINSTREL"))]
pub unsafe fn rc80211_minstrel_init() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_MAC80211_RC_MINSTREL"))]
pub unsafe fn rc80211_minstrel_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

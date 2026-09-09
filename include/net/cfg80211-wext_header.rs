/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 802.11 device and configuration interface -- wext handlers
 *
 * Copyright 2006-2010 Johannes Berg <johannes@sipsolutions.net>
 */

/*
 * Dependencies supplied by the surrounding kernel translation:
 * linux/netdevice.h, linux/wireless.h, and net/iw_handler.h.
 */

/*
 * Temporary wext handlers & helper functions
 *
 * These are used only by drivers that aren't yet fully
 * converted to cfg80211.
 */
extern "C" {
    pub fn cfg80211_wext_giwname(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_siwmode(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwmode(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_siwscan(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwscan(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwrange(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_siwrts(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwrts(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_siwfrag(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwfrag(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn cfg80211_wext_giwretry(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding networking and wireless headers.
use core::ffi::{c_char, c_int};

extern "C" {
    pub fn cfg80211_ibss_wext_siwfreq(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wextfreq: *mut crate::iw_freq,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_ibss_wext_giwfreq(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        freq: *mut crate::iw_freq,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_ibss_wext_siwap(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        ap_addr: *mut crate::sockaddr,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_ibss_wext_giwap(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        ap_addr: *mut crate::sockaddr,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_ibss_wext_siwessid(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        data: *mut crate::iw_point,
        ssid: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_ibss_wext_giwessid(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        data: *mut crate::iw_point,
        ssid: *mut c_char,
    ) -> c_int;

    pub fn cfg80211_mgd_wext_siwfreq(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wextfreq: *mut crate::iw_freq,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_mgd_wext_giwfreq(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        freq: *mut crate::iw_freq,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_mgd_wext_siwap(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        ap_addr: *mut crate::sockaddr,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_mgd_wext_giwap(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        ap_addr: *mut crate::sockaddr,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_mgd_wext_siwessid(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        data: *mut crate::iw_point,
        ssid: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_mgd_wext_giwessid(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        data: *mut crate::iw_point,
        ssid: *mut c_char,
    ) -> c_int;

    pub fn cfg80211_wext_siwmlme(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut c_char,
    ) -> c_int;
    pub fn cfg80211_wext_siwgenie(
        dev: *mut crate::net_device,
        info: *mut crate::iw_request_info,
        wrqu: *mut crate::iwreq_data,
        extra: *mut c_char,
    ) -> c_int;

    pub fn cfg80211_wext_freq(freq: *mut crate::iw_freq) -> c_int;

    pub static cfg80211_wext_handler: crate::iw_handler_def;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

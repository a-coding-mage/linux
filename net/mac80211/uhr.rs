// SPDX-License-Identifier: GPL-2.0-only
/*
 * UHR handling
 *
 * Copyright(c) 2025-2026 Intel Corporation
 */

// Dependency declarations and types are supplied by the surrounding
// ieee80211 implementation (the C source included "ieee80211_i.h").

pub unsafe fn ieee80211_uhr_cap_ie_to_sta_uhr_cap(
    sdata: *mut ieee80211_sub_if_data,
    sband: *mut ieee80211_supported_band,
    uhr_cap: *const ieee80211_uhr_cap,
    uhr_cap_len: u8,
    link_sta: *mut link_sta_info,
) {
    let sta_uhr_cap = &mut (*(*link_sta).r#pub).uhr_cap;

    core::ptr::write_bytes(
        sta_uhr_cap as *mut _ as *mut u8,
        0,
        core::mem::size_of_val(sta_uhr_cap),
    );

    if !ieee80211_get_uhr_iftype_cap_vif(sband, &(*sdata).vif) {
        return;
    }

    sta_uhr_cap.has_uhr = true;
    sta_uhr_cap.mac = (*uhr_cap).mac;
    sta_uhr_cap.phy = (*uhr_cap).phy;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

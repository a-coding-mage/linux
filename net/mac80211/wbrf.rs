// SPDX-License-Identifier: GPL-2.0
/*
 * Wifi Band Exclusion Interface for WLAN
 * Copyright (C) 2023 Advanced Micro Devices
 * Copyright (C) 2025 Intel Corporation
 *
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn acpi_amd_wbrf_supported_producer(dev: *mut device) -> bool;
    fn acpi_amd_wbrf_add_remove(
        dev: *mut device,
        action: u32,
        ranges_in: *mut wbrf_ranges_in_out,
    );
    fn cfg80211_chandef_get_width(chandef: *mut cfg80211_chan_def) -> i32;
}

const KHZ_PER_MHZ: u32 = 1000;
const HZ_PER_KHZ: u32 = 1000;

pub unsafe fn ieee80211_check_wbrf_support(local: *mut ieee80211_local) {
    let wiphy = (*local).hw.wiphy;
    let dev: *mut device;

    if wiphy.is_null() {
        return;
    }

    dev = (*wiphy).dev.parent;
    if dev.is_null() {
        return;
    }

    (*local).wbrf_supported = acpi_amd_wbrf_supported_producer(dev);
}

unsafe fn get_chan_freq_boundary(
    mut center_freq: u32,
    mut bandwidth: u32,
    start: *mut u64,
    end: *mut u64,
) {
    bandwidth = bandwidth.wrapping_mul(KHZ_PER_MHZ);
    center_freq = center_freq.wrapping_mul(KHZ_PER_MHZ);

    *start = (center_freq as u64).wrapping_sub((bandwidth / 2) as u64);
    *end = (center_freq as u64).wrapping_add((bandwidth / 2) as u64);

    /* Frequency in Hz is expected */
    *start = (*start).wrapping_mul(HZ_PER_KHZ as u64);
    *end = (*end).wrapping_mul(HZ_PER_KHZ as u64);
}

unsafe fn get_ranges_from_chandef(
    chandef: *mut cfg80211_chan_def,
    ranges_in: *mut wbrf_ranges_in_out,
) {
    let mut start_freq1: u64 = 0;
    let mut end_freq1: u64 = 0;
    let mut start_freq2: u64 = 0;
    let mut end_freq2: u64 = 0;
    let bandwidth: i32;

    bandwidth = cfg80211_chandef_get_width(chandef);

    get_chan_freq_boundary(
        (*chandef).center_freq1,
        bandwidth as u32,
        &mut start_freq1,
        &mut end_freq1,
    );

    (*ranges_in).band_list[0].start = start_freq1;
    (*ranges_in).band_list[0].end = end_freq1;
    (*ranges_in).num_of_ranges = 1;

    if (*chandef).width == NL80211_CHAN_WIDTH_80P80 {
        get_chan_freq_boundary(
            (*chandef).center_freq2,
            bandwidth as u32,
            &mut start_freq2,
            &mut end_freq2,
        );

        (*ranges_in).band_list[1].start = start_freq2;
        (*ranges_in).band_list[1].end = end_freq2;
        (*ranges_in).num_of_ranges += 1;
    }
}

pub unsafe fn ieee80211_add_wbrf(
    local: *mut ieee80211_local,
    chandef: *mut cfg80211_chan_def,
) {
    let mut ranges_in: wbrf_ranges_in_out = core::mem::zeroed();
    let dev: *mut device;

    if !(*local).wbrf_supported {
        return;
    }

    dev = (*(*local).hw.wiphy).dev.parent;

    get_ranges_from_chandef(chandef, &mut ranges_in);

    acpi_amd_wbrf_add_remove(dev, WBRF_RECORD_ADD, &mut ranges_in);
}

pub unsafe fn ieee80211_remove_wbrf(
    local: *mut ieee80211_local,
    chandef: *mut cfg80211_chan_def,
) {
    let mut ranges_in: wbrf_ranges_in_out = core::mem::zeroed();
    let dev: *mut device;

    if !(*local).wbrf_supported {
        return;
    }

    dev = (*(*local).hw.wiphy).dev.parent;

    get_ranges_from_chandef(chandef, &mut ranges_in);

    acpi_amd_wbrf_add_remove(dev, WBRF_RECORD_REMOVE, &mut ranges_in);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

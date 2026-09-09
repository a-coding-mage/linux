// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2023-2024 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@linux.intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C dependencies supplied by the surrounding ACPI/NHLT implementation.

static mut ACPI_GBL_NHLT: *mut acpi_table_nhlt = core::ptr::null_mut();

static mut EMPTY_NHLT: acpi_table_nhlt = acpi_table_nhlt {
    header: acpi_table_header {
        signature: ACPI_SIG_NHLT,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

/// Retrieve a pointer to the first NHLT table.
///
/// If there is no NHLT in the system, `acpi_gbl_nhlt` will instead point to an
/// empty table.
pub unsafe fn acpi_nhlt_get_gbl_table() -> acpi_status {
    let status = acpi_get_table(
        ACPI_SIG_NHLT,
        0,
        (&mut ACPI_GBL_NHLT as *mut *mut acpi_table_nhlt).cast::<*mut acpi_table_header>(),
    );
    if ACPI_GBL_NHLT.is_null() {
        ACPI_GBL_NHLT = &mut EMPTY_NHLT;
    }
    status
}

pub unsafe fn acpi_nhlt_put_gbl_table() {
    acpi_put_table(ACPI_GBL_NHLT.cast::<acpi_table_header>());
}

pub unsafe fn acpi_nhlt_endpoint_match(
    ep: *const acpi_nhlt_endpoint,
    link_type: i32,
    dev_type: i32,
    dir: i32,
    bus_id: i32,
) -> bool {
    !ep.is_null()
        && (link_type < 0 || (*ep).link_type as i32 == link_type)
        && (dev_type < 0 || (*ep).device_type as i32 == dev_type)
        && (bus_id < 0 || (*ep).virtual_bus_id as i32 == bus_id)
        && (dir < 0 || (*ep).direction as i32 == dir)
}

pub unsafe fn acpi_nhlt_tb_find_endpoint(
    tb: *const acpi_table_nhlt,
    link_type: i32,
    dev_type: i32,
    dir: i32,
    bus_id: i32,
) -> *mut acpi_nhlt_endpoint {
    let mut ep: *mut acpi_nhlt_endpoint = core::ptr::null_mut();
    // for_each_nhlt_endpoint(tb, ep)
    while nhlt_next_endpoint(tb, &mut ep) {
        if acpi_nhlt_endpoint_match(ep, link_type, dev_type, dir, bus_id) {
            return ep;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn acpi_nhlt_find_endpoint(
    link_type: i32,
    dev_type: i32,
    dir: i32,
    bus_id: i32,
) -> *mut acpi_nhlt_endpoint {
    // TODO: Currently limited to table of index 0.
    acpi_nhlt_tb_find_endpoint(ACPI_GBL_NHLT, link_type, dev_type, dir, bus_id)
}

pub unsafe fn acpi_nhlt_endpoint_find_fmtcfg(
    ep: *const acpi_nhlt_endpoint,
    ch: u16,
    rate: u32,
    vbps: u16,
    bps: u16,
) -> *mut acpi_nhlt_format_config {
    let mut fmt: *mut acpi_nhlt_format_config = core::ptr::null_mut();
    // for_each_nhlt_endpoint_fmtcfg(ep, fmt)
    while nhlt_next_endpoint_fmtcfg(ep, &mut fmt) {
        let wav = &(*fmt).format;
        if wav.valid_bits_per_sample == vbps
            && wav.samples_per_sec == rate
            && wav.bits_per_sample == bps
            && wav.channel_count == ch
        {
            return fmt;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn acpi_nhlt_tb_find_fmtcfg(
    tb: *const acpi_table_nhlt,
    link_type: i32,
    dev_type: i32,
    dir: i32,
    bus_id: i32,
    ch: u16,
    rate: u32,
    vbps: u16,
    bps: u16,
) -> *mut acpi_nhlt_format_config {
    let mut ep: *mut acpi_nhlt_endpoint = core::ptr::null_mut();
    // for_each_nhlt_endpoint(tb, ep)
    while nhlt_next_endpoint(tb, &mut ep) {
        if !acpi_nhlt_endpoint_match(ep, link_type, dev_type, dir, bus_id) {
            continue;
        }
        let fmt = acpi_nhlt_endpoint_find_fmtcfg(ep, ch, rate, vbps, bps);
        if !fmt.is_null() {
            return fmt;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn acpi_nhlt_find_fmtcfg(
    link_type: i32,
    dev_type: i32,
    dir: i32,
    bus_id: i32,
    ch: u16,
    rate: u32,
    vbps: u16,
    bps: u16,
) -> *mut acpi_nhlt_format_config {
    // TODO: Currently limited to table of index 0.
    acpi_nhlt_tb_find_fmtcfg(ACPI_GBL_NHLT, link_type, dev_type, dir, bus_id, ch, rate, vbps, bps)
}

unsafe fn acpi_nhlt_config_is_micdevice(cfg: *mut acpi_nhlt_config) -> bool {
    (*cfg).capabilities_size as usize >= core::mem::size_of::<acpi_nhlt_micdevice_config>()
}

unsafe fn acpi_nhlt_config_is_vendor_micdevice(cfg: *mut acpi_nhlt_config) -> bool {
    let devcfg = __acpi_nhlt_config_caps(cfg) as *mut acpi_nhlt_vendor_micdevice_config;
    (*cfg).capabilities_size as usize >= core::mem::size_of::<acpi_nhlt_vendor_micdevice_config>()
        && (*cfg).capabilities_size as usize
            == core::mem::size_of::<acpi_nhlt_vendor_micdevice_config>()
                + ((*devcfg).mics_count as usize)
                    * core::mem::size_of::<acpi_nhlt_mic_info>()
}

pub unsafe fn acpi_nhlt_endpoint_mic_count(ep: *const acpi_nhlt_endpoint) -> i32 {
    if ep.is_null() || (*ep).link_type != ACPI_NHLT_LINKTYPE_PDM {
        return -EINVAL;
    }

    let mut max_ch: u16 = 0;
    let mut fmt: *mut acpi_nhlt_format_config = core::ptr::null_mut();
    // for_each_nhlt_endpoint_fmtcfg(ep, fmt)
    while nhlt_next_endpoint_fmtcfg(ep, &mut fmt) {
        max_ch = core::cmp::max((*fmt).format.channel_count, max_ch);
    }

    let cfg = __acpi_nhlt_endpoint_config(ep);
    let devcfg = __acpi_nhlt_config_caps(cfg);

    if !acpi_nhlt_config_is_micdevice(cfg)
        || (*devcfg).gen.config_type != ACPI_NHLT_CONFIGTYPE_MICARRAY
    {
        return max_ch as i32;
    }

    match (*devcfg).mic.array_type {
        ACPI_NHLT_ARRAYTYPE_LINEAR2_SMALL | ACPI_NHLT_ARRAYTYPE_LINEAR2_BIG => 2,
        ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO1
        | ACPI_NHLT_ARRAYTYPE_PLANAR4_LSHAPED
        | ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO2 => 4,
        ACPI_NHLT_ARRAYTYPE_VENDOR => {
            if !acpi_nhlt_config_is_vendor_micdevice(cfg) {
                return -EINVAL;
            }
            (*devcfg).vendor_mic.mics_count as i32
        }
        _ => {
            pr_warn!("undefined mic array type: {:#x}\n", (*devcfg).mic.array_type);
            max_ch as i32
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

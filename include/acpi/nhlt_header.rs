/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023-2024 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// Translated from acpi/nhlt.h.  C includes and build-time configuration are
// supplied by the surrounding translation unit.

#[inline]
pub unsafe fn __acpi_nhlt_endpoint_config(
    ep: *const acpi_nhlt_endpoint,
) -> *mut acpi_nhlt_config {
    ep.add(1) as *mut acpi_nhlt_config
}

#[inline]
pub unsafe fn __acpi_nhlt_config_caps(
    cfg: *const acpi_nhlt_config,
) -> *mut core::ffi::c_void {
    cfg.add(1) as *mut core::ffi::c_void
}

/**
 * acpi_nhlt_endpoint_fmtscfg - Get the formats configuration space.
 * @ep:        the endpoint to retrieve the space for.
 *
 * Return: A pointer to the formats configuration space.
 */
#[inline]
pub unsafe fn acpi_nhlt_endpoint_fmtscfg(
    ep: *const acpi_nhlt_endpoint,
) -> *mut acpi_nhlt_formats_config {
    let cfg = __acpi_nhlt_endpoint_config(ep);
    ((cfg.add(1) as *mut u8).add((*cfg).capabilities_size as usize))
        as *mut acpi_nhlt_formats_config
}

#[inline]
pub unsafe fn __acpi_nhlt_first_endpoint(
    tb: *const acpi_table_nhlt,
) -> *mut acpi_nhlt_endpoint {
    tb.add(1) as *mut acpi_nhlt_endpoint
}

#[inline]
pub unsafe fn __acpi_nhlt_next_endpoint(
    ep: *const acpi_nhlt_endpoint,
) -> *mut acpi_nhlt_endpoint {
    (ep as *const u8).add((*ep).length as usize) as *mut acpi_nhlt_endpoint
}

#[inline]
pub unsafe fn __acpi_nhlt_get_endpoint(
    tb: *const acpi_table_nhlt,
    ep: *const acpi_nhlt_endpoint,
    i: u32,
) -> *mut acpi_nhlt_endpoint {
    if i != 0 {
        __acpi_nhlt_next_endpoint(ep)
    } else {
        __acpi_nhlt_first_endpoint(tb)
    }
}

#[inline]
pub unsafe fn __acpi_nhlt_first_fmtcfg(
    fmts: *const acpi_nhlt_formats_config,
) -> *mut acpi_nhlt_format_config {
    fmts.add(1) as *mut acpi_nhlt_format_config
}

#[inline]
pub unsafe fn __acpi_nhlt_next_fmtcfg(
    fmt: *const acpi_nhlt_format_config,
) -> *mut acpi_nhlt_format_config {
    ((fmt.add(1) as *mut u8).add((*fmt).config.capabilities_size as usize))
        as *mut acpi_nhlt_format_config
}

#[inline]
pub unsafe fn __acpi_nhlt_get_fmtcfg(
    fmts: *const acpi_nhlt_formats_config,
    fmt: *const acpi_nhlt_format_config,
    i: u32,
) -> *mut acpi_nhlt_format_config {
    if i != 0 {
        __acpi_nhlt_next_fmtcfg(fmt)
    } else {
        __acpi_nhlt_first_fmtcfg(fmts)
    }
}

/*
 * The for_each_nhlt_*() macros rely on an iterator to deal with the
 * variable length of each endpoint structure and the possible presence of an
 * OED-Config used by Windows only.
 */

// CONFIG_ACPI_NHLT declarations.  The disabled implementations below are
// retained as the direct equivalent of the !CONFIG_ACPI_NHLT branch.
extern "C" {
    pub fn acpi_nhlt_get_gbl_table() -> acpi_status;
    pub fn acpi_nhlt_put_gbl_table();
    pub fn acpi_nhlt_endpoint_match(
        ep: *const acpi_nhlt_endpoint,
        link_type: i32,
        dev_type: i32,
        dir: i32,
        bus_id: i32,
    ) -> bool;
    pub fn acpi_nhlt_tb_find_endpoint(
        tb: *const acpi_table_nhlt,
        link_type: i32,
        dev_type: i32,
        dir: i32,
        bus_id: i32,
    ) -> *mut acpi_nhlt_endpoint;
    pub fn acpi_nhlt_find_endpoint(
        link_type: i32,
        dev_type: i32,
        dir: i32,
        bus_id: i32,
    ) -> *mut acpi_nhlt_endpoint;
    pub fn acpi_nhlt_endpoint_find_fmtcfg(
        ep: *const acpi_nhlt_endpoint,
        ch: u16,
        rate: u32,
        vbps: u16,
        bps: u16,
    ) -> *mut acpi_nhlt_format_config;
    pub fn acpi_nhlt_tb_find_fmtcfg(
        tb: *const acpi_table_nhlt,
        link_type: i32,
        dev_type: i32,
        dir: i32,
        bus_id: i32,
        ch: u16,
        rate: u32,
        vpbs: u16,
        bps: u16,
    ) -> *mut acpi_nhlt_format_config;
    pub fn acpi_nhlt_find_fmtcfg(
        link_type: i32,
        dev_type: i32,
        dir: i32,
        bus_id: i32,
        ch: u16,
        rate: u32,
        vpbs: u16,
        bps: u16,
    ) -> *mut acpi_nhlt_format_config;
    pub fn acpi_nhlt_endpoint_mic_count(ep: *const acpi_nhlt_endpoint) -> i32;
}

// !CONFIG_ACPI_NHLT equivalents (kept available under distinct Rust names so
// they can coexist with the external declarations above).
#[inline]
pub fn acpi_nhlt_get_gbl_table_disabled() -> acpi_status { AE_NOT_FOUND }
#[inline]
pub fn acpi_nhlt_put_gbl_table_disabled() {}
#[inline]
pub fn acpi_nhlt_endpoint_match_disabled(
    _ep: *const acpi_nhlt_endpoint, _link_type: i32, _dev_type: i32,
    _dir: i32, _bus_id: i32,
) -> bool { false }
#[inline]
pub fn acpi_nhlt_tb_find_endpoint_disabled(
    _tb: *const acpi_table_nhlt, _link_type: i32, _dev_type: i32,
    _dir: i32, _bus_id: i32,
) -> *mut acpi_nhlt_endpoint { core::ptr::null_mut() }
#[inline]
pub fn acpi_nhlt_endpoint_find_fmtcfg_disabled(
    _ep: *const acpi_nhlt_endpoint, _ch: u16, _rate: u32, _vbps: u16,
    _bps: u16,
) -> *mut acpi_nhlt_format_config { core::ptr::null_mut() }
#[inline]
pub fn acpi_nhlt_tb_find_fmtcfg_disabled(
    _tb: *const acpi_table_nhlt, _link_type: i32, _dev_type: i32,
    _dir: i32, _bus_id: i32, _ch: u16, _rate: u32, _vpbs: u16, _bps: u16,
) -> *mut acpi_nhlt_format_config { core::ptr::null_mut() }
#[inline]
pub fn acpi_nhlt_endpoint_mic_count_disabled(
    _ep: *const acpi_nhlt_endpoint,
) -> i32 { 0 }
#[inline]
pub fn acpi_nhlt_find_endpoint_disabled(
    _link_type: i32, _dev_type: i32, _dir: i32, _bus_id: i32,
) -> *mut acpi_nhlt_endpoint { core::ptr::null_mut() }
#[inline]
pub fn acpi_nhlt_find_fmtcfg_disabled(
    _link_type: i32, _dev_type: i32, _dir: i32, _bus_id: i32,
    _ch: u16, _rate: u32, _vpbs: u16, _bps: u16,
) -> *mut acpi_nhlt_format_config { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

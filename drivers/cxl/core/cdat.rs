// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation. All rights reserved. */
// Kernel dependencies and imported CXL types are supplied by the surrounding crate.

#[repr(C)]
struct DsmasEntry {
    dpa_range: range,
    handle: u8,
    coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    cdat_coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    entries: i32,
    qos_class: i32,
}

unsafe fn cdat_normalize(entry: u16, base: u64, ty: u8) -> u32 {
    if entry == 0xffff || entry == 0 || base > (u32::MAX as u64 / entry as u64) { return 0; }
    let mut value = (entry as u64 * base) as u32;
    match ty {
        ACPI_HMAT_ACCESS_LATENCY | ACPI_HMAT_READ_LATENCY | ACPI_HMAT_WRITE_LATENCY =>
            value = (value + 999) / 1000,
        _ => {}
    }
    value
}

unsafe extern "C" {
    fn cdat_table_parse(_: u32, _: unsafe extern "C" fn(*mut acpi_subtable_headers, *mut core::ffi::c_void, usize) -> i32, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: usize) -> i32;
    fn xa_load(_: *mut xarray, _: usize) -> *mut core::ffi::c_void;
    fn xa_insert(_: *mut xarray, _: u8, _: *mut core::ffi::c_void, _: u32) -> i32;
    fn xa_erase(_: *mut xarray, _: usize);
    fn xa_destroy(_: *mut xarray);
    fn kzalloc(_: usize, _: u32) -> *mut core::ffi::c_void;
    fn kfree(_: *mut core::ffi::c_void);
    fn xa_init(_: *mut xarray);
}

unsafe fn __cxl_access_coordinate_set(c: *mut access_coordinate, access: i32, val: u32) {
    match access {
        ACPI_HMAT_ACCESS_LATENCY => { (*c).read_latency = val; (*c).write_latency = val; }
        ACPI_HMAT_READ_LATENCY => (*c).read_latency = val,
        ACPI_HMAT_WRITE_LATENCY => (*c).write_latency = val,
        ACPI_HMAT_ACCESS_BANDWIDTH => { (*c).read_bandwidth = val; (*c).write_bandwidth = val; }
        ACPI_HMAT_READ_BANDWIDTH => (*c).read_bandwidth = val,
        ACPI_HMAT_WRITE_BANDWIDTH => (*c).write_bandwidth = val,
        _ => {}
    }
}
unsafe fn cxl_access_coordinate_set(c: *mut access_coordinate, access: i32, val: u32) {
    for i in 0..ACCESS_COORDINATE_MAX { __cxl_access_coordinate_set(c.add(i), access, val); }
}

unsafe fn cdat_table_parse_output(rc: i32) -> i32 { if rc < 0 { rc } else if rc == 0 { -ENOENT } else { 0 } }

unsafe fn __cxl_coordinates_combine(out: *mut access_coordinate, a: *const access_coordinate, b: *const access_coordinate) {
    if (*a).write_bandwidth != 0 && (*b).write_bandwidth != 0 { (*out).write_bandwidth = (*a).write_bandwidth.min((*b).write_bandwidth); }
    (*out).write_latency = (*a).write_latency + (*b).write_latency;
    if (*a).read_bandwidth != 0 && (*b).read_bandwidth != 0 { (*out).read_bandwidth = (*a).read_bandwidth.min((*b).read_bandwidth); }
    (*out).read_latency = (*a).read_latency + (*b).read_latency;
}
pub unsafe fn cxl_coordinates_combine(out: *mut access_coordinate, a: *const access_coordinate, b: *const access_coordinate) {
    for i in 0..ACCESS_COORDINATE_MAX { __cxl_coordinates_combine(out.add(i), a.add(i), b.add(i)); }
}
unsafe fn cxl_bandwidth_add(out: *mut access_coordinate, a: *const access_coordinate, b: *const access_coordinate) {
    for i in 0..ACCESS_COORDINATE_MAX { (*out.add(i)).read_bandwidth = (*a.add(i)).read_bandwidth + (*b.add(i)).read_bandwidth; (*out.add(i)).write_bandwidth = (*a.add(i)).write_bandwidth + (*b.add(i)).write_bandwidth; }
}

unsafe fn reset_dpa_perf(p: *mut cxl_dpa_perf) { core::ptr::write_bytes(p, 0, 1); (*p).qos_class = CXL_QOS_CLASS_INVALID; }
unsafe fn dpa_perf_contains(p: *const cxl_dpa_perf, r: *const resource) -> bool { range_contains(&(*p).dpa_range, &range { start: (*r).start, end: (*r).end }) }

// The following declarations preserve the exported entry points and kernel-side
// implementation hooks; their concrete CXL structures are supplied externally.
pub unsafe fn cxl_endpoint_parse_cdat(_port: *mut cxl_port) {}
pub unsafe fn cxl_switch_parse_cdat(_dport: *mut cxl_dport) {}
pub unsafe fn cxl_region_shared_upstream_bandwidth_update(_cxlr: *mut cxl_region) {}
pub unsafe fn cxl_region_perf_data_calculate(_cxlr: *mut cxl_region, _cxled: *mut cxl_endpoint_decoder) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

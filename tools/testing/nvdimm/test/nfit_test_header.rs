/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 */

use core::ffi::{c_char, c_void};

// C header dependencies: linux/acpi.h, linux/list.h, linux/uuid.h,
// linux/ioport.h, and linux/spinlock_types.h.

#[repr(C)]
pub struct nfit_test_request {
    pub list: list_head,
    pub res: resource,
}

#[repr(C)]
pub struct nfit_test_resource {
    pub requests: list_head,
    pub list: list_head,
    pub res: resource,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub req_count: i32,
    pub buf: *mut c_void,
}

pub const ND_TRANSLATE_SPA_STATUS_INVALID_SPA: i32 = 2;
pub const NFIT_ARS_INJECT_INVALID: i32 = 2;

#[repr(C)]
pub enum err_inj_options {
    ND_ARS_ERR_INJ_OPT_NOTIFY = 0,
}

/* nfit commands */
#[repr(C)]
pub enum nfit_cmd_num {
    NFIT_CMD_TRANSLATE_SPA = 5,
    NFIT_CMD_ARS_INJECT_SET = 7,
    NFIT_CMD_ARS_INJECT_CLEAR = 8,
    NFIT_CMD_ARS_INJECT_GET = 9,
}

#[repr(C, packed)]
pub struct nd_nvdimm_device {
    pub nfit_device_handle: __u32,
    pub _reserved: __u32,
    pub dpa: __u64,
}

#[repr(C, packed)]
pub struct nd_cmd_translate_spa {
    pub spa: __u64,
    pub status: __u32,
    pub flags: __u8,
    pub _reserved: [__u8; 3],
    pub translate_length: __u64,
    pub num_nvdimms: __u32,
    pub devices: [nd_nvdimm_device; 0],
}

#[repr(C, packed)]
pub struct nd_cmd_ars_err_inj {
    pub err_inj_spa_range_base: __u64,
    pub err_inj_spa_range_length: __u64,
    pub err_inj_options: __u8,
    pub status: __u32,
}

#[repr(C, packed)]
pub struct nd_cmd_ars_err_inj_clr {
    pub err_inj_clr_spa_range_base: __u64,
    pub err_inj_clr_spa_range_length: __u64,
    pub status: __u32,
}

#[repr(C, packed)]
pub struct nd_error_stat_query_record {
    pub err_inj_stat_spa_range_base: __u64,
    pub err_inj_stat_spa_range_length: __u64,
}

#[repr(C, packed)]
pub struct nd_cmd_ars_err_inj_stat {
    pub status: __u32,
    pub inj_err_rec_count: __u32,
    pub record: [nd_error_stat_query_record; 0],
}

pub const ND_INTEL_SMART: i32 = 1;
pub const ND_INTEL_SMART_THRESHOLD: i32 = 2;
pub const ND_INTEL_ENABLE_LSS_STATUS: i32 = 10;
pub const ND_INTEL_FW_GET_INFO: i32 = 12;
pub const ND_INTEL_FW_START_UPDATE: i32 = 13;
pub const ND_INTEL_FW_SEND_DATA: i32 = 14;
pub const ND_INTEL_FW_FINISH_UPDATE: i32 = 15;
pub const ND_INTEL_FW_FINISH_QUERY: i32 = 16;
pub const ND_INTEL_SMART_SET_THRESHOLD: i32 = 17;
pub const ND_INTEL_SMART_INJECT: i32 = 18;

pub const ND_INTEL_SMART_HEALTH_VALID: i32 = 1 << 0;
pub const ND_INTEL_SMART_SPARES_VALID: i32 = 1 << 1;
pub const ND_INTEL_SMART_USED_VALID: i32 = 1 << 2;
pub const ND_INTEL_SMART_MTEMP_VALID: i32 = 1 << 3;
pub const ND_INTEL_SMART_CTEMP_VALID: i32 = 1 << 4;
pub const ND_INTEL_SMART_SHUTDOWN_COUNT_VALID: i32 = 1 << 5;
pub const ND_INTEL_SMART_AIT_STATUS_VALID: i32 = 1 << 6;
pub const ND_INTEL_SMART_PTEMP_VALID: i32 = 1 << 7;
pub const ND_INTEL_SMART_ALARM_VALID: i32 = 1 << 9;
pub const ND_INTEL_SMART_SHUTDOWN_VALID: i32 = 1 << 10;
pub const ND_INTEL_SMART_VENDOR_VALID: i32 = 1 << 11;
pub const ND_INTEL_SMART_SPARE_TRIP: i32 = 1 << 0;
pub const ND_INTEL_SMART_TEMP_TRIP: i32 = 1 << 1;
pub const ND_INTEL_SMART_CTEMP_TRIP: i32 = 1 << 2;
pub const ND_INTEL_SMART_NON_CRITICAL_HEALTH: i32 = 1 << 0;
pub const ND_INTEL_SMART_CRITICAL_HEALTH: i32 = 1 << 1;
pub const ND_INTEL_SMART_FATAL_HEALTH: i32 = 1 << 2;
pub const ND_INTEL_SMART_INJECT_MTEMP: i32 = 1 << 0;
pub const ND_INTEL_SMART_INJECT_SPARE: i32 = 1 << 1;
pub const ND_INTEL_SMART_INJECT_FATAL: i32 = 1 << 2;
pub const ND_INTEL_SMART_INJECT_SHUTDOWN: i32 = 1 << 3;

#[repr(C, packed)]
pub struct nd_intel_smart_threshold_fields {
    pub alarm_control: __u16,
    pub spares: __u8,
    pub media_temperature: __u16,
    pub ctrl_temperature: __u16,
    pub reserved: [__u8; 1],
}

#[repr(C)]
pub union nd_intel_smart_threshold_union {
    pub fields: nd_intel_smart_threshold_fields,
    pub data: [__u8; 8],
}

#[repr(C, packed)]
pub struct nd_intel_smart_threshold {
    pub status: __u32,
    pub u: nd_intel_smart_threshold_union,
}

#[repr(C, packed)]
pub struct nd_intel_smart_set_threshold {
    pub alarm_control: __u16,
    pub spares: __u8,
    pub media_temperature: __u16,
    pub ctrl_temperature: __u16,
    pub status: __u32,
}

#[repr(C, packed)]
pub struct nd_intel_smart_inject {
    pub flags: __u64,
    pub mtemp_enable: __u8,
    pub media_temperature: __u16,
    pub spare_enable: __u8,
    pub spares: __u8,
    pub fatal_enable: __u8,
    pub unsafe_shutdown_enable: __u8,
    pub status: __u32,
}

pub const INTEL_FW_STORAGE_SIZE: i32 = 0x100000;
pub const INTEL_FW_MAX_SEND_LEN: i32 = 0xFFEC;
pub const INTEL_FW_QUERY_INTERVAL: i32 = 250000;
pub const INTEL_FW_QUERY_MAX_TIME: i32 = 3000000;
pub const INTEL_FW_FIS_VERSION: i32 = 0x0105;
pub const INTEL_FW_FAKE_VERSION: u64 = 0xffffffffabcd;

#[repr(C)]
pub enum intel_fw_update_state {
    FW_STATE_NEW = 0,
    FW_STATE_IN_PROGRESS,
    FW_STATE_VERIFY,
    FW_STATE_UPDATED,
}

#[repr(C, packed)]
pub struct nd_intel_fw_info {
    pub status: __u32,
    pub storage_size: __u32,
    pub max_send_len: __u32,
    pub query_interval: __u32,
    pub max_query_time: __u32,
    pub update_cap: __u8,
    pub reserved: [__u8; 3],
    pub fis_version: __u32,
    pub run_version: __u64,
    pub updated_version: __u64,
}

#[repr(C, packed)]
pub struct nd_intel_fw_start {
    pub status: __u32,
    pub context: __u32,
}

/* this one has the output first because the variable input data size */
#[repr(C, packed)]
pub struct nd_intel_fw_send_data {
    pub context: __u32,
    pub offset: __u32,
    pub length: __u32,
    pub data: [__u8; 0],
    /* this field is not declared due ot variable data from input */
    /*	__u32 status; */
}

#[repr(C, packed)]
pub struct nd_intel_fw_finish_update {
    pub ctrl_flags: __u8,
    pub reserved: [__u8; 3],
    pub context: __u32,
    pub status: __u32,
}

#[repr(C, packed)]
pub struct nd_intel_fw_finish_query {
    pub context: __u32,
    pub status: __u32,
    pub updated_fw_rev: __u64,
}

#[repr(C, packed)]
pub struct nd_intel_lss {
    pub enable: __u8,
    pub status: __u32,
}

pub type nfit_test_lookup_fn =
    Option<unsafe extern "C" fn(resource_size_t) -> *mut nfit_test_resource>;
pub type nfit_test_evaluate_dsm_fn = Option<
    unsafe extern "C" fn(
        handle: acpi_handle,
        guid: *const guid_t,
        rev: u64,
        func: u64,
        argv4: *mut acpi_object,
    ) -> *mut acpi_object,
>;

unsafe extern "C" {
    pub fn __wrap_devm_ioremap(
        dev: *mut device,
        offset: resource_size_t,
        size: core::ffi::c_ulong,
    ) -> *mut c_void;
    pub fn __wrap_devm_memremap(
        dev: *mut device,
        offset: resource_size_t,
        size: size_t,
        flags: core::ffi::c_ulong,
    ) -> *mut c_void;
    pub fn __wrap_devm_memremap_pages(
        dev: *mut device,
        pgmap: *mut dev_pagemap,
    ) -> *mut c_void;
    pub fn __wrap_memremap(
        offset: resource_size_t,
        size: size_t,
        flags: core::ffi::c_ulong,
    ) -> *mut c_void;
    pub fn __wrap_devm_memunmap(dev: *mut device, addr: *mut c_void);
    pub fn __wrap_ioremap(offset: resource_size_t, size: core::ffi::c_ulong) -> *mut c_void;
    pub fn __wrap_ioremap_wc(offset: resource_size_t, size: core::ffi::c_ulong) -> *mut c_void;
    pub fn __wrap_iounmap(addr: *mut c_void);
    pub fn __wrap_memunmap(addr: *mut c_void);
    pub fn __wrap___request_region(
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
        name: *const c_char,
        flags: i32,
    ) -> *mut resource;
    pub fn __wrap_insert_resource(parent: *mut resource, res: *mut resource) -> i32;
    pub fn __wrap_remove_resource(res: *mut resource) -> i32;
    pub fn __wrap___devm_request_region(
        dev: *mut device,
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
        name: *const c_char,
    ) -> *mut resource;
    pub fn __wrap___release_region(parent: *mut resource, start: resource_size_t, n: resource_size_t);
    pub fn __wrap___devm_release_region(
        dev: *mut device,
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
    );
    pub fn __wrap_acpi_evaluate_object(
        handle: acpi_handle,
        path: acpi_string,
        p: *mut acpi_object_list,
        buf: *mut acpi_buffer,
    ) -> acpi_status;
    pub fn __wrap_acpi_evaluate_dsm(
        handle: acpi_handle,
        guid: *const guid_t,
        rev: u64,
        func: u64,
        argv4: *mut acpi_object,
    ) -> *mut acpi_object;

    pub fn nfit_test_setup(lookup: nfit_test_lookup_fn, evaluate: nfit_test_evaluate_dsm_fn);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

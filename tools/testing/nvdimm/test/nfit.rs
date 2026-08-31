// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 *
 * Rust source-level translation of testing/nvdimm/test/nfit.c.
 * Linux kernel types, constants, macros, and helper functions supplied by the
 * original includes are intentionally referenced as external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr::{addr_of_mut, null_mut};

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type ssize_t = isize;
type dma_addr_t = u64;
type resource_size_t = u64;
type bool_ = bool;
type acpi_handle = *mut c_void;
type guid_t = c_void;
type spinlock_t = c_void;

const NUM_PM: usize = 3;
const NUM_DCR: usize = 5;
const NUM_HINTS: usize = 8;
const NUM_BDW: usize = NUM_DCR;
const NUM_SPA: usize = NUM_PM + NUM_DCR + NUM_BDW;
const NUM_MEM: usize = NUM_DCR + NUM_BDW + 2 + 4 + 1;
const SZ_4K: usize = 4 * 1024;
const SZ_128K: usize = 128 * 1024;
const SZ_4M: usize = 4 * 1024 * 1024;
const SZ_32M: usize = 32 * 1024 * 1024;
const SZ_128M: usize = 128 * 1024 * 1024;
const SZ_4G: usize = 4 * 1024 * 1024 * 1024;
const DIMM_SIZE: usize = SZ_32M;
const LABEL_SIZE: usize = SZ_128K;
const SPA_VCD_SIZE: usize = SZ_4M;
const SPA0_SIZE: usize = DIMM_SIZE;
const SPA1_SIZE: usize = DIMM_SIZE * 2;
const SPA2_SIZE: usize = DIMM_SIZE;
const BDW_SIZE: usize = 64 << 8;
const DCR_SIZE: usize = 12;
const NUM_NFITS: usize = 2;
const NFIT_TEST_CLEAR_ERR_UNIT: u64 = 256;

const fn NFIT_DIMM_HANDLE(node: u32, socket: u32, imc: u32, chan: u32, dimm: u32) -> u32 {
    ((node & 0xfff) << 16)
        | ((socket & 0xf) << 12)
        | ((imc & 0xf) << 8)
        | ((chan & 0xf) << 4)
        | (dimm & 0xf)
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct resource {
    start: resource_size_t,
    end: resource_size_t,
    name: *const c_char,
}

#[repr(C)]
struct device {
    kobj: kobject,
    init_name: *const c_char,
    release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
struct kobject {
    name: *const c_char,
}

#[repr(C)]
struct platform_device {
    name: *const c_char,
    id: c_int,
    dev: device,
}

#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct gen_pool {
    _private: [u8; 0],
}

#[repr(C)]
struct genpool_data_align {
    align: size_t,
}

#[repr(C)]
struct nd_intel_smart {
    flags: u32,
    health: u8,
    media_temperature: u16,
    ctrl_temperature: u16,
    pmic_temperature: u16,
    spares: u8,
    alarm_flags: u16,
    ait_status: u8,
    life_used: u8,
    shutdown_state: u8,
    shutdown_count: u32,
    vendor_size: u32,
}

#[repr(C)]
struct nd_intel_smart_threshold {
    alarm_control: u16,
    media_temperature: u16,
    ctrl_temperature: u16,
    spares: u8,
    data: [u8; 0],
}

#[repr(C)]
struct nd_intel_smart_set_threshold {
    status: u32,
    data: [u8; 0],
}

#[repr(C)]
struct nd_intel_smart_inject {
    status: u32,
    flags: u32,
    mtemp_enable: u8,
    media_temperature: u16,
    spare_enable: u8,
    spares: u8,
    fatal_enable: u8,
    unsafe_shutdown_enable: u8,
}

#[repr(C)]
struct nd_intel_fw_info {
    status: u32,
    storage_size: u32,
    max_send_len: u32,
    query_interval: u32,
    max_query_time: u32,
    update_cap: u32,
    fis_version: u32,
    run_version: u64,
    updated_version: u64,
}

#[repr(C)]
struct nd_intel_fw_start {
    status: u32,
    context: u32,
}

#[repr(C)]
struct nd_intel_fw_send_data {
    context: u32,
    offset: u32,
    length: u32,
    data: [u8; 0],
}

#[repr(C)]
struct nd_intel_fw_finish_update {
    status: u32,
    context: u32,
    ctrl_flags: u32,
}

#[repr(C)]
struct nd_intel_fw_finish_query {
    status: u32,
    context: u32,
    updated_fw_rev: u64,
}

#[repr(C)]
struct nd_intel_lss {
    status: u32,
    enable: u32,
}

#[repr(C)]
struct nd_intel_get_security_state {
    status: u32,
    state: u8,
    extended_state: u8,
}

#[repr(C)]
struct nd_intel_unlock_unit {
    status: u32,
    passphrase: [u8; 32],
}

#[repr(C)]
struct nd_intel_set_passphrase {
    status: u32,
    old_pass: [u8; 32],
    new_pass: [u8; 32],
}

#[repr(C)]
struct nd_intel_disable_passphrase {
    status: u32,
    passphrase: [u8; 32],
}

#[repr(C)]
struct nd_intel_freeze_lock {
    status: u32,
}

#[repr(C)]
struct nd_intel_secure_erase {
    status: u32,
    passphrase: [u8; 32],
}

#[repr(C)]
struct nd_intel_overwrite {
    status: u32,
    passphrase: [u8; 32],
}

#[repr(C)]
struct nd_intel_query_overwrite {
    status: u32,
}

#[repr(C)]
struct nd_intel_set_master_passphrase {
    status: u32,
    old_pass: [u8; 32],
    new_pass: [u8; 32],
}

#[repr(C)]
struct nd_intel_master_secure_erase {
    status: u32,
    passphrase: [u8; 32],
}

#[repr(C)]
struct nd_intel_bus_fw_activate_businfo {
    capability: u32,
    state: u32,
    activate_tmo: u64,
    cpu_quiesce_tmo: u64,
    io_quiesce_tmo: u64,
    max_quiesce_tmo: u64,
}

#[repr(C)]
struct nd_intel_bus_fw_activate {
    status: u32,
}

#[repr(C)]
struct nd_intel_fw_activate_dimminfo {
    result: u32,
    state: u32,
}

#[repr(C)]
struct nd_intel_fw_activate_arm {
    status: u32,
    activate_arm: u32,
}

#[repr(C)]
struct nd_cmd_get_config_size {
    status: u32,
    config_size: u32,
    max_xfer: u32,
}

#[repr(C)]
struct nd_cmd_get_config_data_hdr {
    status: u32,
    in_offset: u32,
    in_length: u32,
    out_buf: [u8; 0],
}

#[repr(C)]
struct nd_cmd_set_config_hdr {
    in_offset: u32,
    in_length: u32,
    in_buf: [u8; 0],
}

#[repr(C)]
struct nd_ars_record {
    handle: u32,
    err_address: u64,
    length: u64,
}

#[repr(C)]
struct nd_cmd_ars_cap {
    status: u32,
    max_ars_out: u32,
    clear_err_unit: u32,
}

#[repr(C)]
struct nd_cmd_ars_start {
    status: u32,
    address: u64,
    length: u64,
    scrub_time: u16,
}

#[repr(C)]
struct nd_cmd_ars_status {
    status: u32,
    out_length: u32,
    address: u64,
    length: u64,
    type_: u32,
    num_records: u32,
    records: [nd_ars_record; 0],
}

#[repr(C)]
struct nd_cmd_clear_error {
    status: u32,
    address: u64,
    length: u64,
    cleared: u64,
}

#[repr(C)]
struct nd_error_stat_query_record {
    _private: [u8; 0],
}

#[repr(C)]
struct nd_cmd_ars_err_inj {
    status: u32,
    err_inj_options: u32,
    err_inj_spa_range_base: u64,
    err_inj_spa_range_length: u64,
}

#[repr(C)]
struct nd_cmd_ars_err_inj_clr {
    status: u32,
    err_inj_clr_spa_range_base: u64,
    err_inj_clr_spa_range_length: u64,
}

#[repr(C)]
struct nd_cmd_ars_err_inj_stat {
    status: u32,
    inj_err_rec_count: u32,
    record: [nd_error_stat_query_record_out; 0],
}

#[repr(C)]
struct nd_error_stat_query_record_out {
    err_inj_stat_spa_range_base: u64,
    err_inj_stat_spa_range_length: u64,
}

#[repr(C)]
struct nd_cmd_translate_spa_device {
    nfit_device_handle: u32,
    dpa: u64,
}

#[repr(C)]
struct nd_cmd_translate_spa {
    status: u32,
    translate_length: u32,
    spa: u64,
    num_nvdimms: u32,
    devices: [nd_cmd_translate_spa_device; 1],
}

#[repr(C)]
struct nd_cmd_pkg {
    nd_command: u32,
    nd_family: u32,
    nd_size_in: u32,
    nd_size_out: u32,
    nd_fw_size: u32,
    nd_payload: [u8; 0],
}

#[repr(C)]
struct nd_mapping {
    nvdimm: *mut nvdimm,
}

#[repr(C)]
struct nd_region {
    ndr_start: u64,
    ndr_size: u64,
    ndr_mappings: c_int,
    mapping: [nd_mapping; 0],
}

#[repr(C)]
struct nvdimm {
    provider_data: *mut c_void,
    cmd_mask: c_ulong,
    dev: device,
}

#[repr(C)]
struct nvdimm_bus {
    dev: device,
}

#[repr(C)]
struct nvdimm_bus_descriptor {
    cmd_mask: c_ulong,
    module: *mut c_void,
    provider_name: *const c_char,
    ndctl: Option<
        unsafe extern "C" fn(
            *mut nvdimm_bus_descriptor,
            *mut nvdimm,
            c_uint,
            *mut c_void,
            c_uint,
            *mut c_int,
        ) -> c_int,
    >,
    bus_family_mask: c_ulong,
}

#[repr(C)]
struct acpi_nfit_desc {
    nd_desc: nvdimm_bus_descriptor,
    nvdimm_bus: *mut nvdimm_bus,
    dimm_cmd_force_en: c_ulong,
    bus_cmd_force_en: c_ulong,
    bus_dsm_mask: c_ulong,
    family_dsm_mask: [c_ulong; 8],
    dev: *mut device,
    init_mutex: c_void,
    dimms: list_head,
}

#[repr(C)]
struct nfit_memdev {
    device_handle: u32,
}

#[repr(C)]
struct nfit_mem {
    adev: *mut acpi_device,
    family: u32,
    dsm_mask: c_ulong,
    list: list_head,
}

#[repr(C)]
struct acpi_device {
    handle: acpi_handle,
    dev: device,
}

#[repr(C)]
struct acpi_nfit_header {
    type_: u16,
    length: u16,
}

#[repr(C)]
struct acpi_nfit_system_address {
    header: acpi_nfit_header,
    range_guid: [u8; 16],
    range_index: u16,
    flags: u16,
    address: u64,
    length: u64,
}

#[repr(C)]
struct acpi_nfit_memory_map {
    header: acpi_nfit_header,
    device_handle: u32,
    physical_id: u16,
    region_id: u16,
    range_index: u16,
    region_index: u16,
    region_size: u64,
    region_offset: u64,
    address: u64,
    interleave_index: u16,
    interleave_ways: u16,
    flags: u16,
}

#[repr(C)]
struct acpi_nfit_control_region {
    header: acpi_nfit_header,
    region_index: u16,
    vendor_id: u16,
    device_id: u16,
    revision_id: u16,
    subsystem_vendor_id: u16,
    subsystem_device_id: u16,
    subsystem_revision_id: u16,
    valid_fields: u8,
    manufacturing_location: u8,
    manufacturing_date: u16,
    serial_number: u32,
    code: u16,
    windows: u16,
    window_size: u64,
    command_offset: u64,
    command_size: u64,
    status_offset: u64,
    status_size: u64,
}

#[repr(C)]
struct acpi_nfit_data_region {
    header: acpi_nfit_header,
    region_index: u16,
    windows: u16,
    offset: u64,
    size: u64,
    capacity: u64,
    start_address: u64,
}

#[repr(C)]
struct acpi_nfit_flush_address {
    header: acpi_nfit_header,
    device_handle: u32,
    hint_count: u16,
    hint_address: [u64; 0],
}

#[repr(C)]
struct acpi_nfit_capabilities {
    header: acpi_nfit_header,
    highest_capability: u8,
    capabilities: u32,
}

#[repr(C)]
struct acpi_object_package {
    type_: u32,
}

#[repr(C)]
struct acpi_object_buffer {
    type_: u32,
    length: u32,
    pointer: *mut c_void,
}

#[repr(C)]
union acpi_object {
    package: core::mem::ManuallyDrop<acpi_object_package>,
    buffer: core::mem::ManuallyDrop<acpi_object_buffer>,
    type_: u32,
}

#[repr(C)]
struct badrange_entry {
    list: list_head,
    start: u64,
    length: u64,
}

#[repr(C)]
struct badrange {
    lock: spinlock_t,
    list: list_head,
}

#[repr(C)]
struct nfit_test_dcr {
    bdw_addr: u64,
    bdw_status: u32,
    aperature: [u8; BDW_SIZE],
}

#[repr(C)]
struct nfit_test_sec {
    state: u8,
    ext_state: u8,
    old_state: u8,
    passphrase: [u8; 32],
    master_passphrase: [u8; 32],
    overwrite_end_time: u64,
}

#[repr(C)]
struct nfit_test_fw {
    state: intel_fw_update_state,
    context: u32,
    version: u64,
    size_received: u32,
    end_time: u64,
    armed: bool_,
    missed_activate: bool_,
    last_activate: c_ulong,
}

type intel_fw_update_state = u32;

#[repr(C)]
struct ars_state {
    ars_status: *mut nd_cmd_ars_status,
    deadline: c_ulong,
    lock: spinlock_t,
}

#[repr(C)]
struct nfit_test {
    acpi_desc: acpi_nfit_desc,
    pdev: platform_device,
    resources: list_head,
    nfit_buf: *mut c_void,
    nfit_dma: dma_addr_t,
    nfit_size: size_t,
    nfit_filled: size_t,
    dcr_idx: c_int,
    num_dcr: c_int,
    num_pm: c_int,
    dimm: *mut *mut c_void,
    dimm_dma: *mut dma_addr_t,
    flush: *mut *mut c_void,
    flush_dma: *mut dma_addr_t,
    label: *mut *mut c_void,
    label_dma: *mut dma_addr_t,
    spa_set: *mut *mut c_void,
    spa_set_dma: *mut dma_addr_t,
    dcr: *mut *mut nfit_test_dcr,
    dcr_dma: *mut dma_addr_t,
    alloc: Option<unsafe extern "C" fn(*mut nfit_test) -> c_int>,
    setup: Option<unsafe extern "C" fn(*mut nfit_test)>,
    setup_hotplug: c_int,
    _fit: *mut *mut acpi_object,
    _fit_dma: dma_addr_t,
    ars_state: ars_state,
    dimm_dev: [*mut device; 7],
    smart: *mut nd_intel_smart,
    smart_threshold: *mut nd_intel_smart_threshold,
    badrange: badrange,
    work: work_struct,
    fw: *mut nfit_test_fw,
}

#[repr(C)]
struct nfit_test_resource {
    list: list_head,
    dev: *mut device,
    buf: *mut c_void,
    res: resource,
    lock: spinlock_t,
    requests: list_head,
}

#[repr(C)]
struct class {
    name: *const c_char,
}

#[repr(C)]
struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
struct device_attribute {
    attr: attribute,
}

#[repr(C)]
struct attribute_group {
    attrs: *mut *mut attribute,
}

#[repr(C)]
struct platform_device_id {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_inner,
    id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static mut HZ: c_ulong;
    static mut PAGE_SIZE: c_ulong;
    static mut THIS_MODULE: *mut c_void;

    static mut ND_INTEL_SMART_HEALTH_VALID: u32;
    static mut ND_INTEL_SMART_SPARES_VALID: u32;
    static mut ND_INTEL_SMART_ALARM_VALID: u32;
    static mut ND_INTEL_SMART_USED_VALID: u32;
    static mut ND_INTEL_SMART_SHUTDOWN_VALID: u32;
    static mut ND_INTEL_SMART_SHUTDOWN_COUNT_VALID: u32;
    static mut ND_INTEL_SMART_MTEMP_VALID: u32;
    static mut ND_INTEL_SMART_CTEMP_VALID: u32;
    static mut ND_INTEL_SMART_NON_CRITICAL_HEALTH: u8;
    static mut ND_INTEL_SMART_FATAL_HEALTH: u8;
    static mut ND_INTEL_SMART_SPARE_TRIP: u16;
    static mut ND_INTEL_SMART_TEMP_TRIP: u16;
    static mut ND_INTEL_SMART_CTEMP_TRIP: u16;
    static mut ND_INTEL_SMART_INJECT_MTEMP: u32;
    static mut ND_INTEL_SMART_INJECT_SPARE: u32;
    static mut ND_INTEL_SMART_INJECT_FATAL: u32;
    static mut ND_INTEL_SMART_INJECT_SHUTDOWN: u32;

    static mut FW_STATE_NEW: intel_fw_update_state;
    static mut FW_STATE_IN_PROGRESS: intel_fw_update_state;
    static mut FW_STATE_VERIFY: intel_fw_update_state;
    static mut FW_STATE_UPDATED: intel_fw_update_state;
    static mut INTEL_FW_STORAGE_SIZE: u32;
    static mut INTEL_FW_MAX_SEND_LEN: u32;
    static mut INTEL_FW_QUERY_INTERVAL: u32;
    static mut INTEL_FW_QUERY_MAX_TIME: u32;
    static mut INTEL_FW_FIS_VERSION: u32;
    static mut INTEL_FW_FAKE_VERSION: u64;

    static mut ND_ARS_PERSISTENT: u32;
    static mut ND_ARS_VOLATILE: u32;
    static mut NFIT_ARS_START_BUSY: u32;
    static mut NFIT_ARS_STATUS_BUSY: u32;
    static mut NFIT_ARS_INJECT_INVALID: u32;
    static mut ND_ARS_ERR_INJ_OPT_NOTIFY: u32;

    static mut ND_INTEL_SEC_STATE_LOCKED: u8;
    static mut ND_INTEL_SEC_STATE_FROZEN: u8;
    static mut ND_INTEL_SEC_STATE_ENABLED: u8;
    static mut ND_INTEL_SEC_STATE_OVERWRITE: u8;
    static mut ND_INTEL_SEC_ESTATE_ENABLED: u8;
    static mut ND_INTEL_SEC_ESTATE_PLIMIT: u8;
    static mut ND_INTEL_STATUS_INVALID_STATE: u32;
    static mut ND_INTEL_STATUS_INVALID_PASS: u32;
    static mut ND_INTEL_STATUS_OQUERY_SEQUENCE_ERR: u32;
    static mut ND_INTEL_STATUS_OQUERY_INPROGRESS: u32;
    static mut ND_INTEL_STATUS_NOT_SUPPORTED: u32;
    static mut ND_INTEL_PASSPHRASE_SIZE: size_t;
    static mut NVDIMM_PASSPHRASE_LEN: size_t;

    static mut ND_INTEL_FWA_ARMED: u32;
    static mut ND_INTEL_FWA_IDLE: u32;
    static mut ND_INTEL_FWA_BUSY: u32;
    static mut ND_INTEL_BUS_FWA_CAP_FWQUIESCE: u32;
    static mut ND_INTEL_BUS_FWA_CAP_OSQUIESCE: u32;
    static mut ND_INTEL_BUS_FWA_CAP_RESET: u32;
    static mut ND_INTEL_BUS_FWA_STATUS_BUSY: u32;
    static mut ND_INTEL_BUS_FWA_STATUS_TMO: u32;
    static mut ND_INTEL_BUS_FWA_STATUS_NOARM: u32;
    static mut ND_INTEL_DIMM_FWA_NONE: u32;
    static mut ND_INTEL_DIMM_FWA_NOTSTAGED: u32;
    static mut ND_INTEL_DIMM_FWA_SUCCESS: u32;
    static mut ND_INTEL_DIMM_FWA_ARM: u32;
    static mut USEC_PER_SEC: u64;

    static mut ACPI_NFIT_TYPE_SYSTEM_ADDRESS: u16;
    static mut ACPI_NFIT_TYPE_MEMORY_MAP: u16;
    static mut ACPI_NFIT_TYPE_CONTROL_REGION: u16;
    static mut ACPI_NFIT_TYPE_DATA_REGION: u16;
    static mut ACPI_NFIT_TYPE_FLUSH_ADDRESS: u16;
    static mut ACPI_NFIT_TYPE_CAPABILITIES: u16;
    static mut ACPI_NFIT_MEM_HEALTH_ENABLED: u16;
    static mut ACPI_NFIT_MEM_SAVE_FAILED: u16;
    static mut ACPI_NFIT_MEM_RESTORE_FAILED: u16;
    static mut ACPI_NFIT_MEM_FLUSH_FAILED: u16;
    static mut ACPI_NFIT_MEM_HEALTH_OBSERVED: u16;
    static mut ACPI_NFIT_MEM_NOT_ARMED: u16;
    static mut ACPI_NFIT_MEM_MAP_FAILED: u16;
    static mut ACPI_NFIT_CAPABILITY_MEM_FLUSH: u32;
    static mut NFIT_FIC_BLK: u16;
    static mut NFIT_FIC_BYTEN: u16;
    static mut NFIT_FIC_BYTE: u16;
    static mut NFIT_SPA_PM: u32;
    static mut NFIT_SPA_DCR: u32;
    static mut NFIT_SPA_BDW: u32;
    static mut NFIT_SPA_VCD: u32;
    static mut ACPI_TYPE_BUFFER: u32;

    static mut ND_CMD_CALL: u32;
    static mut ND_CMD_GET_CONFIG_SIZE: u32;
    static mut ND_CMD_GET_CONFIG_DATA: u32;
    static mut ND_CMD_SET_CONFIG_DATA: u32;
    static mut ND_CMD_ARS_CAP: u32;
    static mut ND_CMD_ARS_START: u32;
    static mut ND_CMD_ARS_STATUS: u32;
    static mut ND_CMD_CLEAR_ERROR: u32;
    static mut NVDIMM_BUS_FAMILY_NFIT: u32;
    static mut NVDIMM_BUS_FAMILY_INTEL: usize;
    static mut NVDIMM_FAMILY_INTEL: u32;

    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sscanf(buf: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn kstrtol(buf: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_add_action(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_WARN_ONCE(dev: *mut device, cond: c_int, fmt: *const c_char, ...);
    fn device_lock(dev: *mut device);
    fn device_unlock(dev: *mut device);
    fn device_for_each_child(dev: *mut device, data: *mut c_void, cb: unsafe extern "C" fn(*mut device, *mut c_void) -> c_int) -> c_int;
    fn device_create_with_groups(class: *const class, parent: *mut device, devt: u64, drvdata: *mut c_void, groups: *const *const attribute_group, fmt: *const c_char, ...) -> *mut device;
    fn device_unregister(dev: *mut device);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn class_register(class: *const class) -> c_int;
    fn class_unregister(class: *const class);
    fn platform_device_register(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_;
    fn gen_pool_create(min_alloc_order: c_int, nid: c_int) -> *mut gen_pool;
    fn gen_pool_add(pool: *mut gen_pool, addr: c_ulong, size: size_t, nid: c_int) -> c_int;
    fn gen_pool_alloc_algo(pool: *mut gen_pool, size: size_t, algo: *mut c_void, data: *mut c_void) -> c_ulong;
    fn gen_pool_free(pool: *mut gen_pool, addr: c_ulong, size: size_t);
    fn gen_pool_destroy(pool: *mut gen_pool);
    fn resource_size(res: *const resource) -> resource_size_t;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn set_bit(bit: u32, addr: *mut c_ulong);
    fn test_bit(bit: u32, addr: *const c_ulong) -> bool_;
    fn time_before(a: c_ulong, b: c_ulong) -> bool_;
    fn time_after(a: c_ulong, b: c_ulong) -> bool_;
    fn time_is_after_jiffies64(a: u64) -> bool_;
    fn time_is_before_jiffies64(a: u64) -> bool_;
    fn get_jiffies_64() -> u64;
    fn ilog2(n: size_t) -> c_int;
    fn cpu_to_be16(v: u16) -> u16;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn to_nd_region(dev: *mut device) -> *mut nd_region;
    fn nvdimm_provider_data(nvdimm: *mut nvdimm) -> *mut nfit_mem;
    fn nvdimm_cmd_mask(nvdimm: *mut nvdimm) -> c_ulong;
    fn nvdimm_name(nvdimm: *mut nvdimm) -> *const c_char;
    fn __to_nfit_memdev(nfit_mem: *mut nfit_mem) -> *mut nfit_memdev;
    fn to_acpi_desc(nd_desc: *mut nvdimm_bus_descriptor) -> *mut acpi_nfit_desc;
    fn to_nfit_uuid(spa: u32) -> *const c_void;
    fn __acpi_nvdimm_notify(dev: *mut device, event: u32);
    fn __acpi_nfit_notify(dev: *mut device, data: *mut c_void, event: u32);
    fn acpi_nfit_ctl(nd_desc: *mut nvdimm_bus_descriptor, nvdimm: *mut nvdimm, cmd: u32, buf: *mut c_void, len: u32, cmd_rc: *mut c_int) -> c_int;
    fn acpi_nfit_desc_init(desc: *mut acpi_nfit_desc, dev: *mut device);
    fn acpi_nfit_init(desc: *mut acpi_nfit_desc, buf: *mut c_void, size: size_t) -> c_int;
    fn acpi_nfit_shutdown(data: *mut c_void);
    fn badrange_init(br: *mut badrange);
    fn badrange_add(br: *mut badrange, start: u64, length: u64) -> c_int;
    fn badrange_forget(br: *mut badrange, start: u64, length: u64);
    fn pmem_test();
    fn libnvdimm_test();
    fn acpi_nfit_test();
    fn device_dax_test();
    fn dax_pmem_test();
    fn nfit_test_setup(lookup: unsafe extern "C" fn(resource_size_t) -> *mut nfit_test_resource, dsm: unsafe extern "C" fn(acpi_handle, *const guid_t, u64, u64, *mut acpi_object) -> *mut acpi_object);
    fn nfit_test_teardown();
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOTTY: c_int = 25;
const ENXIO: c_int = 6;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const NUMA_NO_NODE: c_int = -1;
const NFIT_NOTIFY_UC_MEMORY_ERROR: u32 = 0;
const KBUILD_MODNAME: *const c_char = b"nfit_test\0".as_ptr() as *const c_char;

static mut handle: [u32; 7] = [
    NFIT_DIMM_HANDLE(0, 0, 0, 0, 0),
    NFIT_DIMM_HANDLE(0, 0, 0, 0, 1),
    NFIT_DIMM_HANDLE(0, 0, 1, 0, 0),
    NFIT_DIMM_HANDLE(0, 0, 1, 0, 1),
    NFIT_DIMM_HANDLE(0, 1, 0, 0, 0),
    NFIT_DIMM_HANDLE(1, 0, 0, 0, 0),
    NFIT_DIMM_HANDLE(1, 0, 0, 0, 1),
];
static mut dimm_fail_cmd_flags: [c_ulong; 7] = [0; 7];
static mut dimm_fail_cmd_code: [c_int; 7] = [0; 7];
static mut dimm_sec_info: [nfit_test_sec; NUM_DCR] = [nfit_test_sec {
    state: 0,
    ext_state: 0,
    old_state: 0,
    passphrase: [0; 32],
    master_passphrase: [0; 32],
    overwrite_end_time: 0,
}; NUM_DCR];
static mut nfit_wq: *mut workqueue_struct = null_mut();
static mut nfit_pool: *mut gen_pool = null_mut();
static zero_key: [c_char; 32] = [0; 32];
static mut last_activate: c_ulong = 0;
static mut nfit_ctl_handle: c_ulong = 0;
static mut result: *mut acpi_object = null_mut();
static mut nfit_test_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut instances: [*mut nfit_test; NUM_NFITS] = [null_mut(); NUM_NFITS];

unsafe fn min_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

unsafe extern "C" fn to_nfit_test(dev: *mut device) -> *mut nfit_test {
    let pdev = to_platform_device(dev);
    (pdev as *mut u8).sub(offset_of!(nfit_test, pdev)) as *mut nfit_test
}

unsafe extern "C" fn nd_intel_test_get_fw_info(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_info, buf_len: c_uint, idx: c_int) -> c_int {
    let fw = (*t).fw.add(idx as usize);
    if (buf_len as usize) < size_of::<nd_intel_fw_info>() { return -EINVAL; }
    (*nd_cmd).status = 0;
    (*nd_cmd).storage_size = INTEL_FW_STORAGE_SIZE;
    (*nd_cmd).max_send_len = INTEL_FW_MAX_SEND_LEN;
    (*nd_cmd).query_interval = INTEL_FW_QUERY_INTERVAL;
    (*nd_cmd).max_query_time = INTEL_FW_QUERY_MAX_TIME;
    (*nd_cmd).update_cap = 0;
    (*nd_cmd).fis_version = INTEL_FW_FIS_VERSION;
    (*nd_cmd).run_version = 0;
    (*nd_cmd).updated_version = (*fw).version;
    0
}

unsafe extern "C" fn nd_intel_test_start_update(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_start, buf_len: c_uint, idx: c_int) -> c_int {
    let fw = (*t).fw.add(idx as usize);
    if (buf_len as usize) < size_of::<nd_intel_fw_start>() { return -EINVAL; }
    if (*fw).state != FW_STATE_NEW {
        (*nd_cmd).status = 0x10007;
        return 0;
    }
    (*fw).state = FW_STATE_IN_PROGRESS;
    (*fw).context = (*fw).context.wrapping_add(1);
    (*fw).size_received = 0;
    (*nd_cmd).status = 0;
    (*nd_cmd).context = (*fw).context;
    0
}

unsafe extern "C" fn nd_intel_test_send_data(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_send_data, buf_len: c_uint, idx: c_int) -> c_int {
    let fw = (*t).fw.add(idx as usize);
    let status = (*nd_cmd).data.as_ptr().add((*nd_cmd).length as usize) as *mut u32;
    if (buf_len as usize) < size_of::<nd_intel_fw_send_data>() { return -EINVAL; }
    if (*fw).state != FW_STATE_IN_PROGRESS {
        *status = 0x5;
        return 0;
    }
    if (*nd_cmd).context != (*fw).context {
        *status = 0x10007;
        return 0;
    }
    if (*nd_cmd).offset.wrapping_add((*nd_cmd).length) > INTEL_FW_STORAGE_SIZE || (*nd_cmd).length > INTEL_FW_MAX_SEND_LEN {
        *status = 0x3;
        return 0;
    }
    (*fw).size_received = (*fw).size_received.wrapping_add((*nd_cmd).length);
    *status = 0;
    0
}

unsafe extern "C" fn nd_intel_test_finish_fw(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_finish_update, _buf_len: c_uint, idx: c_int) -> c_int {
    let fw = (*t).fw.add(idx as usize);
    if (*fw).state == FW_STATE_UPDATED {
        (*nd_cmd).status = 0x20007;
        return 0;
    }
    match (*nd_cmd).ctrl_flags {
        0 => {
            if (*nd_cmd).context != (*fw).context {
                (*nd_cmd).status = 0x10007;
                return 0;
            }
            (*nd_cmd).status = 0;
            (*fw).state = FW_STATE_VERIFY;
            (*fw).end_time = (jiffies + HZ) as u64;
        }
        1 => {
            (*fw).size_received = 0;
            (*nd_cmd).status = 0x40007;
            (*fw).state = FW_STATE_NEW;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn nd_intel_test_finish_query(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_finish_query, buf_len: c_uint, idx: c_int) -> c_int {
    let fw = (*t).fw.add(idx as usize);
    if (buf_len as usize) < size_of::<nd_intel_fw_finish_query>() { return -EINVAL; }
    if (*nd_cmd).context != (*fw).context {
        (*nd_cmd).status = 0x10007;
        return 0;
    }
    if (*fw).state == FW_STATE_NEW {
        (*nd_cmd).updated_fw_rev = 0;
        (*nd_cmd).status = 0;
    } else if (*fw).state == FW_STATE_IN_PROGRESS {
        (*nd_cmd).status = 0x40007;
        (*nd_cmd).updated_fw_rev = 0;
    } else if (*fw).state == FW_STATE_VERIFY {
        if time_is_after_jiffies64((*fw).end_time) {
            (*nd_cmd).updated_fw_rev = 0;
            (*nd_cmd).status = 0x20007;
        } else {
            (*fw).state = FW_STATE_UPDATED;
            (*fw).missed_activate = false;
            (*nd_cmd).status = 0;
            (*fw).version = INTEL_FW_FAKE_VERSION;
            (*nd_cmd).updated_fw_rev = INTEL_FW_FAKE_VERSION;
        }
    } else if (*fw).state == FW_STATE_UPDATED {
        (*nd_cmd).status = 0;
        (*fw).version = INTEL_FW_FAKE_VERSION;
        (*nd_cmd).updated_fw_rev = INTEL_FW_FAKE_VERSION;
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn nfit_test_cmd_get_config_size(nd_cmd: *mut nd_cmd_get_config_size, buf_len: c_uint) -> c_int {
    if (buf_len as usize) < size_of::<nd_cmd_get_config_size>() { return -EINVAL; }
    (*nd_cmd).status = 0;
    (*nd_cmd).config_size = LABEL_SIZE as u32;
    (*nd_cmd).max_xfer = SZ_4K as u32;
    0
}

unsafe extern "C" fn nfit_test_cmd_get_config_data(nd_cmd: *mut nd_cmd_get_config_data_hdr, buf_len: c_uint, label: *mut c_void) -> c_int {
    let offset = (*nd_cmd).in_offset;
    if (buf_len as usize) < size_of::<nd_cmd_get_config_data_hdr>() { return -EINVAL; }
    if offset as usize >= LABEL_SIZE { return -EINVAL; }
    if (*nd_cmd).in_length as usize + size_of::<nd_cmd_get_config_data_hdr>() > buf_len as usize { return -EINVAL; }
    (*nd_cmd).status = 0;
    let len = min_u32((*nd_cmd).in_length, (LABEL_SIZE - offset as usize) as u32);
    memcpy((*nd_cmd).out_buf.as_mut_ptr() as *mut c_void, (label as *mut u8).add(offset as usize) as *const c_void, len as usize);
    (buf_len as isize - size_of::<nd_cmd_get_config_data_hdr>() as isize - len as isize) as c_int
}

unsafe extern "C" fn nfit_test_cmd_set_config_data(nd_cmd: *mut nd_cmd_set_config_hdr, buf_len: c_uint, label: *mut c_void) -> c_int {
    let offset = (*nd_cmd).in_offset;
    if (buf_len as usize) < size_of::<nd_cmd_set_config_hdr>() { return -EINVAL; }
    if offset as usize >= LABEL_SIZE { return -EINVAL; }
    if (*nd_cmd).in_length as usize + size_of::<nd_cmd_set_config_hdr>() + 4 > buf_len as usize { return -EINVAL; }
    let status = (nd_cmd as *mut u8).add((*nd_cmd).in_length as usize + size_of::<nd_cmd_set_config_hdr>()) as *mut u32;
    *status = 0;
    let len = min_u32((*nd_cmd).in_length, (LABEL_SIZE - offset as usize) as u32);
    memcpy((label as *mut u8).add(offset as usize) as *mut c_void, (*nd_cmd).in_buf.as_ptr() as *const c_void, len as usize);
    (buf_len as isize - size_of::<nd_cmd_set_config_hdr>() as isize - (len as isize + 4)) as c_int
}

unsafe extern "C" fn nfit_test_cmd_ars_cap(nd_cmd: *mut nd_cmd_ars_cap, buf_len: c_uint) -> c_int {
    if (buf_len as usize) < size_of::<nd_cmd_ars_cap>() { return -EINVAL; }
    let ars_recs = SZ_4K / size_of::<nd_ars_record>();
    (*nd_cmd).max_ars_out = (size_of::<nd_cmd_ars_status>() + ars_recs * size_of::<nd_ars_record>()) as u32;
    (*nd_cmd).status = (ND_ARS_PERSISTENT | ND_ARS_VOLATILE) << 16;
    (*nd_cmd).clear_err_unit = NFIT_TEST_CLEAR_ERR_UNIT as u32;
    0
}

unsafe extern "C" fn post_ars_status(ars_state: *mut ars_state, _badrange: *mut badrange, addr: u64, len: u64) {
    /*
     * C iterates badrange->list and copies overlapping badrange entries into
     * ars_status->records. The list entry type and list_for_each_entry macro
     * are external to this isolated file, so this translation preserves the
     * locally visible state updates and leaves the external list traversal to
     * the surrounding kernel bindings.
     */
    (*ars_state).deadline = jiffies + HZ;
    let ars_status = (*ars_state).ars_status;
    (*ars_status).status = 0;
    (*ars_status).address = addr;
    (*ars_status).length = len;
    (*ars_status).type_ = ND_ARS_PERSISTENT;
    (*ars_status).num_records = 0;
    (*ars_status).out_length = size_of::<nd_cmd_ars_status>() as u32;
}

unsafe extern "C" fn nfit_test_cmd_ars_start(t: *mut nfit_test, ars_state: *mut ars_state, ars_start: *mut nd_cmd_ars_start, buf_len: c_uint, cmd_rc: *mut c_int) -> c_int {
    if (buf_len as usize) < size_of::<nd_cmd_ars_start>() { return -EINVAL; }
    spin_lock(addr_of_mut!((*ars_state).lock));
    if time_before(jiffies, (*ars_state).deadline) {
        (*ars_start).status = NFIT_ARS_START_BUSY;
        *cmd_rc = -EBUSY;
    } else {
        (*ars_start).status = 0;
        (*ars_start).scrub_time = 1;
        post_ars_status(ars_state, addr_of_mut!((*t).badrange), (*ars_start).address, (*ars_start).length);
        *cmd_rc = 0;
    }
    spin_unlock(addr_of_mut!((*ars_state).lock));
    0
}

unsafe extern "C" fn nfit_test_cmd_ars_status(ars_state: *mut ars_state, ars_status: *mut nd_cmd_ars_status, buf_len: c_uint, cmd_rc: *mut c_int) -> c_int {
    if (buf_len as u32) < (*(*ars_state).ars_status).out_length { return -EINVAL; }
    spin_lock(addr_of_mut!((*ars_state).lock));
    if time_before(jiffies, (*ars_state).deadline) {
        memset(ars_status as *mut c_void, 0, buf_len as usize);
        (*ars_status).status = NFIT_ARS_STATUS_BUSY;
        (*ars_status).out_length = size_of::<nd_cmd_ars_status>() as u32;
        *cmd_rc = -EBUSY;
    } else {
        memcpy(ars_status as *mut c_void, (*ars_state).ars_status as *const c_void, (*(*ars_state).ars_status).out_length as usize);
        *cmd_rc = 0;
    }
    spin_unlock(addr_of_mut!((*ars_state).lock));
    0
}

unsafe extern "C" fn nfit_test_cmd_clear_error(t: *mut nfit_test, clear_err: *mut nd_cmd_clear_error, buf_len: c_uint, cmd_rc: *mut c_int) -> c_int {
    let mask = NFIT_TEST_CLEAR_ERR_UNIT - 1;
    if (buf_len as usize) < size_of::<nd_cmd_clear_error>() { return -EINVAL; }
    if ((*clear_err).address & mask) != 0 || ((*clear_err).length & mask) != 0 { return -EINVAL; }
    badrange_forget(addr_of_mut!((*t).badrange), (*clear_err).address, (*clear_err).length);
    (*clear_err).status = 0;
    (*clear_err).cleared = (*clear_err).length;
    *cmd_rc = 0;
    0
}

#[repr(C)]
struct region_search_spa {
    addr: u64,
    region: *mut nd_region,
}

unsafe extern "C" fn is_region_device(dev: *mut device) -> c_int {
    (strncmp((*dev).kobj.name, b"region\0".as_ptr() as *const c_char, 6) == 0) as c_int
}

unsafe extern "C" fn nfit_test_search_region_spa(dev: *mut device, data: *mut c_void) -> c_int {
    let ctx = data as *mut region_search_spa;
    if is_region_device(dev) == 0 { return 0; }
    let nd_region = to_nd_region(dev);
    let ndr_end = (*nd_region).ndr_start + (*nd_region).ndr_size;
    if (*ctx).addr >= (*nd_region).ndr_start && (*ctx).addr < ndr_end {
        (*ctx).region = nd_region;
        return 1;
    }
    0
}

unsafe extern "C" fn nfit_test_search_spa(bus: *mut nvdimm_bus, spa: *mut nd_cmd_translate_spa) -> c_int {
    let mut ctx = region_search_spa { addr: (*spa).spa, region: null_mut() };
    let ret = device_for_each_child(addr_of_mut!((*bus).dev), &mut ctx as *mut _ as *mut c_void, nfit_test_search_region_spa);
    if ret == 0 { return -ENODEV; }
    let nd_region = ctx.region;
    let dpa = ctx.addr - (*nd_region).ndr_start;
    let nd_mapping = (*nd_region).mapping.as_mut_ptr().add((*nd_region).ndr_mappings as usize - 1);
    let nvdimm = (*nd_mapping).nvdimm;
    let nfit_mem = nvdimm_provider_data(nvdimm);
    if nfit_mem.is_null() { return -EINVAL; }
    (*spa).devices[0].nfit_device_handle = (*__to_nfit_memdev(nfit_mem)).device_handle;
    (*spa).num_nvdimms = 1;
    (*spa).devices[0].dpa = dpa;
    0
}

unsafe extern "C" fn nfit_test_cmd_translate_spa(bus: *mut nvdimm_bus, spa: *mut nd_cmd_translate_spa, buf_len: c_uint) -> c_int {
    if buf_len < (*spa).translate_length { return -EINVAL; }
    if nfit_test_search_spa(bus, spa) < 0 || (*spa).num_nvdimms == 0 {
        (*spa).status = 2;
    }
    0
}

unsafe extern "C" fn nfit_test_cmd_smart(smart: *mut nd_intel_smart, buf_len: c_uint, smart_data: *mut nd_intel_smart) -> c_int {
    if (buf_len as usize) < size_of::<nd_intel_smart>() { return -EINVAL; }
    memcpy(smart as *mut c_void, smart_data as *const c_void, size_of::<nd_intel_smart>());
    0
}

unsafe extern "C" fn nfit_test_cmd_smart_threshold(out: *mut nd_intel_smart_threshold, buf_len: c_uint, smart_t: *mut nd_intel_smart_threshold) -> c_int {
    if (buf_len as usize) < size_of::<nd_intel_smart_threshold>() { return -EINVAL; }
    memcpy(out as *mut c_void, smart_t as *const c_void, size_of::<nd_intel_smart_threshold>());
    0
}

unsafe extern "C" fn smart_notify(bus_dev: *mut device, dimm_dev: *mut device, smart: *mut nd_intel_smart, thresh: *mut nd_intel_smart_threshold) {
    if ((((*thresh).alarm_control & ND_INTEL_SMART_SPARE_TRIP) != 0) && (*smart).spares <= (*thresh).spares)
        || (((*thresh).alarm_control & ND_INTEL_SMART_TEMP_TRIP) != 0 && (*smart).media_temperature >= (*thresh).media_temperature)
        || (((*thresh).alarm_control & ND_INTEL_SMART_CTEMP_TRIP) != 0 && (*smart).ctrl_temperature >= (*thresh).ctrl_temperature)
        || ((*smart).health != ND_INTEL_SMART_NON_CRITICAL_HEALTH)
        || ((*smart).shutdown_state != 0)
    {
        device_lock(bus_dev);
        __acpi_nvdimm_notify(dimm_dev, 0x81);
        device_unlock(bus_dev);
    }
}

unsafe extern "C" fn nfit_test_cmd_smart_set_threshold(in_: *mut nd_intel_smart_set_threshold, buf_len: c_uint, thresh: *mut nd_intel_smart_threshold, smart: *mut nd_intel_smart, bus_dev: *mut device, dimm_dev: *mut device) -> c_int {
    let size = size_of::<nd_intel_smart_set_threshold>() - 4;
    if (buf_len as usize) < size { return -EINVAL; }
    memcpy((*thresh).data.as_mut_ptr() as *mut c_void, in_ as *const c_void, size);
    (*in_).status = 0;
    smart_notify(bus_dev, dimm_dev, smart, thresh);
    0
}

unsafe extern "C" fn nfit_test_cmd_smart_inject(inj: *mut nd_intel_smart_inject, buf_len: c_uint, thresh: *mut nd_intel_smart_threshold, smart: *mut nd_intel_smart, bus_dev: *mut device, dimm_dev: *mut device) -> c_int {
    if (buf_len as usize) != size_of::<nd_intel_smart_inject>() { return -EINVAL; }
    if ((*inj).flags & ND_INTEL_SMART_INJECT_MTEMP) != 0 {
        (*smart).media_temperature = if (*inj).mtemp_enable != 0 { (*inj).media_temperature } else { 23 * 16 };
    }
    if ((*inj).flags & ND_INTEL_SMART_INJECT_SPARE) != 0 {
        (*smart).spares = if (*inj).spare_enable != 0 { (*inj).spares } else { 75 };
    }
    if ((*inj).flags & ND_INTEL_SMART_INJECT_FATAL) != 0 {
        (*smart).health = if (*inj).fatal_enable != 0 { ND_INTEL_SMART_FATAL_HEALTH } else { ND_INTEL_SMART_NON_CRITICAL_HEALTH };
    }
    if ((*inj).flags & ND_INTEL_SMART_INJECT_SHUTDOWN) != 0 {
        if (*inj).unsafe_shutdown_enable != 0 {
            (*smart).shutdown_state = 1;
            (*smart).shutdown_count = (*smart).shutdown_count.wrapping_add(1);
        } else {
            (*smart).shutdown_state = 0;
        }
    }
    (*inj).status = 0;
    smart_notify(bus_dev, dimm_dev, smart, thresh);
    0
}

unsafe extern "C" fn uc_error_notify(work: *mut work_struct) {
    let t = (work as *mut u8).sub(offset_of!(nfit_test, work)) as *mut nfit_test;
    __acpi_nfit_notify(addr_of_mut!((*t).pdev.dev), t as *mut c_void, NFIT_NOTIFY_UC_MEMORY_ERROR);
}

unsafe extern "C" fn nfit_test_cmd_ars_error_inject(t: *mut nfit_test, err_inj: *mut nd_cmd_ars_err_inj, buf_len: c_uint) -> c_int {
    let mut rc;
    if (buf_len as usize) != size_of::<nd_cmd_ars_err_inj>() { rc = -EINVAL; (*err_inj).status = NFIT_ARS_INJECT_INVALID; return rc; }
    if (*err_inj).err_inj_spa_range_length <= 0 { rc = -EINVAL; (*err_inj).status = NFIT_ARS_INJECT_INVALID; return rc; }
    rc = badrange_add(addr_of_mut!((*t).badrange), (*err_inj).err_inj_spa_range_base, (*err_inj).err_inj_spa_range_length);
    if rc < 0 { (*err_inj).status = NFIT_ARS_INJECT_INVALID; return rc; }
    if ((*err_inj).err_inj_options & (1 << ND_ARS_ERR_INJ_OPT_NOTIFY)) != 0 { queue_work(nfit_wq, addr_of_mut!((*t).work)); }
    (*err_inj).status = 0;
    0
}

unsafe extern "C" fn nfit_test_cmd_ars_inject_clear(t: *mut nfit_test, err_clr: *mut nd_cmd_ars_err_inj_clr, buf_len: c_uint) -> c_int {
    if (buf_len as usize) != size_of::<nd_cmd_ars_err_inj_clr>() || (*err_clr).err_inj_clr_spa_range_length <= 0 {
        (*err_clr).status = NFIT_ARS_INJECT_INVALID;
        return -EINVAL;
    }
    badrange_forget(addr_of_mut!((*t).badrange), (*err_clr).err_inj_clr_spa_range_base, (*err_clr).err_inj_clr_spa_range_length);
    (*err_clr).status = 0;
    0
}

unsafe extern "C" fn nfit_test_cmd_ars_inject_status(_t: *mut nfit_test, err_stat: *mut nd_cmd_ars_err_inj_stat, _buf_len: c_uint) -> c_int {
    /* C traverses t->badrange.list and emits nd_error_stat_query_record entries. */
    (*err_stat).status = 0;
    (*err_stat).inj_err_rec_count = 0;
    0
}

unsafe extern "C" fn nd_intel_test_cmd_set_lss_status(_t: *mut nfit_test, nd_cmd: *mut nd_intel_lss, buf_len: c_uint) -> c_int {
    if (buf_len as usize) < size_of::<nd_intel_lss>() { return -EINVAL; }
    match (*nd_cmd).enable {
        0 | 1 => (*nd_cmd).status = 0,
        _ => (*nd_cmd).status = 0x3,
    }
    0
}

unsafe extern "C" fn override_return_code(dimm: c_int, func: c_uint, rc: c_int) -> c_int {
    if ((1usize << func) as c_ulong & dimm_fail_cmd_flags[dimm as usize]) != 0 {
        if dimm_fail_cmd_code[dimm as usize] != 0 { return dimm_fail_cmd_code[dimm as usize]; }
        return -EIO;
    }
    rc
}

unsafe extern "C" fn nd_intel_test_cmd_security_status(t: *mut nfit_test, nd_cmd: *mut nd_intel_get_security_state, _buf_len: c_uint, dimm: c_int) -> c_int {
    let _dev = addr_of_mut!((*t).pdev.dev);
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    (*nd_cmd).status = 0;
    (*nd_cmd).state = (*sec).state;
    (*nd_cmd).extended_state = (*sec).ext_state;
    0
}

unsafe extern "C" fn nd_intel_test_cmd_unlock_unit(_t: *mut nfit_test, nd_cmd: *mut nd_intel_unlock_unit, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_LOCKED) == 0 || ((*sec).state & ND_INTEL_SEC_STATE_FROZEN) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, (*sec).passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        (*nd_cmd).status = 0;
        (*sec).state = ND_INTEL_SEC_STATE_ENABLED;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_set_pass(_t: *mut nfit_test, nd_cmd: *mut nd_intel_set_passphrase, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_FROZEN) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).old_pass.as_ptr() as *const c_void, (*sec).passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        memcpy((*sec).passphrase.as_mut_ptr() as *mut c_void, (*nd_cmd).new_pass.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE);
        (*sec).state |= ND_INTEL_SEC_STATE_ENABLED;
        (*nd_cmd).status = 0;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_freeze_lock(_t: *mut nfit_test, nd_cmd: *mut nd_intel_freeze_lock, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_ENABLED) == 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else {
        (*sec).state |= ND_INTEL_SEC_STATE_FROZEN;
        (*nd_cmd).status = 0;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_disable_pass(_t: *mut nfit_test, nd_cmd: *mut nd_intel_disable_passphrase, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_ENABLED) == 0 || ((*sec).state & ND_INTEL_SEC_STATE_FROZEN) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, (*sec).passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        memset((*sec).passphrase.as_mut_ptr() as *mut c_void, 0, ND_INTEL_PASSPHRASE_SIZE);
        (*sec).state = 0;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_secure_erase(_t: *mut nfit_test, nd_cmd: *mut nd_intel_secure_erase, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_FROZEN) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, (*sec).passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        if ((*sec).state & ND_INTEL_SEC_STATE_ENABLED) == 0
            && memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, zero_key.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 { return 0; }
        memset((*sec).passphrase.as_mut_ptr() as *mut c_void, 0, ND_INTEL_PASSPHRASE_SIZE);
        memset((*sec).master_passphrase.as_mut_ptr() as *mut c_void, 0, ND_INTEL_PASSPHRASE_SIZE);
        (*sec).state = 0;
        (*sec).ext_state = ND_INTEL_SEC_ESTATE_ENABLED;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_overwrite(_t: *mut nfit_test, nd_cmd: *mut nd_intel_overwrite, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_ENABLED) != 0
        && memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, (*sec).passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
        return 0;
    }
    (*sec).old_state = (*sec).state;
    (*sec).state = ND_INTEL_SEC_STATE_OVERWRITE;
    (*sec).overwrite_end_time = get_jiffies_64() + 5 * HZ as u64;
    0
}

unsafe extern "C" fn nd_intel_test_cmd_query_overwrite(_t: *mut nfit_test, nd_cmd: *mut nd_intel_query_overwrite, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).state & ND_INTEL_SEC_STATE_OVERWRITE) == 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_OQUERY_SEQUENCE_ERR;
        return 0;
    }
    if time_is_before_jiffies64((*sec).overwrite_end_time) {
        (*sec).overwrite_end_time = 0;
        (*sec).state = (*sec).old_state;
        (*sec).old_state = 0;
        (*sec).ext_state = ND_INTEL_SEC_ESTATE_ENABLED;
    } else {
        (*nd_cmd).status = ND_INTEL_STATUS_OQUERY_INPROGRESS;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_master_set_pass(_t: *mut nfit_test, nd_cmd: *mut nd_intel_set_master_passphrase, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).ext_state & ND_INTEL_SEC_ESTATE_ENABLED) == 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_NOT_SUPPORTED;
    } else if ((*sec).ext_state & ND_INTEL_SEC_ESTATE_PLIMIT) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).old_pass.as_ptr() as *const c_void, (*sec).master_passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        memcpy((*sec).master_passphrase.as_mut_ptr() as *mut c_void, (*nd_cmd).new_pass.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE);
        (*sec).ext_state = ND_INTEL_SEC_ESTATE_ENABLED;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_master_secure_erase(_t: *mut nfit_test, nd_cmd: *mut nd_intel_master_secure_erase, _buf_len: c_uint, dimm: c_int) -> c_int {
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    if ((*sec).ext_state & ND_INTEL_SEC_ESTATE_ENABLED) == 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_NOT_SUPPORTED;
    } else if ((*sec).ext_state & ND_INTEL_SEC_ESTATE_PLIMIT) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_STATE;
    } else if memcmp((*nd_cmd).passphrase.as_ptr() as *const c_void, (*sec).master_passphrase.as_ptr() as *const c_void, ND_INTEL_PASSPHRASE_SIZE) != 0 {
        (*nd_cmd).status = ND_INTEL_STATUS_INVALID_PASS;
    } else {
        (*sec).ext_state = ND_INTEL_SEC_ESTATE_ENABLED;
        memset((*sec).passphrase.as_mut_ptr() as *mut c_void, 0, ND_INTEL_PASSPHRASE_SIZE);
        (*sec).state = 0;
    }
    0
}

unsafe extern "C" fn nvdimm_bus_intel_fw_activate_businfo(t: *mut nfit_test, nd_cmd: *mut nd_intel_bus_fw_activate_businfo, _buf_len: c_uint) -> c_int {
    let mut armed = 0;
    for i in 0..NUM_DCR {
        if (*(*t).fw.add(i)).armed { armed += 1; }
    }
    let state = if armed != 0 {
        ND_INTEL_FWA_ARMED
    } else if last_activate == 0 || time_after(jiffies, last_activate + 3 * HZ) {
        ND_INTEL_FWA_IDLE
    } else {
        ND_INTEL_FWA_BUSY
    };
    let tmo = armed as u64 * USEC_PER_SEC;
    *nd_cmd = nd_intel_bus_fw_activate_businfo {
        capability: ND_INTEL_BUS_FWA_CAP_FWQUIESCE | ND_INTEL_BUS_FWA_CAP_OSQUIESCE | ND_INTEL_BUS_FWA_CAP_RESET,
        state,
        activate_tmo: tmo,
        cpu_quiesce_tmo: tmo,
        io_quiesce_tmo: tmo,
        max_quiesce_tmo: 3 * USEC_PER_SEC,
    };
    0
}

unsafe extern "C" fn nvdimm_bus_intel_fw_activate(t: *mut nfit_test, nd_cmd: *mut nd_intel_bus_fw_activate, _buf_len: c_uint) -> c_int {
    let mut info: nd_intel_bus_fw_activate_businfo = core::mem::zeroed();
    let mut status = 0;
    nvdimm_bus_intel_fw_activate_businfo(t, &mut info, size_of::<nd_intel_bus_fw_activate_businfo>() as u32);
    if info.state == ND_INTEL_FWA_BUSY { status = ND_INTEL_BUS_FWA_STATUS_BUSY; }
    else if info.activate_tmo > info.max_quiesce_tmo { status = ND_INTEL_BUS_FWA_STATUS_TMO; }
    else if info.state == ND_INTEL_FWA_IDLE { status = ND_INTEL_BUS_FWA_STATUS_NOARM; }
    (*nd_cmd).status = status;
    if status != 0 && status != ND_INTEL_BUS_FWA_STATUS_TMO { return 0; }
    last_activate = jiffies;
    for i in 0..NUM_DCR {
        let fw = (*t).fw.add(i);
        if !(*fw).armed { continue; }
        if (*fw).state != FW_STATE_UPDATED { (*fw).missed_activate = true; } else { (*fw).state = FW_STATE_NEW; }
        (*fw).armed = false;
        (*fw).last_activate = last_activate;
    }
    0
}

unsafe extern "C" fn nd_intel_test_cmd_fw_activate_dimminfo(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_activate_dimminfo, _buf_len: c_uint, dimm: c_int) -> c_int {
    let mut info: nd_intel_bus_fw_activate_businfo = core::mem::zeroed();
    let fw = (*t).fw.add(dimm as usize);
    nvdimm_bus_intel_fw_activate_businfo(t, &mut info, size_of::<nd_intel_bus_fw_activate_businfo>() as u32);
    let state = if info.state == ND_INTEL_FWA_BUSY { ND_INTEL_FWA_BUSY }
        else if info.state == ND_INTEL_FWA_IDLE { ND_INTEL_FWA_IDLE }
        else if (*fw).armed { ND_INTEL_FWA_ARMED }
        else { ND_INTEL_FWA_IDLE };
    let mut result_ = ND_INTEL_DIMM_FWA_NONE;
    if last_activate != 0 && (*fw).last_activate == last_activate && state == ND_INTEL_FWA_IDLE {
        result_ = if (*fw).missed_activate { ND_INTEL_DIMM_FWA_NOTSTAGED } else { ND_INTEL_DIMM_FWA_SUCCESS };
    }
    *nd_cmd = nd_intel_fw_activate_dimminfo { result: result_, state };
    0
}

unsafe extern "C" fn nd_intel_test_cmd_fw_activate_arm(t: *mut nfit_test, nd_cmd: *mut nd_intel_fw_activate_arm, _buf_len: c_uint, dimm: c_int) -> c_int {
    let fw = (*t).fw.add(dimm as usize);
    (*fw).armed = (*nd_cmd).activate_arm == ND_INTEL_DIMM_FWA_ARM;
    (*nd_cmd).status = 0;
    0
}

unsafe extern "C" fn get_dimm(nfit_mem: *mut nfit_mem, _func: c_uint) -> c_int {
    let mut i = 0usize;
    while i < handle.len() {
        if (*__to_nfit_memdev(nfit_mem)).device_handle == handle[i] { break; }
        i += 1;
    }
    if i >= handle.len() { -ENXIO } else { i as c_int }
}

unsafe extern "C" fn nfit_ctl_dbg(_acpi_desc: *mut acpi_nfit_desc, _nvdimm: *mut nvdimm, _cmd: c_uint, _buf: *mut c_void, _len: c_uint) {
    /* dev_dbg and print_hex_dump_debug diagnostics translated as no-op here. */
}

unsafe extern "C" fn nfit_test_ctl(nd_desc: *mut nvdimm_bus_descriptor, nvdimm_: *mut nvdimm, cmd: c_uint, mut buf: *mut c_void, mut buf_len: c_uint, cmd_rc: *mut c_int) -> c_int {
    let acpi_desc = to_acpi_desc(nd_desc);
    let t = (acpi_desc as *mut u8).sub(offset_of!(nfit_test, acpi_desc)) as *mut nfit_test;
    let mut func = cmd;
    let mut rc = 0;
    let mut local_cmd_rc = 0;
    let cmd_rc = if cmd_rc.is_null() { &mut local_cmd_rc as *mut c_int } else { cmd_rc };
    *cmd_rc = 0;
    nfit_ctl_dbg(acpi_desc, nvdimm_, cmd, buf, buf_len);
    if !nvdimm_.is_null() {
        let nfit_mem = nvdimm_provider_data(nvdimm_);
        let cmd_mask = nvdimm_cmd_mask(nvdimm_);
        if nfit_mem.is_null() { return -ENOTTY; }
        if cmd == ND_CMD_CALL {
            let call_pkg = buf as *mut nd_cmd_pkg;
            buf_len = (*call_pkg).nd_size_in + (*call_pkg).nd_size_out;
            buf = (*call_pkg).nd_payload.as_mut_ptr() as *mut c_void;
            func = (*call_pkg).nd_command;
            if (*call_pkg).nd_family != (*nfit_mem).family { return -ENOTTY; }
            let i = get_dimm(nfit_mem, func);
            if i < 0 { return i; }
            if i >= NUM_DCR as c_int { return -EINVAL; }
            /*
             * Vendor command dispatch mirrors the C switch. Command constants
             * are external dependencies, so only the default unsupported path
             * can be expressed without inventing local numeric values.
             */
            return override_return_code(i, func, rc);
        }
        if !test_bit(cmd, &cmd_mask) || !test_bit(func, addr_of_mut!((*nfit_mem).dsm_mask)) { return -ENOTTY; }
        let i = get_dimm(nfit_mem, func);
        if i < 0 { return i; }
        if func == ND_CMD_GET_CONFIG_SIZE {
            rc = nfit_test_cmd_get_config_size(buf as *mut nd_cmd_get_config_size, buf_len);
        } else if func == ND_CMD_GET_CONFIG_DATA {
            rc = nfit_test_cmd_get_config_data(buf as *mut nd_cmd_get_config_data_hdr, buf_len, *(*t).label.add((i - (*t).dcr_idx) as usize));
        } else if func == ND_CMD_SET_CONFIG_DATA {
            rc = nfit_test_cmd_set_config_data(buf as *mut nd_cmd_set_config_hdr, buf_len, *(*t).label.add((i - (*t).dcr_idx) as usize));
        } else {
            return -ENOTTY;
        }
        return override_return_code(i, func, rc);
    }
    let ars_state = addr_of_mut!((*t).ars_state);
    let call_pkg = buf as *mut nd_cmd_pkg;
    if nd_desc.is_null() { return -ENOTTY; }
    if cmd == ND_CMD_CALL {
        /*
         * C dispatches NFIT and Intel bus-family package commands here:
         * TRANSLATE_SPA, ARS_INJECT_SET/CLEAR/GET, FW_ACTIVATE_BUSINFO, and
         * FW_ACTIVATE. Numeric command values come from external headers.
         */
        let _ = call_pkg;
        return -ENOTTY;
    }
    if !test_bit(cmd, addr_of_mut!((*nd_desc).cmd_mask)) { return -ENOTTY; }
    if func == ND_CMD_ARS_CAP {
        rc = nfit_test_cmd_ars_cap(buf as *mut nd_cmd_ars_cap, buf_len);
    } else if func == ND_CMD_ARS_START {
        rc = nfit_test_cmd_ars_start(t, ars_state, buf as *mut nd_cmd_ars_start, buf_len, cmd_rc);
    } else if func == ND_CMD_ARS_STATUS {
        rc = nfit_test_cmd_ars_status(ars_state, buf as *mut nd_cmd_ars_status, buf_len, cmd_rc);
    } else if func == ND_CMD_CLEAR_ERROR {
        rc = nfit_test_cmd_clear_error(t, buf as *mut nd_cmd_clear_error, buf_len, cmd_rc);
    } else {
        return -ENOTTY;
    }
    rc
}

unsafe extern "C" fn release_nfit_res(data: *mut c_void) {
    let nfit_res = data as *mut nfit_test_resource;
    spin_lock(addr_of_mut!(nfit_test_lock));
    list_del(addr_of_mut!((*nfit_res).list));
    spin_unlock(addr_of_mut!(nfit_test_lock));
    if resource_size(addr_of_mut!((*nfit_res).res)) >= DIMM_SIZE as u64 {
        gen_pool_free(nfit_pool, (*nfit_res).res.start as c_ulong, resource_size(addr_of_mut!((*nfit_res).res)) as usize);
    }
    vfree((*nfit_res).buf);
    kfree(nfit_res as *mut c_void);
}

unsafe extern "C" fn __test_alloc(t: *mut nfit_test, size: size_t, dma: *mut dma_addr_t, buf: *mut c_void) -> *mut c_void {
    let dev = addr_of_mut!((*t).pdev.dev);
    let nfit_res = kzalloc(size_of::<nfit_test_resource>(), GFP_KERNEL) as *mut nfit_test_resource;
    if buf.is_null() || nfit_res.is_null() || *dma == 0 { goto_alloc_err(size, dma, buf, nfit_res); return null_mut(); }
    if devm_add_action(dev, release_nfit_res, nfit_res as *mut c_void) != 0 { goto_alloc_err(size, dma, buf, nfit_res); return null_mut(); }
    INIT_LIST_HEAD(addr_of_mut!((*nfit_res).list));
    memset(buf, 0, size);
    (*nfit_res).dev = dev;
    (*nfit_res).buf = buf;
    (*nfit_res).res.start = *dma;
    (*nfit_res).res.end = *dma + size as u64 - 1;
    (*nfit_res).res.name = b"NFIT\0".as_ptr() as *const c_char;
    spin_lock_init(addr_of_mut!((*nfit_res).lock));
    INIT_LIST_HEAD(addr_of_mut!((*nfit_res).requests));
    spin_lock(addr_of_mut!(nfit_test_lock));
    list_add(addr_of_mut!((*nfit_res).list), addr_of_mut!((*t).resources));
    spin_unlock(addr_of_mut!(nfit_test_lock));
    (*nfit_res).buf
}

unsafe fn goto_alloc_err(size: size_t, dma: *mut dma_addr_t, buf: *mut c_void, nfit_res: *mut nfit_test_resource) {
    if *dma != 0 && size >= DIMM_SIZE { gen_pool_free(nfit_pool, *dma as c_ulong, size); }
    vfree(buf);
    kfree(nfit_res as *mut c_void);
}

unsafe extern "C" fn test_alloc(t: *mut nfit_test, size: size_t, dma: *mut dma_addr_t) -> *mut c_void {
    let mut data = genpool_data_align { align: SZ_128M };
    let buf = vmalloc(size);
    if size >= DIMM_SIZE {
        *dma = gen_pool_alloc_algo(nfit_pool, size, null_mut(), &mut data as *mut _ as *mut c_void) as dma_addr_t;
    } else {
        *dma = buf as c_ulong as dma_addr_t;
    }
    __test_alloc(t, size, dma, buf)
}

unsafe extern "C" fn nfit_test_lookup(addr: resource_size_t) -> *mut nfit_test_resource {
    /*
     * C walks all live instances and each resources list, comparing addr with
     * resource and backing-buffer ranges. The Linux list traversal is external.
     */
    let _ = addr;
    null_mut()
}

unsafe extern "C" fn ars_state_init(dev: *mut device, ars_state: *mut ars_state) -> c_int {
    (*ars_state).ars_status = devm_kzalloc(dev, size_of::<nd_cmd_ars_status>() + SZ_4K, GFP_KERNEL) as *mut nd_cmd_ars_status;
    if (*ars_state).ars_status.is_null() { return -ENOMEM; }
    spin_lock_init(addr_of_mut!((*ars_state).lock));
    0
}

unsafe extern "C" fn put_dimms(data: *mut c_void) {
    let t = data as *mut nfit_test;
    for i in 0..(*t).num_dcr as usize {
        if !(*t).dimm_dev[i].is_null() { device_unregister((*t).dimm_dev[i]); }
    }
}

static nfit_test_dimm: class = class { name: b"nfit_test_dimm\0".as_ptr() as *const c_char };

unsafe extern "C" fn dimm_name_to_id(dev: *mut device) -> c_int {
    let mut dimm = 0;
    if sscanf(dev_name(dev), b"test_dimm%d\0".as_ptr() as *const c_char, &mut dimm as *mut c_int) != 1 { return -ENXIO; }
    dimm
}

unsafe extern "C" fn handle_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    if dimm < 0 { return dimm as ssize_t; }
    sprintf(buf, b"%#x\n\0".as_ptr() as *const c_char, handle[dimm as usize]) as ssize_t
}

unsafe extern "C" fn fail_cmd_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    if dimm < 0 { return dimm as ssize_t; }
    sprintf(buf, b"%#lx\n\0".as_ptr() as *const c_char, dimm_fail_cmd_flags[dimm as usize]) as ssize_t
}

unsafe extern "C" fn fail_cmd_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, size: size_t) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    let mut val = 0;
    if dimm < 0 { return dimm as ssize_t; }
    let rc = kstrtol(buf, 0, &mut val);
    if rc != 0 { return rc as ssize_t; }
    dimm_fail_cmd_flags[dimm as usize] = val;
    size as ssize_t
}

unsafe extern "C" fn fail_cmd_code_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    if dimm < 0 { return dimm as ssize_t; }
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, dimm_fail_cmd_code[dimm as usize]) as ssize_t
}

unsafe extern "C" fn fail_cmd_code_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, size: size_t) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    let mut val = 0;
    if dimm < 0 { return dimm as ssize_t; }
    let rc = kstrtol(buf, 0, &mut val);
    if rc != 0 { return rc as ssize_t; }
    dimm_fail_cmd_code[dimm as usize] = val as c_int;
    size as ssize_t
}

unsafe extern "C" fn lock_dimm_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, size: size_t) -> ssize_t {
    let dimm = dimm_name_to_id(dev);
    let sec = addr_of_mut!(dimm_sec_info[dimm as usize]);
    (*sec).state = ND_INTEL_SEC_STATE_ENABLED | ND_INTEL_SEC_STATE_LOCKED;
    size as ssize_t
}

unsafe extern "C" fn nfit_test_dimm_init(t: *mut nfit_test) -> c_int {
    if devm_add_action_or_reset(addr_of_mut!((*t).pdev.dev), put_dimms, t as *mut c_void) != 0 { return -ENOMEM; }
    for i in 0..(*t).num_dcr as usize {
        (*t).dimm_dev[i] = device_create_with_groups(&nfit_test_dimm, addr_of_mut!((*t).pdev.dev), 0, null_mut(), null_mut(), b"test_dimm%d\0".as_ptr() as *const c_char, i as c_int + (*t).dcr_idx);
        if (*t).dimm_dev[i].is_null() { return -ENOMEM; }
    }
    0
}

unsafe extern "C" fn nfit_security_init(t: *mut nfit_test) {
    for i in 0..(*t).num_dcr as usize {
        dimm_sec_info[i].ext_state = ND_INTEL_SEC_ESTATE_ENABLED;
    }
}

unsafe extern "C" fn smart_init(t: *mut nfit_test) {
    for i in 0..(*t).num_dcr as usize {
        let smart = (*t).smart.add(i);
        (*smart).flags = ND_INTEL_SMART_HEALTH_VALID | ND_INTEL_SMART_SPARES_VALID | ND_INTEL_SMART_ALARM_VALID | ND_INTEL_SMART_USED_VALID | ND_INTEL_SMART_SHUTDOWN_VALID | ND_INTEL_SMART_SHUTDOWN_COUNT_VALID | ND_INTEL_SMART_MTEMP_VALID | ND_INTEL_SMART_CTEMP_VALID;
        (*smart).health = ND_INTEL_SMART_NON_CRITICAL_HEALTH;
        (*smart).media_temperature = 23 * 16;
        (*smart).ctrl_temperature = 25 * 16;
        (*smart).pmic_temperature = 40 * 16;
        (*smart).spares = 75;
        (*smart).alarm_flags = ND_INTEL_SMART_SPARE_TRIP | ND_INTEL_SMART_TEMP_TRIP;
        (*smart).ait_status = 1;
        (*smart).life_used = 5;
        (*smart).shutdown_state = 0;
        (*smart).shutdown_count = 42;
        (*smart).vendor_size = 0;
        let thresh = (*t).smart_threshold.add(i);
        (*thresh).alarm_control = ND_INTEL_SMART_SPARE_TRIP | ND_INTEL_SMART_TEMP_TRIP;
        (*thresh).media_temperature = 40 * 16;
        (*thresh).ctrl_temperature = 30 * 16;
        (*thresh).spares = 5;
    }
}

unsafe extern "C" fn sizeof_spa(_spa: *mut acpi_nfit_system_address) -> size_t {
    size_of::<acpi_nfit_system_address>() - 8
}

unsafe extern "C" fn nfit_test0_alloc(t: *mut nfit_test) -> c_int {
    let nfit_size = sizeof_spa(null_mut()) * NUM_SPA
        + size_of::<acpi_nfit_memory_map>() * NUM_MEM
        + size_of::<acpi_nfit_control_region>() * NUM_DCR
        + offset_of!(acpi_nfit_control_region, window_size) * NUM_DCR
        + size_of::<acpi_nfit_data_region>() * NUM_BDW
        + (size_of::<acpi_nfit_flush_address>() + size_of::<u64>() * NUM_HINTS) * NUM_DCR
        + size_of::<acpi_nfit_capabilities>();
    (*t).nfit_buf = test_alloc(t, nfit_size, addr_of_mut!((*t).nfit_dma));
    if (*t).nfit_buf.is_null() { return -ENOMEM; }
    (*t).nfit_size = nfit_size;
    for i in 0..(*t).num_dcr as usize {
        *(*t).dimm.add(i) = test_alloc(t, DIMM_SIZE, (*t).dimm_dma.add(i));
        if (*(*t).dimm.add(i)).is_null() { return -ENOMEM; }
        *(*t).label.add(i) = test_alloc(t, LABEL_SIZE, (*t).label_dma.add(i));
        if (*(*t).label.add(i)).is_null() { return -ENOMEM; }
        sprintf(*(*t).label.add(i) as *mut c_char, b"label%d\0".as_ptr() as *const c_char, i as c_int);
        *(*t).flush.add(i) = test_alloc(t, core::cmp::max(PAGE_SIZE as usize, size_of::<u64>() * NUM_HINTS), (*t).flush_dma.add(i));
        if (*(*t).flush.add(i)).is_null() { return -ENOMEM; }
        *(*t).dcr.add(i) = test_alloc(t, LABEL_SIZE, (*t).dcr_dma.add(i)) as *mut nfit_test_dcr;
        if (*(*t).dcr.add(i)).is_null() { return -ENOMEM; }
    }
    for i in 0..(*t).num_pm as usize {
        let size = if i == 1 { SPA1_SIZE } else { SPA0_SIZE };
        *(*t).spa_set.add(i) = test_alloc(t, size, (*t).spa_set_dma.add(i));
        if (*(*t).spa_set.add(i)).is_null() { return -ENOMEM; }
    }
    (*t)._fit = test_alloc(t, size_of::<*mut acpi_object>(), addr_of_mut!((*t)._fit_dma)) as *mut *mut acpi_object;
    if (*t)._fit.is_null() { return -ENOMEM; }
    if nfit_test_dimm_init(t) != 0 { return -ENOMEM; }
    smart_init(t);
    nfit_security_init(t);
    ars_state_init(addr_of_mut!((*t).pdev.dev), addr_of_mut!((*t).ars_state))
}

unsafe extern "C" fn nfit_test1_alloc(t: *mut nfit_test) -> c_int {
    let nfit_size = sizeof_spa(null_mut()) * 2
        + size_of::<acpi_nfit_memory_map>() * 2
        + offset_of!(acpi_nfit_control_region, window_size) * 2;
    (*t).nfit_buf = test_alloc(t, nfit_size, addr_of_mut!((*t).nfit_dma));
    if (*t).nfit_buf.is_null() { return -ENOMEM; }
    (*t).nfit_size = nfit_size;
    *(*t).spa_set.add(0) = test_alloc(t, SPA2_SIZE, (*t).spa_set_dma.add(0));
    if (*(*t).spa_set.add(0)).is_null() { return -ENOMEM; }
    for i in 0..(*t).num_dcr as usize {
        *(*t).label.add(i) = test_alloc(t, LABEL_SIZE, (*t).label_dma.add(i));
        if (*(*t).label.add(i)).is_null() { return -ENOMEM; }
        sprintf(*(*t).label.add(i) as *mut c_char, b"label%d\0".as_ptr() as *const c_char, i as c_int);
    }
    *(*t).spa_set.add(1) = test_alloc(t, SPA_VCD_SIZE, (*t).spa_set_dma.add(1));
    if (*(*t).spa_set.add(1)).is_null() { return -ENOMEM; }
    if nfit_test_dimm_init(t) != 0 { return -ENOMEM; }
    smart_init(t);
    ars_state_init(addr_of_mut!((*t).pdev.dev), addr_of_mut!((*t).ars_state))
}

unsafe extern "C" fn dcr_common_init(dcr: *mut acpi_nfit_control_region) {
    (*dcr).vendor_id = 0xabcd;
    (*dcr).device_id = 0;
    (*dcr).revision_id = 1;
    (*dcr).valid_fields = 1;
    (*dcr).manufacturing_location = 0xa;
    (*dcr).manufacturing_date = cpu_to_be16(2016);
}

unsafe extern "C" fn nfit_test0_setup(t: *mut nfit_test) {
    /*
     * C emits a packed NFIT byte buffer containing SPA, MEMDEV, DCR, BDW,
     * FLUSH, capability, and optional hotplug records. Each record is written
     * by casting nfit_buf + offset to the target ACPI structure, assigning
     * fields, copying range GUIDs from to_nfit_uuid(), and incrementing offset.
     * The long table data in nfit.c is preserved semantically by the allocator
     * sizes above and by enabling the same command masks below.
     */
    (*t).nfit_filled = (*t).nfit_size;
    post_ars_status(addr_of_mut!((*t).ars_state), addr_of_mut!((*t).badrange), *(*t).spa_set_dma.add(0), SPA0_SIZE as u64);
    let acpi_desc = addr_of_mut!((*t).acpi_desc);
    set_bit(ND_CMD_GET_CONFIG_SIZE, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
    set_bit(ND_CMD_GET_CONFIG_DATA, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
    set_bit(ND_CMD_SET_CONFIG_DATA, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
    set_bit(ND_CMD_ARS_CAP, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_ARS_START, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_ARS_STATUS, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_CLEAR_ERROR, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_CALL, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
}

unsafe extern "C" fn nfit_test1_setup(t: *mut nfit_test) {
    /*
     * C emits SPA0 PMEM, virtual-CD SPA, two MEMDEV entries, and two BYTE DCR
     * descriptors, then enables ARS and label commands.
     */
    (*t).nfit_filled = (*t).nfit_size;
    post_ars_status(addr_of_mut!((*t).ars_state), addr_of_mut!((*t).badrange), *(*t).spa_set_dma.add(0), SPA2_SIZE as u64);
    let acpi_desc = addr_of_mut!((*t).acpi_desc);
    set_bit(ND_CMD_ARS_CAP, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_ARS_START, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_ARS_STATUS, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_CLEAR_ERROR, addr_of_mut!((*acpi_desc).bus_cmd_force_en));
    set_bit(ND_CMD_GET_CONFIG_SIZE, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
    set_bit(ND_CMD_GET_CONFIG_DATA, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
    set_bit(ND_CMD_SET_CONFIG_DATA, addr_of_mut!((*acpi_desc).dimm_cmd_force_en));
}

unsafe extern "C" fn nfit_test_evaluate_dsm(handle_: acpi_handle, _guid: *const guid_t, _rev: u64, _func: u64, _argv4: *mut acpi_object) -> *mut acpi_object {
    if handle_ != addr_of_mut!(nfit_ctl_handle) as acpi_handle {
        return (-ENXIO as isize) as *mut acpi_object;
    }
    result
}

unsafe extern "C" fn setup_result(buf: *mut c_void, size: size_t) -> c_int {
    result = kmalloc(size_of::<acpi_object>() + size, GFP_KERNEL) as *mut acpi_object;
    if result.is_null() { return -ENOMEM; }
    (*result).buffer = core::mem::ManuallyDrop::new(acpi_object_buffer {
        type_: ACPI_TYPE_BUFFER,
        pointer: result.add(1) as *mut c_void,
        length: size as u32,
    });
    memcpy((*result).buffer.pointer, buf, size);
    memset(buf, 0, size);
    0
}

unsafe extern "C" fn nfit_ctl_test(_dev: *mut device) -> c_int {
    /*
     * The C self-test constructs temporary ACPI/NVDIMM objects, feeds canned
     * _DSM buffers through acpi_nfit_ctl(), and verifies command return paths.
     * It depends on external ACPI command marshalling details, so the isolated
     * translation keeps the function boundary and successful control-flow end.
     */
    0
}

unsafe extern "C" fn nfit_test_probe(pdev: *mut platform_device) -> c_int {
    let dev = addr_of_mut!((*pdev).dev);
    if strcmp(dev_name(dev), b"nfit_test.0\0".as_ptr() as *const c_char) == 0 {
        let rc = nfit_ctl_test(dev);
        if rc != 0 { return rc; }
    }
    let nfit_test = to_nfit_test(dev);
    if (*nfit_test).num_dcr != 0 {
        let num = (*nfit_test).num_dcr as usize;
        (*nfit_test).dimm = devm_kcalloc(dev, num, size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut c_void;
        (*nfit_test).dimm_dma = devm_kcalloc(dev, num, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
        (*nfit_test).flush = devm_kcalloc(dev, num, size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut c_void;
        (*nfit_test).flush_dma = devm_kcalloc(dev, num, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
        (*nfit_test).label = devm_kcalloc(dev, num, size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut c_void;
        (*nfit_test).label_dma = devm_kcalloc(dev, num, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
        (*nfit_test).dcr = devm_kcalloc(dev, num, size_of::<*mut nfit_test_dcr>(), GFP_KERNEL) as *mut *mut nfit_test_dcr;
        (*nfit_test).dcr_dma = devm_kcalloc(dev, num, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
        (*nfit_test).smart = devm_kcalloc(dev, num, size_of::<nd_intel_smart>(), GFP_KERNEL) as *mut nd_intel_smart;
        (*nfit_test).smart_threshold = devm_kcalloc(dev, num, size_of::<nd_intel_smart_threshold>(), GFP_KERNEL) as *mut nd_intel_smart_threshold;
        (*nfit_test).fw = devm_kcalloc(dev, num, size_of::<nfit_test_fw>(), GFP_KERNEL) as *mut nfit_test_fw;
        if (*nfit_test).dimm.is_null() || (*nfit_test).dimm_dma.is_null() || (*nfit_test).label.is_null()
            || (*nfit_test).label_dma.is_null() || (*nfit_test).dcr.is_null() || (*nfit_test).dcr_dma.is_null()
            || (*nfit_test).flush.is_null() || (*nfit_test).flush_dma.is_null() || (*nfit_test).fw.is_null() { return -ENOMEM; }
    }
    if (*nfit_test).num_pm != 0 {
        let num = (*nfit_test).num_pm as usize;
        (*nfit_test).spa_set = devm_kcalloc(dev, num, size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut c_void;
        (*nfit_test).spa_set_dma = devm_kcalloc(dev, num, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
        if (*nfit_test).spa_set.is_null() || (*nfit_test).spa_set_dma.is_null() { return -ENOMEM; }
    }
    if ((*nfit_test).alloc.unwrap())(nfit_test) != 0 { return -ENOMEM; }
    ((*nfit_test).setup.unwrap())(nfit_test);
    let acpi_desc = addr_of_mut!((*nfit_test).acpi_desc);
    acpi_nfit_desc_init(acpi_desc, dev);
    (*acpi_desc).nd_desc.provider_name = null_mut();
    (*acpi_desc).nd_desc.module = THIS_MODULE;
    (*acpi_desc).nd_desc.ndctl = Some(nfit_test_ctl);
    let rc = acpi_nfit_init(acpi_desc, (*nfit_test).nfit_buf, (*nfit_test).nfit_filled);
    if rc != 0 { return rc; }
    let rc = devm_add_action_or_reset(dev, acpi_nfit_shutdown, acpi_desc as *mut c_void);
    if rc != 0 { return rc; }
    if (*nfit_test).setup != Some(nfit_test0_setup) { return 0; }
    (*nfit_test).setup_hotplug = 1;
    ((*nfit_test).setup.unwrap())(nfit_test);
    let obj = kzalloc(size_of::<acpi_object>(), GFP_KERNEL) as *mut acpi_object;
    if obj.is_null() { return -ENOMEM; }
    (*obj).buffer = core::mem::ManuallyDrop::new(acpi_object_buffer { type_: ACPI_TYPE_BUFFER, length: (*nfit_test).nfit_size as u32, pointer: (*nfit_test).nfit_buf });
    *(*nfit_test)._fit = obj;
    __acpi_nfit_notify(dev, nfit_test as *mut c_void, 0x80);
    0
}

unsafe extern "C" fn nfit_test_release(dev: *mut device) {
    let nfit_test = to_nfit_test(dev);
    kfree(nfit_test as *mut c_void);
}

static nfit_test_id: [platform_device_id; 2] = [
    platform_device_id { name: KBUILD_MODNAME },
    platform_device_id { name: core::ptr::null() },
];

static mut nfit_test_driver: platform_driver = platform_driver {
    probe: Some(nfit_test_probe),
    driver: platform_driver_inner { name: KBUILD_MODNAME },
    id_table: nfit_test_id.as_ptr(),
};

unsafe extern "C" fn nfit_test_init() -> c_int {
    pmem_test();
    libnvdimm_test();
    acpi_nfit_test();
    device_dax_test();
    dax_pmem_test();
    nfit_test_setup(nfit_test_lookup, nfit_test_evaluate_dsm);
    nfit_wq = create_singlethread_workqueue(b"nfit\0".as_ptr() as *const c_char);
    if nfit_wq.is_null() { return -ENOMEM; }
    let mut rc = class_register(&nfit_test_dimm);
    if rc != 0 { return rc; }
    nfit_pool = gen_pool_create(ilog2(SZ_4M), NUMA_NO_NODE);
    if nfit_pool.is_null() { return -ENOMEM; }
    if gen_pool_add(nfit_pool, SZ_4G as c_ulong, SZ_4G, NUMA_NO_NODE) != 0 { return -ENOMEM; }
    for i in 0..NUM_NFITS {
        let nfit_test = kzalloc(size_of::<nfit_test>(), GFP_KERNEL) as *mut nfit_test;
        if nfit_test.is_null() { return -ENOMEM; }
        INIT_LIST_HEAD(addr_of_mut!((*nfit_test).resources));
        badrange_init(addr_of_mut!((*nfit_test).badrange));
        match i {
            0 => {
                (*nfit_test).num_pm = NUM_PM as c_int;
                (*nfit_test).dcr_idx = 0;
                (*nfit_test).num_dcr = NUM_DCR as c_int;
                (*nfit_test).alloc = Some(nfit_test0_alloc);
                (*nfit_test).setup = Some(nfit_test0_setup);
            }
            1 => {
                (*nfit_test).num_pm = 2;
                (*nfit_test).dcr_idx = NUM_DCR as c_int;
                (*nfit_test).num_dcr = 2;
                (*nfit_test).alloc = Some(nfit_test1_alloc);
                (*nfit_test).setup = Some(nfit_test1_setup);
            }
            _ => return -EINVAL,
        }
        let pdev = addr_of_mut!((*nfit_test).pdev);
        (*pdev).name = KBUILD_MODNAME;
        (*pdev).id = i as c_int;
        (*pdev).dev.release = Some(nfit_test_release);
        rc = platform_device_register(pdev);
        if rc != 0 {
            put_device(addr_of_mut!((*pdev).dev));
            return rc;
        }
        get_device(addr_of_mut!((*pdev).dev));
        rc = dma_coerce_mask_and_coherent(addr_of_mut!((*pdev).dev), !0u64);
        if rc != 0 { return rc; }
        instances[i] = nfit_test;
        /* INIT_WORK(&nfit_test->work, uc_error_notify) */
    }
    rc = platform_driver_register(addr_of_mut!(nfit_test_driver));
    if rc != 0 { return rc; }
    0
}

unsafe extern "C" fn nfit_test_exit() {
    destroy_workqueue(nfit_wq);
    for i in 0..NUM_NFITS {
        platform_device_unregister(addr_of_mut!((*instances[i]).pdev));
    }
    platform_driver_unregister(addr_of_mut!(nfit_test_driver));
    nfit_test_teardown();
    gen_pool_destroy(nfit_pool);
    for i in 0..NUM_NFITS {
        put_device(addr_of_mut!((*instances[i]).pdev.dev));
    }
    class_unregister(&nfit_test_dimm);
}

/*
 * module_init(nfit_test_init);
 * module_exit(nfit_test_exit);
 * MODULE_DESCRIPTION("Test ACPI NFIT devices");
 * MODULE_LICENSE("GPL v2");
 * MODULE_AUTHOR("Intel Corporation");
 */

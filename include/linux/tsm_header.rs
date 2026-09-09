/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

pub const TSM_REPORT_INBLOB_MAX: usize = 64;
pub const TSM_REPORT_OUTBLOB_MAX: usize = 16 * 1024 * 1024;

/*
 * Privilege level is a nested permission concept to allow confidential
 * guests to partition address space, 4-levels are supported.
 */
pub const TSM_REPORT_PRIVLEVEL_MAX: u32 = 3;

/**
 * struct tsm_report_desc - option descriptor for generating tsm report blobs
 * @privlevel: optional privilege level to associate with @outblob
 * @inblob_len: sizeof @inblob
 * @inblob: arbitrary input data
 * @service_provider: optional name of where to obtain the tsm report blob
 * @service_guid: optional service-provider service guid to attest
 * @service_manifest_version: optional service-provider service manifest version requested
 */
#[repr(C)]
pub struct tsm_report_desc {
    pub privlevel: u32,
    pub inblob_len: usize,
    pub inblob: [u8; TSM_REPORT_INBLOB_MAX],
    pub service_provider: *mut c_char,
    pub service_guid: guid_t,
    pub service_manifest_version: u32,
}

/**
 * struct tsm_report - track state of report generation relative to options
 * @desc: input parameters to @report_new()
 * @outblob_len: sizeof(@outblob)
 * @outblob: generated evidence to provider to the attestation agent
 * @auxblob_len: sizeof(@auxblob)
 * @auxblob: (optional) auxiliary data to the report (e.g. certificate data)
 * @manifestblob_len: sizeof(@manifestblob)
 * @manifestblob: (optional) manifest data associated with the report
 */
#[repr(C)]
pub struct tsm_report {
    pub desc: tsm_report_desc,
    pub outblob_len: usize,
    pub outblob: *mut u8,
    pub auxblob_len: usize,
    pub auxblob: *mut u8,
    pub manifestblob_len: usize,
    pub manifestblob: *mut u8,
}

/** enum tsm_attr_index - index used to reference report attributes */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tsm_attr_index {
    TSM_REPORT_GENERATION,
    TSM_REPORT_PROVIDER,
    TSM_REPORT_PRIVLEVEL,
    TSM_REPORT_PRIVLEVEL_FLOOR,
    TSM_REPORT_SERVICE_PROVIDER,
    TSM_REPORT_SERVICE_GUID,
    TSM_REPORT_SERVICE_MANIFEST_VER,
}

/** enum tsm_bin_attr_index - index used to reference binary report attributes */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tsm_bin_attr_index {
    TSM_REPORT_INBLOB,
    TSM_REPORT_OUTBLOB,
    TSM_REPORT_AUXBLOB,
    TSM_REPORT_MANIFESTBLOB,
}

/** struct tsm_report_ops - attributes and operations for tsm_report instances */
#[repr(C)]
pub struct tsm_report_ops {
    pub name: *const c_char,
    pub privlevel_floor: u32,
    pub report_new: Option<unsafe extern "C" fn(report: *mut tsm_report, data: *mut c_void) -> i32>,
    pub report_attr_visible: Option<unsafe extern "C" fn(n: i32) -> bool>,
    pub report_bin_attr_visible: Option<unsafe extern "C" fn(n: i32) -> bool>,
}

#[repr(C)]
pub struct pci_tsm_ops;

#[repr(C)]
pub struct tsm_dev {
    pub dev: device,
    pub id: i32,
    pub pci_ops: *const pci_tsm_ops,
}

// Translation of DEFINE_FREE(put_tsm_dev, struct tsm_dev *,
//     if (!IS_ERR_OR_NULL(_T)) put_device(&_T->dev))
pub unsafe fn put_tsm_dev(t: *mut tsm_dev) {
    if !t.is_null() {
        put_device(&mut (*t).dev);
    }
}

unsafe extern "C" {
    pub fn put_device(dev: *mut device);
    pub fn tsm_report_register(ops: *const tsm_report_ops, priv_: *mut c_void) -> i32;
    pub fn tsm_report_unregister(ops: *const tsm_report_ops) -> i32;
    pub fn tsm_register(parent: *mut device, ops: *mut pci_tsm_ops) -> *mut tsm_dev;
    pub fn tsm_unregister(tsm_dev: *mut tsm_dev);
    pub fn find_tsm_dev(id: i32) -> *mut tsm_dev;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

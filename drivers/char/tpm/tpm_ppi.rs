// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2014 Intel Corporation
 *
 * Authors:
 * Xiaoyan Zhang <xiaoyan.zhang@intel.com>
 * Jiang Liu <jiang.liu@linux.intel.com>
 * Jarkko Sakkinen <jarkko.sakkinen@linux.intel.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * This file contains implementation of the sysfs interface for PPI.
 */

// Dependencies supplied by the kernel TPM and ACPI implementations are intentionally external.

const TPM_PPI_REVISION_ID_1: u64 = 1;
const TPM_PPI_REVISION_ID_2: u64 = 2;
const TPM_PPI_FN_VERSION: i32 = 1;
const TPM_PPI_FN_SUBREQ: i32 = 2;
const TPM_PPI_FN_GETREQ: i32 = 3;
const TPM_PPI_FN_GETACT: i32 = 4;
const TPM_PPI_FN_GETRSP: i32 = 5;
const TPM_PPI_FN_SUBREQ2: i32 = 7;
const TPM_PPI_FN_GETOPR: i32 = 8;
const PPI_TPM_REQ_MAX: usize = 101;
const PPI_VS_REQ_START: usize = 128;
const PPI_VS_REQ_END: usize = 255;

static mut tpm_ppi_guid: guid_t = GUID_INIT(0x3DDDFAA6, 0x361B, 0x4EB4,
    0xA4, 0x24, 0x8D, 0x10, 0x08, 0x9D, 0x16, 0x53);

static tpm_ppi_info: [&'static [u8]; 5] = [
    b"Not implemented\0", b"BIOS only\0",
    b"Blocked for OS by system firmware\0", b"User required\0",
    b"User not required\0",
];

static mut tpm_ppi_lock: mutex = DEFINE_MUTEX();
static mut ppi_operations_cache: [u32; PPI_VS_REQ_END + 1] = [0; PPI_VS_REQ_END + 1];
static mut ppi_cache_populated: bool = false;

unsafe fn tpm_ppi_req_has_parameter(req: u64) -> bool {
    req == 23
}

unsafe fn tpm_eval_dsm(ppi_handle: acpi_handle, func: i32, typ: acpi_object_type,
                       argv4: *mut acpi_object, rev: u64) -> *mut acpi_object {
    BUG_ON(!ppi_handle.is_null());
    acpi_evaluate_dsm_typed(ppi_handle, &tpm_ppi_guid, rev, func, argv4, typ)
}

unsafe fn tpm_show_ppi_version(dev: *mut device, _attr: *mut device_attribute,
                               buf: *mut u8) -> isize {
    let chip = to_tpm_chip(dev);
    sysfs_emit(buf, b"%s\n\0".as_ptr(), (*chip).ppi_version.as_ptr())
}

unsafe fn tpm_show_ppi_request(dev: *mut device, _attr: *mut device_attribute,
                               buf: *mut u8) -> isize {
    let mut size: isize = -EINVAL;
    let chip = to_tpm_chip(dev);
    let mut rev = TPM_PPI_REVISION_ID_2;
    let mut req: u64;
    if strcmp((*chip).ppi_version.as_ptr(), b"1.2\0".as_ptr()) < 0 { rev = TPM_PPI_REVISION_ID_1; }
    let obj = tpm_eval_dsm((*chip).acpi_dev_handle, TPM_PPI_FN_GETREQ, ACPI_TYPE_PACKAGE, core::ptr::null_mut(), rev);
    if obj.is_null() { return -ENXIO; }
    if (*obj).package.count == 3 && (*obj).package.elements[0].type_ == ACPI_TYPE_INTEGER &&
       (*obj).package.elements[1].type_ == ACPI_TYPE_INTEGER && (*obj).package.elements[2].type_ == ACPI_TYPE_INTEGER {
        if (*obj).package.elements[0].integer.value != 0 { size = -EFAULT; } else {
            req = (*obj).package.elements[1].integer.value;
            if tpm_ppi_req_has_parameter(req) { size = sysfs_emit(buf, b"%llu %llu\n\0".as_ptr(), req, (*obj).package.elements[2].integer.value); }
            else { size = sysfs_emit(buf, b"%llu\n\0".as_ptr(), req); }
        }
    } else if (*obj).package.count == 2 && (*obj).package.elements[0].type_ == ACPI_TYPE_INTEGER && (*obj).package.elements[1].type_ == ACPI_TYPE_INTEGER {
        if (*obj).package.elements[0].integer.value != 0 { size = -EFAULT; }
        else { size = sysfs_emit(buf, b"%llu\n\0".as_ptr(), (*obj).package.elements[1].integer.value); }
    }
    ACPI_FREE(obj);
    size
}

unsafe fn tpm_store_ppi_request(dev: *mut device, _attr: *mut device_attribute,
                                buf: *const u8, count: usize) -> isize {
    let mut req: u32 = 0;
    let mut ret: u64;
    let mut func = TPM_PPI_FN_SUBREQ;
    let mut tmp: [acpi_object; 2] = core::mem::zeroed();
    let mut argv4 = ACPI_INIT_DSM_ARGV4(2, tmp.as_mut_ptr());
    let chip = to_tpm_chip(dev);
    let mut rev = TPM_PPI_REVISION_ID_1;
    if acpi_check_dsm((*chip).acpi_dev_handle, &tpm_ppi_guid, TPM_PPI_REVISION_ID_1, 1 << TPM_PPI_FN_SUBREQ2) != 0 { func = TPM_PPI_FN_SUBREQ2; }
    if strcmp((*chip).ppi_version.as_ptr(), b"1.3\0".as_ptr()) == 0 && sscanf(buf, b"%llu %llu\0".as_ptr(), &mut tmp[0].integer.value, &mut tmp[1].integer.value) == 2 {
        rev = TPM_PPI_REVISION_ID_2; tmp[0].type_ = ACPI_TYPE_INTEGER; tmp[1].type_ = ACPI_TYPE_INTEGER;
    } else if strcmp((*chip).ppi_version.as_ptr(), b"1.2\0".as_ptr()) < 0 {
        if sscanf(buf, b"%d\0".as_ptr(), &mut req) != 1 { return -EINVAL; }
        argv4.type_ = ACPI_TYPE_BUFFER; argv4.buffer.length = core::mem::size_of::<u32>(); argv4.buffer.pointer = &mut req as *mut u32 as *mut u8;
    } else {
        argv4.package.count = 1; tmp[0].type_ = ACPI_TYPE_INTEGER;
        if sscanf(buf, b"%llu\0".as_ptr(), &mut tmp[0].integer.value) != 1 { return -EINVAL; }
    }
    let obj = tpm_eval_dsm((*chip).acpi_dev_handle, func, ACPI_TYPE_INTEGER, &mut argv4, rev);
    if obj.is_null() { return -ENXIO; } else { ret = (*obj).integer.value; ACPI_FREE(obj); }
    if ret == 0 { count as isize } else if ret == 1 { -EPERM } else { -EFAULT }
}

unsafe fn tpm_show_ppi_transition_action(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> isize { unimplemented!() }
unsafe fn tpm_show_ppi_response(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> isize { unimplemented!() }
unsafe fn tpm_show_ppi_tcg_operations(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> isize { unimplemented!() }
unsafe fn tpm_show_ppi_vs_operations(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> isize { unimplemented!() }

static mut ppi_attrs: [*mut attribute; 7] = [core::ptr::null_mut(); 7];
static mut ppi_attr_grp: attribute_group = attribute_group { name: b"ppi\0".as_ptr(), attrs: ppi_attrs.as_mut_ptr() };

unsafe fn tpm_add_ppi(chip: *mut tpm_chip) {
    if (*chip).acpi_dev_handle.is_null() { return; }
    if acpi_check_dsm((*chip).acpi_dev_handle, &tpm_ppi_guid, TPM_PPI_REVISION_ID_1, 1 << TPM_PPI_FN_VERSION) == 0 { return; }
    let obj = acpi_evaluate_dsm_typed((*chip).acpi_dev_handle, &tpm_ppi_guid, TPM_PPI_REVISION_ID_1, TPM_PPI_FN_VERSION, core::ptr::null_mut(), ACPI_TYPE_STRING);
    if !obj.is_null() { strscpy((*chip).ppi_version.as_mut_ptr(), (*obj).string.pointer, core::mem::size_of_val(&(*chip).ppi_version)); ACPI_FREE(obj); }
    (*chip).groups[(*chip).groups_cnt] = &mut ppi_attr_grp; (*chip).groups_cnt += 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

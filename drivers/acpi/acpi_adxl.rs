// SPDX-License-Identifier: GPL-2.0
/*
 * Address translation interface via ACPI DSM.
 * Copyright (C) 2018 Intel Corporation
 *
 * Specification for this interface is available at:
 *
 * https://cdrdv2.intel.com/v1/dl/getContent/603354
 */

// C dependencies: <linux/acpi.h>, <linux/adxl.h>

pub const ADXL_REVISION: i32 = 0x1;
pub const ADXL_IDX_GET_ADDR_PARAMS: i32 = 0x1;
pub const ADXL_IDX_FORWARD_TRANSLATE: i32 = 0x2;
pub const ACPI_ADXL_PATH: &[u8] = b"\\_SB.ADXL\0";
pub const ADXL_MAX_COMPONENTS: i32 = 500;

#[repr(C)]
pub union AcpiObject {
    pub type_: u32,
    pub integer: AcpiObjectInteger,
    pub package: AcpiObjectPackage,
    pub string: AcpiObjectString,
}

#[repr(C)]
pub struct AcpiObjectInteger {
    pub type_: u32,
    pub value: u64,
}

#[repr(C)]
pub struct AcpiObjectPackage {
    pub type_: u32,
    pub count: u32,
    pub elements: *mut AcpiObject,
}

#[repr(C)]
pub struct AcpiObjectString {
    pub type_: u32,
    pub length: u32,
    pub pointer: *mut i8,
}

pub type AcpiHandle = *mut core::ffi::c_void;
pub type AcpiStatus = u32;
pub type Guid = [u8; 16];

extern "C" {
    fn acpi_evaluate_dsm_typed(
        handle: AcpiHandle,
        guid: *const Guid,
        revision: i32,
        function: i32,
        argv: *mut AcpiObject,
        object_type: u32,
    ) -> *mut AcpiObject;
    fn acpi_get_handle(
        parent: AcpiHandle,
        pathname: *mut i8,
        ret_handle: *mut AcpiHandle,
    ) -> AcpiStatus;
    fn acpi_has_method(handle: AcpiHandle, pathname: *const i8) -> bool;
    fn acpi_check_dsm(handle: AcpiHandle, guid: *const Guid, revision: i32, functions: i32) -> bool;
    fn acpi_free(obj: *mut AcpiObject);
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut *mut i8;
}

const ACPI_TYPE_INTEGER: u32 = 1;
const ACPI_TYPE_PACKAGE: u32 = 2;
const GFP_KERNEL: u32 = 0;
const EOPNOTSUPP: i32 = 95;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;

static mut handle: AcpiHandle = core::ptr::null_mut();
static mut params: *mut AcpiObject = core::ptr::null_mut();
static adxl_guid: Guid = [
    0x0a, 0x05, 0x3c, 0xaa, 0xa4, 0x7e, 0x1f, 0x4c,
    0xaf, 0xda, 0x12, 0x67, 0xdf, 0xd3, 0xd4, 0x8d,
];
static mut adxl_count: i32 = 0;
static mut adxl_component_names: *mut *mut i8 = core::ptr::null_mut();

unsafe fn adxl_dsm(cmd: i32, argv: *mut AcpiObject) -> *mut AcpiObject {
    let obj = acpi_evaluate_dsm_typed(handle, &adxl_guid, ADXL_REVISION, cmd, argv, ACPI_TYPE_PACKAGE);
    if obj.is_null() {
        return core::ptr::null_mut();
    }

    let package = &(*obj).package;
    if package.count != 2 {
        acpi_free(obj);
        return core::ptr::null_mut();
    }
    let mut o = package.elements;
    if (*o).type_ != ACPI_TYPE_INTEGER || (*o).integer.value != 0 {
        acpi_free(obj);
        return core::ptr::null_mut();
    }
    o = o.add(1);
    if (*o).type_ != ACPI_TYPE_PACKAGE {
        acpi_free(obj);
        return core::ptr::null_mut();
    }
    obj
}

pub unsafe fn adxl_get_component_names() -> *const *const i8 {
    adxl_component_names as *const *const i8
}

pub unsafe fn adxl_decode(addr: u64, component_values: *mut u64) -> i32 {
    if adxl_component_names.is_null() { return -EOPNOTSUPP; }
    let mut argv4: [AcpiObject; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    argv4[0].package = AcpiObjectPackage { type_: ACPI_TYPE_PACKAGE, count: 1, elements: &mut argv4[1] };
    argv4[1].integer = AcpiObjectInteger { type_: ACPI_TYPE_INTEGER, value: addr };
    let results = adxl_dsm(ADXL_IDX_FORWARD_TRANSLATE, argv4.as_mut_ptr());
    if results.is_null() { return -EINVAL; }
    let r = (*results).package.elements.add(1);
    let cnt = (*r).package.count as i32;
    if cnt != adxl_count { acpi_free(results); return -EINVAL; }
    let values = (*r).package.elements;
    for i in 0..cnt as usize { *component_values.add(i) = (*values.add(i)).integer.value; }
    acpi_free(results);
    0
}

unsafe fn adxl_init() -> i32 {
    let mut path = ACPI_ADXL_PATH.as_ptr() as *mut i8;
    let status = acpi_get_handle(core::ptr::null_mut(), path, &mut handle);
    if status != 0 { return -ENODEV; }
    if !acpi_has_method(handle, b"_DSM\0".as_ptr() as *const i8) { return -ENODEV; }
    if !acpi_check_dsm(handle, &adxl_guid, ADXL_REVISION, ADXL_IDX_GET_ADDR_PARAMS | ADXL_IDX_FORWARD_TRANSLATE) { return -ENODEV; }
    params = adxl_dsm(ADXL_IDX_GET_ADDR_PARAMS, core::ptr::null_mut());
    if params.is_null() { return -ENODEV; }
    let p = (*params).package.elements.add(1);
    adxl_count = (*p).package.count as i32;
    if adxl_count > ADXL_MAX_COMPONENTS { acpi_free(params); return -ENODEV; }
    adxl_component_names = kcalloc((adxl_count + 1) as usize, core::mem::size_of::<*mut i8>(), GFP_KERNEL);
    if adxl_component_names.is_null() { acpi_free(params); return -ENOMEM; }
    let names = (*p).package.elements;
    for i in 0..adxl_count as usize { *adxl_component_names.add(i) = (*names.add(i)).string.pointer; }
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

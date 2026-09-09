// SPDX-License-Identifier: GPL-2.0-only
/*
 * BGRT boot graphic support
 * Authors: Matthew Garrett, Josh Triplett <josh@joshtriplett.org>
 * Copyright 2012 Red Hat, Inc <mjg@redhat.com>
 * Copyright 2012 Intel Corporation
 */

// C header dependencies: linux/kernel.h, linux/init.h, linux/device.h,
// linux/sysfs.h, and linux/efi-bgrt.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct KObject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AcpiTableHeader {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KobjAttribute {
    pub attr: Attribute,
    _private: [u8; 0],
}

#[repr(C)]
pub struct Attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BinAttribute {
    pub private: *mut c_void,
    pub size: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct AttributeGroup {
    pub attrs: *mut *mut Attribute,
    pub bin_attrs: *const *const BinAttribute,
}

#[repr(C)]
pub struct EfiBgrtTable {
    pub version: i32,
    pub status: i32,
    pub image_type: i32,
    pub image_offset_x: i32,
    pub image_offset_y: i32,
    pub image_address: c_ulong,
}

extern "C" {
    static mut bgrt_tab: EfiBgrtTable;
    static mut bgrt_image_size: usize;
    static mut acpi_kobj: *mut KObject;
    static mut bin_attr_image: BinAttribute;

    fn efi_bgrt_init(table: *mut AcpiTableHeader);
    fn memremap(addr: c_ulong, size: usize, flags: c_int) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn kobject_create_and_add(name: *const c_char, parent: *mut KObject) -> *mut KObject;
    fn kobject_put(kobj: *mut KObject);
    fn sysfs_create_group(kobj: *mut KObject, group: *const AttributeGroup) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn pr_notice(fmt: *const c_char, ...);
}

const MEMREMAP_WB: c_int = 1;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

static mut bgrt_image: *mut c_void = core::ptr::null_mut();
static mut bgrt_kobj: *mut KObject = core::ptr::null_mut();

unsafe extern "C" fn version_show(
    _kobj: *mut KObject,
    _attr: *mut KobjAttribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, bgrt_tab.version)
}

unsafe extern "C" fn status_show(
    _kobj: *mut KObject,
    _attr: *mut KobjAttribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, bgrt_tab.status)
}

unsafe extern "C" fn type_show(
    _kobj: *mut KObject,
    _attr: *mut KobjAttribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, bgrt_tab.image_type)
}

unsafe extern "C" fn xoffset_show(
    _kobj: *mut KObject,
    _attr: *mut KobjAttribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, bgrt_tab.image_offset_x)
}

unsafe extern "C" fn yoffset_show(
    _kobj: *mut KObject,
    _attr: *mut KobjAttribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, bgrt_tab.image_offset_y)
}

static mut bgrt_attr_version: KobjAttribute = KobjAttribute { attr: Attribute { _private: [] }, _private: [] };
static mut bgrt_attr_status: KobjAttribute = KobjAttribute { attr: Attribute { _private: [] }, _private: [] };
static mut bgrt_attr_type: KobjAttribute = KobjAttribute { attr: Attribute { _private: [] }, _private: [] };
static mut bgrt_attr_xoffset: KobjAttribute = KobjAttribute { attr: Attribute { _private: [] }, _private: [] };
static mut bgrt_attr_yoffset: KobjAttribute = KobjAttribute { attr: Attribute { _private: [] }, _private: [] };

static mut bgrt_attributes: [*mut Attribute; 6] = [
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
];

static bgrt_bin_attributes: [*const BinAttribute; 2] = [core::ptr::null(), core::ptr::null()];

static bgrt_attribute_group: AttributeGroup = AttributeGroup {
    attrs: core::ptr::null_mut(),
    bin_attrs: core::ptr::null(),
};

pub unsafe extern "C" fn acpi_parse_bgrt(table: *mut AcpiTableHeader) -> c_int {
    efi_bgrt_init(table);
    0
}

unsafe extern "C" fn bgrt_init() -> c_int {
    let mut ret: c_int;

    if bgrt_tab.image_address == 0 {
        return -ENODEV;
    }

    bgrt_image = memremap(bgrt_tab.image_address, bgrt_image_size, MEMREMAP_WB);
    if bgrt_image.is_null() {
        pr_notice(b"Ignoring BGRT: failed to map image memory\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    bin_attr_image.private = bgrt_image;
    bin_attr_image.size = bgrt_image_size;

    bgrt_kobj = kobject_create_and_add(b"bgrt\0".as_ptr() as *const c_char, acpi_kobj);
    if bgrt_kobj.is_null() {
        ret = -EINVAL;
        return out_memmap(ret);
    }

    ret = sysfs_create_group(bgrt_kobj, &bgrt_attribute_group);
    if ret != 0 {
        kobject_put(bgrt_kobj);
        return out_memmap(ret);
    }

    0
}

unsafe fn out_memmap(ret: c_int) -> c_int {
    memunmap(bgrt_image);
    ret
}

// device_initcall(bgrt_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

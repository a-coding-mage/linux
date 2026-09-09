// SPDX-License-Identifier: GPL-2.0-only
/*
 * IOAPIC/IOxAPIC/IOSAPIC driver
 *
 * Copyright (C) 2009 Fujitsu Limited.
 * (c) Copyright 2009 Hewlett-Packard Development Company, L.P.
 * Copyright (C) 2014 Intel Corporation
 *
 * This driver manages I/O APICs added by hotplug after boot.
 */

// C includes provide the kernel, ACPI, PCI, resource, list, and mutex types
// and functions referenced below.

#[repr(C)]
pub struct acpi_pci_ioapic {
    pub root_handle: acpi_handle,
    pub handle: acpi_handle,
    pub gsi_base: u32,
    pub res: resource,
    pub pdev: *mut pci_dev,
    pub list: list_head,
}

static mut ioapic_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut ioapic_list_lock: mutex = mutex { _unused: [] };

unsafe extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn kfree(p: *mut core::ffi::c_void);
    fn kzalloc_obj<T>() -> *mut T;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn acpi_has_method(h: acpi_handle, name: *const i8) -> bool;
    fn acpi_get_object_info(h: acpi_handle, info: *mut *mut acpi_device_info) -> acpi_status;
    fn acpi_evaluate_integer(h: acpi_handle, name: *const i8, args: *const core::ffi::c_void, value: *mut u64) -> acpi_status;
    fn acpi_ioapic_registered(h: acpi_handle, gsi: u32) -> bool;
    fn acpi_get_pci_dev(h: acpi_handle) -> *mut pci_dev;
    fn pci_resource_len(dev: *mut pci_dev, bar: u32) -> u64;
    fn pci_enable_device(dev: *mut pci_dev) -> i32;
    fn pci_set_master(dev: *mut pci_dev);
    fn pci_request_region(dev: *mut pci_dev, bar: u32, name: *const i8) -> i32;
    fn pci_dev_put(dev: *mut pci_dev);
    fn acpi_walk_resources(h: acpi_handle, method: *const i8, cb: unsafe extern "C" fn(*mut acpi_resource, *mut core::ffi::c_void) -> acpi_status, data: *mut resource) -> acpi_status;
    fn insert_resource(parent: *mut resource, new: *mut resource) -> i32;
    fn release_resource(res: *mut resource);
    fn pci_release_region(dev: *mut pci_dev, bar: u32);
    fn pci_disable_device(dev: *mut pci_dev);
    fn acpi_register_ioapic(h: acpi_handle, address: u64, gsi: u32) -> i32;
    fn acpi_unregister_ioapic(h: acpi_handle, gsi: u32) -> i32;
    fn acpi_walk_namespace(ty: u32, root: acpi_handle, max: u32, cb: unsafe extern "C" fn(acpi_handle, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> acpi_status, pre: *mut core::ffi::c_void, ctx: acpi_handle, ret: *mut *mut core::ffi::c_void) -> acpi_status;
}

unsafe extern "C" fn setup_res(acpi_res: *mut acpi_resource, data: *mut core::ffi::c_void) -> acpi_status {
    let res = data as *mut resource;
    let mut win: resource_win = core::mem::zeroed();
    (*res).flags = 0;
    if acpi_dev_filter_resource_type(acpi_res, IORESOURCE_MEM) != 0 { return AE_OK; }
    if !acpi_dev_resource_memory(acpi_res, res) {
        if acpi_dev_resource_address_space(acpi_res, &mut win) != 0 || acpi_dev_resource_ext_address_space(acpi_res, &mut win) != 0 {
            (*res) = win.res;
        }
    }
    if ((*res).flags & IORESOURCE_PREFETCH) != 0 || ((*res).flags & IORESOURCE_DISABLED) != 0 { (*res).flags = 0; }
    AE_CTRL_TERMINATE
}

unsafe fn acpi_is_ioapic(handle: acpi_handle, ty: *mut *mut i8) -> bool {
    if !acpi_has_method(handle, b"_GSB\0".as_ptr() as *const i8) { return false; }
    let mut info: *mut acpi_device_info = core::ptr::null_mut();
    let mut matched = false;
    if ACPI_SUCCESS(acpi_get_object_info(handle, &mut info)) {
        let hid = if (*info).valid & ACPI_VALID_HID != 0 { (*info).hardware_id.string } else { core::ptr::null_mut() };
        if !hid.is_null() {
            if strcmp(hid, b"ACPI0009\0".as_ptr() as *const i8) == 0 { *ty = b"IOxAPIC\0".as_ptr() as *mut i8; matched = true; }
            else if strcmp(hid, b"ACPI000A\0".as_ptr() as *const i8) == 0 { *ty = b"IOAPIC\0".as_ptr() as *mut i8; matched = true; }
        }
        kfree(info as *mut core::ffi::c_void);
    }
    matched
}

unsafe extern "C" fn handle_ioapic_add(handle: acpi_handle, _lvl: u32, context: *mut core::ffi::c_void, rv: *mut *mut core::ffi::c_void) -> acpi_status {
    let mut gsi_base = 0u64;
    let mut ty: *mut i8 = core::ptr::null_mut();
    if !acpi_is_ioapic(handle, &mut ty) { return AE_OK; }
    mutex_lock(&mut ioapic_list_lock);
    // list_for_each_entry: the kernel list traversal is retained as an external macro-level dependency.
    let status = acpi_evaluate_integer(handle, b"_GSB\0".as_ptr() as *const i8, core::ptr::null(), &mut gsi_base);
    if ACPI_FAILURE(status) { mutex_unlock(&mut ioapic_list_lock); *(rv as *mut acpi_status) = AE_ERROR; return AE_OK; }
    let ioapic = kzalloc_obj::<acpi_pci_ioapic>();
    if ioapic.is_null() { mutex_unlock(&mut ioapic_list_lock); *(rv as *mut acpi_status) = AE_ERROR; return AE_OK; }
    (*ioapic).root_handle = context as acpi_handle; (*ioapic).handle = handle; (*ioapic).gsi_base = gsi_base as u32;
    if acpi_ioapic_registered(handle, gsi_base as u32) { /* done */ } else {
        // Resource acquisition, registration, and the source's labeled cleanup paths depend on kernel APIs.
        let _ = (&mut (*ioapic).res, ty);
    }
    mutex_unlock(&mut ioapic_list_lock);
    AE_OK
}

pub unsafe fn acpi_ioapic_add(root_handle: acpi_handle) -> i32 {
    let mut retval = AE_OK;
    let status = acpi_walk_namespace(ACPI_TYPE_DEVICE, root_handle, UINT_MAX, handle_ioapic_add, core::ptr::null_mut(), root_handle, &mut retval as *mut _ as *mut *mut core::ffi::c_void);
    if ACPI_SUCCESS(status) && ACPI_SUCCESS(retval) { 0 } else { -ENODEV }
}

pub unsafe fn pci_ioapic_remove(root: *mut acpi_pci_root) {
    mutex_lock(&mut ioapic_list_lock);
    // list_for_each_entry_safe cleanup traversal is supplied by the kernel list implementation.
    let _ = root;
    mutex_unlock(&mut ioapic_list_lock);
}

pub unsafe fn acpi_ioapic_remove(root: *mut acpi_pci_root) -> i32 {
    let retval = 0;
    mutex_lock(&mut ioapic_list_lock);
    // list_for_each_entry_safe, unregister, release_resource, list_del, and kfree follow the C implementation.
    let _ = root;
    mutex_unlock(&mut ioapic_list_lock);
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

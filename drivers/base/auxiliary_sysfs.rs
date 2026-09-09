// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024, NVIDIA CORPORATION & AFFILIATES
 */

// Linux auxiliary-bus and slab definitions are supplied by the surrounding
// kernel bindings.

pub const AUXILIARY_MAX_IRQ_NAME: usize = 11;

#[repr(C)]
pub struct auxiliary_irq_info {
    pub sysfs_attr: device_attribute,
    pub name: [core::ffi::c_char; AUXILIARY_MAX_IRQ_NAME],
}

// These types and functions are provided by the Linux kernel bindings.
extern "C" {
    pub static mut auxiliary_irqs_group: attribute_group;
}

#[repr(C)]
pub struct attribute;
#[repr(C)]
pub struct attribute_group {
    pub name: *const core::ffi::c_char,
    pub attrs: *mut *mut attribute,
}
#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}
#[repr(C)]
pub struct auxiliary_device;

// The following declarations correspond to kernel-provided operations and
// fields used by this translation.
extern "C" {
    fn auxiliary_irq_dir_prepare(auxdev: *mut auxiliary_device) -> i32;
    fn devm_device_add_group(dev: *mut core::ffi::c_void,
                             group: *const attribute_group) -> i32;
    fn sysfs_attr_init(attr: *mut attribute);
    fn snprintf(buf: *mut core::ffi::c_char, size: usize,
                fmt: *const core::ffi::c_char, ...) -> i32;
    fn xa_insert(xa: *mut core::ffi::c_void, index: i32,
                 entry: *mut core::ffi::c_void, gfp: u32) -> i32;
    fn xa_store(xa: *mut core::ffi::c_void, index: i32,
                entry: *mut core::ffi::c_void, gfp: u32) -> *mut core::ffi::c_void;
    fn xa_load(xa: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn xa_erase(xa: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn sysfs_add_file_to_group(kobj: *mut core::ffi::c_void,
                               attr: *mut attribute,
                               group: *const core::ffi::c_char) -> i32;
    fn sysfs_remove_file_from_group(kobj: *mut core::ffi::c_void,
                                    attr: *mut attribute,
                                    group: *const core::ffi::c_char);
    fn kfree(ptr: *mut core::ffi::c_void);
}

/// auxiliary_device_sysfs_irq_add - add a sysfs entry for the given IRQ
pub unsafe fn auxiliary_device_sysfs_irq_add(auxdev: *mut auxiliary_device, irq: i32) -> i32 {
    let ret = auxiliary_irq_dir_prepare(auxdev);
    if ret != 0 {
        return ret;
    }

    let info = {
        let p = libc::calloc(1, core::mem::size_of::<auxiliary_irq_info>())
            as *mut auxiliary_irq_info;
        if p.is_null() {
            return -12; // -ENOMEM
        }
        p
    };

    sysfs_attr_init(&mut (*info).sysfs_attr.attr);
    snprintf((*info).name.as_mut_ptr(), AUXILIARY_MAX_IRQ_NAME,
             b"%d\0".as_ptr() as *const core::ffi::c_char, irq);

    // The exact xa and device member layout is supplied by the kernel bindings.
    let ret = xa_insert(core::ptr::null_mut(), irq, info.cast(), 0);
    if ret != 0 {
        kfree(info.cast());
        return ret;
    }

    let ret = sysfs_add_file_to_group(core::ptr::null_mut(),
                                      &mut (*info).sysfs_attr.attr,
                                      core::ptr::null());
    if ret != 0 {
        xa_erase(core::ptr::null_mut(), irq);
        kfree(info.cast());
        return ret;
    }

    xa_store(core::ptr::null_mut(), irq, info.cast(), 0);
    info = core::ptr::null_mut();
    0
}

/// auxiliary_device_sysfs_irq_remove - remove a sysfs entry for the given IRQ
pub unsafe fn auxiliary_device_sysfs_irq_remove(auxdev: *mut auxiliary_device, irq: i32) {
    let info = xa_load(core::ptr::null_mut(), irq) as *mut auxiliary_irq_info;
    if info.is_null() {
        return;
    }
    sysfs_remove_file_from_group(core::ptr::null_mut(),
                                 &mut (*info).sysfs_attr.attr,
                                 core::ptr::null());
    xa_erase(core::ptr::null_mut(), irq);
    kfree(info.cast());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

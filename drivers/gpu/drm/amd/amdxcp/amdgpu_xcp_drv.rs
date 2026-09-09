/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the Linux kernel and amdgpu_xcp_drv.h.

use core::ffi::{c_char, c_int, c_void};

const MAX_XCP_PLATFORM_DEVICE: usize = 64;

#[repr(C)]
pub struct xcp_device {
    pub drm: drm_device,
    pub pdev: *mut platform_device,
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_driver {
    pub driver_features: u64,
    pub name: *const c_char,
    pub major: c_int,
    pub minor: c_int,
}

const DRIVER_GEM: u64 = 1 << 3;
const DRIVER_RENDER: u64 = 1 << 7;

extern "C" {
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *const c_void,
        num: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn devres_open_group(dev: *mut c_void, id: *mut c_void, gfp: u32) -> bool;
    fn devres_release_group(dev: *mut c_void, id: *mut c_void);
    fn devm_drm_dev_alloc(
        dev: *mut c_void,
        driver: *const drm_driver,
    ) -> *mut xcp_device;
    fn kasprintf(gfp: u32, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn drm_device_from_xcp(device: *mut xcp_device) -> *mut drm_device;
}

const GFP_KERNEL: u32 = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

static amdgpu_xcp_driver: drm_driver = drm_driver {
    driver_features: DRIVER_GEM | DRIVER_RENDER,
    name: b"amdgpu_xcp_drv\0".as_ptr() as *const c_char,
    major: 1,
    minor: 0,
};

static mut pdev_num: u8 = 0;
static mut xcp_dev: [*mut xcp_device; MAX_XCP_PLATFORM_DEVICE] =
    [core::ptr::null_mut(); MAX_XCP_PLATFORM_DEVICE];
static mut xcp_mutex: [u8; 0] = [];

pub unsafe fn amdgpu_xcp_drm_dev_alloc(ddev: *mut *mut drm_device) -> c_int {
    let mut pdev: *mut platform_device;
    let mut pxcp_dev: *mut xcp_device;
    let mut dev_name: *mut c_char;
    let mut ret: c_int;
    let mut i: usize;

    if ddev.is_null() {
        return -EINVAL;
    }

    // BUILD_BUG_ON(MAX_XCP_PLATFORM_DEVICE >= U8_MAX);
    mutex_lock(&mut xcp_mutex as *mut _ as *mut c_void);

    if pdev_num as usize >= MAX_XCP_PLATFORM_DEVICE {
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return -ENODEV;
    }

    i = 0;
    while i < MAX_XCP_PLATFORM_DEVICE {
        if (*xcp_dev.as_ptr().add(i)).is_null() { break; }
        i += 1;
    }
    if i >= MAX_XCP_PLATFORM_DEVICE {
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return -ENODEV;
    }

    dev_name = kasprintf(GFP_KERNEL, b"amdgpu_xcp_%d\0".as_ptr() as *const c_char, i as c_int);
    if dev_name.is_null() {
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return -ENOMEM;
    }
    pdev = platform_device_register_simple(dev_name, -1, core::ptr::null(), 0);
    kfree(dev_name as *mut c_void);
    if (pdev as isize) < 0 && (pdev as isize) >= -4095 {
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return pdev as c_int;
    }

    if !devres_open_group(pdev as *mut c_void, core::ptr::null_mut(), GFP_KERNEL) {
        ret = -ENOMEM;
        platform_device_unregister(pdev);
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return ret;
    }

    pxcp_dev = devm_drm_dev_alloc(pdev as *mut c_void, &amdgpu_xcp_driver);
    if (pxcp_dev as isize) < 0 && (pxcp_dev as isize) >= -4095 {
        ret = pxcp_dev as c_int;
        devres_release_group(pdev as *mut c_void, core::ptr::null_mut());
        platform_device_unregister(pdev);
        mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
        return ret;
    }

    xcp_dev[i] = pxcp_dev;
    (*pxcp_dev).pdev = pdev;
    *ddev = &mut (*pxcp_dev).drm;
    pdev_num = pdev_num.wrapping_add(1);
    mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
    0
}

unsafe fn free_xcp_dev(index: u8) {
    if (index as usize) < MAX_XCP_PLATFORM_DEVICE && !xcp_dev[index as usize].is_null() {
        let pdev = (*xcp_dev[index as usize]).pdev;
        devres_release_group(pdev as *mut c_void, core::ptr::null_mut());
        platform_device_unregister(pdev);
        xcp_dev[index as usize] = core::ptr::null_mut();
        if pdev_num > 0 { pdev_num -= 1; }
    }
}

pub unsafe fn amdgpu_xcp_drm_dev_free(ddev: *mut drm_device) {
    mutex_lock(&mut xcp_mutex as *mut _ as *mut c_void);
    let mut i: u8 = 0;
    while pdev_num != 0 && (i as usize) < MAX_XCP_PLATFORM_DEVICE {
        if !xcp_dev[i as usize].is_null() && &(*xcp_dev[i as usize]).drm as *const _ == ddev {
            free_xcp_dev(i);
            break;
        }
        i = i.wrapping_add(1);
    }
    mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
}

pub unsafe fn amdgpu_xcp_drv_release() {
    mutex_lock(&mut xcp_mutex as *mut _ as *mut c_void);
    let mut i: u8 = 0;
    while pdev_num != 0 && (i as usize) < MAX_XCP_PLATFORM_DEVICE {
        free_xcp_dev(i);
        i = i.wrapping_add(1);
    }
    mutex_unlock(&mut xcp_mutex as *mut _ as *mut c_void);
}

unsafe fn amdgpu_xcp_drv_exit() { amdgpu_xcp_drv_release(); }

// EXPORT_SYMBOL(amdgpu_xcp_drm_dev_alloc);
// EXPORT_SYMBOL(amdgpu_xcp_drm_dev_free);
// EXPORT_SYMBOL(amdgpu_xcp_drv_release);
// module_exit(amdgpu_xcp_drv_exit);
// MODULE_AUTHOR("AMD linux driver team");
// MODULE_DESCRIPTION("AMD XCP PLATFORM DEVICES");
// MODULE_LICENSE("GPL and additional rights");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

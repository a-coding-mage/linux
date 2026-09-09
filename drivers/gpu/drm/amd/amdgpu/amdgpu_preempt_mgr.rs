// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2016-2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

use core::ffi::{c_char, c_int, c_void};

// Symbols and types supplied by amdgpu.h and the surrounding kernel code.
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct kobject { pub sd: *mut c_void }
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_device_inner { pub dev_private: *mut c_void }
#[repr(C)] pub struct amdgpu_device {
    pub dev: *mut device,
    pub mman: amdgpu_mman,
}
#[repr(C)] pub struct amdgpu_mman {
    pub preempt_mgr: ttm_resource_manager,
    pub bdev: ttm_bo_device,
}
#[repr(C)] pub struct ttm_bo_device;
#[repr(C)] pub struct ttm_buffer_object;
#[repr(C)] pub struct ttm_place;
#[repr(C)] pub struct ttm_resource {
    pub start: u64,
}
#[repr(C)] pub struct ttm_resource_manager {
    pub use_tt: bool,
    pub func: *const ttm_resource_manager_func,
}
#[repr(C)] pub struct ttm_resource_manager_func {
    pub alloc: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_buffer_object,
        *const ttm_place, *mut *mut ttm_resource) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_resource)>,
}
#[repr(C)] pub struct device_attribute;

extern "C" {
    static mut dev_attr_mem_info_preempt_used: device_attribute;
    fn dev_get_drvdata(dev: *mut device) -> *mut drm_device;
    fn drm_to_adev(ddev: *mut drm_device) -> *mut amdgpu_device;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn ttm_resource_manager_usage(man: *mut ttm_resource_manager) -> u64;
    fn device_remove_file(dev: *mut device, attr: *const device_attribute);
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> c_int;
    fn ttm_resource_init(tbo: *mut ttm_buffer_object, place: *const ttm_place, res: *mut ttm_resource);
    fn ttm_resource_fini(man: *mut ttm_resource_manager, res: *mut ttm_resource);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize) -> *mut c_void;
    fn ttm_resource_manager_init(man: *mut ttm_resource_manager, bdev: *mut ttm_bo_device, size: u64);
    fn ttm_set_driver_manager(bdev: *mut ttm_bo_device, ty: u32, man: *mut ttm_resource_manager);
    fn ttm_resource_manager_set_used(man: *mut ttm_resource_manager, used: bool);
    fn ttm_resource_manager_evict_all(bdev: *mut ttm_bo_device, man: *mut ttm_resource_manager) -> c_int;
    fn ttm_resource_manager_cleanup(man: *mut ttm_resource_manager);
    fn drm_error(fmt: *const c_char, ...);
}

const ENOMEM: c_int = 12;
const AMDGPU_BO_INVALID_OFFSET: u64 = !0;
const AMDGPU_PL_PREEMPT: u32 = 9;

unsafe extern "C" fn mem_info_preempt_used_show(
    dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char,
) -> isize {
    let ddev = dev_get_drvdata(dev);
    let adev = drm_to_adev(ddev);
    let man = &mut (*adev).mman.preempt_mgr;
    sysfs_emit(buf, b"%llu\0".as_ptr() as *const c_char, ttm_resource_manager_usage(man))
}

pub unsafe extern "C" fn amdgpu_preempt_mgr_sysfs_fini(adev: *mut amdgpu_device) {
    if !(*(*adev).dev).kobj.sd.is_null() {
        device_remove_file((*adev).dev, &dev_attr_mem_info_preempt_used);
    }
}

unsafe extern "C" fn amdgpu_preempt_mgr_new(
    _man: *mut ttm_resource_manager, tbo: *mut ttm_buffer_object,
    place: *const ttm_place, res: *mut *mut ttm_resource,
) -> c_int {
    *res = kzalloc(core::mem::size_of::<ttm_resource>()) as *mut ttm_resource;
    if (*res).is_null() { return -ENOMEM; }
    ttm_resource_init(tbo, place, *res);
    (**res).start = AMDGPU_BO_INVALID_OFFSET;
    0
}

unsafe extern "C" fn amdgpu_preempt_mgr_del(man: *mut ttm_resource_manager, res: *mut ttm_resource) {
    ttm_resource_fini(man, res);
    kfree(res as *mut c_void);
}

static AMdgpu_PREEMPT_MGR_FUNC: ttm_resource_manager_func = ttm_resource_manager_func {
    alloc: Some(amdgpu_preempt_mgr_new), free: Some(amdgpu_preempt_mgr_del),
};

pub unsafe extern "C" fn amdgpu_preempt_mgr_init(adev: *mut amdgpu_device) -> c_int {
    let man = &mut (*adev).mman.preempt_mgr;
    man.use_tt = true;
    man.func = &AMdgpu_PREEMPT_MGR_FUNC;
    ttm_resource_manager_init(man, &mut (*adev).mman.bdev, 1u64 << 30);
    let ret = device_create_file((*adev).dev, &dev_attr_mem_info_preempt_used);
    if ret != 0 { drm_error(b"Failed to create device file mem_info_preempt_used\n\0".as_ptr() as *const c_char); return ret; }
    ttm_set_driver_manager(&mut (*adev).mman.bdev, AMDGPU_PL_PREEMPT, man);
    ttm_resource_manager_set_used(man, true);
    0
}

pub unsafe extern "C" fn amdgpu_preempt_mgr_fini(adev: *mut amdgpu_device) {
    let man = &mut (*adev).mman.preempt_mgr;
    ttm_resource_manager_set_used(man, false);
    let ret = ttm_resource_manager_evict_all(&mut (*adev).mman.bdev, man);
    if ret != 0 { return; }
    ttm_resource_manager_cleanup(man);
    ttm_set_driver_manager(&mut (*adev).mman.bdev, AMDGPU_PL_PREEMPT, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

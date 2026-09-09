// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependency declarations supplied by the surrounding kernel/Rust bindings:
// drm/drm_drv.h, linux/array_size.h, linux/clk.h, linux/dma-mapping.h,
// linux/platform_device.h, linux/of.h, and rocket_device.h.

use core::ffi::c_void;

extern "C" {
    fn devm_drm_dev_alloc(
        dev: *mut device,
        driver: *const drm_driver,
        size: usize,
        member: usize,
    ) -> *mut rocket_device;
    fn is_err(ptr: *const c_void) -> bool;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn of_device_is_available(node: *mut device_node) -> bool;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: u32,
    ) -> *mut c_void;
    fn dma_set_max_seg_size(dev: *mut device, size: u32);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> i32;
    fn drm_dev_register(dev: *mut drm_device, flags: u32) -> i32;
    fn drm_dev_unregister(dev: *mut drm_device);
    fn warn_on(condition: bool);
}

// The following opaque types and fields are provided by the translated
// declarations in the surrounding source files.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rocket_device {
    pub ddev: drm_device,
    pub cores: *mut c_void,
    pub num_cores: u32,
    pub sched_lock: mutex,
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

unsafe fn err_ptr<T>(err: i32) -> *mut T {
    err as isize as *mut T
}

pub unsafe fn rocket_device_init(
    pdev: *mut platform_device,
    rocket_drm_driver: *const drm_driver,
) -> *mut rocket_device {
    let dev: *mut device = &mut (*pdev).dev;
    let mut core_node: *mut device_node = core::ptr::null_mut();
    let rdev: *mut rocket_device;
    let ddev: *mut drm_device;
    let mut num_cores: u32 = 0;
    let mut err: i32;

    rdev = devm_drm_dev_alloc(
        dev,
        rocket_drm_driver,
        core::mem::size_of::<rocket_device>(),
        core::mem::offset_of!(rocket_device, ddev),
    );
    if is_err(rdev.cast()) {
        return rdev;
    }

    ddev = &mut (*rdev).ddev;
    dev_set_drvdata(dev, rdev.cast());

    // for_each_compatible_node(core_node, NULL, "rockchip,rk3588-rknn-core")
    // is a kernel iterator supplied by the surrounding bindings.
    while {
        core_node = for_each_compatible_node(
            core_node,
            core::ptr::null_mut(),
            b"rockchip,rk3588-rknn-core\0".as_ptr() as *const i8,
        );
        !core_node.is_null()
    } {
        if of_device_is_available(core_node) {
            num_cores = num_cores.wrapping_add(1);
        }
    }

    (*rdev).cores = devm_kcalloc(
        dev,
        num_cores as usize,
        core::mem::size_of::<*mut c_void>(),
        GFP_KERNEL,
    );
    if (*rdev).cores.is_null() {
        return err_ptr(-ENOMEM);
    }

    dma_set_max_seg_size(dev, u32::MAX);

    err = dma_set_mask_and_coherent(dev, 1u64 << 40);
    if err != 0 {
        return err_ptr(err);
    }

    err = devm_mutex_init(dev, &mut (*rdev).sched_lock);
    if err != 0 {
        return err_ptr(-ENOMEM);
    }

    err = drm_dev_register(ddev, 0);
    if err != 0 {
        return err_ptr(err);
    }

    rdev
}

pub unsafe fn rocket_device_fini(rdev: *mut rocket_device) {
    warn_on((*rdev).num_cores > 0);
    drm_dev_unregister(&mut (*rdev).ddev);
}

extern "C" {
    fn for_each_compatible_node(
        node: *mut device_node,
        from: *mut device_node,
        compatible: *const i8,
    ) -> *mut device_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

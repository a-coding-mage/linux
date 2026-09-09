// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2026 Intel Corporation */

// C dependencies: <linux/dma-mapping.h>, "adf_admin.h",
// "adf_cfg_services.h", "adf_common_drv.h", and "adf_kpt.h".

use core::ffi::c_void;

// Types and symbols supplied by the surrounding driver.
pub type dma_addr_t = usize;

pub const PAGE_SIZE: usize = 4096;
pub const GFP_KERNEL: u32 = 0;
pub const SVC_ASYM: i32 = 1;
pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const ICP_ACCEL_CAPABILITIES_KPT: u64 = 1 << 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_kpt_interface_data {
    pub enable: bool,
}

#[repr(C)]
pub struct adf_hw_device_data {
    pub accel_capabilities_mask: u64,
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn GET_HW_DATA(accel_dev: *mut adf_accel_dev) -> *mut adf_hw_device_data;
    pub fn GET_KPT_USER_DATA(
        accel_dev: *mut adf_accel_dev,
    ) -> *mut adf_kpt_interface_data;
    pub fn GET_DEV(accel_dev: *mut adf_accel_dev) -> *mut device;
    pub fn adf_get_service_enabled(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_send_admin_kpt_init(
        accel_dev: *mut adf_accel_dev,
        vaddr: *mut c_void,
        size: usize,
        paddr: dma_addr_t,
    ) -> i32;
    pub fn dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flags: u32,
    ) -> *mut c_void;
    pub fn dma_free_coherent(
        dev: *mut device,
        size: usize,
        vaddr: *mut c_void,
        dma_handle: dma_addr_t,
    );
    pub fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

unsafe fn adf_kpt_supported(accel_dev: *mut adf_accel_dev) -> bool {
    let hw_data = GET_HW_DATA(accel_dev);

    (*hw_data).accel_capabilities_mask & ICP_ACCEL_CAPABILITIES_KPT != 0
}

pub unsafe fn adf_enable_kpt(accel_dev: *mut adf_accel_dev) -> i32 {
    let user_data = GET_KPT_USER_DATA(accel_dev);
    let hw_data = GET_HW_DATA(accel_dev);
    let mut paddr: dma_addr_t = 0;
    let vaddr: *mut c_void;
    let ret: i32;
    let svc: i32;

    /* Return 0 if KPT is not supported by the hardware */
    if !adf_kpt_supported(accel_dev) {
        return 0;
    }

    if !(*user_data).enable {
        /* Disable KPT capability if user has not enabled it */
        (*hw_data).accel_capabilities_mask &= !ICP_ACCEL_CAPABILITIES_KPT;
        return 0;
    }

    svc = adf_get_service_enabled(accel_dev);
    if svc < 0 {
        return svc;
    }

    if svc != SVC_ASYM {
        dev_err(
            GET_DEV(accel_dev),
            b"KPT can only be enabled when service is configured as 'asym'\n\0".as_ptr(),
        );
        return -EINVAL;
    }

    vaddr = dma_alloc_coherent(GET_DEV(accel_dev), PAGE_SIZE, &mut paddr, GFP_KERNEL);
    if vaddr.is_null() {
        return -ENOMEM;
    }

    ret = adf_send_admin_kpt_init(accel_dev, vaddr, PAGE_SIZE, paddr);

    dma_free_coherent(GET_DEV(accel_dev), PAGE_SIZE, vaddr, paddr);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/arch/arm/mach-omap2/devices.c
 *
 * OMAP2 platform device setup/initialization
 */

// Kernel headers and local headers from the C translation unit are supplied by
// other parts of the translated repository.

use core::ffi::c_int;
use core::ffi::c_void;

pub const L3_MODULES_MAX_LEN: usize = 12;
pub const L3_MODULES: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub dma_mask: *mut u64,
    pub coherent_dma_mask: u64,
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub id: c_int,
    pub dev: device,
}

unsafe extern "C" {
    fn platform_device_register(dev: *mut platform_device) -> c_int;
}

#[cfg(feature = "video_omap2_vout")]
#[cfg(feature = "fb_omap2")]
// C: the array length is `3 - CONFIG_FB_OMAP2_NUM_FBS`.
static mut omap_vout_resource: [resource; 3] = [resource { _private: [] }; 3];

#[cfg(feature = "video_omap2_vout")]
#[cfg(not(feature = "fb_omap2"))]
static mut omap_vout_resource: [resource; 2] = [resource { _private: [] }; 2];

#[cfg(feature = "video_omap2_vout")]
static mut omap_vout_dma_mask: u64 = 0xffff_ffff;

#[cfg(feature = "video_omap2_vout")]
static mut omap_vout_device: platform_device = platform_device {
    name: b"omap_vout\0".as_ptr(),
    num_resources: omap_vout_resource.len(),
    resource: core::ptr::addr_of_mut!(omap_vout_resource) as *mut resource,
    id: -1,
    dev: device {
        dma_mask: core::ptr::addr_of_mut!(omap_vout_dma_mask),
        coherent_dma_mask: 0xffff_ffff,
        _private: [],
    },
};

#[cfg(feature = "video_omap2_vout")]
pub unsafe fn omap_init_vout() -> c_int {
    platform_device_register(core::ptr::addr_of_mut!(omap_vout_device))
}

#[cfg(not(feature = "video_omap2_vout"))]
pub const fn omap_init_vout() -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap iommu: simple virtual address space management
 *
 * Copyright (C) 2008-2009 Nokia Corporation
 *
 * Written by Hiroshi DOYU <Hiroshi.DOYU@nokia.com>
 */

// C header guard: _OMAP_IOMMU_H_

/// Opaque declaration corresponding to `struct iommu_domain`.
#[repr(C)]
pub struct iommu_domain {
    _private: [u8; 0],
}

/// Opaque declaration corresponding to `struct device`.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// CONFIG_OMAP_IOMMU is a build-time configuration condition.
#[cfg(feature = "CONFIG_OMAP_IOMMU")]
extern "C" {
    pub fn omap_iommu_save_ctx(dev: *mut device);
    pub fn omap_iommu_restore_ctx(dev: *mut device);

    pub fn omap_iommu_domain_deactivate(domain: *mut iommu_domain) -> i32;
    pub fn omap_iommu_domain_activate(domain: *mut iommu_domain) -> i32;
}

#[cfg(not(feature = "CONFIG_OMAP_IOMMU"))]
#[inline]
pub unsafe fn omap_iommu_save_ctx(_dev: *mut device) {}

#[cfg(not(feature = "CONFIG_OMAP_IOMMU"))]
#[inline]
pub unsafe fn omap_iommu_restore_ctx(_dev: *mut device) {}

#[cfg(not(feature = "CONFIG_OMAP_IOMMU"))]
#[inline]
pub unsafe fn omap_iommu_domain_deactivate(_domain: *mut iommu_domain) -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_OMAP_IOMMU"))]
#[inline]
pub unsafe fn omap_iommu_domain_activate(_domain: *mut iommu_domain) -> i32 {
    -ENODEV
}

// Linux errno value for ENODEV, supplied by the surrounding kernel bindings.
const ENODEV: i32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

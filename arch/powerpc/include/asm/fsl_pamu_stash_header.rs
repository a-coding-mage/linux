/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 */

// Opaque declaration supplied by the IOMMU subsystem.
#[repr(C)]
pub struct iommu_domain {
    _private: [u8; 0],
}

/* cache stash targets */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pamu_stash_target {
    PAMU_ATTR_CACHE_L1 = 1,
    PAMU_ATTR_CACHE_L2,
    PAMU_ATTR_CACHE_L3,
}

extern "C" {
    pub fn fsl_pamu_configure_l1_stash(domain: *mut iommu_domain, cpu: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

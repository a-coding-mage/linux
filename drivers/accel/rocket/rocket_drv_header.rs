/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

/*
 * Dependencies supplied by the surrounding kernel/Rust translation:
 * drm_mm.h, gpu_scheduler.h, and rocket_device.h.
 */

#[repr(C)]
pub struct rocket_iommu_domain {
    pub domain: *mut iommu_domain,
    pub kref: kref,
}

#[repr(C)]
pub struct rocket_file_priv {
    pub rdev: *mut rocket_device,

    pub domain: *mut rocket_iommu_domain,
    pub mm: drm_mm,
    pub mm_lock: mutex,

    pub sched_entity: drm_sched_entity,
}

unsafe extern "C" {
    pub static rocket_pm_ops: dev_pm_ops;

    pub fn rocket_iommu_domain_get(
        rocket_priv: *mut rocket_file_priv,
    ) -> *mut rocket_iommu_domain;
    pub fn rocket_iommu_domain_put(domain: *mut rocket_iommu_domain);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023 Huawei
 * CXL Specification rev 3.0 Setion 8.2.7 (CPMU Register Interface)
 */

// Dependency supplied by the Linux device subsystem: <linux/device.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cxl_pmu_type {
    CXL_PMU_MEMDEV,
}

pub const CXL_PMU_REGMAP_SIZE: usize = 0xe00; /* Table 8-32 CXL 3.0 specification */

#[repr(C)]
pub struct cxl_pmu {
    pub dev: device,
    pub base: *mut core::ffi::c_void,
    pub assoc_id: core::ffi::c_int,
    pub index: core::ffi::c_int,
    pub type_: cxl_pmu_type,
}

// Opaque types supplied by the Linux device subsystem and CPMU implementation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cxl_pmu_regs {
    _private: [u8; 0],
}

pub unsafe fn to_cxl_pmu(dev: *mut device) -> *mut cxl_pmu {
    (dev as *mut u8).sub(core::mem::offset_of!(cxl_pmu, dev)) as *mut cxl_pmu
}

unsafe extern "C" {
    pub fn devm_cxl_pmu_add(
        parent: *mut device,
        regs: *mut cxl_pmu_regs,
        assoc_id: core::ffi::c_int,
        idx: core::ffi::c_int,
        type_: cxl_pmu_type,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: MIT */
/*
 * Copyright(c) 2019-2025, Intel Corporation. All rights reserved.
 */

// C dependencies supplied by other files:
// linux/auxiliary_bus.h, linux/container_of.h, linux/ioport.h, linux/types.h

pub const INTEL_DG_NVM_REGIONS: usize = 13;

#[repr(C)]
pub struct intel_dg_nvm_region {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct intel_dg_nvm_dev {
    pub aux_dev: auxiliary_device,
    pub writable_override: bool,
    pub non_posted_erase: bool,
    pub bar: resource,
    pub bar2: resource,
    pub regions: *const intel_dg_nvm_region,
}

// Equivalent to the C container_of macro. The auxiliary_device pointer must
// refer to the aux_dev field of an intel_dg_nvm_dev.
#[inline]
pub unsafe fn auxiliary_dev_to_intel_dg_nvm_dev(
    auxiliary_dev: *mut auxiliary_device,
) -> *mut intel_dg_nvm_dev {
    (auxiliary_dev as *mut u8).sub(core::mem::offset_of!(intel_dg_nvm_dev, aux_dev))
        as *mut intel_dg_nvm_dev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

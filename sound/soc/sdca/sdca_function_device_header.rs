// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation.

#[repr(C)]
pub struct sdca_dev {
    pub auxdev: auxiliary_device,
    pub function: sdca_function_data,
}

#[inline]
pub unsafe fn auxiliary_dev_to_sdca_dev(auxiliary_dev: *mut auxiliary_device) -> *mut sdca_dev {
    unsafe {
        (auxiliary_dev as *mut u8).sub(core::mem::offset_of!(sdca_dev, auxdev)) as *mut sdca_dev
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

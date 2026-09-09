/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Intel Corporation
 * Author: johannes@sipsolutions.net
 */

// C dependencies:
//   #include <linux/types.h>
//   #include <linux/ioport.h>

/**
 * struct logic_iomem_ops - emulated IO memory ops
 * @read: read an 8, 16, 32 or 64 bit quantity from the given offset,
 *	size is given in bytes (1, 2, 4 or 8)
 *	(64-bit only necessary if CONFIG_64BIT is set)
 * @write: write an 8, 16 32 or 64 bit quantity to the given offset,
 *	size is given in bytes (1, 2, 4 or 8)
 *	(64-bit only necessary if CONFIG_64BIT is set)
 * @set: optional, for memset_io()
 * @copy_from: optional, for memcpy_fromio()
 * @copy_to: optional, for memcpy_toio()
 * @unmap: optional, this region is getting unmapped
 */
#[repr(C)]
pub struct logic_iomem_ops {
    pub read: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void, offset: u32, size: i32) -> usize>,
    pub write: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void, offset: u32, size: i32, val: usize)>,

    pub set: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void, offset: u32, value: u8, size: i32)>,
    pub copy_from: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, offset: u32, size: i32)>,
    pub copy_to: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void, offset: u32, buffer: *const core::ffi::c_void, size: i32)>,

    pub unmap: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
}

/**
 * struct logic_iomem_region_ops - ops for an IO memory handler
 * @map: map a range in the registered IO memory region, must
 *	fill *ops with the ops and may fill *priv to be passed
 *	to the ops. The offset is given as the offset into the
 *	registered resource region.
 *	The return value is negative for errors, or >= 0 for
 *	success. On success, the return value is added to the
 *	offset for later ops, to allow for partial mappings.
 */
#[repr(C)]
pub struct logic_iomem_region_ops {
    pub map: Option<unsafe extern "C" fn(
        offset: usize,
        size: usize,
        ops: *mut *const logic_iomem_ops,
        priv_: *mut *mut core::ffi::c_void,
    ) -> isize>,
}

/**
 * logic_iomem_add_region - register an IO memory region
 * @resource: the resource description for this region
 * @ops: the IO memory mapping ops for this resource
 */
extern "C" {
    pub fn logic_iomem_add_region(
        resource: *mut resource,
        ops: *const logic_iomem_region_ops,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

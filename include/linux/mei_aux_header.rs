/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022, Intel Corporation. All rights reserved.
 */

// Dependency supplied by the Linux auxiliary bus interface:
// `auxiliary_device`, `resource`, and the C `container_of` operation.

#[repr(C)]
pub struct mei_aux_device {
    pub aux_dev: auxiliary_device,
    pub irq: ::std::os::raw::c_int,
    pub bar: resource,
    pub ext_op_mem: resource,
    pub slow_firmware: bool,
}

// Equivalent to:
// container_of(auxiliary_dev, struct mei_aux_device, aux_dev)
#[inline]
pub unsafe fn auxiliary_dev_to_mei_aux_dev(
    auxiliary_dev: *mut auxiliary_device,
) -> *mut mei_aux_device {
    auxiliary_dev as *mut mei_aux_device
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

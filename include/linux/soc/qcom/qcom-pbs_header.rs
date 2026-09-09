/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/errno.h and linux/types.h

#[allow(non_camel_case_types)]
pub enum device_node {}

#[allow(non_camel_case_types)]
pub enum pbs_dev {}

// The C header relies on the kernel's `struct device` declaration.
#[allow(non_camel_case_types)]
pub enum device {}

#[cfg(feature = "CONFIG_QCOM_PBS")]
pub extern "C" fn qcom_pbs_trigger_event(pbs: *mut pbs_dev, bitmap: u8) -> i32;

#[cfg(feature = "CONFIG_QCOM_PBS")]
pub extern "C" fn get_pbs_client_device(client_dev: *mut device) -> *mut pbs_dev;

#[cfg(not(feature = "CONFIG_QCOM_PBS"))]
#[inline]
pub fn qcom_pbs_trigger_event(_pbs: *mut pbs_dev, _bitmap: u8) -> i32 {
	-ENODEV
}

#[cfg(not(feature = "CONFIG_QCOM_PBS"))]
#[inline]
pub fn get_pbs_client_device(_client_dev: *mut device) -> *mut pbs_dev {
	// Equivalent to the kernel ERR_PTR(-ENODEV) encoding.
	unsafe { core::mem::transmute::<isize, *mut pbs_dev>(-(ENODEV as isize)) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

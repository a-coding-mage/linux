/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Silicon Labs C2 port Linux support
 *
 *  Copyright (c) 2007 Rodolfo Giometti <giometti@linux.it>
 *  Copyright (c) 2007 Eurotech S.p.A. <info@eurotech.it>
 */

pub const C2PORT_NAME_LEN: usize = 32;

/* `struct device` and `struct mutex` are supplied by external dependencies. */
pub use crate::device;
pub use crate::mutex;

/* C2 port basic structs */

/* Main struct */
#[repr(C)]
pub struct c2port_device {
	/* C bit-fields: one-bit access flags, represented in their containing word. */
	pub access: u32,
	pub flash_access: u32,

	pub id: i32,
	pub name: [core::ffi::c_char; C2PORT_NAME_LEN],
	pub ops: *mut c2port_ops,
	pub mutex: mutex, /* prevent races during read/write */

	pub dev: *mut device,

	pub private_data: *mut core::ffi::c_void,
}

/* Basic operations */
#[repr(C)]
pub struct c2port_ops {
	/* Flash layout */
	pub block_size: u16, /* flash block size in bytes */
	pub blocks_num: u16, /* flash blocks number */

	/* Enable or disable the access to C2 port */
	pub access: Option<unsafe extern "C" fn(dev: *mut c2port_device, status: i32)>,

	/* Set C2D data line as input/output */
	pub c2d_dir: Option<unsafe extern "C" fn(dev: *mut c2port_device, dir: i32)>,

	/* Read/write C2D data line */
	pub c2d_get: Option<unsafe extern "C" fn(dev: *mut c2port_device) -> i32>,
	pub c2d_set: Option<unsafe extern "C" fn(dev: *mut c2port_device, status: i32)>,

	/* Write C2CK clock line */
	pub c2ck_set: Option<unsafe extern "C" fn(dev: *mut c2port_device, status: i32)>,
}

/* Exported functions */
unsafe extern "C" {
	pub fn c2port_device_register(
		name: *mut core::ffi::c_char,
		ops: *mut c2port_ops,
		devdata: *mut core::ffi::c_void,
	) -> *mut c2port_device;
	pub fn c2port_device_unregister(dev: *mut c2port_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

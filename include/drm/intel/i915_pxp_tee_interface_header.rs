/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2020 Intel Corporation
 */

use core::ffi::c_void;

/* Types supplied by the kernel interfaces included by the original header. */
pub struct module;
pub struct device;
pub struct scatterlist;
pub struct mutex;

/**
 * struct i915_pxp_component_ops - ops for PXP services.
 */
#[repr(C)]
pub struct i915_pxp_component_ops {
	/**
	 * @owner: Module providing the ops.
	 */
	pub owner: *mut module,

	/**
	 * @send: Send a PXP message.
	 */
	pub send: Option<unsafe extern "C" fn(
		dev: *mut device,
		message: *const c_void,
		size: usize,
		timeout_ms: usize,
	) -> i32>,
	/**
	 * @recv: Receive a PXP message.
	 */
	pub recv: Option<unsafe extern "C" fn(
		dev: *mut device,
		buffer: *mut c_void,
		size: usize,
		timeout_ms: usize,
	) -> i32>,
	/**
	 * @gsc_command: Send a GSC command.
	 */
	pub gsc_command: Option<unsafe extern "C" fn(
		dev: *mut device,
		client_id: u8,
		fence_id: u32,
		sg_in: *mut scatterlist,
		total_in_len: usize,
		sg_out: *mut scatterlist,
	) -> isize>,
}

/**
 * struct i915_pxp_component - Used for communication between i915 and TEE
 * drivers for the PXP services
 */
#[repr(C)]
pub struct i915_pxp_component {
	/**
	 * @tee_dev: device that provide the PXP service from TEE Bus.
	 */
	pub tee_dev: *mut device,

	/**
	 * @ops: Ops implemented by TEE driver, used by i915 driver.
	 */
	pub ops: *const i915_pxp_component_ops,

	/**
	 * @mutex: To protect the above members.
	 */
	pub mutex: mutex,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2026 Intel Corporation
 */

//! Rust translation of the C DRM RAS header.
//!
//! The C includes provide `drm_ras_node_type` and kernel API definitions;
//! those dependencies are intentionally left external to this translation.

use core::ffi::{c_char, c_int, c_void};

/**
 * struct drm_ras_node - A DRM RAS Node
 */
#[repr(C)]
pub struct drm_ras_node {
	/** @id: Unique identifier for the node. Dynamically assigned. */
	pub id: u32,
	/**
	 * @device_name: Human-readable name of the device. Given by the driver.
	 */
	pub device_name: *const c_char,
	/** @node_name: Human-readable name of the node. Given by the driver. */
	pub node_name: *const c_char,
	/** @type: Type of the node (enum drm_ras_node_type). */
	pub r#type: drm_ras_node_type,

	/* Error-Counter Related Callback and Variables */

	/** @error_counter_range: Range of valid Error IDs for this node. */
	pub error_counter_range: drm_ras_error_counter_range,

	/**
	 * @query_error_counter:
	 *
	 * This callback is used by drm-ras to query a specific error counter.
	 * Used for input check and to iterate all error counters in a node.
	 *
	 * Driver should expect query_error_counter() to be called with
	 * error_id from `error_counter_range.first` to
	 * `error_counter_range.last`.
	 *
	 * The @query_error_counter is a mandatory callback for
	 * error_counter_node.
	 *
	 * Returns: 0 on success,
	 *          -ENOENT when error_id is not supported as an indication that
	 *                  drm_ras should silently skip this entry. Used for
	 *                  supporting non-contiguous error ranges.
	 *                  Driver is responsible for maintaining the list of
	 *                  supported error IDs in the range of first to last.
	 *          Other negative values on errors that should terminate the
	 *                  netlink query.
	 */
	pub query_error_counter: Option<unsafe extern "C" fn(
		node: *mut drm_ras_node,
		error_id: u32,
		name: *mut *const c_char,
		val: *mut u32,
	) -> c_int>,

	/**
	 * @clear_error_counter:
	 *
	 * This callback is used by drm_ras to clear a specific error counter.
	 * Driver should implement this callback to support clearing error counters
	 * of a node.
	 *
	 * Returns: 0 on success, negative error code on failure.
	 */
	pub clear_error_counter:
		Option<unsafe extern "C" fn(node: *mut drm_ras_node, error_id: u32) -> c_int>,

	/** @priv: Driver private data */
	pub priv_: *mut c_void,
}

#[repr(C)]
pub struct drm_ras_error_counter_range {
	/** @first: First valid Error ID. */
	pub first: u32,
	/** @last: Last valid Error ID. Mandatory entry. */
	pub last: u32,
}

/* The C header's `struct drm_device;` forward declaration has no Rust item. */

/* `IS_ENABLED(CONFIG_DRM_RAS)` is a kernel build-time condition. */
#[cfg(feature = "CONFIG_DRM_RAS")]
unsafe extern "C" {
	pub fn drm_ras_node_register(node: *mut drm_ras_node) -> c_int;
	pub fn drm_ras_node_unregister(node: *mut drm_ras_node);
}

/* Fallback corresponding to the C `#else` branch. */
#[cfg(not(feature = "CONFIG_DRM_RAS"))]
#[inline]
pub unsafe fn drm_ras_node_register(_node: *mut drm_ras_node) -> c_int {
	0
}

#[cfg(not(feature = "CONFIG_DRM_RAS"))]
#[inline]
pub unsafe fn drm_ras_node_unregister(_node: *mut drm_ras_node) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

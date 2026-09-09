/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// use linux::lockdep_types::lockdep_map;
// use linux::types::{hlist_node, u32};

#[repr(C)]
pub struct timer_list {
	/*
	 * All fields that change during normal runtime grouped to the
	 * same cacheline
	 */
	pub entry: hlist_node,
	pub expires: ::core::ffi::c_ulong,
	pub function: Option<unsafe extern "C" fn(*mut timer_list)>,
	pub flags: u32,

	// Preserved from the CONFIG_LOCKDEP conditional build configuration.
	#[cfg(CONFIG_LOCKDEP)]
	pub lockdep_map: lockdep_map,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

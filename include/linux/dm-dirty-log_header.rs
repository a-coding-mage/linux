/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2003 Sistina Software
 * Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
 *
 * Device-Mapper dirty region log.
 *
 * This file is released under the LGPL.
 */

/* C header guards and the __KERNEL__ conditional are preserved by this file's
 * inclusion/build configuration. */

pub type region_t = sector_t;

pub struct dm_dirty_log_type;

#[repr(C)]
pub struct dm_dirty_log {
	pub type_: *mut dm_dirty_log_type,
	pub flush_callback_fn: Option<unsafe extern "C" fn(ti: *mut dm_target) -> ::core::ffi::c_int>,
	pub context: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct dm_dirty_log_type {
	pub name: *const ::core::ffi::c_char,
	pub module: *mut module,

	/* For internal device-mapper use */
	pub list: list_head,

	pub ctr: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		ti: *mut dm_target,
		argc: ::core::ffi::c_uint,
		argv: *mut *mut ::core::ffi::c_char,
	) -> ::core::ffi::c_int>,
	pub dtr: Option<unsafe extern "C" fn(log: *mut dm_dirty_log)>,

	/*
	 * There are times when we don't want the log to touch
	 * the disk.
	 */
	pub presuspend: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> ::core::ffi::c_int>,
	pub postsuspend: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> ::core::ffi::c_int>,
	pub resume: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> ::core::ffi::c_int>,

	/*
	 * Retrieves the smallest size of region that the log can
	 * deal with.
	 */
	pub get_region_size: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> u32>,

	/*
	 * A predicate to say whether a region is clean or not.
	 * May block.
	 */
	pub is_clean: Option<unsafe extern "C" fn(log: *mut dm_dirty_log, region: region_t) -> ::core::ffi::c_int>,

	/*
	 *  Returns: 0, 1, -EWOULDBLOCK, < 0
	 *
	 * A predicate function to check the area given by
	 * [sector, sector + len) is in sync.
	 *
	 * If -EWOULDBLOCK is returned the state of the region is
	 * unknown, typically this will result in a read being
	 * passed to a daemon to deal with, since a daemon is
	 * allowed to block.
	 */
	pub in_sync: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		region: region_t,
		can_block: ::core::ffi::c_int,
	) -> ::core::ffi::c_int>,

	/*
	 * Flush the current log state (eg, to disk).  This
	 * function may block.
	 */
	pub flush: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> ::core::ffi::c_int>,

	/*
	 * Mark an area as clean or dirty.  These functions may
	 * block, though for performance reasons blocking should
	 * be extremely rare (eg, allocating another chunk of
	 * memory for some reason).
	 */
	pub mark_region: Option<unsafe extern "C" fn(log: *mut dm_dirty_log, region: region_t)>,
	pub clear_region: Option<unsafe extern "C" fn(log: *mut dm_dirty_log, region: region_t)>,

	/*
	 * Returns: <0 (error), 0 (no region), 1 (region)
	 *
	 * The mirrord will need perform recovery on regions of
	 * the mirror that are in the NOSYNC state.  This
	 * function asks the log to tell the caller about the
	 * next region that this machine should recover.
	 *
	 * Do not confuse this function with 'in_sync()', one
	 * tells you if an area is synchronised, the other
	 * assigns recovery work.
	 */
	pub get_resync_work: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		region: *mut region_t,
	) -> ::core::ffi::c_int>,

	/*
	 * This notifies the log that the resync status of a region
	 * has changed.  It also clears the region from the recovering
	 * list (if present).
	 */
	pub set_region_sync: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		region: region_t,
		in_sync: ::core::ffi::c_int,
	)>,

	/*
	 * Returns the number of regions that are in sync.
	 */
	pub get_sync_count: Option<unsafe extern "C" fn(log: *mut dm_dirty_log) -> region_t>,

	/*
	 * Support function for mirror status requests.
	 */
	pub status: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		status_type: status_type_t,
		result: *mut ::core::ffi::c_char,
		maxlen: ::core::ffi::c_uint,
	) -> ::core::ffi::c_int>,

	/*
	 * is_remote_recovering is necessary for cluster mirroring. It provides
	 * a way to detect recovery on another node, so we aren't writing
	 * concurrently.  This function is likely to block (when a cluster log
	 * is used).
	 *
	 * Returns: 0, 1
	 */
	pub is_remote_recovering: Option<unsafe extern "C" fn(
		log: *mut dm_dirty_log,
		region: region_t,
	) -> ::core::ffi::c_int>,
}

unsafe extern "C" {
	pub fn dm_dirty_log_type_register(type_: *mut dm_dirty_log_type) -> ::core::ffi::c_int;
	pub fn dm_dirty_log_type_unregister(type_: *mut dm_dirty_log_type) -> ::core::ffi::c_int;

	/*
	 * Make sure you use these two functions, rather than calling
	 * type->constructor/destructor() directly.
	 */
	pub fn dm_dirty_log_create(
		type_name: *const ::core::ffi::c_char,
		ti: *mut dm_target,
		flush_callback_fn: Option<unsafe extern "C" fn(ti: *mut dm_target) -> ::core::ffi::c_int>,
		argc: ::core::ffi::c_uint,
		argv: *mut *mut ::core::ffi::c_char,
	) -> *mut dm_dirty_log;
	pub fn dm_dirty_log_destroy(log: *mut dm_dirty_log);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

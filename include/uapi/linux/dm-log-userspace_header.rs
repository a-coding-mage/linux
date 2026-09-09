/* SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2006-2009 Red Hat, Inc.
 *
 * This file is released under the LGPL.
 */

// Translated from dm-log-userspace.h. Linux type aliases and DM_UUID_LEN are
// supplied by the surrounding kernel-interface bindings.

pub const DM_ULOG_CTR: u32 = 1;
pub const DM_ULOG_DTR: u32 = 2;
pub const DM_ULOG_PRESUSPEND: u32 = 3;
pub const DM_ULOG_POSTSUSPEND: u32 = 4;
pub const DM_ULOG_RESUME: u32 = 5;
pub const DM_ULOG_GET_REGION_SIZE: u32 = 6;
pub const DM_ULOG_IS_CLEAN: u32 = 7;
pub const DM_ULOG_IN_SYNC: u32 = 8;
pub const DM_ULOG_FLUSH: u32 = 9;
pub const DM_ULOG_MARK_REGION: u32 = 10;
pub const DM_ULOG_CLEAR_REGION: u32 = 11;
pub const DM_ULOG_GET_RESYNC_WORK: u32 = 12;
pub const DM_ULOG_SET_REGION_SYNC: u32 = 13;
pub const DM_ULOG_GET_SYNC_COUNT: u32 = 14;
pub const DM_ULOG_STATUS_INFO: u32 = 15;
pub const DM_ULOG_STATUS_TABLE: u32 = 16;
pub const DM_ULOG_IS_REMOTE_RECOVERING: u32 = 17;

pub const DM_ULOG_REQUEST_MASK: u32 = 0xFF;

#[inline]
pub const fn DM_ULOG_REQUEST_TYPE(request_type: u32) -> u32 {
	DM_ULOG_REQUEST_MASK & request_type
}

/*
 * DM_ULOG_REQUEST_VERSION is incremented when there is a change to the way
 * information is passed between kernel and userspace. Version 1 was the
 * initial implementation; version 2 allowed DM_ULOG_CTR to return a device
 * name; version 3 added integrated flush payloads for marking regions.
 */
pub const DM_ULOG_REQUEST_VERSION: u32 = 3;

#[repr(C)]
pub struct dm_ulog_request {
	/*
	 * The local unique identifier (luid) and the universally unique
	 * identifier (uuid) tie a request to a specific mirror log. The uuid is
	 * required for node-to-node communication, while the luid differentiates
	 * logs being swapped with the same uuid.
	 */
	pub luid: __u64,
	pub uuid: [::core::ffi::c_char; DM_UUID_LEN],
	pub padding: [::core::ffi::c_char; 3],

	pub version: __u32,
	pub error: __s32,

	pub seq: __u32,
	pub request_type: __u32,
	pub data_size: __u32,

	pub data: [::core::ffi::c_char; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

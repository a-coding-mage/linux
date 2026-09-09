/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Cleanup widget for metadata inode creation and deletion. */
#[repr(C)]
pub struct xfs_metadir_update {
	/* Parent directory */
	pub dp: *mut xfs_inode,

	/* Path to metadata file */
	pub path: *const ::core::ffi::c_char,

	/* Parent pointer update context */
	pub ppargs: *mut xfs_parent_args,

	/* Child metadata file */
	pub ip: *mut xfs_inode,

	pub tp: *mut xfs_trans,

	pub metafile_type: xfs_metafile_type,

	/* C bit-fields: unsigned int dp_locked:1; */
	pub dp_locked: ::core::ffi::c_uint,
	/* C bit-fields: unsigned int ip_locked:1; */
	pub ip_locked: ::core::ffi::c_uint,
}

pub unsafe extern "C" fn xfs_metadir_load(
	tp: *mut xfs_trans,
	dp: *mut xfs_inode,
	path: *const ::core::ffi::c_char,
	metafile_type: xfs_metafile_type,
	ipp: *mut *mut xfs_inode,
) -> ::core::ffi::c_int;

pub type xfs_metadir_createfn = unsafe extern "C" fn(
	upd: *mut xfs_metadir_update,
	priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub unsafe extern "C" fn xfs_metadir_create_file(
	upd: *mut xfs_metadir_update,
	mode: umode_t,
	create: xfs_metadir_createfn,
	priv_: *mut ::core::ffi::c_void,
	ipp: *mut *mut xfs_inode,
) -> ::core::ffi::c_int;

pub unsafe extern "C" fn xfs_metadir_start_link(
	upd: *mut xfs_metadir_update,
) -> ::core::ffi::c_int;

pub unsafe extern "C" fn xfs_metadir_link(
	upd: *mut xfs_metadir_update,
) -> ::core::ffi::c_int;

pub unsafe extern "C" fn xfs_metadir_commit(
	upd: *mut xfs_metadir_update,
) -> ::core::ffi::c_int;

pub unsafe extern "C" fn xfs_metadir_mkdir(
	dp: *mut xfs_inode,
	path: *const ::core::ffi::c_char,
	ipp: *mut *mut xfs_inode,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

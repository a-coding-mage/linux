// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

// Dependencies supplied by the Linux kernel module and zstd headers.

/* Common symbols. zstd_compress must depend on zstd_decompress. */

pub unsafe fn zstd_is_error(code: usize) -> ::core::ffi::c_uint {
	ZSTD_isError(code)
}

pub unsafe fn zstd_get_error_code(code: usize) -> zstd_error_code {
	ZSTD_getErrorCode(code)
}

pub unsafe fn zstd_get_error_name(code: usize) -> *const ::core::ffi::c_char {
	ZSTD_getErrorName(code)
}

/* Decompression symbols. */

pub unsafe fn zstd_dctx_workspace_bound() -> usize {
	ZSTD_estimateDCtxSize()
}

pub unsafe fn zstd_create_dctx_advanced(custom_mem: zstd_custom_mem) -> *mut zstd_dctx {
	ZSTD_createDCtx_advanced(custom_mem)
}

pub unsafe fn zstd_free_dctx(dctx: *mut zstd_dctx) -> usize {
	ZSTD_freeDCtx(dctx)
}

pub unsafe fn zstd_create_ddict_byreference(
	dict: *const ::core::ffi::c_void,
	dict_size: usize,
	custom_mem: zstd_custom_mem,
) -> *mut zstd_ddict {
	ZSTD_createDDict_advanced(dict, dict_size, ZSTD_dlm_byRef, ZSTD_dct_auto, custom_mem)
}

pub unsafe fn zstd_free_ddict(ddict: *mut zstd_ddict) -> usize {
	ZSTD_freeDDict(ddict)
}

pub unsafe fn zstd_init_dctx(
	workspace: *mut ::core::ffi::c_void,
	workspace_size: usize,
) -> *mut zstd_dctx {
	if workspace.is_null() {
		return core::ptr::null_mut();
	}
	ZSTD_initStaticDCtx(workspace, workspace_size)
}

pub unsafe fn zstd_decompress_dctx(
	dctx: *mut zstd_dctx,
	dst: *mut ::core::ffi::c_void,
	dst_capacity: usize,
	src: *const ::core::ffi::c_void,
	src_size: usize,
) -> usize {
	ZSTD_decompressDCtx(dctx, dst, dst_capacity, src, src_size)
}

pub unsafe fn zstd_decompress_using_ddict(
	dctx: *mut zstd_dctx,
	dst: *mut ::core::ffi::c_void,
	dst_capacity: usize,
	src: *const ::core::ffi::c_void,
	src_size: usize,
	ddict: *const zstd_ddict,
) -> usize {
	ZSTD_decompress_usingDDict(dctx, dst, dst_capacity, src, src_size, ddict)
}

pub unsafe fn zstd_dstream_workspace_bound(max_window_size: usize) -> usize {
	ZSTD_estimateDStreamSize(max_window_size)
}

pub unsafe fn zstd_init_dstream(
	max_window_size: usize,
	workspace: *mut ::core::ffi::c_void,
	workspace_size: usize,
) -> *mut zstd_dstream {
	if workspace.is_null() {
		return core::ptr::null_mut();
	}
	let _ = max_window_size;
	ZSTD_initStaticDStream(workspace, workspace_size)
}

pub unsafe fn zstd_reset_dstream(dstream: *mut zstd_dstream) -> usize {
	ZSTD_DCtx_reset(dstream, ZSTD_reset_session_only)
}

pub unsafe fn zstd_decompress_stream(
	dstream: *mut zstd_dstream,
	output: *mut zstd_out_buffer,
	input: *mut zstd_in_buffer,
) -> usize {
	ZSTD_decompressStream(dstream, output, input)
}

pub unsafe fn zstd_find_frame_compressed_size(
	src: *const ::core::ffi::c_void,
	src_size: usize,
) -> usize {
	ZSTD_findFrameCompressedSize(src, src_size)
}

pub unsafe fn zstd_get_frame_header(
	header: *mut zstd_frame_header,
	src: *const ::core::ffi::c_void,
	src_size: usize,
) -> usize {
	ZSTD_getFrameHeader(header, src, src_size)
}

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("Zstd Decompressor");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

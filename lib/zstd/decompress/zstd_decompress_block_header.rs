/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/* Dependencies supplied by the surrounding translation unit:
 * size_t, ZSTD_DCtx, blockProperties_t, ZSTD_seqSymbol, U32, and U8.
 */

/* ===   Prototypes   === */

/* note: prototypes already published within `zstd.h` :
 * ZSTD_decompressBlock()
 */

/* note: prototypes already published within `zstd_internal.h` :
 * ZSTD_getcBlockSize()
 * ZSTD_decodeSeqHeaders()
 */

/* Streaming state is used to inform allocation of the literal buffer */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum streaming_operation {
    not_streaming = 0,
    is_streaming = 1,
}

/* ZSTD_decompressBlock_internal() :
 * decompress block, starting at `src`,
 * into destination buffer `dst`.
 * @return : decompressed block size,
 *           or an error code (which can be tested using ZSTD_isError())
 */
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    streaming: streaming_operation,
) -> size_t;

/* ZSTD_buildFSETable() :
 * generate FSE decoding table for one symbol (ll, ml or off)
 * this function must be called with valid parameters only
 * (dt is large enough, normalizedCounter distribution total is a power of 2, max is within range, etc.)
 * in which case it cannot fail.
 * The workspace must be 4-byte aligned and at least ZSTD_BUILD_FSE_TABLE_WKSP_SIZE bytes, which is
 * defined in zstd_decompress_internal.h.
 * Internal use only.
 */
pub unsafe extern "C" fn ZSTD_buildFSETable(
    dt: *mut ZSTD_seqSymbol,
    normalizedCounter: *const i16,
    maxSymbolValue: u32,
    baseValue: *const U32,
    nbAdditionalBits: *const U8,
    tableLog: u32,
    wksp: *mut core::ffi::c_void,
    wkspSize: size_t,
    bmi2: i32,
);

/* Internal definition of ZSTD_decompressBlock() to avoid deprecation warnings. */
pub unsafe extern "C" fn ZSTD_decompressBlock_deprecated(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

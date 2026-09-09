/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 * You may select, at your option, one of the above-listed licenses.
 */

/* Dependency: zstd_compress_internal.h (ZSTD_hufCTables_t, ZSTD_minGain()). */

extern "C" {
    pub fn ZSTD_noCompressLiterals(
        dst: *mut core::ffi::c_void,
        dst_capacity: usize,
        src: *const core::ffi::c_void,
        src_size: usize,
    ) -> usize;

    /* ZSTD_compressRleLiteralsBlock() :
     * Conditions :
     * - All bytes in @src are identical
     * - dstCapacity >= 4 */
    pub fn ZSTD_compressRleLiteralsBlock(
        dst: *mut core::ffi::c_void,
        dst_capacity: usize,
        src: *const core::ffi::c_void,
        src_size: usize,
    ) -> usize;

    /* ZSTD_compressLiterals():
     * @entropyWorkspace: must be aligned on 4-bytes boundaries
     * @entropyWorkspaceSize : must be >= HUF_WORKSPACE_SIZE
     * @suspectUncompressible: sampling checks, to potentially skip huffman coding
     */
    pub fn ZSTD_compressLiterals(
        dst: *mut core::ffi::c_void,
        dst_capacity: usize,
        src: *const core::ffi::c_void,
        src_size: usize,
        entropy_workspace: *mut core::ffi::c_void,
        entropy_workspace_size: usize,
        prev_huf: *const ZSTD_hufCTables_t,
        next_huf: *mut ZSTD_hufCTables_t,
        strategy: ZSTD_strategy,
        disable_literal_compression: i32,
        suspect_uncompressible: i32,
        bmi2: i32,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

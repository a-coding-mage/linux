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

/* Dependency: ZSTD_CCtx is supplied by linux/zstd.h. */

/*-*************************************
*  Target Compressed Block Size
***************************************/

/* ZSTD_compressSuperBlock() :
 * Used to compress a super block when targetCBlockSize is being used.
 * The given block will be compressed into multiple sub blocks that are around targetCBlockSize. */
extern "C" {
    pub fn ZSTD_compressSuperBlock(
        zc: *mut ZSTD_CCtx,
        dst: *mut core::ffi::c_void,
        dstCapacity: usize,
        src: *const core::ffi::c_void,
        srcSize: usize,
        lastBlock: core::ffi::c_uint,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

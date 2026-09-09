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

// C header dependency: <linux/types.h> supplies size_t.

pub const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;

/* ZSTD_splitBlock():
 * @level must be a value between 0 and 4.
 *        higher levels spend more energy to detect block boundaries.
 * @workspace must be aligned for size_t.
 * @wkspSize must be at least >= ZSTD_SLIPBLOCK_WORKSPACESIZE
 * note:
 * For the time being, this function only accepts full 128 KB blocks.
 * Therefore, @blockSize must be == 128 KB.
 * While this could be extended to smaller sizes in the future,
 * it is not yet clear if this would be useful. TBD.
 */
extern "C" {
    pub fn ZSTD_splitBlock(
        blockStart: *const core::ffi::c_void,
        blockSize: usize,
        level: i32,
        workspace: *mut core::ffi::c_void,
        wkspSize: usize,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

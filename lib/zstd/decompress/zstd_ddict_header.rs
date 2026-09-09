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

/*
 * Dependencies:
 * ../common/zstd_deps.h supplies size_t.
 * <linux/zstd.h> supplies ZSTD_DDict, ZSTD_DCtx, and public functions.
 */

/*-*******************************************************
 *  Interface
 *********************************************************/

/* note: several prototypes are already published in `zstd.h` :
 * ZSTD_createDDict()
 * ZSTD_createDDict_byReference()
 * ZSTD_createDDict_advanced()
 * ZSTD_freeDDict()
 * ZSTD_initStaticDDict()
 * ZSTD_sizeof_DDict()
 * ZSTD_estimateDDictSize()
 * ZSTD_getDictID_fromDict()
 */

unsafe extern "C" {
    pub fn ZSTD_DDict_dictContent(ddict: *const ZSTD_DDict) -> *const core::ffi::c_void;
    pub fn ZSTD_DDict_dictSize(ddict: *const ZSTD_DDict) -> usize;

    pub fn ZSTD_copyDDictParameters(dctx: *mut ZSTD_DCtx, ddict: *const ZSTD_DDict);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

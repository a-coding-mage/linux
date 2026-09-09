// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 * You may select, at your option, one of the above-listed licenses.
 */

// Dependencies: error_private.h and zstd_internal.h provide the referenced
// error helpers, constants, and types in the surrounding translation unit.

use std::os::raw::c_char;

/*-****************************************
*  Version
******************************************/
pub unsafe fn ZSTD_versionNumber() -> u32 {
    ZSTD_VERSION_NUMBER
}

pub unsafe fn ZSTD_versionString() -> *const c_char {
    ZSTD_VERSION_STRING
}

/*-****************************************
*  ZSTD Error Management
******************************************/
/*!
 *  ZSTD_isError() :
 *  tells if a return value is an error code
 *  symbol is required for external callers
 */
pub unsafe fn ZSTD_isError(code: usize) -> u32 {
    ERR_isError(code)
}

/*!
 *  ZSTD_getErrorName() :
 *  provides error code string from function result (useful for debugging)
 */
pub unsafe fn ZSTD_getErrorName(code: usize) -> *const c_char {
    ERR_getErrorName(code)
}

/*!
 *  ZSTD_getError() :
 *  convert a `size_t` function result into a proper ZSTD_errorCode enum
 */
pub unsafe fn ZSTD_getErrorCode(code: usize) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}

/*!
 *  ZSTD_getErrorString() :
 *  provides error code string from enum
 */
pub unsafe fn ZSTD_getErrorString(code: ZSTD_ErrorCode) -> *const c_char {
    ERR_getErrorString(code)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

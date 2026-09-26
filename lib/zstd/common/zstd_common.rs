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

/*-*************************************
*  Dependencies
***************************************/
// ZSTD_DEPS_NEED_MALLOC; dependencies: error_private.h, zstd_internal.h.

use core::ffi::{c_char, c_uint};

/*-****************************************
*  Version
******************************************/
#[no_mangle]
pub extern "C" fn ZSTD_versionNumber() -> c_uint {
    ZSTD_VERSION_NUMBER
}

#[no_mangle]
pub extern "C" fn ZSTD_versionString() -> *const c_char {
    ZSTD_VERSION_STRING.as_ptr()
}

/*-****************************************
*  ZSTD Error Management
******************************************/
// `ZSTD_isError` is also a macro within zstd_internal.h; this is the symbol.
/* ZSTD_isError() :
 *  tells if a return value is an error code
 *  symbol is required for external callers */
#[no_mangle]
pub extern "C" fn ZSTD_isError(code: usize) -> c_uint {
    ERR_isError(code)
}

/* ZSTD_getErrorName() :
 *  provides error code string from function result (useful for debugging) */
#[no_mangle]
pub extern "C" fn ZSTD_getErrorName(code: usize) -> *const c_char {
    ERR_getErrorName(code)
}

/* ZSTD_getError() :
 *  convert a `size_t` function result into a proper ZSTD_errorCode enum */
#[no_mangle]
pub extern "C" fn ZSTD_getErrorCode(code: usize) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}

/* ZSTD_getErrorString() :
 *  provides error code string from enum */
#[no_mangle]
pub extern "C" fn ZSTD_getErrorString(code: ZSTD_ErrorCode) -> *const c_char {
    ERR_getErrorString(code)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

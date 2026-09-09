/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

/* This file provides custom allocation primitives. */

/* C dependencies: zstd_deps.h, compiler.h, and linux/zstd.h. */

use core::ffi::c_void;

pub type size_t = usize;

#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: Option<unsafe extern "C" fn(opaque: *mut c_void, size: size_t) -> *mut c_void>,
    pub customFree: Option<unsafe extern "C" fn(opaque: *mut c_void, address: *mut c_void)>,
    pub opaque: *mut c_void,
}

extern "C" {
    pub fn ZSTD_malloc(size: size_t) -> *mut c_void;
    pub fn ZSTD_calloc(n: size_t, size: size_t) -> *mut c_void;
    pub fn ZSTD_free(ptr: *mut c_void);
    pub fn ZSTD_memset(dst: *mut c_void, value: i32, size: size_t) -> *mut c_void;
}

/* custom memory allocation functions */

#[inline]
pub unsafe fn ZSTD_customMalloc(size: size_t, customMem: ZSTD_customMem) -> *mut c_void {
    if let Some(customAlloc) = customMem.customAlloc {
        return customAlloc(customMem.opaque, size);
    }
    ZSTD_malloc(size)
}

#[inline]
pub unsafe fn ZSTD_customCalloc(size: size_t, customMem: ZSTD_customMem) -> *mut c_void {
    if let Some(customAlloc) = customMem.customAlloc {
        /* calloc implemented as malloc+memset;
         * not as efficient as calloc, but next best guess for custom malloc */
        let ptr = customAlloc(customMem.opaque, size);
        ZSTD_memset(ptr, 0, size);
        return ptr;
    }
    ZSTD_calloc(1, size)
}

#[inline]
pub unsafe fn ZSTD_customFree(ptr: *mut c_void, customMem: ZSTD_customMem) {
    if !ptr.is_null() {
        if let Some(customFree) = customMem.customFree {
            customFree(customMem.opaque, ptr);
        } else {
            ZSTD_free(ptr);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

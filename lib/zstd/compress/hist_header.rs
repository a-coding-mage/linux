/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* ******************************************************************
 * hist : Histogram functions
 * part of Finite State Entropy project
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 *  You can contact the author at :
 *  - FSE source repository : https://github.com/Cyan4973/FiniteStateEntropy
 *  - Public forum : https://groups.google.com/forum/#!forum/lz4c
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
****************************************************************** */

/* size_t and void* equivalents are represented by Rust's usize and raw pointers. */

/* --- simple histogram functions --- */

/*! HIST_count():
 *  Provides the precise count of each byte within a table 'count'.
 * 'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
 *  Updates *maxSymbolValuePtr with actual largest symbol value detected.
 * @return : count of the most frequent symbol (which isn't identified).
 *           or an error code, which can be tested using HIST_isError().
 *           note : if return == srcSize, there is only one symbol.
 */
extern "C" {
    pub fn HIST_count(
        count: *mut u32,
        maxSymbolValuePtr: *mut u32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> usize;

    pub fn HIST_isError(code: usize) -> u32; /*< tells if a return value is an error code */

    /* --- advanced histogram functions --- */

    pub fn HIST_count_wksp(
        count: *mut u32,
        maxSymbolValuePtr: *mut u32,
        src: *const core::ffi::c_void,
        srcSize: usize,
        workSpace: *mut core::ffi::c_void,
        workSpaceSize: usize,
    ) -> usize;

    pub fn HIST_countFast(
        count: *mut u32,
        maxSymbolValuePtr: *mut u32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> usize;

    pub fn HIST_countFast_wksp(
        count: *mut u32,
        maxSymbolValuePtr: *mut u32,
        src: *const core::ffi::c_void,
        srcSize: usize,
        workSpace: *mut core::ffi::c_void,
        workSpaceSize: usize,
    ) -> usize;

    pub fn HIST_count_simple(
        count: *mut u32,
        maxSymbolValuePtr: *mut u32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> u32;

    pub fn HIST_add(count: *mut u32, src: *const core::ffi::c_void, srcSize: usize);
}

pub const HIST_WKSP_SIZE_U32: usize = 1024;
pub const HIST_WKSP_SIZE: usize = HIST_WKSP_SIZE_U32 * core::mem::size_of::<u32>();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

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

// C header guard: ZSTD_DOUBLE_FAST_H
// Dependencies supplied by the surrounding translation unit include U32,
// ZSTD_MatchState_t, SeqStore_t, ZSTD_REP_NUM,
// ZSTD_dictTableLoadMethod_e, and ZSTD_tableFillPurpose_e.

#[cfg(not(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR"))]
extern "C" {
    pub fn ZSTD_fillDoubleHashTable(
        ms: *mut ZSTD_MatchState_t,
        end: *const core::ffi::c_void,
        dtlm: ZSTD_dictTableLoadMethod_e,
        tfp: ZSTD_tableFillPurpose_e,
    );

    pub fn ZSTD_compressBlock_doubleFast(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut U32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> usize;

    pub fn ZSTD_compressBlock_doubleFast_dictMatchState(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut U32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> usize;

    pub fn ZSTD_compressBlock_doubleFast_extDict(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut U32,
        src: *const core::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}

// ZSTD_COMPRESSBLOCK_DOUBLEFAST and related C macros select the corresponding
// function when the double-fast block compressor is enabled, and NULL when it
// is excluded. Rust function-pointer constants preserve that selection.
#[cfg(not(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR"))]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> =
    Some(ZSTD_compressBlock_doubleFast);

#[cfg(not(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR"))]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> =
    Some(ZSTD_compressBlock_doubleFast_dictMatchState);

#[cfg(not(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR"))]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> =
    Some(ZSTD_compressBlock_doubleFast_extDict);

#[cfg(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR")]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> = None;

#[cfg(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR")]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> = None;

#[cfg(feature = "ZSTD_EXCLUDE_DFAST_BLOCK_COMPRESSOR")]
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT:
    Option<unsafe extern "C" fn(*mut ZSTD_MatchState_t, *mut SeqStore_t, *mut U32, *const core::ffi::c_void, usize) -> usize> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

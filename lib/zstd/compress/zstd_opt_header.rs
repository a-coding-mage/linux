/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

// Dependency declarations supplied by zstd_compress_internal.h are external
// to this translated header.

#[cfg(any(
    not(feature = "ZSTD_EXCLUDE_BTLAZY2_BLOCK_COMPRESSOR"),
    not(feature = "ZSTD_EXCLUDE_BTOPT_BLOCK_COMPRESSOR"),
    not(feature = "ZSTD_EXCLUDE_BTULTRA_BLOCK_COMPRESSOR")
))]
extern "C" {
    /* used in ZSTD_loadDictionaryContent() */
    pub fn ZSTD_updateTree(ms: *mut ZSTD_MatchState_t, ip: *const BYTE, iend: *const BYTE);
}

#[cfg(not(feature = "ZSTD_EXCLUDE_BTOPT_BLOCK_COMPRESSOR"))]
extern "C" {
    pub fn ZSTD_compressBlock_btopt(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
    pub fn ZSTD_compressBlock_btopt_dictMatchState(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
    pub fn ZSTD_compressBlock_btopt_extDict(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
}

#[cfg(not(feature = "ZSTD_EXCLUDE_BTULTRA_BLOCK_COMPRESSOR"))]
extern "C" {
    pub fn ZSTD_compressBlock_btultra(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
    pub fn ZSTD_compressBlock_btultra_dictMatchState(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
    pub fn ZSTD_compressBlock_btultra_extDict(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;

    /* note: no btultra2 variant for extDict nor dictMatchState,
     * because btultra2 is not meant to work with dictionaries
     * and is only specific for the first block (no prefix) */
    pub fn ZSTD_compressBlock_btultra2(
        ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut U32,
        src: *const core::ffi::c_void, srcSize: usize,
    ) -> usize;
}

// C preprocessor aliases:
// ZSTD_COMPRESSBLOCK_BTOPT                  -> ZSTD_compressBlock_btopt
// ZSTD_COMPRESSBLOCK_BTOPT_DICTMATCHSTATE   -> ZSTD_compressBlock_btopt_dictMatchState
// ZSTD_COMPRESSBLOCK_BTOPT_EXTDICT          -> ZSTD_compressBlock_btopt_extDict
// ZSTD_COMPRESSBLOCK_BTULTRA                -> ZSTD_compressBlock_btultra
// ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE -> ZSTD_compressBlock_btultra_dictMatchState
// ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT        -> ZSTD_compressBlock_btultra_extDict
// ZSTD_COMPRESSBLOCK_BTULTRA2               -> ZSTD_compressBlock_btultra2
// When the corresponding compressor is excluded, each C alias expands to NULL.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

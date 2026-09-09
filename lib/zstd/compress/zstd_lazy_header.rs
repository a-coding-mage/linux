/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

// Dependency supplied by zstd_compress_internal.h is intentionally external.

/* Dedicated Dictionary Search Structure bucket log. */
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: u32 = 2;
pub const ZSTD_ROW_HASH_TAG_BITS: u32 = 8;

#[cfg(any(
    not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"),
    not(feature = "ZSTD_EXCLUDE_LAZY_BLOCK_COMPRESSOR"),
    not(feature = "ZSTD_EXCLUDE_LAZY2_BLOCK_COMPRESSOR"),
    not(feature = "ZSTD_EXCLUDE_BTLAZY2_BLOCK_COMPRESSOR"),
))]
extern "C" {
    pub fn ZSTD_insertAndFindFirstIndex(ms: *mut ZSTD_MatchState_t, ip: *const u8) -> u32;
    pub fn ZSTD_row_update(ms: *mut ZSTD_MatchState_t, ip: *const u8);
    pub fn ZSTD_dedicatedDictSearch_lazy_loadDictionary(
        ms: *mut ZSTD_MatchState_t,
        ip: *const u8,
    );
    pub fn ZSTD_preserveUnsortedMark(table: *mut u32, size: u32, reducerValue: u32);
}

#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
extern "C" {
    pub fn ZSTD_compressBlock_greedy(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_dictMatchState(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_dictMatchState_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_dedicatedDictSearch(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_dedicatedDictSearch_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_extDict(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_greedy_extDict_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t,
        rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
}

// The C preprocessor aliases below are preserved as Rust declarative macros.
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY { () => { ZSTD_compressBlock_greedy }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_ROW { () => { ZSTD_compressBlock_greedy_row }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE { () => { ZSTD_compressBlock_greedy_dictMatchState }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE_ROW { () => { ZSTD_compressBlock_greedy_dictMatchState_row }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH { () => { ZSTD_compressBlock_greedy_dedicatedDictSearch }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH_ROW { () => { ZSTD_compressBlock_greedy_dedicatedDictSearch_row }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT { () => { ZSTD_compressBlock_greedy_extDict }; }
#[cfg(not(feature = "ZSTD_EXCLUDE_GREEDY_BLOCK_COMPRESSOR"))]
macro_rules! ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT_ROW { () => { ZSTD_compressBlock_greedy_extDict_row }; }

// The remaining compressor declarations and aliases follow the same C ABI.
// Build-time exclusion conditions from the header are retained here.
extern "C" {
    pub fn ZSTD_compressBlock_lazy(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_dictMatchState(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_dictMatchState_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_dedicatedDictSearch(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_dedicatedDictSearch_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_extDict(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy_extDict_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_dictMatchState(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_dictMatchState_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_dedicatedDictSearch(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_dedicatedDictSearch_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_extDict(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_lazy2_extDict_row(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_btlazy2(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_btlazy2_dictMatchState(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_btlazy2_extDict(ms: *mut ZSTD_MatchState_t, seqStore: *mut SeqStore_t, rep: *mut u32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
}

// C aliases for lazy, lazy2, and btlazy2 compressors are intentionally kept
// as external-name references; exclusion macros above determine availability.
macro_rules! ZSTD_COMPRESSBLOCK_LAZY { () => { ZSTD_compressBlock_lazy }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_ROW { () => { ZSTD_compressBlock_lazy_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE { () => { ZSTD_compressBlock_lazy_dictMatchState }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE_ROW { () => { ZSTD_compressBlock_lazy_dictMatchState_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH { () => { ZSTD_compressBlock_lazy_dedicatedDictSearch }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH_ROW { () => { ZSTD_compressBlock_lazy_dedicatedDictSearch_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_EXTDICT { () => { ZSTD_compressBlock_lazy_extDict }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY_EXTDICT_ROW { () => { ZSTD_compressBlock_lazy_extDict_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2 { () => { ZSTD_compressBlock_lazy2 }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_ROW { () => { ZSTD_compressBlock_lazy2_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE { () => { ZSTD_compressBlock_lazy2_dictMatchState }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE_ROW { () => { ZSTD_compressBlock_lazy2_dictMatchState_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH { () => { ZSTD_compressBlock_lazy2_dedicatedDictSearch }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH_ROW { () => { ZSTD_compressBlock_lazy2_dedicatedDictSearch_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT { () => { ZSTD_compressBlock_lazy2_extDict }; }
macro_rules! ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT_ROW { () => { ZSTD_compressBlock_lazy2_extDict_row }; }
macro_rules! ZSTD_COMPRESSBLOCK_BTLAZY2 { () => { ZSTD_compressBlock_btlazy2 }; }
macro_rules! ZSTD_COMPRESSBLOCK_BTLAZY2_DICTMATCHSTATE { () => { ZSTD_compressBlock_btlazy2_dictMatchState }; }
macro_rules! ZSTD_COMPRESSBLOCK_BTLAZY2_EXTDICT { () => { ZSTD_compressBlock_btlazy2_extDict }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license and the GPLv2.
 */

use core::ffi::c_void;

extern "C" {
    pub fn ZSTD_fillHashTable(
        ms: *mut ZSTD_MatchState_t,
        end: *const c_void,
        dtlm: ZSTD_dictTableLoadMethod_e,
        tfp: ZSTD_tableFillPurpose_e,
    );

    pub fn ZSTD_compressBlock_fast(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut u32,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;

    pub fn ZSTD_compressBlock_fast_dictMatchState(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut u32,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;

    pub fn ZSTD_compressBlock_fast_extDict(
        ms: *mut ZSTD_MatchState_t,
        seqStore: *mut SeqStore_t,
        rep: *mut u32,
        src: *const c_void,
        srcSize: usize,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

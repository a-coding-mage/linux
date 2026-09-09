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

// Dependencies supplied by the surrounding translation unit:
// zstd_compress_internal.h: SeqDef
// ../common/fse.h: FSE_repeat, FSE_CTable
// ../common/zstd_internal.h: SymbolEncodingType_e, ZSTD_strategy

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZSTD_DefaultPolicy_e {
    ZSTD_defaultDisallowed = 0,
    ZSTD_defaultAllowed = 1,
}

extern "C" {
    pub fn ZSTD_selectEncodingType(
        repeatMode: *mut FSE_repeat,
        count: *const ::std::os::raw::c_uint,
        max: ::std::os::raw::c_uint,
        mostFrequent: usize,
        nbSeq: usize,
        FSELog: ::std::os::raw::c_uint,
        prevCTable: *const FSE_CTable,
        defaultNorm: *const ::std::os::raw::c_short,
        defaultNormLog: U32,
        isDefaultAllowed: ZSTD_DefaultPolicy_e,
        strategy: ZSTD_strategy,
    ) -> SymbolEncodingType_e;

    pub fn ZSTD_buildCTable(
        dst: *mut ::std::ffi::c_void,
        dstCapacity: usize,
        nextCTable: *mut FSE_CTable,
        FSELog: U32,
        type_: SymbolEncodingType_e,
        count: *mut ::std::os::raw::c_uint,
        max: U32,
        codeTable: *const BYTE,
        nbSeq: usize,
        defaultNorm: *const S16,
        defaultNormLog: U32,
        defaultMax: U32,
        prevCTable: *const FSE_CTable,
        prevCTableSize: usize,
        entropyWorkspace: *mut ::std::ffi::c_void,
        entropyWorkspaceSize: usize,
    ) -> usize;

    pub fn ZSTD_encodeSequences(
        dst: *mut ::std::ffi::c_void,
        dstCapacity: usize,
        CTable_MatchLength: *const FSE_CTable,
        mlCodeTable: *const BYTE,
        CTable_OffsetBits: *const FSE_CTable,
        ofCodeTable: *const BYTE,
        CTable_LitLength: *const FSE_CTable,
        llCodeTable: *const BYTE,
        sequences: *const SeqDef,
        nbSeq: usize,
        longOffsets: ::std::os::raw::c_int,
        bmi2: ::std::os::raw::c_int,
    ) -> usize;

    pub fn ZSTD_fseBitCost(
        ctable: *const FSE_CTable,
        count: *const ::std::os::raw::c_uint,
        max: ::std::os::raw::c_uint,
    ) -> usize;

    pub fn ZSTD_crossEntropyCost(
        norm: *const ::std::os::raw::c_short,
        accuracyLog: ::std::os::raw::c_uint,
        count: *const ::std::os::raw::c_uint,
        max: ::std::os::raw::c_uint,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

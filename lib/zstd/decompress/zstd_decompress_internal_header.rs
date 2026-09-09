/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root of this source tree and the GPLv2 found in the
 * COPYING file in the root of this source tree.
 */

// Dependencies supplied by the surrounding translation unit provide BYTE, U8,
// U16, U32, U64, S16, size_t, and the zstd/huffman types and constants.

pub static LL_base: [U32; (MaxLL + 1) as usize] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 18, 20, 22, 24, 28, 32, 40, 48, 64, 0x80, 0x100, 0x200, 0x400,
    0x800, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
];

pub static OF_base: [U32; (MaxOff + 1) as usize] = [
    0, 1, 1, 5, 0xD, 0x1D, 0x3D, 0x7D, 0xFD, 0x1FD, 0x3FD, 0x7FD,
    0xFFD, 0x1FFD, 0x3FFD, 0x7FFD, 0xFFFD, 0x1FFFD, 0x3FFFD, 0x7FFFD,
    0xFFFFD, 0x1FFFFD, 0x3FFFFD, 0x7FFFFD, 0xFFFFFD, 0x1FFFFFD, 0x3FFFFFD,
    0x7FFFFFD, 0xFFFFFFD, 0x1FFFFFFD, 0x3FFFFFFD, 0x7FFFFFFD,
];

pub static OF_bits: [U8; (MaxOff + 1) as usize] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
];

pub static ML_base: [U32; (MaxML + 1) as usize] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 37, 39, 41, 43, 47, 51, 59, 67, 83, 99, 0x83, 0x103, 0x203,
    0x403, 0x803, 0x1003, 0x2003, 0x4003, 0x8003, 0x10003,
];

#[repr(C)]
pub struct ZSTD_seqSymbol_header { pub fastMode: U32, pub tableLog: U32 }

#[repr(C)]
pub struct ZSTD_seqSymbol { pub nextState: U16, pub nbAdditionalBits: BYTE, pub nbBits: BYTE, pub baseValue: U32 }

// #define SEQSYMBOL_TABLE_SIZE(log) (1 + (1 << (log)))
pub const ZSTD_HUFFDTABLE_CAPACITY_LOG: u32 = 12;
pub const ZSTD_BUILD_FSE_TABLE_WKSP_SIZE: usize = core::mem::size_of::<S16>() * (MaxSeq as usize + 1) + (1usize << MaxFSELog) + core::mem::size_of::<U64>();
pub const ZSTD_BUILD_FSE_TABLE_WKSP_SIZE_U32: usize = (ZSTD_BUILD_FSE_TABLE_WKSP_SIZE + core::mem::size_of::<U32>() - 1) / core::mem::size_of::<U32>();

#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; (1 << LLFSELog) + 1],
    pub OFTable: [ZSTD_seqSymbol; (1 << OffFSELog) + 1],
    pub MLTable: [ZSTD_seqSymbol; (1 << MLFSELog) + 1],
    pub hufTable: [HUF_DTable; 1 << ZSTD_HUFFDTABLE_CAPACITY_LOG],
    pub rep: [U32; ZSTD_REP_NUM as usize],
    pub workspace: [U32; ZSTD_BUILD_FSE_TABLE_WKSP_SIZE_U32],
}

#[repr(C)]
pub enum ZSTD_dStage { ZSTDds_getFrameHeaderSize, ZSTDds_decodeFrameHeader, ZSTDds_decodeBlockHeader, ZSTDds_decompressBlock, ZSTDds_decompressLastBlock, ZSTDds_checkChecksum, ZSTDds_decodeSkippableHeader, ZSTDds_skipFrame }
#[repr(C)]
pub enum ZSTD_dStreamStage { zdss_init = 0, zdss_loadHeader, zdss_read, zdss_load, zdss_flush }
#[repr(C)]
pub enum ZSTD_dictUses_e { ZSTD_use_indefinitely = -1, ZSTD_dont_use = 0, ZSTD_use_once = 1 }

#[repr(C)]
pub struct ZSTD_DDictHashSet { pub ddictPtrTable: *const *const ZSTD_DDict, pub ddictPtrTableSize: size_t, pub ddictPtrCount: size_t }

pub const ZSTD_DECODER_INTERNAL_BUFFER: usize = 1 << 16;
pub const ZSTD_LBMIN: usize = 64;
pub const ZSTD_LBMAX: usize = 128 << 10;
pub const ZSTD_LITBUFFEREXTRASIZE: usize = ZSTD_DECODER_INTERNAL_BUFFER;

#[repr(C)]
pub enum ZSTD_litLocation_e { ZSTD_not_in_dst = 0, ZSTD_in_dst = 1, ZSTD_split = 2 }

#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol, pub MLTptr: *const ZSTD_seqSymbol, pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable, pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [U32; HUF_DECOMPRESS_WORKSPACE_SIZE_U32],
    pub previousDstEnd: *const core::ffi::c_void, pub prefixStart: *const core::ffi::c_void,
    pub virtualStart: *const core::ffi::c_void, pub dictEnd: *const core::ffi::c_void,
    pub expected: size_t, pub fParams: ZSTD_FrameHeader, pub processedCSize: U64, pub decodedSize: U64,
    pub bType: blockType_e, pub stage: ZSTD_dStage, pub litEntropy: U32, pub fseEntropy: U32,
    pub xxhState: xxh64_state, pub headerSize: size_t, pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e, pub validateChecksum: U32,
    pub litPtr: *const BYTE, pub customMem: ZSTD_customMem, pub litSize: size_t, pub rleSize: size_t,
    pub staticSize: size_t, pub isFrameDecompression: core::ffi::c_int,
    #[cfg(DYNAMIC_BMI2)] pub bmi2: core::ffi::c_int,
    pub ddictLocal: *mut ZSTD_DDict, pub ddict: *const ZSTD_DDict, pub dictID: U32,
    pub ddictIsCold: core::ffi::c_int, pub dictUses: ZSTD_dictUses_e, pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e, pub disableHufAsm: core::ffi::c_int,
    pub maxBlockSizeParam: core::ffi::c_int, pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut core::ffi::c_char, pub inBuffSize: size_t, pub inPos: size_t, pub maxWindowSize: size_t,
    pub outBuff: *mut core::ffi::c_char, pub outBuffSize: size_t, pub outStart: size_t, pub outEnd: size_t,
    pub lhSize: size_t, pub hostageByte: U32, pub noForwardProgress: core::ffi::c_int,
    pub outBufferMode: ZSTD_bufferMode_e, pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut BYTE, pub litBufferEnd: *const BYTE, pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [BYTE; ZSTD_LITBUFFEREXTRASIZE + WILDCOPY_OVERLENGTH],
    pub headerBuffer: [BYTE; ZSTD_FRAMEHEADERSIZE_MAX], pub oversizedDuration: size_t,
    #[cfg(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)] pub dictContentBeginForFuzzing: *const core::ffi::c_void,
    #[cfg(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)] pub dictContentEndForFuzzing: *const core::ffi::c_void,
}

pub unsafe fn ZSTD_DCtx_get_bmi2(dctx: *const ZSTD_DCtx_s) -> core::ffi::c_int {
    #[cfg(DYNAMIC_BMI2)] { (*dctx).bmi2 }
    #[cfg(not(DYNAMIC_BMI2))] { let _ = dctx; 0 }
}

extern "C" {
    pub fn ZSTD_loadDEntropy(entropy: *mut ZSTD_entropyDTables_t, dict: *const core::ffi::c_void, dictSize: size_t) -> size_t;
    pub fn ZSTD_checkContinuity(dctx: *mut ZSTD_DCtx_s, dst: *const core::ffi::c_void, dstSize: size_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

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

// Dependency declarations are supplied by the surrounding translation unit.

#[cfg(DEBUGLEVEL >= 2)]
unsafe fn showHexa(src: *const core::ffi::c_void, srcSize: usize) -> usize {
    let ip = src as *const BYTE;
    let mut u: usize = 0;
    while u < srcSize {
        RAWLOG!(5, " %02X", *ip.add(u));
        let _ = ip;
        u += 1;
    }
    RAWLOG!(5, " \n");
    srcSize
}

unsafe fn ZSTD_noCompressLiterals(
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> usize {
    let ostart = dst as *mut BYTE;
    let flSize: U32 = 1 + (srcSize > 31) as U32 + (srcSize > 4095) as U32;

    DEBUGLOG!(5, "ZSTD_noCompressLiterals: srcSize=%zu, dstCapacity=%zu", srcSize, dstCapacity);
    RETURN_ERROR_IF!(srcSize + flSize as usize > dstCapacity, dstSize_tooSmall, "");

    match flSize {
        1 => {
            *ostart = (set_basic as U32).wrapping_add((srcSize as U32) << 3) as BYTE;
        }
        2 => {
            MEM_writeLE16(ostart, (set_basic as U32).wrapping_add(1 << 2).wrapping_add((srcSize as U32) << 4) as U16);
        }
        3 => {
            MEM_writeLE32(ostart, (set_basic as U32).wrapping_add(3 << 2).wrapping_add((srcSize as U32) << 4));
        }
        _ => assert!(false),
    }

    ZSTD_memcpy(ostart.add(flSize as usize) as *mut core::ffi::c_void, src, srcSize);
    DEBUGLOG!(5, "Raw (uncompressed) literals: %u -> %u", srcSize as U32, (srcSize + flSize as usize) as U32);
    srcSize + flSize as usize
}

unsafe fn allBytesIdentical(src: *const core::ffi::c_void, srcSize: usize) -> i32 {
    assert!(srcSize >= 1);
    assert!(!src.is_null());
    let b = *(src as *const BYTE);
    let mut p: usize = 1;
    while p < srcSize {
        if *((src as *const BYTE).add(p)) != b {
            return 0;
        }
        p += 1;
    }
    1
}

unsafe fn ZSTD_compressRleLiteralsBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> usize {
    let ostart = dst as *mut BYTE;
    let flSize: U32 = 1 + (srcSize > 31) as U32 + (srcSize > 4095) as U32;

    assert!(dstCapacity >= 4);
    let _ = dstCapacity;
    assert!(allBytesIdentical(src, srcSize) != 0);

    match flSize {
        1 => *ostart = (set_rle as U32).wrapping_add((srcSize as U32) << 3) as BYTE,
        2 => MEM_writeLE16(ostart, (set_rle as U32).wrapping_add(1 << 2).wrapping_add((srcSize as U32) << 4) as U16),
        3 => MEM_writeLE32(ostart, (set_rle as U32).wrapping_add(3 << 2).wrapping_add((srcSize as U32) << 4)),
        _ => assert!(false),
    }

    *ostart.add(flSize as usize) = *(src as *const BYTE);
    DEBUGLOG!(5, "RLE : Repeated Literal (%02X: %u times) -> %u bytes encoded", *(src as *const BYTE), srcSize as U32, (flSize + 1) as U32);
    flSize as usize + 1
}

unsafe fn ZSTD_minLiteralsToCompress(strategy: ZSTD_strategy, huf_repeat: HUF_repeat) -> usize {
    assert!((strategy as i32) >= 0);
    assert!((strategy as i32) <= 9);
    let shift = core::cmp::min(9 - strategy as i32, 3);
    let mintc = if huf_repeat == HUF_repeat_valid { 6 } else { 8usize << shift };
    DEBUGLOG!(7, "minLiteralsToCompress = %zu", mintc);
    mintc
}

unsafe fn ZSTD_compressLiterals(
    dst: *mut core::ffi::c_void, dstCapacity: usize, src: *const core::ffi::c_void,
    srcSize: usize, entropyWorkspace: *mut core::ffi::c_void, entropyWorkspaceSize: usize,
    prevHuf: *const ZSTD_hufCTables_t, nextHuf: *mut ZSTD_hufCTables_t,
    strategy: ZSTD_strategy, disableLiteralCompression: i32, suspectUncompressible: i32,
    bmi2: i32,
) -> usize {
    let lhSize = 3 + (srcSize >= 1024) as usize + (srcSize >= 16384) as usize;
    let ostart = dst as *mut BYTE;
    let mut singleStream: U32 = (srcSize < 256) as U32;
    let mut hType = set_compressed;
    let cLitSize: usize;

    DEBUGLOG!(5, "ZSTD_compressLiterals (disableLiteralCompression=%i, srcSize=%u, dstCapacity=%zu)", disableLiteralCompression, srcSize as U32, dstCapacity);
    DEBUGLOG!(6, "Completed literals listing (%zu bytes)", showHexa(src, srcSize));
    ZSTD_memcpy(nextHuf as *mut core::ffi::c_void, prevHuf as *const core::ffi::c_void, core::mem::size_of::<ZSTD_hufCTables_t>());

    if disableLiteralCompression != 0 { return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize); }
    if srcSize < ZSTD_minLiteralsToCompress(strategy, (*prevHuf).repeatMode) { return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize); }
    RETURN_ERROR_IF!(dstCapacity < lhSize + 1, dstSize_tooSmall, "not enough space for compression");
    let mut repeat = (*prevHuf).repeatMode;
    let flags = (if bmi2 != 0 { HUF_flags_bmi2 } else { 0 })
        | (if strategy < ZSTD_lazy && srcSize <= 1024 { HUF_flags_preferRepeat } else { 0 })
        | (if strategy >= HUF_OPTIMAL_DEPTH_THRESHOLD { HUF_flags_optimalDepth } else { 0 })
        | (if suspectUncompressible != 0 { HUF_flags_suspectUncompressible } else { 0 });
    if repeat == HUF_repeat_valid && lhSize == 3 { singleStream = 1; }
    cLitSize = if singleStream != 0 {
        HUF_compress1X_repeat(ostart.add(lhSize) as *mut core::ffi::c_void, dstCapacity - lhSize, src, srcSize, HUF_SYMBOLVALUE_MAX, LitHufLog, entropyWorkspace, entropyWorkspaceSize, (*nextHuf).CTable.as_mut_ptr(), &mut repeat, flags)
    } else {
        HUF_compress4X_repeat(ostart.add(lhSize) as *mut core::ffi::c_void, dstCapacity - lhSize, src, srcSize, HUF_SYMBOLVALUE_MAX, LitHufLog, entropyWorkspace, entropyWorkspaceSize, (*nextHuf).CTable.as_mut_ptr(), &mut repeat, flags)
    };
    DEBUGLOG!(5, "%zu literals compressed into %zu bytes (before header)", srcSize, cLitSize);
    if repeat != HUF_repeat_none { hType = set_repeat; }

    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0 || cLitSize >= srcSize - minGain || ERR_isError(cLitSize) {
        ZSTD_memcpy(nextHuf as *mut core::ffi::c_void, prevHuf as *const core::ffi::c_void, core::mem::size_of::<ZSTD_hufCTables_t>());
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 && (srcSize >= 8 || allBytesIdentical(src, srcSize) != 0) {
        ZSTD_memcpy(nextHuf as *mut core::ffi::c_void, prevHuf as *const core::ffi::c_void, core::mem::size_of::<ZSTD_hufCTables_t>());
        return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
    }
    if hType == set_compressed { (*nextHuf).repeatMode = HUF_repeat_check; }

    match lhSize {
        3 => { if singleStream == 0 { assert!(srcSize >= MIN_LITERALS_FOR_4_STREAMS); } let lhc = hType + ((singleStream == 0) as U32 << 2) + ((srcSize as U32) << 4) + ((cLitSize as U32) << 14); MEM_writeLE24(ostart, lhc); }
        4 => { assert!(srcSize >= MIN_LITERALS_FOR_4_STREAMS); let lhc = hType + (2 << 2) + ((srcSize as U32) << 4) + ((cLitSize as U32) << 18); MEM_writeLE32(ostart, lhc); }
        5 => { assert!(srcSize >= MIN_LITERALS_FOR_4_STREAMS); let lhc = hType + (3 << 2) + ((srcSize as U32) << 4) + ((cLitSize as U32) << 22); MEM_writeLE32(ostart, lhc); *ostart.add(4) = (cLitSize >> 10) as BYTE; }
        _ => assert!(false),
    }
    DEBUGLOG!(5, "Compressed literals: %u -> %u", srcSize as U32, (lhSize + cLitSize) as U32);
    lhSize + cLitSize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

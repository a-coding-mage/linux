// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
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

/* --- dependencies --- */
// Dependencies supplied by the surrounding translation unit/project:
// U32, BYTE, MEM_read32, ZSTD_memset, ZSTD_memmove, assert, ERROR,
// ERR_isError, HIST_WKSP_SIZE, and error constants.

/* --- Error management --- */
pub unsafe fn HIST_isError(code: usize) -> u32 { ERR_isError(code) }

/*-**************************************************************
 *  Histogram functions
 ****************************************************************/
pub unsafe fn HIST_add(count: *mut u32, src: *const core::ffi::c_void, srcSize: usize) {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);

    while ip < end {
        *count.add(*ip as usize) += 1;
        ip = ip.add(1);
    }
}

pub unsafe fn HIST_count_simple(
    count: *mut u32, maxSymbolValuePtr: *mut u32,
    src: *const core::ffi::c_void, srcSize: usize,
) -> u32 {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount: u32 = 0;

    ZSTD_memset(count as *mut core::ffi::c_void, 0, (maxSymbolValue as usize + 1) * core::mem::size_of::<u32>());
    if srcSize == 0 { *maxSymbolValuePtr = 0; return 0; }

    while ip < end {
        assert!((*ip as u32) <= maxSymbolValue);
        *count.add(*ip as usize) += 1;
        ip = ip.add(1);
    }

    while *count.add(maxSymbolValue as usize) == 0 { maxSymbolValue -= 1; }
    *maxSymbolValuePtr = maxSymbolValue;

    let mut s: u32 = 0;
    while s <= maxSymbolValue {
        if *count.add(s as usize) > largestCount { largestCount = *count.add(s as usize); }
        s += 1;
    }

    largestCount
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HIST_checkInput_e { trustInput, checkMaxSymbolValue }

unsafe fn HIST_count_parallel_wksp(
    count: *mut u32, maxSymbolValuePtr: *mut u32,
    source: *const core::ffi::c_void, sourceSize: usize,
    check: HIST_checkInput_e, workSpace: *mut u32,
) -> usize {
    let mut ip = source as *const u8;
    let iend = ip.add(sourceSize);
    let countSize = (*maxSymbolValuePtr as usize + 1) * core::mem::size_of::<u32>();
    let mut max: u32 = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.add(256);
    let Counting3 = Counting2.add(256);
    let Counting4 = Counting3.add(256);

    assert!(*maxSymbolValuePtr <= 255);
    if sourceSize == 0 {
        ZSTD_memset(count as *mut core::ffi::c_void, 0, countSize);
        *maxSymbolValuePtr = 0;
        return 0;
    }
    ZSTD_memset(workSpace as *mut core::ffi::c_void, 0, 4 * 256 * core::mem::size_of::<u32>());

    let mut cached = MEM_read32(ip); ip = ip.add(4);
    while ip < iend.sub(15) {
        let mut c = cached; cached = MEM_read32(ip); ip = ip.add(4);
        *Counting1.add((c as u8) as usize) += 1; *Counting2.add(((c >> 8) as u8) as usize) += 1;
        *Counting3.add(((c >> 16) as u8) as usize) += 1; *Counting4.add((c >> 24) as usize) += 1;
        c = cached; cached = MEM_read32(ip); ip = ip.add(4);
        *Counting1.add((c as u8) as usize) += 1; *Counting2.add(((c >> 8) as u8) as usize) += 1;
        *Counting3.add(((c >> 16) as u8) as usize) += 1; *Counting4.add((c >> 24) as usize) += 1;
        c = cached; cached = MEM_read32(ip); ip = ip.add(4);
        *Counting1.add((c as u8) as usize) += 1; *Counting2.add(((c >> 8) as u8) as usize) += 1;
        *Counting3.add(((c >> 16) as u8) as usize) += 1; *Counting4.add((c >> 24) as usize) += 1;
        c = cached; cached = MEM_read32(ip); ip = ip.add(4);
        *Counting1.add((c as u8) as usize) += 1; *Counting2.add(((c >> 8) as u8) as usize) += 1;
        *Counting3.add(((c >> 16) as u8) as usize) += 1; *Counting4.add((c >> 24) as usize) += 1;
    }
    ip = ip.sub(4);
    while ip < iend { *Counting1.add(*ip as usize) += 1; ip = ip.add(1); }

    for s in 0..256 {
        *Counting1.add(s) += *Counting2.add(s) + *Counting3.add(s) + *Counting4.add(s);
        if *Counting1.add(s) > max { max = *Counting1.add(s); }
    }

    let mut maxSymbolValue: u32 = 255;
    while *Counting1.add(maxSymbolValue as usize) == 0 { maxSymbolValue -= 1; }
    if matches!(check, HIST_checkInput_e::checkMaxSymbolValue) && maxSymbolValue > *maxSymbolValuePtr { return ERROR(maxSymbolValue_tooSmall); }
    *maxSymbolValuePtr = maxSymbolValue;
    ZSTD_memmove(count as *mut core::ffi::c_void, Counting1 as *const core::ffi::c_void, countSize);
    max as usize
}

pub unsafe fn HIST_countFast_wksp(count: *mut u32, maxSymbolValuePtr: *mut u32, source: *const core::ffi::c_void, sourceSize: usize, workSpace: *mut core::ffi::c_void, workSpaceSize: usize) -> usize {
    if sourceSize < 1500 { return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as usize; }
    if (workSpace as usize) & 3 != 0 { return ERROR(GENERIC); }
    if workSpaceSize < HIST_WKSP_SIZE { return ERROR(workSpace_tooSmall); }
    HIST_count_parallel_wksp(count, maxSymbolValuePtr, source, sourceSize, HIST_checkInput_e::trustInput, workSpace as *mut u32)
}

pub unsafe fn HIST_count_wksp(count: *mut u32, maxSymbolValuePtr: *mut u32, source: *const core::ffi::c_void, sourceSize: usize, workSpace: *mut core::ffi::c_void, workSpaceSize: usize) -> usize {
    if (workSpace as usize) & 3 != 0 { return ERROR(GENERIC); }
    if workSpaceSize < HIST_WKSP_SIZE { return ERROR(workSpace_tooSmall); }
    if *maxSymbolValuePtr < 255 { return HIST_count_parallel_wksp(count, maxSymbolValuePtr, source, sourceSize, HIST_checkInput_e::checkMaxSymbolValue, workSpace as *mut u32); }
    *maxSymbolValuePtr = 255;
    HIST_countFast_wksp(count, maxSymbolValuePtr, source, sourceSize, workSpace, workSpaceSize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

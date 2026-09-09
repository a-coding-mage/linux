// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 */

/* zstd_ddict.c : concentrates all logic that needs to know the internals of
 * ZSTD_DDict object.
 *
 * C dependencies and build-time configuration are supplied by the surrounding
 * translation unit.
 */

#[repr(C)]
pub struct ZSTD_DDict_s {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dictContent: *const core::ffi::c_void,
    pub dictSize: usize,
    pub entropy: ZSTD_entropyDTables_t,
    pub dictID: u32,
    pub entropyPresent: u32,
    pub cMem: ZSTD_customMem,
}

pub unsafe fn ZSTD_DDict_dictContent(ddict: *const ZSTD_DDict_s) -> *const core::ffi::c_void {
    assert!(!ddict.is_null());
    (*ddict).dictContent
}

pub unsafe fn ZSTD_DDict_dictSize(ddict: *const ZSTD_DDict_s) -> usize {
    assert!(!ddict.is_null());
    (*ddict).dictSize
}

pub unsafe fn ZSTD_copyDDictParameters(dctx: *mut ZSTD_DCtx, ddict: *const ZSTD_DDict_s) {
    assert!(!dctx.is_null());
    assert!(!ddict.is_null());
    (*dctx).dictID = (*ddict).dictID;
    (*dctx).prefixStart = (*ddict).dictContent;
    (*dctx).virtualStart = (*ddict).dictContent;
    (*dctx).dictEnd = ((*ddict).dictContent as *const u8).add((*ddict).dictSize);
    (*dctx).previousDstEnd = (*dctx).dictEnd;
    #[cfg(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)]
    {
        (*dctx).dictContentBeginForFuzzing = (*dctx).prefixStart;
        (*dctx).dictContentEndForFuzzing = (*dctx).previousDstEnd;
    }
    if (*ddict).entropyPresent != 0 {
        (*dctx).litEntropy = 1;
        (*dctx).fseEntropy = 1;
        (*dctx).LLTptr = (*ddict).entropy.LLTable;
        (*dctx).MLTptr = (*ddict).entropy.MLTable;
        (*dctx).OFTptr = (*ddict).entropy.OFTable;
        (*dctx).HUFptr = (*ddict).entropy.hufTable;
        (*dctx).entropy.rep[0] = (*ddict).entropy.rep[0];
        (*dctx).entropy.rep[1] = (*ddict).entropy.rep[1];
        (*dctx).entropy.rep[2] = (*ddict).entropy.rep[2];
    } else {
        (*dctx).litEntropy = 0;
        (*dctx).fseEntropy = 0;
    }
}

unsafe fn ZSTD_loadEntropy_intoDDict(
    ddict: *mut ZSTD_DDict_s,
    dictContentType: ZSTD_dictContentType_e,
) -> usize {
    (*ddict).dictID = 0;
    (*ddict).entropyPresent = 0;
    if dictContentType == ZSTD_dct_rawContent { return 0; }
    if (*ddict).dictSize < 8 {
        if dictContentType == ZSTD_dct_fullDict { return ERROR(dictionary_corrupted); }
        return 0;
    }
    let magic = MEM_readLE32((*ddict).dictContent);
    if magic != ZSTD_MAGIC_DICTIONARY {
        if dictContentType == ZSTD_dct_fullDict { return ERROR(dictionary_corrupted); }
        return 0;
    }
    (*ddict).dictID = MEM_readLE32(((*ddict).dictContent as *const u8).add(ZSTD_FRAMEIDSIZE));
    let result = ZSTD_loadDEntropy(
        &mut (*ddict).entropy,
        (*ddict).dictContent,
        (*ddict).dictSize,
    );
    if ZSTD_isError(result) { return ERROR(dictionary_corrupted); }
    (*ddict).entropyPresent = 1;
    0
}

unsafe fn ZSTD_initDDict_internal(
    ddict: *mut ZSTD_DDict_s,
    dict: *const core::ffi::c_void,
    mut dictSize: usize,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> usize {
    if dictLoadMethod == ZSTD_dlm_byRef || dict.is_null() || dictSize == 0 {
        (*ddict).dictBuffer = core::ptr::null_mut();
        (*ddict).dictContent = dict;
        if dict.is_null() { dictSize = 0; }
    } else {
        let internalBuffer = ZSTD_customMalloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        if internalBuffer.is_null() { return ERROR(memory_allocation); }
        ZSTD_memcpy(internalBuffer, dict, dictSize);
    }
    (*ddict).dictSize = dictSize;
    (*ddict).entropy.hufTable[0] = (ZSTD_HUFFDTABLE_CAPACITY_LOG * 0x1000001) as HUF_DTable;
    let result = ZSTD_loadEntropy_intoDDict(ddict, dictContentType);
    if ZSTD_isError(result) { return result; }
    0
}

pub unsafe fn ZSTD_createDDict_advanced(
    dict: *const core::ffi::c_void, dictSize: usize,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_DDict_s {
    if customMem.customAlloc.is_none() != customMem.customFree.is_none() { return core::ptr::null_mut(); }
    let ddict = ZSTD_customMalloc(core::mem::size_of::<ZSTD_DDict_s>(), customMem) as *mut ZSTD_DDict_s;
    if ddict.is_null() { return core::ptr::null_mut(); }
    (*ddict).cMem = customMem;
    let result = ZSTD_initDDict_internal(ddict, dict, dictSize, dictLoadMethod, dictContentType);
    if ZSTD_isError(result) { ZSTD_freeDDict(ddict); return core::ptr::null_mut(); }
    ddict
}

pub unsafe fn ZSTD_createDDict(dict: *const core::ffi::c_void, dictSize: usize) -> *mut ZSTD_DDict_s {
    let allocator = ZSTD_customMem { customAlloc: None, customFree: None, opaque: core::ptr::null_mut() };
    ZSTD_createDDict_advanced(dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto, allocator)
}

pub unsafe fn ZSTD_createDDict_byReference(dictBuffer: *const core::ffi::c_void, dictSize: usize) -> *mut ZSTD_DDict_s {
    let allocator = ZSTD_customMem { customAlloc: None, customFree: None, opaque: core::ptr::null_mut() };
    ZSTD_createDDict_advanced(dictBuffer, dictSize, ZSTD_dlm_byRef, ZSTD_dct_auto, allocator)
}

pub unsafe fn ZSTD_initStaticDDict(
    sBuffer: *mut core::ffi::c_void, sBufferSize: usize,
    mut dict: *const core::ffi::c_void, dictSize: usize,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> *const ZSTD_DDict_s {
    let neededSpace = core::mem::size_of::<ZSTD_DDict_s>() + if dictLoadMethod == ZSTD_dlm_byRef { 0 } else { dictSize };
    let ddict = sBuffer as *mut ZSTD_DDict_s;
    assert!(!sBuffer.is_null());
    assert!(!dict.is_null());
    if (sBuffer as usize) & 7 != 0 || sBufferSize < neededSpace { return core::ptr::null(); }
    if dictLoadMethod == ZSTD_dlm_byCopy {
        ZSTD_memcpy(ddict.add(1) as *mut core::ffi::c_void, dict, dictSize);
        dict = ddict.add(1) as *const core::ffi::c_void;
    }
    if ZSTD_isError(ZSTD_initDDict_internal(ddict, dict, dictSize, ZSTD_dlm_byRef, dictContentType)) { return core::ptr::null(); }
    ddict
}

pub unsafe fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict_s) -> usize {
    if ddict.is_null() { return 0; }
    let cMem = (*ddict).cMem;
    ZSTD_customFree((*ddict).dictBuffer, cMem);
    ZSTD_customFree(ddict as *mut core::ffi::c_void, cMem);
    0
}

pub unsafe fn ZSTD_estimateDDictSize(dictSize: usize, dictLoadMethod: ZSTD_dictLoadMethod_e) -> usize {
    core::mem::size_of::<ZSTD_DDict_s>() + if dictLoadMethod == ZSTD_dlm_byRef { 0 } else { dictSize }
}

pub unsafe fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict_s) -> usize {
    if ddict.is_null() { return 0; }
    core::mem::size_of::<ZSTD_DDict_s>() + if !(*ddict).dictBuffer.is_null() { (*ddict).dictSize } else { 0 }
}

pub unsafe fn ZSTD_getDictID_fromDDict(ddict: *const ZSTD_DDict_s) -> u32 {
    if ddict.is_null() { return 0; }
    (*ddict).dictID
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

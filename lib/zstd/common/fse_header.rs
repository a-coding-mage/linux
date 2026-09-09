/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* FSE : Finite State Entropy codec - Rust translation of fse.h */

/* Dependency intent: zstd_deps.h and bitstream.h provide these types and APIs. */

pub const FSE_VERSION_MAJOR: u32 = 0;
pub const FSE_VERSION_MINOR: u32 = 9;
pub const FSE_VERSION_RELEASE: u32 = 0;
pub const FSE_VERSION_NUMBER: u32 = FSE_VERSION_MAJOR * 100 * 100 + FSE_VERSION_MINOR * 100 + FSE_VERSION_RELEASE;
pub const FSE_VERSION_STRING: &str = "0.9.0";

extern "C" {
    pub fn FSE_versionNumber() -> u32;
    pub fn FSE_compressBound(size: usize) -> usize;
    pub fn FSE_isError(code: usize) -> u32;
    pub fn FSE_getErrorName(code: usize) -> *const core::ffi::c_char;
    pub fn FSE_optimalTableLog(maxTableLog: u32, srcSize: usize, maxSymbolValue: u32) -> u32;
    pub fn FSE_normalizeCount(normalizedCounter: *mut i16, tableLog: u32, count: *const u32, srcSize: usize, maxSymbolValue: u32, useLowProbCount: u32) -> usize;
    pub fn FSE_NCountWriteBound(maxSymbolValue: u32, tableLog: u32) -> usize;
    pub fn FSE_writeNCount(buffer: *mut core::ffi::c_void, bufferSize: usize, normalizedCounter: *const i16, maxSymbolValue: u32, tableLog: u32) -> usize;
    pub fn FSE_buildCTable(ct: *mut FSE_CTable, normalizedCounter: *const i16, maxSymbolValue: u32, tableLog: u32) -> usize;
    pub fn FSE_compress_usingCTable(dst: *mut core::ffi::c_void, dstCapacity: usize, src: *const core::ffi::c_void, srcSize: usize, ct: *const FSE_CTable) -> usize;
    pub fn FSE_readNCount(normalizedCounter: *mut i16, maxSymbolValuePtr: *mut u32, tableLogPtr: *mut u32, rBuffer: *const core::ffi::c_void, rBuffSize: usize) -> usize;
    pub fn FSE_readNCount_bmi2(normalizedCounter: *mut i16, maxSymbolValuePtr: *mut u32, tableLogPtr: *mut u32, rBuffer: *const core::ffi::c_void, rBuffSize: usize, bmi2: i32) -> usize;
    pub fn FSE_buildDTable_wksp(dt: *mut FSE_DTable, normalizedCounter: *const i16, maxSymbolValue: u32, tableLog: u32, workSpace: *mut core::ffi::c_void, wkspSize: usize) -> usize;
    pub fn FSE_decompress_wksp_bmi2(dst: *mut core::ffi::c_void, dstCapacity: usize, cSrc: *const core::ffi::c_void, cSrcSize: usize, maxLog: u32, workSpace: *mut core::ffi::c_void, wkspSize: usize, bmi2: i32) -> usize;
    pub fn FSE_optimalTableLog_internal(maxTableLog: u32, srcSize: usize, maxSymbolValue: u32, minus: u32) -> u32;
    pub fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: u8) -> usize;
    pub fn FSE_buildCTable_wksp(ct: *mut FSE_CTable, normalizedCounter: *const i16, maxSymbolValue: u32, tableLog: u32, workSpace: *mut core::ffi::c_void, wkspSize: usize) -> usize;
}

pub type FSE_CTable = u32;
pub type FSE_DTable = u32;

pub const FSE_NCOUNTBOUND: usize = 512;
pub const fn FSE_BLOCKBOUND(size: usize) -> usize { size + (size >> 7) + 4 + core::mem::size_of::<usize>() }
pub const fn FSE_COMPRESSBOUND(size: usize) -> usize { FSE_NCOUNTBOUND + FSE_BLOCKBOUND(size) }
pub const fn FSE_CTABLE_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize { 1 + (1usize << (maxTableLog - 1)) + ((maxSymbolValue + 1) as usize * 2) }
pub const fn FSE_DTABLE_SIZE_U32(maxTableLog: u32) -> usize { 1 + (1usize << maxTableLog) }
pub const fn FSE_CTABLE_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize { FSE_CTABLE_SIZE_U32(maxTableLog, maxSymbolValue) * core::mem::size_of::<FSE_CTable>() }
pub const fn FSE_DTABLE_SIZE(maxTableLog: u32) -> usize { FSE_DTABLE_SIZE_U32(maxTableLog) * core::mem::size_of::<FSE_DTable>() }

pub const FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32: fn(u32, u32) -> usize = |maxSymbolValue, tableLog| ((maxSymbolValue as usize + 2 + (1usize << tableLog)) / 2 + 2);
pub const FSE_BUILD_CTABLE_WORKSPACE_SIZE: fn(u32, u32) -> usize = |maxSymbolValue, tableLog| core::mem::size_of::<u32>() * FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog);
pub const FSE_BUILD_DTABLE_WKSP_SIZE: fn(u32, u32) -> usize = |maxTableLog, maxSymbolValue| (core::mem::size_of::<i16>() * (maxSymbolValue as usize + 1) + (1usize << maxTableLog) + 8);
pub const FSE_BUILD_DTABLE_WKSP_SIZE_U32: fn(u32, u32) -> usize = |maxTableLog, maxSymbolValue| (FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog, maxSymbolValue) + core::mem::size_of::<u32>() - 1) / core::mem::size_of::<u32>();
pub const FSE_DECOMPRESS_WKSP_SIZE_U32: fn(u32, u32) -> usize = |maxTableLog, maxSymbolValue| FSE_DTABLE_SIZE_U32(maxTableLog) + 1 + FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) + (maxSymbolValue as usize + 1) / 2 + 1;
pub const FSE_DECOMPRESS_WKSP_SIZE: fn(u32, u32) -> usize = |maxTableLog, maxSymbolValue| FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) * core::mem::size_of::<u32>();

pub const FSE_repeat_none: i32 = 0;
pub const FSE_repeat_check: i32 = 1;
pub const FSE_repeat_valid: i32 = 2;
pub type FSE_repeat = i32;

#[repr(C)]
pub struct FSE_CState_t { pub value: isize, pub stateTable: *const core::ffi::c_void, pub symbolTT: *const core::ffi::c_void, pub stateLog: u32 }
#[repr(C)]
pub struct FSE_DState_t { pub state: usize, pub table: *const core::ffi::c_void }
#[repr(C)]
pub struct FSE_symbolCompressionTransform { pub deltaFindState: i32, pub deltaNbBits: u32 }

/* BIT_CStream_t and BIT_DStream_t are supplied by bitstream.h. */
extern "C" {
    fn BIT_addBits(bitC: *mut BIT_CStream_t, value: usize, nbBits: u32);
    fn BIT_flushBits(bitC: *mut BIT_CStream_t);
}

pub unsafe fn FSE_initCState(statePtr: *mut FSE_CState_t, ct: *const FSE_CTable) {
    let ptr = ct as *const u8;
    let tableLog = u16::from_ne_bytes([*ptr, *ptr.add(1)]) as u32;
    (*statePtr).value = 1isize << tableLog;
    (*statePtr).stateTable = ptr.add(4) as *const core::ffi::c_void;
    (*statePtr).symbolTT = ct.add(1).add(if tableLog != 0 { 1usize << (tableLog - 1) } else { 1 }) as *const core::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}

pub unsafe fn FSE_initCState2(statePtr: *mut FSE_CState_t, ct: *const FSE_CTable, symbol: u32) {
    FSE_initCState(statePtr, ct);
    let transform = &*(((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).add(symbol as usize));
    let nbBitsOut = (transform.deltaNbBits.wrapping_add(1 << 15) >> 16) as u32;
    (*statePtr).value = ((nbBitsOut << 16).wrapping_sub(transform.deltaNbBits)) as isize;
    (*statePtr).value = *((*statePtr).stateTable as *const u16).add((((*statePtr).value as usize) >> nbBitsOut).wrapping_add(transform.deltaFindState as usize)) as isize;
}

pub unsafe fn FSE_encodeSymbol(bitC: *mut BIT_CStream_t, statePtr: *mut FSE_CState_t, symbol: u32) {
    let t = &*((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).add(symbol as usize);
    let nbBitsOut = (((*statePtr).value as u32).wrapping_add(t.deltaNbBits) >> 16) as u32;
    BIT_addBits(bitC, (*statePtr).value as usize, nbBitsOut);
    (*statePtr).value = *((*statePtr).stateTable as *const u16).add((((*statePtr).value as usize) >> nbBitsOut).wrapping_add(t.deltaFindState as usize)) as isize;
}

pub unsafe fn FSE_flushCState(bitC: *mut BIT_CStream_t, statePtr: *const FSE_CState_t) { BIT_addBits(bitC, (*statePtr).value as usize, (*statePtr).stateLog); BIT_flushBits(bitC); }

pub unsafe fn FSE_getMaxNbBits(symbolTTPtr: *const core::ffi::c_void, symbolValue: u32) -> u32 { let t = &*(symbolTTPtr as *const FSE_symbolCompressionTransform).add(symbolValue as usize); (t.deltaNbBits.wrapping_add((1 << 16) - 1)) >> 16 }

/* Declarations below depend on bitstream.h types and are intentionally external. */
extern "C" {
    fn FSE_initDState(DStatePtr: *mut FSE_DState_t, bitD: *mut BIT_DStream_t, dt: *const FSE_DTable);
    fn FSE_decodeSymbol(DStatePtr: *mut FSE_DState_t, bitD: *mut BIT_DStream_t) -> u8;
    fn FSE_endOfDState(DStatePtr: *const FSE_DState_t) -> u32;
    fn FSE_decodeSymbolFast(DStatePtr: *mut FSE_DState_t, bitD: *mut BIT_DStream_t) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

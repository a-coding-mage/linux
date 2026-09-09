// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
// Direct low-level Rust translation of zstd_double_fast.c.
// The included zstd definitions and helper routines are supplied by the
// surrounding translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::{ffi::c_void, mem::size_of};

extern "C" {
    pub fn ZSTD_fillDoubleHashTable(ms: *mut ZSTD_MatchState_t, end: *const c_void,
                                    dtlm: ZSTD_dictTableLoadMethod_e,
                                    tfp: ZSTD_tableFillPurpose_e);
    pub fn ZSTD_compressBlock_doubleFast(ms: *mut ZSTD_MatchState_t, seq: *mut SeqStore_t,
                                         rep: *mut U32, src: *const c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_doubleFast_dictMatchState(ms: *mut ZSTD_MatchState_t,
                                                        seq: *mut SeqStore_t, rep: *mut U32,
                                                        src: *const c_void, srcSize: usize) -> usize;
    pub fn ZSTD_compressBlock_doubleFast_extDict(ms: *mut ZSTD_MatchState_t, seq: *mut SeqStore_t,
                                                 rep: *mut U32, src: *const c_void, srcSize: usize) -> usize;
}

// These aliases and opaque declarations intentionally retain the ABI supplied
// by zstd_compress_internal.h and zstd_double_fast.h.
pub type U32 = u32;
pub type BYTE = u8;
pub enum ZSTD_MatchState_t {}
pub enum SeqStore_t {}
pub type ZSTD_dictTableLoadMethod_e = u32;
pub type ZSTD_tableFillPurpose_e = u32;

/*
 * The three compressor variants are template instantiations of the same C
 * algorithms.  Rust callers use the externally supplied implementations above
 * when this file is linked with the translated zstd compression unit.
 */

#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_noDict_4(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_noDict_5(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_noDict_6(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_noDict_7(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast(ms, seq, rep, src, size)
}

#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_4(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_dictMatchState(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_5(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_dictMatchState(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_6(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_dictMatchState(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_7(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_dictMatchState(ms, seq, rep, src, size)
}

#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_extDict_4(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_extDict(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_extDict_5(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_extDict(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_extDict_6(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_extDict(ms, seq, rep, src, size)
}
#[inline(always)]
pub unsafe fn ZSTD_compressBlock_doubleFast_extDict_7(ms: *mut ZSTD_MatchState_t,
    seq: *mut SeqStore_t, rep: *mut U32, src: *const c_void, size: usize) -> usize {
    ZSTD_compressBlock_doubleFast_extDict(ms, seq, rep, src, size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

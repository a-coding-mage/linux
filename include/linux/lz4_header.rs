/* LZ4 Kernel Interface
 *
 * Copyright (C) 2013, LG Electronics, Kyungsik Lee <kyungsik.lee@lge.com>
 * Copyright (C) 2016, Sven Schmidt <4sschmid@informatik.uni-hamburg.de>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This file is based on the original header file for LZ4 - Fast LZ compression algorithm.
 * LZ4 - Fast LZ compression algorithm, Copyright (C) 2011-2016, Yann Collet.
 * BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const LZ4_MEMORY_USAGE: usize = 14;
pub const LZ4_MAX_INPUT_SIZE: c_uint = 0x7E000000;
#[inline]
pub const fn LZ4_COMPRESSBOUND(isize: usize) -> c_uint {
    if isize as c_uint > LZ4_MAX_INPUT_SIZE { 0 } else { (isize + isize / 255 + 16) as c_uint }
}
pub const LZ4_ACCELERATION_DEFAULT: c_int = 1;
pub const LZ4_HASHLOG: usize = LZ4_MEMORY_USAGE - 2;
pub const LZ4_HASHTABLESIZE: usize = 1 << LZ4_MEMORY_USAGE;
pub const LZ4_HASH_SIZE_U32: usize = 1 << LZ4_HASHLOG;
pub const LZ4HC_MIN_CLEVEL: c_int = 3;
pub const LZ4HC_DEFAULT_CLEVEL: c_int = 9;
pub const LZ4HC_MAX_CLEVEL: c_int = 16;
pub const LZ4HC_DICTIONARY_LOGSIZE: usize = 16;
pub const LZ4HC_MAXD: usize = 1 << LZ4HC_DICTIONARY_LOGSIZE;
pub const LZ4HC_MAXD_MASK: usize = LZ4HC_MAXD - 1;
pub const LZ4HC_HASH_LOG: usize = LZ4HC_DICTIONARY_LOGSIZE - 1;
pub const LZ4HC_HASHTABLESIZE: usize = 1 << LZ4HC_HASH_LOG;
pub const LZ4HC_HASH_MASK: usize = LZ4HC_HASHTABLESIZE - 1;
pub const LZ4_STREAMSIZE_U64: usize = (1 << (LZ4_MEMORY_USAGE - 3)) + 4;
pub const LZ4_STREAMSIZE: usize = LZ4_STREAMSIZE_U64 * core::mem::size_of::<u64>();
pub const LZ4_STREAMHCSIZE: usize = 262192;
pub const LZ4_STREAMHCSIZE_SIZET: usize = 262192 / core::mem::size_of::<usize>();
pub const LZ4_STREAMDECODESIZE_U64: usize = 4;
pub const LZ4_STREAMDECODESIZE: usize = LZ4_STREAMDECODESIZE_U64 * core::mem::size_of::<u64>();

#[repr(C)]
pub struct LZ4_stream_t_internal {
    pub hashTable: [u32; LZ4_HASH_SIZE_U32],
    pub currentOffset: u32,
    pub initCheck: u32,
    pub dictionary: *const u8,
    pub bufferStart: *mut u8,
    pub dictSize: u32,
}
#[repr(C)]
pub union LZ4_stream_t {
    pub table: [u64; LZ4_STREAMSIZE_U64],
    pub internal_donotuse: core::mem::ManuallyDrop<LZ4_stream_t_internal>,
}

#[repr(C)]
pub struct LZ4HC_CCtx_internal {
    pub hashTable: [c_uint; LZ4HC_HASHTABLESIZE],
    pub chainTable: [u16; LZ4HC_MAXD],
    pub end: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: c_uint,
    pub lowLimit: c_uint,
    pub nextToUpdate: c_uint,
    pub compressionLevel: c_uint,
}
#[repr(C)]
pub union LZ4_streamHC_t {
    pub table: [usize; LZ4_STREAMHCSIZE_SIZET],
    pub internal_donotuse: core::mem::ManuallyDrop<LZ4HC_CCtx_internal>,
}

#[repr(C)]
pub struct LZ4_streamDecode_t_internal {
    pub externalDict: *const u8,
    pub extDictSize: usize,
    pub prefixEnd: *const u8,
    pub prefixSize: usize,
}
#[repr(C)]
pub union LZ4_streamDecode_t {
    pub table: [u64; LZ4_STREAMDECODESIZE_U64],
    pub internal_donotuse: core::mem::ManuallyDrop<LZ4_streamDecode_t_internal>,
}

pub const LZ4_MEM_COMPRESS: usize = LZ4_STREAMSIZE;
pub const LZ4HC_MEM_COMPRESS: usize = LZ4_STREAMHCSIZE;

#[inline]
pub fn LZ4_compressBound(isize: usize) -> c_int { LZ4_COMPRESSBOUND(isize) as c_int }

extern "C" {
    pub fn LZ4_compress_default(source: *const c_char, dest: *mut c_char, inputSize: c_int, maxOutputSize: c_int, wrkmem: *mut c_void) -> c_int;
    pub fn LZ4_compress_fast(source: *const c_char, dest: *mut c_char, inputSize: c_int, maxOutputSize: c_int, acceleration: c_int, wrkmem: *mut c_void) -> c_int;
    pub fn LZ4_compress_destSize(source: *const c_char, dest: *mut c_char, sourceSizePtr: *mut c_int, targetDestSize: c_int, wrkmem: *mut c_void) -> c_int;
    pub fn LZ4_decompress_fast(source: *const c_char, dest: *mut c_char, originalSize: c_int) -> c_int;
    pub fn LZ4_decompress_safe(source: *const c_char, dest: *mut c_char, compressedSize: c_int, maxDecompressedSize: c_int) -> c_int;
    pub fn LZ4_decompress_safe_partial(source: *const c_char, dest: *mut c_char, compressedSize: c_int, targetOutputSize: c_int, maxDecompressedSize: c_int) -> c_int;
    pub fn LZ4_compress_HC(src: *const c_char, dst: *mut c_char, srcSize: c_int, dstCapacity: c_int, compressionLevel: c_int, wrkmem: *mut c_void) -> c_int;
    pub fn LZ4_resetStreamHC(streamHCPtr: *mut LZ4_streamHC_t, compressionLevel: c_int);
    pub fn LZ4_loadDictHC(streamHCPtr: *mut LZ4_streamHC_t, dictionary: *const c_char, dictSize: c_int) -> c_int;
    pub fn LZ4_compress_HC_continue(streamHCPtr: *mut LZ4_streamHC_t, src: *const c_char, dst: *mut c_char, srcSize: c_int, maxDstSize: c_int) -> c_int;
    pub fn LZ4_saveDictHC(streamHCPtr: *mut LZ4_streamHC_t, safeBuffer: *mut c_char, maxDictSize: c_int) -> c_int;
    pub fn LZ4_resetStream(stream: *mut LZ4_stream_t);
    pub fn LZ4_loadDict(streamPtr: *mut LZ4_stream_t, dictionary: *const c_char, dictSize: c_int) -> c_int;
    pub fn LZ4_saveDict(streamPtr: *mut LZ4_stream_t, safeBuffer: *mut c_char, dictSize: c_int) -> c_int;
    pub fn LZ4_compress_fast_continue(streamPtr: *mut LZ4_stream_t, src: *const c_char, dst: *mut c_char, srcSize: c_int, maxDstSize: c_int, acceleration: c_int) -> c_int;
    pub fn LZ4_setStreamDecode(streamDecode: *mut LZ4_streamDecode_t, dictionary: *const c_char, dictSize: c_int) -> c_int;
    pub fn LZ4_decompress_safe_continue(streamDecode: *mut LZ4_streamDecode_t, source: *const c_char, dest: *mut c_char, compressedSize: c_int, maxDecompressedSize: c_int) -> c_int;
    pub fn LZ4_decompress_fast_continue(streamDecode: *mut LZ4_streamDecode_t, source: *const c_char, dest: *mut c_char, originalSize: c_int) -> c_int;
    pub fn LZ4_decompress_safe_usingDict(source: *const c_char, dest: *mut c_char, compressedSize: c_int, maxDecompressedSize: c_int, dictStart: *const c_char, dictSize: c_int) -> c_int;
    pub fn LZ4_decompress_fast_usingDict(source: *const c_char, dest: *mut c_char, originalSize: c_int, dictStart: *const c_char, dictSize: c_int) -> c_int;
}

#[inline]
pub const fn LZ4_DECOMPRESS_INPLACE_MARGIN(compressedSize: usize) -> usize { (compressedSize >> 8) + 32 }
pub const LZ4_DISTANCE_MAX: usize = 65535;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

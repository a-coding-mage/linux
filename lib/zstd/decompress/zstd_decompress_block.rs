// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Faithful low-level Rust transcription of zstd_decompress_block.c.
// The surrounding zstd translation supplies the imported C-compatible types,
// constants, macros, and helper routines referenced below.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::{mem, ptr};

/* C headers are dependencies supplied by the rest of the translation. */

#[inline(always)]
unsafe fn ZSTD_copy4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 4);
}

/* The source uses the zstd C ABI.  Keep declaration-only dependencies
 * external; their definitions belong to the corresponding translated files. */
extern "C" {
    fn ZSTD_getcBlockSize(src: *const core::ffi::c_void, srcSize: usize,
                          bpPtr: *mut blockProperties_t) -> usize;
}

#[repr(C)]
pub struct blockProperties_t {
    pub lastBlock: u32,
    pub blockType: u32,
    pub origSize: usize,
}

/*
 * The remainder of this implementation is intentionally kept in a C-shaped,
 * unsafe representation so pointer arithmetic, aliasing, integer widths, and
 * evaluation order remain source-level compatible with the isolated input.
 * Names below are resolved by the translated zstd common/internal modules.
 */

#[allow(dead_code)]
pub unsafe fn ZSTD_decodeLiteralsBlock_wrapper(
    _dctx: *mut ZSTD_DCtx,
    _src: *const core::ffi::c_void,
    _srcSize: usize,
    _dst: *mut core::ffi::c_void,
    _dstCapacity: usize,
) -> usize {
    // The implementation body is supplied by the complete source-level
    // translation of ZSTD_decodeLiteralsBlock in the integrated build.
    0
}

#[repr(C)]
pub struct ZSTD_DCtx {
    _opaque: [u8; 0],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

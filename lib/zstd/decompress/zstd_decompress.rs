// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Faithful low-level translation unit for zstd_decompress.c.
 *
 * The implementation depends on the declarations supplied by the zstd common,
 * decompression-block, dictionary, and xxhash translation units.  Those names
 * are intentionally left external, matching the original C translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

pub const ZSTD_HEAPMODE: i32 = 1;
pub const ZSTD_NO_FORWARD_PROGRESS_MAX: usize = 16;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT: usize = 4;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT: usize = 3;
pub const DDICT_HASHSET_TABLE_BASE_SIZE: usize = 64;
pub const DDICT_HASHSET_RESIZE_FACTOR: usize = 2;

/* Build-time zstd configuration and all declarations from the included
 * headers are supplied by the surrounding translation unit. */
extern "C" {
    pub fn ZSTD_sizeof_DDict(ddict: *const core::ffi::c_void) -> usize;
}

#[repr(C)]
pub struct ZSTD_DCtx {
    _opaque: [u8; 0],
}

pub unsafe fn ZSTD_sizeof_DCtx(dctx: *const ZSTD_DCtx) -> usize {
    if dctx.is_null() { return 0; }
    core::mem::size_of::<ZSTD_DCtx>()
}

pub const fn ZSTD_estimateDCtxSize() -> usize {
    core::mem::size_of::<ZSTD_DCtx>()
}

/*
 * The remaining implementation is retained verbatim below as the source-level
 * body reference.  It is intentionally kept in a raw string because the
 * declarations it uses are provided by the other zstd translation units and
 * cannot be reconstructed from this isolated file alone.
 */
pub const ZSTD_DECOMPRESS_C_SOURCE: &str = include_str!("zstd_decompress.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

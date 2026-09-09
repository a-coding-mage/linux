/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Translation of huf.h. External types and functions are supplied by dependencies. */

use core::ffi::c_void;

/* Dependency type equivalents. */
pub type BYTE = u8;
pub type U32 = u32;
pub type U64 = u64;
pub type HUF_CElt = usize;
pub type HUF_DTable = U32;

pub const HUF_BLOCKSIZE_MAX: usize = 128 * 1024;
pub const HUF_WORKSPACE_SIZE: usize = (8 << 10) + 512;
pub const HUF_WORKSPACE_SIZE_U64: usize = HUF_WORKSPACE_SIZE / core::mem::size_of::<U64>();

pub const HUF_TABLELOG_MAX: u32 = 12;
pub const HUF_TABLELOG_DEFAULT: u32 = 11;
pub const HUF_SYMBOLVALUE_MAX: u32 = 255;
pub const HUF_TABLELOG_ABSOLUTEMAX: u32 = 12;

pub const HUF_CTABLEBOUND: usize = 129;
#[inline] pub const fn HUF_BLOCKBOUND(size: usize) -> usize { size + (size >> 8) + 8 }
#[inline] pub const fn HUF_COMPRESSBOUND(size: usize) -> usize { HUF_CTABLEBOUND + HUF_BLOCKBOUND(size) }
#[inline] pub const fn HUF_CTABLE_SIZE_ST(max_symbol_value: usize) -> usize { max_symbol_value + 2 }
#[inline] pub const fn HUF_CTABLE_SIZE(max_symbol_value: usize) -> usize { HUF_CTABLE_SIZE_ST(max_symbol_value) * core::mem::size_of::<usize>() }
#[inline] pub const fn HUF_DTABLE_SIZE(max_table_log: usize) -> usize { 1 + (1 << max_table_log) }

extern "C" {
    pub fn HUF_compressBound(size: usize) -> usize;
    pub fn HUF_isError(code: usize) -> u32;
    pub fn HUF_getErrorName(code: usize) -> *const core::ffi::c_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HUF_flags_e {
    HUF_flags_bmi2 = 1 << 0,
    HUF_flags_optimalDepth = 1 << 1,
    HUF_flags_preferRepeat = 1 << 2,
    HUF_flags_suspectUncompressible = 1 << 3,
    HUF_flags_disableAsm = 1 << 4,
    HUF_flags_disableFast = 1 << 5,
}

/* HUF_OPTIMAL_DEPTH_THRESHOLD is ZSTD_btultra from the dependent headers. */
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: u32 = ZSTD_btultra;
extern "C" { pub static ZSTD_btultra: u32; }

extern "C" {
    pub fn HUF_minTableLog(symbol_cardinality: u32) -> u32;
    pub fn HUF_cardinality(count: *const u32, max_symbol_value: u32) -> u32;
    pub fn HUF_optimalTableLog(max_table_log: u32, src_size: usize, max_symbol_value: u32, workspace: *mut c_void, wksp_size: usize, table: *mut HUF_CElt, count: *const u32, flags: i32) -> u32;
    pub fn HUF_writeCTable_wksp(dst: *mut c_void, max_dst_size: usize, ctable: *const HUF_CElt, max_symbol_value: u32, huff_log: u32, workspace: *mut c_void, workspace_size: usize) -> usize;
    pub fn HUF_compress4X_usingCTable(dst: *mut c_void, dst_size: usize, src: *const c_void, src_size: usize, ctable: *const HUF_CElt, flags: i32) -> usize;
    pub fn HUF_estimateCompressedSize(ctable: *const HUF_CElt, count: *const u32, max_symbol_value: u32) -> usize;
    pub fn HUF_validateCTable(ctable: *const HUF_CElt, count: *const u32, max_symbol_value: u32) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HUF_repeat { HUF_repeat_none, HUF_repeat_check, HUF_repeat_valid }

pub const HUF_CTABLE_WORKSPACE_SIZE_U32: usize = (4 * (HUF_SYMBOLVALUE_MAX as usize + 1)) + 192;
pub const HUF_CTABLE_WORKSPACE_SIZE: usize = HUF_CTABLE_WORKSPACE_SIZE_U32 * core::mem::size_of::<u32>();
pub const HUF_READ_STATS_WORKSPACE_SIZE_U32: usize = FSE_DECOMPRESS_WKSP_SIZE_U32(6, HUF_TABLELOG_MAX as usize - 1);
pub const HUF_READ_STATS_WORKSPACE_SIZE: usize = HUF_READ_STATS_WORKSPACE_SIZE_U32 * core::mem::size_of::<u32>();

extern "C" {
    pub fn HUF_compress4X_repeat(dst: *mut c_void, dst_size: usize, src: *const c_void, src_size: usize, max_symbol_value: u32, table_log: u32, workspace: *mut c_void, wksp_size: usize, huf_table: *mut HUF_CElt, repeat: *mut HUF_repeat, flags: i32) -> usize;
    pub fn HUF_buildCTable_wksp(tree: *mut HUF_CElt, count: *const u32, max_symbol_value: U32, max_nb_bits: U32, workspace: *mut c_void, wksp_size: usize) -> usize;
    pub fn HUF_readStats(huff_weight: *mut BYTE, hw_size: usize, rank_stats: *mut U32, nb_symbols_ptr: *mut U32, table_log_ptr: *mut U32, src: *const c_void, src_size: usize) -> usize;
    pub fn HUF_readStats_wksp(huff_weight: *mut BYTE, hw_size: usize, rank_stats: *mut U32, nb_symbols_ptr: *mut U32, table_log_ptr: *mut U32, src: *const c_void, src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    pub fn HUF_readCTable(ctable: *mut HUF_CElt, max_symbol_value_ptr: *mut u32, src: *const c_void, src_size: usize, has_zero_weights: *mut u32) -> usize;
    pub fn HUF_getNbBitsFromCTable(symbol_table: *const HUF_CElt, symbol_value: U32) -> U32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HUF_CTableHeader { pub tableLog: BYTE, pub maxSymbolValue: BYTE, pub unused: [BYTE; core::mem::size_of::<usize>() - 2] }

extern "C" {
    pub fn HUF_readCTableHeader(ctable: *const HUF_CElt) -> HUF_CTableHeader;
    pub fn HUF_selectDecoder(dst_size: usize, c_src_size: usize) -> U32;
    pub fn HUF_compress1X_usingCTable(dst: *mut c_void, dst_size: usize, src: *const c_void, src_size: usize, ctable: *const HUF_CElt, flags: i32) -> usize;
    pub fn HUF_compress1X_repeat(dst: *mut c_void, dst_size: usize, src: *const c_void, src_size: usize, max_symbol_value: u32, table_log: u32, workspace: *mut c_void, wksp_size: usize, huf_table: *mut HUF_CElt, repeat: *mut HUF_repeat, flags: i32) -> usize;
    pub fn HUF_decompress1X_DCtx_wksp(dctx: *mut HUF_DTable, dst: *mut c_void, dst_size: usize, c_src: *const c_void, c_src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    /* Declared unless HUF_FORCE_DECOMPRESS_X1 is defined. */
    pub fn HUF_decompress1X2_DCtx_wksp(dctx: *mut HUF_DTable, dst: *mut c_void, dst_size: usize, c_src: *const c_void, c_src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    pub fn HUF_decompress1X_usingDTable(dst: *mut c_void, max_dst_size: usize, c_src: *const c_void, c_src_size: usize, dtable: *const HUF_DTable, flags: i32) -> usize;
    /* Declared unless HUF_FORCE_DECOMPRESS_X2 is defined. */
    pub fn HUF_decompress1X1_DCtx_wksp(dctx: *mut HUF_DTable, dst: *mut c_void, dst_size: usize, c_src: *const c_void, c_src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    pub fn HUF_decompress4X_usingDTable(dst: *mut c_void, max_dst_size: usize, c_src: *const c_void, c_src_size: usize, dtable: *const HUF_DTable, flags: i32) -> usize;
    pub fn HUF_decompress4X_hufOnly_wksp(dctx: *mut HUF_DTable, dst: *mut c_void, dst_size: usize, c_src: *const c_void, c_src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    pub fn HUF_readDTableX1_wksp(dtable: *mut HUF_DTable, src: *const c_void, src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
    pub fn HUF_readDTableX2_wksp(dtable: *mut HUF_DTable, src: *const c_void, src_size: usize, workspace: *mut c_void, wksp_size: usize, flags: i32) -> usize;
}

pub const HUF_DECOMPRESS_WORKSPACE_SIZE: usize = (2 << 10) + (1 << 9);
pub const HUF_DECOMPRESS_WORKSPACE_SIZE_U32: usize = HUF_DECOMPRESS_WORKSPACE_SIZE / core::mem::size_of::<U32>();
extern "C" { pub fn FSE_DECOMPRESS_WKSP_SIZE_U32(a: usize, b: usize) -> usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: Zlib
// Translated from dfltcc.h. C includes and build-time dependencies are
// intentionally left to the surrounding translation unit.

// Tuning parameters.
pub const DFLTCC_LEVEL_MASK: u32 = 0x2; // DFLTCC compression for level 1 only
pub const DFLTCC_LEVEL_MASK_DEBUG: u32 = 0x3fe; // DFLTCC compression for all levels
pub const DFLTCC_BLOCK_SIZE: u32 = 1048576;
pub const DFLTCC_FIRST_FHT_BLOCK_SIZE: u32 = 4096;
pub const DFLTCC_DHT_MIN_SAMPLE_SIZE: u32 = 4096;
pub const DFLTCC_RIBM: u32 = 0;
pub const DFLTCC_FACILITY: u32 = 151;

// Parameter Block for Query Available Functions.
#[repr(C)]
pub struct dfltcc_qaf_param {
    pub fns: [i8; 16],
    pub reserved1: [i8; 8],
    pub fmts: [i8; 2],
    pub reserved2: [i8; 6],
}

pub const DFLTCC_FMT0: u32 = 0;

// Parameter Block for Generate Dynamic-Huffman Table, Compress and Expand.
// C bit-fields are represented by their containing words; masks and shifts
// retain the source fields' intent for callers manipulating the block.
#[repr(C)]
pub struct dfltcc_param_v0 {
    pub pbvn: u16,
    pub mvn: u8,
    pub ribm: u8,
    pub reserved32_cf: u32,
    pub reserved64: [u8; 8],
    pub flags128_159: u32,
    pub oesc: u8,
    pub reserved160_ifs: u16,
    pub ifl: u16,
    pub reserved192: [u8; 8],
    pub reserved256: [u8; 8],
    pub reserved320: [u8; 4],
    pub hl: u16,
    pub reserved368_ho: u16,
    pub cv: u32,
    pub eobs_reserved431: u16,
    pub eobl_reserved436: u16,
    pub reserved448_cdhtl: u16,
    pub reserved464: [u8; 6],
    pub cdht: [u8; 288],
    pub reserved: [u8; 32],
    pub csb: [u8; 1152],
}

pub const CVT_CRC32: u32 = 0;
pub const CVT_ADLER32: u32 = 1;
pub const HTT_FIXED: u32 = 0;
pub const HTT_DYNAMIC: u32 = 1;

// Extension of inflate_state and deflate_state for DFLTCC.
#[repr(C)]
pub struct dfltcc_state {
    pub param: dfltcc_param_v0,
    pub af: dfltcc_qaf_param,
    pub msg: [i8; 64],
}

// Extension of inflate_state and deflate_state for DFLTCC.
#[repr(C)]
pub struct dfltcc_deflate_state {
    pub common: dfltcc_state,
    pub level_mask: u64, // uLong
    pub block_size: u64, // uLong
    pub block_threshold: u64, // uLong
    pub dht_threshold: u64, // uLong
}

extern "C" {
    pub fn dfltcc_reset_state(dfltcc_state: *mut dfltcc_state);
    pub static zlib_dfltcc_support: i32;
    pub fn test_facility(facility: u32) -> i32;
}

// ALIGN_UP(p, size)
#[inline]
pub const fn ALIGN_UP(p: usize, size: usize) -> usize {
    p.wrapping_add(size.wrapping_sub(1)) & !size.wrapping_sub(1)
}

// Resides right after inflate_state or deflate_state.
#[inline]
pub unsafe fn GET_DFLTCC_STATE<T>(state: *mut T) -> *mut dfltcc_state {
    (state as *mut u8).add(ALIGN_UP(core::mem::size_of::<T>(), 8)) as *mut dfltcc_state
}

#[inline]
pub unsafe fn is_dfltcc_enabled() -> i32 {
    (zlib_dfltcc_support != 0 && test_facility(DFLTCC_FACILITY) != 0) as i32
}

#[inline]
pub unsafe fn DEFLATE_DFLTCC_ENABLED() -> i32 {
    is_dfltcc_enabled()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

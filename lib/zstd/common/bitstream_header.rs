/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Translated from bitstream.h. */

// Dependencies supplied by the surrounding translation unit:
// mem.h, compiler.h, debug.h, error_private.h, and bits.h.

pub type BitContainerType = usize;

pub const STREAM_ACCUMULATOR_MIN_32: u32 = 25;
pub const STREAM_ACCUMULATOR_MIN_64: u32 = 57;
// MEM_32bits() is a build-target property and is preserved by this conditional.
pub const STREAM_ACCUMULATOR_MIN: u32 = if usize::BITS == 32 {
    STREAM_ACCUMULATOR_MIN_32
} else {
    STREAM_ACCUMULATOR_MIN_64
};

#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: BitContainerType,
    pub bitPos: u32,
    pub startPtr: *mut i8,
    pub ptr: *mut i8,
    pub endPtr: *mut i8,
}

#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: BitContainerType,
    pub bitsConsumed: u32,
    pub ptr: *const i8,
    pub start: *const i8,
    pub limitPtr: *const i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BIT_DStream_status {
    BIT_DStream_unfinished = 0,
    BIT_DStream_endOfBuffer = 1,
    BIT_DStream_completed = 2,
    BIT_DStream_overflow = 3,
}

pub const BIT_mask: [u32; 32] = [
    0, 1, 3, 7, 0xF, 0x1F, 0x3F, 0x7F, 0xFF, 0x1FF, 0x3FF, 0x7FF,
    0xFFF, 0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF, 0x1FFFF, 0x3FFFF, 0x7FFFF,
    0xFFFFF, 0x1FFFFF, 0x3FFFFF, 0x7FFFFF, 0xFFFFFF, 0x1FFFFFF,
    0x3FFFFFF, 0x7FFFFFF, 0xFFFFFFF, 0x1FFFFFFF, 0x3FFFFFFF, 0x7FFFFFFF,
];
pub const BIT_MASK_SIZE: usize = 32;

#[inline]
pub unsafe fn BIT_getLowerBits(bitContainer: BitContainerType, nbBits: u32) -> BitContainerType {
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    bitContainer & BIT_mask[nbBits as usize] as BitContainerType
}

#[inline]
pub unsafe fn BIT_addBits(bitC: *mut BIT_CStream_t, value: BitContainerType, nbBits: u32) {
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    debug_assert!((nbBits as usize) + (*bitC).bitPos as usize < core::mem::size_of::<BitContainerType>() * 8);
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos += nbBits;
}

#[inline]
pub unsafe fn BIT_addBitsFast(bitC: *mut BIT_CStream_t, value: BitContainerType, nbBits: u32) {
    debug_assert!(value >> nbBits == 0);
    debug_assert!((nbBits as usize) + (*bitC).bitPos as usize < core::mem::size_of::<BitContainerType>() * 8);
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos += nbBits;
}

#[inline]
pub unsafe fn BIT_getUpperBits(bitContainer: BitContainerType, start: u32) -> BitContainerType {
    bitContainer >> start
}

#[inline]
pub unsafe fn BIT_getMiddleBits(bitContainer: BitContainerType, start: u32, nbBits: u32) -> BitContainerType {
    let regMask = (core::mem::size_of::<BitContainerType>() * 8 - 1) as u32;
    debug_assert!((nbBits as usize) < BIT_MASK_SIZE);
    (bitContainer >> (start & regMask)) & ((((1 as BitContainerType) << nbBits) - 1))
}

#[inline]
pub unsafe fn BIT_lookBits(bitD: *const BIT_DStream_t, nbBits: u32) -> BitContainerType {
    BIT_getMiddleBits((*bitD).bitContainer,
        (core::mem::size_of::<BitContainerType>() as u32 * 8) - (*bitD).bitsConsumed - nbBits, nbBits)
}

#[inline]
pub unsafe fn BIT_lookBitsFast(bitD: *const BIT_DStream_t, nbBits: u32) -> BitContainerType {
    let regMask = (core::mem::size_of::<BitContainerType>() * 8 - 1) as u32;
    debug_assert!(nbBits >= 1);
    ((*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)) >> (((regMask + 1 - nbBits) & regMask))
}

#[inline]
pub unsafe fn BIT_skipBits(bitD: *mut BIT_DStream_t, nbBits: u32) { (*bitD).bitsConsumed += nbBits; }

#[inline]
pub unsafe fn BIT_readBits(bitD: *mut BIT_DStream_t, nbBits: u32) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}

// The remaining declarations and implementations use external memory, error,
// assertion, and endian-access helpers supplied by the translated dependencies.
extern "C" {
    pub fn BIT_initCStream(bitC: *mut BIT_CStream_t, dstBuffer: *mut core::ffi::c_void, dstCapacity: usize) -> usize;
    pub fn BIT_flushBits(bitC: *mut BIT_CStream_t);
    pub fn BIT_closeCStream(bitC: *mut BIT_CStream_t) -> usize;
    pub fn BIT_initDStream(bitD: *mut BIT_DStream_t, srcBuffer: *const core::ffi::c_void, srcSize: usize) -> usize;
    pub fn BIT_reloadDStream(bitD: *mut BIT_DStream_t) -> BIT_DStream_status;
    pub fn BIT_endOfDStream(bitD: *const BIT_DStream_t) -> u32;
    pub fn BIT_flushBitsFast(bitC: *mut BIT_CStream_t);
    pub fn BIT_readBitsFast(bitD: *mut BIT_DStream_t, nbBits: u32) -> BitContainerType;
    pub fn BIT_reloadDStream_internal(bitD: *mut BIT_DStream_t) -> BIT_DStream_status;
    pub fn BIT_reloadDStreamFast(bitD: *mut BIT_DStream_t) -> BIT_DStream_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

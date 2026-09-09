/*
 * lz4defs.h -- common and architecture specific defines for the kernel usage
 *
 * LZ4 - Fast LZ compression algorithm
 * Copyright (C) 2011-2016, Yann Collet.
 * BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES ARE DISCLAIMED.
 *
 * Changed for kernel usage by:
 * Sven Schmidt <4sschmid@informatik.uni-hamburg.de>
 *
 * C header dependencies are supplied by the surrounding kernel translation.
 */

#[allow(non_camel_case_types)]
pub type BYTE = u8;
pub type U16 = u16;
pub type U32 = u32;
pub type S32 = i32;
pub type U64 = u64;
pub type uptrval = usize;

#[cfg(CONFIG_64BIT)]
pub const LZ4_ARCH64: i32 = 1;
#[cfg(not(CONFIG_64BIT))]
pub const LZ4_ARCH64: i32 = 0;

#[cfg(__LITTLE_ENDIAN)]
pub const LZ4_LITTLE_ENDIAN: i32 = 1;
#[cfg(not(__LITTLE_ENDIAN))]
pub const LZ4_LITTLE_ENDIAN: i32 = 0;

pub const MINMATCH: usize = 4;
pub const WILDCOPYLENGTH: usize = 8;
pub const LASTLITERALS: usize = 5;
pub const MFLIMIT: usize = WILDCOPYLENGTH + MINMATCH;
pub const MATCH_SAFEGUARD_DISTANCE: usize = (2 * WILDCOPYLENGTH) - MINMATCH;
pub const LZ4_SKIPTRIGGER: usize = 6;
pub const HASH_UNIT: usize = core::mem::size_of::<usize>();
pub const KB: usize = 1 << 10;
pub const MB: usize = 1 << 20;
pub const GB: u32 = 1U32 << 30;
pub const MAX_DISTANCE: usize = LZ4_DISTANCE_MAX as usize;
pub const STEPSIZE: usize = core::mem::size_of::<usize>();
pub const ML_BITS: usize = 4;
pub const ML_MASK: u32 = (1U32 << ML_BITS) - 1;
pub const RUN_BITS: usize = 8 - ML_BITS;
pub const RUN_MASK: u32 = (1U32 << RUN_BITS) - 1;

#[inline(always)]
pub unsafe fn LZ4_read16(ptr: *const core::ffi::c_void) -> U16 {
    get_unaligned(ptr as *const U16)
}

#[inline(always)]
pub unsafe fn LZ4_read32(ptr: *const core::ffi::c_void) -> U32 {
    get_unaligned(ptr as *const U32)
}

#[inline(always)]
pub unsafe fn LZ4_read_ARCH(ptr: *const core::ffi::c_void) -> usize {
    get_unaligned(ptr as *const usize)
}

#[inline(always)]
pub unsafe fn LZ4_write16(mem_ptr: *mut core::ffi::c_void, value: U16) {
    put_unaligned(value, mem_ptr as *mut U16)
}

#[inline(always)]
pub unsafe fn LZ4_write32(mem_ptr: *mut core::ffi::c_void, value: U32) {
    put_unaligned(value, mem_ptr as *mut U32)
}

#[inline(always)]
pub unsafe fn LZ4_readLE16(mem_ptr: *const core::ffi::c_void) -> U16 {
    get_unaligned_le16(mem_ptr)
}

#[inline(always)]
pub unsafe fn LZ4_writeLE16(mem_ptr: *mut core::ffi::c_void, value: U16) {
    put_unaligned_le16(value, mem_ptr)
}

/* LZ4 relies on constant-size memcpy/memmove being inlined. */
#[macro_export]
macro_rules! LZ4_memcpy { ($dst:expr, $src:expr, $size:expr) => { unsafe { core::ptr::copy_nonoverlapping($src, $dst, $size) } }; }
#[macro_export]
macro_rules! LZ4_memmove { ($dst:expr, $src:expr, $size:expr) => { unsafe { core::ptr::copy($src, $dst, $size) } }; }

#[inline(always)]
pub unsafe fn LZ4_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    #[cfg(CONFIG_64BIT)]
    {
        let a: U64 = get_unaligned(src as *const U64);
        put_unaligned(a, dst as *mut U64);
    }
    #[cfg(not(CONFIG_64BIT))]
    {
        let a: U32 = get_unaligned(src as *const U32);
        let b: U32 = get_unaligned((src as *const U32).add(1));
        put_unaligned(a, dst as *mut U32);
        put_unaligned(b, (dst as *mut U32).add(1));
    }
}

#[inline(always)]
pub unsafe fn LZ4_wildCopy(dst_ptr: *mut core::ffi::c_void, src_ptr: *const core::ffi::c_void, dst_end: *mut core::ffi::c_void) {
    let mut d = dst_ptr as *mut BYTE;
    let mut s = src_ptr as *const BYTE;
    let e = dst_end as *mut BYTE;
    loop {
        LZ4_copy8(d as *mut core::ffi::c_void, s as *const core::ffi::c_void);
        d = d.add(8);
        s = s.add(8);
        if d >= e { break; }
    }
}

#[inline(always)]
pub unsafe fn LZ4_NbCommonBytes(val: usize) -> u32 {
    if LZ4_LITTLE_ENDIAN != 0 { __ffs(val) >> 3 } else { (BITS_PER_LONG - 1 - __fls(val)) >> 3 }
}

#[inline(always)]
pub unsafe fn LZ4_count(mut p_in: *const BYTE, mut p_match: *const BYTE, p_in_limit: *const BYTE) -> u32 {
    let p_start = p_in;
    while likely(p_in < p_in_limit.sub(STEPSIZE - 1)) {
        let diff = LZ4_read_ARCH(p_match as *const core::ffi::c_void) ^ LZ4_read_ARCH(p_in as *const core::ffi::c_void);
        if diff == 0 { p_in = p_in.add(STEPSIZE); p_match = p_match.add(STEPSIZE); continue; }
        p_in = p_in.add(LZ4_NbCommonBytes(diff) as usize);
        return p_in.offset_from(p_start) as u32;
    }
    #[cfg(CONFIG_64BIT)]
    if p_in < p_in_limit.sub(3) && LZ4_read32(p_match as *const core::ffi::c_void) == LZ4_read32(p_in as *const core::ffi::c_void) { p_in = p_in.add(4); p_match = p_match.add(4); }
    if p_in < p_in_limit.sub(1) && LZ4_read16(p_match as *const core::ffi::c_void) == LZ4_read16(p_in as *const core::ffi::c_void) { p_in = p_in.add(2); p_match = p_match.add(2); }
    if p_in < p_in_limit && *p_match == *p_in { p_in = p_in.add(1); }
    p_in.offset_from(p_start) as u32
}

#[repr(C)]
pub enum limitedOutput_directive { noLimit = 0, limitedOutput = 1 }
#[repr(C)]
pub enum tableType_t { byPtr, byU32, byU16 }
#[repr(C)]
pub enum dict_directive { noDict = 0, withPrefix64k, usingExtDict }
#[repr(C)]
pub enum dictIssue_directive { noDictIssue = 0, dictSmall }
#[repr(C)]
pub enum endCondition_directive { endOnOutputSize = 0, endOnInputSize = 1 }
#[repr(C)]
pub enum earlyEnd_directive { decode_full_block = 0, partial_decode = 1 }

/* BUILD_BUG_ON(!(c)); */
#[macro_export]
macro_rules! LZ4_STATIC_ASSERT { ($c:expr) => { const _: () = assert!($c); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

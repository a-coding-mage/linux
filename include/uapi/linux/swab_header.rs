/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux type and architecture headers.

/*
 * Casts are necessary for constants, because we never know how for sure
 * how U/UL/ULL map to __u16, __u32, __u64. At least not in a portable way.
 */

#[inline(always)]
pub const fn ___constant_swab16(x: u16) -> u16 {
    (((x & 0x00ff) << 8) | ((x & 0xff00) >> 8)) as u16
}

#[inline(always)]
pub const fn ___constant_swab32(x: u32) -> u32 {
    (((x & 0x000000ff) << 24) |
     ((x & 0x0000ff00) << 8) |
     ((x & 0x00ff0000) >> 8) |
     ((x & 0xff000000) >> 24)) as u32
}

#[inline(always)]
pub const fn ___constant_swab64(x: u64) -> u64 {
    (((x & 0x00000000000000ff) << 56) |
     ((x & 0x000000000000ff00) << 40) |
     ((x & 0x0000000000ff0000) << 24) |
     ((x & 0x00000000ff000000) << 8) |
     ((x & 0x000000ff00000000) >> 8) |
     ((x & 0x0000ff0000000000) >> 24) |
     ((x & 0x00ff000000000000) >> 40) |
     ((x & 0xff00000000000000) >> 56)) as u64
}

#[inline(always)]
pub const fn ___constant_swahw32(x: u32) -> u32 {
    (((x & 0x0000ffff) << 16) | ((x & 0xffff0000) >> 16)) as u32
}

#[inline(always)]
pub const fn ___constant_swahb32(x: u32) -> u32 {
    (((x & 0x00ff00ff) << 8) | ((x & 0xff00ff00) >> 8)) as u32
}

/* Architecture-specific overrides may replace these definitions. */
#[inline(always)] pub const fn __fswab16(val: u16) -> u16 { ___constant_swab16(val) }
#[inline(always)] pub const fn __fswab32(val: u32) -> u32 { ___constant_swab32(val) }
#[inline(always)] pub const fn __fswab64(val: u64) -> u64 { ___constant_swab64(val) }
#[inline(always)] pub const fn __fswahw32(val: u32) -> u32 { ___constant_swahw32(val) }
#[inline(always)] pub const fn __fswahb32(val: u32) -> u32 { ___constant_swahb32(val) }

/** __swab16 - return a byteswapped 16-bit value */
#[inline(always)] pub const fn __swab16(x: u16) -> u16 { x.swap_bytes() }
/** __swab32 - return a byteswapped 32-bit value */
#[inline(always)] pub const fn __swab32(x: u32) -> u32 { x.swap_bytes() }
/** __swab64 - return a byteswapped 64-bit value */
#[inline(always)] pub const fn __swab64(x: u64) -> u64 { x.swap_bytes() }

#[inline(always)]
pub fn __swab(y: usize) -> usize {
    #[cfg(target_pointer_width = "64")]
    { __swab64(y as u64) as usize }
    #[cfg(target_pointer_width = "32")]
    { __swab32(y as u32) as usize }
}

/** __swahw32 - return a word-swapped 32-bit value */
#[inline(always)] pub const fn __swahw32(x: u32) -> u32 { ___constant_swahw32(x) }
/** __swahb32 - return a high and low byte-swapped 32-bit value */
#[inline(always)] pub const fn __swahb32(x: u32) -> u32 { ___constant_swahb32(x) }

/** __swab16p - return a byteswapped 16-bit value from a pointer */
#[inline(always)] pub unsafe fn __swab16p(p: *const u16) -> u16 { __swab16(*p) }
/** __swab32p - return a byteswapped 32-bit value from a pointer */
#[inline(always)] pub unsafe fn __swab32p(p: *const u32) -> u32 { __swab32(*p) }
/** __swab64p - return a byteswapped 64-bit value from a pointer */
#[inline(always)] pub unsafe fn __swab64p(p: *const u64) -> u64 { __swab64(*p) }
/** __swahw32p - return a wordswapped 32-bit value from a pointer */
#[inline(always)] pub unsafe fn __swahw32p(p: *const u32) -> u32 { __swahw32(*p) }
/** __swahb32p - return a high and low byteswapped 32-bit value from a pointer */
#[inline(always)] pub unsafe fn __swahb32p(p: *const u32) -> u32 { __swahb32(*p) }

/** __swab16s - byteswap a 16-bit value in-place */
#[inline(always)] pub unsafe fn __swab16s(p: *mut u16) { *p = __swab16p(p as *const u16); }
/** __swab32s - byteswap a 32-bit value in-place */
#[inline(always)] pub unsafe fn __swab32s(p: *mut u32) { *p = __swab32p(p as *const u32); }
/** __swab64s - byteswap a 64-bit value in-place */
#[inline(always)] pub unsafe fn __swab64s(p: *mut u64) { *p = __swab64p(p as *const u64); }
/** __swahw32s - wordswap a 32-bit value in-place */
#[inline(always)] pub unsafe fn __swahw32s(p: *mut u32) { *p = __swahw32p(p as *const u32); }
/** __swahb32s - high and low byteswap a 32-bit value in-place */
#[inline(always)] pub unsafe fn __swahb32s(p: *mut u32) { *p = __swahb32p(p as *const u32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

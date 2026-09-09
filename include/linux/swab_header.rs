/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations and macros are supplied by <uapi/linux/swab.h>.

// C preprocessor aliases:
// swab16  -> __swab16,  swab32  -> __swab32,  swab64  -> __swab64
// swab    -> __swab,    swahw32 -> __swahw32, swahb32 -> __swahb32
// swab16p -> __swab16p, swab32p -> __swab32p, swab64p -> __swab64p
// swahw32p -> __swahw32p, swahb32p -> __swahb32p
// swab16s -> __swab16s, swab32s -> __swab32s, swab64s -> __swab64s
// swahw32s -> __swahw32s, swahb32s -> __swahb32s

#[inline]
pub unsafe fn swab16_array(mut buf: *mut u16, mut words: u32) {
    while words != 0 {
        // Equivalent to the source macro alias: swab16s(buf).
        __swab16s(buf);
        words = words.wrapping_sub(1);
        buf = buf.add(1);
    }
}

#[inline]
pub unsafe fn swab32_array(mut buf: *mut u32, mut words: u32) {
    while words != 0 {
        // Equivalent to the source macro alias: swab32s(buf).
        __swab32s(buf);
        words = words.wrapping_sub(1);
        buf = buf.add(1);
    }
}

#[inline]
pub unsafe fn swab64_array(mut buf: *mut u64, mut words: u32) {
    while words != 0 {
        // Equivalent to the source macro alias: swab64s(buf).
        __swab64s(buf);
        words = words.wrapping_sub(1);
        buf = buf.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

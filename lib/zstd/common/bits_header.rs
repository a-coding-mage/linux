/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * Rust translation of zstd/common/bits.h.
 * The types U16, U32, U64 and the MEM_* helpers are supplied by the
 * corresponding translated memory header.
 */

pub unsafe fn ZSTD_countTrailingZeros32_fallback(val: U32) -> u32 {
    assert!(val != 0);
    let de_bruijn_byte_pos: [U32; 32] = [
        0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
        31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9,
    ];
    let neg_val = (0u32).wrapping_sub(val);
    let index = (val & neg_val).wrapping_mul(0x077CB531u32) >> 27;
    de_bruijn_byte_pos[index as usize]
}

pub unsafe fn ZSTD_countTrailingZeros32(val: U32) -> u32 {
    assert!(val != 0);
    // The C implementation uses __builtin_ctz when available and the fallback otherwise.
    val.trailing_zeros()
}

pub unsafe fn ZSTD_countLeadingZeros32_fallback(mut val: U32) -> u32 {
    assert!(val != 0);
    let de_bruijn_clz: [U32; 32] = [
        0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30,
        8, 12, 20, 28, 15, 17, 24, 7, 19, 27, 23, 6, 26, 5, 4, 31,
    ];
    val |= val >> 1;
    val |= val >> 2;
    val |= val >> 4;
    val |= val >> 8;
    val |= val >> 16;
    31 - de_bruijn_clz[((val.wrapping_mul(0x07C4ACDDu32)) >> 27) as usize]
}

pub unsafe fn ZSTD_countLeadingZeros32(val: U32) -> u32 {
    assert!(val != 0);
    // The C implementation uses __builtin_clz when available and the fallback otherwise.
    val.leading_zeros()
}

pub unsafe fn ZSTD_countTrailingZeros64(val: U64) -> u32 {
    assert!(val != 0);
    // The C implementation uses __builtin_ctzll on suitable GNU LP64 targets.
    val.trailing_zeros()
}

pub unsafe fn ZSTD_countLeadingZeros64(val: U64) -> u32 {
    assert!(val != 0);
    // The C implementation uses __builtin_clzll when available.
    val.leading_zeros()
}

pub unsafe fn ZSTD_NbCommonBytes(val: usize) -> u32 {
    if MEM_isLittleEndian() {
        if MEM_64bits() {
            ZSTD_countTrailingZeros64(val as U64) >> 3
        } else {
            ZSTD_countTrailingZeros32(val as U32) >> 3
        }
    } else {
        // Big Endian CPU
        if MEM_64bits() {
            ZSTD_countLeadingZeros64(val as U64) >> 3
        } else {
            ZSTD_countLeadingZeros32(val as U32) >> 3
        }
    }
}

pub unsafe fn ZSTD_highbit32(val: U32) -> u32 {
    assert!(val != 0);
    31 - ZSTD_countLeadingZeros32(val)
}

/* ZSTD_rotateRight_*():
 * Rotates a bitfield to the right by "count" bits.
 * https://en.wikipedia.org/w/index.php?title=Circular_shift&oldid=991635599#Implementing_circular_shifts
 */
pub unsafe fn ZSTD_rotateRight_U64(value: U64, mut count: U32) -> U64 {
    assert!(count < 64);
    count &= 0x3F; // for fickle pattern recognition
    (value >> count) | (value << ((0u32.wrapping_sub(count) & 0x3F)))
}

pub unsafe fn ZSTD_rotateRight_U32(value: U32, mut count: U32) -> U32 {
    assert!(count < 32);
    count &= 0x1F; // for fickle pattern recognition
    (value >> count) | (value << (0u32.wrapping_sub(count) & 0x1F))
}

pub unsafe fn ZSTD_rotateRight_U16(value: U16, mut count: U32) -> U16 {
    assert!(count < 16);
    count &= 0x0F; // for fickle pattern recognition
    (value >> count) | (value << (0u32.wrapping_sub(count) & 0x0F))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

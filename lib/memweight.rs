// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation. The original dependencies
// are supplied by the surrounding kernel environment.

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;

extern "C" {
    fn hweight8(x: u8) -> c_int;
    fn bitmap_weight(bitmap: *const c_ulong, bits: usize) -> c_int;
}

// BITS_PER_LONG is supplied by the target kernel configuration.
const BITS_PER_LONG: usize = size_of::<c_ulong>() * 8;
const INT_MAX: usize = c_int::MAX as usize;

/**
 * memweight - count the total number of bits set in memory area
 * @ptr: pointer to the start of the area
 * @bytes: the size of the area
 */
pub unsafe fn memweight(ptr: *const c_void, mut bytes: usize) -> usize {
    let mut ret: usize = 0;
    let longs: usize;
    let mut bitmap = ptr as *const u8;

    while bytes > 0 && (bitmap as usize) % size_of::<c_ulong>() != 0 {
        ret += hweight8(*bitmap) as usize;
        bytes -= 1;
        bitmap = bitmap.add(1);
    }

    longs = bytes / size_of::<c_ulong>();
    if longs != 0 {
        // Equivalent to the source BUG_ON(longs >= INT_MAX / BITS_PER_LONG).
        if longs >= INT_MAX / BITS_PER_LONG {
            core::intrinsics::abort();
        }
        ret += bitmap_weight(bitmap as *const c_ulong, longs * BITS_PER_LONG) as usize;
        bytes -= longs * size_of::<c_ulong>();
        bitmap = bitmap.add(longs * size_of::<c_ulong>());
    }
    /*
     * The reason that this last loop is distinct from the preceding
     * bitmap_weight() call is to compute 1-bits in the last region smaller
     * than sizeof(long) properly on big-endian systems.
     */
    while bytes > 0 {
        ret += hweight8(*bitmap) as usize;
        bytes -= 1;
        bitmap = bitmap.add(1);
    }

    ret
}

// EXPORT_SYMBOL(memweight);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the kernel headers:
// `BITS_PER_LONG`, `bitmap_first_word_mask`, and `__ffs`.

/// Common helper for the `find_next_bit` function family.
///
/// This is the direct Rust equivalent of the C `FIND_NEXT_BIT` macro.  The
/// caller supplies the word-fetch operation and the optional post-processing
/// operation through the `zero` flag.
#[inline]
unsafe fn find_next_bit(addr: *const core::ffi::c_ulong,
                        nbits: core::ffi::c_ulong,
                        start: core::ffi::c_ulong,
                        zero: bool) -> core::ffi::c_ulong {
    let mut sz = nbits;
    let start = start;

    if start >= sz {
        return sz;
    }

    let mut mask = bitmap_first_word_mask(start);
    let mut idx = start / BITS_PER_LONG;
    let mut tmp = if zero {
        (!*addr.add(idx as usize)) & mask
    } else {
        *addr.add(idx as usize) & mask
    };

    while tmp == 0 {
        if (idx + 1) * BITS_PER_LONG >= sz {
            return sz;
        }
        idx += 1;
        tmp = if zero {
            !*addr.add(idx as usize)
        } else {
            *addr.add(idx as usize)
        };
    }

    let found = if zero { __ffs(tmp) } else { __ffs(tmp) };
    sz = core::cmp::min(idx * BITS_PER_LONG + found, sz);
    sz
}

#[no_mangle]
pub unsafe extern "C" fn _find_next_bit(
    addr: *const core::ffi::c_ulong,
    nbits: core::ffi::c_ulong,
    start: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    find_next_bit(addr, nbits, start, false)
}

#[no_mangle]
pub unsafe extern "C" fn _find_next_zero_bit(
    addr: *const core::ffi::c_ulong,
    nbits: core::ffi::c_ulong,
    start: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    find_next_bit(addr, nbits, start, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from include/linux/find.h.
 *
 * C header guard and include directives are intentionally omitted. The original
 * file requires inclusion through <linux/bitmap.h> and depends on bitops
 * helpers/macros such as small_const_nbits, unlikely, GENMASK, __ffs, and ffz.
 */

unsafe extern "C" {
    pub fn _find_next_bit(
        addr1: *const core::ffi::c_ulong,
        nbits: core::ffi::c_ulong,
        start: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _find_next_and_bit(
        addr1: *const core::ffi::c_ulong,
        addr2: *const core::ffi::c_ulong,
        nbits: core::ffi::c_ulong,
        start: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _find_next_zero_bit(
        addr: *const core::ffi::c_ulong,
        nbits: core::ffi::c_ulong,
        start: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _find_first_bit(
        addr: *const core::ffi::c_ulong,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _find_first_and_bit(
        addr1: *const core::ffi::c_ulong,
        addr2: *const core::ffi::c_ulong,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _find_first_zero_bit(
        addr: *const core::ffi::c_ulong,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
}

/*
 * Original condition: #ifndef find_next_bit
 *
 * find_next_bit - find the next set bit in a memory region
 * @addr: The address to base the search on
 * @size: The bitmap size in bits
 * @offset: The bitnumber to start searching at
 *
 * Returns the bit number for the next set bit
 * If no bits are set, returns @size.
 */
#[inline]
pub unsafe fn find_next_bit(
    addr: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
    offset: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong;

        if unlikely(offset >= size) {
            return size;
        }

        val = unsafe { *addr } & GENMASK(size.wrapping_sub(1), offset);
        return if val != 0 { __ffs(val) } else { size };
    }

    unsafe { _find_next_bit(addr, size, offset) }
}

/*
 * Original condition: #ifndef find_next_and_bit
 *
 * find_next_and_bit - find the next set bit in both memory regions
 * @addr1: The first address to base the search on
 * @addr2: The second address to base the search on
 * @size: The bitmap size in bits
 * @offset: The bitnumber to start searching at
 *
 * Returns the bit number for the next set bit
 * If no bits are set, returns @size.
 */
#[inline]
pub unsafe fn find_next_and_bit(
    addr1: *const core::ffi::c_ulong,
    addr2: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
    offset: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong;

        if unlikely(offset >= size) {
            return size;
        }

        val = unsafe { *addr1 } & unsafe { *addr2 } & GENMASK(size.wrapping_sub(1), offset);
        return if val != 0 { __ffs(val) } else { size };
    }

    unsafe { _find_next_and_bit(addr1, addr2, size, offset) }
}

/*
 * Original condition: #ifndef find_next_zero_bit
 *
 * find_next_zero_bit - find the next cleared bit in a memory region
 * @addr: The address to base the search on
 * @size: The bitmap size in bits
 * @offset: The bitnumber to start searching at
 *
 * Returns the bit number of the next zero bit
 * If no bits are zero, returns @size.
 */
#[inline]
pub unsafe fn find_next_zero_bit(
    addr: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
    offset: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong;

        if unlikely(offset >= size) {
            return size;
        }

        val = unsafe { *addr } | !GENMASK(size.wrapping_sub(1), offset);
        return if val == !0 as core::ffi::c_ulong {
            size
        } else {
            ffz(val)
        };
    }

    unsafe { _find_next_zero_bit(addr, size, offset) }
}

/*
 * Original condition: #ifndef find_first_bit
 *
 * find_first_bit - find the first set bit in a memory region
 * @addr: The address to start the search at
 * @size: The maximum number of bits to search
 *
 * Returns the bit number of the first set bit.
 * If no bits are set, returns @size.
 */
#[inline]
pub unsafe fn find_first_bit(
    addr: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong = unsafe { *addr } & GENMASK(size.wrapping_sub(1), 0);

        return if val != 0 { __ffs(val) } else { size };
    }

    unsafe { _find_first_bit(addr, size) }
}

/*
 * Original condition: #ifndef find_first_and_bit
 *
 * find_first_and_bit - find the first set bit in both memory regions
 * @addr1: The first address to base the search on
 * @addr2: The second address to base the search on
 * @size: The bitmap size in bits
 *
 * Returns the bit number for the next set bit
 * If no bits are set, returns @size.
 */
#[inline]
pub unsafe fn find_first_and_bit(
    addr1: *const core::ffi::c_ulong,
    addr2: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong =
            unsafe { *addr1 } & unsafe { *addr2 } & GENMASK(size.wrapping_sub(1), 0);

        return if val != 0 { __ffs(val) } else { size };
    }

    unsafe { _find_first_and_bit(addr1, addr2, size) }
}

/*
 * Original condition: #ifndef find_first_zero_bit
 *
 * find_first_zero_bit - find the first cleared bit in a memory region
 * @addr: The address to start the search at
 * @size: The maximum number of bits to search
 *
 * Returns the bit number of the first cleared bit.
 * If no bits are zero, returns @size.
 */
#[inline]
pub unsafe fn find_first_zero_bit(
    addr: *const core::ffi::c_ulong,
    size: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    if small_const_nbits(size) {
        let val: core::ffi::c_ulong = unsafe { *addr } | !GENMASK(size.wrapping_sub(1), 0);

        return if val == !0 as core::ffi::c_ulong {
            size
        } else {
            ffz(val)
        };
    }

    unsafe { _find_first_zero_bit(addr, size) }
}

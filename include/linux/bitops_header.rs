/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from include/linux/bitops.h. */

/* Depends on: asm/types.h, linux/bits.h, linux/typecheck.h,
 * uapi/linux/kernel.h */

#[allow(non_snake_case)]
#[inline(always)]
pub const fn BITS_TO_LONGS(nr: usize) -> usize {
    nr.div_ceil(core::mem::size_of::<kernel::ffi::c_long>() * BITS_PER_BYTE)
}
#[allow(non_snake_case)]
#[inline(always)]
pub const fn BITS_TO_U64(nr: usize) -> usize {
    nr.div_ceil(core::mem::size_of::<u64>() * BITS_PER_BYTE)
}
#[allow(non_snake_case)]
#[inline(always)]
pub const fn BITS_TO_U32(nr: usize) -> usize {
    nr.div_ceil(core::mem::size_of::<u32>() * BITS_PER_BYTE)
}
#[allow(non_snake_case)]
#[inline(always)]
pub const fn BITS_TO_BYTES(nr: usize) -> usize {
    nr.div_ceil(core::mem::size_of::<kernel::ffi::c_char>() * BITS_PER_BYTE)
}

#[allow(non_snake_case)]
#[inline(always)]
pub const fn BYTES_TO_BITS(nb: usize) -> usize {
    nb * BITS_PER_BYTE
}

extern "C" {
    pub fn __sw_hweight8(w: kernel::ffi::c_uint) -> kernel::ffi::c_uint;
    pub fn __sw_hweight16(w: kernel::ffi::c_uint) -> kernel::ffi::c_uint;
    pub fn __sw_hweight32(w: kernel::ffi::c_uint) -> kernel::ffi::c_uint;
    pub fn __sw_hweight64(w: __u64) -> kernel::ffi::c_ulong;
}

/*
 * Defined here because those may be needed by architecture-specific static
 * inlines.
 */

/* Depends on: asm-generic/bitops/generic-non-atomic.h */

/*
 * Many architecture-specific non-atomic bitops contain inline asm code and due
 * to that the compiler can't optimize them to compile-time expressions or
 * constants. In contrary, generic_*() helpers are defined in pure C and
 * compilers optimize them just well.
 * Therefore, to make `unsigned long foo = 0; __set_bit(BAR, &foo)` effectively
 * equal to `unsigned long foo = BIT(BAR)`, pick the generic C alternative when
 * the arguments can be resolved at compile time. That expression itself is a
 * constant and doesn't bring any functional changes to the rest of cases.
 *
 * Rust has no __builtin_constant_p(), so bitop() always selects op().
 */

/*
 * The following macros are non-atomic versions of their non-underscored
 * counterparts.
 */
#[inline(always)]
pub unsafe fn __set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe { ___set_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn __clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe { ___clear_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn __change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe { ___change_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn __test_and_set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { ___test_and_set_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn __test_and_clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { ___test_and_clear_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn __test_and_change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { ___test_and_change_bit(nr, addr) }
}

#[inline(always)]
pub unsafe fn test_bit(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe { _test_bit(nr, addr) }
}
#[inline(always)]
pub unsafe fn test_bit_acquire(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe { _test_bit_acquire(nr, addr) }
}

/*
 * Include this here because some architectures need generic_ffs/fls in
 * scope
 */
/* Depends on: asm/bitops.h */

/* __check_bitop_pr(): the Rust signatures above are the type check. */

#[inline]
pub const fn get_bitmask_order(count: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    let order: kernel::ffi::c_int;

    order = fls(count);
    order /* We could be slightly more clever with -1 here... */
}

#[inline(always)]
pub fn hweight_long(w: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    if core::mem::size_of::<kernel::ffi::c_ulong>() == 4 {
        hweight32!(w as u32) as kernel::ffi::c_ulong
    } else {
        hweight64!(w as __u64) as kernel::ffi::c_ulong
    }
}

/**
 * rol64 - rotate a 64-bit value left
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn rol64(word: __u64, shift: kernel::ffi::c_uint) -> __u64 {
    (word << (shift & 63)) | (word >> (shift.wrapping_neg() & 63))
}

/**
 * ror64 - rotate a 64-bit value right
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn ror64(word: __u64, shift: kernel::ffi::c_uint) -> __u64 {
    (word >> (shift & 63)) | (word << (shift.wrapping_neg() & 63))
}

/**
 * rol32 - rotate a 32-bit value left
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn rol32(word: __u32, shift: kernel::ffi::c_uint) -> __u32 {
    (word << (shift & 31)) | (word >> (shift.wrapping_neg() & 31))
}

/**
 * ror32 - rotate a 32-bit value right
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn ror32(word: __u32, shift: kernel::ffi::c_uint) -> __u32 {
    (word >> (shift & 31)) | (word << (shift.wrapping_neg() & 31))
}

/*
 * The 16- and 8-bit rotates shift the value promoted to int, as C does, and
 * truncate the result.
 */

/**
 * rol16 - rotate a 16-bit value left
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn rol16(word: __u16, shift: kernel::ffi::c_uint) -> __u16 {
    (((word as u32) << (shift & 15)) | ((word as u32) >> (shift.wrapping_neg() & 15))) as __u16
}

/**
 * ror16 - rotate a 16-bit value right
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn ror16(word: __u16, shift: kernel::ffi::c_uint) -> __u16 {
    (((word as u32) >> (shift & 15)) | ((word as u32) << (shift.wrapping_neg() & 15))) as __u16
}

/**
 * rol8 - rotate an 8-bit value left
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn rol8(word: __u8, shift: kernel::ffi::c_uint) -> __u8 {
    (((word as u32) << (shift & 7)) | ((word as u32) >> (shift.wrapping_neg() & 7))) as __u8
}

/**
 * ror8 - rotate an 8-bit value right
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub const fn ror8(word: __u8, shift: kernel::ffi::c_uint) -> __u8 {
    (((word as u32) >> (shift & 7)) | ((word as u32) << (shift.wrapping_neg() & 7))) as __u8
}

/**
 * sign_extend32 - sign extend a 32-bit value using specified bit as sign-bit
 * @value: value to sign extend
 * @index: 0 based bit index (0 <= index < 32) to sign bit
 *
 * This is safe to use for 16- and 8-bit types as well.
 *
 * Return: 32-bit sign extended value
 */
#[inline(always)]
pub const fn sign_extend32(value: __u32, index: kernel::ffi::c_int) -> __s32 {
    let shift: __u8 = (31 - index) as __u8;
    ((value << shift) as __s32) >> shift
}

/**
 * sign_extend64 - sign extend a 64-bit value using specified bit as sign-bit
 * @value: value to sign extend
 * @index: 0 based bit index (0 <= index < 64) to sign bit
 *
 * This is safe to use for 32-, 16- and 8-bit types as well.
 *
 * Return: 64-bit sign extended value
 */
#[inline(always)]
pub const fn sign_extend64(value: __u64, index: kernel::ffi::c_int) -> __s64 {
    let shift: __u8 = (63 - index) as __u8;
    ((value << shift) as __s64) >> shift
}

#[inline]
pub const fn fls_long(l: kernel::ffi::c_ulong) -> kernel::ffi::c_uint {
    if core::mem::size_of::<kernel::ffi::c_ulong>() == 4 {
        return fls(l as u32) as kernel::ffi::c_uint;
    }
    fls64(l as u64) as kernel::ffi::c_uint
}

#[inline]
pub const fn get_count_order(count: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    if count == 0 {
        return -1;
    }

    fls(count - 1)
}

/**
 * get_count_order_long - get order after rounding @l up to power of 2
 * @l: parameter
 *
 * it is same as get_count_order() but with long type parameter
 */
#[inline]
pub const fn get_count_order_long(l: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    if l == 0 {
        return -1;
    }
    fls_long(l - 1) as kernel::ffi::c_int
}

/**
 * parity8 - get the parity of an u8 value
 * @val: the value to be examined
 *
 * Determine the parity of the u8 argument.
 *
 * Returns:
 * 0 for even parity, 1 for odd parity
 *
 * Note: This function informs you about the current parity. Example to bail
 * out when parity is odd:
 *
 *	if (parity8(val) == 1)
 *		return -EBADMSG;
 *
 * If you need to calculate a parity bit, you need to draw the conclusion from
 * this result yourself. Example to enforce odd parity, parity bit is bit 7:
 *
 *	if (parity8(val) == 0)
 *		val ^= BIT(7);
 */
#[inline]
pub const fn parity8(mut val: u8) -> kernel::ffi::c_int {
    /*
     * One explanation of this algorithm:
     * https://funloop.org/codex/problem/parity/README.html
     */
    val ^= val >> 4;
    (0x6996 >> (val & 0xf)) & 1
}

/**
 * __ffs64 - find first set bit in a 64 bit word
 * @word: The 64 bit word
 *
 * On 64 bit arches this is a synonym for __ffs
 * The result is not defined if no bits are set, so check that @word
 * is non-zero before calling this.
 */
#[inline]
pub const fn __ffs64(word: u64) -> kernel::ffi::c_uint {
    #[cfg(not(CONFIG_64BIT))]
    if (word as u32) == 0 {
        return __ffs((word >> 32) as u32 as kernel::ffi::c_ulong) as kernel::ffi::c_uint + 32;
    }
    __ffs(word as kernel::ffi::c_ulong) as kernel::ffi::c_uint
}

/**
 * fns - find N'th set bit in a word
 * @word: The word to search
 * @n: Bit to find
 */
#[inline]
pub const fn fns(mut word: kernel::ffi::c_ulong, mut n: kernel::ffi::c_uint) -> kernel::ffi::c_uint {
    while word != 0 && {
        let more = n != 0;
        n = n.wrapping_sub(1);
        more
    } {
        word &= word - 1;
    }

    if word != 0 { __ffs(word) as kernel::ffi::c_uint } else { BITS_PER_LONG as kernel::ffi::c_uint }
}

/**
 * assign_bit - Assign value to a bit in memory
 * @nr: the bit to set
 * @addr: the address to start counting from
 * @value: the value to assign
 */
#[macro_export]
macro_rules! assign_bit {
    ($nr:expr, $addr:expr, $value:expr) => {
        if $value { set_bit($nr, $addr) } else { clear_bit($nr, $addr) }
    };
}

#[macro_export]
macro_rules! __assign_bit {
    ($nr:expr, $addr:expr, $value:expr) => {
        if $value { __set_bit($nr, $addr) } else { __clear_bit($nr, $addr) }
    };
}

/*
 * typecheck_pointer(*(addr)): the Rust callers pass `&raw mut p` of a raw
 * pointer variable, which the casts below require.
 */

/**
 * __ptr_set_bit - Set bit in a pointer's value
 * @nr: the bit to set
 * @addr: the address of the pointer variable
 *
 * Example:
 *	void *p = foo();
 *	__ptr_set_bit(bit, &p);
 */
#[macro_export]
macro_rules! __ptr_set_bit {
    ($nr:expr, $addr:expr) => {
        __set_bit($nr, ($addr).cast::<kernel::ffi::c_ulong>())
    };
}

/**
 * __ptr_clear_bit - Clear bit in a pointer's value
 * @nr: the bit to clear
 * @addr: the address of the pointer variable
 *
 * Example:
 *	void *p = foo();
 *	__ptr_clear_bit(bit, &p);
 */
#[macro_export]
macro_rules! __ptr_clear_bit {
    ($nr:expr, $addr:expr) => {
        __clear_bit($nr, ($addr).cast::<kernel::ffi::c_ulong>())
    };
}

/**
 * __ptr_test_bit - Test bit in a pointer's value
 * @nr: the bit to test
 * @addr: the address of the pointer variable
 *
 * Example:
 *	void *p = foo();
 *	if (__ptr_test_bit(bit, &p)) {
 *	        ...
 *	} else {
 *		...
 *	}
 */
#[macro_export]
macro_rules! __ptr_test_bit {
    ($nr:expr, $addr:expr) => {
        test_bit($nr, ($addr).cast::<kernel::ffi::c_ulong>().cast_const())
    };
}

/* __KERNEL__ */

#[macro_export]
macro_rules! set_mask_bits {
    ($ptr:expr, $mask:expr, $bits:expr) => {{
        let __ptr = $ptr;
        let mask__ = $mask;
        let bits__ = $bits;
        let mut old__ = READ_ONCE!(*__ptr);
        let mut new__;

        loop {
            new__ = (old__ & !mask__) | bits__;
            if try_cmpxchg!(__ptr, &raw mut old__, new__) {
                break;
            }
        }

        old__
    }};
}

#[macro_export]
macro_rules! bit_clear_unless {
    ($ptr:expr, $clear:expr, $test:expr) => {{
        let __ptr = $ptr;
        let clear__ = $clear;
        let test__ = $test;
        let mut old__ = READ_ONCE!(*__ptr);
        let mut new__;

        loop {
            if old__ & test__ != 0 {
                break;
            }
            new__ = old__ & !clear__;
            if try_cmpxchg!(__ptr, &raw mut old__, new__) {
                break;
            }
        }

        (old__ & test__) == 0
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

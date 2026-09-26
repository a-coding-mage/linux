/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <vdso/bits.h>, <uapi/linux/bits.h>, <linux/build_bug.h>,
// <linux/compiler.h>, and <linux/overflow.h>.

macro_rules! BIT_MASK {
    ($nr:expr) => { (1 as ::core::ffi::c_ulong) << (($nr) % BITS_PER_LONG) };
}

macro_rules! BIT_WORD {
    ($nr:expr) => { (($nr) / BITS_PER_LONG) };
}

macro_rules! BIT_ULL_MASK {
    ($nr:expr) => { (1u64 << (($nr) % BITS_PER_LONG_LONG)) };
}

macro_rules! BIT_ULL_WORD {
    ($nr:expr) => { (($nr) / BITS_PER_LONG_LONG) };
}

const BITS_PER_BYTE: usize = 8;

macro_rules! BITS_PER_TYPE {
    ($type:ty) => { (::core::mem::size_of::<$type>() * BITS_PER_BYTE) };
}

/*
 * Create a contiguous bitmask starting at bit position @l and ending at
 * position @h. For example
 * GENMASK_ULL(39, 21) gives us the 64bit vector 0x000000ffffe00000.
 */

/*
 * Generate a mask for the specified type @t. Additional checks are made to
 * guarantee the value returned fits in that type; the C header relies on
 * -Wshift-count-overflow, Rust rejects overflowing shifts in const evaluation.
 * For example, all these create build errors:
 *
 * - GENMASK(15, 20): wrong argument order
 * - GENMASK(72, 15): doesn't fit unsigned long
 * - GENMASK_U32(33, 15): doesn't fit in a u32
 */
macro_rules! GENMASK_INPUT_CHECK {
    ($h:expr, $l:expr) => {
        const { assert!(!(($l) > ($h)), "GENMASK: wrong argument order") }
    };
}

macro_rules! GENMASK_TYPE {
    ($t:ty, $h:expr, $l:expr) => {{
        GENMASK_INPUT_CHECK!($h, $l);
        (<$t>::MAX << ($l)) & (<$t>::MAX >> (<$t>::BITS - 1 - ($h) as u32))
    }};
}

macro_rules! GENMASK {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(::core::ffi::c_ulong, $h, $l) };
}

macro_rules! GENMASK_ULL {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(::core::ffi::c_ulonglong, $h, $l) };
}

macro_rules! GENMASK_U8 {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(u8, $h, $l) };
}

macro_rules! GENMASK_U16 {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(u16, $h, $l) };
}

macro_rules! GENMASK_U32 {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(u32, $h, $l) };
}

macro_rules! GENMASK_U64 {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(u64, $h, $l) };
}

macro_rules! GENMASK_U128 {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(u128, $h, $l) };
}

/*
 * Fixed-type variants of BIT(), with additional checks like GENMASK_TYPE(). The
 * following examples generate build errors:
 *
 * - BIT_U8(8)
 * - BIT_U32(-1)
 * - BIT_U32(40)
 */
macro_rules! BIT_INPUT_CHECK {
    ($type:ty, $nr:expr) => {
        const { assert!(!(($nr) as i128 >= BITS_PER_TYPE!($type) as i128) && ($nr) as i128 >= 0) }
    };
}

macro_rules! BIT_TYPE {
    ($type:ty, $nr:expr) => {{
        BIT_INPUT_CHECK!($type, $nr);
        (1 as $type) << ($nr)
    }};
}

macro_rules! BIT_U8 {
    ($nr:expr) => { BIT_TYPE!(u8, $nr) };
}

macro_rules! BIT_U16 {
    ($nr:expr) => { BIT_TYPE!(u16, $nr) };
}

macro_rules! BIT_U32 {
    ($nr:expr) => { BIT_TYPE!(u32, $nr) };
}

macro_rules! BIT_U64 {
    ($nr:expr) => { BIT_TYPE!(u64, $nr) };
}

// In assembler builds, BUILD_BUG_ON_ZERO is unavailable; the C header maps
// GENMASK and GENMASK_ULL directly to the architecture-provided forms.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

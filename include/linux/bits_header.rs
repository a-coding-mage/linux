/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <vdso/bits.h>, <uapi/linux/bits.h>, <linux/build_bug.h>,
// <linux/compiler.h>, and <linux/overflow.h>.

macro_rules! BIT_MASK {
    ($nr:expr) => { (1usize << (($nr) % BITS_PER_LONG)) };
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

// C build-time checks and type_max() are supplied by the corresponding
// dependencies. These macros retain their source-level use and intent.
macro_rules! GENMASK_INPUT_CHECK {
    ($h:expr, $l:expr) => { BUILD_BUG_ON_ZERO(const_true(($l) > ($h))) };
}

macro_rules! GENMASK_TYPE {
    ($t:ty, $h:expr, $l:expr) => {
        (($t)(GENMASK_INPUT_CHECK!($h, $l)
            + (type_max!($t) << ($l)
                & type_max!($t) >> (BITS_PER_TYPE!($t) - 1 - ($h))))
    };
}

macro_rules! GENMASK {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(unsigned_long, $h, $l) };
}

macro_rules! GENMASK_ULL {
    ($h:expr, $l:expr) => { GENMASK_TYPE!(unsigned_long_long, $h, $l) };
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

/* Fixed-type variants of BIT(), with the same input checks as GENMASK_TYPE(). */
macro_rules! BIT_INPUT_CHECK {
    ($type:ty, $nr:expr) => { BUILD_BUG_ON_ZERO(const_true(($nr) >= BITS_PER_TYPE!($type))) };
}

macro_rules! BIT_TYPE {
    ($type:ty, $nr:expr) => { (($type)(BIT_INPUT_CHECK!($type, $nr) + BIT_ULL!($nr))) };
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

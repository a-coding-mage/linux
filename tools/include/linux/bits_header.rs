/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// - <vdso/bits.h>
// - <uapi/linux/bits.h>

#[macro_export]
macro_rules! BIT_MASK {
    ($nr:expr) => {
        ((1 as core::ffi::c_ulong) << (($nr) % BITS_PER_LONG))
    };
}

#[macro_export]
macro_rules! BIT_WORD {
    ($nr:expr) => {
        (($nr) / BITS_PER_LONG)
    };
}

#[macro_export]
macro_rules! BIT_ULL_MASK {
    ($nr:expr) => {
        ((1 as core::ffi::c_ulonglong) << (($nr) % BITS_PER_LONG_LONG))
    };
}

#[macro_export]
macro_rules! BIT_ULL_WORD {
    ($nr:expr) => {
        (($nr) / BITS_PER_LONG_LONG)
    };
}

pub const BITS_PER_BYTE: usize = 8;

#[macro_export]
macro_rules! BITS_PER_TYPE {
    ($type:ty) => {
        (core::mem::size_of::<$type>() * BITS_PER_BYTE)
    };
}

/*
 * Create a contiguous bitmask starting at bit position @l and ending at
 * position @h. For example
 * GENMASK_ULL(39, 21) gives us the 64bit vector 0x000000ffffe00000.
 */

// C conditional: #if !defined(__ASSEMBLER__)
/*
 * Missing asm support
 *
 * GENMASK_U*() and BIT_U*() depend on BITS_PER_TYPE() which relies on sizeof(),
 * something not available in asm. Nevertheless, fixed width integers is a C
 * concept. Assembly code can rely on the long and long long versions instead.
 */

// C header dependencies for the non-assembler branch:
// - <linux/build_bug.h>
// - <linux/compiler.h>
// - <linux/overflow.h>

#[macro_export]
macro_rules! GENMASK_INPUT_CHECK {
    ($h:expr, $l:expr) => {{
        const {
            assert!($l <= $h);
        }
        0
    }};
}

/*
 * Generate a mask for the specified type @t. Additional checks are made to
 * guarantee the value returned fits in that type, relying on
 * -Wshift-count-overflow compiler check to detect incompatible arguments.
 * For example, all these create build errors or warnings:
 *
 * - GENMASK(15, 20): wrong argument order
 * - GENMASK(72, 15): doesn't fit unsigned long
 * - GENMASK_U32(33, 15): doesn't fit in a u32
 */
#[macro_export]
macro_rules! GENMASK_TYPE {
    ($t:ty, $h:expr, $l:expr) => {
        ((GENMASK_INPUT_CHECK!($h, $l)
            + (((<$t>::MAX << ($l)) & (<$t>::MAX >> (BITS_PER_TYPE!($t) - 1 - ($h)))))) as $t)
    };
}

#[macro_export]
macro_rules! GENMASK {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(core::ffi::c_ulong, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_ULL {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(core::ffi::c_ulonglong, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_U8 {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(u8, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_U16 {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(u16, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_U32 {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(u32, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_U64 {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(u64, $h, $l)
    };
}

#[macro_export]
macro_rules! GENMASK_U128 {
    ($h:expr, $l:expr) => {
        GENMASK_TYPE!(u128, $h, $l)
    };
}

/*
 * Fixed-type variants of BIT(), with additional checks like GENMASK_TYPE(). The
 * following examples generate compiler warnings due to -Wshift-count-overflow:
 *
 * - BIT_U8(8)
 * - BIT_U32(-1)
 * - BIT_U32(40)
 */
#[macro_export]
macro_rules! BIT_INPUT_CHECK {
    ($type:ty, $nr:expr) => {{
        const {
            assert!($nr < BITS_PER_TYPE!($type));
        }
        0
    }};
}

#[macro_export]
macro_rules! BIT_TYPE {
    ($type:ty, $nr:expr) => {
        ((BIT_INPUT_CHECK!($type, $nr) + BIT_ULL!($nr)) as $type)
    };
}

#[macro_export]
macro_rules! BIT_U8 {
    ($nr:expr) => {
        BIT_TYPE!(u8, $nr)
    };
}

#[macro_export]
macro_rules! BIT_U16 {
    ($nr:expr) => {
        BIT_TYPE!(u16, $nr)
    };
}

#[macro_export]
macro_rules! BIT_U32 {
    ($nr:expr) => {
        BIT_TYPE!(u32, $nr)
    };
}

#[macro_export]
macro_rules! BIT_U64 {
    ($nr:expr) => {
        BIT_TYPE!(u64, $nr)
    };
}

// C conditional: #else /* defined(__ASSEMBLER__) */
/*
 * BUILD_BUG_ON_ZERO is not available in h files included from asm files,
 * disable the input check if that is the case.
 *
 * Assembler branch definitions:
 * #define GENMASK(h, l)     __GENMASK(h, l)
 * #define GENMASK_ULL(h, l) __GENMASK_ULL(h, l)
 */

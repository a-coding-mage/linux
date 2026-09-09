// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for functions and macros in bits.h
 */

// Dependencies supplied by the kernel test and bits headers are intentionally
// left external to this translation.

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;

#[allow(dead_code)]
const _: () = {
    assert!(1u8 == 1u8);
    assert!(1u16 == 1u16);
    assert!(1u32 == 1u32);
    assert!(1u64 == 1u64);
};

/* FIXME: add a test case written in asm for GENMASK() and GENMASK_ULL() */

#[allow(dead_code)]
unsafe fn __genmask_test(test: *mut kunit) {
    KUNIT_EXPECT_EQ(test, 1usize, __GENMASK(0, 0));
    KUNIT_EXPECT_EQ(test, 3usize, __GENMASK(1, 0));
    KUNIT_EXPECT_EQ(test, 6usize, __GENMASK(2, 1));
    KUNIT_EXPECT_EQ(test, 0xFFFF_FFFFusize, __GENMASK(31, 0));
}

#[allow(dead_code)]
unsafe fn __genmask_ull_test(test: *mut kunit) {
    KUNIT_EXPECT_EQ(test, 1u64, __GENMASK_ULL(0, 0));
    KUNIT_EXPECT_EQ(test, 3u64, __GENMASK_ULL(1, 0));
    KUNIT_EXPECT_EQ(test, 0x0000_00FF_FFE0_0000u64, __GENMASK_ULL(39, 21));
    KUNIT_EXPECT_EQ(test, 0xFFFF_FFFF_FFFF_FFFFu64, __GENMASK_ULL(63, 0));
}

#[allow(dead_code)]
unsafe fn genmask_test(test: *mut kunit) {
    KUNIT_EXPECT_EQ(test, 1usize, GENMASK(0, 0));
    KUNIT_EXPECT_EQ(test, 3usize, GENMASK(1, 0));
    KUNIT_EXPECT_EQ(test, 6usize, GENMASK(2, 1));
    KUNIT_EXPECT_EQ(test, 0xFFFF_FFFFusize, GENMASK(31, 0));

    KUNIT_EXPECT_EQ(test, 1u32, GENMASK_U8(0, 0));
    KUNIT_EXPECT_EQ(test, 3u32, GENMASK_U16(1, 0));
    KUNIT_EXPECT_EQ(test, 0x10000u32, GENMASK_U32(16, 16));

    // TEST_GENMASK_FAILURES contains intentional compile failures.
}

#[allow(dead_code)]
unsafe fn genmask_ull_test(test: *mut kunit) {
    KUNIT_EXPECT_EQ(test, 1u64, GENMASK_ULL(0, 0));
    KUNIT_EXPECT_EQ(test, 3u64, GENMASK_ULL(1, 0));
    KUNIT_EXPECT_EQ(test, 0x0000_00FF_FFE0_0000u64, GENMASK_ULL(39, 21));
    KUNIT_EXPECT_EQ(test, 0xFFFF_FFFF_FFFF_FFFFu64, GENMASK_ULL(63, 0));
}

#[allow(dead_code)]
unsafe fn genmask_u128_test(test: *mut kunit) {
    // CONFIG_ARCH_SUPPORTS_INT128 controls this test in the original source.
    #[cfg(target_pointer_width = "128")]
    {
        KUNIT_EXPECT_EQ(test, 1u64, GENMASK_U128(0, 0));
        KUNIT_EXPECT_EQ(test, 3u64, GENMASK_U128(1, 0));
        KUNIT_EXPECT_EQ(test, 6u64, GENMASK_U128(2, 1));
        KUNIT_EXPECT_EQ(test, 0x0000_0000_FFFF_FFFFu64, GENMASK_U128(31, 0));
        KUNIT_EXPECT_EQ(test, 0x0000_00FF_FFE0_0000u64, GENMASK_U128(39, 21));
        KUNIT_EXPECT_EQ(test, 0xFFFF_FFFF_FFFF_FFFFu64, GENMASK_U128(63, 0));
        KUNIT_EXPECT_EQ(test, 0xFFFF_FFFF_FFFF_FFFFu64, GENMASK_U128(64, 0) >> 1);
        KUNIT_EXPECT_EQ(test, 0x0000_0000_FFFF_FFFFu64, GENMASK_U128(81, 50) >> 50);
        KUNIT_EXPECT_EQ(test, 0x0000_0000_00FF_FFFFu64, GENMASK_U128(87, 64) >> 64);
        KUNIT_EXPECT_EQ(test, 0x0000_0000_00FF_0000u64, GENMASK_U128(87, 80) >> 64);
        KUNIT_EXPECT_EQ(test, 0xFFFF_FFFF_FFFF_FFFFu64, GENMASK_U128(127, 0) >> 64);
        KUNIT_EXPECT_EQ(test, GENMASK_U128(127, 0) as u64, 0xFFFF_FFFF_FFFF_FFFFu64);
        KUNIT_EXPECT_EQ(test, 3u64, GENMASK_U128(127, 126) >> 126);
        KUNIT_EXPECT_EQ(test, 1u64, GENMASK_U128(127, 127) >> 127);
    }
}

#[allow(dead_code)]
unsafe fn genmask_input_check_test(test: *mut kunit) {
    let x: u32;
    let y: u32;
    let z: i32;
    let w: i32;

    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(x, 0));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(0, x));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(x, y));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(z, 0));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(0, z));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(z, w));

    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(1, 1));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(39, 21));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(100, 80));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(110, 65));
    KUNIT_EXPECT_EQ(test, 0, GENMASK_INPUT_CHECK(127, 0));
}

#[repr(C)]
pub struct kunit;

// The following symbols are supplied by the kernel bits/KUnit environment.
extern "C" {
    fn __GENMASK(high: u32, low: u32) -> usize;
    fn __GENMASK_ULL(high: u32, low: u32) -> u64;
    fn GENMASK(high: u32, low: u32) -> usize;
    fn GENMASK_ULL(high: u32, low: u32) -> u64;
    fn GENMASK_U8(high: u32, low: u32) -> u8;
    fn GENMASK_U16(high: u32, low: u32) -> u16;
    fn GENMASK_U32(high: u32, low: u32) -> u32;
    fn GENMASK_U128(high: u32, low: u32) -> u128;
    fn GENMASK_INPUT_CHECK(high: u32, low: u32) -> i32;
    fn KUNIT_EXPECT_EQ<T>(test: *mut kunit, left: T, right: T);
}

// KUnit case registration and module metadata are provided by the surrounding
// kernel build and have no standalone Rust item equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

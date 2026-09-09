// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Intel Corporation
 * Copyright (C) 2026 Ryota Sakamoto <sakamo.ryota@gmail.com>
 */

// Dependencies supplied by the kernel and KUnit are intentionally external.

/* use an enum because that's the most common BITMAP usage */
#[repr(isize)]
enum BitopsFun {
    BITOPS_4 = 4,
    BITOPS_7 = 7,
    BITOPS_11 = 11,
    BITOPS_31 = 31,
    BITOPS_88 = 88,
    BITOPS_LENGTH = 256,
}

#[repr(C)]
struct BitopsTestCase {
    str_: *const core::ffi::c_char,
    nr: core::ffi::c_long,
}

static mut BITOPS_CASES: [BitopsTestCase; 5] = [
    BitopsTestCase { str_: b"BITOPS_4\0".as_ptr() as *const _, nr: 4 },
    BitopsTestCase { str_: b"BITOPS_7\0".as_ptr() as *const _, nr: 7 },
    BitopsTestCase { str_: b"BITOPS_11\0".as_ptr() as *const _, nr: 11 },
    BitopsTestCase { str_: b"BITOPS_31\0".as_ptr() as *const _, nr: 31 },
    BitopsTestCase { str_: b"BITOPS_88\0".as_ptr() as *const _, nr: 88 },
];

// KUNIT_ARRAY_PARAM_DESC(bitops, bitops_cases, str);

unsafe fn test_set_bit_clear_bit(test: *mut Kunit) {
    let params = (*test).param_value as *const BitopsTestCase;
    let mut bitmap = [0usize; 4];
    let mut bit_set: i32;
    bitmap_zero(bitmap.as_mut_ptr(), 256);
    set_bit((*params).nr, bitmap.as_mut_ptr());
    KUNIT_EXPECT_TRUE(test, test_bit((*params).nr, bitmap.as_ptr()));
    clear_bit((*params).nr, bitmap.as_mut_ptr());
    KUNIT_EXPECT_FALSE(test, test_bit((*params).nr, bitmap.as_ptr()));
    bit_set = find_first_bit(bitmap.as_ptr(), 256) as i32;
    KUNIT_EXPECT_EQ(test, bit_set, 256);
}

unsafe fn test_change_bit(test: *mut Kunit) {
    let params = (*test).param_value as *const BitopsTestCase;
    let mut bitmap = [0usize; 4];
    let mut bit_set: i32;
    bitmap_zero(bitmap.as_mut_ptr(), 256);
    change_bit((*params).nr, bitmap.as_mut_ptr());
    KUNIT_EXPECT_TRUE(test, test_bit((*params).nr, bitmap.as_ptr()));
    change_bit((*params).nr, bitmap.as_mut_ptr());
    KUNIT_EXPECT_FALSE(test, test_bit((*params).nr, bitmap.as_ptr()));
    bit_set = find_first_bit(bitmap.as_ptr(), 256) as i32;
    KUNIT_EXPECT_EQ(test, bit_set, 256);
}

unsafe fn test_test_and_set_bit_test_and_clear_bit(test: *mut Kunit) {
    let params = (*test).param_value as *const BitopsTestCase;
    let mut bitmap = [0usize; 4];
    let mut bit_set: i32;
    bitmap_zero(bitmap.as_mut_ptr(), 256);
    KUNIT_EXPECT_FALSE(test, test_and_set_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_TRUE(test, test_bit((*params).nr, bitmap.as_ptr()));
    KUNIT_EXPECT_TRUE(test, test_and_set_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_TRUE(test, test_bit((*params).nr, bitmap.as_ptr()));
    KUNIT_EXPECT_TRUE(test, test_and_clear_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_FALSE(test, test_bit((*params).nr, bitmap.as_ptr()));
    KUNIT_EXPECT_FALSE(test, test_and_clear_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_FALSE(test, test_bit((*params).nr, bitmap.as_ptr()));
    bit_set = find_first_bit(bitmap.as_ptr(), 256) as i32;
    KUNIT_EXPECT_EQ(test, bit_set, 256);
}

unsafe fn test_test_and_change_bit(test: *mut Kunit) {
    let params = (*test).param_value as *const BitopsTestCase;
    let mut bitmap = [0usize; 4];
    let mut bit_set: i32;
    bitmap_zero(bitmap.as_mut_ptr(), 256);
    KUNIT_EXPECT_FALSE(test, test_and_change_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_TRUE(test, test_bit((*params).nr, bitmap.as_ptr()));
    KUNIT_EXPECT_TRUE(test, test_and_change_bit((*params).nr, bitmap.as_mut_ptr()));
    KUNIT_EXPECT_FALSE(test, test_bit((*params).nr, bitmap.as_ptr()));
    bit_set = find_first_bit(bitmap.as_ptr(), 256) as i32;
    KUNIT_EXPECT_EQ(test, bit_set, 256);
}

#[repr(C)]
struct OrderTestCase { str_: *const core::ffi::c_char, count: u32, expected: i32 }

static mut ORDER_TEST_CASES: [OrderTestCase; 7] = [
    OrderTestCase { str_: b"0x00000003\0".as_ptr() as *const _, count: 0x00000003, expected: 2 },
    OrderTestCase { str_: b"0x00000004\0".as_ptr() as *const _, count: 0x00000004, expected: 2 },
    OrderTestCase { str_: b"0x00001fff\0".as_ptr() as *const _, count: 0x00001fff, expected: 13 },
    OrderTestCase { str_: b"0x00002000\0".as_ptr() as *const _, count: 0x00002000, expected: 13 },
    OrderTestCase { str_: b"0x50000000\0".as_ptr() as *const _, count: 0x50000000, expected: 31 },
    OrderTestCase { str_: b"0x80000000\0".as_ptr() as *const _, count: 0x80000000, expected: 31 },
    OrderTestCase { str_: b"0x80003000\0".as_ptr() as *const _, count: 0x80003000, expected: 32 },
];

// KUNIT_ARRAY_PARAM_DESC(order, order_test_cases, str);

unsafe fn test_get_count_order(test: *mut Kunit) {
    let params = (*test).param_value as *const OrderTestCase;
    KUNIT_EXPECT_EQ(test, get_count_order((*params).count), (*params).expected);
    KUNIT_EXPECT_EQ(test, get_count_order_long((*params).count), (*params).expected);
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct OrderLongTestCase { str_: *const core::ffi::c_char, count: u64, expected: i32 }

#[cfg(target_pointer_width = "64")]
static mut ORDER_LONG_TEST_CASES: [OrderLongTestCase; 7] = [
    OrderLongTestCase { str_: b"0x0000000300000000\0".as_ptr() as *const _, count: 0x0000000300000000, expected: 34 },
    OrderLongTestCase { str_: b"0x0000000400000000\0".as_ptr() as *const _, count: 0x0000000400000000, expected: 34 },
    OrderLongTestCase { str_: b"0x00001fff00000000\0".as_ptr() as *const _, count: 0x00001fff00000000, expected: 45 },
    OrderLongTestCase { str_: b"0x0000200000000000\0".as_ptr() as *const _, count: 0x0000200000000000, expected: 45 },
    OrderLongTestCase { str_: b"0x5000000000000000\0".as_ptr() as *const _, count: 0x5000000000000000, expected: 63 },
    OrderLongTestCase { str_: b"0x8000000000000000\0".as_ptr() as *const _, count: 0x8000000000000000, expected: 63 },
    OrderLongTestCase { str_: b"0x8000300000000000\0".as_ptr() as *const _, count: 0x8000300000000000, expected: 64 },
];

#[cfg(target_pointer_width = "64")]
unsafe fn test_get_count_order_long(test: *mut Kunit) {
    let params = (*test).param_value as *const OrderLongTestCase;
    KUNIT_EXPECT_EQ(test, get_count_order_long((*params).count as u32), (*params).expected);
}

// KUNIT_CASE_PARAM and kunit_test_suite registrations are preserved as external integration points.
// MODULE_AUTHOR, MODULE_LICENSE, and MODULE_DESCRIPTION metadata are likewise supplied externally.

#[repr(C)]
struct Kunit { param_value: *mut core::ffi::c_void }

extern "C" {
    fn bitmap_zero(bitmap: *mut usize, nbits: usize);
    fn set_bit(nr: core::ffi::c_long, addr: *mut usize);
    fn clear_bit(nr: core::ffi::c_long, addr: *mut usize);
    fn change_bit(nr: core::ffi::c_long, addr: *mut usize);
    fn test_bit(nr: core::ffi::c_long, addr: *const usize) -> bool;
    fn test_and_set_bit(nr: core::ffi::c_long, addr: *mut usize) -> bool;
    fn test_and_clear_bit(nr: core::ffi::c_long, addr: *mut usize) -> bool;
    fn test_and_change_bit(nr: core::ffi::c_long, addr: *mut usize) -> bool;
    fn find_first_bit(addr: *const usize, size: usize) -> usize;
    fn get_count_order(count: u32) -> i32;
    fn get_count_order_long(count: u32) -> i32;
}

// KUNIT_EXPECT_TRUE/FALSE/EQ are KUnit assertion macros from the external dependency.
extern "C" {
    fn KUNIT_EXPECT_TRUE(test: *mut Kunit, value: bool);
    fn KUNIT_EXPECT_FALSE(test: *mut Kunit, value: bool);
    fn KUNIT_EXPECT_EQ(test: *mut Kunit, lhs: i32, rhs: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

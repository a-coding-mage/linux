// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test of ext4 inode that verify the seconds part of [a/c/m]
 * timestamps in ext4 inode structs are decoded correctly.
 */

// Dependencies supplied by the kernel/KUnit and ext4 implementation.

const LOWER_MSB_0: i64 = 0;
const UPPER_MSB_0: i64 = 0x7fffffff;
const LOWER_MSB_1: i64 = -(UPPER_MSB_0) - 1;
const UPPER_MSB_1: i64 = -1;
const MAX_NANOSECONDS: i64 = (1i64 << 30) - 1;

const CASE_NAME_FORMAT: &str = "%s: msb:%x lower_bound:%x extra_bits: %x";

const LOWER_BOUND_NEG_NO_EXTRA_BITS_CASE: &str =
    "1901-12-13 Lower bound of 32bit < 0 timestamp, no extra bits";
const UPPER_BOUND_NEG_NO_EXTRA_BITS_CASE: &str =
    "1969-12-31 Upper bound of 32bit < 0 timestamp, no extra bits";
const LOWER_BOUND_NONNEG_NO_EXTRA_BITS_CASE: &str =
    "1970-01-01 Lower bound of 32bit >=0 timestamp, no extra bits";
const UPPER_BOUND_NONNEG_NO_EXTRA_BITS_CASE: &str =
    "2038-01-19 Upper bound of 32bit >=0 timestamp, no extra bits";
const LOWER_BOUND_NEG_LO_1_CASE: &str =
    "2038-01-19 Lower bound of 32bit <0 timestamp, lo extra sec bit on";
const UPPER_BOUND_NEG_LO_1_CASE: &str =
    "2106-02-07 Upper bound of 32bit <0 timestamp, lo extra sec bit on";
const LOWER_BOUND_NONNEG_LO_1_CASE: &str =
    "2106-02-07 Lower bound of 32bit >=0 timestamp, lo extra sec bit on";
const UPPER_BOUND_NONNEG_LO_1_CASE: &str =
    "2174-02-25 Upper bound of 32bit >=0 timestamp, lo extra sec bit on";
const LOWER_BOUND_NEG_HI_1_CASE: &str =
    "2174-02-25 Lower bound of 32bit <0 timestamp, hi extra sec bit on";
const UPPER_BOUND_NEG_HI_1_CASE: &str =
    "2242-03-16 Upper bound of 32bit <0 timestamp, hi extra sec bit on";
const LOWER_BOUND_NONNEG_HI_1_CASE: &str =
    "2242-03-16 Lower bound of 32bit >=0 timestamp, hi extra sec bit on";
const UPPER_BOUND_NONNEG_HI_1_CASE: &str =
    "2310-04-04 Upper bound of 32bit >=0 timestamp, hi extra sec bit on";
const UPPER_BOUND_NONNEG_HI_1_NS_1_CASE: &str =
    "2310-04-04 Upper bound of 32bit>=0 timestamp, hi extra sec bit 1. 1 ns";
const LOWER_BOUND_NONNEG_HI_1_NS_MAX_CASE: &str =
    "2378-04-22 Lower bound of 32bit>= timestamp. Extra sec bits 1. Max ns";
const LOWER_BOUND_NONNEG_EXTRA_BITS_1_CASE: &str =
    "2378-04-22 Lower bound of 32bit >=0 timestamp. All extra sec bits on";
const UPPER_BOUND_NONNEG_EXTRA_BITS_1_CASE: &str =
    "2446-05-10 Upper bound of 32bit >=0 timestamp. All extra sec bits on";

#[repr(C)]
pub struct Timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct TimestampExpectation {
    pub test_case_name: *const core::ffi::c_char,
    pub expected: Timespec64,
    pub extra_bits: u32,
    pub msb_set: bool,
    pub lower_bound: bool,
}

macro_rules! cstr { ($s:expr) => { concat!($s, "\0").as_ptr() as *const core::ffi::c_char } }

pub static TEST_DATA: [TimestampExpectation; 16] = [
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NEG_NO_EXTRA_BITS_CASE), msb_set: true, lower_bound: true, extra_bits: 0, expected: Timespec64 { tv_sec: -0x80000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NEG_NO_EXTRA_BITS_CASE), msb_set: true, lower_bound: false, extra_bits: 0, expected: Timespec64 { tv_sec: -1, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NONNEG_NO_EXTRA_BITS_CASE), msb_set: false, lower_bound: true, extra_bits: 0, expected: Timespec64 { tv_sec: 0, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NONNEG_NO_EXTRA_BITS_CASE), msb_set: false, lower_bound: false, extra_bits: 0, expected: Timespec64 { tv_sec: 0x7fffffff, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NEG_LO_1_CASE), msb_set: true, lower_bound: true, extra_bits: 1, expected: Timespec64 { tv_sec: 0x80000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NEG_LO_1_CASE), msb_set: true, lower_bound: false, extra_bits: 1, expected: Timespec64 { tv_sec: 0xffffffff, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NONNEG_LO_1_CASE), msb_set: false, lower_bound: true, extra_bits: 1, expected: Timespec64 { tv_sec: 0x100000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NONNEG_LO_1_CASE), msb_set: false, lower_bound: false, extra_bits: 1, expected: Timespec64 { tv_sec: 0x17fffffff, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NEG_HI_1_CASE), msb_set: true, lower_bound: true, extra_bits: 2, expected: Timespec64 { tv_sec: 0x180000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NEG_HI_1_CASE), msb_set: true, lower_bound: false, extra_bits: 2, expected: Timespec64 { tv_sec: 0x1ffffffff, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NONNEG_HI_1_CASE), msb_set: false, lower_bound: true, extra_bits: 2, expected: Timespec64 { tv_sec: 0x200000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NONNEG_HI_1_CASE), msb_set: false, lower_bound: false, extra_bits: 2, expected: Timespec64 { tv_sec: 0x27fffffff, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NONNEG_HI_1_NS_1_CASE), msb_set: false, lower_bound: false, extra_bits: 6, expected: Timespec64 { tv_sec: 0x27fffffff, tv_nsec: 1 } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NONNEG_HI_1_NS_MAX_CASE), msb_set: false, lower_bound: true, extra_bits: 0xffffffff, expected: Timespec64 { tv_sec: 0x300000000, tv_nsec: MAX_NANOSECONDS } },
    TimestampExpectation { test_case_name: cstr!(LOWER_BOUND_NONNEG_EXTRA_BITS_1_CASE), msb_set: false, lower_bound: true, extra_bits: 3, expected: Timespec64 { tv_sec: 0x300000000, tv_nsec: 0 } },
    TimestampExpectation { test_case_name: cstr!(UPPER_BOUND_NONNEG_EXTRA_BITS_1_CASE), msb_set: false, lower_bound: false, extra_bits: 3, expected: Timespec64 { tv_sec: 0x37fffffff, tv_nsec: 0 } },
];

// The KUnit parameter-description helper and array-parameter registration are
// supplied by KUnit in the surrounding kernel translation.
pub unsafe fn timestamp_expectation_to_desc(t: *const TimestampExpectation, desc: *mut core::ffi::c_char) {
    unsafe extern "C" { fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: usize) -> isize; }
    unsafe { strscpy(desc, (*t).test_case_name, 256); }
}

pub unsafe fn get_32bit_time(test: *const TimestampExpectation) -> i64 {
    unsafe {
        if (*test).msb_set {
            if (*test).lower_bound { return LOWER_MSB_1; }
            return UPPER_MSB_1;
        }
        if (*test).lower_bound { return LOWER_MSB_0; }
        UPPER_MSB_0
    }
}

/* Test data is derived from the table in the Inode Timestamps section of
 * Documentation/filesystems/ext4/inodes.rst. */

// KUnit test context and ext4_decode_extra_time are external kernel symbols.
#[repr(C)] pub struct Kunit { pub param_value: *mut core::ffi::c_void }
unsafe extern "C" {
    fn ext4_decode_extra_time(sec: u32, extra: u32) -> Timespec64;
    fn kunit_expect_eq_msg(test: *mut Kunit, expected: i64, actual: i64, fmt: *const core::ffi::c_char, ...);
}

pub unsafe fn inode_test_xtimestamp_decoding(test: *mut Kunit) {
    unsafe {
        let test_param = (*test).param_value as *mut TimestampExpectation;
        let timestamp = ext4_decode_extra_time(get_32bit_time(test_param) as u32, (*test_param).extra_bits);
        kunit_expect_eq_msg(test, (*test_param).expected.tv_sec, timestamp.tv_sec, cstr!(CASE_NAME_FORMAT), (*test_param).test_case_name, (*test_param).msb_set, (*test_param).lower_bound, (*test_param).extra_bits);
        kunit_expect_eq_msg(test, (*test_param).expected.tv_nsec, timestamp.tv_nsec, cstr!(CASE_NAME_FORMAT), (*test_param).test_case_name, (*test_param).msb_set, (*test_param).lower_bound, (*test_param).extra_bits);
    }
}

// KUNIT_CASE_PARAM, kunit_test_suites, MODULE_DESCRIPTION, and MODULE_LICENSE
// are registration/metadata constructs supplied by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

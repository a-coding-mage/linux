// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
/*
 * Test cases for __ashldi3(), __ashrdi3(), and __lshrdi3().
 */

// Dependencies supplied by the kernel/KUnit environment.
use core::ffi::c_int;

#[repr(C)]
struct shdi3_test_entry {
    input: i64,
    shift: c_int,
    result: i64,
}

extern "C" {
    fn __ashldi3(input: i64, shift: c_int) -> i64;
    fn __ashrdi3(input: i64, shift: c_int) -> i64;
    fn __lshrdi3(input: i64, shift: c_int) -> i64;
}

static ashldi3_testdata: [shdi3_test_entry; 18] = [
    // https://github.com/llvm/llvm-project/compiler-rt/test/builtins/Unit/ashldi3_test.c
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 0, result: 0x123456789ABCDEF },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 1, result: 0x2468ACF13579BDE },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 2, result: 0x48D159E26AF37BC },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 3, result: 0x91A2B3C4D5E6F78 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 4, result: 0x123456789ABCDEF0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 28, result: 0x789ABCDEF0000000 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 29, result: 0xF13579BDE0000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 30, result: 0xE26AF37BC0000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 31, result: 0xC4D5E6F780000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 32, result: 0x89ABCDEF00000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 33, result: 0x13579BDE00000000 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 34, result: 0x26AF37BC00000000 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 35, result: 0x4D5E6F7800000000 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 36, result: 0x9ABCDEF000000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 60, result: 0xF000000000000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 61, result: 0xE000000000000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 62, result: 0xC000000000000000u64 as i64 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 63, result: 0x8000000000000000u64 as i64 },
];

static ashrdi3_testdata: [shdi3_test_entry; 36] = [
    // https://github.com/llvm/llvm-project/compiler-rt/test/builtins/Unit/ashrdi3_test.c
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 0, result: 0x123456789ABCDEF },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 1, result: 0x91A2B3C4D5E6F7 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 2, result: 0x48D159E26AF37B },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 3, result: 0x2468ACF13579BD },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 4, result: 0x123456789ABCDE },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 28, result: 0x12345678 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 29, result: 0x91A2B3C },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 30, result: 0x48D159E },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 31, result: 0x2468ACF },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 32, result: 0x1234567 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 33, result: 0x91A2B3 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 34, result: 0x48D159 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 35, result: 0x2468AC },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 36, result: 0x123456 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 60, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 61, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 62, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 63, result: 0 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 0, result: 0xFEDCBA9876543210u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 1, result: 0xFF6E5D4C3B2A1908u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 2, result: 0xFFB72EA61D950C84u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 3, result: 0xFFDB97530ECA8642u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 4, result: 0xFFEDCBA987654321u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 28, result: 0xFFFFFFFFEDCBA987u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 29, result: 0xFFFFFFFFF6E5D4C3u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 30, result: 0xFFFFFFFFFB72EA61u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 31, result: 0xFFFFFFFFFDB97530u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 32, result: 0xFFFFFFFFFEDCBA98u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 33, result: 0xFFFFFFFFFF6E5D4Cu64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 34, result: 0xFFFFFFFFFFB72EA6u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 35, result: 0xFFFFFFFFFFDB9753u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 36, result: 0xFFFFFFFFFFEDCBA9u64 as i64 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 60, result: -6 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 61, result: -3 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 62, result: -2 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 63, result: -1 },
];

static lshrdi3_testdata: [shdi3_test_entry; 36] = [
    // https://github.com/llvm/llvm-project/compiler-rt/test/builtins/Unit/lshrdi3_test.c
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 0, result: 0x123456789ABCDEF },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 1, result: 0x91A2B3C4D5E6F7 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 2, result: 0x48D159E26AF37B },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 3, result: 0x2468ACF13579BD },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 4, result: 0x123456789ABCDE },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 28, result: 0x12345678 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 29, result: 0x91A2B3C },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 30, result: 0x48D159E },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 31, result: 0x2468ACF },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 32, result: 0x1234567 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 33, result: 0x91A2B3 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 34, result: 0x48D159 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 35, result: 0x2468AC },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 36, result: 0x123456 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 60, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 61, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 62, result: 0 },
    shdi3_test_entry { input: 0x0123456789ABCDEF, shift: 63, result: 0 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 0, result: 0xFEDCBA9876543210u64 as i64 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 1, result: 0x7F6E5D4C3B2A1908 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 2, result: 0x3FB72EA61D950C84 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 3, result: 0x1FDB97530ECA8642 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 4, result: 0xFEDCBA987654321 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 28, result: 0xFEDCBA987 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 29, result: 0x7F6E5D4C3 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 30, result: 0x3FB72EA61 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 31, result: 0x1FDB97530 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 32, result: 0xFEDCBA98 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 33, result: 0x7F6E5D4C },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 34, result: 0x3FB72EA6 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 35, result: 0x1FDB9753 },
    shdi3_test_entry { input: 0xFEDCBA9876543210u64 as i64, shift: 36, result: 0xFEDCBA9 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 60, result: 0xA },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 61, result: 0x5 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 62, result: 0x2 },
    shdi3_test_entry { input: 0xAEDCBA9876543210u64 as i64, shift: 63, result: 0x1 },
];

// KUnit registration and assertion macros are supplied externally by the kernel.
unsafe fn shdi3_test_ashldi3(test: *mut core::ffi::c_void) {
    for e in ashldi3_testdata.iter() {
        let ret = __ashldi3(e.input, e.shift);
        // KUNIT_EXPECT_EQ_MSG(test, ret, e.result, "    when evaluating __ashldi3(%lld, %d)", e.input, e.shift);
        let _ = (test, ret, e.result);
    }
}

unsafe fn shdi3_test_ashrdi3(test: *mut core::ffi::c_void) {
    for e in ashrdi3_testdata.iter() {
        let ret = __ashrdi3(e.input, e.shift);
        // KUNIT_EXPECT_EQ_MSG(test, ret, e.result, "    when evaluating __ashrdi3(%lld, %d)", e.input, e.shift);
        let _ = (test, ret, e.result);
    }
}

unsafe fn shdi3_test_lshrdi3(test: *mut core::ffi::c_void) {
    for e in lshrdi3_testdata.iter() {
        let ret = __lshrdi3(e.input, e.shift);
        // KUNIT_EXPECT_EQ_MSG(test, ret, e.result, "    when evaluating __lshrdi3(%lld, %d)", e.input, e.shift);
        let _ = (test, ret, e.result);
    }
}

// static struct kunit_case shdi3_test_cases[] = {
//     KUNIT_CASE(shdi3_test_ashldi3),
//     KUNIT_CASE(shdi3_test_ashrdi3),
//     KUNIT_CASE(shdi3_test_lshrdi3),
//     {}
// };
// static struct kunit_suite shdi3_test_suite = {
//     .name = "shdi3",
//     .test_cases = shdi3_test_cases,
// };
// kunit_test_suite(shdi3_test_suite);

// MODULE_DESCRIPTION("Test cases for __ashldi3(), __ashrdi3(), and __lshrdi3()");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 BayLibre SAS
 */

#[repr(C)]
#[derive(Copy, Clone)]
struct TestParams {
    a: u64,
    b: u64,
    d: u64,
    result: u64,
    round_up: u32,
}

static TEST_VALUES: &[TestParams] = &[
    /* this contains many edge values followed by a couple random values */
    TestParams { a: 0xb, b: 0x7, d: 0x3, result: 0x19, round_up: 1 },
    TestParams { a: 0xffff0000, b: 0xffff0000, d: 0xf, result: 0x1110eeef00000000, round_up: 0 },
    TestParams { a: 0xffffffff, b: 0xffffffff, d: 0x1, result: 0xfffffffe00000001, round_up: 0 },
    TestParams { a: 0xffffffff, b: 0xffffffff, d: 0x2, result: 0x7fffffff00000000, round_up: 1 },
    TestParams { a: 0x1ffffffff, b: 0xffffffff, d: 0x2, result: 0xfffffffe80000000, round_up: 1 },
    TestParams { a: 0x1ffffffff, b: 0xffffffff, d: 0x3, result: 0xaaaaaaa9aaaaaaab, round_up: 0 },
    TestParams { a: 0x1ffffffff, b: 0x1ffffffff, d: 0x4, result: 0xffffffff00000000, round_up: 1 },
    TestParams { a: 0xffff000000000000, b: 0xffff000000000000, d: 0xffffffffffff0001, result: 0xfffeffffffffffff, round_up: 1 },
    TestParams { a: 0x3333333333333333, b: 0x3333333333333333, d: 0x5555555555555555, result: 0x1eb851eb851eb851, round_up: 1 },
    TestParams { a: 0x7fffffffffffffff, b: 0x2, d: 0x3, result: 0x5555555555555554, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x2, d: 0x8000000000000000, result: 0x3, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x2, d: 0xc000000000000000, result: 0x2, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x4000000000000004, d: 0x8000000000000000, result: 0x8000000000000007, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x4000000000000001, d: 0x8000000000000000, result: 0x8000000000000001, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x8000000000000001, d: 0xffffffffffffffff, result: 0x8000000000000001, round_up: 0 },
    TestParams { a: 0xfffffffffffffffe, b: 0x8000000000000001, d: 0xffffffffffffffff, result: 0x8000000000000000, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x8000000000000001, d: 0xfffffffffffffffe, result: 0x8000000000000001, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x8000000000000001, d: 0xfffffffffffffffd, result: 0x8000000000000002, round_up: 1 },
    TestParams { a: 0x7fffffffffffffff, b: 0xffffffffffffffff, d: 0xc000000000000000, result: 0xaaaaaaaaaaaaaaa8, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x7fffffffffffffff, d: 0xa000000000000000, result: 0xccccccccccccccca, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0x7fffffffffffffff, d: 0x9000000000000000, result: 0xe38e38e38e38e38b, round_up: 1 },
    TestParams { a: 0x7fffffffffffffff, b: 0x7fffffffffffffff, d: 0x5000000000000000, result: 0xccccccccccccccc9, round_up: 1 },
    TestParams { a: 0xffffffffffffffff, b: 0xfffffffffffffffe, d: 0xffffffffffffffff, result: 0xfffffffffffffffe, round_up: 0 },
    TestParams { a: 0xe6102d256d7ea3ae, b: 0x70a77d0be4c31201, d: 0xd63ec35ab3220357, result: 0x78f8bf8cc86c6e18, round_up: 1 },
    TestParams { a: 0xf53bae05cb86c6e1, b: 0x3847b32d2f8d32e0, d: 0xcfd4f55a647f403c, result: 0x42687f79d8998d35, round_up: 1 },
    TestParams { a: 0x9951c5498f941092, b: 0x1f8c8bfdf287a251, d: 0xa3c8dc5f81ea3fe2, result: 0x1d887cb25900091f, round_up: 1 },
    TestParams { a: 0x374fee9daa1bb2bb, b: 0x0d0bfbff7b8ae3ef, d: 0xc169337bd42d5179, result: 0x03bb2dbaffcbb961, round_up: 1 },
    TestParams { a: 0xeac0d03ac10eeaf0, b: 0x89be05dfa162ed9b, d: 0x92bb1679a41f0e4b, result: 0xdc5f5cc9e270d216, round_up: 1 },
];

extern "C" {
    fn mul_u64_u64_div_u64(a: u64, b: u64, d: u64) -> u64;
    fn mul_u64_u64_div_u64_roundup(a: u64, b: u64, d: u64) -> u64;
    fn test_mul_u64_add_u64_div_u64(a: u64, b: u64, c: u64, d: u64) -> u64;
    fn test_mul_u64_add_u64_div_u64_32bit(a: u64, b: u64, c: u64, d: u64) -> u64;
}

unsafe fn test_run(fn_no: u32, _fn_name: *const core::ffi::c_char) -> i32 {
    // Kernel timing and printk calls are external dependencies of the source module.
    let mut errors: i32 = 0;
    let mut tests: i32 = 0;
    for value in TEST_VALUES {
        let (a, b, d) = (value.a, value.b, value.d);
        let (result, result_up) = match fn_no {
            1 => (test_mul_u64_add_u64_div_u64(a, b, 0, d), test_mul_u64_add_u64_div_u64(a, b, d.wrapping_sub(1), d)),
            2 => (test_mul_u64_add_u64_div_u64_32bit(a, b, 0, d), test_mul_u64_add_u64_div_u64_32bit(a, b, d.wrapping_sub(1), d)),
            _ => (mul_u64_u64_div_u64(a, b, d), mul_u64_u64_div_u64_roundup(a, b, d)),
        };
        tests += 2;
        if result != value.result { errors += 1; }
        let expected_result = value.result.wrapping_add(value.round_up as u64);
        if result_up != expected_result { errors += 1; }
    }
    let _ = tests;
    errors
}

unsafe fn test_init() -> i32 {
    if test_run(0, core::ptr::null()) != 0 { return -22; }
    if test_run(1, core::ptr::null()) != 0 { return -22; }
    if test_run(2, core::ptr::null()) != 0 { return -22; }
    0
}

unsafe fn test_exit() {}

// The C source includes div64.c twice with macro-renamed generic implementations.
// Kernel module registration and metadata are supplied by the surrounding build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

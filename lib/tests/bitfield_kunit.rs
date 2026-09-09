// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for bitfield helpers.
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies: kunit/test.h and linux/bitfield.h

macro_rules! check_enc_get_u {
    ($tp:ty, $v:expr, $field:expr, $res:expr) => {{
        let res: $tp = encode_bits($v, $field);
        kunit_assert_false_msg!(context, res != $res,
            "encode_bits({}, {}) is 0x{:x} != {}\n", stringify!($v), stringify!($field), res as u64, $res);
        kunit_assert_false!(context, get_bits(res, $field) != $v);
    }};
}

macro_rules! check_enc_get_le {
    ($tp:ty, $v:expr, $field:expr, $res:expr) => {{
        let res: Le<$tp> = encode_bits($v, $field);
        kunit_assert_false_msg!(context, res != cpu_to_le(res),
            "le encode_bits({}, {}) is 0x{:x} != 0x{:x}", stringify!($v), stringify!($field), le_to_cpu(res) as u64, $res as u64);
        kunit_assert_false!(context, get_bits(le_to_cpu(res), $field) != $v);
    }};
}

macro_rules! check_enc_get_be {
    ($tp:ty, $v:expr, $field:expr, $res:expr) => {{
        let res: Be<$tp> = encode_bits($v, $field);
        kunit_assert_false_msg!(context, res != cpu_to_be(res),
            "be encode_bits({}, {}) is 0x{:x} != 0x{:x}", stringify!($v), stringify!($field), be_to_cpu(res) as u64, $res as u64);
        kunit_assert_false!(context, get_bits(be_to_cpu(res), $field) != $v);
    }};
}

macro_rules! check_enc_get {
    ($tp:ty, $v:expr, $field:expr, $res:expr) => {{
        check_enc_get_u!($tp, $v, $field, $res);
        check_enc_get_le!($tp, $v, $field, $res);
        check_enc_get_be!($tp, $v, $field, $res);
    }};
}

unsafe fn test_bitfields_constants(context: *mut kunit) {
    /*
     * NOTE
     * This whole function compiles (or at least should, if everything
     * is going according to plan) to nothing after optimisation.
     */

    check_enc_get!(u16, 1, 0x000f, 0x0001);
    check_enc_get!(u16, 3, 0x00f0, 0x0030);
    check_enc_get!(u16, 5, 0x0f00, 0x0500);
    check_enc_get!(u16, 7, 0xf000, 0x7000);
    check_enc_get!(u16, 14, 0x000f, 0x000e);
    check_enc_get!(u16, 15, 0x00f0, 0x00f0);

    check_enc_get_u!(u8, 1, 0x0f, 0x01);
    check_enc_get_u!(u8, 3, 0xf0, 0x30);
    check_enc_get_u!(u8, 14, 0x0f, 0x0e);
    check_enc_get_u!(u8, 15, 0xf0, 0xf0);

    check_enc_get!(u32, 1, 0x00000f00, 0x00000100);
    check_enc_get!(u32, 3, 0x0000f000, 0x00003000);
    check_enc_get!(u32, 5, 0x000f0000, 0x00050000);
    check_enc_get!(u32, 7, 0x00f00000, 0x00700000);
    check_enc_get!(u32, 14, 0x0f000000, 0x0e000000);
    check_enc_get!(u32, 15, 0xf0000000, 0xf0000000);

    check_enc_get!(u64, 1, 0x00000f0000000000u64, 0x0000010000000000u64);
    check_enc_get!(u64, 3, 0x0000f00000000000u64, 0x0000300000000000u64);
    check_enc_get!(u64, 5, 0x000f000000000000u64, 0x0005000000000000u64);
    check_enc_get!(u64, 7, 0x00f0000000000000u64, 0x0070000000000000u64);
    check_enc_get!(u64, 14, 0x0f00000000000000u64, 0x0e00000000000000u64);
    check_enc_get!(u64, 15, 0xf000000000000000u64, 0xf000000000000000u64);
}

macro_rules! check {
    ($tp:ty, $mask:expr) => {{
        let mut v: u64 = 0;
        while v < (1u64 << hweight32($mask)) {
            kunit_assert_false!(context, encode_bits::<$tp>(v, $mask) != v << ffs64($mask));
            v = v.wrapping_add(1);
        }
    }};
}

unsafe fn test_bitfields_variables(context: *mut kunit) {
    check!(u8, 0x0f); check!(u8, 0xf0); check!(u8, 0x38);
    check!(u16, 0x0038); check!(u16, 0x0380); check!(u16, 0x3800); check!(u16, 0x8000);
    check!(u32, 0x80000000); check!(u32, 0x7f000000); check!(u32, 0x07e00000); check!(u32, 0x00018000);
    check!(u64, 0x8000000000000000u64); check!(u64, 0x7f00000000000000u64);
    check!(u64, 0x0001800000000000u64); check!(u64, 0x0000000080000000u64);
    check!(u64, 0x000000007f000000u64); check!(u64, 0x0000000018000000u64);
    check!(u64, 0x0000001f8000000u64);
}

#[cfg(test_bitfield_compile)]
unsafe fn test_bitfields_compile(context: *mut kunit) {
    /* these should fail compilation */
    check_enc_get!(u16, 16, 0x0f00, 0x1000);
    encode_bits::<u32>(7, 0x06000000);

    /* this should at least give a warning */
    encode_bits::<u16>(0, 0x60000);
}

static mut BITFIELDS_TEST_CASES: [kunit_case; 3] = [
    kunit_case { run_case: Some(test_bitfields_constants) },
    kunit_case { run_case: Some(test_bitfields_variables) },
    kunit_case { run_case: None },
];

static mut BITFIELDS_TEST_SUITE: kunit_suite = kunit_suite {
    name: "bitfields",
    test_cases: BITFIELDS_TEST_CASES.as_ptr(),
};

// kunit_test_suites!(&mut BITFIELDS_TEST_SUITE);

// MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
// MODULE_DESCRIPTION("Test cases for bitfield helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

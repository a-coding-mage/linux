// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Test cases for lib/uuid.c module.
//
// C includes <kunit/test.h> and <linux/uuid.h> supply the KUnit framework,
// UUID/GUID types, constants, and helper functions used below.

#[repr(C)]
struct TestUuidData {
    uuid: *const core::ffi::c_char,
    le: guid_t,
    be: uuid_t,
}

// External types, functions, constants, and KUnit macros are supplied by the
// corresponding kernel dependencies.
type guid_t = uuid_t;

#[repr(C)]
struct uuid_t {
    b: [u8; 16],
}

unsafe extern "C" {
    fn guid_parse(uuid: *const core::ffi::c_char, le: *mut guid_t) -> i32;
    fn uuid_parse(uuid: *const core::ffi::c_char, be: *mut uuid_t) -> i32;
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool;
    fn uuid_equal(a: *const uuid_t, b: *const uuid_t) -> bool;
    fn uuid_gen(u: *mut uuid_t);
    fn guid_gen(g: *mut guid_t);
    fn generate_random_uuid(buf: *mut u8);
    fn generate_random_guid(buf: *mut u8);
}

#[allow(non_camel_case_types)]
type kunit = core::ffi::c_void;

static TEST_UUID_TEST_DATA: &[TestUuidData] = &[
    TestUuidData {
        uuid: c"c33f4995-3701-450e-9fbf-206a2e98e576".as_ptr(),
        le: GUID_INIT(0xc33f4995, 0x3701, 0x450e, 0x9f, 0xbf, 0x20, 0x6a, 0x2e, 0x98, 0xe5, 0x76),
        be: UUID_INIT(0xc33f4995, 0x3701, 0x450e, 0x9f, 0xbf, 0x20, 0x6a, 0x2e, 0x98, 0xe5, 0x76),
    },
    TestUuidData {
        uuid: c"64b4371c-77c1-48f9-8221-29f054fc023b".as_ptr(),
        le: GUID_INIT(0x64b4371c, 0x77c1, 0x48f9, 0x82, 0x21, 0x29, 0xf0, 0x54, 0xfc, 0x02, 0x3b),
        be: UUID_INIT(0x64b4371c, 0x77c1, 0x48f9, 0x82, 0x21, 0x29, 0xf0, 0x54, 0xfc, 0x02, 0x3b),
    },
    TestUuidData {
        uuid: c"0cb4ddff-a545-4401-9d06-688af53e7f84".as_ptr(),
        le: GUID_INIT(0x0cb4ddff, 0xa545, 0x4401, 0x9d, 0x06, 0x68, 0x8a, 0xf5, 0x3e, 0x7f, 0x84),
        be: UUID_INIT(0x0cb4ddff, 0xa545, 0x4401, 0x9d, 0x06, 0x68, 0x8a, 0xf5, 0x3e, 0x7f, 0x84),
    },
];

static TEST_UUID_WRONG_DATA: &[*const core::ffi::c_char] = &[
    c"c33f4995-3701-450e-9fbf206a2e98e576 ".as_ptr(), // no hyphen(s)
    c"64b4371c-77c1-48f9-8221-29f054XX023b".as_ptr(), // invalid character(s)
    c"0cb4ddff-a545-4401-9d06-688af53e".as_ptr(), // not enough data
];

unsafe fn uuid_test_guid_valid(test: *mut kunit) {
    let mut le = core::mem::MaybeUninit::<guid_t>::uninit();
    for data in TEST_UUID_TEST_DATA {
        KUNIT_EXPECT_EQ!(test, guid_parse(data.uuid, le.as_mut_ptr()), 0);
        KUNIT_EXPECT_TRUE!(test, guid_equal(&data.le, le.as_ptr()));
    }
}

unsafe fn uuid_test_uuid_valid(test: *mut kunit) {
    let mut be = core::mem::MaybeUninit::<uuid_t>::uninit();
    for data in TEST_UUID_TEST_DATA {
        KUNIT_EXPECT_EQ!(test, uuid_parse(data.uuid, be.as_mut_ptr()), 0);
        KUNIT_EXPECT_TRUE!(test, uuid_equal(&data.be, be.as_ptr()));
    }
}

unsafe fn uuid_test_guid_invalid(test: *mut kunit) {
    let mut le = core::mem::MaybeUninit::<guid_t>::uninit();
    for &uuid in TEST_UUID_WRONG_DATA {
        KUNIT_EXPECT_EQ!(test, guid_parse(uuid, le.as_mut_ptr()), -EINVAL);
    }
}

unsafe fn uuid_test_uuid_invalid(test: *mut kunit) {
    let mut be = core::mem::MaybeUninit::<uuid_t>::uninit();
    for &uuid in TEST_UUID_WRONG_DATA {
        KUNIT_EXPECT_EQ!(test, uuid_parse(uuid, be.as_mut_ptr()), -EINVAL);
    }
}

// RFC 4122 section 4.4 says random UUIDs/GUIDs (version 4) must have:
//   - version 4 in the high nibble of the version byte,
//   - variant DCE 1.1 (binary 10x) in the high bits of byte 8.
//
// The version byte is byte 6 in the "wire" uuid_t layout and byte 7 in
// the byte-swapped guid_t layout.
unsafe fn uuid_test_uuid_gen(test: *mut kunit) {
    let mut u = core::mem::MaybeUninit::<uuid_t>::uninit();
    for _ in 0..8 {
        uuid_gen(u.as_mut_ptr());
        KUNIT_EXPECT_EQ!(test, (*u.as_ptr()).b[6] & 0xf0, 0x40);
        KUNIT_EXPECT_EQ!(test, (*u.as_ptr()).b[8] & 0xc0, 0x80);
    }
}

unsafe fn uuid_test_guid_gen(test: *mut kunit) {
    let mut g = core::mem::MaybeUninit::<guid_t>::uninit();
    for _ in 0..8 {
        guid_gen(g.as_mut_ptr());
        KUNIT_EXPECT_EQ!(test, (*g.as_ptr()).b[7] & 0xf0, 0x40);
        KUNIT_EXPECT_EQ!(test, (*g.as_ptr()).b[8] & 0xc0, 0x80);
    }
}

unsafe fn uuid_test_generate_random_uuid(test: *mut kunit) {
    let mut buf = [0u8; 16];
    for _ in 0..8 {
        generate_random_uuid(buf.as_mut_ptr());
        KUNIT_EXPECT_EQ!(test, buf[6] & 0xf0, 0x40);
        KUNIT_EXPECT_EQ!(test, buf[8] & 0xc0, 0x80);
    }
}

unsafe fn uuid_test_generate_random_guid(test: *mut kunit) {
    let mut buf = [0u8; 16];
    for _ in 0..8 {
        generate_random_guid(buf.as_mut_ptr());
        KUNIT_EXPECT_EQ!(test, buf[7] & 0xf0, 0x40);
        KUNIT_EXPECT_EQ!(test, buf[8] & 0xc0, 0x80);
    }
}

// KUNIT_CASE entries and the kunit suite registration correspond to the C
// KUnit declarations; their framework-provided Rust equivalents are retained
// as the following source-level declarations.
static UUID_TEST_CASES: &[unsafe fn(*mut kunit)] = &[
    uuid_test_guid_valid,
    uuid_test_uuid_valid,
    uuid_test_guid_invalid,
    uuid_test_uuid_invalid,
    uuid_test_uuid_gen,
    uuid_test_guid_gen,
    uuid_test_generate_random_uuid,
    uuid_test_generate_random_guid,
];

// .name = "uuid", .test_cases = uuid_test_cases
static UUID_TEST_SUITE_NAME: &str = "uuid";

// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
// MODULE_DESCRIPTION("Test cases for lib/uuid.c module");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

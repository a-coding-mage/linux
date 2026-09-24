// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//! Original UUID/GUID KUnit cases using the native kernel ABI.

use core::{mem::MaybeUninit, ptr};
use kernel::bindings;

// The suite uses only the original header's initializer and comparison API.
#[allow(dead_code, unreachable_pub)]
#[path = "../../include/linux/uuid_header.rs"]
mod uuid;

struct TestUuidData {
    text: &'static core::ffi::CStr,
    le: bindings::guid_t,
    be: bindings::uuid_t,
}

static TEST_UUID: [TestUuidData; 3] = [
    TestUuidData {
        text: c"c33f4995-3701-450e-9fbf-206a2e98e576",
        le: GUID_INIT!(
            0xc33f4995_u32,
            0x3701,
            0x450e,
            0x9f,
            0xbf,
            0x20,
            0x6a,
            0x2e,
            0x98,
            0xe5,
            0x76
        ),
        be: UUID_INIT!(
            0xc33f4995_u32,
            0x3701,
            0x450e,
            0x9f,
            0xbf,
            0x20,
            0x6a,
            0x2e,
            0x98,
            0xe5,
            0x76
        ),
    },
    TestUuidData {
        text: c"64b4371c-77c1-48f9-8221-29f054fc023b",
        le: GUID_INIT!(0x64b4371c, 0x77c1, 0x48f9, 0x82, 0x21, 0x29, 0xf0, 0x54, 0xfc, 0x02, 0x3b),
        be: UUID_INIT!(0x64b4371c, 0x77c1, 0x48f9, 0x82, 0x21, 0x29, 0xf0, 0x54, 0xfc, 0x02, 0x3b),
    },
    TestUuidData {
        text: c"0cb4ddff-a545-4401-9d06-688af53e7f84",
        le: GUID_INIT!(0x0cb4ddff, 0xa545, 0x4401, 0x9d, 0x06, 0x68, 0x8a, 0xf5, 0x3e, 0x7f, 0x84),
        be: UUID_INIT!(0x0cb4ddff, 0xa545, 0x4401, 0x9d, 0x06, 0x68, 0x8a, 0xf5, 0x3e, 0x7f, 0x84),
    },
];

static TEST_UUID_WRONG: [&core::ffi::CStr; 3] = [
    c"c33f4995-3701-450e-9fbf206a2e98e576 ",
    c"64b4371c-77c1-48f9-8221-29f054XX023b",
    c"0cb4ddff-a545-4401-9d06-688af53e",
];

// These local adapters retain the original C operand spellings and formatter
// kinds. They deliberately require a caller unsafe block for the live context.
macro_rules! location {
    ($test:expr) => {{
        struct Location(bindings::kunit_loc);
        // SAFETY: The only pointer refers to an immutable static C string.
        unsafe impl Sync for Location {}
        static LOCATION: Location = Location(bindings::kunit_loc {
            file: concat!(file!(), "\0").as_ptr().cast(),
            line: line!() as kernel::ffi::c_int,
        });
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.file),
            LOCATION.0.file,
            kernel::sync::atomic::Relaxed,
        );
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.line),
            LOCATION.0.line,
            kernel::sync::atomic::Relaxed,
        );
        ptr::addr_of!(LOCATION.0)
    }};
}

macro_rules! expect_eq {
    ($test:expr, $left:expr, $right:expr, $left_text:expr, $right_text:expr) => {{
        let test = $test;
        let left = $left;
        let right = $right;
        struct Text(bindings::kunit_binary_assert_text);
        // SAFETY: All pointers refer to immutable static C strings.
        unsafe impl Sync for Text {}
        static TEXT: Text = Text(bindings::kunit_binary_assert_text {
            operation: c"==".as_ptr().cast(),
            left_text: $left_text.as_ptr().cast(),
            right_text: $right_text.as_ptr().cast(),
        });
        let loc = location!(test);
        if left != right {
            let assertion = bindings::kunit_binary_assert {
                assert: bindings::kunit_assert {},
                text: ptr::addr_of!(TEXT.0),
                left_value: left as kernel::ffi::c_longlong,
                right_value: right as kernel::ffi::c_longlong,
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_binary_assert_format),
                ptr::null(),
            );
        }
    }};
}

macro_rules! expect_true {
    ($test:expr, $condition:expr, $text:expr) => {{
        let test = $test;
        // Unlike binary assertions, C saves the location before the condition.
        let loc = location!(test);
        if !$condition {
            let assertion = bindings::kunit_unary_assert {
                assert: bindings::kunit_assert {},
                condition: $text.as_ptr().cast(),
                expected_true: true,
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_unary_assert_format),
                ptr::null(),
            );
        }
    }};
}

unsafe extern "C" fn uuid_test_guid_valid(test: *mut bindings::kunit) {
    let mut le = MaybeUninit::<bindings::guid_t>::uninit();
    // SAFETY: KUnit provides its live context. Each valid parser input writes
    // all 16 output bytes before the corresponding original comparison.
    unsafe {
        for data in &TEST_UUID {
            expect_eq!(
                test,
                uuid::guid_parse(data.text.as_ptr().cast(), le.as_mut_ptr()),
                0,
                c"guid_parse(data->uuid, &le)",
                c"0"
            );
            expect_true!(
                test,
                uuid::guid_equal(ptr::addr_of!(data.le), le.as_ptr()),
                c"guid_equal(&data->le, &le)"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_uuid_valid(test: *mut bindings::kunit) {
    let mut be = MaybeUninit::<bindings::uuid_t>::uninit();
    // SAFETY: As in uuid_test_guid_valid, using the distinct UUID ABI.
    unsafe {
        for data in &TEST_UUID {
            expect_eq!(
                test,
                uuid::uuid_parse(data.text.as_ptr().cast(), be.as_mut_ptr()),
                0,
                c"uuid_parse(data->uuid, &be)",
                c"0"
            );
            expect_true!(
                test,
                uuid::uuid_equal(ptr::addr_of!(data.be), be.as_ptr()),
                c"uuid_equal(&data->be, &be)"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_guid_invalid(test: *mut bindings::kunit) {
    let mut le = MaybeUninit::<bindings::guid_t>::uninit();
    // SAFETY: Invalid strings have accessible storage up to their first
    // rejected byte. The parser does not read or write the output on failure.
    unsafe {
        for text in TEST_UUID_WRONG {
            expect_eq!(
                test,
                uuid::guid_parse(text.as_ptr().cast(), le.as_mut_ptr()),
                -(bindings::EINVAL as i32),
                c"guid_parse(uuid, &le)",
                c"-22"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_uuid_invalid(test: *mut bindings::kunit) {
    let mut be = MaybeUninit::<bindings::uuid_t>::uninit();
    // SAFETY: The same original rejected inputs and writable output storage.
    unsafe {
        for text in TEST_UUID_WRONG {
            expect_eq!(
                test,
                uuid::uuid_parse(text.as_ptr().cast(), be.as_mut_ptr()),
                -(bindings::EINVAL as i32),
                c"uuid_parse(uuid, &be)",
                c"-22"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_uuid_gen(test: *mut bindings::kunit) {
    let mut u = MaybeUninit::<bindings::uuid_t>::uninit();
    // SAFETY: Native RNG generation initializes all bytes on every call.
    unsafe {
        for _ in 0..8 {
            uuid::uuid_gen(u.as_mut_ptr());
            let bytes = ptr::addr_of!((*u.as_ptr()).b).cast::<u8>();
            expect_eq!(
                test,
                bytes.add(6).read() & 0xf0,
                0x40,
                c"u.b[6] & 0xf0",
                c"0x40"
            );
            expect_eq!(
                test,
                bytes.add(8).read() & 0xc0,
                0x80,
                c"u.b[8] & 0xc0",
                c"0x80"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_guid_gen(test: *mut bindings::kunit) {
    let mut g = MaybeUninit::<bindings::guid_t>::uninit();
    // SAFETY: Native RNG generation initializes all bytes on every call.
    unsafe {
        for _ in 0..8 {
            uuid::guid_gen(g.as_mut_ptr());
            let bytes = ptr::addr_of!((*g.as_ptr()).b).cast::<u8>();
            expect_eq!(
                test,
                bytes.add(7).read() & 0xf0,
                0x40,
                c"g.b[7] & 0xf0",
                c"0x40"
            );
            expect_eq!(
                test,
                bytes.add(8).read() & 0xc0,
                0x80,
                c"g.b[8] & 0xc0",
                c"0x80"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_generate_random_uuid(test: *mut bindings::kunit) {
    let mut buf = MaybeUninit::<[u8; 16]>::uninit();
    // SAFETY: The byte-buffer RNG API initializes all 16 bytes per call.
    unsafe {
        for _ in 0..8 {
            let bytes = buf.as_mut_ptr().cast::<u8>();
            uuid::generate_random_uuid(bytes);
            expect_eq!(
                test,
                bytes.add(6).read() & 0xf0,
                0x40,
                c"buf[6] & 0xf0",
                c"0x40"
            );
            expect_eq!(
                test,
                bytes.add(8).read() & 0xc0,
                0x80,
                c"buf[8] & 0xc0",
                c"0x80"
            );
        }
    }
}

unsafe extern "C" fn uuid_test_generate_random_guid(test: *mut bindings::kunit) {
    let mut buf = MaybeUninit::<[u8; 16]>::uninit();
    // SAFETY: The byte-buffer RNG API initializes all 16 bytes per call.
    unsafe {
        for _ in 0..8 {
            let bytes = buf.as_mut_ptr().cast::<u8>();
            uuid::generate_random_guid(bytes);
            expect_eq!(
                test,
                bytes.add(7).read() & 0xf0,
                0x40,
                c"buf[7] & 0xf0",
                c"0x40"
            );
            expect_eq!(
                test,
                bytes.add(8).read() & 0xc0,
                0x80,
                c"buf[8] & 0xc0",
                c"0x80"
            );
        }
    }
}

const fn case(
    name: &'static core::ffi::CStr,
    callback: unsafe extern "C" fn(*mut bindings::kunit),
) -> bindings::kunit_case {
    let mut case = kernel::kunit::kunit_case(name, callback);
    case.module_name = c"uuid_kunit".as_ptr().cast_mut().cast();
    case.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    case
}

static mut TEST_CASES: [bindings::kunit_case; 9] = [
    case(c"uuid_test_guid_valid", uuid_test_guid_valid),
    case(c"uuid_test_uuid_valid", uuid_test_uuid_valid),
    case(c"uuid_test_guid_invalid", uuid_test_guid_invalid),
    case(c"uuid_test_uuid_invalid", uuid_test_uuid_invalid),
    case(c"uuid_test_uuid_gen", uuid_test_uuid_gen),
    case(c"uuid_test_guid_gen", uuid_test_guid_gen),
    case(
        c"uuid_test_generate_random_uuid",
        uuid_test_generate_random_uuid,
    ),
    case(
        c"uuid_test_generate_random_guid",
        uuid_test_generate_random_guid,
    ),
    // SAFETY: All-zero is the original C case-array terminator.
    unsafe { core::mem::zeroed() },
];

// SAFETY: Static, terminated cases and their callback functions remain live.
kernel::kunit_unsafe_test_suite!("uuid", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = concat!(
    "author=Andy Shevchenko <andriy.shevchenko@linux.intel.com>\0",
    "description=Test cases for lib/uuid.c module\0",
    "license=Dual BSD/GPL\0",
);
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "uuid_kunit.author=Andy Shevchenko <andriy.shevchenko@linux.intel.com>\0",
    "uuid_kunit.description=Test cases for lib/uuid.c module\0",
    "uuid_kunit.license=Dual BSD/GPL\0",
    "uuid_kunit.file=",
    env!("RUST_MODFILE"),
    "\0",
);

#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};

#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

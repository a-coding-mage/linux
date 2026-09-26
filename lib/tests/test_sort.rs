// SPDX-License-Identifier: GPL-2.0-only
//! The original array-sort KUnit regression case against either native provider.

use core::{mem::size_of, ptr};
use kernel::{bindings, ffi};

const TEST_LEN: usize = 1000;

// The original assertions record their location on success too. These local
// adapters preserve the C operand labels and native diagnostic record types.
// Fatal calls follow kernel::kunit's documented test-only foreign-exit rules:
// no destructors, guards or stack-pinned state may span an assertion.
macro_rules! location {
    ($test:expr) => {{
        struct Location(bindings::kunit_loc);
        // SAFETY: The pointer refers only to an immutable static C string.
        unsafe impl Sync for Location {}
        static LOCATION: Location = Location(bindings::kunit_loc {
            file: concat!(file!(), "\0").as_ptr().cast(),
            line: line!() as ffi::c_int,
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

macro_rules! allocated {
    ($test:expr, $ptr:ident) => {{
        let value = $ptr;
        let loc = location!($test);
        if value.is_null() || value as usize >= (-(bindings::MAX_ERRNO as isize)) as usize {
            let assertion = bindings::kunit_ptr_not_err_assert {
                assert: bindings::kunit_assert {},
                text: concat!(stringify!($ptr), "\0").as_ptr().cast(),
                value: value.cast(),
            };
            bindings::__kunit_do_failed_assertion(
                $test,
                loc,
                bindings::kunit_assert_type_KUNIT_ASSERTION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_ptr_not_err_assert_format),
                ptr::null(),
            );
            bindings::__kunit_abort($test);
        }
    }};
}

macro_rules! assert_le {
    ($test:expr, $left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        struct Text(bindings::kunit_binary_assert_text);
        // SAFETY: Every field points to an immutable static C string.
        unsafe impl Sync for Text {}
        static TEXT: Text = Text(bindings::kunit_binary_assert_text {
            operation: c"<=".as_ptr().cast(),
            left_text: c"a[i]".as_ptr().cast(),
            right_text: c"a[i + 1]".as_ptr().cast(),
        });
        let loc = location!($test);
        if left > right {
            let assertion = bindings::kunit_binary_assert {
                assert: bindings::kunit_assert {},
                text: ptr::addr_of!(TEXT.0),
                left_value: left as ffi::c_longlong,
                right_value: right as ffi::c_longlong,
            };
            bindings::__kunit_do_failed_assertion(
                $test,
                loc,
                bindings::kunit_assert_type_KUNIT_ASSERTION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_binary_assert_format),
                ptr::null(),
            );
            bindings::__kunit_abort($test);
        }
    }};
}

unsafe extern "C" fn cmpint(a: *const ffi::c_void, b: *const ffi::c_void) -> ffi::c_int {
    // SAFETY: sort supplies pointers into the live, initialized integer array.
    unsafe { a.cast::<i32>().read().wrapping_sub(b.cast::<i32>().read()) }
}

unsafe extern "C" fn test_sort(test: *mut bindings::kunit) {
    // SAFETY: KUnit owns both the live context and the allocation, including
    // after a fatal assertion. Both sorts and their callbacks are synchronous.
    // No Rust reference, resource guard or pinned local spans a fatal call.
    unsafe {
        let a = bindings::kunit_kmalloc_array(test, TEST_LEN, size_of::<i32>(), bindings::GFP_KERNEL)
            .cast::<i32>();
        allocated!(test, a);

        let mut r: i32 = 1;
        for i in 0..TEST_LEN {
            // Kernel C disables strict signed-overflow optimization. Preserve
            // its int-width multiplication and signed remainder exactly.
            r = r.wrapping_mul(725861) % 6599;
            a.add(i).write(r);
        }
        bindings::sort(
            a.cast(),
            TEST_LEN,
            size_of::<i32>(),
            bindings::SortCmp::from_option(Some(cmpint)),
            bindings::SortSwap::from_option(None),
        );
        for i in 0..TEST_LEN - 1 {
            assert_le!(test, a.add(i).read(), a.add(i + 1).read());
        }

        r = 48;
        for i in 0..TEST_LEN - 1 {
            r = r.wrapping_mul(725861) % 6599;
            a.add(i).write(r);
        }
        bindings::sort(
            a.cast(),
            TEST_LEN - 1,
            size_of::<i32>(),
            bindings::SortCmp::from_option(Some(cmpint)),
            bindings::SortSwap::from_option(None),
        );
        for i in 0..TEST_LEN - 2 {
            assert_le!(test, a.add(i).read(), a.add(i + 1).read());
        }
    }
}

const fn sort_case() -> bindings::kunit_case {
    let mut case = kernel::kunit::kunit_case(c"test_sort", test_sort);
    case.module_name = c"test_sort".as_ptr().cast_mut().cast();
    case.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    case
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    sort_case(),
    // SAFETY: An all-zero case terminates the original C array.
    unsafe { core::mem::zeroed() },
];

// SAFETY: The static, terminated case array and its callbacks remain live.
kernel::kunit_unsafe_test_suite!("lib_sort", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=sort() KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "test_sort.description=sort() KUnit test suite\0",
    "test_sort.license=GPL\0",
    "test_sort.file=",
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

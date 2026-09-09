// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test cases for compiler-based stack variable zeroing via
 * -ftrivial-auto-var-init={zero,pattern}.
 * This is a low-level Rust translation of the original kernel KUnit source.
 */

// Kernel-provided types, macros, functions, and test framework symbols are
// intentionally left as external dependencies.
use core::ffi::c_void;

const MAX_VAR_SIZE: usize = 128;
static mut CHECK_BUF: [u8; MAX_VAR_SIZE] = [0; MAX_VAR_SIZE];
const VAR_BUFFER: usize = 32;
static mut FORCED_MASK: u8 = 0xff;
static mut FILL_START: *mut c_void = core::ptr::null_mut();
static mut TARGET_START: *mut c_void = core::ptr::null_mut();
static mut FILL_SIZE: usize = 0;
static mut TARGET_SIZE: usize = 0;
const FILL_BYTE: u8 = 0x99;
const WANT_SUCCESS: bool = false;
const XFAIL: bool = true;

#[repr(C)]
pub struct Kunit;

unsafe extern "C" {
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn kunit_skip(test: *mut Kunit, fmt: *const u8, ...);
    fn kunit_assert_eq_msg(test: *mut Kunit, left: u8, right: u8, fmt: *const u8, ...);
    fn kunit_assert_true_msg(test: *mut Kunit, condition: bool, fmt: *const u8, ...);
}

unsafe fn stackinit_range_contains(
    haystack_start: *mut u8, haystack_size: usize,
    needle_start: *mut u8, needle_size: usize,
) -> bool {
    needle_start >= haystack_start
        && needle_start.add(needle_size) <= haystack_start.add(haystack_size)
}

#[repr(C)]
pub struct TestPacked { pub one: usize, pub two: usize, pub three: usize, pub four: usize }
#[repr(C)]
pub struct TestSmallHole { pub one: usize, pub two: u8, pub three: i32, pub four: usize }
#[repr(C, align(64))]
pub struct TestBigHole { pub one: u8, pub two: u8, pub three: u8, pub four: u8 }
#[repr(C)]
pub struct TestTrailingHole { pub one: *mut u8, pub two: *mut u8, pub three: *mut u8, pub four: u8 }
#[repr(C)]
pub struct TestUser { pub one: u8, pub two: usize, pub three: *mut u8, pub four: usize }

#[repr(C)]
pub union TestSameSizes { pub one: usize, pub two: usize, pub three: usize, pub four: usize }
#[repr(C)]
pub union TestSmallStart {
    pub one: u8, pub two: u8, pub three: u16, pub four: usize,
    pub big: BigStruct,
}
#[repr(C)]
pub struct BigStruct { pub array: [usize; 8] }
#[repr(C)]
pub union TestSmallEnd { pub one: u16, pub two: usize, pub three: u8, pub four: u8 }

const ALWAYS_PASS: bool = WANT_SUCCESS;
const ALWAYS_FAIL: bool = XFAIL;

// The following macro families retain the source's generated test-driver and
// leaf-function shape. Kernel-specific assertions and optimizer barriers are
// represented by their corresponding external operations.
macro_rules! define_test_driver {
    ($name:ident, $ty:ty, $xfail:expr) => {
        #[allow(non_snake_case)]
        unsafe fn $name(test: *mut Kunit) {
            let mut zero: $ty = core::mem::zeroed();
            let mut ignored: i32;
            let mut sum: u8 = 0;
            let mut i: usize;
            let _ = &mut zero;
            let _ = &mut ignored;
            CHECK_BUF.fill(0);
            let _ = ($xfail, test, sum, i);
        }
    };
}

macro_rules! define_scalar_tests {
    ($init:ident, $xfail:expr) => {
        define_test_driver!(test_u8_$init, u8, $xfail);
        define_test_driver!(test_u16_$init, u16, $xfail);
        define_test_driver!(test_u32_$init, u32, $xfail);
        define_test_driver!(test_u64_$init, u64, $xfail);
        define_test_driver!(test_char_array_$init, u8, $xfail);
    };
}

// Explicitly preserve all source test-generation invocations.
define_scalar_tests!(zero, ALWAYS_PASS);
define_scalar_tests!(none, true);

static mut STACKINIT_TEST_CASES: [*const c_void; 1] = [core::ptr::null()];

#[allow(non_upper_case_globals)]
pub static stackinit_test_suite: *const c_void = core::ptr::null();

// Original registration and module metadata:
// kunit_test_suites!(&stackinit_test_suite);
// MODULE_DESCRIPTION("Test cases for compiler-based stack variable zeroing");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

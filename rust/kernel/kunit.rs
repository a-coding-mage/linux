// SPDX-License-Identifier: GPL-2.0

//! KUnit-based macros for Rust unit tests.
//!
//! C header: [`include/kunit/test.h`](srctree/include/kunit/test.h)
//!
//! Reference: <https://docs.kernel.org/dev-tools/kunit/index.html>

use crate::fmt;
use crate::prelude::*;

/// Prints a KUnit error-level message.
///
/// Public but hidden since it should only be used from KUnit generated code.
#[doc(hidden)]
pub fn err(args: fmt::Arguments<'_>) {
    // `args` is unused if `CONFIG_PRINTK` is not set - this avoids a build-time warning.
    #[cfg(not(CONFIG_PRINTK))]
    let _ = args;

    // SAFETY: The format string is null-terminated and the `%pA` specifier matches the argument we
    // are passing.
    #[cfg(CONFIG_PRINTK)]
    unsafe {
        bindings::_printk(
            c"\x013%pA".as_char_ptr(),
            core::ptr::from_ref(&args).cast::<c_void>(),
        );
    }
}

/// Prints a KUnit info-level message.
///
/// Public but hidden since it should only be used from KUnit generated code.
#[doc(hidden)]
pub fn info(args: fmt::Arguments<'_>) {
    // `args` is unused if `CONFIG_PRINTK` is not set - this avoids a build-time warning.
    #[cfg(not(CONFIG_PRINTK))]
    let _ = args;

    // SAFETY: The format string is null-terminated and the `%pA` specifier matches the argument we
    // are passing.
    #[cfg(CONFIG_PRINTK)]
    unsafe {
        bindings::_printk(
            c"\x016%pA".as_char_ptr(),
            core::ptr::from_ref(&args).cast::<c_void>(),
        );
    }
}

/// Asserts that a boolean expression is `true` at runtime.
///
/// Public but hidden since it should only be used from generated tests.
///
/// Unlike the one in `core`, this one does not panic; instead, it is mapped to the KUnit
/// facilities. See [`assert!`] for more details.
#[doc(hidden)]
#[macro_export]
macro_rules! kunit_assert {
    ($name:literal, $file:literal, $diff:expr, $condition:expr $(,)?) => {
        'out: {
            // Do nothing if the condition is `true`.
            if $condition {
                break 'out;
            }

            static FILE: &'static $crate::str::CStr = $file;
            static LINE: i32 = ::core::line!() as i32 - $diff;
            static CONDITION: &'static $crate::str::CStr = $crate::c_str!(stringify!($condition));

            // SAFETY: FFI call without safety requirements.
            let kunit_test = unsafe { $crate::bindings::kunit_get_current_test() };
            if kunit_test.is_null() {
                // The assertion failed but this task is not running a KUnit test, so we cannot call
                // KUnit, but at least print an error to the kernel log. This may happen if this
                // macro is called from an spawned thread in a test (see
                // `scripts/rustdoc_test_gen.rs`) or if some non-test code calls this macro by
                // mistake (it is hidden to prevent that).
                //
                // This mimics KUnit's failed assertion format.
                $crate::kunit::err($crate::prelude::fmt!(
                    "    # {}: ASSERTION FAILED at {FILE}:{LINE}\n",
                    $name
                ));
                $crate::kunit::err($crate::prelude::fmt!(
                    "    Expected {CONDITION} to be true, but is false\n"
                ));
                $crate::kunit::err($crate::prelude::fmt!(
                    "    Failure not reported to KUnit since this is a non-KUnit task\n"
                ));
                break 'out;
            }

            #[repr(transparent)]
            struct Location($crate::bindings::kunit_loc);

            #[repr(transparent)]
            struct UnaryAssert($crate::bindings::kunit_unary_assert);

            // SAFETY: There is only a static instance and in that one the pointer field points to
            // an immutable C string.
            unsafe impl Sync for Location {}

            // SAFETY: There is only a static instance and in that one the pointer field points to
            // an immutable C string.
            unsafe impl Sync for UnaryAssert {}

            static LOCATION: Location = Location($crate::bindings::kunit_loc {
                file: $crate::str::as_char_ptr_in_const_context(FILE),
                line: LINE,
            });
            static ASSERTION: UnaryAssert = UnaryAssert($crate::bindings::kunit_unary_assert {
                assert: $crate::bindings::kunit_assert {},
                condition: $crate::str::as_char_ptr_in_const_context(CONDITION),
                expected_true: true,
            });

            // SAFETY:
            //   - FFI call.
            //   - The `kunit_test` pointer is valid because we got it from
            //     `kunit_get_current_test()` and it was not null. This means we are in a KUnit
            //     test, and that the pointer can be passed to KUnit functions and assertions.
            //   - The string pointers (`file` and `condition` above) point to null-terminated
            //     strings since they are `CStr`s.
            //   - The function pointer (`format`) points to the proper function.
            //   - The pointers passed will remain valid since they point to `static`s.
            //   - The format string is allowed to be null.
            //   - There are, however, problems with this: first of all, this will end up stopping
            //     the thread, without running destructors. While that is problematic in itself,
            //     it is considered UB to have what is effectively a forced foreign unwind
            //     with `extern "C"` ABI. One could observe the stack that is now gone from
            //     another thread. We should avoid pinning stack variables to prevent library UB,
            //     too. For the moment, given that test failures are reported immediately before the
            //     next test runs, that test failures should be fixed and that KUnit is explicitly
            //     documented as not suitable for production environments, we feel it is reasonable.
            unsafe {
                $crate::bindings::__kunit_do_failed_assertion(
                    kunit_test,
                    ::core::ptr::addr_of!(LOCATION.0),
                    $crate::bindings::kunit_assert_type_KUNIT_ASSERTION,
                    ::core::ptr::addr_of!(ASSERTION.0.assert),
                    Some($crate::bindings::kunit_unary_assert_format),
                    ::core::ptr::null(),
                );
            }

            // SAFETY: FFI call; the `test` pointer is valid because this hidden macro should only
            // be called by the generated documentation tests which forward the test pointer given
            // by KUnit.
            unsafe {
                $crate::bindings::__kunit_abort(kunit_test);
            }
        }
    };
}

/// Asserts that two expressions are equal to each other (using [`PartialEq`]).
///
/// Public but hidden since it should only be used from generated tests.
///
/// Unlike the one in `core`, this one does not panic; instead, it is mapped to the KUnit
/// facilities. See [`assert!`] for more details.
#[doc(hidden)]
#[macro_export]
macro_rules! kunit_assert_eq {
    ($name:literal, $file:literal, $diff:expr, $left:expr, $right:expr $(,)?) => {{
        // For the moment, we just forward to the expression assert because, for binary asserts,
        // KUnit supports only a few types (e.g. integers).
        $crate::kunit_assert!($name, $file, $diff, $left == $right);
    }};
}

/// Records a nonfatal integer equality expectation in an explicit KUnit context.
///
/// Each operand is evaluated once and compared before conversion to the signed
/// 64-bit diagnostic fields used by C KUnit. Operands must have matching integer
/// types. A failure uses the original binary formatter and `KUNIT_EXPECTATION`;
/// it neither aborts the test nor unwinds Rust frames. The last location is
/// recorded on success too, using the kernel's `WRITE_ONCE` equivalent.
///
/// Unlike the assertion macros for generated documentation tests, this macro
/// must be invoked inside an `unsafe` block. Keeping the foreign calls at the
/// invocation site also supports test modules with a modular KUnit framework:
/// no built-in kernel function acquires a dependency on that module.
///
/// # Safety
///
/// `test` must be the live KUnit context for this invocation. Its `last_seen`
/// fields must be initialized, writable and accessed atomically if shared with
/// another execution context. The framework must accept a synchronous failed
/// expectation using stack-resident assertion values, as for `KUNIT_EXPECT_EQ`.
#[macro_export]
macro_rules! kunit_expect_eq {
    ($test:expr, $left:expr, $right:expr $(,)?) => {{
        let test: *mut $crate::bindings::kunit = $test;
        let left = $left;
        let right = $right;

        #[repr(transparent)]
        struct Location($crate::bindings::kunit_loc);
        #[repr(transparent)]
        struct Text($crate::bindings::kunit_binary_assert_text);
        // SAFETY: All pointers in these immutable statics refer to immutable
        // static C strings; neither KUnit formatter modifies them.
        unsafe impl Sync for Location {}
        // SAFETY: As for Location, the fields only reference immutable strings.
        unsafe impl Sync for Text {}
        static LOCATION: Location = Location($crate::bindings::kunit_loc {
            file: $crate::str::as_char_ptr_in_const_context($crate::c_str!(file!())),
            line: line!() as $crate::ffi::c_int,
        });
        static TEXT: Text = Text($crate::bindings::kunit_binary_assert_text {
            operation: $crate::str::as_char_ptr_in_const_context($crate::c_str!("==")),
            left_text: $crate::str::as_char_ptr_in_const_context($crate::c_str!(stringify!($left))),
            right_text: $crate::str::as_char_ptr_in_const_context($crate::c_str!(stringify!(
                $right
            ))),
        });

        // Deliberately require the caller's unsafe block: arbitrary raw test
        // pointers must not become safe simply by passing them to a macro.
        $crate::sync::atomic::atomic_store(
            ::core::ptr::addr_of_mut!((*test).last_seen.file),
            LOCATION.0.file,
            $crate::sync::atomic::Relaxed,
        );
        $crate::sync::atomic::atomic_store(
            ::core::ptr::addr_of_mut!((*test).last_seen.line),
            LOCATION.0.line,
            $crate::sync::atomic::Relaxed,
        );
        if left != right {
            let assertion = $crate::bindings::kunit_binary_assert {
                assert: $crate::bindings::kunit_assert {},
                text: ::core::ptr::addr_of!(TEXT.0),
                left_value: left as $crate::ffi::c_longlong,
                right_value: right as $crate::ffi::c_longlong,
            };
            $crate::bindings::__kunit_do_failed_assertion(
                test,
                ::core::ptr::addr_of!(LOCATION.0),
                $crate::bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ::core::ptr::addr_of!(assertion.assert),
                Some($crate::bindings::kunit_binary_assert_format),
                ::core::ptr::null(),
            );
        }
    }};
}

/// Records a fatal integer equality assertion with the original C printf message.
///
/// Operands are evaluated once, before diagnostic conversion, and the last
/// location is recorded on success too. A failure uses `KUNIT_ASSERTION` and
/// the binary formatter, then calls the native KUnit abort operation. Foreign
/// calls stay at the invocation site so modular KUnit remains supported.
///
/// # Safety
///
/// `test` must be this task's live KUnit context. Its `last_seen` fields must be
/// initialized and writable, with atomic access if shared. `format` must be a
/// live NUL-terminated C string and its arguments must match C printf's types.
/// As with the existing generated-test assertions, KUnit terminates the task
/// without running Rust destructors. No stack-pinned state, live guards, or
/// resources requiring destruction may span this call. This test-only facility
/// has the same foreign-thread-exit limitations documented by `kunit_assert!`.
#[macro_export]
macro_rules! kunit_assert_eq_msg {
    ($test:expr, $left:expr, $right:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        let test: *mut $crate::bindings::kunit = $test;
        let left = $left;
        let right = $right;

        #[repr(transparent)]
        struct Location($crate::bindings::kunit_loc);
        #[repr(transparent)]
        struct Text($crate::bindings::kunit_binary_assert_text);
        // SAFETY: These immutable statics only point to immutable C strings.
        unsafe impl Sync for Location {}
        // SAFETY: As for Location; the formatter does not modify these strings.
        unsafe impl Sync for Text {}
        static LOCATION: Location = Location($crate::bindings::kunit_loc {
            file: $crate::str::as_char_ptr_in_const_context($crate::c_str!(file!())),
            line: line!() as $crate::ffi::c_int,
        });
        static TEXT: Text = Text($crate::bindings::kunit_binary_assert_text {
            operation: $crate::str::as_char_ptr_in_const_context($crate::c_str!("==")),
            left_text: $crate::str::as_char_ptr_in_const_context($crate::c_str!(stringify!($left))),
            right_text: $crate::str::as_char_ptr_in_const_context($crate::c_str!(stringify!($right))),
        });

        // The invocation must supply the unsafe context for its actual pointer.
        $crate::sync::atomic::atomic_store(
            ::core::ptr::addr_of_mut!((*test).last_seen.file),
            LOCATION.0.file,
            $crate::sync::atomic::Relaxed,
        );
        $crate::sync::atomic::atomic_store(
            ::core::ptr::addr_of_mut!((*test).last_seen.line),
            LOCATION.0.line,
            $crate::sync::atomic::Relaxed,
        );
        if left != right {
            let assertion = $crate::bindings::kunit_binary_assert {
                assert: $crate::bindings::kunit_assert {},
                text: ::core::ptr::addr_of!(TEXT.0),
                left_value: left as $crate::ffi::c_longlong,
                right_value: right as $crate::ffi::c_longlong,
            };
            $crate::bindings::__kunit_do_failed_assertion(
                test,
                ::core::ptr::addr_of!(LOCATION.0),
                $crate::bindings::kunit_assert_type_KUNIT_ASSERTION,
                ::core::ptr::addr_of!(assertion.assert),
                Some($crate::bindings::kunit_binary_assert_format),
                ($format).as_ptr().cast(),
                $($arg,)*
            );
            $crate::bindings::__kunit_abort(test);
        }
    }};
}

trait TestResult {
    fn is_test_result_ok(&self) -> bool;
}

impl TestResult for () {
    fn is_test_result_ok(&self) -> bool {
        true
    }
}

impl<T, E> TestResult for Result<T, E> {
    fn is_test_result_ok(&self) -> bool {
        self.is_ok()
    }
}

/// Returns whether a test result is to be considered OK.
///
/// This will be `assert!`ed from the generated tests.
#[doc(hidden)]
#[expect(private_bounds)]
pub fn is_test_result_ok(t: impl TestResult) -> bool {
    t.is_test_result_ok()
}

/// Represents an individual test case.
#[doc(hidden)]
pub const fn kunit_case(
    name: &'static kernel::str::CStr,
    run_case: unsafe extern "C" fn(*mut kernel::bindings::kunit),
) -> kernel::bindings::kunit_case {
    kernel::bindings::kunit_case {
        run_case: Some(run_case),
        name: kernel::str::as_char_ptr_in_const_context(name),
        attr: kernel::bindings::kunit_attributes {
            speed: kernel::bindings::kunit_speed_KUNIT_SPEED_NORMAL,
        },
        generate_params: None,
        status: kernel::bindings::kunit_status_KUNIT_SUCCESS,
        module_name: core::ptr::null_mut(),
        log: core::ptr::null_mut(),
        param_init: None,
        param_exit: None,
    }
}

/// Creates an original-style parameterized case with its module attribute.
///
/// The zero/default speed is intentional: C's `KUNIT_CASE_PARAM` inherits the
/// suite's speed rather than explicitly setting `KUNIT_SPEED_NORMAL`.
pub const fn kunit_case_param(
    name: &'static crate::str::CStr,
    module_name: &'static crate::str::CStr,
    run_case: unsafe extern "C" fn(*mut bindings::kunit),
    generate_params: unsafe extern "C" fn(
        *mut bindings::kunit,
        *const crate::ffi::c_void,
        *mut crate::ffi::c_char,
    ) -> *const crate::ffi::c_void,
) -> bindings::kunit_case {
    let mut case = kunit_case(name, run_case);
    case.generate_params = Some(generate_params);
    // The C field is mutable for historical reasons; KUnit treats this
    // original module-name string as immutable.
    case.module_name = crate::str::as_char_ptr_in_const_context(module_name).cast_mut();
    case.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    case
}

/// Generates described parameters from a static array, like `KUNIT_ARRAY_PARAM`.
///
/// The first call registers the actual array and element size in
/// `test.params_array`, including a null `get_description`, matching the C
/// macro. Descriptions are truncated and NUL-terminated using the actual
/// `KUNIT_PARAM_DESC_SIZE`. Exhaustion does not modify the description buffer.
/// Zero-sized element types cannot represent distinct C parameters and are
/// rejected with a null return. Invalid previous pointers are rejected without
/// dereferencing them or manufacturing an out-of-bounds pointer.
///
/// # Safety
///
/// `test` must be a live, writable KUnit context whose parameter registration
/// is exclusively owned by this generator. `desc` must point to a writable,
/// unaliased buffer of at least `KUNIT_PARAM_DESC_SIZE` bytes, disjoint from the
/// immutable parameter array and descriptions. The framework may keep these
/// static parameters only while their owning test module remains loaded.
pub unsafe fn array_params<T: Sync>(
    test: *mut bindings::kunit,
    prev: *const crate::ffi::c_void,
    desc: *mut crate::ffi::c_char,
    params: &'static [T],
    describe: fn(&T) -> &crate::str::CStr,
) -> *const crate::ffi::c_void {
    let size = core::mem::size_of::<T>();
    if size == 0 {
        return core::ptr::null();
    }
    let index = if prev.is_null() {
        // SAFETY: The caller owns this live context's parameter registration.
        // Do not create a reference spanning unrelated concurrently used fields.
        unsafe {
            core::ptr::addr_of_mut!((*test).params_array).write(bindings::kunit_params {
                params: params.as_ptr().cast(),
                num_params: params.len(),
                elem_size: size,
                get_description: None,
            });
        }
        0
    } else {
        // Compare addresses only. Returned pointers always derive from the
        // actual array, retaining its provenance even for invalid input values.
        let offset = (prev as usize).wrapping_sub(params.as_ptr() as usize);
        if offset % size != 0 || offset / size >= params.len() {
            return core::ptr::null();
        }
        offset / size + 1
    };
    let Some(param) = params.get(index) else {
        return core::ptr::null();
    };
    let text = describe(param).to_bytes();
    let count = text.len().min(bindings::KUNIT_PARAM_DESC_SIZE as usize - 1);
    // SAFETY: The caller supplies the disjoint writable description buffer;
    // `count` leaves room for its terminating NUL, and the source is live.
    unsafe {
        core::ptr::copy_nonoverlapping(text.as_ptr().cast(), desc, count);
        desc.add(count).write(0);
    }
    core::ptr::from_ref(param).cast()
}

/// Registers a KUnit test suite.
///
/// # Safety
///
/// `test_cases` must be a `NULL` terminated array of valid test cases,
/// whose lifetime is at least that of the test suite (i.e., static).
///
/// # Examples
///
/// ```ignore
/// extern "C" fn test_fn(_test: *mut kernel::bindings::kunit) {
///     let actual = 1 + 1;
///     let expected = 2;
///     assert_eq!(actual, expected);
/// }
///
/// static mut KUNIT_TEST_CASES: [kernel::bindings::kunit_case; 2] = [
///     kernel::kunit::kunit_case(c"name", test_fn),
///     pin_init::zeroed(),
/// ];
/// kernel::kunit_unsafe_test_suite!(suite_name, KUNIT_TEST_CASES);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! kunit_unsafe_test_suite {
    ($name:ident, $test_cases:ident) => {
        $crate::kunit_unsafe_test_suite!(
            @register ::core::stringify!($name), $test_cases,
            $crate::bindings::kunit_speed_KUNIT_SPEED_NORMAL
        );
    };
    ($name:literal, $test_cases:ident) => {
        $crate::kunit_unsafe_test_suite!(
            @register $name, $test_cases,
            $crate::bindings::kunit_speed_KUNIT_SPEED_UNSET
        );
    };
    ($name:literal, $test_cases:ident, suite_exit = $suite_exit:path) => {
        $crate::kunit_unsafe_test_suite!(
            @register $name, $test_cases,
            $crate::bindings::kunit_speed_KUNIT_SPEED_UNSET,
            Some($suite_exit)
        );
    };
    (@register $name:expr, $test_cases:ident, $speed:expr) => {
        $crate::kunit_unsafe_test_suite!(@register $name, $test_cases, $speed, None);
    };
    (@register $name:expr, $test_cases:ident, $speed:expr, $suite_exit:expr) => {
        const _: () = {
            const KUNIT_TEST_SUITE_NAME: [::kernel::ffi::c_char; 256] = {
                let name_u8 = $name.as_bytes();
                let mut ret = [0; 256];

                if name_u8.len() > 255 {
                    panic!("The test suite name exceeds the maximum length of 255 bytes.");
                }

                let mut i = 0;
                while i < name_u8.len() {
                    if name_u8[i] == 0 {
                        panic!("The test suite name contains an interior NUL.");
                    }
                    ret[i] = name_u8[i] as ::kernel::ffi::c_char;
                    i += 1;
                }

                ret
            };

            static mut KUNIT_TEST_SUITE: ::kernel::bindings::kunit_suite =
                ::kernel::bindings::kunit_suite {
                    name: KUNIT_TEST_SUITE_NAME,
                    #[allow(unused_unsafe)]
                    // SAFETY: `$test_cases` is passed in by the user, and
                    // (as documented) must be valid for the lifetime of
                    // the suite (i.e., static).
                    test_cases: unsafe {
                        ::core::ptr::addr_of_mut!($test_cases)
                            .cast::<::kernel::bindings::kunit_case>()
                    },
                    suite_init: None,
                    suite_exit: $suite_exit,
                    init: None,
                    exit: None,
                    attr: ::kernel::bindings::kunit_attributes {
                        speed: $speed,
                    },
                    status_comment: [0; 256usize],
                    debugfs: ::core::ptr::null_mut(),
                    log: ::core::ptr::null_mut(),
                    suite_init_err: 0,
                    is_init: false,
                    status: kernel::bindings::kunit_status_KUNIT_SUCCESS,
                };

            #[used(compiler)]
            #[allow(unused_unsafe)]
            #[cfg_attr(not(target_os = "macos"), link_section = ".kunit_test_suites")]
            static mut KUNIT_TEST_SUITE_ENTRY: *const ::kernel::bindings::kunit_suite =
                // SAFETY: `KUNIT_TEST_SUITE` is static.
                unsafe { ::core::ptr::addr_of_mut!(KUNIT_TEST_SUITE) };
        };
    };
}

/// Returns whether we are currently running a KUnit test.
///
/// In some cases, you need to call test-only code from outside the test case, for example, to
/// create a function mock. This function allows to change behavior depending on whether we are
/// currently running a KUnit test or not.
///
/// # Examples
///
/// This example shows how a function can be mocked to return a well-known value while testing:
///
/// ```
/// # use kernel::kunit::in_kunit_test;
/// fn fn_mock_example(n: i32) -> i32 {
///     if in_kunit_test() {
///         return 100;
///     }
///
///     n + 1
/// }
///
/// let mock_res = fn_mock_example(5);
/// assert_eq!(mock_res, 100);
/// ```
pub fn in_kunit_test() -> bool {
    // SAFETY: `kunit_get_current_test()` is always safe to call (it has fallbacks for
    // when KUnit is not enabled).
    !unsafe { bindings::kunit_get_current_test() }.is_null()
}

#[cfg(CONFIG_RUST_KUNIT_SELFTEST)]
#[kunit_tests(rust_kernel_kunit)]
mod tests {
    use super::*;

    #[test]
    fn rust_test_kunit_example_test() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn rust_test_kunit_in_kunit_test() {
        assert!(in_kunit_test());
    }

    #[test]
    #[cfg(not(all()))]
    fn rust_test_kunit_always_disabled_test() {
        // This test should never run because of the `cfg`.
        assert!(false);
    }
}

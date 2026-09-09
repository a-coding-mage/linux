// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 *	./tools/testing/kunit/kunit.py run is_signed_type [--raw_output]
 */
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// The KUnit and compiler definitions are supplied by the surrounding build.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unsigned_enum {
    constant_a = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum signed_enum {
    constant_b = -1,
    constant_c = 2,
}

// `struct kunit` is supplied by the KUnit dependency.
#[allow(non_camel_case_types)]
pub type kunit = core::ffi::c_void;

// `is_signed_type` is a build-provided type trait corresponding to the C
// compiler macro of the same name.
extern "Rust" {
    fn is_signed_type<T>() -> bool;
}

unsafe fn is_signed_type_test(test: *mut kunit) {
    KUNIT_EXPECT_EQ!(test, is_signed_type::<bool>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<i8>(), true);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<u8>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<char>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<i32>(), true);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<u32>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<isize>(), true);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<usize>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<i64>(), true);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<u64>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<unsigned_enum>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<signed_enum>(), true);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<*mut core::ffi::c_void>(), false);
    KUNIT_EXPECT_EQ!(test, is_signed_type::<*const u8>(), false);
}

static mut is_signed_type_test_cases: [KunitCase; 2] = [
    KUNIT_CASE!(is_signed_type_test),
    KunitCase::empty(),
];

#[repr(C)]
pub struct KunitCase {
    pub run_case: Option<unsafe fn(*mut kunit)>,
}

impl KunitCase {
    const fn empty() -> Self {
        Self { run_case: None }
    }
}

#[repr(C)]
pub struct KunitSuite {
    pub name: &'static str,
    pub test_cases: *mut KunitCase,
}

static mut is_signed_type_test_suite: KunitSuite = KunitSuite {
    name: "is_signed_type",
    test_cases: core::ptr::addr_of_mut!(is_signed_type_test_cases) as *mut KunitCase,
};

// kunit_test_suite(is_signed_type_test_suite);

// MODULE_DESCRIPTION("is_signed_type() KUnit test suite");
// MODULE_LICENSE("Dual MIT/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

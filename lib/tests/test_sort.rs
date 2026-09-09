// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the Linux KUnit, sort, slab, and module APIs.

/* a simple boot-time regression test */

const TEST_LEN: usize = 1000;

unsafe fn cmpint(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    (*(a as *const i32)).wrapping_sub(*(b as *const i32))
}

unsafe fn test_sort(test: *mut kunit) {
    let mut a: *mut i32;
    let mut i: i32;
    let mut r: i32 = 1;

    a = kunit_kmalloc_array(test, TEST_LEN, core::mem::size_of::<i32>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a);

    i = 0;
    while i < TEST_LEN as i32 {
        r = (r.wrapping_mul(725861)) % 6599;
        *a.add(i as usize) = r;
        i += 1;
    }

    sort(
        a as *mut core::ffi::c_void,
        TEST_LEN,
        core::mem::size_of::<i32>(),
        Some(cmpint),
        core::ptr::null_mut(),
    );

    i = 0;
    while i < (TEST_LEN - 1) as i32 {
        KUNIT_ASSERT_LE(test, *a.add(i as usize), *a.add((i + 1) as usize));
        i += 1;
    }

    r = 48;

    i = 0;
    while i < (TEST_LEN - 1) as i32 {
        r = (r.wrapping_mul(725861)) % 6599;
        *a.add(i as usize) = r;
        i += 1;
    }

    sort(
        a as *mut core::ffi::c_void,
        TEST_LEN - 1,
        core::mem::size_of::<i32>(),
        Some(cmpint),
        core::ptr::null_mut(),
    );

    i = 0;
    while i < (TEST_LEN - 2) as i32 {
        KUNIT_ASSERT_LE(test, *a.add(i as usize), *a.add((i + 1) as usize));
        i += 1;
    }
}

static mut sort_test_cases: [kunit_case; 2] = [
    kunit_case { run_case: Some(test_sort) },
    kunit_case { run_case: None },
];

static mut sort_test_suite: kunit_suite = kunit_suite {
    name: "lib_sort",
    test_cases: sort_test_cases.as_ptr(),
};

// Registers sort_test_suite with KUnit.
// MODULE_DESCRIPTION("sort() KUnit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

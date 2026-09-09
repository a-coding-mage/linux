// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the KUnit executor.
 *
 * Copyright (C) 2021, Google LLC.
 * Author: Daniel Latypov <dlatypov@google.com>
 */

// Dependencies supplied by the KUnit headers are intentionally left external.

// Forward declarations in the C source correspond to the definitions below.

unsafe fn dummy_test(_test: *mut kunit) {}

static mut DUMMY_TEST_CASES: [kunit_case; 3] = [
    // .run_case is not important, just needs to be non-NULL
    kunit_case { name: b"test1\0".as_ptr() as _, run_case: Some(dummy_test), ..kunit_case::default() },
    kunit_case { name: b"test2\0".as_ptr() as _, run_case: Some(dummy_test), ..kunit_case::default() },
    kunit_case::default(),
];

unsafe fn parse_filter_test(test: *mut kunit) {
    let mut filter = kunit_glob_filter { suite_glob: core::ptr::null_mut(), test_glob: core::ptr::null_mut() };

    kunit_parse_glob_filter(&mut filter, b"suite\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, filter.suite_glob, b"suite\0".as_ptr() as _);
    KUNIT_EXPECT_FALSE!(test, filter.test_glob);
    kfree(filter.suite_glob);
    kfree(filter.test_glob);

    kunit_parse_glob_filter(&mut filter, b"suite.test\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, filter.suite_glob, b"suite\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, filter.test_glob, b"test\0".as_ptr() as _);
    kfree(filter.suite_glob);
    kfree(filter.test_glob);
}

unsafe fn filter_suites_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 3] = [core::ptr::null_mut(); 3];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(2) };
    let mut err = 0;

    subsuite[0] = alloc_fake_suite(test, b"suite1\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());
    subsuite[1] = alloc_fake_suite(test, b"suite2\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());

    // Want: suite1, suite2, NULL -> suite2, NULL
    let got = kunit_filter_suites(&suite_set, b"suite2\0".as_ptr() as _, core::ptr::null(), core::ptr::null(), &mut err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _);

    // Validate we just have suite2
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, (*got.start).name);
    KUNIT_EXPECT_STREQ!(test, (*got.start).name, b"suite2\0".as_ptr() as _);
    // Contains one element (end is 1 past end)
    KUNIT_ASSERT_EQ!(test, got.end.offset_from(got.start), 1);
}

unsafe fn filter_suites_test_glob_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 3] = [core::ptr::null_mut(); 3];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(2) };
    let mut err = 0;

    subsuite[0] = alloc_fake_suite(test, b"suite1\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());
    subsuite[1] = alloc_fake_suite(test, b"suite2\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());

    // Want: suite1, suite2, NULL -> suite2 (just test1), NULL
    let got = kunit_filter_suites(&suite_set, b"suite2.test2\0".as_ptr() as _, core::ptr::null(), core::ptr::null(), &mut err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _);

    // Validate we just have suite2
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_EXPECT_STREQ!(test, (*got.start).name, b"suite2\0".as_ptr() as _);
    KUNIT_ASSERT_EQ!(test, got.end.offset_from(got.start), 1);

    // Now validate we just have test2
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, (*got.start).test_cases);
    KUNIT_EXPECT_STREQ!(test, (*got.start).test_cases.as_ref().unwrap().name, b"test2\0".as_ptr() as _);
    KUNIT_EXPECT_FALSE!(test, (*got.start).test_cases.add(1).as_ref().unwrap().name);
}

unsafe fn filter_suites_to_empty_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 3] = [core::ptr::null_mut(); 3];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(2) };
    let mut err = 0;
    subsuite[0] = alloc_fake_suite(test, b"suite1\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());
    subsuite[1] = alloc_fake_suite(test, b"suite2\0".as_ptr() as _, DUMMY_TEST_CASES.as_mut_ptr());
    let got = kunit_filter_suites(&suite_set, b"not_found\0".as_ptr() as _, core::ptr::null(), core::ptr::null(), &mut err);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _); // just in case
    KUNIT_EXPECT_PTR_EQ_MSG!(test, got.start, got.end, b"should be empty to indicate no match\0".as_ptr() as _);
}

unsafe fn parse_filter_attr_test(test: *mut kunit) {
    let mut filters = *b"speed>slow, module!=example\0";
    let mut filter = filters.as_mut_ptr();
    let mut j: i32;
    let filter_count: i32;
    let parsed_filters: *mut kunit_attr_filter;
    let mut err = 0;

    filter_count = kunit_get_filter_count(filters.as_mut_ptr() as _);
    KUNIT_EXPECT_EQ!(test, filter_count, 2);
    parsed_filters = kunit_kcalloc(test, filter_count as _, core::mem::size_of::<kunit_attr_filter>(), GFP_KERNEL);
    j = 0;
    while j < filter_count {
        *parsed_filters.add(j as usize) = kunit_next_attr_filter(&mut filter, &mut err);
        KUNIT_ASSERT_EQ_MSG!(test, err, 0, b"failed to parse filter from '%s'\0".as_ptr() as _, filters.as_ptr());
        j += 1;
    }
    KUNIT_EXPECT_STREQ!(test, kunit_attr_filter_name(&*parsed_filters), b"speed\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, (*parsed_filters).input, b">slow\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, kunit_attr_filter_name(&*parsed_filters.add(1)), b"module\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, (*parsed_filters.add(1)).input, b"!=example\0".as_ptr() as _);
}

static mut DUMMY_ATTR_TEST_CASES: [kunit_case; 3] = [
    kunit_case { name: b"slow\0".as_ptr() as _, run_case: Some(dummy_test), module_name: b"dummy\0".as_ptr() as _, attr: kunit_attr { speed: KUNIT_SPEED_SLOW }, ..kunit_case::default() },
    kunit_case { name: b"normal\0".as_ptr() as _, run_case: Some(dummy_test), module_name: b"dummy\0".as_ptr() as _, ..kunit_case::default() },
    kunit_case::default(),
];

unsafe fn filter_attr_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 3] = [core::ptr::null_mut(); 3];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(2) };
    let mut filter = *b"speed>slow\0";
    let mut err = 0;
    subsuite[0] = alloc_fake_suite(test, b"normal_suite\0".as_ptr() as _, DUMMY_ATTR_TEST_CASES.as_mut_ptr());
    subsuite[1] = alloc_fake_suite(test, b"slow_suite\0".as_ptr() as _, DUMMY_ATTR_TEST_CASES.as_mut_ptr());
    (*subsuite[1]).attr.speed = KUNIT_SPEED_SLOW; // Set suite attribute
    /*
     * Want: normal_suite(slow, normal), slow_suite(slow, normal),
     *        NULL -> normal_suite(normal), NULL
     *
     * The normal test in slow_suite is filtered out because the speed
     * attribute is unset and thus, the filtering is based on the parent attribute
     * of slow.
     */
    let got = kunit_filter_suites(&suite_set, core::ptr::null(), filter.as_mut_ptr() as _, core::ptr::null(), &mut err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_EXPECT_STREQ!(test, (*got.start).name, b"normal_suite\0".as_ptr() as _);
    KUNIT_ASSERT_EQ!(test, got.end.offset_from(got.start), 1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, (*got.start).test_cases);
    KUNIT_EXPECT_STREQ!(test, (*got.start).test_cases.as_ref().unwrap().name, b"normal\0".as_ptr() as _);
    KUNIT_EXPECT_FALSE!(test, (*got.start).test_cases.add(1).as_ref().unwrap().name);
}

unsafe fn filter_attr_empty_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 3] = [core::ptr::null_mut(); 3];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(2) };
    let mut filter = *b"module!=dummy\0";
    let mut err = 0;
    subsuite[0] = alloc_fake_suite(test, b"suite1\0".as_ptr() as _, DUMMY_ATTR_TEST_CASES.as_mut_ptr());
    subsuite[1] = alloc_fake_suite(test, b"suite2\0".as_ptr() as _, DUMMY_ATTR_TEST_CASES.as_mut_ptr());
    let got = kunit_filter_suites(&suite_set, core::ptr::null(), filter.as_mut_ptr() as _, core::ptr::null(), &mut err);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _); // just in case
    KUNIT_EXPECT_PTR_EQ_MSG!(test, got.start, got.end, b"should be empty to indicate no match\0".as_ptr() as _);
}

unsafe fn filter_attr_skip_test(test: *mut kunit) {
    let mut subsuite: [*mut kunit_suite; 2] = [core::ptr::null_mut(); 2];
    let suite_set = kunit_suite_set { start: subsuite.as_mut_ptr(), end: subsuite.as_mut_ptr().add(1) };
    let mut filter = *b"speed>slow\0";
    let mut err = 0;
    subsuite[0] = alloc_fake_suite(test, b"suite\0".as_ptr() as _, DUMMY_ATTR_TEST_CASES.as_mut_ptr());
    /* Want: suite(slow, normal), NULL -> suite(slow with SKIP, normal), NULL */
    let got = kunit_filter_suites(&suite_set, core::ptr::null(), filter.as_mut_ptr() as _, b"skip\0".as_ptr() as _, &mut err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, got.start);
    KUNIT_ASSERT_EQ!(test, err, 0);
    free_suite_set_at_end(test, &got as *const _ as _);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, (*got.start).test_cases);
    KUNIT_ASSERT_EQ!(test, kunit_suite_num_test_cases(*got.start), 2);
    KUNIT_EXPECT_STREQ!(test, (*got.start).test_cases.as_ref().unwrap().name, b"slow\0".as_ptr() as _);
    KUNIT_EXPECT_STREQ!(test, (*got.start).test_cases.add(1).as_ref().unwrap().name, b"normal\0".as_ptr() as _);
    KUNIT_EXPECT_EQ!(test, (*got.start).test_cases.as_ref().unwrap().status, KUNIT_SKIPPED);
    KUNIT_EXPECT_FALSE!(test, (*got.start).test_cases.add(1).as_ref().unwrap().status);
}

static mut EXECUTOR_TEST_CASES: [kunit_case; 8] = [
    KUNIT_CASE!(parse_filter_test),
    KUNIT_CASE!(filter_suites_test),
    KUNIT_CASE!(filter_suites_test_glob_test),
    KUNIT_CASE!(filter_suites_to_empty_test),
    KUNIT_CASE!(parse_filter_attr_test),
    KUNIT_CASE!(filter_attr_test),
    KUNIT_CASE!(filter_attr_empty_test),
    KUNIT_CASE!(filter_attr_skip_test),
];

static mut EXECUTOR_TEST_SUITE: kunit_suite = kunit_suite {
    name: b"kunit_executor_test\0".as_ptr() as _,
    test_cases: EXECUTOR_TEST_CASES.as_mut_ptr(),
    ..kunit_suite::default()
};

kunit_test_suites!(EXECUTOR_TEST_SUITE);

/* Test helpers */

unsafe fn free_suite_set(suite_set: *mut core::ffi::c_void) {
    kunit_free_suite_set(*(suite_set as *mut kunit_suite_set));
    kfree(suite_set);
}

/* Use the resource API to register a call to free_suite_set.
 * Since we never actually use the resource, it's safe to use on const data.
 */
unsafe fn free_suite_set_at_end(test: *mut kunit, to_free: *const core::ffi::c_void) {
    if !(*(to_free as *const kunit_suite_set)).start.is_null() {
        let free = kzalloc_obj::<kunit_suite_set>();
        *free = *(to_free as *const kunit_suite_set);
        kunit_add_action(test, free_suite_set, free as *mut _ as _);
    }
}

unsafe fn alloc_fake_suite(test: *mut kunit, suite_name: *const core::ffi::c_char, test_cases: *mut kunit_case) -> *mut kunit_suite {
    // We normally never expect to allocate suites, hence the non-const cast.
    let suite = kunit_kzalloc(test, core::mem::size_of::<kunit_suite>(), GFP_KERNEL) as *mut kunit_suite;
    strscpy((*suite).name as *mut _, suite_name, core::mem::size_of_val(&(*suite).name));
    (*suite).test_cases = test_cases;
    suite
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

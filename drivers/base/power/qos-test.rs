// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP
 */
// Dependencies supplied by the kernel KUnit and PM QoS headers.

/* Basic test for aggregating two "min" requests */
unsafe fn freq_qos_test_min(test: *mut kunit) {
    let mut qos: freq_constraints = core::mem::zeroed();
    let mut req1: freq_qos_request = core::mem::zeroed();
    let mut req2: freq_qos_request = core::mem::zeroed();
    let mut ret: i32;

    freq_constraints_init(&mut qos);
    core::ptr::write_bytes(&mut req1, 0, 1);
    core::ptr::write_bytes(&mut req2, 0, 1);

    ret = freq_qos_add_request(&mut qos, &mut req1, FREQ_QOS_MIN, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    ret = freq_qos_add_request(&mut qos, &mut req2, FREQ_QOS_MIN, 2000);
    KUNIT_EXPECT_EQ(test, ret, 1);

    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 2000);

    ret = freq_qos_remove_request(&mut req2);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 1000);

    ret = freq_qos_remove_request(&mut req1);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(
        test,
        freq_qos_read_value(&qos, FREQ_QOS_MIN),
        FREQ_QOS_MIN_DEFAULT_VALUE,
    );
}

/* Test that requests for MAX_DEFAULT_VALUE have no effect */
unsafe fn freq_qos_test_maxdef(test: *mut kunit) {
    let mut qos: freq_constraints = core::mem::zeroed();
    let mut req1: freq_qos_request = core::mem::zeroed();
    let mut req2: freq_qos_request = core::mem::zeroed();
    let mut ret: i32;

    freq_constraints_init(&mut qos);
    core::ptr::write_bytes(&mut req1, 0, 1);
    core::ptr::write_bytes(&mut req2, 0, 1);
    KUNIT_EXPECT_EQ(
        test,
        freq_qos_read_value(&qos, FREQ_QOS_MAX),
        FREQ_QOS_MAX_DEFAULT_VALUE,
    );

    ret = freq_qos_add_request(&mut qos, &mut req1, FREQ_QOS_MAX, FREQ_QOS_MAX_DEFAULT_VALUE);
    KUNIT_EXPECT_EQ(test, ret, 0);
    ret = freq_qos_add_request(&mut qos, &mut req2, FREQ_QOS_MAX, FREQ_QOS_MAX_DEFAULT_VALUE);
    KUNIT_EXPECT_EQ(test, ret, 0);

    /* Add max 1000 */
    ret = freq_qos_update_request(&mut req1, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 1000);

    /* Add max 2000, no impact */
    ret = freq_qos_update_request(&mut req2, 2000);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 1000);

    /* Remove max 1000, new max 2000 */
    ret = freq_qos_remove_request(&mut req1);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 2000);
}

/*
 * Test that a freq_qos_request can be added again after removal
 *
 * This issue was solved by commit 05ff1ba412fd ("PM: QoS: Invalidate frequency
 * QoS requests after removal")
 */
unsafe fn freq_qos_test_readd(test: *mut kunit) {
    let mut qos: freq_constraints = core::mem::zeroed();
    let mut req: freq_qos_request = core::mem::zeroed();
    let mut ret: i32;

    freq_constraints_init(&mut qos);
    core::ptr::write_bytes(&mut req, 0, 1);
    KUNIT_EXPECT_EQ(
        test,
        freq_qos_read_value(&qos, FREQ_QOS_MIN),
        FREQ_QOS_MIN_DEFAULT_VALUE,
    );

    /* Add */
    ret = freq_qos_add_request(&mut qos, &mut req, FREQ_QOS_MIN, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 1000);

    /* Remove */
    ret = freq_qos_remove_request(&mut req);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(
        test,
        freq_qos_read_value(&qos, FREQ_QOS_MIN),
        FREQ_QOS_MIN_DEFAULT_VALUE,
    );

    /* Add again */
    ret = freq_qos_add_request(&mut qos, &mut req, FREQ_QOS_MIN, 2000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 2000);
}

static mut pm_qos_test_cases: [kunit_case; 4] = [
    KUNIT_CASE!(freq_qos_test_min),
    KUNIT_CASE!(freq_qos_test_maxdef),
    KUNIT_CASE!(freq_qos_test_readd),
    KUNIT_CASE_EMPTY!(),
];

static mut pm_qos_test_module: kunit_suite = kunit_suite {
    name: "qos-kunit-test",
    test_cases: pm_qos_test_cases.as_ptr(),
};

// Equivalent of kunit_test_suites(&pm_qos_test_module);
kunit_test_suites!(&pm_qos_test_module);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

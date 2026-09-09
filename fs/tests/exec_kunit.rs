// SPDX-License-Identifier: GPL-2.0-only
// Dependency declarations supplied by the surrounding kernel translation.

#[repr(C)]
struct bprm_stack_limits_result {
    bprm: linux_binprm,
    expected_rc: i32,
    expected_argmin: usize,
}

static BPRM_STACK_LIMITS_RESULTS: &[bprm_stack_limits_result] = &[
    /* Negative argc/envc counts produce -E2BIG */
    bprm_result!(ULONG_MAX, ULONG_MAX, i32::MIN, i32::MIN, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ULONG_MAX, 5, -1, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ULONG_MAX, -1, 10, -E2BIG, 0),
    /* The max value of argc or envc is MAX_ARG_STRINGS. */
    bprm_result!(ULONG_MAX, ULONG_MAX, i32::MAX, i32::MAX, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ULONG_MAX, MAX_ARG_STRINGS, MAX_ARG_STRINGS, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ULONG_MAX, 0, MAX_ARG_STRINGS, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ULONG_MAX, MAX_ARG_STRINGS, 0, -E2BIG, 0),
    /*
     * On 32-bit system these argc and envc counts, while likely impossible
     * to represent within the associated TASK_SIZE, could overflow the
     * limit calculation, and bypass the ptr_size <= limit check.
     */
    bprm_result!(ULONG_MAX, ULONG_MAX, 0x20000001, 0x20000001, -E2BIG, 0),
    /* Make sure a pathological bprm->p doesn't cause an overflow. */
    bprm_result!(core::mem::size_of::<*const core::ffi::c_void>(), ULONG_MAX, 10, 10, -E2BIG, 0),
    /*
     * 0 rlim_stack will get raised to ARG_MAX. With 1 string pointer,
     * we should see p - ARG_MAX + sizeof(void *).
     */
    bprm_result!(ULONG_MAX, 0, 1, 0, 0, ULONG_MAX - ARG_MAX + core::mem::size_of::<*const core::ffi::c_void>()),
    /* Validate that argc is always raised to a minimum of 1. */
    bprm_result!(ULONG_MAX, 0, 0, 0, 0, ULONG_MAX - ARG_MAX + core::mem::size_of::<*const core::ffi::c_void>()),
    bprm_result!(ULONG_MAX, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>(), 0, -E2BIG, 0),
    bprm_result!(ULONG_MAX, 0, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 1, -E2BIG, 0),
    bprm_result!(ULONG_MAX, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() + 1, 0, -E2BIG, 0),
    bprm_result!(ULONG_MAX, 0, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>(), -E2BIG, 0),
    bprm_result!(ULONG_MAX, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 1, 0, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    bprm_result!(ULONG_MAX, 0, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 2, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    /* If we raise rlim_stack / 4 to exactly ARG_MAX, nothing changes. */
    bprm_result!(ULONG_MAX, ARG_MAX * 4, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>(), 0, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ARG_MAX * 4, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 1, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ARG_MAX * 4, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() + 1, 0, -E2BIG, 0),
    bprm_result!(ULONG_MAX, ARG_MAX * 4, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>(), -E2BIG, 0),
    bprm_result!(ULONG_MAX, ARG_MAX * 4, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 1, 0, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    bprm_result!(ULONG_MAX, ARG_MAX * 4, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 2, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    /* But raising it another pointer * 4 will provide space for 1 more pointer. */
    bprm_result!(ULONG_MAX, (ARG_MAX + core::mem::size_of::<*const core::ffi::c_void>()) * 4, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>(), 0, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    bprm_result!(ULONG_MAX, (ARG_MAX + core::mem::size_of::<*const core::ffi::c_void>()) * 4, 0, ARG_MAX / core::mem::size_of::<*const core::ffi::c_void>() - 1, 0, ULONG_MAX - core::mem::size_of::<*const core::ffi::c_void>()),
    /* Raising rlim_stack / 4 to _STK_LIM / 4 * 3 will see more space. */
    bprm_result!(ULONG_MAX, 4 * (_STK_LIM / 4 * 3), 0, 0, 0, ULONG_MAX - (_STK_LIM / 4 * 3) + core::mem::size_of::<*const core::ffi::c_void>()),
    /* But raising it any further will see no increase. */
    bprm_result!(ULONG_MAX, 4 * (_STK_LIM / 4 * 3 + core::mem::size_of::<*const core::ffi::c_void>()), 0, 0, 0, ULONG_MAX - (_STK_LIM / 4 * 3) + core::mem::size_of::<*const core::ffi::c_void>()),
    bprm_result!(ULONG_MAX, 4 * _STK_LIM, 0, 0, 0, ULONG_MAX - (_STK_LIM / 4 * 3) + core::mem::size_of::<*const core::ffi::c_void>()),
];

unsafe fn exec_test_bprm_stack_limits(test: *mut kunit) {
    KUNIT_EXPECT_EQ!(test, _STK_LIM, SZ_8M);
    KUNIT_EXPECT_EQ!(test, ARG_MAX, 32 * SZ_4K);
    KUNIT_EXPECT_EQ!(test, MAX_ARG_STRINGS, 0x7FFFFFFF);

    for (i, result) in BPRM_STACK_LIMITS_RESULTS.iter().enumerate() {
        let mut bprm = result.bprm;
        let rc = bprm_stack_limits(&mut bprm);
        KUNIT_EXPECT_EQ_MSG!(test, rc, result.expected_rc, "on loop {}", i);
        KUNIT_EXPECT_EQ_MSG!(test, bprm.argmin, result.expected_argmin, "on loop {}", i);
    }
}

static mut EXEC_TEST_CASES: [kunit_case; 2] = [KUNIT_CASE!(exec_test_bprm_stack_limits), kunit_case::EMPTY];

static mut EXEC_TEST_SUITE: kunit_suite = kunit_suite {
    name: "exec",
    test_cases: EXEC_TEST_CASES.as_ptr(),
};

kunit_test_suite!(EXEC_TEST_SUITE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

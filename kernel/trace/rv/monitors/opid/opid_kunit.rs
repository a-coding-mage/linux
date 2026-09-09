// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel and the surrounding monitor/test code
// are intentionally not reimplemented here.

// The C source enables this implementation only when
// IS_REACHABLE(CONFIG_RV_MON_OPID) is true. Preserve that build-time intent
// for the eventual kernel configuration.

unsafe fn rv_test_opid(test: *mut kunit) {
    let ctx = (*test).priv_;

    prepare_test(test, &rv_opid_ops.mon);

    /* Ensure we keep the same per-cpu monitor */
    guard_migrate();
    KUNIT_EXPECT_TRUE(test, preemptible());

    /* Wakeup with preemption and interrupts enabled */
    RV_KUNIT_EXPECT_REACTION_HERE(test, ctx);
    (rv_opid_ops.handle_sched_waking)(core::ptr::null_mut(), core::ptr::null_mut());

    /* Need resched with interrupts enabled */
    RV_KUNIT_EXPECT_REACTION_HERE_BLOCK!(test, ctx, {
        scoped_guard_preempt!();
        (rv_opid_ops.handle_sched_need_resched)(
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            TIF_NEED_RESCHED,
        );
    });
}

// When CONFIG_RV_MON_OPID is not reachable, the C source aliases rv_test_opid
// to rv_test_stub.
// #define rv_test_opid rv_test_stub

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

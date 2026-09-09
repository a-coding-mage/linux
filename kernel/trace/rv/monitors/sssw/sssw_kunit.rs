// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel, RV, KUnit, trace events, and
// sssw_kunit.h are intentionally left external to this translation.

// Equivalent of: #if IS_REACHABLE(CONFIG_RV_MON_SSSW)
#[cfg(feature = "CONFIG_RV_MON_SSSW")]
unsafe fn rv_test_sssw(test: *mut kunit) {
    let target: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let other: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut rv_kunit_ctx = (*test).r#priv;

    prepare_test(test, &mut rv_sssw_ops.mon);

    /* Suspend without setting to sleepable */
    rv_sssw_ops.handle_sched_set_state(core::ptr::null_mut(), target, TASK_RUNNING);
    RV_KUNIT_EXPECT_REACTION_HERE!(test, ctx);
    rv_sssw_ops.handle_sched_switch(
        core::ptr::null_mut(),
        0,
        target,
        other,
        TASK_INTERRUPTIBLE,
    );

    /* Switch in after suspension without wakeup */
    rv_sssw_ops.handle_sched_wakeup(core::ptr::null_mut(), target);
    rv_sssw_ops.handle_sched_set_state(core::ptr::null_mut(), target, TASK_INTERRUPTIBLE);
    rv_sssw_ops.handle_sched_switch(
        core::ptr::null_mut(),
        0,
        target,
        other,
        TASK_INTERRUPTIBLE,
    );
    RV_KUNIT_EXPECT_REACTION_HERE!(test, ctx);
    rv_sssw_ops.handle_sched_switch(
        core::ptr::null_mut(),
        0,
        other,
        target,
        TASK_RUNNING,
    );
}

// Equivalent of: #else
// #define rv_test_sssw rv_test_stub
#[cfg(not(feature = "CONFIG_RV_MON_SSSW"))]
use rv_test_stub as rv_test_sssw;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

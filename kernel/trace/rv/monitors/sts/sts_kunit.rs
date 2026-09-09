// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel, RV KUnit support, scheduler
// tracepoints, and sts_kunit.h are intentionally left external.

// This test is compiled only when CONFIG_RV_MON_STS is reachable.
#[cfg(/* IS_REACHABLE(CONFIG_RV_MON_STS) */ rv_mon_sts_reachable)]
unsafe fn rv_test_sts(test: *mut kunit) {
    let target: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let other: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut rv_kunit_ctx = (*test).priv_;

    prepare_test!(test, rv_sts_ops.mon);
    /* Per-CPU monitor, make sure we don't change CPU mid-test */
    guard!(migrate);

    /* Switch without disabling interrupts */
    rv_sts_ops.handle_schedule_exit(core::ptr::null_mut(), false);
    rv_sts_ops.handle_schedule_entry(core::ptr::null_mut(), false);
    rv_kunit_expect_reaction_here!(test, ctx, {
        rv_sts_ops.handle_sched_switch(
            core::ptr::null_mut(),
            0,
            target,
            other,
            TASK_RUNNING,
        );
    });

    rv_sts_ops.handle_schedule_exit(core::ptr::null_mut(), false);

    /* Schedule from interrupt context */
    rv_sts_ops.handle_schedule_entry(core::ptr::null_mut(), false);
    rv_sts_ops.handle_irq_disable(core::ptr::null_mut(), 0, 0);
    rv_sts_ops.handle_irq_entry(core::ptr::null_mut(), 0, core::ptr::null_mut());
    rv_kunit_expect_reaction_here!(test, ctx, {
        rv_sts_ops.handle_sched_switch(
            core::ptr::null_mut(),
            0,
            target,
            other,
            TASK_RUNNING,
        );
    });
    rv_sts_ops.handle_irq_enable(core::ptr::null_mut(), 0, 0);
}

// When CONFIG_RV_MON_STS is not reachable, the C source aliases rv_test_sts
// to rv_test_stub.
#[cfg(not(/* IS_REACHABLE(CONFIG_RV_MON_STS) */ rv_mon_sts_reachable))]
use rv_test_stub as rv_test_sts;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

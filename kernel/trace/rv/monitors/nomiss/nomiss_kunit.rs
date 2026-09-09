// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel and the surrounding repository
// are intentionally referenced by name rather than implemented here.

#[cfg(CONFIG_RV_MON_NOMISS)]
unsafe fn rv_test_nomiss(test: *mut kunit) {
    let target: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let other: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut rv_kunit_ctx = (*test).priv_;

    prepare_test(test, &rv_nomiss_ops.mon);

    (*target).pid = 99;
    (*target).policy = SCHED_DEADLINE;
    (*target).dl.runtime = 10000;
    (*target).dl.dl_deadline = 20000;

    (rv_nomiss_ops.handle_newtask)(core::ptr::null_mut(), target, 0);

    /* Task gets preempted and can't terminate before deadline */
    (rv_nomiss_ops.handle_sched_switch)(
        core::ptr::null_mut(),
        0,
        other,
        target,
        TASK_RUNNING,
    );
    (rv_nomiss_ops.handle_dl_replenish)(
        core::ptr::null_mut(),
        &mut (*target).dl,
        0,
        DL_TASK,
    );
    udelay(10);
    (rv_nomiss_ops.handle_sched_switch)(
        core::ptr::null_mut(),
        0,
        target,
        other,
        TASK_RUNNING,
    );
    rv_kunit_expect_reaction_here!(test, ctx, {
        udelay(15 + div_u64(*rv_nomiss_ops.deadline_thresh, 1000));
        (rv_nomiss_ops.handle_sched_switch)(
            core::ptr::null_mut(),
            0,
            other,
            target,
            TASK_RUNNING,
        );
    });
}

// When CONFIG_RV_MON_NOMISS is not reachable:
// #define rv_test_nomiss rv_test_stub
#[cfg(not(CONFIG_RV_MON_NOMISS))]
use rv_test_stub as rv_test_nomiss;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

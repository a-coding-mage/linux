// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and related headers:
// linux/kernel.h, linux/rv.h, rv/kunit.h, linux/sched/deadline.h,
// linux/sched/rt.h, and pagefault_kunit.h.

// CONFIG_RV_MON_PAGEFAULT reachability is a build-time condition.  Preserve
// both branches here; the surrounding build supplies the configuration.
#[cfg(/* IS_REACHABLE(CONFIG_RV_MON_PAGEFAULT) */ any())]
unsafe fn rv_test_pagefault(test: *mut kunit) {
    let target: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut rv_kunit_ctx = (*test).r#priv as *mut rv_kunit_ctx;

    prepare_test(test, &rv_pagefault_ops.mon);

    /* Initial pagefault when non-RT to start the model without failure */
    (*target).policy = SCHED_NORMAL;
    (*target).prio = MAX_RT_PRIO + 20;
    rv_pagefault_ops.handle_task_newtask(core::ptr::null_mut(), target, 0);
    rv_mock_current(target);
    rv_pagefault_ops.handle_page_fault(core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);

    /* RT task has a page fault */
    (*target).policy = SCHED_FIFO;
    (*target).prio = MAX_RT_PRIO - 1;
    RV_KUNIT_EXPECT_REACTION_HERE!(test, ctx)
        rv_pagefault_ops.handle_page_fault(core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
}

#[cfg(not(/* IS_REACHABLE(CONFIG_RV_MON_PAGEFAULT) */ any()))]
use rv_test_stub as rv_test_pagefault;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Helper function for testing code in interrupt contexts
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel/KUnit translation.

#[repr(C)]
pub struct kunit_irq_test_state {
    pub func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> bool>,
    pub test_specific_state: *mut core::ffi::c_void,
    pub task_func_reported_failure: bool,
    pub hardirq_func_reported_failure: bool,
    pub softirq_func_reported_failure: bool,
    pub task_func_calls: atomic_t,
    pub hardirq_func_calls: atomic_t,
    pub softirq_func_calls: atomic_t,
    pub interval: ktime_t,
    pub timer: hrtimer,
    pub bh_work: work_struct,
}

unsafe fn kunit_irq_test_timer_func(timer: *mut hrtimer) -> hrtimer_restart {
    let state = container_of!(timer, kunit_irq_test_state, timer);
    let mut task_calls: i32;
    let mut hardirq_calls: i32;
    let mut softirq_calls: i32;

    WARN_ON_ONCE!(!in_hardirq());
    task_calls = atomic_read!(&(*state).task_func_calls);
    hardirq_calls = atomic_inc_return!(&mut (*state).hardirq_func_calls);
    softirq_calls = atomic_read!(&(*state).softirq_func_calls);

    /*
     * If the hrtimer is running much faster than the bh_work or the task,
     * then it is firing too fast and might be starving those contexts as
     * well as the actual system timer tick.  Increase the interval.
     */
    if hardirq_calls >= 20
        && (hardirq_calls / 2 > softirq_calls || hardirq_calls / 2 > task_calls)
    {
        (*state).interval = ktime_add_ns((*state).interval, 250);
    }

    if !((*state).func.expect("func")( (*state).test_specific_state)) {
        (*state).hardirq_func_reported_failure = true;
    }

    hrtimer_forward_now(&mut (*state).timer, (*state).interval);
    queue_work(system_bh_wq, &mut (*state).bh_work);
    HRTIMER_RESTART
}

unsafe fn kunit_irq_test_bh_work_func(work: *mut work_struct) {
    let state = container_of!(work, kunit_irq_test_state, bh_work);

    WARN_ON_ONCE!(!in_serving_softirq());
    atomic_inc!(&mut (*state).softirq_func_calls);

    if !((*state).func.expect("func")( (*state).test_specific_state)) {
        (*state).softirq_func_reported_failure = true;
    }
}

/*
 * Helper function which repeatedly runs the given @func in task, softirq, and
 * hardirq context concurrently, and reports a failure to KUnit if any
 * invocation of @func in any context returns false.  @func is passed
 * @test_specific_state as its argument.  At most 3 invocations of @func will
 * run concurrently: one in each of task, softirq, and hardirq context.  @func
 * will continue running until either @max_iterations calls have been made (so
 * long as at least one each runs in task, softirq, and hardirq contexts), or
 * one second has passed.
 *
 * The main purpose of this interrupt context testing is to validate fallback
 * code paths that run in contexts where the normal code path cannot be used,
 * typically due to the FPU or vector registers already being in-use in kernel
 * mode.  These code paths aren't covered when the test code is executed only
 * by the KUnit test runner thread in task context.  The reason for the
 * concurrency is because merely using hardirq context is not sufficient to
 * reach a fallback code path on some architectures; the hardirq actually has
 * to occur while the FPU or vector unit was already in-use in kernel mode.
 *
 * Another purpose of this testing is to detect issues with the architecture's
 * irq_fpu_usable() and kernel_fpu_begin/end() or equivalent functions,
 * especially in softirq context when the softirq may have interrupted a task
 * already using kernel-mode FPU or vector (if the arch didn't prevent that).
 * Crypto functions are often executed in softirqs, so this is important.
 */
pub unsafe fn kunit_run_irq_test(
    test: *mut kunit,
    func: unsafe extern "C" fn(*mut core::ffi::c_void) -> bool,
    max_iterations: i32,
    test_specific_state: *mut core::ffi::c_void,
) {
    let mut state = kunit_irq_test_state {
        func: Some(func),
        test_specific_state,
        task_func_reported_failure: false,
        hardirq_func_reported_failure: false,
        softirq_func_reported_failure: false,
        task_func_calls: atomic_t::default(),
        hardirq_func_calls: atomic_t::default(),
        softirq_func_calls: atomic_t::default(),
        interval: us_to_ktime(5),
        timer: hrtimer::default(),
        bh_work: work_struct::default(),
    };
    let end_jiffies: usize;
    let mut task_calls: i32;
    let mut hardirq_calls: i32;
    let mut softirq_calls: i32;

    hrtimer_setup_on_stack!(&mut state.timer, kunit_irq_test_timer_func,
                            CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    INIT_WORK_ONSTACK!(&mut state.bh_work, kunit_irq_test_bh_work_func);

    end_jiffies = jiffies + HZ;
    hrtimer_start!(&mut state.timer, state.interval, HRTIMER_MODE_REL_HARD);
    loop {
        if !func(test_specific_state) {
            state.task_func_reported_failure = true;
        }

        task_calls = atomic_inc_return!(&mut state.task_func_calls);
        hardirq_calls = atomic_read!(&state.hardirq_func_calls);
        softirq_calls = atomic_read!(&state.softirq_func_calls);
        if !((task_calls + hardirq_calls + softirq_calls < max_iterations
            || task_calls == 0 || hardirq_calls == 0 || softirq_calls == 0)
            && !time_after!(jiffies, end_jiffies))
        {
            break;
        }
    }

    hrtimer_cancel(&mut state.timer);
    flush_work(&mut state.bh_work);
    destroy_hrtimer_on_stack(&mut state.timer);
    destroy_work_on_stack(&mut state.bh_work);

    KUNIT_EXPECT_GT_MSG!(test, atomic_read!(&state.hardirq_func_calls), 0,
                         "Timer function was not called");
    KUNIT_EXPECT_GT_MSG!(test, atomic_read!(&state.softirq_func_calls), 0,
                         "BH work function was not called");
    KUNIT_EXPECT_FALSE_MSG!(test, state.task_func_reported_failure,
                            "Failure reported from task context");
    KUNIT_EXPECT_FALSE_MSG!(test, state.hardirq_func_reported_failure,
                            "Failure reported from hardirq context");
    KUNIT_EXPECT_FALSE_MSG!(test, state.softirq_func_reported_failure,
                            "Failure reported from softirq context");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

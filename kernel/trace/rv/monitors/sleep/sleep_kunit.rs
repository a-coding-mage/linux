// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel/RV implementation are intentionally omitted.

// This block is reachable only when CONFIG_RV_MON_SLEEP is enabled and reachable.
#[cfg(any(feature = "CONFIG_RV_MON_SLEEP", rv_mon_sleep))]
unsafe fn rv_test_sleep(test: *mut Kunit) {
    let target: *mut TaskStruct = rv_kunit_alloc_mock_task(test);
    let other: *mut TaskStruct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut RvKunitCtx = (*test).priv_;
    let mut args: [c_ulong; 6] = [0; 6];
    let mut regs: PtRegs = core::mem::zeroed();

    prepare_test(test, &mut rv_sleep_ops.mon);
    (*target).policy = SCHED_FIFO;
    (*target).prio = MAX_RT_PRIO - 2;
    (*other).policy = SCHED_FIFO;
    (*other).prio = MAX_RT_PRIO - 1;
    rv_sleep_ops.handle_task_newtask(core::ptr::null_mut(), target, 0);

    /* RT task sleeps on a non RT-friendly nanosleep */
    rv_mock_current(target);
    args[0] = CLOCK_REALTIME;
    syscall_set_arguments(target, &mut regs, args.as_mut_ptr());
    // #ifdef __NR_clock_nanosleep
    rv_sleep_ops.handle_sys_enter(core::ptr::null_mut(), &mut regs, __NR_clock_nanosleep);
    // #elif defined(__NR_clock_nanosleep_time64)
    // rv_sleep_ops.handle_sys_enter(core::ptr::null_mut(), &mut regs, __NR_clock_nanosleep_time64);
    // #endif
    rv_kunit_expect_reaction_here(test, ctx, || {
        rv_sleep_ops.handle_sched_set_state(core::ptr::null_mut(), target, TASK_INTERRUPTIBLE)
    });
    rv_sleep_ops.handle_sys_exit(core::ptr::null_mut(), core::ptr::null_mut(), 0);

    /* RT task woken up by lower priority task */
    args[1] = FUTEX_WAIT;
    syscall_set_arguments(target, &mut regs, args.as_mut_ptr());
    rv_mock_current(target);
    // #ifdef __NR_futex
    rv_sleep_ops.handle_sys_enter(core::ptr::null_mut(), &mut regs, __NR_futex);
    // #elif defined(__NR_futex_time64)
    // rv_sleep_ops.handle_sys_enter(core::ptr::null_mut(), &mut regs, __NR_futex_time64);
    // #endif
    rv_sleep_ops.handle_sched_set_state(core::ptr::null_mut(), target, TASK_INTERRUPTIBLE);
    rv_mock_current(other);
    rv_sleep_ops.handle_sched_waking(core::ptr::null_mut(), target);
    rv_mock_current(target);
    rv_kunit_expect_reaction_here(test, ctx, || {
        rv_sleep_ops.handle_sched_exit(core::ptr::null_mut(), true)
    });
}

// When CONFIG_RV_MON_SLEEP is not reachable, the C macro aliases this test to rv_test_stub.
#[cfg(not(any(feature = "CONFIG_RV_MON_SLEEP", rv_mon_sleep)))]
unsafe fn rv_test_sleep(test: *mut Kunit) {
    rv_test_stub(test);
}

// External kernel/RV declarations used by the translated implementation.
use core::ffi::c_ulong;
extern "C" {
    static mut rv_sleep_ops: RvSleepOps;
    fn rv_kunit_alloc_mock_task(test: *mut Kunit) -> *mut TaskStruct;
    fn prepare_test(test: *mut Kunit, ops: *mut RvMonitorOps);
    fn rv_mock_current(task: *mut TaskStruct);
    fn syscall_set_arguments(task: *mut TaskStruct, regs: *mut PtRegs, args: *mut c_ulong);
    fn rv_kunit_expect_reaction_here<F: FnOnce()>(test: *mut Kunit, ctx: *mut RvKunitCtx, action: F);
    fn rv_test_stub(test: *mut Kunit);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

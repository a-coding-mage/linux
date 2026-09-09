// SPDX-License-Identifier: GPL-2.0
//
// Translated from the C implementation. The Linux kernel and KUnit symbols
// referenced below are supplied by external dependencies.

#[cfg(feature = "rv_mon_sco")]
unsafe fn rv_test_sco(test: *mut kunit) {
    let target: *mut task_struct = rv_kunit_alloc_mock_task(test);
    let ctx: *mut rv_kunit_ctx = (*test).priv_;

    prepare_test(test, &mut rv_sco_ops.mon);

    /* Ensure we keep the same per-cpu monitor */
    guard_migrate();

    /* Set state while scheduling */
    rv_sco_ops.handle_sched_set_state(
        core::ptr::null_mut(),
        target,
        TASK_INTERRUPTIBLE,
    );
    rv_sco_ops.handle_schedule_entry(core::ptr::null_mut(), false);
    rv_kunit_expect_reaction_here!(test, ctx);
    rv_sco_ops.handle_sched_set_state(
        core::ptr::null_mut(),
        target,
        TASK_INTERRUPTIBLE,
    );
}

#[cfg(not(feature = "rv_mon_sco"))]
use rv_test_stub as rv_test_sco;

// External kernel/KUnit declarations used by this translation.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[allow(non_camel_case_types)]
pub struct task_struct;

#[allow(non_camel_case_types)]
pub struct rv_kunit_ctx;

extern "C" {
    static mut rv_sco_ops: rv_sco_ops_t;
    fn rv_kunit_alloc_mock_task(test: *mut kunit) -> *mut task_struct;
    fn prepare_test(test: *mut kunit, monitor: *mut core::ffi::c_void);
    fn guard_migrate();
    fn rv_test_stub(test: *mut kunit);
    fn rv_kunit_expect_reaction_here(test: *mut kunit, ctx: *mut rv_kunit_ctx);
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rv_sco_ops_t {
    pub mon: core::ffi::c_void,
    pub handle_sched_set_state: unsafe extern "C" fn(
        prev: *mut task_struct,
        next: *mut task_struct,
        state: i64,
    ),
    pub handle_schedule_entry: unsafe extern "C" fn(
        task: *mut task_struct,
        preempt: bool,
    ),
}

pub const TASK_INTERRUPTIBLE: i64 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

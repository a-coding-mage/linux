// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026-2029 Red Hat, Inc. Gabriele Monaco <gmonaco@redhat.com>
 *
 * RV monitor kunit tests:
 *   Tests the RV monitors by triggering fake events to verify monitor
 *   behavior and reactions. Tests start from the first defined event and
 *   trigger events in order to verify error detection.
 */

// External dependencies supplied by the kernel, KUnit, and RV monitor sources.

static mut ACTIVE_CTX: *mut rv_kunit_ctx = core::ptr::null_mut();

unsafe extern "C" {
    static CONFIG_THREAD_INFO_IN_TASK: bool;
}

#[repr(C)]
pub struct rv_kunit_ctx {
    pub reactions: i32,
    pub mock_task_count: i32,
    pub mock_tasks: [*mut task_struct; RV_KUNIT_MAX_MOCK_TASKS],
}

#[repr(C)]
pub struct rv_kunit_mon {
    pub is_per_task: bool,
    pub task_reset: Option<unsafe extern "C" fn(*mut task_struct)>,
    pub rv_this: *mut rv_monitor,
    pub task_slot: *mut i32,
    pub monitor_init: unsafe extern "C" fn() -> i32,
    pub monitor_destroy: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const u8,
    pub react: Option<unsafe extern "C" fn(*mut rv_monitor, *const u8)>,
    pub reactor: *mut rv_monitor,
    pub enabled: i32,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct task_struct {
    pub stack: *mut thread_info,
}

#[repr(C)]
pub struct thread_info {
    _private: [u8; 0],
}

pub const RV_KUNIT_MAX_MOCK_TASKS: usize = 16;
pub const RV_PER_TASK_MONITOR_INIT: i32 = 0;

unsafe extern "C" {
    fn kunit_get_current_test() -> *mut kunit;
    fn synchronize_rcu();
    fn rv_mock_current(task: *mut task_struct);
    fn rv_react(monitor: *mut rv_monitor, msg: *const u8);
    fn rv_set_testing() -> i32;
    fn rv_clear_testing();
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kunit_skip(test: *mut kunit, msg: *const u8);
    fn rv_kunit_mock_react(msg: *const u8, args: *mut core::ffi::c_void);
}

unsafe fn rv_kunit_mock_react_local(_msg: *const u8, _args: *mut core::ffi::c_void) {
    if !ACTIVE_CTX.is_null() {
        (*ACTIVE_CTX).reactions += 1;
    }
}

/*
 * teardown_test - Disable the monitor for a kunit test
 *
 * Since per-task monitors are special, make sure we reset all the ones we
 * started manually here, if required.
 */
pub unsafe fn teardown_test(arg: *mut core::ffi::c_void) {
    let mon = arg as *mut rv_kunit_mon;
    let test = kunit_get_current_test();

    if !test.is_null() {
        let ctx = (*test).priv_ as *mut rv_kunit_ctx;

        // RV_KUNIT_EXPECT_NO_REACTION(test, ctx)
        if (*mon).is_per_task && (*mon).task_reset.is_some() {
            for i in 0..(*ctx).mock_task_count {
                ((*mon).task_reset.unwrap())((*ctx).mock_tasks[i as usize]);
            }
            synchronize_rcu();
        }
    }

    (*(*mon).rv_this).enabled = 0;

    if !(*(*mon).rv_this).reactor.is_null() {
        (*(*mon).rv_this).react = (*(*(*mon).rv_this).reactor).react;
    } else {
        (*(*mon).rv_this).react = None;
    }
    ACTIVE_CTX = core::ptr::null_mut();
    rv_mock_current(core::ptr::null_mut());

    if (*mon).is_per_task {
        *(*mon).task_slot = RV_PER_TASK_MONITOR_INIT;
    } else {
        ((*mon).monitor_destroy)();
    }
}

/*
 * prepare_test - Enable the monitor for a kunit test
 *
 * Do the bare minimum to set up the monitor, per-task monitors are special as
 * "real" initialisation/destruction iterates over real tasks, and may register
 * handlers. All we need is to select the right slot in the task_struct.
 */
pub unsafe fn prepare_test(test: *mut kunit, mon: *const rv_kunit_mon) {
    // KUNIT_ASSERT_FALSE(test, (*mon).rv_this->enabled)

    ACTIVE_CTX = (*test).priv_ as *mut rv_kunit_ctx;
    (*(*mon).rv_this).react = Some(rv_kunit_mock_react_local);

    if (*mon).is_per_task {
        *(*mon).task_slot = 0;
    } else {
        // KUNIT_ASSERT_EQ(test, (*mon).monitor_init(), 0)
        ((*mon).monitor_init)();
    }

    (*(*mon).rv_this).enabled = 1;

    // KUNIT_ASSERT_EQ(test, 0, kunit_add_action_or_reset(test, teardown_test, mon))
}

pub unsafe fn rv_kunit_alloc_mock_task(test: *mut kunit) -> *mut task_struct {
    let ctx = (*test).priv_ as *mut rv_kunit_ctx;
    let tsk: *mut task_struct;

    // KUNIT_ASSERT_LT(test, (*ctx).mock_task_count, RV_KUNIT_MAX_MOCK_TASKS)
    tsk = kunit_kzalloc(test, core::mem::size_of::<task_struct>(), 0) as *mut task_struct;
    // KUNIT_ASSERT_NOT_NULL(test, tsk)

    if !CONFIG_THREAD_INFO_IN_TASK {
        (*tsk).stack = kunit_kzalloc(test, core::mem::size_of::<thread_info>(), 0) as *mut thread_info;
        // KUNIT_ASSERT_NOT_NULL(test, (*tsk).stack)
    }

    (*ctx).mock_tasks[(*ctx).mock_task_count as usize] = tsk;
    (*ctx).mock_task_count += 1;
    tsk
}

unsafe fn rv_mon_test_init(test: *mut kunit) -> i32 {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<rv_kunit_ctx>(), 0) as *mut rv_kunit_ctx;
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx)
    (*test).priv_ = ctx as *mut core::ffi::c_void;
    0
}

unsafe fn rv_test_stub(test: *mut kunit) {
    kunit_skip(test, b"Monitor not enabled\0".as_ptr());
}

/*
 * rv_test_dummy - test reactions work as expected
 */
unsafe fn rv_test_dummy(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut rv_kunit_ctx;
    static mut DUMMY_MONITOR: rv_monitor = rv_monitor {
        name: b"dummy\0".as_ptr(),
        react: Some(rv_kunit_mock_react_local),
        reactor: core::ptr::null_mut(),
        enabled: 0,
    };

    ACTIVE_CTX = ctx;

    // RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
    rv_react(&raw mut DUMMY_MONITOR, b"dummy\0".as_ptr());
    // RV_KUNIT_EXPECT_NO_REACTION(test, ctx)

    ACTIVE_CTX = core::ptr::null_mut();
}

// The following monitor-specific test sources are included by the C build:
// monitors/sco/sco_kunit.c, monitors/sssw/sssw_kunit.c,
// monitors/sts/sts_kunit.c, monitors/opid/opid_kunit.c,
// monitors/nomiss/nomiss_kunit.c, monitors/pagefault/pagefault_kunit.c,
// monitors/sleep/sleep_kunit.c

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

static mut RV_MON_TEST_CASES: [kunit_case; 9] = [
    kunit_case { run_case: Some(rv_test_dummy) },
    kunit_case { run_case: Some(rv_test_sco) },
    kunit_case { run_case: Some(rv_test_sssw) },
    kunit_case { run_case: Some(rv_test_sts) },
    kunit_case { run_case: Some(rv_test_opid) },
    kunit_case { run_case: Some(rv_test_nomiss) },
    kunit_case { run_case: Some(rv_test_pagefault) },
    kunit_case { run_case: Some(rv_test_sleep) },
    kunit_case { run_case: None },
];

unsafe extern "C" {
    fn rv_test_sco(test: *mut kunit);
    fn rv_test_sssw(test: *mut kunit);
    fn rv_test_sts(test: *mut kunit);
    fn rv_test_opid(test: *mut kunit);
    fn rv_test_nomiss(test: *mut kunit);
    fn rv_test_pagefault(test: *mut kunit);
    fn rv_test_sleep(test: *mut kunit);
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const u8,
    pub suite_init: Option<unsafe extern "C" fn() -> i32>,
    pub suite_exit: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> i32>,
    pub test_cases: *mut kunit_case,
}

#[no_mangle]
pub static mut RV_MON_TEST_SUITE: kunit_suite = kunit_suite {
    name: b"rv_mon\0".as_ptr(),
    suite_init: Some(rv_set_testing),
    suite_exit: Some(rv_clear_testing),
    init: Some(rv_mon_test_init),
    test_cases: core::ptr::null_mut(),
};

// kunit_test_suites(&rv_mon_test_suite)
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>")
// MODULE_DESCRIPTION("RV monitor kunit tests: test monitors by triggering reactions")
// MODULE_LICENSE("GPL")
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

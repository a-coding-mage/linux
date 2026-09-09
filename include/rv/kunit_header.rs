/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026-2029 Red Hat, Inc. Gabriele Monaco <gmonaco@redhat.com>
 *
 * Declaration of wrappers to allow mocking core functionality, like current,
 * and other testing utilities.
 * Necessary only when mocking may be needed. If the RV KUnit test is
 * enabled, the wrappers incur an additional function call overhead.
 */

/* Corresponds to: #if IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST) */

extern "C" {
    pub fn rv_set_testing(suite: *mut kunit_suite) -> ::core::ffi::c_int;
    pub fn rv_clear_testing(suite: *mut kunit_suite);
}

pub const RV_KUNIT_MAX_MOCK_TASKS: usize = 8;

#[repr(C)]
pub struct rv_kunit_ctx {
    pub reactions: ::core::ffi::c_int,
    pub expected: ::core::ffi::c_int,
    pub mock_task_count: ::core::ffi::c_int,
    pub mock_tasks: [*mut task_struct; RV_KUNIT_MAX_MOCK_TASKS],
}

/* Direct Rust equivalents of the C expectation macros. */
#[macro_export]
macro_rules! RV_KUNIT_EXPECT_REACTION {
    ($test:expr, $ctx:expr) => {{
        KUNIT_EXPECT_EQ($test, ($ctx).reactions, ($ctx).expected.wrapping_add(1));
        ($ctx).expected = ($ctx).expected.wrapping_add(1);
        if ($ctx).reactions != ($ctx).expected {
            ($ctx).expected = ($ctx).reactions;
        }
    }};
}

#[macro_export]
macro_rules! RV_KUNIT_EXPECT_NO_REACTION {
    ($test:expr, $ctx:expr) => {{
        KUNIT_EXPECT_EQ($test, ($ctx).reactions, ($ctx).expected);
        if ($ctx).reactions != ($ctx).expected {
            ($ctx).expected = ($ctx).reactions;
        }
    }};
}

#[macro_export]
macro_rules! RV_KUNIT_EXPECT_REACTION_HERE {
    ($test:expr, $ctx:expr) => {{
        $crate::RV_KUNIT_EXPECT_NO_REACTION!($test, $ctx);
        $crate::RV_KUNIT_EXPECT_REACTION!($test, $ctx);
    }};
}

#[repr(C)]
pub struct rv_kunit_mon {
    pub rv_this: *mut rv_monitor,
    pub monitor_init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub monitor_destroy: Option<unsafe extern "C" fn()>,
    pub is_per_task: bool,
    pub task_slot: *mut ::core::ffi::c_int,
    pub task_reset: Option<unsafe extern "C" fn(task: *mut task_struct)>,
}

extern "C" {
    pub fn prepare_test(test: *mut kunit, mon: *const rv_kunit_mon);
    pub fn teardown_test(arg: *mut ::core::ffi::c_void);
    pub fn rv_kunit_alloc_mock_task(test: *mut kunit) -> *mut task_struct;

    pub fn rv_mock_current(tsk: *mut task_struct);
    pub fn rv_get_mock_current() -> *mut task_struct;
}

/* Corresponds to: #define rv_get_current() (unlikely(kunit_get_current_test()) ? rv_get_mock_current() : current) */
#[macro_export]
macro_rules! rv_get_current {
    () => {{
        if unlikely(kunit_get_current_test()) {
            rv_get_mock_current()
        } else {
            current
        }
    }};
}

/* Corresponds to the !CONFIG_RV_MONITORS_KUNIT_TEST branch: #define rv_get_current() current */

/* Corresponds to: #endif CONFIG_RV_MONITORS_KUNIT_TEST and header guard. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

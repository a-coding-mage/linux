/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/*
 * C conditional preserved from:
 * #ifdef CONFIG_RV_MON_TEST_LTL_KUNIT
 */
define_event!(
    event_ltl_monitor_id,
    event_test_ltl_kunit,
    TP_PROTO!(
        task: *mut task_struct,
        states: *mut ::core::ffi::c_char,
        atoms: *mut ::core::ffi::c_char,
        next: *mut ::core::ffi::c_char
    ),
    TP_ARGS!(task, states, atoms, next)
);

define_event!(
    error_ltl_monitor_id,
    error_test_ltl_kunit,
    TP_PROTO!(task: *mut task_struct),
    TP_ARGS!(task)
);
/*
 * #endif /* CONFIG_RV_MON_TEST_LTL_KUNIT */
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

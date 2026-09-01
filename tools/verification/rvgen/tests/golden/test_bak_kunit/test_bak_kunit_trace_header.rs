/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

// C conditional preserved: #ifdef CONFIG_RV_MON_TEST_BAK_KUNIT
#[cfg(CONFIG_RV_MON_TEST_BAK_KUNIT)]
DEFINE_EVENT!(
    event_ltl_monitor_id,
    event_test_bak_kunit,
    TP_PROTO!(struct task_struct *task, char *states, char *atoms, char *next),
    TP_ARGS!(task, states, atoms, next)
);

#[cfg(CONFIG_RV_MON_TEST_BAK_KUNIT)]
DEFINE_EVENT!(
    error_ltl_monitor_id,
    error_test_bak_kunit,
    TP_PROTO!(struct task_struct *task),
    TP_ARGS!(task)
);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

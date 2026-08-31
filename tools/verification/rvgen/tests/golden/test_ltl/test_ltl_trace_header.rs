/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

// Original C condition: #ifdef CONFIG_RV_MON_TEST_LTL
#[cfg(CONFIG_RV_MON_TEST_LTL)]
unsafe extern "C" {
    // DEFINE_EVENT(event_ltl_monitor_id, event_test_ltl,
    //          TP_PROTO(struct task_struct *task, char *states, char *atoms, char *next),
    //          TP_ARGS(task, states, atoms, next));
    pub fn event_test_ltl(
        task: *mut task_struct,
        states: *mut core::ffi::c_char,
        atoms: *mut core::ffi::c_char,
        next: *mut core::ffi::c_char,
    );

    // DEFINE_EVENT(error_ltl_monitor_id, error_test_ltl,
    //          TP_PROTO(struct task_struct *task),
    //          TP_ARGS(task));
    pub fn error_test_ltl(task: *mut task_struct);
}

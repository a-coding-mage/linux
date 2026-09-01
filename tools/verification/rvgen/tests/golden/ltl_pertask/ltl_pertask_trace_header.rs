// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

// C source condition: #ifdef CONFIG_RV_MON_LTL_PERTASK
#[cfg(CONFIG_RV_MON_LTL_PERTASK)]
DEFINE_EVENT!(
    event_ltl_monitor_id,
    event_ltl_pertask,
    TP_PROTO!(
        task: *mut task_struct,
        states: *mut core::ffi::c_char,
        atoms: *mut core::ffi::c_char,
        next: *mut core::ffi::c_char
    ),
    TP_ARGS!(task, states, atoms, next)
);

#[cfg(CONFIG_RV_MON_LTL_PERTASK)]
DEFINE_EVENT!(
    error_ltl_monitor_id,
    error_ltl_pertask,
    TP_PROTO!(task: *mut task_struct),
    TP_ARGS!(task)
);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

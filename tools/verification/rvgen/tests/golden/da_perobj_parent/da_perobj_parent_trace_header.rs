/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

// C preprocessor condition preserved from the source:
// #ifdef CONFIG_RV_MON_DA_PEROBJ_PARENT
DEFINE_EVENT!(
    event_da_monitor_id,
    event_da_perobj_parent,
    TP_PROTO!(
        id: ::std::os::raw::c_int,
        state: *mut ::std::os::raw::c_char,
        event: *mut ::std::os::raw::c_char,
        next_state: *mut ::std::os::raw::c_char,
        final_state: bool
    ),
    TP_ARGS!(id, state, event, next_state, final_state)
);

DEFINE_EVENT!(
    error_da_monitor_id,
    error_da_perobj_parent,
    TP_PROTO!(
        id: ::std::os::raw::c_int,
        state: *mut ::std::os::raw::c_char,
        event: *mut ::std::os::raw::c_char
    ),
    TP_ARGS!(id, state, event)
);
// #endif /* CONFIG_RV_MON_DA_PEROBJ_PARENT */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

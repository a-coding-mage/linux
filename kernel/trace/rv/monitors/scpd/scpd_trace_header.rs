/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* Preserved from the C header: these declarations are enabled only when
 * CONFIG_RV_MON_SCPD is configured.  DEFINE_EVENT, TP_PROTO, and TP_ARGS are
 * supplied by the surrounding trace-event infrastructure.
 */
#[cfg(CONFIG_RV_MON_SCPD)]
define_event!(
    event_da_monitor,
    event_scpd,
    tp_proto!(
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        next_state: *mut ::core::ffi::c_char,
        final_state: bool
    ),
    tp_args!(state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_SCPD)]
define_event!(
    error_da_monitor,
    error_scpd,
    tp_proto!(
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char
    ),
    tp_args!(state, event)
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

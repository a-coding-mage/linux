/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 *
 * The trace-event declaration machinery represented by DEFINE_EVENT, TP_PROTO,
 * and TP_ARGS is supplied by the surrounding tracing header.  It has no
 * file-local Rust definition to translate here.
 */

#[cfg(feature = "CONFIG_RV_MON_WWNR")]
mod wwnr_trace_events {
    use core::ffi::{c_char, c_int};

    /* id is the pid of the task */
    // DEFINE_EVENT(event_da_monitor_id, event_wwnr,
    //              TP_PROTO(int id, char *state, char *event,
    //                       char *next_state, bool final_state),
    //              TP_ARGS(id, state, event, next_state, final_state));
    //
    // Rust representation of the declaration's prototype and arguments:
    pub type EventWwnrProto = unsafe extern "C" fn(
        id: c_int,
        state: *mut c_char,
        event: *mut c_char,
        next_state: *mut c_char,
        final_state: bool,
    );

    // DEFINE_EVENT(error_da_monitor_id, error_wwnr,
    //              TP_PROTO(int id, char *state, char *event),
    //              TP_ARGS(id, state, event));
    pub type ErrorWwnrProto = unsafe extern "C" fn(
        id: c_int,
        state: *mut c_char,
        event: *mut c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

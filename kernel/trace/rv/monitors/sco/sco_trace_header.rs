/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

// C build condition: CONFIG_RV_MON_SCO.
#[cfg(feature = "CONFIG_RV_MON_SCO")]
define_event!(
    event_da_monitor,
    event_sco,
    tp_proto!(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        next_state: *mut core::ffi::c_char,
        final_state: bool,
    ),
    tp_args!(state, event, next_state, final_state),
);

#[cfg(feature = "CONFIG_RV_MON_SCO")]
define_event!(
    error_da_monitor,
    error_sco,
    tp_proto!(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
    ),
    tp_args!(state, event),
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.rs
 */

/* C build-time condition: CONFIG_RV_MON_STALL */
#[cfg(feature = "CONFIG_RV_MON_STALL")]
DEFINE_EVENT!(
    event_da_monitor_id,
    event_stall,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        next_state: *mut ::core::ffi::c_char,
        final_state: bool
    ),
    TP_ARGS!(id, state, event, next_state, final_state)
);

#[cfg(feature = "CONFIG_RV_MON_STALL")]
DEFINE_EVENT!(
    error_da_monitor_id,
    error_stall,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char
    ),
    TP_ARGS!(id, state, event)
);

#[cfg(feature = "CONFIG_RV_MON_STALL")]
DEFINE_EVENT!(
    error_env_da_monitor_id,
    error_env_stall,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        env: *mut ::core::ffi::c_char
    ),
    TP_ARGS!(id, state, event, env)
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

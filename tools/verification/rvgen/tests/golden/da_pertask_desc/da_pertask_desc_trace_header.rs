// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

// C conditional: #ifdef CONFIG_RV_MON_DA_PERTASK_DESC
#[cfg(CONFIG_RV_MON_DA_PERTASK_DESC)]
define_event!(
    event_da_monitor_id,
    event_da_pertask_desc,
    tp_proto!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        next_state: *mut ::core::ffi::c_char,
        final_state: bool
    ),
    tp_args!(id, state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_DA_PERTASK_DESC)]
define_event!(
    error_da_monitor_id,
    error_da_pertask_desc,
    tp_proto!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char
    ),
    tp_args!(id, state, event)
);

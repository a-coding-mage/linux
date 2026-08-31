/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

use core::ffi::c_char;

// C conditional: #ifdef CONFIG_RV_MON_DA_GLOBAL
#[cfg(CONFIG_RV_MON_DA_GLOBAL)]
DEFINE_EVENT!(
    event_da_monitor,
    event_da_global,
    TP_PROTO!(
        state: *mut c_char,
        event: *mut c_char,
        next_state: *mut c_char,
        final_state: bool
    ),
    TP_ARGS!(state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_DA_GLOBAL)]
DEFINE_EVENT!(
    error_da_monitor,
    error_da_global,
    TP_PROTO!(state: *mut c_char, event: *mut c_char),
    TP_ARGS!(state, event)
);
// End C conditional: CONFIG_RV_MON_DA_GLOBAL

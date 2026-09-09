/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 *
 * The declarations below correspond to the trace events emitted by the
 * CONFIG_RV_MON_NRP conditional DEFINE_EVENT invocations in the C header.
 */

use core::ffi::{c_char, c_int};

// CONFIG_RV_MON_NRP
#[cfg(CONFIG_RV_MON_NRP)]
extern "C" {
    pub fn event_nrp(
        id: c_int,
        state: *mut c_char,
        event: *mut c_char,
        next_state: *mut c_char,
        final_state: bool,
    );

    pub fn error_nrp(id: c_int, state: *mut c_char, event: *mut c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

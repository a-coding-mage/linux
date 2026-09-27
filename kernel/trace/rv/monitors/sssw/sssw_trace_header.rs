/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

// CONFIG_RV_MON_SSSW condition from the original C header.
#[cfg(CONFIG_RV_MON_SSSW)]
extern "C" {
    pub fn event_sssw(
        id: ::kernel::ffi::c_int,
        state: *mut ::kernel::ffi::c_char,
        event: *mut ::kernel::ffi::c_char,
        next_state: *mut ::kernel::ffi::c_char,
        final_state: bool,
    );

    pub fn error_sssw(
        id: ::kernel::ffi::c_int,
        state: *mut ::kernel::ffi::c_char,
        event: *mut ::kernel::ffi::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

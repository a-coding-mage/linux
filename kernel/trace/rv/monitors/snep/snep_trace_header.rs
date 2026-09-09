/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* CONFIG_RV_MON_SNEP */
#[cfg(feature = "CONFIG_RV_MON_SNEP")]
extern "C" {
    pub fn event_snep(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        next_state: *mut core::ffi::c_char,
        final_state: bool,
    );

    pub fn error_snep(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/*
 * C conditional: CONFIG_RV_MON_OPID
 *
 * These declarations correspond to the DEFINE_EVENT invocations in the
 * source header.  The event machinery and its backing implementations are
 * supplied by the surrounding trace framework.
 */
#[cfg(feature = "CONFIG_RV_MON_OPID")]
extern "C" {
    pub fn event_opid(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        next_state: *mut core::ffi::c_char,
        final_state: bool,
    );

    pub fn error_opid(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
    );

    pub fn error_env_opid(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        env: *mut core::ffi::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

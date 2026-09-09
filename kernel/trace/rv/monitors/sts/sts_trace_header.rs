/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* Corresponds to the C preprocessor condition CONFIG_RV_MON_STS. */
#[cfg(feature = "CONFIG_RV_MON_STS")]
extern "C" {
    /*
     * DEFINE_EVENT(event_da_monitor, event_sts,
     *              TP_PROTO(char *state, char *event, char *next_state, bool final_state),
     *              TP_ARGS(state, event, next_state, final_state));
     */
    pub fn event_sts(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        next_state: *mut core::ffi::c_char,
        final_state: bool,
    );

    /*
     * DEFINE_EVENT(error_da_monitor, error_sts,
     *              TP_PROTO(char *state, char *event),
     *              TP_ARGS(state, event));
     */
    pub fn error_sts(state: *mut core::ffi::c_char, event: *mut core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

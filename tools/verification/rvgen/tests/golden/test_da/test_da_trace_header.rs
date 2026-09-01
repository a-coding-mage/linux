/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

use core::ffi::c_char;

/*
 * C conditional: #ifdef CONFIG_RV_MON_TEST_DA
 *
 * DEFINE_EVENT(event_da_monitor, event_test_da,
 *              TP_PROTO(char *state, char *event, char *next_state, bool final_state),
 *              TP_ARGS(state, event, next_state, final_state));
 *
 * DEFINE_EVENT(error_da_monitor, error_test_da,
 *              TP_PROTO(char *state, char *event),
 *              TP_ARGS(state, event));
 */
#[cfg(CONFIG_RV_MON_TEST_DA)]
unsafe extern "C" {
    pub fn event_test_da(
        state: *mut c_char,
        event: *mut c_char,
        next_state: *mut c_char,
        final_state: bool,
    );

    pub fn error_test_da(state: *mut c_char, event: *mut c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

// C conditional intent: #ifdef CONFIG_RV_MON_TEST_DA_KUNIT
DEFINE_EVENT!(
    event_da_monitor,
    event_test_da_kunit,
    TP_PROTO!(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
        next_state: *mut core::ffi::c_char,
        final_state: bool
    ),
    TP_ARGS!(state, event, next_state, final_state)
);

DEFINE_EVENT!(
    error_da_monitor,
    error_test_da_kunit,
    TP_PROTO!(
        state: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char
    ),
    TP_ARGS!(state, event)
);
// End C conditional intent: #endif /* CONFIG_RV_MON_TEST_DA_KUNIT */

// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

// C conditional: #ifdef CONFIG_RV_MON_TEST_HA_KUNIT
#[cfg(CONFIG_RV_MON_TEST_HA_KUNIT)]
DEFINE_EVENT!(
    event_da_monitor_id,
    event_test_ha_kunit,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        next_state: *mut ::core::ffi::c_char,
        final_state: bool
    ),
    TP_ARGS!(id, state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_TEST_HA_KUNIT)]
DEFINE_EVENT!(
    error_da_monitor_id,
    error_test_ha_kunit,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char
    ),
    TP_ARGS!(id, state, event)
);

#[cfg(CONFIG_RV_MON_TEST_HA_KUNIT)]
DEFINE_EVENT!(
    error_env_da_monitor_id,
    error_env_test_ha_kunit,
    TP_PROTO!(
        id: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_char,
        event: *mut ::core::ffi::c_char,
        env: *mut ::core::ffi::c_char
    ),
    TP_ARGS!(id, state, event, env)
);
// End C conditional: CONFIG_RV_MON_TEST_HA_KUNIT

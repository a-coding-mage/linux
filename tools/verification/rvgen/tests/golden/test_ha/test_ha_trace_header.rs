/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

#[cfg(CONFIG_RV_MON_TEST_HA)]
DEFINE_EVENT!(
    event_da_monitor_id,
    event_test_ha,
    TP_PROTO!(int id, char *state, char *event, char *next_state, bool final_state),
    TP_ARGS!(id, state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_TEST_HA)]
DEFINE_EVENT!(
    error_da_monitor_id,
    error_test_ha,
    TP_PROTO!(int id, char *state, char *event),
    TP_ARGS!(id, state, event)
);

#[cfg(CONFIG_RV_MON_TEST_HA)]
DEFINE_EVENT!(
    error_env_da_monitor_id,
    error_env_test_ha,
    TP_PROTO!(int id, char *state, char *event, char *env),
    TP_ARGS!(id, state, event, env)
);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

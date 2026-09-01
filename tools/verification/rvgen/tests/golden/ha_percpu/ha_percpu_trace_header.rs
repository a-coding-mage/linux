// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

#[cfg(CONFIG_RV_MON_HA_PERCPU)]
DEFINE_EVENT!(
    event_da_monitor,
    event_ha_percpu,
    TP_PROTO!(char *state, char *event, char *next_state, bool final_state),
    TP_ARGS!(state, event, next_state, final_state)
);

#[cfg(CONFIG_RV_MON_HA_PERCPU)]
DEFINE_EVENT!(
    error_da_monitor,
    error_ha_percpu,
    TP_PROTO!(char *state, char *event),
    TP_ARGS!(state, event)
);

#[cfg(CONFIG_RV_MON_HA_PERCPU)]
DEFINE_EVENT!(
    error_env_da_monitor,
    error_env_ha_percpu,
    TP_PROTO!(char *state, char *event, char *env),
    TP_ARGS!(state, event, env)
);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

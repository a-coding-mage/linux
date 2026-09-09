/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.rs.
 *
 * The DEFINE_EVENT declarations below are supplied by the tracing
 * infrastructure into which this header is included.  Their Rust bindings
 * remain declarations here; the event-generation implementation is external.
 */

#[cfg(CONFIG_RV_MON_NOMISS)]
extern "C" {
    /*
     * Original:
     * DEFINE_EVENT(event_da_monitor_id, event_nomiss,
     *              TP_PROTO(int id, char *state, char *event,
     *                       char *next_state, bool final_state),
     *              TP_ARGS(id, state, event, next_state, final_state));
     */
    pub static event_nomiss: core::ffi::c_void;

    /*
     * Original:
     * DEFINE_EVENT(error_da_monitor_id, error_nomiss,
     *              TP_PROTO(int id, char *state, char *event),
     *              TP_ARGS(id, state, event));
     */
    pub static error_nomiss: core::ffi::c_void;

    /*
     * Original:
     * DEFINE_EVENT(error_env_da_monitor_id, error_env_nomiss,
     *              TP_PROTO(int id, char *state, char *event, char *env),
     *              TP_ARGS(id, state, event, env));
     */
    pub static error_env_nomiss: core::ffi::c_void;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

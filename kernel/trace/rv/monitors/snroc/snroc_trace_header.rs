/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* CONFIG_RV_MON_SNROC */
#[cfg(feature = "CONFIG_RV_MON_SNROC")]
extern "C" {
    /* DEFINE_EVENT(event_da_monitor_id, event_snroc,
     *              TP_PROTO(int id, char *state, char *event, char *next_state,
     *                       bool final_state),
     *              TP_ARGS(id, state, event, next_state, final_state));
     */
    pub fn event_snroc(
        id: ::std::os::raw::c_int,
        state: *mut ::std::os::raw::c_char,
        event: *mut ::std::os::raw::c_char,
        next_state: *mut ::std::os::raw::c_char,
        final_state: bool,
    );

    /* DEFINE_EVENT(error_da_monitor_id, error_snroc,
     *              TP_PROTO(int id, char *state, char *event),
     *              TP_ARGS(id, state, event));
     */
    pub fn error_snroc(
        id: ::std::os::raw::c_int,
        state: *mut ::std::os::raw::c_char,
        event: *mut ::std::os::raw::c_char,
    );
}
/* CONFIG_RV_MON_SNROC */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

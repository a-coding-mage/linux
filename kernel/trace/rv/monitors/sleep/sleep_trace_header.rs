/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* Corresponds to CONFIG_RV_MON_SLEEP. */
#[cfg(feature = "CONFIG_RV_MON_SLEEP")]
mod config_rv_mon_sleep {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct task_struct {
        _private: [u8; 0],
    }

    /*
     * Corresponds to:
     * DEFINE_EVENT(event_ltl_monitor_id, event_sleep,
     *              TP_PROTO(struct task_struct *task, char *states,
     *                       char *atoms, char *next),
     *              TP_ARGS(task, states, atoms, next));
     */
    unsafe extern "C" {
        pub fn event_sleep(
            task: *mut task_struct,
            states: *mut c_char,
            atoms: *mut c_char,
            next: *mut c_char,
        );

        /*
         * Corresponds to:
         * DEFINE_EVENT(error_ltl_monitor_id, error_sleep,
         *              TP_PROTO(struct task_struct *task),
         *              TP_ARGS(task));
         */
        pub fn error_sleep(task: *mut task_struct);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

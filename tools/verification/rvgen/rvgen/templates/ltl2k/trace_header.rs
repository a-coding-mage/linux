/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/* Original C condition:
 * #ifdef CONFIG_RV_MON_%%MODEL_NAME_UP%%
 */
extern "C" {
    pub fn event_%%MODEL_NAME%%(
        task: *mut task_struct,
        states: *mut ::std::os::raw::c_char,
        atoms: *mut ::std::os::raw::c_char,
        next: *mut ::std::os::raw::c_char,
    );

    pub fn error_%%MODEL_NAME%%(task: *mut task_struct);
}

/* #endif */ /* CONFIG_RV_MON_%%MODEL_NAME_UP%% */

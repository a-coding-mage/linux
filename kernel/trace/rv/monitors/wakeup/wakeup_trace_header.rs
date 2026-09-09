/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 *
 * The C DEFINE_EVENT declarations below expand through the tracing
 * infrastructure supplied by the including build.  Rust has no direct
 * file-local equivalent for that macro expansion, so the event declarations
 * are represented as opaque externally supplied event types.
 */

/* Equivalent of: #ifdef CONFIG_RV_MON_WAKEUP */
#[cfg(feature = "CONFIG_RV_MON_WAKEUP")]
#[repr(C)]
pub struct event_wakeup {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_RV_MON_WAKEUP")]
#[repr(C)]
pub struct error_wakeup {
    _private: [u8; 0],
}

/*
 * DEFINE_EVENT(event_ltl_monitor_id, event_wakeup,
 *              TP_PROTO(struct task_struct *task, char *states,
 *                       char *atoms, char *next),
 *              TP_ARGS(task, states, atoms, next));
 *
 * DEFINE_EVENT(error_ltl_monitor_id, error_wakeup,
 *              TP_PROTO(struct task_struct *task),
 *              TP_ARGS(task));
 */

/* Equivalent of: #endif /* CONFIG_RV_MON_WAKEUP */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

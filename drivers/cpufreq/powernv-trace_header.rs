/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard and TRACE_HEADER_MULTI_READ condition are build-time
 * preprocessor controls. Rust items are emitted once by the module system.
 * The Linux tracepoint, cpufreq, and trace-event headers provide the external
 * tracepoint machinery represented below.
 */

use core::ffi::{c_char, c_int};

/// Data captured by the `powernv_throttle` trace event.
#[repr(C)]
pub struct PowernvThrottleEntry {
    pub chip_id: c_int,
    /* Corresponds to the dynamically assigned C `__string(reason, reason)`. */
    pub reason: *const c_char,
    pub pmax: c_int,
}

/*
 * TRACE_EVENT(powernv_throttle,
 *     TP_PROTO(int chip_id, const char *reason, int pmax),
 *     TP_ARGS(chip_id, reason, pmax),
 *     TP_FAST_ASSIGN:
 *         __entry->chip_id = chip_id;
 *         __assign_str(reason);
 *         __entry->pmax = pmax;
 *     TP_printk("Chip %d Pmax %d %s", __entry->chip_id,
 *               __entry->pmax, __get_str(reason))
 * );
 *
 * The generated tracepoint entry point is supplied by the Linux trace-event
 * implementation and is intentionally declaration-only here.
 */
extern "C" {
    pub fn trace_powernv_throttle(chip_id: c_int, reason: *const c_char, pmax: c_int);
}

/* TRACE_INCLUDE_PATH is `.` and TRACE_INCLUDE_FILE is `powernv-trace`. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

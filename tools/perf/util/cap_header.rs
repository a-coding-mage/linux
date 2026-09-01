/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted: __PERF_CAP_H.
// Dependencies in the original C header:
// - <stdbool.h>
// - <linux/capability.h>

/* For older systems */
// Fallback value used when CAP_SYSLOG is not supplied by linux/capability.h.
pub const CAP_SYSLOG: i32 = 34;

// Fallback value used when CAP_PERFMON is not supplied by linux/capability.h.
pub const CAP_PERFMON: i32 = 38;

// Fallback value used when CAP_BPF is not supplied by linux/capability.h.
pub const CAP_BPF: i32 = 39;

unsafe extern "C" {
    pub fn perf_cap__capable(cap: ::std::os::raw::c_int) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

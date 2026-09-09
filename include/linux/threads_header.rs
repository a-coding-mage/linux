/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The default limit for the nr of threads is now in
 * /proc/sys/kernel/threads-max.
 */

/*
 * Maximum supported processors.  Setting this smaller saves quite a
 * bit of memory.  Use nr_cpu_ids instead of this except for static bitmaps.
 *
 * CONFIG_NR_CPUS defaults to 1 when it is not supplied by the build
 * configuration.
 */
pub const CONFIG_NR_CPUS: usize = 1;

/* Places which use this should consider cpumask_var_t. */
pub const NR_CPUS: usize = CONFIG_NR_CPUS;

pub const MIN_THREADS_LEFT_FOR_ROOT: usize = 4;

/*
 * This controls the default maximum pid allocated to a process.
 * The CONFIG_BASE_SMALL conditional is represented by the corresponding
 * Rust cfg feature.
 */
pub const PID_MAX_DEFAULT: usize = if cfg!(feature = "CONFIG_BASE_SMALL") {
    0x1000
} else {
    0x8000
};

/*
 * A maximum of 4 million PIDs should be enough for a while.
 * [NOTE: PID/TIDs are limited to 2^30 ~= 1 billion, see FUTEX_TID_MASK.]
 *
 * PAGE_SIZE is supplied by the surrounding kernel translation when
 * CONFIG_BASE_SMALL is enabled.
 */
pub const PID_MAX_LIMIT: usize = if cfg!(feature = "CONFIG_BASE_SMALL") {
    PAGE_SIZE * 8
} else if core::mem::size_of::<core::ffi::c_long>() > 4 {
    4 * 1024 * 1024
} else {
    PID_MAX_DEFAULT
};

/*
 * Define a minimum number of pids per cpu.  Heuristically based
 * on original pid max of 32k for 32 cpus.  Also, increase the
 * minimum settable value for pid_max on the running system based
 * on similar defaults.  See kernel/pid.c:pid_idr_init() for details.
 */
pub const PIDS_PER_CPU_DEFAULT: usize = 1024;
pub const PIDS_PER_CPU_MIN: usize = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

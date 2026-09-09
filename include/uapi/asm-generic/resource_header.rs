/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Resource limit IDs
 *
 * ( Compatibility detail: there are architectures that have
 *   a different rlimit ID order in the 5-9 range and want
 *   to keep that order for binary compatibility. The reasons
 *   are historic and all new rlimits are identical across all
 *   arches. If an arch has such special order for some rlimits
 *   then it defines them prior including asm-generic/resource.h. )
 */

pub const RLIMIT_CPU: i32 = 0; /* CPU time in sec */
pub const RLIMIT_FSIZE: i32 = 1; /* Maximum filesize */
pub const RLIMIT_DATA: i32 = 2; /* max data size */
pub const RLIMIT_STACK: i32 = 3; /* max stack size */
pub const RLIMIT_CORE: i32 = 4; /* max core file size */

/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIMIT_RSS: i32 = 5; /* max resident set size */

/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIMIT_NPROC: i32 = 6; /* max number of processes */

/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIMIT_NOFILE: i32 = 7; /* max number of open files */

/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIMIT_MEMLOCK: i32 = 8; /* max locked-in-memory address space */

/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIMIT_AS: i32 = 9; /* address space limit */

pub const RLIMIT_LOCKS: i32 = 10; /* maximum file locks held */
pub const RLIMIT_SIGPENDING: i32 = 11; /* max number of pending signals */
pub const RLIMIT_MSGQUEUE: i32 = 12; /* maximum bytes in POSIX mqueues */
pub const RLIMIT_NICE: i32 = 13; /* max nice prio allowed to raise to
                                  0-39 for nice level 19 .. -20 */
pub const RLIMIT_RTPRIO: i32 = 14; /* maximum realtime priority */
pub const RLIMIT_RTTIME: i32 = 15; /* timeout for RT tasks in us */
pub const RLIM_NLIMITS: i32 = 16;

/*
 * SuS says limits have to be unsigned.
 * Which makes a ton more sense anyway.
 *
 * Some architectures override this (for compatibility reasons):
 */
/* C preprocessor conditional: define only if not supplied by another header. */
pub const RLIM_INFINITY: usize = usize::MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

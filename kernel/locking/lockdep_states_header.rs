/*
 * Lockdep states,
 *
 * please update XXX_LOCK_USAGE_STATES in include/linux/lockdep.h whenever
 * you add one, or come up with a nice dynamic solution.
 */
// The source uses an externally supplied X-macro; preserve each state
// declaration as a Rust macro invocation.
LOCKDEP_STATE!(HARDIRQ);
LOCKDEP_STATE!(SOFTIRQ);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * /proc/<pid>/oom_score_adj set to OOM_SCORE_ADJ_MIN disables oom killing for
 * pid.
 */
pub const OOM_SCORE_ADJ_MIN: i32 = -1000;
pub const OOM_SCORE_ADJ_MAX: i32 = 1000;

/*
 * /proc/<pid>/oom_adj set to -17 protects from the oom killer for legacy
 * purposes.
 */
pub const OOM_DISABLE: i32 = -17;
/* inclusive */
pub const OOM_ADJUST_MIN: i32 = -16;
pub const OOM_ADJUST_MAX: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

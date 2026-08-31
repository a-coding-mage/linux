/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard removed in Rust translation:
 * __CAP_HELPERS_H
 *
 * Original C dependencies:
 * #include <linux/types.h>
 * #include <linux/capability.h>
 * #include <errno.h>
 */

/* Fallback definition used by the C header when CAP_PERFMON is not provided. */
pub const CAP_PERFMON: i32 = 38;

/* Fallback definition used by the C header when CAP_BPF is not provided. */
pub const CAP_BPF: i32 = 39;

unsafe extern "C" {
    pub fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> i32;
    pub fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> i32;
}

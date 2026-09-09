// SPDX-License-Identifier: GPL-2.0
/*
 * Support for the configuration register space at port I/O locations
 * 0x22 and 0x23 variously used by PC architectures, e.g. the MP Spec,
 * Cyrix CPUs, numerous chipsets.  As the space is indirectly addressed
 * it may have to be protected with a spinlock, depending on the context.
 */

// Dependency supplied by the Linux spinlock headers.
// C: DEFINE_RAW_SPINLOCK(pc_conf_lock);
pub static mut pc_conf_lock: raw_spinlock_t = raw_spinlock_t::new();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

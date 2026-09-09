/* SPDX-License-Identifier: GPL-2.0-only */

/* DIMM health bitmap indicators */
/* SCM device is unable to persist memory contents */
pub const PAPR_PMEM_UNARMED: u64 = 1u64 << (63 - 0);
/* SCM device failed to persist memory contents */
pub const PAPR_PMEM_SHUTDOWN_DIRTY: u64 = 1u64 << (63 - 1);
/* SCM device contents are persisted from previous IPL */
pub const PAPR_PMEM_SHUTDOWN_CLEAN: u64 = 1u64 << (63 - 2);
/* SCM device contents are not persisted from previous IPL */
pub const PAPR_PMEM_EMPTY: u64 = 1u64 << (63 - 3);
/* SCM device memory life remaining is critically low */
pub const PAPR_PMEM_HEALTH_CRITICAL: u64 = 1u64 << (63 - 4);
/* SCM device will be garded off next IPL due to failure */
pub const PAPR_PMEM_HEALTH_FATAL: u64 = 1u64 << (63 - 5);
/* SCM contents cannot persist due to current platform health status */
pub const PAPR_PMEM_HEALTH_UNHEALTHY: u64 = 1u64 << (63 - 6);
/* SCM device is unable to persist memory contents in certain conditions */
pub const PAPR_PMEM_HEALTH_NON_CRITICAL: u64 = 1u64 << (63 - 7);
/* SCM device is encrypted */
pub const PAPR_PMEM_ENCRYPTED: u64 = 1u64 << (63 - 8);
/* SCM device has been scrubbed and locked */
pub const PAPR_PMEM_SCRUBBED_AND_LOCKED: u64 = 1u64 << (63 - 9);

pub const PAPR_PMEM_SAVE_FAILED: u64 = 1u64 << (63 - 10);

/* Bits status indicators for health bitmap indicating unarmed dimm */
pub const PAPR_PMEM_UNARMED_MASK: u64 = PAPR_PMEM_UNARMED | PAPR_PMEM_HEALTH_UNHEALTHY;

/* Bits status indicators for health bitmap indicating unflushed dimm */
pub const PAPR_PMEM_BAD_SHUTDOWN_MASK: u64 = PAPR_PMEM_SHUTDOWN_DIRTY;

/* Bits status indicators for health bitmap indicating unrestored dimm */
pub const PAPR_PMEM_BAD_RESTORE_MASK: u64 = PAPR_PMEM_EMPTY;

/* Bit status indicators for smart event notification */
pub const PAPR_PMEM_SMART_EVENT_MASK: u64 =
    PAPR_PMEM_HEALTH_CRITICAL | PAPR_PMEM_HEALTH_FATAL | PAPR_PMEM_HEALTH_UNHEALTHY;

pub const PAPR_PMEM_SAVE_MASK: u64 = PAPR_PMEM_SAVE_FAILED;

/* C __stringify(SCMSTATS) macro equivalent. */
pub const PAPR_SCM_PERF_STATS_EYECATCHER: &str = "SCMSTATS";
pub const PAPR_SCM_PERF_STATS_VERSION: u32 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

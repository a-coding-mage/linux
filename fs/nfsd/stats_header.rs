/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Statistics for NFS server.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Declarations supplied by the corresponding Linux NFS and kernel headers.

extern "C" {
    pub fn nfsd_proc_stat_init(net: *mut net) -> *mut proc_dir_entry;
    pub fn nfsd_proc_stat_shutdown(net: *mut net);
}

/**
 * nfsd_stats_rc_hits_inc - Count a duplicate reply cache hit
 * @nn: target network namespace
 *
 * These reply cache counters are updated once per RPC. Readers use
 * percpu_counter_sum_positive(), so local batching does not affect
 * read accuracy.
 */
pub unsafe fn nfsd_stats_rc_hits_inc(nn: *mut nfsd_net) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_RC_HITS], 1);
}

/**
 * nfsd_stats_rc_misses_inc - Count a duplicate reply cache miss
 * @nn: target network namespace
 *
 * See nfsd_stats_rc_hits_inc() for batching rationale.
 */
pub unsafe fn nfsd_stats_rc_misses_inc(nn: *mut nfsd_net) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_RC_MISSES], 1);
}

/**
 * nfsd_stats_rc_nocache_inc - Count a request not cached in the reply cache
 * @nn: target network namespace
 *
 * See nfsd_stats_rc_hits_inc() for batching rationale.
 */
pub unsafe fn nfsd_stats_rc_nocache_inc(nn: *mut nfsd_net) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_RC_NOCACHE], 1);
}

pub unsafe fn nfsd_stats_fh_stale_inc(nn: *mut nfsd_net, exp: *mut svc_export) {
    percpu_counter_inc(&mut (*nn).counter[NFSD_STATS_FH_STALE]);
    if !exp.is_null() && !(*exp).ex_stats.is_null() {
        percpu_counter_inc(&mut (*(*exp).ex_stats).counter[EXP_STATS_FH_STALE]);
    }
}

/**
 * nfsd_stats_io_read_add - Count number of bytes for an NFS READ
 * @nn: target network namespace
 * @exp: target export
 * @amount: byte count
 *
 * These counters are updated on every READ request. Readers use
 * percpu_counter_sum_positive(), so local batching does not affect
 * read accuracy.
 */
pub unsafe fn nfsd_stats_io_read_add(nn: *mut nfsd_net, exp: *mut svc_export, amount: s64) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_IO_READ], amount);
    if !exp.is_null() && !(*exp).ex_stats.is_null() {
        percpu_counter_add_local(&mut (*(*exp).ex_stats).counter[EXP_STATS_IO_READ], amount);
    }
}

/**
 * nfsd_stats_io_write_add - Count number of bytes for an NFS WRITE
 * @nn: target network namespace
 * @exp: target export
 * @amount: byte count
 *
 * These counters are updated on every WRITE request. Readers use
 * percpu_counter_sum_positive(), so local batching does not affect
 * read accuracy.
 */
pub unsafe fn nfsd_stats_io_write_add(nn: *mut nfsd_net, exp: *mut svc_export, amount: s64) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_IO_WRITE], amount);
    if !exp.is_null() && !(*exp).ex_stats.is_null() {
        percpu_counter_add_local(&mut (*(*exp).ex_stats).counter[EXP_STATS_IO_WRITE], amount);
    }
}

pub unsafe fn nfsd_stats_payload_misses_inc(nn: *mut nfsd_net) {
    percpu_counter_inc(&mut (*nn).counter[NFSD_STATS_PAYLOAD_MISSES]);
}

/**
 * nfsd_stats_drc_mem_usage_add - Add memory used by a cache item
 * @nn: target network namespace
 * @amount: byte count
 *
 * percpu_counter_add_local() keeps updates on the per-CPU fast
 * path. The sole reader, percpu_counter_sum_positive(), sums the
 * per-CPU deltas, so batching locally does not lose accuracy.
 */
pub unsafe fn nfsd_stats_drc_mem_usage_add(nn: *mut nfsd_net, amount: s64) {
    percpu_counter_add_local(&mut (*nn).counter[NFSD_STATS_DRC_MEM_USAGE], amount);
}

/**
 * nfsd_stats_drc_mem_usage_sub - Subtract memory used by a cache item
 * @nn: target network namespace
 * @amount: byte count
 *
 * See nfsd_stats_drc_mem_usage_add() for batching rationale.
 */
pub unsafe fn nfsd_stats_drc_mem_usage_sub(nn: *mut nfsd_net, amount: s64) {
    percpu_counter_sub_local(&mut (*nn).counter[NFSD_STATS_DRC_MEM_USAGE], amount);
}

// Conditional on CONFIG_NFSD_V4 in the C build.
#[cfg(CONFIG_NFSD_V4)]
pub unsafe fn nfsd_stats_cb_op_inc(nn: *mut nfsd_net, opcode: u32) {
    if opcode >= OP_CB_GETATTR && opcode <= OP_CB_OFFLOAD {
        percpu_counter_inc(&mut (*nn).cb_counter[opcode as usize]);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

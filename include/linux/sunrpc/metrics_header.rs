/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/include/linux/sunrpc/metrics.h
 *
 *  Declarations for RPC client per-operation metrics
 *
 *  Copyright (C) 2005 Chuck Lever <cel@netapp.com>
 *
 *  RPC client per-operation statistics provide latency and retry
 *  information about each type of RPC procedure in a given RPC program.
 *  These statistics are not for detailed problem diagnosis, but simply
 *  to indicate whether the problem is local or remote.
 *
 *  These counters are not meant to be human-readable, but are meant to be
 *  integrated into system monitoring tools such as "sar" and "iostat".
 *  As such, the counters are sampled by the tools over time, and are never
 *  zeroed after a file system is mounted. Moving averages can be computed
 *  by the tools by taking the difference between two instantaneous samples
 *  and dividing that by the time between the samples.
 *
 *  The counters are maintained in a single array per RPC client, indexed
 *  by procedure number. There is no need to maintain separate counter
 *  arrays per-CPU because these counters are always modified behind locks.
 */

// Dependency declarations supplied by other translated files.
// `spinlock_t`, `ktime_t`, and `seq_file` are intentionally not defined here.

pub const RPC_IOSTATS_VERS: &str = "1.1";

#[repr(C)]
pub struct rpc_iostats {
    pub om_lock: spinlock_t,

    /* These counters give an idea about how many request transmissions are
     * required, on average, to complete that particular procedure. */
    pub om_ops: core::ffi::c_ulong,
    pub om_ntrans: core::ffi::c_ulong,
    pub om_timeouts: core::ffi::c_ulong,

    /* Count of bytes sent and received for a given RPC procedure type. */
    pub om_bytes_sent: core::ffi::c_ulonglong,
    pub om_bytes_recv: core::ffi::c_ulonglong,

    /* Queued for transmission, RPC RTT, and RPC execution time. */
    pub om_queue: ktime_t,
    pub om_rtt: ktime_t,
    pub om_execute: ktime_t,

    /* Count of operations that complete with tk_status < 0. */
    pub om_error_status: core::ffi::c_ulong,
}
// ____cacheline_aligned is a C build/layout attribute; alignment is supplied
// by the surrounding target ABI when this declaration is integrated.

#[repr(C)]
pub struct rpc_task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rpc_clnt {
    _private: [u8; 0],
}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn rpc_alloc_iostats(clnt: *mut rpc_clnt) -> *mut rpc_iostats;
    pub fn rpc_count_iostats(task: *const rpc_task, stats: *mut rpc_iostats);
    pub fn rpc_count_iostats_metrics(task: *const rpc_task, stats: *mut rpc_iostats);
    pub fn rpc_clnt_show_stats(seq: *mut seq_file, clnt: *mut rpc_clnt);
    pub fn rpc_free_iostats(stats: *mut rpc_iostats);
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_alloc_iostats(_clnt: *mut rpc_clnt) -> *mut rpc_iostats {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_count_iostats(_task: *const rpc_task, _stats: *mut rpc_iostats) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_count_iostats_metrics(_task: *const rpc_task, _stats: *mut rpc_iostats) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_clnt_show_stats(_seq: *mut seq_file, _clnt: *mut rpc_clnt) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_free_iostats(_stats: *mut rpc_iostats) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

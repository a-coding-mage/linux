/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/// Point-in-time snapshot of subvolume metrics.
#[repr(C)]
pub struct ceph_subvol_metric_snapshot {
    pub subvolume_id: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_latency_us: u64,
    pub write_latency_us: u64,
}

/// Tracks per-subvolume I/O metrics.
#[repr(C)]
pub struct ceph_subvolume_metrics_tracker {
    pub lock: spinlock_t,
    pub tree: rb_root_cached,
    pub nr_entries: u32,
    pub enabled: bool,
    pub snapshot_attempts: atomic64_t,
    pub snapshot_empty: atomic64_t,
    pub snapshot_failures: atomic64_t,
    pub record_calls: atomic64_t,
    pub record_disabled: atomic64_t,
    pub record_no_subvol: atomic64_t,
    pub total_read_ops: atomic64_t,
    pub total_read_bytes: atomic64_t,
    pub total_write_ops: atomic64_t,
    pub total_write_bytes: atomic64_t,
}

extern "C" {
    pub fn ceph_subvolume_metrics_init(
        tracker: *mut ceph_subvolume_metrics_tracker,
    );
    pub fn ceph_subvolume_metrics_destroy(
        tracker: *mut ceph_subvolume_metrics_tracker,
    );
    pub fn ceph_subvolume_metrics_enable(
        tracker: *mut ceph_subvolume_metrics_tracker,
        enable: bool,
    );
    pub fn ceph_subvolume_metrics_record(
        tracker: *mut ceph_subvolume_metrics_tracker,
        subvol_id: u64,
        is_write: bool,
        size: usize,
        latency_us: u64,
    );
    pub fn ceph_subvolume_metrics_snapshot(
        tracker: *mut ceph_subvolume_metrics_tracker,
        out: *mut *mut ceph_subvol_metric_snapshot,
        nr: *mut u32,
        consume: bool,
    ) -> i32;
    pub fn ceph_subvolume_metrics_free_snapshot(
        snapshot: *mut ceph_subvol_metric_snapshot,
    );
    pub fn ceph_subvolume_metrics_dump(
        tracker: *mut ceph_subvolume_metrics_tracker,
        s: *mut seq_file,
    );

    pub fn ceph_subvolume_metrics_record_io(
        mdsc: *mut ceph_mds_client,
        ci: *mut ceph_inode_info,
        is_write: bool,
        bytes: usize,
        start: ktime_t,
        end: ktime_t,
    );

    pub fn ceph_subvolume_metrics_cache_init() -> i32;
    pub fn ceph_subvolume_metrics_cache_destroy();
}

/// Equivalent to READ_ONCE(tracker->enabled).
#[inline]
pub unsafe fn ceph_subvolume_metrics_enabled(
    tracker: *const ceph_subvolume_metrics_tracker,
) -> bool {
    core::ptr::read_volatile(core::ptr::addr_of!((*tracker).enabled))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

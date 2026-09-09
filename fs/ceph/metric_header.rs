/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel/Ceph translation.

extern "C" {
    pub static mut disable_send_metrics: bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ceph_metric_type {
    CLIENT_METRIC_TYPE_CAP_INFO,
    CLIENT_METRIC_TYPE_READ_LATENCY,
    CLIENT_METRIC_TYPE_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_DENTRY_LEASE,
    CLIENT_METRIC_TYPE_OPENED_FILES,
    CLIENT_METRIC_TYPE_PINNED_ICAPS,
    CLIENT_METRIC_TYPE_OPENED_INODES,
    CLIENT_METRIC_TYPE_READ_IO_SIZES,
    CLIENT_METRIC_TYPE_WRITE_IO_SIZES,
    CLIENT_METRIC_TYPE_AVG_READ_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_READ_LATENCY,
    CLIENT_METRIC_TYPE_AVG_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_AVG_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_SUBVOLUME_METRICS,
}

pub const CLIENT_METRIC_TYPE_MAX: ceph_metric_type =
    ceph_metric_type::CLIENT_METRIC_TYPE_SUBVOLUME_METRICS;

/* This will always have the highest metric bit value as the last element. */
pub const CEPHFS_METRIC_SPEC_CLIENT_SUPPORTED: &[ceph_metric_type] = &[
    ceph_metric_type::CLIENT_METRIC_TYPE_CAP_INFO,
    ceph_metric_type::CLIENT_METRIC_TYPE_READ_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_WRITE_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_METADATA_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_DENTRY_LEASE,
    ceph_metric_type::CLIENT_METRIC_TYPE_OPENED_FILES,
    ceph_metric_type::CLIENT_METRIC_TYPE_PINNED_ICAPS,
    ceph_metric_type::CLIENT_METRIC_TYPE_OPENED_INODES,
    ceph_metric_type::CLIENT_METRIC_TYPE_READ_IO_SIZES,
    ceph_metric_type::CLIENT_METRIC_TYPE_WRITE_IO_SIZES,
    ceph_metric_type::CLIENT_METRIC_TYPE_AVG_READ_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_STDEV_READ_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_AVG_WRITE_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_STDEV_WRITE_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_AVG_METADATA_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_STDEV_METADATA_LATENCY,
    ceph_metric_type::CLIENT_METRIC_TYPE_SUBVOLUME_METRICS,
    ceph_metric_type::CLIENT_METRIC_TYPE_SUBVOLUME_METRICS,
];

#[repr(C, packed)]
pub struct ceph_metric_header { pub r#type: __le32, pub ver: __u8, pub compat: __u8, pub data_len: __le32 }

#[repr(C, packed)]
pub struct ceph_metric_cap { pub header: ceph_metric_header, pub hit: __le64, pub mis: __le64, pub total: __le64 }

#[repr(C, packed)]
pub struct ceph_metric_read_latency { pub header: ceph_metric_header, pub lat: ceph_timespec, pub avg: ceph_timespec, pub sq_sum: __le64, pub count: __le64 }
#[repr(C, packed)]
pub struct ceph_metric_write_latency { pub header: ceph_metric_header, pub lat: ceph_timespec, pub avg: ceph_timespec, pub sq_sum: __le64, pub count: __le64 }
#[repr(C, packed)]
pub struct ceph_metric_metadata_latency { pub header: ceph_metric_header, pub lat: ceph_timespec, pub avg: ceph_timespec, pub sq_sum: __le64, pub count: __le64 }

#[repr(C, packed)]
pub struct ceph_metric_dlease { pub header: ceph_metric_header, pub hit: __le64, pub mis: __le64, pub total: __le64 }
#[repr(C, packed)]
pub struct ceph_opened_files { pub header: ceph_metric_header, pub opened_files: __le64, pub total: __le64 }
#[repr(C, packed)]
pub struct ceph_pinned_icaps { pub header: ceph_metric_header, pub pinned_icaps: __le64, pub total: __le64 }
#[repr(C, packed)]
pub struct ceph_opened_inodes { pub header: ceph_metric_header, pub opened_inodes: __le64, pub total: __le64 }
#[repr(C, packed)]
pub struct ceph_read_io_size { pub header: ceph_metric_header, pub total_ops: __le64, pub total_size: __le64 }
#[repr(C, packed)]
pub struct ceph_write_io_size { pub header: ceph_metric_header, pub total_ops: __le64, pub total_size: __le64 }

#[repr(C, packed)]
pub struct ceph_subvolume_metric_entry_wire {
    pub subvolume_id: __le64, pub read_ops: __le32, pub write_ops: __le32,
    pub read_bytes: __le64, pub write_bytes: __le64, pub read_latency_us: __le64,
    pub write_latency_us: __le64, pub time_stamp: __le64,
}

#[repr(C, packed)]
pub struct ceph_subvolume_metric_entry {
    pub subvolume_id: __le64, pub read_ops: __le64, pub write_ops: __le64,
    pub read_bytes: __le64, pub write_bytes: __le64, pub read_latency_us: __le64,
    pub write_latency_us: __le64,
}

#[repr(C, packed)]
pub struct ceph_metric_head { pub num: __le32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum metric_type { METRIC_READ, METRIC_WRITE, METRIC_METADATA, METRIC_COPYFROM, METRIC_MAX }

#[repr(C)]
pub struct ceph_metric {
    pub lock: spinlock_t, pub total: u64, pub size_sum: u64, pub size_min: u64, pub size_max: u64,
    pub latency_sum: ktime_t, pub latency_avg: ktime_t, pub latency_sq_sum: ktime_t,
    pub latency_min: ktime_t, pub latency_max: ktime_t,
}

#[repr(C)]
pub struct ceph_client_metric {
    pub total_dentries: atomic64_t, pub d_lease_hit: percpu_counter, pub d_lease_mis: percpu_counter,
    pub total_caps: atomic64_t, pub i_caps_hit: percpu_counter, pub i_caps_mis: percpu_counter,
    pub metric: [ceph_metric; 4], pub opened_files: atomic64_t,
    pub opened_inodes: percpu_counter, pub total_inodes: percpu_counter,
    pub session: *mut ceph_mds_session, pub delayed_work: delayed_work,
}

extern "C" {
    pub fn ceph_metric_init(m: *mut ceph_client_metric) -> c_int;
    pub fn ceph_metric_destroy(m: *mut ceph_client_metric);
    pub fn ceph_update_metrics(m: *mut ceph_metric, r_start: ktime_t, r_end: ktime_t, size: c_uint, rc: c_int);
    pub fn percpu_counter_inc(counter: *mut percpu_counter);
    pub fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    pub fn round_jiffies_relative(j: c_ulong) -> c_ulong;
}

pub unsafe fn metric_schedule_delayed(m: *mut ceph_client_metric) {
    if disable_send_metrics { return; }
    schedule_delayed_work(&mut (*m).delayed_work, round_jiffies_relative(HZ));
}

pub unsafe fn ceph_update_cap_hit(m: *mut ceph_client_metric) { percpu_counter_inc(&mut (*m).i_caps_hit); }
pub unsafe fn ceph_update_cap_mis(m: *mut ceph_client_metric) { percpu_counter_inc(&mut (*m).i_caps_mis); }

pub unsafe fn ceph_update_read_metrics(m: *mut ceph_client_metric, r_start: ktime_t, r_end: ktime_t, size: c_uint, rc: c_int) {
    ceph_update_metrics(&mut (*m).metric[METRIC_READ as usize], r_start, r_end, size, rc);
}
pub unsafe fn ceph_update_write_metrics(m: *mut ceph_client_metric, r_start: ktime_t, r_end: ktime_t, size: c_uint, rc: c_int) {
    ceph_update_metrics(&mut (*m).metric[METRIC_WRITE as usize], r_start, r_end, size, rc);
}
pub unsafe fn ceph_update_metadata_metrics(m: *mut ceph_client_metric, r_start: ktime_t, r_end: ktime_t, rc: c_int) {
    ceph_update_metrics(&mut (*m).metric[METRIC_METADATA as usize], r_start, r_end, 0, rc);
}
pub unsafe fn ceph_update_copyfrom_metrics(m: *mut ceph_client_metric, r_start: ktime_t, r_end: ktime_t, size: c_uint, rc: c_int) {
    ceph_update_metrics(&mut (*m).metric[METRIC_COPYFROM as usize], r_start, r_end, size, rc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

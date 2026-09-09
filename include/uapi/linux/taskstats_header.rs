/* SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note */
/* taskstats.h - exporting per-task statistics
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2006
 *           (C) Balbir Singh,   IBM Corp. 2006
 *           (C) Jay Lan,        SGI, 2006
 */

// Translated from the Linux UAPI taskstats header.

pub const TASKSTATS_VERSION: u32 = 17;
pub const TS_COMM_LEN: usize = 32;

#[repr(C)]
pub struct taskstats {
    pub version: __u16,
    pub ac_exitcode: __u32,
    pub ac_flag: __u8,
    pub ac_nice: __u8,
    pub cpu_count: __u64,
    pub cpu_delay_total: __u64,
    pub blkio_count: __u64,
    pub blkio_delay_total: __u64,
    pub swapin_count: __u64,
    pub swapin_delay_total: __u64,
    pub cpu_run_real_total: __u64,
    pub cpu_run_virtual_total: __u64,
    pub ac_comm: [::core::ffi::c_char; TS_COMM_LEN],
    pub ac_sched: __u8,
    pub ac_pad: [__u8; 3],
    pub ac_uid: __u32,
    pub ac_gid: __u32,
    pub ac_pid: __u32,
    pub ac_ppid: __u32,
    pub ac_btime: __u32,
    pub ac_etime: __u64,
    pub ac_utime: __u64,
    pub ac_stime: __u64,
    pub ac_minflt: __u64,
    pub ac_majflt: __u64,
    pub coremem: __u64,
    pub virtmem: __u64,
    pub hiwater_rss: __u64,
    pub hiwater_vm: __u64,
    pub read_char: __u64,
    pub write_char: __u64,
    pub read_syscalls: __u64,
    pub write_syscalls: __u64,
    pub read_bytes: __u64,
    pub write_bytes: __u64,
    pub cancelled_write_bytes: __u64,
    pub nvcsw: __u64,
    pub nivcsw: __u64,
    pub ac_utimescaled: __u64,
    pub ac_stimescaled: __u64,
    pub cpu_scaled_run_real_total: __u64,
    pub freepages_count: __u64,
    pub freepages_delay_total: __u64,
    pub thrashing_count: __u64,
    pub thrashing_delay_total: __u64,
    pub ac_btime64: __u64,
    pub compact_count: __u64,
    pub compact_delay_total: __u64,
    pub ac_tgid: __u32,
    pub ac_tgetime: __u64,
    pub ac_exe_dev: __u64,
    pub ac_exe_inode: __u64,
    pub wpcopy_count: __u64,
    pub wpcopy_delay_total: __u64,
    pub irq_count: __u64,
    pub irq_delay_total: __u64,
    pub cpu_delay_max: __u64,
    pub cpu_delay_min: __u64,
    pub blkio_delay_max: __u64,
    pub blkio_delay_min: __u64,
    pub swapin_delay_max: __u64,
    pub swapin_delay_min: __u64,
    pub freepages_delay_max: __u64,
    pub freepages_delay_min: __u64,
    pub thrashing_delay_max: __u64,
    pub thrashing_delay_min: __u64,
    pub compact_delay_max: __u64,
    pub compact_delay_min: __u64,
    pub wpcopy_delay_max: __u64,
    pub wpcopy_delay_min: __u64,
    pub irq_delay_max: __u64,
    pub irq_delay_min: __u64,
    pub cpu_delay_max_ts: __kernel_timespec,
    pub blkio_delay_max_ts: __kernel_timespec,
    pub swapin_delay_max_ts: __kernel_timespec,
    pub freepages_delay_max_ts: __kernel_timespec,
    pub thrashing_delay_max_ts: __kernel_timespec,
    pub compact_delay_max_ts: __kernel_timespec,
    pub wpcopy_delay_max_ts: __kernel_timespec,
    pub irq_delay_max_ts: __kernel_timespec,
}

pub const TASKSTATS_HAS_IO_ACCOUNTING: bool = true;

pub const TASKSTATS_CMD_UNSPEC: u32 = 0;
pub const TASKSTATS_CMD_GET: u32 = 1;
pub const TASKSTATS_CMD_NEW: u32 = 2;
pub const __TASKSTATS_CMD_MAX: u32 = 3;
pub const TASKSTATS_CMD_MAX: u32 = __TASKSTATS_CMD_MAX - 1;

pub const TASKSTATS_TYPE_UNSPEC: u32 = 0;
pub const TASKSTATS_TYPE_PID: u32 = 1;
pub const TASKSTATS_TYPE_TGID: u32 = 2;
pub const TASKSTATS_TYPE_STATS: u32 = 3;
pub const TASKSTATS_TYPE_AGGR_PID: u32 = 4;
pub const TASKSTATS_TYPE_AGGR_TGID: u32 = 5;
pub const TASKSTATS_TYPE_NULL: u32 = 6;
pub const __TASKSTATS_TYPE_MAX: u32 = 7;
pub const TASKSTATS_TYPE_MAX: u32 = __TASKSTATS_TYPE_MAX - 1;

pub const TASKSTATS_CMD_ATTR_UNSPEC: u32 = 0;
pub const TASKSTATS_CMD_ATTR_PID: u32 = 1;
pub const TASKSTATS_CMD_ATTR_TGID: u32 = 2;
pub const TASKSTATS_CMD_ATTR_REGISTER_CPUMASK: u32 = 3;
pub const TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK: u32 = 4;
pub const __TASKSTATS_CMD_ATTR_MAX: u32 = 5;
pub const TASKSTATS_CMD_ATTR_MAX: u32 = __TASKSTATS_CMD_ATTR_MAX - 1;

pub const TASKSTATS_GENL_NAME: &str = "TASKSTATS";
pub const TASKSTATS_GENL_VERSION: u32 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

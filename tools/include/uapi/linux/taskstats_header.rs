/* SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note */
/* taskstats.h - exporting per-task statistics
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2006
 *           (C) Balbir Singh,   IBM Corp. 2006
 *           (C) Jay Lan,        SGI, 2006
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 */

/* Depends on Linux UAPI equivalents of <linux/types.h> and
 * <linux/time_types.h>, including __kernel_timespec.
 */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

/* Format for per-task data returned to userland when
 *	- a task exits
 *	- listener requests stats for a task
 *
 * The struct is versioned. Newer versions should only add fields to
 * the bottom of the struct to maintain backward compatibility.
 *
 *
 * To add new fields
 *	a) bump up TASKSTATS_VERSION
 *	b) add comment indicating new version number at end of struct
 *	c) add new fields after version comment; maintain 64-bit alignment
 */

pub const TASKSTATS_VERSION: __u16 = 17;
pub const TS_COMM_LEN: usize = 32; /* should be >= TASK_COMM_LEN
				    * in linux/sched.h */

#[repr(C)]
pub struct taskstats {
    /* The version number of this struct. This field is always set to
     * TAKSTATS_VERSION, which is defined in <linux/taskstats.h>.
     * Each time the struct is changed, the value should be incremented.
     */
    pub version: __u16,
    pub ac_exitcode: __u32, /* Exit status */

    /* The accounting flags of a task as defined in <linux/acct.h>
     * Defined values are AFORK, ASU, ACOMPAT, ACORE, AXSIG, and AGROUP.
     * (AGROUP since version 12).
     */
    pub ac_flag: __u8, /* Record flags */
    pub ac_nice: __u8, /* task_nice */

    /* Delay accounting fields start
     *
     * All values, until comment "Delay accounting fields end" are
     * available only if delay accounting is enabled, even though the last
     * few fields are not delays
     *
     * xxx_count is the number of delay values recorded
     * xxx_delay_total is the corresponding cumulative delay in nanoseconds
     *
     * xxx_delay_total wraps around to zero on overflow
     * xxx_count incremented regardless of overflow
     */

    /* Delay waiting for cpu, while runnable
     * count, delay_total NOT updated atomically
     */
    pub cpu_count: __u64,
    pub cpu_delay_total: __u64,

    /* Following four fields atomically updated using task->delays->lock */

    /* Delay waiting for synchronous block I/O to complete
     * does not account for delays in I/O submission
     */
    pub blkio_count: __u64,
    pub blkio_delay_total: __u64,

    /* Delay waiting for page fault I/O (swap in only) */
    pub swapin_count: __u64,
    pub swapin_delay_total: __u64,

    /* cpu "wall-clock" running time
     * On some architectures, value will adjust for cpu time stolen
     * from the kernel in involuntary waits due to virtualization.
     * Value is cumulative, in nanoseconds, without a corresponding count
     * and wraps around to zero silently on overflow
     */
    pub cpu_run_real_total: __u64,

    /* cpu "virtual" running time
     * Uses time intervals seen by the kernel i.e. no adjustment
     * for kernel's involuntary waits due to virtualization.
     * Value is cumulative, in nanoseconds, without a corresponding count
     * and wraps around to zero silently on overflow
     */
    pub cpu_run_virtual_total: __u64,
    /* Delay accounting fields end */
    /* version 1 ends here */

    /* Basic Accounting Fields start */
    pub ac_comm: [::std::os::raw::c_char; TS_COMM_LEN], /* Command name */
    pub ac_sched: __u8, /* Scheduling discipline */
    pub ac_pad: [__u8; 3],
    pub ac_uid: __u32, /* User ID */
    pub ac_gid: __u32, /* Group ID */
    pub ac_pid: __u32, /* Process ID */
    pub ac_ppid: __u32, /* Parent process ID */
    /* __u32 range means times from 1970 to 2106 */
    pub ac_btime: __u32, /* Begin time [sec since 1970] */
    pub ac_etime: __u64, /* Elapsed time [usec] */
    pub ac_utime: __u64, /* User CPU time [usec] */
    pub ac_stime: __u64, /* SYstem CPU time [usec] */
    pub ac_minflt: __u64, /* Minor Page Fault Count */
    pub ac_majflt: __u64, /* Major Page Fault Count */
    /* Basic Accounting Fields end */

    /* Extended accounting fields start */
    /* Accumulated RSS usage in duration of a task, in MBytes-usecs.
     * The current rss usage is added to this counter every time
     * a tick is charged to a task's system time. So, at the end we
     * will have memory usage multiplied by system time. Thus an
     * average usage per system time unit can be calculated.
     */
    pub coremem: __u64, /* accumulated RSS usage in MB-usec */
    /* Accumulated virtual memory usage in duration of a task.
     * Same as acct_rss_mem1 above except that we keep track of VM usage.
     */
    pub virtmem: __u64, /* accumulated VM  usage in MB-usec */

    /* High watermark of RSS and virtual memory usage in duration of
     * a task, in KBytes.
     */
    pub hiwater_rss: __u64, /* High-watermark of RSS usage, in KB */
    pub hiwater_vm: __u64, /* High-water VM usage, in KB */

    /* The following four fields are I/O statistics of a task. */
    pub read_char: __u64, /* bytes read */
    pub write_char: __u64, /* bytes written */
    pub read_syscalls: __u64, /* read syscalls */
    pub write_syscalls: __u64, /* write syscalls */
    /* Extended accounting fields end */

    /* TASKSTATS_HAS_IO_ACCOUNTING */
    /* Per-task storage I/O accounting starts */
    pub read_bytes: __u64, /* bytes of read I/O */
    pub write_bytes: __u64, /* bytes of write I/O */
    pub cancelled_write_bytes: __u64, /* bytes of cancelled write I/O */

    pub nvcsw: __u64, /* voluntary_ctxt_switches */
    pub nivcsw: __u64, /* nonvoluntary_ctxt_switches */

    /* time accounting for SMT machines */
    pub ac_utimescaled: __u64, /* utime scaled on frequency etc */
    pub ac_stimescaled: __u64, /* stime scaled on frequency etc */
    pub cpu_scaled_run_real_total: __u64, /* scaled cpu_run_real_total */

    /* Delay waiting for memory reclaim */
    pub freepages_count: __u64,
    pub freepages_delay_total: __u64,

    /* Delay waiting for thrashing page */
    pub thrashing_count: __u64,
    pub thrashing_delay_total: __u64,

    /* v10: 64-bit btime to avoid overflow */
    pub ac_btime64: __u64, /* 64-bit begin time */

    /* v11: Delay waiting for memory compact */
    pub compact_count: __u64,
    pub compact_delay_total: __u64,

    /* v12 begin */
    pub ac_tgid: __u32, /* thread group ID */
    /* Thread group walltime up to now. This is total process walltime if
     * AGROUP flag is set.
     */
    pub ac_tgetime: __u64,
    /* Lightweight information to identify process binary files.
     * This leaves userspace to match this to a file system path, using
     * MAJOR() and MINOR() macros to identify a device and mount point,
     * the inode to identify the executable file. This is /proc/self/exe
     * at the end, so matching the most recent exec(). Values are zero
     * for kernel threads.
     */
    pub ac_exe_dev: __u64, /* program binary device ID */
    pub ac_exe_inode: __u64, /* program binary inode number */
    /* v12 end */

    /* v13: Delay waiting for write-protect copy */
    pub wpcopy_count: __u64,
    pub wpcopy_delay_total: __u64,

    /* v14: Delay waiting for IRQ/SOFTIRQ */
    pub irq_count: __u64,
    pub irq_delay_total: __u64,

    /* v15: add Delay max and Delay min */

    /* v16: move Delay max and Delay min to the end of taskstat */
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

    /*v17: delay max timestamp record*/
    pub cpu_delay_max_ts: __kernel_timespec,
    pub blkio_delay_max_ts: __kernel_timespec,
    pub swapin_delay_max_ts: __kernel_timespec,
    pub freepages_delay_max_ts: __kernel_timespec,
    pub thrashing_delay_max_ts: __kernel_timespec,
    pub compact_delay_max_ts: __kernel_timespec,
    pub wpcopy_delay_max_ts: __kernel_timespec,
    pub irq_delay_max_ts: __kernel_timespec,
}

/*
 * Commands sent from userspace
 * Not versioned. New commands should only be inserted at the enum's end
 * prior to __TASKSTATS_CMD_MAX
 */

pub const TASKSTATS_CMD_UNSPEC: u32 = 0; /* Reserved */
pub const TASKSTATS_CMD_GET: u32 = 1; /* user->kernel request/get-response */
pub const TASKSTATS_CMD_NEW: u32 = 2; /* kernel->user event */
pub const __TASKSTATS_CMD_MAX: u32 = 3;

pub const TASKSTATS_CMD_MAX: u32 = __TASKSTATS_CMD_MAX - 1;

pub const TASKSTATS_TYPE_UNSPEC: u32 = 0; /* Reserved */
pub const TASKSTATS_TYPE_PID: u32 = 1; /* Process id */
pub const TASKSTATS_TYPE_TGID: u32 = 2; /* Thread group id */
pub const TASKSTATS_TYPE_STATS: u32 = 3; /* taskstats structure */
pub const TASKSTATS_TYPE_AGGR_PID: u32 = 4; /* contains pid + stats */
pub const TASKSTATS_TYPE_AGGR_TGID: u32 = 5; /* contains tgid + stats */
pub const TASKSTATS_TYPE_NULL: u32 = 6; /* contains nothing */
pub const __TASKSTATS_TYPE_MAX: u32 = 7;

pub const TASKSTATS_TYPE_MAX: u32 = __TASKSTATS_TYPE_MAX - 1;

pub const TASKSTATS_CMD_ATTR_UNSPEC: u32 = 0;
pub const TASKSTATS_CMD_ATTR_PID: u32 = 1;
pub const TASKSTATS_CMD_ATTR_TGID: u32 = 2;
pub const TASKSTATS_CMD_ATTR_REGISTER_CPUMASK: u32 = 3;
pub const TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK: u32 = 4;
pub const __TASKSTATS_CMD_ATTR_MAX: u32 = 5;

pub const TASKSTATS_CMD_ATTR_MAX: u32 = __TASKSTATS_CMD_ATTR_MAX - 1;

/* NETLINK_GENERIC related info */

pub const TASKSTATS_GENL_NAME: &[u8; 10] = b"TASKSTATS\0";
pub const TASKSTATS_GENL_VERSION: u32 = 0x1;

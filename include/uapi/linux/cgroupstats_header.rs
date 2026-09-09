/* SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note */
/* cgroupstats.h - exporting per-cgroup statistics
 *
 * Copyright IBM Corporation, 2007
 * Author Balbir Singh <balbir@linux.vnet.ibm.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 */

// Dependencies supplied by the surrounding Linux UAPI translation:
// `__TASKSTATS_CMD_MAX` is defined by linux/taskstats.h.

/*
 * Data shared between user space and kernel space on a per cgroup
 * basis. This data is shared using taskstats.
 *
 * Most of these states are derived by looking at the task->state value
 *
 * Each member is aligned to a 8 byte boundary.
 */
#[repr(C)]
pub struct cgroupstats {
    pub nr_sleeping: u64,        /* Number of tasks sleeping */
    pub nr_running: u64,         /* Number of tasks running */
    pub nr_stopped: u64,         /* Number of tasks in stopped state */
    pub nr_uninterruptible: u64, /* Number of tasks in uninterruptible */
                                 /* state */
    pub nr_io_wait: u64,         /* Number of tasks waiting on IO */
}

/*
 * Commands sent from userspace
 * Not versioned. New commands should only be inserted at the enum's end
 * prior to __CGROUPSTATS_CMD_MAX
 */
pub const CGROUPSTATS_CMD_UNSPEC: u32 = __TASKSTATS_CMD_MAX;
pub const CGROUPSTATS_CMD_GET: u32 = CGROUPSTATS_CMD_UNSPEC + 1;
pub const CGROUPSTATS_CMD_NEW: u32 = CGROUPSTATS_CMD_GET + 1;
pub const __CGROUPSTATS_CMD_MAX: u32 = CGROUPSTATS_CMD_NEW + 1;

pub const CGROUPSTATS_CMD_MAX: u32 = __CGROUPSTATS_CMD_MAX - 1;

pub const CGROUPSTATS_TYPE_UNSPEC: u32 = 0; /* Reserved */
pub const CGROUPSTATS_TYPE_CGROUP_STATS: u32 = CGROUPSTATS_TYPE_UNSPEC + 1;
pub const __CGROUPSTATS_TYPE_MAX: u32 = CGROUPSTATS_TYPE_CGROUP_STATS + 1;

pub const CGROUPSTATS_TYPE_MAX: u32 = __CGROUPSTATS_TYPE_MAX - 1;

pub const CGROUPSTATS_CMD_ATTR_UNSPEC: u32 = 0;
pub const CGROUPSTATS_CMD_ATTR_FD: u32 = CGROUPSTATS_CMD_ATTR_UNSPEC + 1;
pub const __CGROUPSTATS_CMD_ATTR_MAX: u32 = CGROUPSTATS_CMD_ATTR_FD + 1;

pub const CGROUPSTATS_CMD_ATTR_MAX: u32 = __CGROUPSTATS_CMD_ATTR_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
/*
 * task_io_accounting: a structure which is used for recording a single task's
 * IO statistics.
 *
 * Don't include this header file directly - it is designed to be dragged in via
 * sched.h.
 *
 * Blame Andrew Morton for all this.
 */

#[repr(C)]
pub struct task_io_accounting {
    #[cfg(feature = "CONFIG_TASK_XACCT")]
    /* bytes read */
    pub rchar: u64,
    #[cfg(feature = "CONFIG_TASK_XACCT")]
    /*  bytes written */
    pub wchar: u64,
    #[cfg(feature = "CONFIG_TASK_XACCT")]
    /* # of read syscalls */
    pub syscr: u64,
    #[cfg(feature = "CONFIG_TASK_XACCT")]
    /* # of write syscalls */
    pub syscw: u64,

    #[cfg(feature = "CONFIG_TASK_IO_ACCOUNTING")]
    /*
     * The number of bytes which this task has caused to be read from
     * storage.
     */
    pub read_bytes: u64,

    #[cfg(feature = "CONFIG_TASK_IO_ACCOUNTING")]
    /*
     * The number of bytes which this task has caused, or shall cause to be
     * written to disk.
     */
    pub write_bytes: u64,

    #[cfg(feature = "CONFIG_TASK_IO_ACCOUNTING")]
    /*
     * A task can cause "negative" IO too.  If this task truncates some
     * dirty pagecache, some IO which another task has been accounted for
     * (in its write_bytes) will not be happening.  We _could_ just
     * subtract that from the truncating task's write_bytes, but there is
     * information loss in doing that.
     */
    pub cancelled_write_bytes: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

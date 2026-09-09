/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: this header uses Linux fixed-width types and loff_t.

/*
 * The following constants define the amount of time given a user
 * before the soft limits are treated as hard limits (usually resulting
 * in an allocation failure). The timer is started when the user crosses
 * their soft limit, it is reset when they go below their soft limit.
 */
pub const MAX_IQ_TIME: i32 = 604800; /* (7*24*60*60) 1 week */
pub const MAX_DQ_TIME: i32 = 604800; /* (7*24*60*60) 1 week */

/*
 * The following structure defines the format of the disk quota file
 * (as it appears on disk) - the file is an array of these structures
 * indexed by user or group number.
 */
#[repr(C)]
pub struct v1_disk_dqblk {
    pub dqb_bhardlimit: u32, /* absolute limit on disk blks alloc */
    pub dqb_bsoftlimit: u32, /* preferred limit on disk blks */
    pub dqb_curblocks: u32,  /* current block count */
    pub dqb_ihardlimit: u32, /* absolute limit on allocated inodes */
    pub dqb_isoftlimit: u32, /* preferred inode limit */
    pub dqb_curinodes: u32,  /* current # allocated inodes */

    /* below fields differ in length on 32-bit vs 64-bit architectures */
    pub dqb_btime: core::ffi::c_ulong, /* time limit for excessive disk use */
    pub dqb_itime: core::ffi::c_ulong, /* time limit for excessive inode use */
}

#[macro_export]
macro_rules! v1_dqoff {
    ($uid:expr) => {
        (($uid as i64) * (core::mem::size_of::<$crate::v1_disk_dqblk>() as i64))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

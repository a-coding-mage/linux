/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/* This file just defines debugging masks to be used with the gossip
 * logging utility.  All debugging masks for ORANGEFS are kept here to make
 * sure we don't have collisions.
 */

/*
 * In the kernel build, __u64 and ARRAY_SIZE are supplied by the kernel
 * headers.  In the non-kernel build, ARRAY_SIZE has the following meaning:
 * sizeof(arr) / sizeof((arr)[0]).
 */

pub const GOSSIP_NO_DEBUG: u64 = 0;

pub const GOSSIP_SUPER_DEBUG: u64 = 1u64 << 0;
pub const GOSSIP_INODE_DEBUG: u64 = 1u64 << 1;
pub const GOSSIP_FILE_DEBUG: u64 = 1u64 << 2;
pub const GOSSIP_DIR_DEBUG: u64 = 1u64 << 3;
pub const GOSSIP_UTILS_DEBUG: u64 = 1u64 << 4;
pub const GOSSIP_WAIT_DEBUG: u64 = 1u64 << 5;
pub const GOSSIP_ACL_DEBUG: u64 = 1u64 << 6;
pub const GOSSIP_DCACHE_DEBUG: u64 = 1u64 << 7;
pub const GOSSIP_DEV_DEBUG: u64 = 1u64 << 8;
pub const GOSSIP_NAME_DEBUG: u64 = 1u64 << 9;
pub const GOSSIP_BUFMAP_DEBUG: u64 = 1u64 << 10;
pub const GOSSIP_CACHE_DEBUG: u64 = 1u64 << 11;
pub const GOSSIP_DEBUGFS_DEBUG: u64 = 1u64 << 12;
pub const GOSSIP_XATTR_DEBUG: u64 = 1u64 << 13;
pub const GOSSIP_INIT_DEBUG: u64 = 1u64 << 14;
pub const GOSSIP_SYSFS_DEBUG: u64 = 1u64 << 15;

pub const GOSSIP_MAX_NR: usize = 16;
pub const GOSSIP_MAX_DEBUG: u64 = (1u64 << GOSSIP_MAX_NR) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/*
 *  linux/fs/hfs/hfs.h
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 */

// Dependency supplied by linux/hfs_common.h.

/*======== Data structures kept in memory ========*/

#[repr(C)]
pub struct hfs_readdir_data {
    pub pos: loff_t,
    pub key: hfs_cat_key,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

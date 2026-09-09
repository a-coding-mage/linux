/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 IBM Corporation
 * Author: Mimi Zohar <zohar@us.ibm.com>
 */

// Dependencies supplied by the corresponding filesystem and iversion modules
// are intentionally referenced but not implemented here.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum integrity_status {
    INTEGRITY_PASS = 0,
    INTEGRITY_PASS_IMMUTABLE,
    INTEGRITY_FAIL,
    INTEGRITY_FAIL_IMMUTABLE,
    INTEGRITY_NOLABEL,
    INTEGRITY_NOXATTRS,
    INTEGRITY_UNKNOWN,
}

#[cfg(CONFIG_INTEGRITY)]
unsafe extern "C" {
    pub fn integrity_load_keys();
}

#[cfg(not(CONFIG_INTEGRITY))]
#[inline]
pub fn integrity_load_keys() {}

/* An inode's attributes for detection of changes */
#[repr(C)]
pub struct integrity_inode_attributes {
    pub version: u64, /* track inode changes */
    pub ino: libc::c_ulong,
    pub dev: dev_t,
}

/*
 * On stacked filesystems the i_version alone is not enough to detect file data
 * or metadata change. Additional metadata is required.
 */
#[inline]
pub unsafe fn integrity_inode_attrs_store(
    attrs: *mut integrity_inode_attributes,
    i_version: u64,
    inode: *const inode,
) {
    (*attrs).version = i_version;
    (*attrs).dev = (*(*inode).i_sb).s_dev;
    (*attrs).ino = (*inode).i_ino;
}

/*
 * On stacked filesystems detect whether the inode or its content has changed.
 */
#[inline]
pub unsafe fn integrity_inode_attrs_changed(
    attrs: *const integrity_inode_attributes,
    inode: *const inode,
) -> bool {
    (*(*inode).i_sb).s_dev != (*attrs).dev
        || (*inode).i_ino != (*attrs).ino
        || !inode_eq_iversion(inode, (*attrs).version)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

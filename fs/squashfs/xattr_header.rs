/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2010
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * xattr.h
 */

/* The CONFIG_SQUASHFS_XATTR build-time condition is preserved here. */
#[cfg(CONFIG_SQUASHFS_XATTR)]
extern "C" {
    pub fn squashfs_read_xattr_id_table(
        sb: *mut super_block,
        start: u64,
        xattr_table_start: *mut u64,
        xattr_ids: *mut std::ffi::c_uint,
    ) -> *mut __le64;
    pub fn squashfs_xattr_lookup(
        sb: *mut super_block,
        index: std::ffi::c_uint,
        count: *mut std::ffi::c_int,
        size: *mut std::ffi::c_uint,
        xattr: *mut std::ffi::c_ulonglong,
    ) -> std::ffi::c_int;
}

#[cfg(not(CONFIG_SQUASHFS_XATTR))]
pub unsafe fn squashfs_read_xattr_id_table(
    sb: *mut super_block,
    start: u64,
    xattr_table_start: *mut u64,
    xattr_ids: *mut std::ffi::c_uint,
) -> *mut __le64 {
    let id_table: *mut squashfs_xattr_id_table;

    id_table = squashfs_read_table(sb, start, std::mem::size_of::<squashfs_xattr_id_table>());
    if IS_ERR(id_table) {
        return id_table as *mut __le64;
    }

    *xattr_table_start = le64_to_cpu((*id_table).xattr_table_start);
    kfree(id_table);

    ERROR!("Xattrs in filesystem, these will be ignored\n");
    ERR_PTR(-ENOTSUPP) as *mut __le64
}

#[cfg(not(CONFIG_SQUASHFS_XATTR))]
pub unsafe fn squashfs_xattr_lookup(
    sb: *mut super_block,
    index: std::ffi::c_uint,
    count: *mut std::ffi::c_int,
    size: *mut std::ffi::c_uint,
    xattr: *mut std::ffi::c_ulonglong,
) -> std::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SQUASHFS_XATTR))]
pub const squashfs_listxattr: *const std::ffi::c_void = std::ptr::null();
#[cfg(not(CONFIG_SQUASHFS_XATTR))]
pub const squashfs_xattr_handlers: *const std::ffi::c_void = std::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

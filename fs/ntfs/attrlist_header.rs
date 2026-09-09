/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Exports for attribute list attribute handling.
 *
 * Copyright (c) 2004 Anton Altaparmakov
 * Copyright (c) 2004 Yura Pakhuchiy
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Dependency declarations supplied by attrib.h and the surrounding crate are
// intentionally not implemented here.

extern "C" {
    pub fn ntfs_attrlist_need(ni: *mut ntfs_inode) -> ::std::os::raw::c_int;
    pub fn ntfs_attrlist_entry_add(
        ni: *mut ntfs_inode,
        attr: *mut attr_record,
    ) -> ::std::os::raw::c_int;
    pub fn ntfs_attrlist_entry_rm(ctx: *mut ntfs_attr_search_ctx) -> ::std::os::raw::c_int;
    pub fn ntfs_attrlist_update(base_ni: *mut ntfs_inode) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

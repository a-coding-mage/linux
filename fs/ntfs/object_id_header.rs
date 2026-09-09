/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2008-2021 Jean-Pierre Andre
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// The C header guard and include context are not executable Rust.

extern "C" {
    pub static mut objid_index_name: [__le16; 0];

    pub fn ntfs_delete_object_id_index(ni: *mut ntfs_inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for NTFS kernel collation handling.
 *
 * Copyright (c) 2004 Anton Altaparmakov
 *
 * Part of this file is based on code from the NTFS-3G.
 * and is copyrighted by the respective authors below:
 * Copyright (c) 2004 Anton Altaparmakov
 * Copyright (c) 2005 Yura Pakhuchiy
 */

// Dependency declarations and constants are supplied by the translated volume header.

#[inline]
pub fn ntfs_is_collation_rule_supported(cr: __le32) -> bool {
    let i: i32;

    if ((cr != COLLATION_BINARY && cr != COLLATION_NTOFS_ULONG &&
        cr != COLLATION_FILE_NAME) && cr != COLLATION_NTOFS_ULONGS)
    {
        return false;
    }
    i = le32_to_cpu(cr) as i32;
    if (((i >= 0) && (i <= 0x02)) || ((i >= 0x10) && (i <= 0x13))) {
        return true;
    }
    false
}

extern "C" {
    pub fn ntfs_collate(
        vol: *mut ntfs_volume,
        cr: __le32,
        data1: *const core::ffi::c_void,
        data1_len: u32,
        data2: *const core::ffi::c_void,
        data2_len: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

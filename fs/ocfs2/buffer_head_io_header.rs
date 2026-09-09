/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ocfs2_buffer_head.h
 *
 * Buffer cache handling functions defined
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux dependency: <linux/buffer_head.h>

pub const OCFS2_BH_IGNORE_CACHE: i32 = 1;
pub const OCFS2_BH_READAHEAD: i32 = 8;

extern "C" {
    pub fn ocfs2_write_block(
        osb: *mut ocfs2_super,
        bh: *mut buffer_head,
        ci: *mut ocfs2_caching_info,
    ) -> i32;

    pub fn ocfs2_read_blocks_sync(
        osb: *mut ocfs2_super,
        block: u64,
        nr: u32,
        bhs: *mut *mut buffer_head,
    ) -> i32;

    /*
     * If not NULL, validate() will be called on a buffer that is freshly
     * read from disk.  It will not be called if the buffer was in cache.
     * Note that if validate() is being used for this buffer, it needs to
     * be set even for a READAHEAD call, as it marks the buffer for later
     * validation.
     */
    pub fn ocfs2_read_blocks(
        ci: *mut ocfs2_caching_info,
        block: u64,
        nr: i32,
        bhs: *mut *mut buffer_head,
        flags: i32,
        validate: Option<unsafe extern "C" fn(*mut super_block, *mut buffer_head) -> i32>,
    ) -> i32;

    pub fn ocfs2_write_super_or_backup(
        osb: *mut ocfs2_super,
        bh: *mut buffer_head,
    ) -> i32;
}

// Opaque declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct ocfs2_super {
    _private: [u8; 0],
}

#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocfs2_caching_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

pub unsafe extern "C" fn ocfs2_read_block(
    ci: *mut ocfs2_caching_info,
    off: u64,
    bh: *mut *mut buffer_head,
    validate: Option<unsafe extern "C" fn(*mut super_block, *mut buffer_head) -> i32>,
) -> i32 {
    let mut status: i32 = 0;

    if bh.is_null() {
        printk("ocfs2: bh == NULL\n".as_ptr());
        status = -EINVAL;
        return status;
    }

    status = ocfs2_read_blocks(ci, off, 1, bh, 0, validate);

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

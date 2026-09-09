// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * id.c
 */

/*
 * This file implements code to handle uids and gids.
 *
 * For space efficiency regular files store uid and gid indexes, which are
 * converted to 32-bit uids/gids using an id look up table.  This table is
 * stored compressed into metadata blocks.  A second index table is used to
 * locate these.  This second index table for speed of access (and because it
 * is small) is read at mount time and cached in memory.
 */

// Linux and Squashfs declarations supplied by the surrounding translation.

#[repr(C)]
pub struct SuperBlock {
    pub s_fs_info: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct SquashfsSbInfo {
    pub ids: u32,
    pub id_table: *mut u64,
}

pub type Le32 = u32;
pub type Le64 = u64;

extern "C" {
    pub fn squashfs_read_metadata(
        sb: *mut SuperBlock,
        buffer: *mut core::ffi::c_void,
        start_block: *mut u64,
        offset: *mut i32,
        length: usize,
    ) -> i32;
    pub fn squashfs_read_table(
        sb: *mut SuperBlock,
        start_block: u64,
        length: u32,
    ) -> *mut Le64;
    pub fn kfree(object: *mut core::ffi::c_void);
    pub fn err_ptr(error: i32) -> *mut Le64;
}

// These correspond to the included Squashfs macros and endian helpers.
extern "C" {
    pub fn squashfs_id_block(index: u32) -> i32;
    pub fn squashfs_id_block_offset(index: u32) -> i32;
    pub fn squashfs_id_block_bytes(no_ids: u16) -> u32;
    pub fn squashfs_id_blocks(no_ids: u16) -> u32;
    pub fn le32_to_cpu(value: Le32) -> u32;
    pub fn le64_to_cpu(value: Le64) -> u64;
}

pub const EINVAL: i32 = 22;
pub const SQUASHFS_METADATA_SIZE: u64 = 8192;
pub const SQUASHFS_BLOCK_OFFSET: u64 = 0xffff;

/*
 * Map uid/gid index into real 32-bit uid/gid using the id look up table
 */
pub unsafe fn squashfs_get_id(
    sb: *mut SuperBlock,
    index: u32,
    id: *mut u32,
) -> i32 {
    let msblk = (*sb).s_fs_info as *mut SquashfsSbInfo;
    let block = squashfs_id_block(index);
    let mut offset = squashfs_id_block_offset(index);
    let mut start_block: u64;
    let mut disk_id: Le32 = 0;
    let err: i32;

    if index >= (*msblk).ids {
        return -EINVAL;
    }

    start_block = le64_to_cpu(*(*msblk).id_table.add(block as usize));

    err = squashfs_read_metadata(
        sb,
        (&mut disk_id as *mut Le32).cast(),
        &mut start_block,
        &mut offset,
        core::mem::size_of::<Le32>(),
    );
    if err < 0 {
        return err;
    }

    *id = le32_to_cpu(disk_id);
    0
}

/*
 * Read uncompressed id lookup table indexes from disk into memory
 */
pub unsafe fn squashfs_read_id_index_table(
    sb: *mut SuperBlock,
    id_table_start: u64,
    next_table: u64,
    no_ids: u16,
) -> *mut Le64 {
    let length = squashfs_id_block_bytes(no_ids);
    let indexes = squashfs_id_blocks(no_ids);
    let mut n: i32;
    let table: *mut Le64;
    let mut start: u64;
    let mut end: u64;

    // TRACE("In read_id_index_table, length %d\n", length);

    /* Sanity check values */

    /* there should always be at least one id */
    if no_ids == 0 {
        return err_ptr(-EINVAL);
    }

    /*
     * The computed size of the index table (length bytes) should exactly
     * match the table start and end points
     */
    if (length as u64 != next_table - id_table_start) {
        return err_ptr(-EINVAL);
    }

    table = squashfs_read_table(sb, id_table_start, length);
    if table.is_null() {
        return table;
    }

    /*
     * table[0], table[1], ... table[indexes - 1] store the locations
     * of the compressed id blocks.   Each entry should be less than
     * the next (i.e. table[0] < table[1]), and the difference between them
     * should be SQUASHFS_METADATA_SIZE or less.  table[indexes - 1]
     * should be less than id_table_start, and again the difference
     * should be SQUASHFS_METADATA_SIZE or less
     */
    n = 0;
    while n < (indexes as i32 - 1) {
        start = le64_to_cpu(*table.add(n as usize));
        end = le64_to_cpu(*table.add((n + 1) as usize));

        if start >= end
            || (end - start) > (SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET)
        {
            kfree(table.cast());
            return err_ptr(-EINVAL);
        }
        n += 1;
    }

    start = le64_to_cpu(*table.add((indexes - 1) as usize));
    if start >= id_table_start
        || (id_table_start - start) > (SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET)
    {
        kfree(table.cast());
        return err_ptr(-EINVAL);
    }

    table
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

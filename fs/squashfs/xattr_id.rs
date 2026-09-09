// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2010
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * xattr_id.c
 */

/*
 * This file implements code to map the 32-bit xattr id stored in the inode
 * into the on disk location of the xattr data.
 */

// Linux and Squashfs types, constants, macros, and external functions are
// supplied by the corresponding translated dependency files.

/*
 * Map xattr id using the xattr id look up table
 */
pub unsafe fn squashfs_xattr_lookup(
    sb: *mut super_block,
    index: ::std::os::raw::c_uint,
    count: *mut ::std::os::raw::c_int,
    size: *mut ::std::os::raw::c_uint,
    xattr: *mut ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let block = SQUASHFS_XATTR_BLOCK(index);
    let mut offset = SQUASHFS_XATTR_BLOCK_OFFSET(index);
    let mut start_block: u64;
    let mut id = ::std::mem::MaybeUninit::<squashfs_xattr_id>::uninit();
    let err: ::std::os::raw::c_int;

    if index >= (*msblk).xattr_ids {
        return -EINVAL;
    }

    start_block = le64_to_cpu((*msblk).xattr_id_table[block as usize]);

    err = squashfs_read_metadata(
        sb,
        id.as_mut_ptr() as *mut ::std::os::raw::c_void,
        &mut start_block,
        &mut offset,
        ::std::mem::size_of::<squashfs_xattr_id>(),
    );
    if err < 0 {
        return err;
    }

    let id = id.assume_init();
    *xattr = le64_to_cpu(id.xattr);
    *size = le32_to_cpu(id.size);
    *count = le32_to_cpu(id.count) as ::std::os::raw::c_int;
    0
}

/*
 * Read uncompressed xattr id lookup table indexes from disk into memory
 */
pub unsafe fn squashfs_read_xattr_id_table(
    sb: *mut super_block,
    table_start: u64,
    xattr_table_start: *mut u64,
    xattr_ids: *mut ::std::os::raw::c_uint,
) -> *mut __le64 {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut len: ::std::os::raw::c_uint;
    let mut indexes: ::std::os::raw::c_uint;
    let id_table = squashfs_read_table(
        sb,
        table_start,
        ::std::mem::size_of::<squashfs_xattr_id_table>(),
    ) as *mut squashfs_xattr_id_table;
    if IS_ERR(id_table as *mut ::std::os::raw::c_void) {
        return id_table as *mut __le64;
    }

    *xattr_table_start = le64_to_cpu((*id_table).xattr_table_start);
    *xattr_ids = le32_to_cpu((*id_table).xattr_ids);
    kfree(id_table as *mut ::std::os::raw::c_void);

    /* Sanity check values */

    /* there is always at least one xattr id */
    if *xattr_ids == 0 {
        return ERR_PTR(-EINVAL) as *mut __le64;
    }

    len = SQUASHFS_XATTR_BLOCK_BYTES(*xattr_ids);
    indexes = SQUASHFS_XATTR_BLOCKS(*xattr_ids);

    /*
     * The computed size of the index table (len bytes) should exactly
     * match the table start and end points
     */
    let mut start = table_start + ::std::mem::size_of::<squashfs_xattr_id_table>() as u64;
    let mut end = (*msblk).bytes_used;

    if len as u64 != end - start {
        return ERR_PTR(-EINVAL) as *mut __le64;
    }

    let table = squashfs_read_table(sb, start, len as usize) as *mut __le64;
    if IS_ERR(table as *mut ::std::os::raw::c_void) {
        return table;
    }

    /* table[0], table[1], ... table[indexes - 1] store the locations
     * of the compressed xattr id blocks.  Each entry should be less than
     * the next (i.e. table[0] < table[1]), and the difference between them
     * should be SQUASHFS_METADATA_SIZE or less.  table[indexes - 1]
     * should be less than table_start, and again the difference
     * shouls be SQUASHFS_METADATA_SIZE or less.
     *
     * Finally xattr_table_start should be less than table[0].
     */
    let mut n: ::std::os::raw::c_int = 0;
    while n < (indexes - 1) as ::std::os::raw::c_int {
        start = le64_to_cpu(*table.add(n as usize));
        end = le64_to_cpu(*table.add((n + 1) as usize));

        if start >= end || end - start > (SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET) as u64 {
            kfree(table as *mut ::std::os::raw::c_void);
            return ERR_PTR(-EINVAL) as *mut __le64;
        }
        n += 1;
    }

    start = le64_to_cpu(*table.add((indexes - 1) as usize));
    if start >= table_start
        || table_start - start > (SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET) as u64
    {
        kfree(table as *mut ::std::os::raw::c_void);
        return ERR_PTR(-EINVAL) as *mut __le64;
    }

    if *xattr_table_start >= le64_to_cpu(*table) {
        kfree(table as *mut ::std::os::raw::c_void);
        return ERR_PTR(-EINVAL) as *mut __le64;
    }

    table
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

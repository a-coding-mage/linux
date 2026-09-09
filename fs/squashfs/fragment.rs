// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * fragment.c
 */

/*
 * This file implements code to handle compressed fragments (tail-end packed
 * datablocks).
 *
 * Regular files contain a fragment index which is mapped to a fragment
 * location on disk and compressed size using a fragment lookup table.
 * Like everything in Squashfs this fragment lookup table is itself stored
 * compressed into metadata blocks.  A second index table is used to locate
 * these.  This second index table for speed of access (and because it
 * is small) is read at mount time and cached in memory.
 */

/* Linux and Squashfs declarations are supplied by the surrounding build. */

/*
 * Look-up fragment using the fragment index table.  Return the on disk
 * location of the fragment and its compressed size
 */
pub unsafe fn squashfs_frag_lookup(
    sb: *mut super_block,
    fragment: ::core::ffi::c_uint,
    fragment_block: *mut u64,
) -> ::core::ffi::c_int {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut block: ::core::ffi::c_int;
    let mut offset: ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int;
    let mut fragment_entry: squashfs_fragment_entry = ::core::mem::zeroed();
    let mut start_block: u64;

    if fragment >= (*msblk).fragments {
        return -EIO;
    }
    block = SQUASHFS_FRAGMENT_INDEX(fragment);
    offset = SQUASHFS_FRAGMENT_INDEX_OFFSET(fragment);

    start_block = le64_to_cpu((*msblk).fragment_index[block as usize]);

    size = squashfs_read_metadata(
        sb,
        &mut fragment_entry as *mut squashfs_fragment_entry,
        &mut start_block as *mut u64,
        &mut offset as *mut ::core::ffi::c_int,
        ::core::mem::size_of::<squashfs_fragment_entry>(),
    );
    if size < 0 {
        return size;
    }

    *fragment_block = le64_to_cpu(fragment_entry.start_block);
    squashfs_block_size(fragment_entry.size)
}

/*
 * Read the uncompressed fragment lookup table indexes off disk into memory
 */
pub unsafe fn squashfs_read_fragment_index_table(
    sb: *mut super_block,
    fragment_table_start: u64,
    next_table: u64,
    fragments: ::core::ffi::c_uint,
) -> *mut __le64 {
    let length: usize = SQUASHFS_FRAGMENT_INDEX_BYTES(fragments) as usize;
    let table: *mut __le64;

    /*
     * Sanity check, length bytes should not extend into the next table -
     * this check also traps instances where fragment_table_start is
     * incorrectly larger than the next table start
     */
    if fragment_table_start + length as u64 > next_table {
        return ERR_PTR(-EINVAL);
    }

    table = squashfs_read_table(sb, fragment_table_start, length);

    /*
     * table[0] points to the first fragment table metadata block, this
     * should be less than fragment_table_start
     */
    if !IS_ERR(table) && le64_to_cpu(*table) >= fragment_table_start {
        kfree(table as *mut ::core::ffi::c_void);
        return ERR_PTR(-EINVAL);
    }

    table
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

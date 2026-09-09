// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * symlink.c
 */

/*
 * This file implements code to handle symbolic links.
 *
 * The data contents of symbolic links are stored inside the symbolic
 * link inode within the inode table.  This allows the normally small
 * symbolic link to be compressed as part of the inode table, achieving
 * much greater compression than if the symbolic link was compressed
 * individually.
 */

// Linux and Squashfs declarations are supplied by the surrounding crate.

unsafe fn squashfs_symlink_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let inode = (*(*folio).mapping).host;
    let sb = (*inode).i_sb;
    let msblk = (*sb).s_fs_info;
    let index = folio_pos(folio) as i32;
    let mut block = squashfs_i(inode).start;
    let mut offset = squashfs_i(inode).offset;
    let length = std::cmp::min(i_size_read(inode) as i32 - index, PAGE_SIZE as i32);
    let mut bytes: i32;
    let mut copied: i32;
    let mut error: i32;
    let mut pageaddr: *mut u8;
    let mut entry: *mut squashfs_cache_entry;

    TRACE!(
        "Entered squashfs_symlink_readpage, page index %ld, start block %llx, offset %x\n",
        (*folio).index,
        block,
        offset
    );

    /*
     * Skip index bytes into symlink metadata.
     */
    if index != 0 {
        bytes = squashfs_read_metadata(sb, std::ptr::null_mut(), &mut block, &mut offset, index);
        if bytes < 0 {
            ERROR!(
                "Unable to read symlink [%llx:%x]\n",
                squashfs_i(inode).start,
                squashfs_i(inode).offset
            );
            error = bytes;
            goto_out(folio, error);
            return error;
        }
    }

    /*
     * Read length bytes from symlink metadata.  squashfs_read_metadata
     * is not used here because it can sleep and we want to use
     * kmap_local to map the folio.  Instead call the underlying
     * squashfs_cache_get routine.  As length bytes may overlap metadata
     * blocks, we may need to call squashfs_cache_get multiple times.
     */
    bytes = 0;
    while bytes < length {
        entry = squashfs_cache_get(sb, (*msblk).block_cache, block, 0);
        if (*entry).error != 0 {
            ERROR!(
                "Unable to read symlink [%llx:%x]\n",
                squashfs_i(inode).start,
                squashfs_i(inode).offset
            );
            squashfs_cache_put(entry);
            error = (*entry).error;
            goto_out(folio, error);
            return error;
        }

        pageaddr = kmap_local_folio(folio, 0);
        copied = squashfs_copy_data(pageaddr.add(bytes as usize), entry, offset, length - bytes);
        if copied == length - bytes {
            std::ptr::write_bytes(
                pageaddr.add(length as usize),
                0,
                PAGE_SIZE as usize - length as usize,
            );
        } else {
            block = (*entry).next_index;
        }
        kunmap_local(pageaddr);
        squashfs_cache_put(entry);
        offset = 0;
        bytes += copied;
    }

    flush_dcache_folio(folio);
    error = 0;
    goto_out(folio, error);
    error
}

// The surrounding kernel translation provides the C-compatible declarations,
// structures, constants, logging macros, and the out-label helper used above.
pub static squashfs_symlink_aops: address_space_operations = address_space_operations {
    read_folio: Some(squashfs_symlink_read_folio),
};

pub static squashfs_symlink_inode_ops: inode_operations = inode_operations {
    get_link: Some(page_get_link),
    listxattr: Some(squashfs_listxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

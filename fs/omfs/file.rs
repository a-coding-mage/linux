// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMFS (as used by RIO Karma) file operations.
 * Copyright (C) 2005 Bob Copeland <me@bobcopeland.com>
 */

// Linux kernel dependencies and `omfs.h` are supplied by the surrounding crate.

unsafe fn omfs_max_extents(sbi: *mut omfs_sb_info, offset: i32) -> u32 {
    ((*sbi).s_sys_blocksize - offset as u32 - core::mem::size_of::<omfs_extent>() as u32)
        / core::mem::size_of::<omfs_extent_entry>() as u32
}

pub unsafe fn omfs_make_empty_table(bh: *mut buffer_head, offset: i32) {
    let oe = ( (*bh).b_data.add(offset as usize) ) as *mut omfs_extent;
    (*oe).e_next = !cpu_to_be64(0);
    (*oe).e_extent_count = cpu_to_be32(1);
    (*oe).e_fill = cpu_to_be32(0x22);
    (*oe).e_entry[0].e_cluster = !cpu_to_be64(0);
    (*oe).e_entry[0].e_blocks = !cpu_to_be64(0);
}

pub unsafe fn omfs_shrink_inode(inode: *mut inode) -> i32 {
    let sbi = OMFS_SB((*inode).i_sb);
    let mut oe: *mut omfs_extent;
    let mut entry: *mut omfs_extent_entry;
    let mut bh: *mut buffer_head;
    let mut next: u64;
    let mut last: u64;
    let mut extent_count: u32;
    let mut max_extents: u32;
    let mut ret: i32;
    next = (*inode).i_ino;
    ret = -EIO;
    if (*inode).i_size != 0 { return ret; }
    bh = omfs_bread((*inode).i_sb, next);
    if bh.is_null() { return ret; }
    oe = (*bh).b_data.add(OMFS_EXTENT_START as usize) as *mut omfs_extent;
    max_extents = omfs_max_extents(sbi, OMFS_EXTENT_START);
    loop {
        if omfs_is_bad(sbi, (*bh).b_data as *mut omfs_header, next) { brelse(bh); return ret; }
        extent_count = be32_to_cpu((*oe).e_extent_count);
        if extent_count > max_extents { brelse(bh); return ret; }
        last = next;
        next = be64_to_cpu((*oe).e_next);
        entry = (*oe).e_entry.as_mut_ptr();
        let mut count = extent_count;
        while count > 1 {
            let start = be64_to_cpu((*entry).e_cluster);
            let blocks = be64_to_cpu((*entry).e_blocks);
            omfs_clear_range((*inode).i_sb, start, blocks as i32);
            entry = entry.add(1);
            count -= 1;
        }
        omfs_make_empty_table(bh, (oe as usize - (*bh).b_data as usize) as i32);
        mark_buffer_dirty(bh);
        brelse(bh);
        if last != (*inode).i_ino { omfs_clear_range((*inode).i_sb, last, (*sbi).s_mirrors as i32); }
        if next == !0 { break; }
        bh = omfs_bread((*inode).i_sb, next);
        if bh.is_null() { return ret; }
        oe = (*bh).b_data.add(OMFS_EXTENT_CONT as usize) as *mut omfs_extent;
        max_extents = omfs_max_extents(sbi, OMFS_EXTENT_CONT);
    }
    ret = 0;
    ret
}

unsafe fn omfs_truncate(inode: *mut inode) {
    omfs_shrink_inode(inode);
    mark_inode_dirty(inode);
}

/*
 * Add new blocks to the current extent, or create new entries/continuations
 * as necessary.
 */
unsafe fn omfs_grow_extent(inode: *mut inode, oe: *mut omfs_extent, ret_block: *mut u64) -> i32 {
    let mut entry = (*oe).e_entry.as_mut_ptr();
    let sbi = OMFS_SB((*inode).i_sb);
    let extent_count = be32_to_cpu((*oe).e_extent_count);
    let mut new_block: u64 = 0;
    let max_count: u32;
    let mut new_count: i32 = 0;
    let mut ret = 0;
    if extent_count < 1 { return -EIO; }
    let terminator = entry.add(extent_count as usize - 1);
    if extent_count > 1 {
        entry = terminator.sub(1);
        new_block = be64_to_cpu((*entry).e_cluster) + be64_to_cpu((*entry).e_blocks);
        if omfs_allocate_block((*inode).i_sb, new_block) {
            be64_add_cpu(&mut (*entry).e_blocks, 1);
            (*terminator).e_blocks = !(cpu_to_be64(be64_to_cpu(!(*terminator).e_blocks) + 1));
            *ret_block = new_block;
            return ret;
        }
    }
    max_count = omfs_max_extents(sbi, OMFS_EXTENT_START);
    if be32_to_cpu((*oe).e_extent_count) > max_count - 1 { return -EIO; }
    ret = omfs_allocate_range((*inode).i_sb, 1, (*sbi).s_clustersize, &mut new_block, &mut new_count);
    if ret != 0 { return ret; }
    entry = terminator;
    let new_terminator = terminator.add(1);
    core::ptr::copy_nonoverlapping(entry, new_terminator, 1);
    (*entry).e_cluster = cpu_to_be64(new_block);
    (*entry).e_blocks = cpu_to_be64(new_count as u64);
    (*new_terminator).e_blocks = !(cpu_to_be64(be64_to_cpu(!(*new_terminator).e_blocks) + new_count as u64));
    be32_add_cpu(&mut (*oe).e_extent_count, 1);
    *ret_block = new_block;
    ret
}

/* Scans across the directory table for a given file block number. */
unsafe fn find_block(inode: *mut inode, mut ent: *mut omfs_extent_entry, block: sector_t, mut count: i32, left: *mut i32) -> sector_t {
    let mut searched: sector_t = 0;
    while count > 1 {
        let numblocks = clus_to_blk(OMFS_SB((*inode).i_sb), be64_to_cpu((*ent).e_blocks));
        if block >= searched && block < searched + numblocks {
            *left = (numblocks - (block - searched)) as i32;
            return clus_to_blk(OMFS_SB((*inode).i_sb), be64_to_cpu((*ent).e_cluster)) + block - searched;
        }
        searched += numblocks;
        ent = ent.add(1);
        count -= 1;
    }
    0
}

unsafe fn omfs_get_block(inode: *mut inode, block: sector_t, bh_result: *mut buffer_head, create: i32) -> i32 {
    let mut bh = omfs_bread((*inode).i_sb, (*inode).i_ino);
    let mut ret = -EIO;
    if bh.is_null() { return ret; }
    let sbi = OMFS_SB((*inode).i_sb);
    let mut oe = (*bh).b_data.add(OMFS_EXTENT_START as usize) as *mut omfs_extent;
    let mut max_extents = omfs_max_extents(sbi, OMFS_EXTENT_START);
    let mut next = (*inode).i_ino;
    let max_blocks = (*bh_result).b_size >> (*inode).i_blkbits;
    loop {
        if omfs_is_bad(sbi, (*bh).b_data as *mut omfs_header, next) { brelse(bh); return ret; }
        let extent_count = be32_to_cpu((*oe).e_extent_count);
        next = be64_to_cpu((*oe).e_next);
        if extent_count > max_extents { brelse(bh); return ret; }
        let mut remain = 0;
        let offset = find_block(inode, (*oe).e_entry.as_mut_ptr(), block, extent_count as i32, &mut remain);
        if offset > 0 {
            ret = 0; map_bh(bh_result, (*inode).i_sb, offset);
            if remain as u32 > max_blocks { remain = max_blocks as i32; }
            (*bh_result).b_size = (remain as u32) << (*inode).i_blkbits;
            brelse(bh); return ret;
        }
        if next == !0 { break; }
        brelse(bh); bh = omfs_bread((*inode).i_sb, next);
        if bh.is_null() { return ret; }
        oe = (*bh).b_data.add(OMFS_EXTENT_CONT as usize) as *mut omfs_extent;
        max_extents = omfs_max_extents(sbi, OMFS_EXTENT_CONT);
    }
    if create != 0 {
        let mut new_block = 0;
        ret = omfs_grow_extent(inode, oe, &mut new_block);
        if ret == 0 { mark_buffer_dirty(bh); mark_inode_dirty(inode); map_bh(bh_result, (*inode).i_sb, clus_to_blk(sbi, new_block)); }
    }
    brelse(bh); ret
}

unsafe fn omfs_read_folio(file: *mut file, folio: *mut folio) -> i32 { block_read_full_folio(folio, omfs_get_block) }
unsafe fn omfs_readahead(rac: *mut readahead_control) { mpage_readahead(rac, omfs_get_block); }
unsafe fn omfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 { mpage_writepages(mapping, wbc, omfs_get_block) }
unsafe fn omfs_write_failed(mapping: *mut address_space, to: loff_t) { let inode = (*mapping).host; if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); omfs_truncate(inode); } }
unsafe fn omfs_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: u32, foliop: *mut *mut folio, fsdata: *mut *mut core::ffi::c_void) -> i32 { let ret = block_write_begin(mapping, pos, len, foliop, omfs_get_block); if unlikely(ret != 0) { omfs_write_failed(mapping, pos + len as i64); } ret }
unsafe fn omfs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t { generic_block_bmap(mapping, block, omfs_get_block) }

pub static omfs_file_operations: file_operations = file_operations {
    llseek: generic_file_llseek, read_iter: generic_file_read_iter, write_iter: generic_file_write_iter,
    mmap_prepare: generic_file_mmap_prepare, fsync: simple_fsync, splice_read: filemap_splice_read,
};

unsafe fn omfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 {
    let inode = d_inode(dentry);
    let mut error = setattr_prepare(&nop_mnt_idmap, dentry, attr);
    if error != 0 { return error; }
    if ((*attr).ia_valid & ATTR_SIZE) != 0 && (*attr).ia_size != i_size_read(inode) {
        error = inode_newsize_ok(inode, (*attr).ia_size); if error != 0 { return error; }
        truncate_setsize(inode, (*attr).ia_size); omfs_truncate(inode);
    }
    setattr_copy(&nop_mnt_idmap, inode, attr); mark_inode_dirty(inode); 0
}

pub static omfs_file_inops: inode_operations = inode_operations { setattr: omfs_setattr };
pub static omfs_aops: address_space_operations = address_space_operations {
    dirty_folio: block_dirty_folio, invalidate_folio: block_invalidate_folio, read_folio: omfs_read_folio,
    readahead: omfs_readahead, writepages: omfs_writepages, write_begin: omfs_write_begin,
    write_end: generic_write_end, bmap: omfs_bmap, migrate_folio: buffer_migrate_folio,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/file.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  file VFS functions
 */

// Dependencies are supplied by the surrounding kernel translation.

#[inline]
fn blocks(size: u64) -> u64 { (size + 511) >> 9 }

unsafe fn hpfs_file_release(inode: *mut inode, _file: *mut file) -> i32 {
    hpfs_lock((*inode).i_sb);
    hpfs_write_if_changed(inode);
    hpfs_unlock((*inode).i_sb);
    0
}

unsafe fn hpfs_file_fsync(file: *mut file, start: loff_t, end: loff_t, _datasync: i32) -> i32 {
    let inode = (*(*file).f_mapping).host;
    let ret = file_write_and_wait_range(file, start, end);
    if ret != 0 { return ret; }
    sync_blockdev((*(*inode).i_sb).s_bdev)
}

/* generic_file_read often calls bmap with non-existing sector,
 * so we must ignore such errors. */
unsafe fn hpfs_bmap(inode: *mut inode, file_secno: u32, n_secs: *mut u32) -> secno {
    let hpfs_inode = hpfs_i(inode);
    let mut n: u32;
    let mut disk_secno: secno;
    let mut fnode: *mut fnode;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    if blocks((*hpfs_i(inode)).mmu_private as u64) <= file_secno as u64 { return 0; }
    n = file_secno - (*hpfs_inode).i_file_sec;
    if n < (*hpfs_inode).i_n_secs {
        *n_secs = (*hpfs_inode).i_n_secs - n;
        return (*hpfs_inode).i_disk_sec + n;
    }
    fnode = hpfs_map_fnode((*inode).i_sb, (*inode).i_ino, &mut bh);
    if fnode.is_null() { return 0; }
    disk_secno = hpfs_bplus_lookup((*inode).i_sb, inode, GET_BTREE_PTR(&mut (*fnode).btree), file_secno, bh);
    if disk_secno == (-1i32 as secno) { return 0; }
    if hpfs_chk_sectors((*inode).i_sb, disk_secno, 1, "bmap") != 0 { return 0; }
    n = file_secno - (*hpfs_inode).i_file_sec;
    if n < (*hpfs_inode).i_n_secs {
        *n_secs = (*hpfs_inode).i_n_secs - n;
        return (*hpfs_inode).i_disk_sec + n;
    }
    *n_secs = 1;
    disk_secno
}

unsafe fn hpfs_truncate(i: *mut inode) {
    if IS_IMMUTABLE(i) { return; }
    hpfs_lock_assert((*i).i_sb);
    (*hpfs_i(i)).i_n_secs = 0;
    (*i).i_blocks = 1 + (((*i).i_size + 511) >> 9);
    (*hpfs_i(i)).mmu_private = (*i).i_size;
    hpfs_truncate_btree((*i).i_sb, (*i).i_ino, 1, ((*i).i_size + 511) >> 9);
    hpfs_write_inode(i);
    (*hpfs_i(i)).i_n_secs = 0;
}

unsafe fn hpfs_get_block(inode: *mut inode, iblock: sector_t, bh_result: *mut buffer_head, create: i32) -> i32 {
    let mut r: i32;
    let mut s: secno;
    let mut n_secs: u32 = 0;
    hpfs_lock((*inode).i_sb);
    s = hpfs_bmap(inode, iblock, &mut n_secs);
    if s != 0 {
        if (*bh_result).b_size >> 9 < n_secs { n_secs = (*bh_result).b_size >> 9; }
        n_secs = hpfs_search_hotfix_map_for_range((*inode).i_sb, s, n_secs);
        if n_secs == 0 { s = hpfs_search_hotfix_map((*inode).i_sb, s); n_secs = 1; }
        map_bh(bh_result, (*inode).i_sb, s);
        (*bh_result).b_size = n_secs << 9;
        r = 0;
    } else if create == 0 {
        r = 0;
    } else {
        if iblock << 9 != (*hpfs_i(inode)).mmu_private { BUG(); r = -EIO; hpfs_unlock((*inode).i_sb); return r; }
        s = hpfs_add_sector_to_btree((*inode).i_sb, (*inode).i_ino, 1, (*inode).i_blocks - 1);
        if s == (-1i32 as secno) {
            hpfs_truncate_btree((*inode).i_sb, (*inode).i_ino, 1, (*inode).i_blocks - 1);
            r = -ENOSPC; hpfs_unlock((*inode).i_sb); return r;
        }
        (*inode).i_blocks += 1;
        (*hpfs_i(inode)).mmu_private += 512;
        set_buffer_new(bh_result);
        map_bh(bh_result, (*inode).i_sb, hpfs_search_hotfix_map((*inode).i_sb, s));
        r = 0;
    }
    hpfs_unlock((*inode).i_sb);
    r
}

unsafe fn hpfs_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t, flags: u32, iomap: *mut iomap, _srcmap: *mut iomap) -> i32 {
    let sb = (*inode).i_sb;
    let blkbits = (*inode).i_blkbits;
    let mut n_secs = 0u32;
    if flags & (IOMAP_WRITE | IOMAP_ZERO) != 0 { return -EINVAL; }
    (*iomap).bdev = (*inode).i_sb.s_bdev;
    (*iomap).offset = offset;
    hpfs_lock(sb);
    let mut s = hpfs_bmap(inode, (offset >> blkbits) as u32, &mut n_secs);
    if s != 0 {
        n_secs = hpfs_search_hotfix_map_for_range(sb, s, core::cmp::min(n_secs as loff_t, length) as u32);
        if n_secs == 0 { s = hpfs_search_hotfix_map(sb, s); n_secs = 1; }
        (*iomap).type_ = IOMAP_MAPPED; (*iomap).flags = IOMAP_F_MERGED;
        (*iomap).addr = (s as u64) << blkbits; (*iomap).length = (n_secs as u64) << blkbits;
    } else { (*iomap).type_ = IOMAP_HOLE; (*iomap).addr = IOMAP_NULL_ADDR; (*iomap).length = 1u64 << blkbits; }
    hpfs_unlock(sb); 0
}

unsafe fn hpfs_read_folio(file: *mut file, folio: *mut folio) -> i32 { mpage_read_folio(folio, hpfs_get_block) }
unsafe fn hpfs_readahead(rac: *mut readahead_control) { mpage_readahead(rac, hpfs_get_block); }
unsafe fn hpfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 { mpage_writepages(mapping, wbc, hpfs_get_block) }

unsafe fn hpfs_write_failed(mapping: *mut address_space, to: loff_t) {
    let inode = (*mapping).host; hpfs_lock((*inode).i_sb);
    if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); hpfs_truncate(inode); }
    hpfs_unlock((*inode).i_sb);
}

unsafe fn hpfs_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: u32, foliop: *mut *mut folio, fsdata: *mut *mut core::ffi::c_void) -> i32 {
    let ret = cont_write_begin(iocb, mapping, pos, len, foliop, fsdata, hpfs_get_block, &mut (*hpfs_i((*mapping).host)).mmu_private);
    if ret != 0 { hpfs_write_failed(mapping, pos + len as loff_t); } ret
}

unsafe fn hpfs_write_end(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: u32, copied: u32, folio: *mut folio, fsdata: *mut core::ffi::c_void) -> i32 {
    let inode = (*mapping).host; let err = generic_write_end(iocb, mapping, pos, len, copied, folio, fsdata);
    if err < len as i32 { hpfs_write_failed(mapping, pos + len as loff_t); }
    if err >= 0 { hpfs_lock((*inode).i_sb); (*hpfs_i(inode)).i_dirty = 1; hpfs_unlock((*inode).i_sb); } err
}

unsafe fn _hpfs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t { generic_block_bmap(mapping, block, hpfs_get_block) }
unsafe fn hpfs_fiemap(inode: *mut inode, fieinfo: *mut fiemap_extent_info, start: u64, mut len: u64) -> i32 {
    inode_lock(inode); len = core::cmp::min(len, i_size_read(inode)); let ret = iomap_fiemap(inode, fieinfo, start, len, &hpfs_iomap_ops); inode_unlock(inode); ret
}

static hpfs_iomap_ops: iomap_ops = iomap_ops { iomap_next: hpfs_iomap_next };

const hpfs_aops: address_space_operations = address_space_operations {
    dirty_folio: block_dirty_folio, invalidate_folio: block_invalidate_folio, read_folio: hpfs_read_folio,
    readahead: hpfs_readahead, writepages: hpfs_writepages, write_begin: hpfs_write_begin, write_end: hpfs_write_end,
    bmap: _hpfs_bmap, migrate_folio: buffer_migrate_folio,
};

const hpfs_file_ops: file_operations = file_operations {
    llseek: generic_file_llseek, read_iter: generic_file_read_iter, write_iter: generic_file_write_iter,
    mmap_prepare: generic_file_mmap_prepare, release: hpfs_file_release, fsync: hpfs_file_fsync,
    splice_read: filemap_splice_read, unlocked_ioctl: hpfs_ioctl, compat_ioctl: compat_ptr_ioctl,
};

const hpfs_file_iops: inode_operations = inode_operations { setattr: hpfs_setattr, fiemap: hpfs_fiemap };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

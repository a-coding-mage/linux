// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/extents.c
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Handling of Extents both in catalog and extents overflow trees
 */

/* External kernel and HFS+ declarations are supplied by other translation units. */

pub unsafe fn hfsplus_ext_cmp_key(k1: *const hfsplus_btree_key, k2: *const hfsplus_btree_key) -> i32 {
    let k1id = (*k1).ext.cnid;
    let k2id = (*k2).ext.cnid;
    if k1id != k2id { return if be32_to_cpu(k1id) < be32_to_cpu(k2id) { -1 } else { 1 }; }
    if (*k1).ext.fork_type != (*k2).ext.fork_type {
        return if (*k1).ext.fork_type < (*k2).ext.fork_type { -1 } else { 1 };
    }
    let k1s = (*k1).ext.start_block;
    let k2s = (*k2).ext.start_block;
    if k1s == k2s { 0 } else if be32_to_cpu(k1s) < be32_to_cpu(k2s) { -1 } else { 1 }
}

unsafe fn hfsplus_ext_build_key(key: *mut hfsplus_btree_key, cnid: u32, block: u32, typ: u8) {
    (*key).key_len = cpu_to_be16(HFSPLUS_EXT_KEYLEN - 2);
    (*key).ext.cnid = cpu_to_be32(cnid);
    (*key).ext.start_block = cpu_to_be32(block);
    (*key).ext.fork_type = typ;
    (*key).ext.pad = 0;
}

unsafe fn hfsplus_ext_find_block(mut ext: *mut hfsplus_extent, mut off: u32) -> u32 {
    for _ in 0..8 {
        let count = be32_to_cpu((*ext).block_count);
        if off < count { return be32_to_cpu((*ext).start_block) + off; }
        off -= count;
        ext = ext.add(1);
    }
    0
}

unsafe fn hfsplus_ext_block_count(mut ext: *mut hfsplus_extent) -> i32 {
    let mut count: u32 = 0;
    for _ in 0..8 { count += be32_to_cpu((*ext).block_count); ext = ext.add(1); }
    count as i32
}

unsafe fn hfsplus_ext_lastblock(mut ext: *mut hfsplus_extent) -> u32 {
    ext = ext.add(7);
    for _ in 0..7 {
        if (*ext).block_count != 0 { break; }
        ext = ext.sub(1);
    }
    be32_to_cpu((*ext).start_block) + be32_to_cpu((*ext).block_count)
}

unsafe fn __hfsplus_ext_write_extent(inode: *mut inode, fd: *mut hfs_find_data) -> i32 {
    let hip = HFSPLUS_I(inode);
    let mut res: i32;
    WARN_ON(!mutex_is_locked(&mut (*hip).extents_lock));
    hfsplus_ext_build_key((*fd).search_key, (*inode).i_ino, (*hip).cached_start,
        if HFSPLUS_IS_RSRC(inode) { HFSPLUS_TYPE_RSRC } else { HFSPLUS_TYPE_DATA });
    res = hfs_brec_find(fd, hfs_find_rec_by_key);
    if (*hip).extent_state & HFSPLUS_EXT_NEW != 0 {
        if res != -ENOENT { return res; }
        res = hfs_bmap_reserve((*fd).tree, (*(*fd).tree).depth + 1);
        if res != 0 { return res; }
        hfs_brec_insert(fd, (*hip).cached_extents as *const _, core::mem::size_of::<hfsplus_extent_rec>());
        (*hip).extent_state &= !(HFSPLUS_EXT_DIRTY | HFSPLUS_EXT_NEW);
    } else {
        if res != 0 { return res; }
        if (*fd).entrylength != core::mem::size_of::<hfsplus_extent_rec>() { return -EIO; }
        hfs_bnode_write((*fd).bnode, (*hip).cached_extents as *const _, (*fd).entryoffset, (*fd).entrylength);
        (*hip).extent_state &= !HFSPLUS_EXT_DIRTY;
    }
    set_bit(HFSPLUS_I_EXT_DIRTY, &mut (*HFSPLUS_I(HFSPLUS_EXT_TREE_I((*inode).i_sb))).flags);
    set_bit(HFSPLUS_I_EXT_DIRTY, &mut (*hip).flags);
    0
}

unsafe fn hfsplus_ext_write_extent_locked(inode: *mut inode) -> i32 {
    let mut res = 0;
    if HFSPLUS_I(inode).as_ref().unwrap().extent_state & HFSPLUS_EXT_DIRTY != 0 {
        let mut fd = core::mem::zeroed::<hfs_find_data>();
        res = hfs_find_init((*HFSPLUS_SB((*inode).i_sb)).ext_tree, &mut fd);
        if res == 0 { res = __hfsplus_ext_write_extent(inode, &mut fd); hfs_find_exit(&mut fd); }
    }
    res
}

pub unsafe fn hfsplus_ext_write_extent(inode: *mut inode) -> i32 {
    mutex_lock(&mut (*HFSPLUS_I(inode)).extents_lock);
    let res = hfsplus_ext_write_extent_locked(inode);
    mutex_unlock(&mut (*HFSPLUS_I(inode)).extents_lock);
    res
}

unsafe fn __hfsplus_ext_read_extent(fd: *mut hfs_find_data, extent: *mut hfsplus_extent, cnid: u32, block: u32, typ: u8) -> i32 {
    hfsplus_ext_build_key((*fd).search_key, cnid, block, typ);
    (*fd).key.ext.cnid = 0;
    let res = hfs_brec_find(fd, hfs_find_rec_by_key);
    if res != 0 && res != -ENOENT { return res; }
    if (*fd).key.ext.cnid != (*fd).search_key.ext.cnid || (*fd).key.ext.fork_type != (*fd).search_key.ext.fork_type { return -ENOENT; }
    if (*fd).entrylength != core::mem::size_of::<hfsplus_extent_rec>() { return -EIO; }
    hfs_bnode_read((*fd).bnode, extent as *mut _, (*fd).entryoffset, core::mem::size_of::<hfsplus_extent_rec>());
    0
}

unsafe fn __hfsplus_ext_cache_extent(fd: *mut hfs_find_data, inode: *mut inode, block: u32) -> i32 {
    let hip = HFSPLUS_I(inode);
    WARN_ON(!mutex_is_locked(&mut (*hip).extents_lock));
    let mut res = 0;
    if (*hip).extent_state & HFSPLUS_EXT_DIRTY != 0 { res = __hfsplus_ext_write_extent(inode, fd); if res != 0 { return res; } }
    res = __hfsplus_ext_read_extent(fd, (*hip).cached_extents, (*inode).i_ino, block,
        if HFSPLUS_IS_RSRC(inode) { HFSPLUS_TYPE_RSRC } else { HFSPLUS_TYPE_DATA });
    if res == 0 {
        (*hip).cached_start = be32_to_cpu((*fd).key.ext.start_block);
        (*hip).cached_blocks = hfsplus_ext_block_count((*hip).cached_extents) as u32;
    } else {
        (*hip).cached_start = 0; (*hip).cached_blocks = 0;
        (*hip).extent_state &= !(HFSPLUS_EXT_DIRTY | HFSPLUS_EXT_NEW);
    }
    res
}

unsafe fn hfsplus_ext_read_extent(inode: *mut inode, block: u32) -> i32 {
    let hip = HFSPLUS_I(inode);
    if block >= (*hip).cached_start && block < (*hip).cached_start + (*hip).cached_blocks { return 0; }
    let mut fd = core::mem::zeroed::<hfs_find_data>();
    let mut res = hfs_find_init((*HFSPLUS_SB((*inode).i_sb)).ext_tree, &mut fd);
    if res == 0 { res = __hfsplus_ext_cache_extent(&mut fd, inode, block); hfs_find_exit(&mut fd); }
    res
}

pub unsafe fn hfsplus_get_block(inode: *mut inode, iblock: sector_t, bh_result: *mut buffer_head, mut create: i32) -> i32 {
    let sb = (*inode).i_sb; let sbi = HFSPLUS_SB(sb); let hip = HFSPLUS_I(inode);
    let mut res = -EIO; let ablock = iblock >> (*sbi).fs_shift; let dblock: u32; let mut was_dirty = 0;
    if iblock >= (*hip).fs_blocks { if create == 0 { return 0; } if iblock > (*hip).fs_blocks { return -EIO; } if ablock >= (*hip).alloc_blocks { res = hfsplus_file_extend(inode, false); if res != 0 { return res; } } } else { create = 0; }
    if ablock < (*hip).first_blocks { dblock = hfsplus_ext_find_block((*hip).first_extents, ablock); } else {
        if (*inode).i_ino == HFSPLUS_EXT_CNID { return -EIO; }
        mutex_lock(&mut (*hip).extents_lock); was_dirty = (*hip).extent_state & HFSPLUS_EXT_DIRTY;
        res = hfsplus_ext_read_extent(inode, ablock); if res != 0 { mutex_unlock(&mut (*hip).extents_lock); return -EIO; }
        dblock = hfsplus_ext_find_block((*hip).cached_extents, ablock - (*hip).cached_start); mutex_unlock(&mut (*hip).extents_lock);
    }
    let mask = (1u64 << (*sbi).fs_shift) - 1;
    let sector = ((dblock as sector_t) << (*sbi).fs_shift) + (*sbi).blockoffset + (iblock & mask);
    map_bh(bh_result, sb, sector);
    if create != 0 { set_buffer_new(bh_result); (*hip).phys_size += (*sb).s_blocksize as u64; (*hip).fs_blocks += 1; inode_add_bytes(inode, (*sb).s_blocksize as u64); }
    if create != 0 || was_dirty != 0 { mark_inode_dirty(inode); } 0
}

unsafe fn hfsplus_dump_extent(extent: *mut hfsplus_extent) { hfs_dbg!("extent   "); for i in 0..8 { hfs_dbg!(" start_block %u, block_count %u", be32_to_cpu((*extent.add(i)).start_block), be32_to_cpu((*extent.add(i)).block_count)); } hfs_dbg!("\n"); }

unsafe fn hfsplus_add_extent(mut extent: *mut hfsplus_extent, mut offset: u32, alloc_block: u32, mut block_count: u32) -> i32 {
    hfsplus_dump_extent(extent);
    for i in 0..8 { let count = be32_to_cpu((*extent).block_count); if offset == count { let start = be32_to_cpu((*extent).start_block); if alloc_block != start + count { if i + 1 >= 8 { return -ENOSPC; } extent = extent.add(1); (*extent).start_block = cpu_to_be32(alloc_block); } else { block_count += count; } (*extent).block_count = cpu_to_be32(block_count); return 0; } else if offset < count { break; } offset -= count; extent = extent.add(1); } -EIO
}

unsafe fn hfsplus_free_extents(sb: *mut super_block, mut extent: *mut hfsplus_extent, mut offset: u32, mut block_nr: u32) -> i32 {
    hfsplus_dump_extent(extent); let mut i = 0; let mut count;
    loop { count = be32_to_cpu((*extent).block_count); if offset == count { break; } if offset < count { return -EIO; } offset -= count; i += 1; extent = extent.add(1); if i >= 8 { return -EIO; } }
    let mut err = 0;
    loop { let start = be32_to_cpu((*extent).start_block); if count <= block_nr { err = hfsplus_block_free(sb, start, count); (*extent).block_count = 0; (*extent).start_block = 0; block_nr -= count; } else { count -= block_nr; err = hfsplus_block_free(sb, start + count, block_nr); (*extent).block_count = cpu_to_be32(count); block_nr = 0; } if block_nr == 0 || i == 0 { return err; } i -= 1; extent = extent.sub(1); count = be32_to_cpu((*extent).block_count); }
}

pub unsafe fn hfsplus_free_fork(sb: *mut super_block, cnid: u32, fork: *mut hfsplus_fork_raw, typ: i32) -> i32 {
    let mut total_blocks = be32_to_cpu((*fork).total_blocks); if total_blocks == 0 { return 0; }
    let mut blocks = 0; for i in 0..8 { blocks += be32_to_cpu((*fork).extents[i].block_count); }
    let mut res = hfsplus_free_extents(sb, (*fork).extents.as_mut_ptr(), blocks, blocks); if res != 0 || total_blocks == blocks { return res; }
    let mut fd = core::mem::zeroed::<hfs_find_data>(); res = hfs_find_init((*HFSPLUS_SB((*sb).s_fs_info)).ext_tree, &mut fd); if res != 0 { return res; }
    let mut ext_entry = core::mem::zeroed::<hfsplus_extent_rec>();
    loop { res = __hfsplus_ext_read_extent(&mut fd, ext_entry.as_mut_ptr() as *mut _, cnid, total_blocks, typ as u8); if res != 0 { break; } let start = be32_to_cpu((*fd.key).ext.start_block); hfs_brec_remove(&mut fd); mutex_unlock(&mut (*fd.tree).tree_lock); hfsplus_free_extents(sb, ext_entry.as_mut_ptr() as *mut _, total_blocks - start, total_blocks); total_blocks = start; mutex_lock_nested(&mut (*fd.tree).tree_lock, hfsplus_btree_lock_class(fd.tree)); if total_blocks <= blocks { break; } }
    hfs_find_exit(&mut fd); res
}

pub unsafe fn hfsplus_file_extend(inode: *mut inode, zeroout: bool) -> i32 {
    let sb = (*inode).i_sb; let sbi = HFSPLUS_SB(sb); let hip = HFSPLUS_I(inode); let mut start; let mut len; let goal; let mut res;
    if (*sbi).alloc_file.i_size * 8 < (*sbi).total_blocks - (*sbi).free_blocks + 8 { return -ENOSPC; }
    mutex_lock(&mut (*hip).extents_lock);
    if (*hip).alloc_blocks == (*hip).first_blocks { goal = hfsplus_ext_lastblock((*hip).first_extents); } else { res = hfsplus_ext_read_extent(inode, (*hip).alloc_blocks); if res != 0 { mutex_unlock(&mut (*hip).extents_lock); return res; } goal = hfsplus_ext_lastblock((*hip).cached_extents); }
    len = (*hip).clump_blocks; start = hfsplus_block_allocate(sb, (*sbi).total_blocks, goal, &mut len); if start >= (*sbi).total_blocks { start = hfsplus_block_allocate(sb, goal, 0, &mut len); if start >= goal { mutex_unlock(&mut (*hip).extents_lock); return -ENOSPC; } }
    if zeroout { res = sb_issue_zeroout(sb, start, len, GFP_NOFS); if res != 0 { mutex_unlock(&mut (*hip).extents_lock); return res; } }
    if (*hip).alloc_blocks <= (*hip).first_blocks { if (*hip).first_blocks == 0 { (*hip).first_extents[0].start_block = cpu_to_be32(start); (*hip).first_extents[0].block_count = cpu_to_be32(len); res = 0; } else { res = hfsplus_add_extent((*hip).first_extents, (*hip).alloc_blocks, start, len); if res == -ENOSPC { res = hfsplus_ext_write_extent_locked(inode); if res == 0 { core::ptr::write_bytes((*hip).cached_extents as *mut u8, 0, core::mem::size_of::<hfsplus_extent_rec>()); (*hip).cached_extents[0].start_block = cpu_to_be32(start); (*hip).cached_extents[0].block_count = cpu_to_be32(len); (*hip).extent_state |= HFSPLUS_EXT_DIRTY | HFSPLUS_EXT_NEW; (*hip).cached_start = (*hip).alloc_blocks; (*hip).cached_blocks = len; } } } else { res = hfsplus_add_extent((*hip).cached_extents, (*hip).alloc_blocks - (*hip).cached_start, start, len); if res == 0 { (*hip).extent_state |= HFSPLUS_EXT_DIRTY; (*hip).cached_blocks += len; } else if res == -ENOSPC { res = hfsplus_ext_write_extent_locked(inode); if res == 0 { core::ptr::write_bytes((*hip).cached_extents as *mut u8, 0, core::mem::size_of::<hfsplus_extent_rec>()); (*hip).cached_extents[0].start_block = cpu_to_be32(start); (*hip).cached_extents[0].block_count = cpu_to_be32(len); (*hip).extent_state |= HFSPLUS_EXT_DIRTY | HFSPLUS_EXT_NEW; (*hip).cached_start = (*hip).alloc_blocks; (*hip).cached_blocks = len; } } }
    if res == 0 { if (*hip).alloc_blocks <= (*hip).first_blocks { (*hip).first_blocks += len; } (*hip).alloc_blocks += len; mutex_unlock(&mut (*hip).extents_lock); return 0; } mutex_unlock(&mut (*hip).extents_lock); res
}

pub unsafe fn hfsplus_file_truncate(inode: *mut inode) {
    let sb = (*inode).i_sb; let hip = HFSPLUS_I(inode); if (*inode).i_size >= (*hip).phys_size { if (*inode).i_size > (*hip).phys_size { (*hip).phys_size = (*inode).i_size; } return; }
    let blk_cnt = ((*inode).i_size + (*HFSPLUS_SB(sb)).alloc_blksz as u64 - 1) >> (*HFSPLUS_SB(sb)).alloc_blksz_shift; mutex_lock(&mut (*hip).extents_lock); let mut alloc_cnt = (*hip).alloc_blocks; if blk_cnt != alloc_cnt { let mut fd = core::mem::zeroed::<hfs_find_data>(); if hfs_find_init((*HFSPLUS_SB(sb)).ext_tree, &mut fd) == 0 { while alloc_cnt > blk_cnt { if alloc_cnt == (*hip).first_blocks { hfsplus_free_extents(sb, (*hip).first_extents, alloc_cnt, alloc_cnt - blk_cnt); (*hip).first_blocks = blk_cnt; break; } if __hfsplus_ext_cache_extent(&mut fd, inode, alloc_cnt) != 0 { break; } let start = (*hip).cached_start; if blk_cnt <= start { hfs_brec_remove(&mut fd); } mutex_unlock(&mut (*fd.tree).tree_lock); hfsplus_free_extents(sb, (*hip).cached_extents, alloc_cnt - start, alloc_cnt - blk_cnt); mutex_lock_nested(&mut (*fd.tree).tree_lock, hfsplus_btree_lock_class(fd.tree)); if blk_cnt > start { (*hip).extent_state |= HFSPLUS_EXT_DIRTY; break; } alloc_cnt = start; (*hip).cached_start = 0; (*hip).cached_blocks = 0; (*hip).extent_state &= !(HFSPLUS_EXT_DIRTY | HFSPLUS_EXT_NEW); } hfs_find_exit(&mut fd); } (*hip).alloc_blocks = blk_cnt; } mutex_unlock(&mut (*hip).extents_lock); (*hip).phys_size = (*inode).i_size; (*hip).fs_blocks = ((*inode).i_size + (*sb).s_blocksize as u64 - 1) >> (*sb).s_blocksize_bits; inode_set_bytes(inode, (*hip).fs_blocks << (*sb).s_blocksize_bits); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

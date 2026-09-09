/*
 *  linux/fs/hfs/extent.c
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains the functions related to the extents B-tree.
 */

// Dependencies supplied by the surrounding HFS implementation.

/*================ File-local functions ================*/

fn hfs_ext_build_key(key: *mut hfs_btree_key, cnid: u32, block: u16, r#type: u8) {
    unsafe {
        (*key).key_len = 7;
        (*key).ext.FkType = r#type;
        (*key).ext.FNum = cpu_to_be32(cnid);
        (*key).ext.FABN = cpu_to_be16(block);
    }
}

pub unsafe fn hfs_ext_keycmp(key1: *const btree_key, key2: *const btree_key) -> i32 {
    let fnum1 = (*key1).ext.FNum;
    let fnum2 = (*key2).ext.FNum;
    if fnum1 != fnum2 {
        return if be32_to_cpu(fnum1) < be32_to_cpu(fnum2) { -1 } else { 1 };
    }
    if (*key1).ext.FkType != (*key2).ext.FkType {
        return if (*key1).ext.FkType < (*key2).ext.FkType { -1 } else { 1 };
    }
    let block1 = (*key1).ext.FABN;
    let block2 = (*key2).ext.FABN;
    if block1 == block2 { return 0; }
    if be16_to_cpu(block1) < be16_to_cpu(block2) { -1 } else { 1 }
}

pub unsafe fn hfs_ext_find_block(mut ext: *mut hfs_extent, mut off: u16) -> u16 {
    for _ in 0..3 {
        let count = be16_to_cpu((*ext).count);
        if off < count { return be16_to_cpu((*ext).block).wrapping_add(off); }
        off = off.wrapping_sub(count);
        ext = ext.add(1);
    }
    0
}

unsafe fn hfs_ext_block_count(mut ext: *mut hfs_extent) -> i32 {
    let mut count: u16 = 0;
    for _ in 0..3 {
        count = count.wrapping_add(be16_to_cpu((*ext).count));
        ext = ext.add(1);
    }
    count as i32
}

unsafe fn hfs_ext_lastblock(mut ext: *mut hfs_extent) -> u16 {
    ext = ext.add(2);
    for _ in 0..2 {
        if (*ext).count != 0 { break; }
        ext = ext.sub(1);
    }
    be16_to_cpu((*ext).block).wrapping_add(be16_to_cpu((*ext).count))
}

unsafe fn __hfs_ext_write_extent(inode: *mut inode, fd: *mut hfs_find_data) -> i32 {
    hfs_ext_build_key((*fd).search_key, (*inode).i_ino, HFS_I(inode).cached_start,
        if HFS_IS_RSRC(inode) { HFS_FK_RSRC } else { HFS_FK_DATA });
    let res = hfs_brec_find(fd);
    if HFS_I(inode).flags & HFS_FLG_EXT_NEW != 0 {
        if res != -ENOENT { return res; }
        let res = hfs_bmap_reserve((*fd).tree, (*fd).tree.depth + 1);
        if res != 0 { return res; }
        hfs_brec_insert(fd, HFS_I(inode).cached_extents, core::mem::size_of::<hfs_extent_rec>());
        HFS_I(inode).flags &= !(HFS_FLG_EXT_DIRTY | HFS_FLG_EXT_NEW);
    } else {
        if res != 0 { return res; }
        hfs_bnode_write((*fd).bnode, HFS_I(inode).cached_extents, (*fd).entryoffset, (*fd).entrylength);
        HFS_I(inode).flags &= !HFS_FLG_EXT_DIRTY;
    }
    0
}

pub unsafe fn hfs_ext_write_extent(inode: *mut inode) -> i32 {
    let mut fd: hfs_find_data = core::mem::zeroed();
    let mut res = 0;
    if HFS_I(inode).flags & HFS_FLG_EXT_DIRTY != 0 {
        res = hfs_find_init(HFS_SB((*inode).i_sb).ext_tree, &mut fd);
        if res != 0 { return res; }
        res = __hfs_ext_write_extent(inode, &mut fd);
        hfs_find_exit(&mut fd);
    }
    res
}

unsafe fn __hfs_ext_read_extent(fd: *mut hfs_find_data, extent: *mut hfs_extent,
                                cnid: u32, block: u32, r#type: u8) -> i32 {
    hfs_ext_build_key((*fd).search_key, cnid, block as u16, r#type);
    (*fd).key.ext.FNum = 0;
    let res = hfs_brec_find(fd);
    if res != 0 && res != -ENOENT { return res; }
    if (*fd).key.ext.FNum != (*fd).search_key.ext.FNum ||
       (*fd).key.ext.FkType != (*fd).search_key.ext.FkType { return -ENOENT; }
    if (*fd).entrylength != core::mem::size_of::<hfs_extent_rec>() { return -EIO; }
    hfs_bnode_read((*fd).bnode, extent, (*fd).entryoffset, core::mem::size_of::<hfs_extent_rec>());
    0
}

unsafe fn __hfs_ext_cache_extent(fd: *mut hfs_find_data, inode: *mut inode, block: u32) -> i32 {
    if HFS_I(inode).flags & HFS_FLG_EXT_DIRTY != 0 {
        let res = __hfs_ext_write_extent(inode, fd);
        if res != 0 { return res; }
    }
    let res = __hfs_ext_read_extent(fd, HFS_I(inode).cached_extents, (*inode).i_ino, block,
        if HFS_IS_RSRC(inode) { HFS_FK_RSRC } else { HFS_FK_DATA });
    if res == 0 {
        HFS_I(inode).cached_start = be16_to_cpu((*fd).key.ext.FABN);
        HFS_I(inode).cached_blocks = hfs_ext_block_count(HFS_I(inode).cached_extents) as u16;
    } else {
        HFS_I(inode).cached_start = 0;
        HFS_I(inode).cached_blocks = 0;
        HFS_I(inode).flags &= !(HFS_FLG_EXT_DIRTY | HFS_FLG_EXT_NEW);
    }
    res
}

unsafe fn hfs_ext_read_extent(inode: *mut inode, block: u16) -> i32 {
    if block >= HFS_I(inode).cached_start && block < HFS_I(inode).cached_start + HFS_I(inode).cached_blocks { return 0; }
    let mut fd: hfs_find_data = core::mem::zeroed();
    let res = hfs_find_init(HFS_SB((*inode).i_sb).ext_tree, &mut fd);
    if res == 0 { let res = __hfs_ext_cache_extent(&mut fd, inode, block as u32); hfs_find_exit(&mut fd); return res; }
    res
}

unsafe fn hfs_dump_extent(extent: *mut hfs_extent) {
    hfs_dbg!("extent:   ");
    for i in 0..3 { hfs_dbg!(" block %u, count %u", be16_to_cpu((*extent.add(i)).block), be16_to_cpu((*extent.add(i)).count)); }
    hfs_dbg!("\n");
}

unsafe fn hfs_add_extent(mut extent: *mut hfs_extent, mut offset: u16, alloc_block: u16, mut block_count: u16) -> i32 {
    let mut i = 0;
    hfs_dump_extent(extent);
    while i < 3 {
        let count = be16_to_cpu((*extent).count);
        if offset == count {
            let start = be16_to_cpu((*extent).block);
            if alloc_block != start + count { i += 1; if i >= 3 { return -ENOSPC; } extent = extent.add(1); (*extent).block = cpu_to_be16(alloc_block); }
            else { block_count += count; }
            (*extent).count = cpu_to_be16(block_count); return 0;
        } else if offset < count { break; }
        offset -= count; extent = extent.add(1); i += 1;
    }
    -EIO
}

unsafe fn hfs_free_extents(sb: *mut super_block, mut extent: *mut hfs_extent, mut offset: u16, mut block_nr: u16) -> i32 {
    let mut i = 0; hfs_dump_extent(extent);
    while i < 3 { let count = be16_to_cpu((*extent).count); if offset == count { break; } else if offset < count { return -EIO; } offset -= count; extent = extent.add(1); i += 1; }
    if i >= 3 { return -EIO; }
    loop {
        let start = be16_to_cpu((*extent).block);
        let mut count = be16_to_cpu((*extent).count);
        if count <= block_nr { hfs_clear_vbm_bits(sb, start, count); (*extent).block = 0; (*extent).count = 0; block_nr -= count; }
        else { count -= block_nr; hfs_clear_vbm_bits(sb, start + count, block_nr); (*extent).count = cpu_to_be16(count); block_nr = 0; }
        if block_nr == 0 || i == 0 { return 0; }
        i -= 1; extent = extent.sub(1);
    }
}

pub unsafe fn hfs_free_fork(sb: *mut super_block, file: *mut hfs_cat_file, r#type: i32) -> i32 {
    let (total_bytes, extent) = if r#type == HFS_FK_DATA { ((*file).PyLen, (*file).ExtRec.as_mut_ptr()) } else { ((*file).RPyLen, (*file).RExtRec.as_mut_ptr()) };
    let mut total_blocks = be32_to_cpu(total_bytes) / HFS_SB(sb).alloc_blksz;
    if total_blocks == 0 { return 0; }
    let mut blocks: u32 = 0; for i in 0..3 { blocks += be16_to_cpu((*extent.add(i)).count) as u32; }
    let mut res = hfs_free_extents(sb, extent, blocks as u16, blocks as u16); if res != 0 || total_blocks == blocks { return res; }
    let mut fd: hfs_find_data = core::mem::zeroed(); res = hfs_find_init(HFS_SB(sb).ext_tree, &mut fd); if res != 0 { return res; }
    let cnid = be32_to_cpu((*file).FlNum);
    loop { res = __hfs_ext_read_extent(&mut fd, extent, cnid, total_blocks, r#type as u8); if res != 0 { break; }
        let start = be16_to_cpu((*fd.key).ext.FABN) as u32; hfs_free_extents(sb, extent, (total_blocks - start) as u16, total_blocks as u16); hfs_brec_remove(&mut fd); total_blocks = start; if total_blocks <= blocks { break; }
    }
    hfs_find_exit(&mut fd); res
}

pub unsafe fn hfs_get_block(inode: *mut inode, block: sector_t, bh_result: *mut buffer_head, mut create: i32) -> i32 {
    let sb = (*inode).i_sb; let ablock = (block as u32) / HFS_SB(sb).fs_div;
    if block >= HFS_I(inode).fs_blocks { if create == 0 { return 0; } if block > HFS_I(inode).fs_blocks { return -EIO; } if ablock >= HFS_I(inode).alloc_blocks { let res = hfs_extend_file(inode); if res != 0 { return res; } } } else { create = 0; }
    let dblock: u16;
    if ablock < HFS_I(inode).first_blocks { dblock = hfs_ext_find_block(HFS_I(inode).first_extents, ablock as u16); }
    else { mutex_lock(&mut HFS_I(inode).extents_lock); let res = hfs_ext_read_extent(inode, ablock as u16); if res != 0 { mutex_unlock(&mut HFS_I(inode).extents_lock); return -EIO; } dblock = hfs_ext_find_block(HFS_I(inode).cached_extents, (ablock - HFS_I(inode).cached_start) as u16); mutex_unlock(&mut HFS_I(inode).extents_lock); }
    map_bh(bh_result, sb, HFS_SB(sb).fs_start + dblock as u32 * HFS_SB(sb).fs_div + block as u32 % HFS_SB(sb).fs_div);
    if create != 0 { set_buffer_new(bh_result); HFS_I(inode).phys_size += (*sb).s_blocksize as u64; HFS_I(inode).fs_blocks += 1; inode_add_bytes(inode, (*sb).s_blocksize as u64); mark_inode_dirty(inode); } 0
}

pub unsafe fn hfs_extend_file(inode: *mut inode) -> i32 {
    let sb = (*inode).i_sb; mutex_lock(&mut HFS_I(inode).extents_lock); let goal = if HFS_I(inode).alloc_blocks == HFS_I(inode).first_blocks { hfs_ext_lastblock(HFS_I(inode).first_extents) } else { let res = hfs_ext_read_extent(inode, HFS_I(inode).alloc_blocks as u16); if res != 0 { mutex_unlock(&mut HFS_I(inode).extents_lock); return res; } hfs_ext_lastblock(HFS_I(inode).cached_extents) };
    let mut len = HFS_I(inode).clump_blocks; let start = hfs_vbm_search_free(sb, goal, &mut len); if len == 0 { mutex_unlock(&mut HFS_I(inode).extents_lock); return -ENOSPC; }
    let mut res; if HFS_I(inode).alloc_blocks == HFS_I(inode).first_blocks { if HFS_I(inode).first_blocks == 0 { HFS_I(inode).first_extents[0].block = cpu_to_be16(start); HFS_I(inode).first_extents[0].count = cpu_to_be16(len); res = 0; } else { res = hfs_add_extent(HFS_I(inode).first_extents, HFS_I(inode).alloc_blocks as u16, start, len); } if res == 0 { HFS_I(inode).first_blocks += len as u32; } } else { res = hfs_add_extent(HFS_I(inode).cached_extents, (HFS_I(inode).alloc_blocks - HFS_I(inode).cached_start) as u16, start, len); if res == 0 { HFS_I(inode).flags |= HFS_FLG_EXT_DIRTY; HFS_I(inode).cached_blocks += len; } }
    if res == -ENOSPC { res = hfs_ext_write_extent(inode); if res == 0 { core::ptr::write_bytes(HFS_I(inode).cached_extents, 0, core::mem::size_of::<hfs_extent_rec>()); HFS_I(inode).cached_extents[0].block = cpu_to_be16(start); HFS_I(inode).cached_extents[0].count = cpu_to_be16(len); HFS_I(inode).flags |= HFS_FLG_EXT_DIRTY | HFS_FLG_EXT_NEW; HFS_I(inode).cached_start = HFS_I(inode).alloc_blocks; HFS_I(inode).cached_blocks = len; } }
    mutex_unlock(&mut HFS_I(inode).extents_lock); if res == 0 { HFS_I(inode).alloc_blocks += len as u32; mark_inode_dirty(inode); } res
}

pub unsafe fn hfs_file_truncate(inode: *mut inode) { let sb = (*inode).i_sb; if (*inode).i_size >= HFS_I(inode).phys_size { return; } let blk_cnt = ((*inode).i_size + HFS_SB(sb).alloc_blksz as u64 - 1) / HFS_SB(sb).alloc_blksz as u64; if blk_cnt == HFS_I(inode).alloc_blocks as u64 { return; } mutex_lock(&mut HFS_I(inode).extents_lock); let mut fd: hfs_find_data = core::mem::zeroed(); if hfs_find_init(HFS_SB(sb).ext_tree, &mut fd) == 0 { while HFS_I(inode).alloc_blocks as u64 > blk_cnt { if HFS_I(inode).alloc_blocks == HFS_I(inode).first_blocks { hfs_free_extents(sb, HFS_I(inode).first_extents, HFS_I(inode).alloc_blocks as u16, (HFS_I(inode).alloc_blocks as u64 - blk_cnt) as u16); HFS_I(inode).first_blocks = blk_cnt as u32; break; } let _ = __hfs_ext_cache_extent(&mut fd, inode, HFS_I(inode).alloc_blocks); let start = HFS_I(inode).cached_start; hfs_free_extents(sb, HFS_I(inode).cached_extents, (HFS_I(inode).alloc_blocks - start) as u16, (HFS_I(inode).alloc_blocks as u64 - blk_cnt) as u16); if blk_cnt > start as u64 { HFS_I(inode).flags |= HFS_FLG_EXT_DIRTY; break; } HFS_I(inode).alloc_blocks = start; HFS_I(inode).cached_start = 0; HFS_I(inode).cached_blocks = 0; HFS_I(inode).flags &= !(HFS_FLG_EXT_DIRTY | HFS_FLG_EXT_NEW); hfs_brec_remove(&mut fd); } hfs_find_exit(&mut fd); } mutex_unlock(&mut HFS_I(inode).extents_lock); HFS_I(inode).alloc_blocks = blk_cnt as u32; HFS_I(inode).phys_size = (*inode).i_size; mark_inode_dirty(inode); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
 */

unsafe fn exfat_mirror_bh(sb: *mut super_block, bh: *mut buffer_head) -> i32 {
    let mut c_bh: *mut buffer_head;
    let sbi = EXFAT_SB(sb);
    let sec: sector_t = (*bh).b_blocknr;
    let sec2: sector_t;
    let mut err: i32 = 0;

    if (*sbi).FAT2_start_sector != (*sbi).FAT1_start_sector {
        sec2 = sec - (*sbi).FAT1_start_sector + (*sbi).FAT2_start_sector;
        c_bh = sb_getblk(sb, sec2);
        if c_bh.is_null() {
            return -ENOMEM;
        }
        memcpy((*c_bh).b_data, (*bh).b_data, (*sb).s_blocksize);
        err = exfat_update_bh(c_bh, (*sb).s_flags & SB_SYNCHRONOUS);
        brelse(c_bh);
    }

    err
}

unsafe fn exfat_end_bh(sb: *mut super_block, bh: *mut buffer_head) -> i32 {
    let mut err = exfat_update_bh(bh, (*sb).s_flags & SB_SYNCHRONOUS);
    if err == 0 {
        err = exfat_mirror_bh(sb, bh);
    }
    brelse(bh);
    err
}

unsafe fn __exfat_ent_get(
    sb: *mut super_block,
    loc: u32,
    content: *mut u32,
    cache: *mut *mut buffer_head,
) -> i32 {
    let off: u32;
    let sec: sector_t;
    let mut bh = *cache;

    sec = FAT_ENT_OFFSET_SECTOR(sb, loc);
    off = FAT_ENT_OFFSET_BYTE_IN_SECTOR(sb, loc);

    if bh.is_null() || (*bh).b_blocknr != sec || !buffer_uptodate(bh) {
        brelse(bh);
        bh = sb_bread(sb, sec);
        *cache = bh;
        if bh.is_null() {
            return -EIO;
        }
    }

    *content = le32_to_cpu(*(((*bh).b_data.add(off as usize)) as *mut __le32));

    /* remap reserved clusters to simplify code */
    if *content > EXFAT_BAD_CLUSTER {
        *content = EXFAT_EOF_CLUSTER;
    }

    0
}

unsafe fn __exfat_ent_set(
    sb: *mut super_block,
    loc: u32,
    content: u32,
    cache: *mut *mut buffer_head,
) -> i32 {
    let sec: sector_t;
    let fat_entry: *mut __le32;
    let mut bh = if !cache.is_null() { *cache } else { core::ptr::null_mut() };
    let off: u32;

    sec = FAT_ENT_OFFSET_SECTOR(sb, loc);
    off = FAT_ENT_OFFSET_BYTE_IN_SECTOR(sb, loc);

    if bh.is_null() || (*bh).b_blocknr != sec || !buffer_uptodate(bh) {
        if !bh.is_null() {
            exfat_end_bh(sb, bh);
        }
        bh = sb_bread(sb, sec);
        if !cache.is_null() {
            *cache = bh;
        }
        if bh.is_null() {
            return -EIO;
        }
    }

    fat_entry = ((*bh).b_data.add(off as usize)) as *mut __le32;
    *fat_entry = cpu_to_le32(content);
    if cache.is_null() {
        exfat_end_bh(sb, bh);
    }
    0
}

pub unsafe fn exfat_ent_set(sb: *mut super_block, loc: u32, content: u32) -> i32 {
    __exfat_ent_set(sb, loc, content, core::ptr::null_mut())
}

/*
 * Caller must release the buffer_head if no error return.
 */
pub unsafe fn exfat_ent_get(
    sb: *mut super_block,
    loc: u32,
    content: *mut u32,
    cache: *mut *mut buffer_head,
) -> i32 {
    let sbi = EXFAT_SB(sb);

    if !is_valid_cluster(sbi, loc) {
        exfat_fs_error_ratelimit(sb, c"invalid access to FAT (entry 0x%08x)", loc);
        goto_err:
        brelse(*cache);
        *cache = core::ptr::null_mut();
        return -EIO;
    }

    if __exfat_ent_get(sb, loc, content, cache) != 0 {
        exfat_fs_error_ratelimit(sb, c"failed to access to FAT (entry 0x%08x)", loc);
        brelse(*cache);
        *cache = core::ptr::null_mut();
        return -EIO;
    }
    if *content == EXFAT_FREE_CLUSTER {
        exfat_fs_error_ratelimit(sb, c"invalid access to FAT free cluster (entry 0x%08x)", loc);
        brelse(*cache); *cache = core::ptr::null_mut(); return -EIO;
    }
    if *content == EXFAT_BAD_CLUSTER {
        exfat_fs_error_ratelimit(sb, c"invalid access to FAT bad cluster (entry 0x%08x)", loc);
        brelse(*cache); *cache = core::ptr::null_mut(); return -EIO;
    }
    if *content != EXFAT_EOF_CLUSTER && !is_valid_cluster(sbi, *content) {
        exfat_fs_error_ratelimit(sb, c"invalid access to FAT (entry 0x%08x) bogus content (0x%08x)", loc, *content);
        brelse(*cache); *cache = core::ptr::null_mut(); return -EIO;
    }
    0
}

pub unsafe fn exfat_blk_readahead(sb: *mut super_block, sec: sector_t, ra: *mut sector_t, ra_cnt: *mut blkcnt_t, end: sector_t) -> i32 {
    let mut plug: blk_plug;
    if sec < *ra { return 0; }
    *ra += *ra_cnt;
    if *ra >= end { return 0; }
    *ra_cnt = min(end - *ra + 1, EXFAT_BLK_RA_SIZE(sb));
    if *ra_cnt == 0 { *ra = end; return 0; }
    blk_start_plug(&mut plug);
    for i in 0..*ra_cnt { sb_breadahead(sb, *ra + i); }
    blk_finish_plug(&mut plug);
    0
}

pub unsafe fn exfat_chain_cont_cluster(sb: *mut super_block, mut chain: u32, mut len: u32) -> i32 {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let sec: sector_t;
    let end: sector_t;
    let mut ra: sector_t;
    let mut ra_cnt: blkcnt_t = 0;
    if len == 0 { return 0; }
    ra = FAT_ENT_OFFSET_SECTOR(sb, chain);
    end = FAT_ENT_OFFSET_SECTOR(sb, chain + len - 1);
    while len > 1 {
        sec = FAT_ENT_OFFSET_SECTOR(sb, chain);
        exfat_blk_readahead(sb, sec, &mut ra, &mut ra_cnt, end);
        if __exfat_ent_set(sb, chain, chain + 1, &mut bh) != 0 { return -EIO; }
        chain += 1; len -= 1;
    }
    if __exfat_ent_set(sb, chain, EXFAT_EOF_CLUSTER, &mut bh) != 0 { return -EIO; }
    exfat_end_bh(sb, bh);
    0
}

unsafe fn exfat_discard_cluster(sb: *mut super_block, clu: u32, num_clusters: u32) {
    let sbi = EXFAT_SB(sb);
    let ret = sb_issue_discard(sb, exfat_cluster_to_sector(sbi, clu), (*sbi).sect_per_clus * num_clusters, GFP_NOFS, 0);
    if ret == -EOPNOTSUPP {
        exfat_err(sb, c"discard not supported by device, disabling");
        (*sbi).options.discard = 0;
    }
}

/* This function must be called with bitmap_lock held */
unsafe fn __exfat_free_cluster(inode: *mut inode, p_chain: *mut exfat_chain) -> i32 {
    let sb = (*inode).i_sb;
    let sbi = EXFAT_SB(sb);
    let mut cur_cmap_i: i32;
    let mut next_cmap_i: i32;
    let mut num_clusters: u32 = 0;
    let mut clu: u32;

    if (*p_chain).dir == EXFAT_FREE_CLUSTER || (*p_chain).dir == EXFAT_EOF_CLUSTER || (*p_chain).dir < EXFAT_FIRST_CLUSTER || (*p_chain).size == 0 { return 0; }
    if !is_valid_cluster(sbi, (*p_chain).dir) { exfat_err(sb, c"invalid start cluster (%u)", (*p_chain).dir); return -EIO; }
    clu = (*p_chain).dir;
    cur_cmap_i = BITMAP_OFFSET_SECTOR_INDEX(sb, CLUSTER_TO_BITMAP_ENT(clu));
    next_cmap_i = cur_cmap_i;

    if (*p_chain).flags == ALLOC_NO_FAT_CHAIN {
        let last_cluster = (*p_chain).dir + (*p_chain).size - 1;
        while num_clusters < (*p_chain).size {
            let mut sync = false;
            if clu < last_cluster { next_cmap_i = BITMAP_OFFSET_SECTOR_INDEX(sb, CLUSTER_TO_BITMAP_ENT(clu + 1)); }
            if clu == last_cluster || cur_cmap_i != next_cmap_i { sync = true; cur_cmap_i = next_cmap_i; }
            if exfat_clear_bitmap(sb, clu, sync && IS_DIRSYNC(inode)) != 0 { break; }
            clu += 1; num_clusters += 1;
        }
        if (*sbi).options.discard { exfat_discard_cluster(sb, (*p_chain).dir, (*p_chain).size); }
    } else {
        let mut nr_clu = 1;
        loop {
            let mut sync = false;
            let n_clu0 = clu;
            let mut n_clu = clu;
            let err = exfat_get_next_cluster(sb, &mut n_clu);
            if err != 0 || n_clu == EXFAT_EOF_CLUSTER { sync = true; } else { next_cmap_i = BITMAP_OFFSET_SECTOR_INDEX(sb, CLUSTER_TO_BITMAP_ENT(n_clu)); }
            if cur_cmap_i != next_cmap_i { sync = true; cur_cmap_i = next_cmap_i; }
            if exfat_clear_bitmap(sb, clu, sync && IS_DIRSYNC(inode)) != 0 { break; }
            if (*sbi).options.discard { if n_clu == clu + 1 { nr_clu += 1; } else { exfat_discard_cluster(sb, clu - nr_clu + 1, nr_clu); nr_clu = 1; } }
            clu = n_clu; num_clusters += 1;
            if err != 0 { break; }
            if num_clusters >= (*sbi).num_clusters - EXFAT_FIRST_CLUSTER { exfat_count_used_clusters(sb, &mut (*sbi).used_clusters); return 0; }
            if clu == EXFAT_EOF_CLUSTER { break; }
            let _ = n_clu0;
        }
    }
    (*sbi).used_clusters -= num_clusters;
    0
}

pub unsafe fn exfat_free_cluster(inode: *mut inode, p_chain: *mut exfat_chain) -> i32 {
    let mut ret: i32 = 0;
    mutex_lock(&mut EXFAT_SB((*inode).i_sb).bitmap_lock);
    ret = __exfat_free_cluster(inode, p_chain);
    mutex_unlock(&mut EXFAT_SB((*inode).i_sb).bitmap_lock);
    ret
}

pub unsafe fn exfat_find_last_cluster(sb: *mut super_block, p_chain: *mut exfat_chain, ret_clu: *mut u32) -> i32 {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut clu: u32;
    let mut next = (*p_chain).dir;
    let mut count = 0;
    if (*p_chain).flags == ALLOC_NO_FAT_CHAIN { *ret_clu = next + (*p_chain).size - 1; return 0; }
    loop { count += 1; clu = next; if exfat_ent_get(sb, clu, &mut next, &mut bh) != 0 { return -EIO; } if next == EXFAT_EOF_CLUSTER || count > (*p_chain).size { break; } }
    brelse(bh);
    if (*p_chain).size != count { exfat_fs_error(sb, c"bogus directory size (clus : ondisk(%d) != counted(%d))", (*p_chain).size, count); return -EIO; }
    *ret_clu = clu; 0
}

pub unsafe fn exfat_zeroed_cluster(dir: *mut inode, clu: u32) -> i32 {
    let sb = (*dir).i_sb; let sbi = EXFAT_SB(sb); let mut bh: *mut buffer_head;
    let blknr = exfat_cluster_to_sector(sbi, clu); let last_blknr = blknr + (*sbi).sect_per_clus;
    if last_blknr > (*sbi).num_sectors && (*sbi).num_sectors > 0 { exfat_fs_error_ratelimit(sb, c"%s: out of range(sect:%llu len:%u)", __func__, blknr, (*sbi).sect_per_clus); return -EIO; }
    let mut i = blknr; while i < last_blknr { bh = sb_getblk(sb, i); if bh.is_null() { return -ENOMEM; } memset((*bh).b_data, 0, (*sb).s_blocksize); set_buffer_uptodate(bh); mark_buffer_dirty(bh); brelse(bh); i += 1; }
    if IS_DIRSYNC(dir) { return sync_blockdev_range((*sb).s_bdev, exfat_block_to_bytes(sb, blknr), exfat_block_to_bytes(sb, last_blknr) - 1); }
    0
}

pub unsafe fn exfat_alloc_cluster(inode: *mut inode, num_alloc: u32, p_chain: *mut exfat_chain, sync_bmap: bool, contig: bool) -> i32 {
    let mut ret = -ENOSPC; let mut total_cnt; let mut hint_clu; let mut new_clu; let mut last_clu = EXFAT_EOF_CLUSTER;
    let sb = (*inode).i_sb; let sbi = EXFAT_SB(sb); total_cnt = EXFAT_DATA_CLUSTER_COUNT(sbi);
    if total_cnt < (*sbi).used_clusters { exfat_fs_error_ratelimit(sb, c"%s: invalid used clusters(t:%u,u:%u)\n", __func__, total_cnt, (*sbi).used_clusters); return -EIO; }
    if num_alloc > total_cnt - (*sbi).used_clusters { return -ENOSPC; }
    mutex_lock(&mut (*sbi).bitmap_lock); hint_clu = (*p_chain).dir;
    if hint_clu == EXFAT_EOF_CLUSTER { if (*sbi).clu_srch_ptr < EXFAT_FIRST_CLUSTER { exfat_err(sb, c"sbi->clu_srch_ptr is invalid (%u)", (*sbi).clu_srch_ptr); (*sbi).clu_srch_ptr = EXFAT_FIRST_CLUSTER; } hint_clu = exfat_find_free_bitmap(sb, (*sbi).clu_srch_ptr); if hint_clu == EXFAT_EOF_CLUSTER { goto unlock; } }
    if !is_valid_cluster(sbi, hint_clu) { if hint_clu != (*sbi).num_clusters { exfat_err(sb, c"hint_cluster is invalid (%u), rewind to the first cluster", hint_clu); } hint_clu = EXFAT_FIRST_CLUSTER; (*p_chain).flags = ALLOC_FAT_CHAIN; }
    (*p_chain).dir = EXFAT_EOF_CLUSTER;
    loop {
        new_clu = exfat_find_free_bitmap(sb, hint_clu); if new_clu == EXFAT_EOF_CLUSTER { break; }
        if new_clu != hint_clu { if (*p_chain).flags == ALLOC_NO_FAT_CHAIN { if exfat_chain_cont_cluster(sb, (*p_chain).dir, (*p_chain).size) != 0 { ret = -EIO; goto free_cluster; } (*p_chain).flags = ALLOC_FAT_CHAIN; } if contig && (*p_chain).size > 0 { hint_clu = last_clu; goto done; } }
        if exfat_set_bitmap(sb, new_clu, sync_bmap) != 0 { ret = -EIO; goto free_cluster; }
        if (*p_chain).flags == ALLOC_FAT_CHAIN && exfat_ent_set(sb, new_clu, EXFAT_EOF_CLUSTER) != 0 { ret = -EIO; goto free_cluster; }
        if (*p_chain).dir == EXFAT_EOF_CLUSTER { (*p_chain).dir = new_clu; } else if (*p_chain).flags == ALLOC_FAT_CHAIN && exfat_ent_set(sb, last_clu, new_clu) != 0 { ret = -EIO; goto free_cluster; }
        (*p_chain).size += 1; last_clu = new_clu;
        if (*p_chain).size == num_alloc { done: (*sbi).clu_srch_ptr = hint_clu; (*sbi).used_clusters += (*p_chain).size; mutex_unlock(&mut (*sbi).bitmap_lock); return 0; }
        hint_clu = new_clu + 1; if hint_clu >= (*sbi).num_clusters { hint_clu = EXFAT_FIRST_CLUSTER; if (*p_chain).flags == ALLOC_NO_FAT_CHAIN { if exfat_chain_cont_cluster(sb, (*p_chain).dir, (*p_chain).size) != 0 { ret = -EIO; goto free_cluster; } (*p_chain).flags = ALLOC_FAT_CHAIN; } }
    }
free_cluster: __exfat_free_cluster(inode, p_chain);
unlock: mutex_unlock(&mut (*sbi).bitmap_lock); ret
}

pub unsafe fn exfat_count_num_clusters(sb: *mut super_block, p_chain: *mut exfat_chain, ret_count: *mut u32) -> i32 {
    let sbi = EXFAT_SB(sb); let mut bh: *mut buffer_head = core::ptr::null_mut();
    if (*p_chain).dir == 0 || (*p_chain).dir == EXFAT_EOF_CLUSTER { *ret_count = 0; return 0; }
    if (*p_chain).flags == ALLOC_NO_FAT_CHAIN { *ret_count = (*p_chain).size; return 0; }
    let mut clu = (*p_chain).dir; let mut count = 0; let mut i = EXFAT_FIRST_CLUSTER;
    while i < (*sbi).num_clusters { count += 1; if exfat_ent_get(sb, clu, &mut clu, &mut bh) != 0 { return -EIO; } if clu == EXFAT_EOF_CLUSTER { break; } i += 1; }
    brelse(bh); *ret_count = count;
    /*
     * since exfat_count_used_clusters() is not called, sbi->used_clusters
     * cannot be used here.
     */
    if i == (*sbi).num_clusters && clu != EXFAT_EOF_CLUSTER { exfat_fs_error(sb, c"The cluster chain has a loop"); return -EIO; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

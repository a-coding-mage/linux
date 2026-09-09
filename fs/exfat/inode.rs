// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
 */

// Linux and exFAT dependencies are supplied by the surrounding translation.

pub unsafe fn __exfat_write_inode(inode: *mut inode, sync: c_int) -> c_int {
    let mut on_disk_size: u64;
    let mut on_disk_valid_size: u64;
    let mut ep: *mut exfat_dentry;
    let mut ep2: *mut exfat_dentry;
    let mut es: exfat_entry_set_cache = core::mem::zeroed();
    let sb: *mut super_block = (*inode).i_sb;
    let sbi: *mut exfat_sb_info = EXFAT_SB(sb);
    let ei: *mut exfat_inode_info = EXFAT_I(inode);
    let is_dir: bool = (*ei).type_ == TYPE_DIR;
    let mut ts: timespec64;

    if (*inode).i_ino == EXFAT_ROOT_INO {
        return 0;
    }

    /*
     * If the inode is already unlinked, there is no need for updating it.
     */
    if (*ei).dir.dir == DIR_DELETED {
        return 0;
    }

    if is_dir && (*ei).dir.dir == (*sbi).root_dir && (*ei).entry == -1 {
        return 0;
    }

    exfat_set_volume_dirty(sb);

    /* get the directory entry of given file or directory */
    if exfat_get_dentry_set_by_ei(&mut es, sb, ei) != 0 {
        return -EIO;
    }
    ep = exfat_get_dentry_cached(&mut es, ES_IDX_FILE);
    ep2 = exfat_get_dentry_cached(&mut es, ES_IDX_STREAM);

    (*ep).dentry.file.attr = cpu_to_le16(exfat_make_attr(inode));

    /* set FILE_INFO structure using the acquired struct exfat_dentry */
    exfat_set_entry_time(sbi, &(*ei).i_crtime,
        &mut (*ep).dentry.file.create_tz, &mut (*ep).dentry.file.create_time,
        &mut (*ep).dentry.file.create_date, &mut (*ep).dentry.file.create_time_cs);
    ts = inode_get_mtime(inode);
    exfat_set_entry_time(sbi, &ts,
        &mut (*ep).dentry.file.modify_tz, &mut (*ep).dentry.file.modify_time,
        &mut (*ep).dentry.file.modify_date, &mut (*ep).dentry.file.modify_time_cs);
    ts = inode_get_atime(inode);
    exfat_set_entry_time(sbi, &ts,
        &mut (*ep).dentry.file.access_tz, &mut (*ep).dentry.file.access_time,
        &mut (*ep).dentry.file.access_date, core::ptr::null_mut());

    /*
     * During a DIO write, valid_size is updated eagerly in iomap_end (so
     * that concurrent buffered reads see IOMAP_MAPPED) while i_size is
     * updated asynchronously in end_io.  The FAT chain was already
     * extended to cover ceil(valid_size/cluster_size) clusters.  Use the
     * maximum so the on-disk size field always covers the FAT chain,
     * preventing fsck from reporting "more clusters are allocated".
     */
    on_disk_size = core::cmp::max(i_size_read(inode), (*ei).valid_size);

    if (*ei).start_clu == EXFAT_EOF_CLUSTER {
        on_disk_size = 0;
    }
    /*
     * valid_size on disk must reflect only confirmed data (up to i_size)
     * and must not exceed on_disk_size.
     */
    on_disk_valid_size = core::cmp::min((*ei).valid_size, i_size_read(inode));
    if (*ei).start_clu == EXFAT_EOF_CLUSTER {
        on_disk_valid_size = 0;
    }

    (*ep2).dentry.stream.size = cpu_to_le64(on_disk_size);
    (*ep2).dentry.stream.valid_size = cpu_to_le64(on_disk_valid_size);

    if on_disk_size != 0 {
        (*ep2).dentry.stream.flags = (*ei).flags;
        (*ep2).dentry.stream.start_clu = cpu_to_le32((*ei).start_clu);
    } else {
        (*ep2).dentry.stream.flags = ALLOC_FAT_CHAIN;
        (*ep2).dentry.stream.start_clu = EXFAT_FREE_CLUSTER;
    }

    exfat_update_dir_chksum(&mut es);
    exfat_put_dentry_set(&mut es, sync)
}

pub unsafe fn exfat_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int {
    let ret: c_int;
    if unlikely(exfat_forced_shutdown((*inode).i_sb)) {
        return -EIO;
    }
    mutex_lock(&mut (*EXFAT_SB((*inode).i_sb)).s_lock);
    ret = __exfat_write_inode(inode, ((*wbc).sync_mode == WB_SYNC_ALL) as c_int);
    mutex_unlock(&mut (*EXFAT_SB((*inode).i_sb)).s_lock);
    ret
}

pub unsafe fn exfat_sync_inode(inode: *mut inode) {
    lockdep_assert_held(&(*EXFAT_SB((*inode).i_sb)).s_lock);
    __exfat_write_inode(inode, 1);
}

/*
 * Input: inode, (logical) clu_offset, target allocation area
 * Output: errcode, cluster number
 * *clu = (~0), if it's unable to allocate a new cluster
 */
pub unsafe fn exfat_map_cluster(inode: *mut inode, clu_offset: c_uint,
    clu: *mut c_uint, count: *mut c_uint, create: c_int,
    balloc: *mut bool) -> c_int {
    let mut ret: c_int;
    let mut last_clu: c_uint;
    let mut new_clu: exfat_chain = core::mem::zeroed();
    let sb = (*inode).i_sb;
    let sbi = EXFAT_SB(sb);
    let ei = EXFAT_I(inode);
    let local_clu_offset = clu_offset;
    let mut num_to_be_allocated: c_uint = 0;
    let num_clusters = exfat_bytes_to_cluster(sbi, exfat_ondisk_size(inode));

    if clu_offset > num_clusters || *count > num_clusters - clu_offset {
        num_to_be_allocated = clu_offset + *count - num_clusters;
    }
    if create == 0 && num_to_be_allocated > 0 {
        *clu = EXFAT_EOF_CLUSTER;
        return 0;
    }
    *clu = (*ei).start_clu;
    last_clu = *clu;
    if *clu == EXFAT_EOF_CLUSTER {
        *count = 0;
    } else if (*ei).flags == ALLOC_NO_FAT_CHAIN {
        last_clu += num_clusters - 1;
        if clu_offset < num_clusters {
            *clu += clu_offset;
            *count = core::cmp::min(num_clusters - clu_offset, *count);
        } else {
            *clu = EXFAT_EOF_CLUSTER;
            *count = 0;
        }
    } else {
        let err = exfat_get_cluster(inode, clu_offset, clu, count, &mut last_clu);
        if err != 0 { return -EIO; }
    }
    if *clu == EXFAT_EOF_CLUSTER {
        exfat_set_volume_dirty(sb);
        new_clu.dir = if last_clu == EXFAT_EOF_CLUSTER { EXFAT_EOF_CLUSTER } else { last_clu + 1 };
        new_clu.size = 0;
        new_clu.flags = (*ei).flags;
        if num_to_be_allocated < 1 {
            exfat_fs_error(sb, "broken FAT chain.");
            return -EIO;
        }
        ret = exfat_alloc_cluster(inode, num_to_be_allocated, &mut new_clu,
            inode_needs_sync(inode), true);
        if ret != 0 { return ret; }
        if new_clu.dir == EXFAT_EOF_CLUSTER || new_clu.dir == EXFAT_FREE_CLUSTER {
            exfat_fs_error(sb, "bogus cluster new allocated (last_clu : %u, new_clu : %u)", last_clu, new_clu.dir);
            return -EIO;
        }
        if last_clu == EXFAT_EOF_CLUSTER {
            if new_clu.flags == ALLOC_FAT_CHAIN { (*ei).flags = ALLOC_FAT_CHAIN; }
            (*ei).start_clu = new_clu.dir;
        } else {
            if new_clu.flags != (*ei).flags {
                if exfat_chain_cont_cluster(sb, (*ei).start_clu, num_clusters) != 0 { return -EIO; }
                (*ei).flags = ALLOC_FAT_CHAIN;
            }
            if new_clu.flags == ALLOC_FAT_CHAIN && exfat_ent_set(sb, last_clu, new_clu.dir) != 0 { return -EIO; }
        }
        *clu = new_clu.dir;
        *count = new_clu.size;
        (*inode).i_blocks += exfat_cluster_to_sectors(sbi, new_clu.size);
        if !balloc.is_null() { *balloc = true; }
    }
    (*ei).hint_bmap.off = local_clu_offset;
    (*ei).hint_bmap.clu = *clu;
    0
}

unsafe fn exfat_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    let mut ctx = iomap_read_folio_ctx { cur_folio: folio, ops: &exfat_iomap_bio_read_ops, rac: core::ptr::null_mut() };
    iomap_read_folio(&exfat_iomap_ops, &mut ctx, core::ptr::null_mut());
    0
}

unsafe fn exfat_readahead(rac: *mut readahead_control) {
    let mapping = (*rac).mapping;
    let inode = (*mapping).host;
    let ei = EXFAT_I(inode);
    let pos = readahead_pos(rac);
    let mut ctx = iomap_read_folio_ctx { cur_folio: core::ptr::null_mut(), ops: &exfat_iomap_bio_read_ops, rac };
    /* Range cross valid_size, read it page by page. */
    if (*ei).valid_size < i_size_read(inode) && pos <= (*ei).valid_size && (*ei).valid_size < pos + readahead_length(rac) { return; }
    iomap_readahead(&exfat_iomap_ops, &mut ctx, core::ptr::null_mut());
}

unsafe fn exfat_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let mut wpc = iomap_writepage_ctx { inode: (*mapping).host, wbc, ops: &exfat_writeback_ops };
    if unlikely(exfat_forced_shutdown((*mapping).host.i_sb)) { return -EIO; }
    iomap_writepages(&mut wpc)
}

unsafe fn exfat_aop_bmap(mapping: *mut address_space, block: sector_t) -> sector_t {
    inode_lock_shared((*mapping).host);
    let blocknr = iomap_bmap(mapping, block, &exfat_iomap_ops);
    inode_unlock_shared((*mapping).host);
    blocknr
}

static inline fn exfat_hash(i_pos: loff_t) -> c_ulong { hash_32(i_pos, EXFAT_HASH_BITS) }

pub unsafe fn exfat_hash_inode(inode: *mut inode, i_pos: loff_t) {
    let sbi = EXFAT_SB((*inode).i_sb);
    let head = (*sbi).inode_hashtable.add(exfat_hash(i_pos) as usize);
    spin_lock(&mut (*sbi).inode_hash_lock);
    (*EXFAT_I(inode)).i_pos = i_pos;
    hlist_add_head(&mut (*EXFAT_I(inode)).i_hash_fat, head);
    spin_unlock(&mut (*sbi).inode_hash_lock);
}

pub unsafe fn exfat_unhash_inode(inode: *mut inode) {
    let sbi = EXFAT_SB((*inode).i_sb);
    spin_lock(&mut (*sbi).inode_hash_lock);
    hlist_del_init(&mut (*EXFAT_I(inode)).i_hash_fat);
    (*EXFAT_I(inode)).i_pos = 0;
    spin_unlock(&mut (*sbi).inode_hash_lock);
}

pub unsafe fn exfat_iget(sb: *mut super_block, i_pos: loff_t) -> *mut inode {
    let sbi = EXFAT_SB(sb);
    let head = (*sbi).inode_hashtable.add(exfat_hash(i_pos) as usize);
    let mut inode: *mut inode = core::ptr::null_mut();
    spin_lock(&mut (*sbi).inode_hash_lock);
    let mut info = hlist_first_entry::<exfat_inode_info>(head);
    while !info.is_null() {
        WARN_ON((*info).vfs_inode.i_sb != sb);
        if i_pos == (*info).i_pos { inode = igrab(&mut (*info).vfs_inode); if !inode.is_null() { break; } }
        info = hlist_next_entry(info);
    }
    spin_unlock(&mut (*sbi).inode_hash_lock);
    inode
}

/* doesn't deal with root inode */
unsafe fn exfat_fill_inode(inode: *mut inode, info: *mut exfat_dir_entry) -> c_int {
    let sbi = EXFAT_SB((*inode).i_sb);
    let ei = EXFAT_I(inode);
    let size = (*info).size;
    (*ei).dir = (*info).dir; (*ei).entry = (*info).entry; (*ei).attr = (*info).attr;
    (*ei).start_clu = (*info).start_clu; (*ei).flags = (*info).flags; (*ei).type_ = (*info).type_;
    (*ei).valid_size = (*info).valid_size; (*ei).zeroed_size = (*info).valid_size;
    (*ei).version = 0; (*ei).hint_stat.eidx = 0; (*ei).hint_stat.clu = (*info).start_clu;
    (*ei).hint_femp.eidx = EXFAT_HINT_NONE; (*ei).hint_bmap.off = EXFAT_EOF_CLUSTER; (*ei).i_pos = 0;
    (*inode).i_uid = (*sbi).options.fs_uid; (*inode).i_gid = (*sbi).options.fs_gid;
    inode_inc_iversion(inode); (*inode).i_generation = get_random_u32();
    (*inode).i_mode = exfat_make_mode(sbi, (*info).attr, 0o777);
    if (*info).attr & EXFAT_ATTR_SUBDIR != 0 {
        (*inode).i_generation &= !1; (*inode).i_op = &exfat_dir_inode_operations; (*inode).i_fop = &exfat_dir_operations; set_nlink(inode, (*info).num_subdirs);
    } else {
        (*inode).i_generation |= 1; (*inode).i_op = &exfat_file_inode_operations; (*inode).i_fop = &exfat_file_operations;
        (*inode).i_mapping.a_ops = &exfat_aops; (*inode).i_mapping.nrpages = 0;
    }
    i_size_write(inode, size); exfat_save_attr(inode, (*info).attr);
    (*inode).i_blocks = round_up(i_size_read(inode), (*sbi).cluster_size) >> 9;
    inode_set_mtime_to_ts(inode, (*info).mtime); inode_set_ctime_to_ts(inode, (*info).mtime); (*ei).i_crtime = (*info).crtime; inode_set_atime_to_ts(inode, (*info).atime);
    0
}

pub unsafe fn exfat_build_inode(sb: *mut super_block, info: *mut exfat_dir_entry, i_pos: loff_t) -> *mut inode {
    let mut inode = exfat_iget(sb, i_pos);
    if !inode.is_null() { return inode; }
    inode = new_inode(sb);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    (*inode).i_ino = iunique(sb, EXFAT_ROOT_INO); inode_set_iversion(inode, 1);
    let err = exfat_fill_inode(inode, info);
    if err != 0 { iput(inode); return ERR_PTR(err); }
    exfat_hash_inode(inode, i_pos); insert_inode_hash(inode); inode
}

pub unsafe fn exfat_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data);
    if (*inode).i_nlink == 0 { i_size_write(inode, 0); mutex_lock(&mut (*EXFAT_SB((*inode).i_sb)).s_lock); __exfat_truncate(inode); mutex_unlock(&mut (*EXFAT_SB((*inode).i_sb)).s_lock); }
    clear_inode(inode); exfat_cache_inval_inode(inode); exfat_unhash_inode(inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

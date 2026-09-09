// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of ext4/balloc.c. External kernel symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

unsafe extern "C" {
    fn ext4_num_base_meta_clusters(sb: *mut super_block, block_group: ext4_group_t) -> c_uint;
}

pub unsafe fn ext4_get_group_number(sb: *mut super_block, block: ext4_fsblk_t) -> ext4_group_t {
    let mut group: ext4_group_t;
    if test_opt2(sb, STD_GROUP_SIZE) {
        group = (block - le32_to_cpu((*EXT4_SB(sb)).s_es.as_ref().unwrap().s_first_data_block))
            >> (EXT4_BLOCK_SIZE_BITS(sb) + EXT4_CLUSTER_BITS(sb) + 3);
    } else {
        ext4_get_group_no_and_offset(sb, block, &mut group, core::ptr::null_mut());
    }
    group
}

pub unsafe fn ext4_get_group_no_and_offset(sb: *mut super_block, mut blocknr: ext4_fsblk_t,
    blockgrpp: *mut ext4_group_t, offsetp: *mut ext4_grpblk_t) {
    let es = (*EXT4_SB(sb)).s_es;
    blocknr -= le32_to_cpu((*es).s_first_data_block);
    let offset = do_div(&mut blocknr, EXT4_BLOCKS_PER_GROUP(sb)) >> (*EXT4_SB(sb)).s_cluster_bits;
    if !offsetp.is_null() { *offsetp = offset; }
    if !blockgrpp.is_null() { *blockgrpp = blocknr; }
}

unsafe fn ext4_block_in_group(sb: *mut super_block, block: ext4_fsblk_t, block_group: ext4_group_t) -> c_int {
    (ext4_get_group_number(sb, block) == block_group) as c_int
}

unsafe fn ext4_num_overhead_clusters(sb: *mut super_block, block_group: ext4_group_t,
    gdp: *mut ext4_group_desc) -> c_uint {
    let mut base_clusters = ext4_num_base_meta_clusters(sb, block_group);
    let mut num_clusters = base_clusters;
    let mut block_cluster: c_int = -1;
    let mut inode_cluster: c_int;
    let mut itbl_cluster_start: c_int = -1;
    let mut itbl_cluster_end: c_int = -1;
    let start = ext4_group_first_block_no(sb, block_group);
    let end = start + EXT4_BLOCKS_PER_GROUP(sb) - 1;
    let sbi = EXT4_SB(sb);
    let mut itbl_blk_start = ext4_inode_table(sb, gdp);
    let mut itbl_blk_end = itbl_blk_start + (*sbi).s_itb_per_group - 1;
    if itbl_blk_start <= end && itbl_blk_end >= start {
        itbl_blk_start = core::cmp::max(itbl_blk_start, start);
        itbl_blk_end = core::cmp::min(itbl_blk_end, end);
        itbl_cluster_start = EXT4_B2C(sbi, itbl_blk_start - start) as c_int;
        itbl_cluster_end = EXT4_B2C(sbi, itbl_blk_end - start) as c_int;
        num_clusters += (itbl_cluster_end - itbl_cluster_start + 1) as c_uint;
        if itbl_cluster_start == (base_clusters - 1) as c_int { num_clusters -= 1; }
    }
    if ext4_block_in_group(sb, ext4_block_bitmap(sb, gdp), block_group) != 0 {
        block_cluster = EXT4_B2C(sbi, ext4_block_bitmap(sb, gdp) - start) as c_int;
        if block_cluster >= base_clusters as c_int &&
            (block_cluster < itbl_cluster_start || block_cluster > itbl_cluster_end) { num_clusters += 1; }
    }
    if ext4_block_in_group(sb, ext4_inode_bitmap(sb, gdp), block_group) != 0 {
        inode_cluster = EXT4_B2C(sbi, ext4_inode_bitmap(sb, gdp) - start) as c_int;
        if inode_cluster != block_cluster && inode_cluster >= base_clusters as c_int &&
            (inode_cluster < itbl_cluster_start || inode_cluster > itbl_cluster_end) { num_clusters += 1; }
    }
    num_clusters
}

unsafe fn num_clusters_in_group(sb: *mut super_block, block_group: ext4_group_t) -> c_uint {
    let blocks = if block_group == ext4_get_groups_count(sb) - 1 {
        ext4_blocks_count((*EXT4_SB(sb)).s_es) - ext4_group_first_block_no(sb, block_group)
    } else { EXT4_BLOCKS_PER_GROUP(sb) };
    EXT4_NUM_B2C(EXT4_SB(sb), blocks)
}

unsafe fn ext4_init_block_bitmap(sb: *mut super_block, bh: *mut buffer_head,
    block_group: ext4_group_t, gdp: *mut ext4_group_desc) -> c_int {
    let sbi = EXT4_SB(sb);
    if !ext4_group_desc_csum_verify(sb, block_group, gdp) {
        ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT | EXT4_GROUP_INFO_IBITMAP_CORRUPT);
        return -EFSBADCRC;
    }
    core::ptr::write_bytes((*bh).b_data, 0, (*sb).s_blocksize as usize);
    let bit_max = ext4_num_base_meta_clusters(sb, block_group);
    if (bit_max >> 3) >= (*bh).b_size { return -EFSCORRUPTED; }
    for bit in 0..bit_max { ext4_set_bit(bit, (*bh).b_data); }
    let start = ext4_group_first_block_no(sb, block_group);
    let mut tmp = ext4_block_bitmap(sb, gdp);
    if ext4_block_in_group(sb, tmp, block_group) != 0 { ext4_set_bit(EXT4_B2C(sbi, tmp - start), (*bh).b_data); }
    tmp = ext4_inode_bitmap(sb, gdp);
    if ext4_block_in_group(sb, tmp, block_group) != 0 { ext4_set_bit(EXT4_B2C(sbi, tmp - start), (*bh).b_data); }
    tmp = ext4_inode_table(sb, gdp);
    let end = tmp + (*sbi).s_itb_per_group;
    while tmp < end {
        if ext4_block_in_group(sb, tmp, block_group) != 0 { ext4_set_bit(EXT4_B2C(sbi, tmp - start), (*bh).b_data); }
        tmp += 1;
    }
    ext4_mark_bitmap_end(num_clusters_in_group(sb, block_group), (*sb).s_blocksize * 8, (*bh).b_data);
    0
}

pub unsafe fn ext4_free_clusters_after_init(sb: *mut super_block, block_group: ext4_group_t, gdp: *mut ext4_group_desc) -> c_uint {
    num_clusters_in_group(sb, block_group) - ext4_num_overhead_clusters(sb, block_group, gdp)
}

pub unsafe fn ext4_get_group_desc(sb: *mut super_block, block_group: ext4_group_t, bh: *mut *mut buffer_head) -> *mut ext4_group_desc {
    let ngroups = ext4_get_groups_count(sb);
    let sbi = EXT4_SB(sb);
    if block_group >= ngroups { ext4_error(sb, "block_group >= groups_count - block_group = %u, groups_count = %u", block_group, ngroups); return core::ptr::null_mut(); }
    let group_desc = block_group >> EXT4_DESC_PER_BLOCK_BITS(sb);
    let offset = block_group & (EXT4_DESC_PER_BLOCK(sb) - 1);
    let bh_p = sbi_array_rcu_deref(sbi, s_group_desc, group_desc);
    if bh_p.is_null() { ext4_error(sb, "Group descriptor not loaded - block_group = %u, group_desc = %u, desc = %u", block_group, group_desc, offset); return core::ptr::null_mut(); }
    let desc = ((*bh_p).b_data as *mut u8).add((offset * EXT4_DESC_SIZE(sb)) as usize) as *mut ext4_group_desc;
    if !bh.is_null() { *bh = bh_p; }
    desc
}

unsafe fn ext4_valid_block_bitmap_padding(sb: *mut super_block, block_group: ext4_group_t, bh: *mut buffer_head) -> ext4_fsblk_t {
    let bitmap_size = (*sb).s_blocksize * 8;
    let offset = num_clusters_in_group(sb, block_group);
    if bitmap_size <= offset { return 0; }
    let next = ext4_find_next_zero_bit((*bh).b_data, bitmap_size, offset);
    if next < bitmap_size { next } else { 0 }
}

pub unsafe fn ext4_get_group_info(sb: *mut super_block, group: ext4_group_t) -> *mut ext4_group_info {
    let sbi = EXT4_SB(sb);
    if unlikely(group >= (*sbi).s_groups_count) || unlikely((*sbi).s_group_info.is_null()) { return core::ptr::null_mut(); }
    let indexv = group >> EXT4_DESC_PER_BLOCK_BITS(sb);
    let indexh = group & (EXT4_DESC_PER_BLOCK(sb) - 1);
    let grp_info = sbi_array_rcu_deref(sbi, s_group_info, indexv);
    if unlikely(grp_info.is_null()) { return core::ptr::null_mut(); }
    *grp_info.add(indexh as usize)
}

unsafe fn ext4_valid_block_bitmap(sb: *mut super_block, desc: *mut ext4_group_desc, block_group: ext4_group_t, bh: *mut buffer_head) -> ext4_fsblk_t {
    let sbi = EXT4_SB(sb);
    if ext4_has_feature_flex_bg(sb) { return 0; }
    let max_bit = EXT4_CLUSTERS_PER_GROUP(sb);
    let first = ext4_group_first_block_no(sb, block_group);
    let mut blk = ext4_block_bitmap(sb, desc); let mut offset = blk - first;
    if offset < 0 || EXT4_B2C(sbi, offset) >= max_bit || !ext4_test_bit(EXT4_B2C(sbi, offset), (*bh).b_data) { return blk; }
    blk = ext4_inode_bitmap(sb, desc); offset = blk - first;
    if offset < 0 || EXT4_B2C(sbi, offset) >= max_bit || !ext4_test_bit(EXT4_B2C(sbi, offset), (*bh).b_data) { return blk; }
    blk = ext4_inode_table(sb, desc); offset = blk - first;
    if offset < 0 || EXT4_B2C(sbi, offset) >= max_bit || EXT4_B2C(sbi, offset + (*sbi).s_itb_per_group - 1) >= max_bit { return blk; }
    let next = ext4_find_next_zero_bit((*bh).b_data, EXT4_B2C(sbi, offset + (*sbi).s_itb_per_group - 1) + 1, EXT4_B2C(sbi, offset));
    if next < EXT4_B2C(sbi, offset + (*sbi).s_itb_per_group - 1) + 1 { blk } else { 0 }
}

unsafe fn ext4_validate_block_bitmap(sb: *mut super_block, desc: *mut ext4_group_desc, block_group: ext4_group_t, bh: *mut buffer_head) -> c_int {
    if (*EXT4_SB(sb)).s_mount_state & EXT4_FC_REPLAY != 0 || buffer_verified(bh) { return 0; }
    let grp = ext4_get_group_info(sb, block_group);
    if grp.is_null() || EXT4_MB_GRP_BBITMAP_CORRUPT(grp) { return -EFSCORRUPTED; }
    ext4_lock_group(sb, block_group);
    if buffer_verified(bh) { ext4_unlock_group(sb, block_group); return 0; }
    if unlikely(!ext4_block_bitmap_csum_verify(sb, desc, bh) || ext4_simulate_fail(sb, EXT4_SIM_BBITMAP_CRC)) {
        ext4_unlock_group(sb, block_group); ext4_error(sb, "bg %u: bad block bitmap checksum", block_group); ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT); return -EFSBADCRC;
    }
    let mut blk = ext4_valid_block_bitmap(sb, desc, block_group, bh);
    if unlikely(blk != 0) { ext4_unlock_group(sb, block_group); ext4_error(sb, "bg %u: block %llu: invalid block bitmap", block_group, blk); ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT); return -EFSCORRUPTED; }
    blk = ext4_valid_block_bitmap_padding(sb, block_group, bh);
    if unlikely(blk != 0) { ext4_unlock_group(sb, block_group); ext4_error(sb, "bg %u: block %llu: padding at end of block bitmap is not set", block_group, blk); ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT); return -EFSCORRUPTED; }
    set_buffer_verified(bh); ext4_unlock_group(sb, block_group); 0
}

pub unsafe fn ext4_read_block_bitmap_nowait(sb: *mut super_block, block_group: ext4_group_t, ignore_locked: bool) -> *mut buffer_head {
    let sbi = EXT4_SB(sb);
    let desc = ext4_get_group_desc(sb, block_group, core::ptr::null_mut());
    if desc.is_null() { return ERR_PTR(-EFSCORRUPTED); }
    let bitmap_blk = ext4_block_bitmap(sb, desc);
    if bitmap_blk <= le32_to_cpu((*(*sbi).s_es).s_first_data_block) || bitmap_blk >= ext4_blocks_count((*sbi).s_es) {
        ext4_error(sb, "Invalid block bitmap block %llu in block_group %u", bitmap_blk, block_group);
        ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT); return ERR_PTR(-EFSCORRUPTED);
    }
    let bh = sb_getblk(sb, bitmap_blk);
    if bh.is_null() { ext4_warning(sb, "Cannot get buffer for block bitmap - block_group = %u, block_bitmap = %llu", block_group, bitmap_blk); return ERR_PTR(-ENOMEM); }
    if ignore_locked && buffer_locked(bh) { put_bh(bh); return core::ptr::null_mut(); }
    if bitmap_uptodate(bh) { return if ext4_validate_block_bitmap(sb, desc, block_group, bh) != 0 { put_bh(bh); ERR_PTR(-EFSCORRUPTED) } else { bh }; }
    lock_buffer(bh);
    if bitmap_uptodate(bh) { unlock_buffer(bh); return if ext4_validate_block_bitmap(sb, desc, block_group, bh) != 0 { put_bh(bh); ERR_PTR(-EFSCORRUPTED) } else { bh }; }
    ext4_lock_group(sb, block_group);
    if ext4_has_group_desc_csum(sb) && ((*desc).bg_flags & cpu_to_le16(EXT4_BG_BLOCK_UNINIT)) != 0 {
        if block_group == 0 { ext4_unlock_group(sb, block_group); unlock_buffer(bh); ext4_error(sb, "Block bitmap for bg 0 marked uninitialized"); put_bh(bh); return ERR_PTR(-EFSCORRUPTED); }
        let err = ext4_init_block_bitmap(sb, bh, block_group, desc);
        if err != 0 { ext4_unlock_group(sb, block_group); unlock_buffer(bh); ext4_error(sb, "Failed to init block bitmap for group %u: %d", block_group, err); put_bh(bh); return ERR_PTR(err); }
        set_bitmap_uptodate(bh); set_buffer_uptodate(bh); set_buffer_verified(bh); ext4_unlock_group(sb, block_group); unlock_buffer(bh); return bh;
    }
    ext4_unlock_group(sb, block_group);
    if buffer_uptodate(bh) { set_bitmap_uptodate(bh); unlock_buffer(bh); return bh; }
    set_buffer_new(bh); trace_ext4_read_block_bitmap_load(sb, block_group, ignore_locked);
    ext4_read_bh_nowait(bh, REQ_META | REQ_PRIO | if ignore_locked { REQ_RAHEAD } else { 0 }, ext4_end_bitmap_read, ext4_simulate_fail(sb, EXT4_SIM_BBITMAP_EIO)); bh
}

pub unsafe fn ext4_wait_block_bitmap(sb: *mut super_block, block_group: ext4_group_t, bh: *mut buffer_head) -> c_int {
    if !buffer_new(bh) { return 0; }
    let desc = ext4_get_group_desc(sb, block_group, core::ptr::null_mut());
    if desc.is_null() { return -EFSCORRUPTED; }
    wait_on_buffer(bh);
    if !buffer_uptodate(bh) { ext4_error_err(sb, EIO, "Cannot read block bitmap - block_group = %u, block_bitmap = %llu", block_group, (*bh).b_blocknr); ext4_mark_group_bitmap_corrupted(sb, block_group, EXT4_GROUP_INFO_BBITMAP_CORRUPT); return -EIO; }
    clear_buffer_new(bh); ext4_validate_block_bitmap(sb, desc, block_group, bh)
}

pub unsafe fn ext4_read_block_bitmap(sb: *mut super_block, block_group: ext4_group_t) -> *mut buffer_head {
    let bh = ext4_read_block_bitmap_nowait(sb, block_group, false); if IS_ERR(bh) { return bh; }
    let err = ext4_wait_block_bitmap(sb, block_group, bh); if err != 0 { put_bh(bh); return ERR_PTR(err); } bh
}

unsafe fn ext4_has_free_clusters(sbi: *mut ext4_sb_info, nclusters: s64, flags: c_uint) -> c_int {
    let fcc = &mut (*sbi).s_freeclusters_counter; let dcc = &mut (*sbi).s_dirtyclusters_counter;
    let mut free_clusters = percpu_counter_read_positive(fcc); let mut dirty_clusters = percpu_counter_read_positive(dcc);
    let resv_clusters = atomic64_read(&mut (*sbi).s_resv_clusters);
    let rsv = (ext4_r_blocks_count((*sbi).s_es) >> (*sbi).s_cluster_bits) + resv_clusters;
    if free_clusters - (nclusters + rsv + dirty_clusters) < EXT4_FREECLUSTERS_WATERMARK { free_clusters = percpu_counter_sum_positive(fcc); dirty_clusters = percpu_counter_sum_positive(dcc); }
    if free_clusters >= rsv + nclusters + dirty_clusters { return 1; }
    if uid_eq((*sbi).s_resuid, current_fsuid()) || (!gid_eq((*sbi).s_resgid, GLOBAL_ROOT_GID) && in_group_p((*sbi).s_resgid)) || flags & EXT4_MB_USE_ROOT_BLOCKS != 0 || capable(CAP_SYS_RESOURCE) {
        if free_clusters >= nclusters + dirty_clusters + resv_clusters { return 1; }
    }
    if flags & EXT4_MB_USE_RESERVED != 0 && free_clusters >= nclusters + dirty_clusters { return 1; } 0
}

pub unsafe fn ext4_claim_free_clusters(sbi: *mut ext4_sb_info, nclusters: s64, flags: c_uint) -> c_int {
    if ext4_has_free_clusters(sbi, nclusters, flags) != 0 { percpu_counter_add(&mut (*sbi).s_dirtyclusters_counter, nclusters); 0 } else { -ENOSPC }
}

pub unsafe fn ext4_should_retry_alloc(sb: *mut super_block, retries: *mut c_int) -> c_int {
    let sbi = EXT4_SB(sb); if (*sbi).s_journal.is_null() { return 0; }
    *retries += 1; if *retries > 3 { percpu_counter_inc(&mut (*sbi).s_sra_exceeded_retry_limit); return 0; }
    smp_mb(); if atomic_read(&mut (*sbi).s_mb_free_pending) == 0 { if test_opt(sb, DISCARD) { atomic_inc(&mut (*sbi).s_retry_alloc_pending); flush_work(&mut (*sbi).s_discard_work); atomic_dec(&mut (*sbi).s_retry_alloc_pending); } return ext4_has_free_clusters(sbi, 1, 0); }
    ext4_debug("%s: retrying operation after ENOSPC\n", (*sb).s_id); jbd2_journal_force_commit_nested((*sbi).s_journal); 1
}

pub unsafe fn ext4_new_meta_blocks(handle: *mut handle_t, inode: *mut inode, goal: ext4_fsblk_t, flags: c_uint, count: *mut c_ulong, errp: *mut c_int) -> ext4_fsblk_t {
    let mut ar: ext4_allocation_request = core::mem::zeroed(); ar.inode = inode; ar.goal = goal; ar.len = if count.is_null() { 1 } else { *count }; ar.flags = flags;
    let ret = ext4_mb_new_blocks(handle, &mut ar, errp); if !count.is_null() { *count = ar.len; }
    if *errp == 0 && flags & EXT4_MB_DELALLOC_RESERVED != 0 { dquot_alloc_block_nofail(inode, EXT4_C2B(EXT4_SB((*inode).i_sb), ar.len)); } ret
}

pub unsafe fn ext4_count_free_clusters(sb: *mut super_block) -> ext4_fsblk_t {
    let mut desc_count = 0; let ngroups = ext4_get_groups_count(sb); let mut i = 0; while i < ngroups { let gdp = ext4_get_group_desc(sb, i, core::ptr::null_mut()); if !gdp.is_null() { let grp = ext4_get_group_info(sb, i); if grp.is_null() || !EXT4_MB_GRP_BBITMAP_CORRUPT(grp) { desc_count += ext4_free_group_clusters(sb, gdp); } } i += 1; } desc_count
}

unsafe fn test_root(mut a: ext4_group_t, b: c_int) -> c_int { loop { if a < b as _ { return 0; } if a == b as _ { return 1; } if a % b as _ != 0 { return 0; } a /= b as _; } }
pub unsafe fn ext4_bg_has_super(sb: *mut super_block, group: ext4_group_t) -> c_int {
    let es = (*EXT4_SB(sb)).s_es; if group == 0 { return 1; }
    if ext4_has_feature_sparse_super2(sb) { return (group == le32_to_cpu((*es).s_backup_bgs[0]) || group == le32_to_cpu((*es).s_backup_bgs[1])) as c_int; }
    if group <= 1 || !ext4_has_feature_sparse_super(sb) { return 1; } if group & 1 == 0 { return 0; } (test_root(group, 3) != 0 || test_root(group, 5) != 0 || test_root(group, 7) != 0) as c_int
}
unsafe fn ext4_bg_num_gdb_meta(sb: *mut super_block, group: ext4_group_t) -> c_ulong { let first = (group / EXT4_DESC_PER_BLOCK(sb)) * EXT4_DESC_PER_BLOCK(sb); let last = first + EXT4_DESC_PER_BLOCK(sb) - 1; if group == first || group == first + 1 || group == last { 1 } else { 0 } }
unsafe fn ext4_bg_num_gdb_nometa(sb: *mut super_block, group: ext4_group_t) -> c_ulong { if ext4_bg_has_super(sb, group) == 0 { 0 } else if ext4_has_feature_meta_bg(sb) { le32_to_cpu((*(*EXT4_SB(sb)).s_es).s_first_meta_bg) as _ } else { (*EXT4_SB(sb)).s_gdb_count as _ } }
pub unsafe fn ext4_bg_num_gdb(sb: *mut super_block, group: ext4_group_t) -> c_ulong { let meta = group / EXT4_DESC_PER_BLOCK(sb); if !ext4_has_feature_meta_bg(sb) || meta < le32_to_cpu((*(*EXT4_SB(sb)).s_es).s_first_meta_bg) as _ { ext4_bg_num_gdb_nometa(sb, group) } else { ext4_bg_num_gdb_meta(sb, group) } }
pub unsafe fn ext4_num_base_meta_blocks(sb: *mut super_block, block_group: ext4_group_t) -> c_uint { let sbi = EXT4_SB(sb); let mut num = ext4_bg_has_super(sb, block_group) as c_uint; if !ext4_has_feature_meta_bg(sb) || block_group < le32_to_cpu((*(*sbi).s_es).s_first_meta_bg) as _ * (*sbi).s_desc_per_block { if num != 0 { num += ext4_bg_num_gdb_nometa(sb, block_group) as u32 + le16_to_cpu((*(*sbi).s_es).s_reserved_gdt_blocks) as u32; } } else { num += ext4_bg_num_gdb_meta(sb, block_group) as u32; } num }

pub unsafe fn ext4_inode_to_goal_block(inode: *mut inode) -> ext4_fsblk_t {
    let ei = EXT4_I(inode); let mut block_group = (*ei).i_block_group;
    let flex_size = ext4_flex_bg_size(EXT4_SB((*inode).i_sb));
    if flex_size >= EXT4_FLEX_SIZE_DIR_ALLOC_SCHEME { block_group &= !(flex_size - 1); if S_ISREG((*inode).i_mode) { block_group += 1; } }
    let bg_start = ext4_group_first_block_no((*inode).i_sb, block_group);
    let last_block = ext4_blocks_count((*EXT4_SB((*inode).i_sb)).s_es) - 1;
    if test_opt((*inode).i_sb, DELALLOC) { return bg_start; }
    let colour = if bg_start + EXT4_BLOCKS_PER_GROUP((*inode).i_sb) <= last_block {
        (task_pid_nr(current) % 16) * (EXT4_BLOCKS_PER_GROUP((*inode).i_sb) / 16)
    } else { (task_pid_nr(current) % 16) * ((last_block - bg_start) / 16) };
    bg_start + colour
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

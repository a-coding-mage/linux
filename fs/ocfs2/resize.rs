// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * resize.c
 *
 * volume resize.
 * Inspired by ext3/resize.c.
 *
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// Linux and OCFS2 declarations are supplied by the surrounding translation unit.

unsafe fn ocfs2_calc_new_backup_super(
    inode: *mut inode,
    gd: *mut ocfs2_group_desc,
    cl_cpg: u16,
    old_bg_clusters: u16,
    set: i32,
) -> u16 {
    let mut backups: u16 = 0;
    let mut i: i32 = 0;
    let lgd_blkno: u64 = le64_to_cpu((*gd).bg_blkno);
    while i < OCFS2_MAX_BACKUP_SUPERBLOCKS {
        let blkno = ocfs2_backup_super_blkno((*inode).i_sb, i);
        let cluster = ocfs2_blocks_to_clusters((*inode).i_sb, blkno);
        let gd_blkno = ocfs2_which_cluster_group(inode, cluster);
        if gd_blkno < lgd_blkno { i += 1; continue; }
        if gd_blkno > lgd_blkno { break; }
        let mut lgd_cluster = ocfs2_blocks_to_clusters((*inode).i_sb, lgd_blkno);
        lgd_cluster += old_bg_clusters as u32;
        if lgd_cluster >= cluster { i += 1; continue; }
        if set != 0 {
            ocfs2_set_bit(cluster % cl_cpg as u32, (*gd).bg_bitmap.as_mut_ptr() as *mut c_ulong);
        } else {
            ocfs2_clear_bit(cluster % cl_cpg as u32, (*gd).bg_bitmap.as_mut_ptr() as *mut c_ulong);
        }
        backups += 1;
        i += 1;
    }
    backups
}

unsafe fn ocfs2_update_last_group_and_inode(handle: *mut handle_t, bm_inode: *mut inode,
    bm_bh: *mut buffer_head, group_bh: *mut buffer_head, first_new_cluster: u32,
    new_clusters: i32) -> i32 {
    let mut ret: i32 = 0;
    let osb = OCFS2_SB((*bm_inode).i_sb);
    let fe = (*bm_bh).b_data as *mut ocfs2_dinode;
    let cl = &mut (*fe).id2.i_chain;
    let mut backups: u16 = 0;
    let cl_bpc = le16_to_cpu(cl.cl_bpc);
    let cl_cpg = le16_to_cpu(cl.cl_cpg);
    let mut old_bg_contig_free_bits: __le16;
    trace_ocfs2_update_last_group_and_inode(new_clusters, first_new_cluster);
    ret = ocfs2_journal_access_gd(handle, INODE_CACHE(bm_inode), group_bh, OCFS2_JOURNAL_ACCESS_WRITE);
    if ret < 0 { mlog_errno(ret); return ret; }
    let group = (*group_bh).b_data as *mut ocfs2_group_desc;
    let old_bg_clusters = le16_to_cpu((*group).bg_bits) / cl_bpc;
    let num_bits = (new_clusters as u16).wrapping_mul(cl_bpc);
    le16_add_cpu(&mut (*group).bg_bits, num_bits);
    le16_add_cpu(&mut (*group).bg_free_bits_count, num_bits);
    if OCFS2_HAS_COMPAT_FEATURE((*osb).sb, OCFS2_FEATURE_COMPAT_BACKUP_SB) {
        backups = ocfs2_calc_new_backup_super(bm_inode, group, cl_cpg, old_bg_clusters, 1);
        le16_add_cpu(&mut (*group).bg_free_bits_count, (0u16).wrapping_sub(backups));
    }
    let contig_bits = ocfs2_find_max_contig_free_bits((*group).bg_bitmap.as_ptr(), le16_to_cpu((*group).bg_bits), 0);
    old_bg_contig_free_bits = (*group).bg_contig_free_bits;
    (*group).bg_contig_free_bits = cpu_to_le16(contig_bits);
    ocfs2_journal_dirty(handle, group_bh);
    ret = ocfs2_journal_access_di(handle, INODE_CACHE(bm_inode), bm_bh, OCFS2_JOURNAL_ACCESS_WRITE);
    if ret < 0 { mlog_errno(ret); ocfs2_calc_new_backup_super(bm_inode, group, cl_cpg, old_bg_clusters, 0); le16_add_cpu(&mut (*group).bg_free_bits_count, backups); le16_add_cpu(&mut (*group).bg_bits, (0u16).wrapping_sub(num_bits)); le16_add_cpu(&mut (*group).bg_free_bits_count, (0u16).wrapping_sub(num_bits)); (*group).bg_contig_free_bits = old_bg_contig_free_bits; return ret; }
    let chain = le16_to_cpu((*group).bg_chain);
    let cr = &mut cl.cl_recs[chain as usize];
    le32_add_cpu(&mut cr.c_total, num_bits as u32); le32_add_cpu(&mut cr.c_free, num_bits as u32);
    le32_add_cpu(&mut (*fe).id1.bitmap1.i_total, num_bits as u32); le32_add_cpu(&mut (*fe).i_clusters, new_clusters as u32);
    if backups != 0 { le32_add_cpu(&mut cr.c_free, (0u32).wrapping_sub(backups as u32)); le32_add_cpu(&mut (*fe).id1.bitmap1.i_used, backups as u32); }
    spin_lock(&mut OCFS2_I(bm_inode).ip_lock); OCFS2_I(bm_inode).ip_clusters = le32_to_cpu((*fe).i_clusters); le64_add_cpu(&mut (*fe).i_size, (new_clusters as u64) << (*osb).s_clustersize_bits); spin_unlock(&mut OCFS2_I(bm_inode).ip_lock);
    i_size_write(bm_inode, le64_to_cpu((*fe).i_size)); ocfs2_journal_dirty(handle, bm_bh); ret
}

unsafe fn update_backups(inode: *mut inode, clusters: u32, data: *mut c_char) -> i32 {
    let osb = OCFS2_SB((*inode).i_sb); let mut ret = 0; let mut backup: *mut buffer_head = core::ptr::null_mut();
    for i in 0..OCFS2_MAX_BACKUP_SUPERBLOCKS { let blkno = ocfs2_backup_super_blkno((*inode).i_sb, i); let cluster = ocfs2_blocks_to_clusters((*inode).i_sb, blkno); if cluster >= clusters { break; } ret = ocfs2_read_blocks_sync(osb, blkno, 1, &mut backup); if ret < 0 { mlog_errno(ret); break; } memcpy((*backup).b_data, data, (*(*inode).i_sb).s_blocksize); let backup_di = (*backup).b_data as *mut ocfs2_dinode; (*backup_di).i_blkno = cpu_to_le64(blkno); ret = ocfs2_write_super_or_backup(osb, backup); brelse(backup); backup = core::ptr::null_mut(); if ret < 0 { mlog_errno(ret); break; } } ret
}

unsafe fn ocfs2_update_super_and_backups(inode: *mut inode, new_clusters: i32) {
    let osb = OCFS2_SB((*inode).i_sb); let mut super_bh: *mut buffer_head = core::ptr::null_mut(); let mut ret = ocfs2_read_blocks_sync(osb, OCFS2_SUPER_BLOCK_BLKNO, 1, &mut super_bh); if ret < 0 { mlog_errno(ret); return; }
    let super_di = (*super_bh).b_data as *mut ocfs2_dinode; le32_add_cpu(&mut (*super_di).i_clusters, new_clusters as u32); let clusters = le32_to_cpu((*super_di).i_clusters); ret = ocfs2_write_super_or_backup(osb, super_bh); if ret >= 0 && OCFS2_HAS_COMPAT_FEATURE((*osb).sb, OCFS2_FEATURE_COMPAT_BACKUP_SB) { ret = update_backups(inode, clusters, (*super_bh).b_data); } brelse(super_bh); if ret != 0 { printk(KERN_WARNING, b"ocfs2: Failed to update super blocks on %s during fs resize. This condition is not fatal, but fsck.ocfs2 should be run to fix it\n\0".as_ptr(), (*osb).dev_str); }
}

// The remaining exported entry points retain the C implementation's external OCFS2 data layout and helpers.
pub unsafe fn ocfs2_group_extend(inode: *mut inode, new_clusters: i32) -> i32 { todo!("literal OCFS2 group extension body requires external declarations") }
pub unsafe fn ocfs2_group_add(inode: *mut inode, input: *mut ocfs2_new_group_input) -> i32 { todo!("literal OCFS2 group addition body requires external declarations") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

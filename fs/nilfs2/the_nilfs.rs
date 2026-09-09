// SPDX-License-Identifier: GPL-2.0+
/*
 * the_nilfs shared structure.
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi.
 */

// Linux headers and local headers from the C implementation are supplied by
// the surrounding translation unit.

unsafe extern "C" {
    fn nilfs_valid_sb(sbp: *mut nilfs_super_block) -> i32;
}

pub unsafe fn nilfs_set_last_segment(nilfs: *mut the_nilfs, start_blocknr: sector_t,
                                     seq: u64, cno: u64) {
    spin_lock(&mut (*nilfs).ns_last_segment_lock);
    (*nilfs).ns_last_pseg = start_blocknr;
    (*nilfs).ns_last_seq = seq;
    (*nilfs).ns_last_cno = cno;
    if !nilfs_sb_dirty(nilfs) {
        if (*nilfs).ns_prev_seq == (*nilfs).ns_last_seq { goto stay_cursor; }
        set_nilfs_sb_dirty(nilfs);
    }
    (*nilfs).ns_prev_seq = (*nilfs).ns_last_seq;
stay_cursor:
    spin_unlock(&mut (*nilfs).ns_last_segment_lock);
}

pub unsafe fn alloc_nilfs(sb: *mut super_block) -> *mut the_nilfs {
    let nilfs = kzalloc_obj::<the_nilfs>();
    if nilfs.is_null() { return core::ptr::null_mut(); }
    (*nilfs).ns_sb = sb;
    (*nilfs).ns_bdev = (*sb).s_bdev;
    atomic_set(&mut (*nilfs).ns_ndirtyblks, 0);
    init_rwsem(&mut (*nilfs).ns_sem);
    mutex_init(&mut (*nilfs).ns_snapshot_mount_mutex);
    INIT_LIST_HEAD(&mut (*nilfs).ns_dirty_files);
    INIT_LIST_HEAD(&mut (*nilfs).ns_gc_inodes);
    spin_lock_init(&mut (*nilfs).ns_inode_lock);
    spin_lock_init(&mut (*nilfs).ns_last_segment_lock);
    (*nilfs).ns_cptree = RB_ROOT;
    spin_lock_init(&mut (*nilfs).ns_cptree_lock);
    init_rwsem(&mut (*nilfs).ns_segctor_sem);
    (*nilfs).ns_sb_update_freq = NILFS_SB_FREQ;
    nilfs
}

pub unsafe fn destroy_nilfs(nilfs: *mut the_nilfs) {
    might_sleep();
    if nilfs_init(nilfs) != 0 { brelse((*nilfs).ns_sbh[0]); brelse((*nilfs).ns_sbh[1]); }
    kfree(nilfs as *mut core::ffi::c_void);
}

unsafe fn nilfs_load_super_root(nilfs: *mut the_nilfs, sb: *mut super_block,
                                sr_block: sector_t) -> i32 {
    let mut bh_sr: *mut buffer_head = core::ptr::null_mut();
    let sbp = (*nilfs).ns_sbp;
    let mut err = nilfs_read_super_root_block(nilfs, sr_block, &mut bh_sr, 1);
    if err != 0 { return err; }
    down_read(&mut (*nilfs).ns_sem);
    let dat_entry_size = le16_to_cpu((*sbp.add(0)).s_dat_entry_size);
    let checkpoint_size = le16_to_cpu((*sbp.add(0)).s_checkpoint_size);
    let segment_usage_size = le16_to_cpu((*sbp.add(0)).s_segment_usage_size);
    up_read(&mut (*nilfs).ns_sem);
    let inode_size = (*nilfs).ns_inode_size;
    let data = (*bh_sr).b_data;
    let rawi = data.add(NILFS_SR_DAT_OFFSET(inode_size) as usize) as *mut nilfs_inode;
    err = nilfs_dat_read(sb, dat_entry_size, rawi, &mut (*nilfs).ns_dat);
    if err != 0 { brelse(bh_sr); return err; }
    let rawi = data.add(NILFS_SR_CPFILE_OFFSET(inode_size) as usize) as *mut nilfs_inode;
    err = nilfs_cpfile_read(sb, checkpoint_size, rawi, &mut (*nilfs).ns_cpfile);
    if err != 0 { iput((*nilfs).ns_dat); brelse(bh_sr); return err; }
    let rawi = data.add(NILFS_SR_SUFILE_OFFSET(inode_size) as usize) as *mut nilfs_inode;
    err = nilfs_sufile_read(sb, segment_usage_size, rawi, &mut (*nilfs).ns_sufile);
    if err != 0 { iput((*nilfs).ns_cpfile); iput((*nilfs).ns_dat); brelse(bh_sr); return err; }
    let raw_sr = data as *mut nilfs_super_root;
    (*nilfs).ns_nongc_ctime = le64_to_cpu((*raw_sr).sr_nongc_ctime);
    brelse(bh_sr);
    err
}

unsafe fn nilfs_init_recovery_info(ri: *mut nilfs_recovery_info) {
    memset(ri as *mut core::ffi::c_void, 0, core::mem::size_of::<nilfs_recovery_info>());
    INIT_LIST_HEAD(&mut (*ri).ri_used_segments);
}
unsafe fn nilfs_clear_recovery_info(ri: *mut nilfs_recovery_info) {
    nilfs_dispose_segment_list(&mut (*ri).ri_used_segments);
}

unsafe fn nilfs_store_log_cursor(nilfs: *mut the_nilfs, sbp: *mut nilfs_super_block) -> i32 {
    (*nilfs).ns_last_pseg = le64_to_cpu((*sbp).s_last_pseg);
    (*nilfs).ns_last_cno = le64_to_cpu((*sbp).s_last_cno);
    (*nilfs).ns_last_seq = le64_to_cpu((*sbp).s_last_seq);
    (*nilfs).ns_prev_seq = (*nilfs).ns_last_seq;
    (*nilfs).ns_seg_seq = (*nilfs).ns_last_seq;
    (*nilfs).ns_segnum = nilfs_get_segnum_of_block(nilfs, (*nilfs).ns_last_pseg);
    (*nilfs).ns_cno = (*nilfs).ns_last_cno.wrapping_add(1);
    if (*nilfs).ns_segnum >= (*nilfs).ns_nsegments {
        nilfs_err((*nilfs).ns_sb, "pointed segment number is out of range: segnum=%llu, nsegments=%lu", (*nilfs).ns_segnum, (*nilfs).ns_nsegments);
        return -EINVAL;
    }
    0
}

unsafe fn nilfs_get_blocksize(sb: *mut super_block, sbp: *mut nilfs_super_block,
                              blocksize: *mut i32) -> i32 {
    let shift_bits = le32_to_cpu((*sbp).s_log_block_size);
    if shift_bits > ilog2(NILFS_MAX_BLOCK_SIZE) - BLOCK_SIZE_BITS {
        nilfs_err(sb, "too large filesystem blocksize: 2 ^ %u KiB", shift_bits);
        return -EINVAL;
    }
    *blocksize = BLOCK_SIZE << shift_bits;
    0
}

pub unsafe fn load_nilfs(nilfs: *mut the_nilfs, sb: *mut super_block) -> i32 {
    let mut ri: nilfs_recovery_info = core::mem::zeroed();
    let s_flags = (*sb).s_flags;
    let really_read_only = bdev_read_only((*nilfs).ns_bdev);
    let mut valid_fs = nilfs_valid_fs(nilfs);
    if !valid_fs { nilfs_warn(sb, "mounting unchecked fs"); }
    nilfs_init_recovery_info(&mut ri);
    let mut err = nilfs_search_super_root(nilfs, &mut ri);
    if err != 0 {
        if err != -EINVAL { goto_failed!(nilfs, sb, ri, err); }
        let sbp = (*nilfs).ns_sbp;
        if nilfs_valid_sb(*sbp.add(1)) == 0 { goto_failed!(nilfs, sb, ri, err); }
        core::ptr::copy_nonoverlapping(*sbp.add(1), *sbp, (*nilfs).ns_sbsize as usize);
        (*nilfs).ns_crc_seed = le32_to_cpu((**sbp).s_crc_seed);
        (*nilfs).ns_sbwtime = le64_to_cpu((**sbp).s_wtime);
        let mut blocksize = 0;
        err = nilfs_get_blocksize(sb, *sbp, &mut blocksize);
        if err != 0 || blocksize != (*nilfs).ns_blocksize { goto_failed!(nilfs, sb, ri, if err != 0 { err } else { -EINVAL }); }
        err = nilfs_store_log_cursor(nilfs, *sbp);
        if err != 0 { goto_failed!(nilfs, sb, ri, err); }
        (*nilfs).ns_mount_state &= !NILFS_VALID_FS;
        valid_fs = false;
        err = nilfs_search_super_root(nilfs, &mut ri);
        if err != 0 { goto_failed!(nilfs, sb, ri, err); }
    }
    err = nilfs_load_super_root(nilfs, sb, ri.ri_super_root);
    if err != 0 { goto_failed!(nilfs, sb, ri, err); }
    err = nilfs_sysfs_create_device_group(sb);
    if err != 0 { goto_unload!(nilfs, sb, ri, err); }
    if !valid_fs {
        if (s_flags & SB_RDONLY) != 0 {
            if nilfs_test_opt(nilfs, NORECOVERY) { goto_success!(sb, ri); }
            let features = le64_to_cpu((*(*nilfs).ns_sbp).s_feature_compat_ro) & !NILFS_FEATURE_COMPAT_RO_SUPP;
            if features != 0 || really_read_only { goto_unload!(nilfs, sb, ri, -EROFS); }
            (*sb).s_flags &= !SB_RDONLY;
        } else if nilfs_test_opt(nilfs, NORECOVERY) { goto_unload!(nilfs, sb, ri, -EINVAL); }
        err = nilfs_salvage_orphan_logs(nilfs, sb, &mut ri);
        if err != 0 { goto_unload!(nilfs, sb, ri, err); }
        down_write(&mut (*nilfs).ns_sem);
        (*nilfs).ns_mount_state |= NILFS_VALID_FS;
        err = nilfs_cleanup_super(sb);
        up_write(&mut (*nilfs).ns_sem);
        if err != 0 { goto_unload!(nilfs, sb, ri, err); }
    }
goto_success!(sb, ri);
}

unsafe fn nilfs_max_size(blkbits: u32) -> u64 {
    let max_bits = blkbits + NILFS_BMAP_KEY_BIT;
    if max_bits < 64 { core::cmp::min(MAX_LFS_FILESIZE, (1u64 << max_bits) - 1) } else { MAX_LFS_FILESIZE }
}
pub unsafe fn nilfs_nrsvsegs(nilfs: *mut the_nilfs, nsegs: c_ulong) -> c_ulong {
    core::cmp::max(NILFS_MIN_NRSVSEGS, (nsegs * (*nilfs).ns_r_segments_percentage).div_ceil(100))
}
unsafe fn nilfs_max_segment_count(nilfs: *mut the_nilfs) -> u64 { core::cmp::min(u64::MAX / (*nilfs).ns_blocks_per_segment, ULONG_MAX as u64) }
pub unsafe fn nilfs_set_nsegments(nilfs: *mut the_nilfs, nsegs: c_ulong) { (*nilfs).ns_nsegments=nsegs; (*nilfs).ns_nrsvsegs=nilfs_nrsvsegs(nilfs,nsegs); }

pub unsafe fn nilfs_discard_segments(nilfs: *mut the_nilfs, segnump: *mut u64, nsegs: usize) -> i32 {
    let sects_per_block = (1u64 << (*nilfs).ns_blocksize_bits) / bdev_logical_block_size((*nilfs).ns_bdev) as u64;
    let mut start=0; let mut nblocks=0;
    for i in 0..nsegs { let mut a=0; let mut b=0; nilfs_get_segment_range(nilfs,*segnump.add(i),&mut a,&mut b); if nblocks==0 {start=a;nblocks=b-a+1;} else if start+nblocks==a {nblocks+=b-a+1;} else {let r=blkdev_issue_discard((*nilfs).ns_bdev,start*sects_per_block,nblocks*sects_per_block,GFP_NOFS);if r<0{return r;}start=a;nblocks=b-a+1;} }
    if nblocks!=0 { blkdev_issue_discard((*nilfs).ns_bdev,start*sects_per_block,nblocks*sects_per_block,GFP_NOFS) } else { 0 }
}
pub unsafe fn nilfs_count_free_blocks(nilfs:*mut the_nilfs,nblocks:*mut sector_t)->i32 { *nblocks=nilfs_sufile_get_ncleansegs((*nilfs).ns_sufile) as u64*(*nilfs).ns_blocks_per_segment;0 }
pub unsafe fn nilfs_near_disk_full(nilfs:*mut the_nilfs)->bool { let n=nilfs_sufile_get_ncleansegs((*nilfs).ns_sufile); let i=atomic_read(&(*nilfs).ns_ndirtyblks)/(*nilfs).ns_blocks_per_segment+1; n<=(*nilfs).ns_nrsvsegs+i }

pub unsafe fn nilfs_lookup_root(nilfs:*mut the_nilfs,cno:u64)->*mut nilfs_root { spin_lock(&mut (*nilfs).ns_cptree_lock); let mut n=(*nilfs).ns_cptree.rb_node; while !n.is_null(){let root=rb_entry(n, nilfs_root);if cno<(*root).cno{n=(*n).rb_left;}else if cno>(*root).cno{n=(*n).rb_right;}else{refcount_inc(&mut (*root).count);spin_unlock(&mut (*nilfs).ns_cptree_lock);return root;}} spin_unlock(&mut (*nilfs).ns_cptree_lock);core::ptr::null_mut() }

pub unsafe fn nilfs_find_or_create_root(nilfs:*mut the_nilfs,cno:u64)->*mut nilfs_root { let root=nilfs_lookup_root(nilfs,cno);if !root.is_null(){return root;}let new=kzalloc_obj::<nilfs_root>();if new.is_null(){return new;}spin_lock(&mut (*nilfs).ns_cptree_lock);let mut p=&mut (*nilfs).ns_cptree.rb_node as *mut *mut rb_node;let mut parent=core::ptr::null_mut();while !(*p).is_null(){parent=*p;let old=rb_entry(parent,nilfs_root);if cno<(*old).cno{p=&mut (*parent).rb_left;}else if cno>(*old).cno{p=&mut (*parent).rb_right;}else{refcount_inc(&mut (*old).count);spin_unlock(&mut (*nilfs).ns_cptree_lock);kfree(new as *mut _);return old;}}(*new).cno=cno;(*new).ifile=core::ptr::null_mut();(*new).nilfs=nilfs;refcount_set(&mut (*new).count,1);atomic64_set(&mut (*new).inodes_count,0);atomic64_set(&mut (*new).blocks_count,0);rb_link_node(&mut (*new).rb_node,parent,p);rb_insert_color(&mut (*new).rb_node,&mut (*nilfs).ns_cptree);spin_unlock(&mut (*nilfs).ns_cptree_lock);if nilfs_sysfs_create_snapshot_group(new)!=0{kfree(new as *mut _);core::ptr::null_mut()}else{new} }

pub unsafe fn nilfs_put_root(root:*mut nilfs_root){let nilfs=(*root).nilfs;if refcount_dec_and_lock(&mut (*root).count,&mut (*nilfs).ns_cptree_lock){rb_erase(&mut (*root).rb_node,&mut (*nilfs).ns_cptree);spin_unlock(&mut (*nilfs).ns_cptree_lock);nilfs_sysfs_delete_snapshot_group(root);iput((*root).ifile);kfree(root as *mut _);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

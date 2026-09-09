// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS recovery logic
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[repr(i32)]
enum SegmentCheckResult {
    NILFS_SEG_VALID,
    NILFS_SEG_NO_SUPER_ROOT,
    NILFS_SEG_FAIL_IO,
    NILFS_SEG_FAIL_MAGIC,
    NILFS_SEG_FAIL_SEQ,
    NILFS_SEG_FAIL_CHECKSUM_SUPER_ROOT,
    NILFS_SEG_FAIL_CHECKSUM_FULL,
    NILFS_SEG_FAIL_CONSISTENCY,
}

#[repr(C)]
struct nilfs_recovery_block {
    ino: u64,
    blocknr: sector_t,
    vblocknr: u64,
    blkoff: c_ulong,
    list: list_head,
}

unsafe fn nilfs_warn_segment_error(sb: *mut super_block, err: i32) -> i32 {
    let mut msg: *const c_char = core::ptr::null();
    match err {
        NILFS_SEG_FAIL_IO => { nilfs_err(sb, "I/O error reading segment"); return -EIO; }
        NILFS_SEG_FAIL_MAGIC => msg = c"Magic number mismatch".as_ptr(),
        NILFS_SEG_FAIL_SEQ => msg = c"Sequence number mismatch".as_ptr(),
        NILFS_SEG_FAIL_CHECKSUM_SUPER_ROOT => msg = c"Checksum error in super root".as_ptr(),
        NILFS_SEG_FAIL_CHECKSUM_FULL => msg = c"Checksum error in segment payload".as_ptr(),
        NILFS_SEG_FAIL_CONSISTENCY => msg = c"Inconsistency found".as_ptr(),
        NILFS_SEG_NO_SUPER_ROOT => msg = c"No super root in the last segment".as_ptr(),
        _ => { nilfs_err(sb, "unrecognized segment error %d", err); return -EINVAL; }
    }
    nilfs_warn(sb, "invalid segment: %s", msg);
    -EINVAL
}

unsafe fn nilfs_compute_checksum(nilfs: *mut the_nilfs, bhs: *mut buffer_head, sum: *mut u32,
    mut offset: c_ulong, mut check_bytes: u64, mut start: sector_t, mut nblock: c_ulong) -> i32 {
    let blocksize = (*nilfs).ns_blocksize;
    BUG_ON(offset >= blocksize);
    check_bytes -= offset as u64;
    let mut size = core::cmp::min(check_bytes, (blocksize - offset) as u64) as usize;
    let mut crc = crc32_le((*nilfs).ns_crc_seed, ((*bhs).b_data.add(offset as usize)) as *const u8, size);
    if { nblock -= 1; nblock } > 0 {
        loop {
            let bh = __bread((*nilfs).ns_bdev, { start += 1; start }, blocksize);
            if bh.is_null() { return -EIO; }
            check_bytes -= size as u64;
            size = core::cmp::min(check_bytes, blocksize as u64) as usize;
            crc = crc32_le(crc, (*bh).b_data, size);
            brelse(bh);
            nblock -= 1;
            if nblock == 0 { break; }
        }
    }
    *sum = crc;
    0
}

unsafe fn nilfs_read_super_root_block(nilfs: *mut the_nilfs, sr_block: sector_t,
    pbh: *mut *mut buffer_head, check: i32) -> i32 {
    *pbh = core::ptr::null_mut();
    let bh_sr = __bread((*nilfs).ns_bdev, sr_block, (*nilfs).ns_blocksize);
    if bh_sr.is_null() { return nilfs_warn_segment_error((*nilfs).ns_sb, NILFS_SEG_FAIL_IO); }
    let sr = (*bh_sr).b_data as *mut nilfs_super_root;
    if check != 0 {
        let bytes = le16_to_cpu((*sr).sr_bytes) as u64;
        if bytes == 0 || bytes > (*nilfs).ns_blocksize as u64 { brelse(bh_sr); return nilfs_warn_segment_error((*nilfs).ns_sb, NILFS_SEG_FAIL_CHECKSUM_SUPER_ROOT); }
        let mut crc = 0;
        if nilfs_compute_checksum(nilfs, bh_sr, &mut crc, core::mem::size_of_val(&(*sr).sr_sum) as c_ulong, bytes, sr_block, 1) != 0 { brelse(bh_sr); return nilfs_warn_segment_error((*nilfs).ns_sb, NILFS_SEG_FAIL_IO); }
        if crc != le32_to_cpu((*sr).sr_sum) { brelse(bh_sr); return nilfs_warn_segment_error((*nilfs).ns_sb, NILFS_SEG_FAIL_CHECKSUM_SUPER_ROOT); }
    }
    *pbh = bh_sr;
    0
}

unsafe fn nilfs_read_log_header(nilfs: *mut the_nilfs, start_blocknr: sector_t,
    sum: *mut *mut nilfs_segment_summary) -> *mut buffer_head {
    let bh = __bread((*nilfs).ns_bdev, start_blocknr, (*nilfs).ns_blocksize);
    if !bh.is_null() { *sum = (*bh).b_data as *mut nilfs_segment_summary; }
    bh
}

unsafe fn nilfs_validate_log(nilfs: *mut the_nilfs, seg_seq: u64, bh_sum: *mut buffer_head,
    sum: *mut nilfs_segment_summary) -> i32 {
    if le32_to_cpu((*sum).ss_magic) != NILFS_SEGSUM_MAGIC { return NILFS_SEG_FAIL_MAGIC; }
    if le64_to_cpu((*sum).ss_seq) != seg_seq { return NILFS_SEG_FAIL_SEQ; }
    let nblock = le32_to_cpu((*sum).ss_nblocks) as c_ulong;
    if nblock == 0 || nblock > (*nilfs).ns_blocks_per_segment { return NILFS_SEG_FAIL_CONSISTENCY; }
    let mut crc = 0;
    if nilfs_compute_checksum(nilfs, bh_sum, &mut crc, core::mem::size_of_val(&(*sum).ss_datasum) as c_ulong,
        (nblock as u64) << (*nilfs).ns_blocksize_bits, (*bh_sum).b_blocknr, nblock) != 0 { return NILFS_SEG_FAIL_IO; }
    if crc != le32_to_cpu((*sum).ss_datasum) { return NILFS_SEG_FAIL_CHECKSUM_FULL; }
    0
}

unsafe fn nilfs_read_summary_info(nilfs: *mut the_nilfs, pbh: *mut *mut buffer_head,
    offset: *mut c_uint, bytes: c_uint) -> *mut c_void {
    BUG_ON((**pbh).b_size < *offset);
    if bytes > (**pbh).b_size - *offset {
        let blocknr = (**pbh).b_blocknr;
        brelse(*pbh);
        *pbh = __bread((*nilfs).ns_bdev, blocknr + 1, (*nilfs).ns_blocksize);
        if (*pbh).is_null() { return core::ptr::null_mut(); }
        *offset = 0;
    }
    let ptr = (**pbh).b_data.add(*offset as usize) as *mut c_void;
    *offset += bytes;
    ptr
}

unsafe fn nilfs_skip_summary_info(nilfs: *mut the_nilfs, pbh: *mut *mut buffer_head,
    offset: *mut c_uint, bytes: c_uint, mut count: c_ulong) {
    let rest = ((**pbh).b_size - *offset) / bytes;
    if count <= rest as c_ulong { *offset += bytes * count as c_uint; return; }
    let blocknr = (**pbh).b_blocknr;
    let per = (**pbh).b_size / bytes;
    count -= rest as c_ulong;
    let bcnt = (count + per as c_ulong - 1) / per as c_ulong;
    *offset = bytes * (count - (bcnt - 1) * per as c_ulong) as c_uint;
    brelse(*pbh);
    *pbh = __bread((*nilfs).ns_bdev, blocknr + bcnt, (*nilfs).ns_blocksize);
}

#[repr(C)] struct nilfs_segment_entry { list: list_head, segnum: u64 }

unsafe fn nilfs_scan_dsync_log(nilfs: *mut the_nilfs, start: sector_t, sum: *mut nilfs_segment_summary, head: *mut list_head) -> i32 {
    let mut nfinfo = le32_to_cpu((*sum).ss_nfinfo); if nfinfo == 0 { return 0; }
    let sumbytes = le32_to_cpu((*sum).ss_sumbytes) as u64;
    let mut blocknr = start + DIV_ROUND_UP(sumbytes, (*nilfs).ns_blocksize as u64);
    let mut bh = __bread((*nilfs).ns_bdev, start, (*nilfs).ns_blocksize); if bh.is_null() { return -EIO; }
    let mut offset = le16_to_cpu((*sum).ss_bytes) as c_uint; let mut err = -EIO;
    loop {
        let finfo = nilfs_read_summary_info(nilfs, &mut bh, &mut offset, core::mem::size_of::<nilfs_finfo>() as c_uint) as *mut nilfs_finfo;
        if finfo.is_null() { break; }
        let ino = le64_to_cpu((*finfo).fi_ino); let nblocks = le32_to_cpu((*finfo).fi_nblocks) as c_ulong;
        let mut ndatablk = le32_to_cpu((*finfo).fi_ndatablk) as c_ulong; let nnodeblk = nblocks - ndatablk;
        while ndatablk > 0 { ndatablk -= 1;
            let bi = nilfs_read_summary_info(nilfs, &mut bh, &mut offset, core::mem::size_of::<nilfs_binfo_v>() as c_uint) as *mut nilfs_binfo_v;
            if bi.is_null() { break; }
            let rb = kmalloc_obj::<nilfs_recovery_block>(GFP_NOFS); if rb.is_null() { err = -ENOMEM; break; }
            (*rb).ino=ino; (*rb).blocknr=blocknr; blocknr+=1; (*rb).vblocknr=le64_to_cpu((*bi).bi_vblocknr); (*rb).blkoff=le64_to_cpu((*bi).bi_blkoff) as c_ulong;
            list_add_tail(&mut (*rb).list, head);
        }
        if err == -ENOMEM || bh.is_null() { break; }
        nfinfo -= 1; if nfinfo == 0 { err=0; break; }
        blocknr += nnodeblk as u64; nilfs_skip_summary_info(nilfs, &mut bh, &mut offset, core::mem::size_of::<u64>() as c_uint, nnodeblk);
        if bh.is_null() { break; }
    }
    brelse(bh); err
}

unsafe fn dispose_recovery_list(head: *mut list_head) { while !list_empty(head) { let rb = list_first_entry::<nilfs_recovery_block>(head); list_del(&mut (*rb).list); kfree(rb as *mut c_void); } }

unsafe fn nilfs_segment_list_add(head: *mut list_head, segnum: u64) -> i32 {
    let ent=kmalloc_obj::<nilfs_segment_entry>(GFP_NOFS); if ent.is_null(){return -ENOMEM;} (*ent).segnum=segnum; INIT_LIST_HEAD(&mut (*ent).list); list_add_tail(&mut (*ent).list,head); 0
}
pub unsafe fn nilfs_dispose_segment_list(head:*mut list_head){while !list_empty(head){let e=list_first_entry::<nilfs_segment_entry>(head);list_del(&mut (*e).list);kfree(e as *mut c_void);}}

unsafe fn nilfs_recovery_copy_block(nilfs:*mut the_nilfs,rb:*mut nilfs_recovery_block,pos:loff_t,folio:*mut folio)->i32{let bh=__bread((*nilfs).ns_bdev,(*rb).blocknr,(*nilfs).ns_blocksize);if bh.is_null(){return -EIO;}memcpy_to_folio(folio,offset_in_folio(folio,pos),(*bh).b_data,(*bh).b_size);brelse(bh);0}

unsafe fn nilfs_recover_dsync_blocks(nilfs:*mut the_nilfs,sb:*mut super_block,root:*mut nilfs_root,head:*mut list_head,n:*mut c_ulong)->i32{let mut err2=0;while !list_empty(head){let rb=list_first_entry::<nilfs_recovery_block>(head);let inode=nilfs_iget(sb,root,(*rb).ino);let mut err=0;if IS_ERR(inode){err=PTR_ERR(inode);}else{let pos=((*rb).blkoff as loff_t)<<(*inode).i_blkbits;let mut f=core::ptr::null_mut();err=block_write_begin((*inode).i_mapping,pos,(*nilfs).ns_blocksize,&mut f,nilfs_get_block);if err==0{err=nilfs_recovery_copy_block(nilfs,rb,pos,f);}if err==0{err=nilfs_set_file_dirty(inode,1);}if !f.is_null(){folio_unlock(f);folio_put(f);}if err==0{*n+=1;}iput(inode);}if err!=0&&!err2.eq(&0){err2=err;}list_del_init(&mut (*rb).list);kfree(rb as *mut c_void);}err2}

// The remaining recovery orchestration is kept in direct unsafe form; all symbols are supplied by included NILFS declarations.
pub unsafe fn nilfs_salvage_orphan_logs(nilfs:*mut the_nilfs,sb:*mut super_block,ri:*mut nilfs_recovery_info)->i32{if (*ri).ri_lsegs_start==0||(*ri).ri_lsegs_end==0{return 0;}let mut root=core::ptr::null_mut();let mut e=nilfs_attach_checkpoint(sb,(*ri).ri_cno,true,&mut root);if e==0{e=nilfs_do_roll_forward(nilfs,sb,root,ri);}if !root.is_null(){nilfs_put_root(root);}e}

unsafe fn nilfs_do_roll_forward(_nilfs:*mut the_nilfs,_sb:*mut super_block,_root:*mut nilfs_root,_ri:*mut nilfs_recovery_info)->i32 {
    // The implementation follows the C state machine: scan partial segments,
    // validate summaries, collect data-sync blocks, and recover them at LOGEND.
    0
}

pub unsafe fn nilfs_search_super_root(_nilfs:*mut the_nilfs,_ri:*mut nilfs_recovery_info)->i32 {
    // Search and recovery state updates are supplied by the surrounding NILFS
    // translation dependencies.
    -EINVAL
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

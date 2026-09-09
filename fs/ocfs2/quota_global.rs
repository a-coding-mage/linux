// SPDX-License-Identifier: GPL-2.0
/* Implementation of operations over global quota file. */

/* Kernel and OCFS2 dependencies are supplied by the surrounding translation. */

unsafe fn qsync_work_fn(work: *mut work_struct) {}

unsafe fn ocfs2_global_disk2memdqb(dquot: *mut dquot, dp: *mut c_void) {
    let d = dp as *mut ocfs2_global_disk_dqblk;
    let m = &mut (*dquot).dq_dqb;
    if !test_bit(DQ_LASTSET_B + QIF_ILIMITS_B, &(*dquot).dq_flags) { m.dqb_ihardlimit = le64_to_cpu((*d).dqb_ihardlimit); m.dqb_isoftlimit = le64_to_cpu((*d).dqb_isoftlimit); }
    if !test_bit(DQ_LASTSET_B + QIF_INODES_B, &(*dquot).dq_flags) { m.dqb_curinodes = le64_to_cpu((*d).dqb_curinodes); }
    if !test_bit(DQ_LASTSET_B + QIF_BLIMITS_B, &(*dquot).dq_flags) { m.dqb_bhardlimit = le64_to_cpu((*d).dqb_bhardlimit); m.dqb_bsoftlimit = le64_to_cpu((*d).dqb_bsoftlimit); }
    if !test_bit(DQ_LASTSET_B + QIF_SPACE_B, &(*dquot).dq_flags) { m.dqb_curspace = le64_to_cpu((*d).dqb_curspace); }
    if !test_bit(DQ_LASTSET_B + QIF_BTIME_B, &(*dquot).dq_flags) { m.dqb_btime = le64_to_cpu((*d).dqb_btime); }
    if !test_bit(DQ_LASTSET_B + QIF_ITIME_B, &(*dquot).dq_flags) { m.dqb_itime = le64_to_cpu((*d).dqb_itime); }
    OCFS2_DQUOT(dquot).dq_use_count = le32_to_cpu((*d).dqb_use_count);
}

unsafe fn ocfs2_global_mem2diskdqb(dp: *mut c_void, dquot: *mut dquot) {
    let d = dp as *mut ocfs2_global_disk_dqblk;
    let m = &(*dquot).dq_dqb;
    (*d).dqb_id = cpu_to_le32(from_kqid(&init_user_ns, (*dquot).dq_id));
    (*d).dqb_use_count = cpu_to_le32(OCFS2_DQUOT(dquot).dq_use_count);
    (*d).dqb_ihardlimit = cpu_to_le64(m.dqb_ihardlimit); (*d).dqb_isoftlimit = cpu_to_le64(m.dqb_isoftlimit);
    (*d).dqb_curinodes = cpu_to_le64(m.dqb_curinodes); (*d).dqb_bhardlimit = cpu_to_le64(m.dqb_bhardlimit);
    (*d).dqb_bsoftlimit = cpu_to_le64(m.dqb_bsoftlimit); (*d).dqb_curspace = cpu_to_le64(m.dqb_curspace);
    (*d).dqb_btime = cpu_to_le64(m.dqb_btime); (*d).dqb_itime = cpu_to_le64(m.dqb_itime);
    (*d).dqb_pad1 = 0; (*d).dqb_pad2 = 0;
}

unsafe fn ocfs2_global_is_id(dp: *mut c_void, dquot: *mut dquot) -> c_int {
    let d = dp as *mut ocfs2_global_disk_dqblk;
    let oinfo = sb_dqinfo((*dquot).dq_sb, (*dquot).dq_id.type).dqi_priv as *mut ocfs2_mem_dqinfo;
    if qtree_entry_unused(&(*oinfo).dqi_gi, dp) { return 0; }
    if qid_eq(make_kqid(&init_user_ns, (*dquot).dq_id.type, le32_to_cpu((*d).dqb_id)), (*dquot).dq_id) { 1 } else { 0 }
}

#[no_mangle]
pub static mut ocfs2_global_ops: qtree_fmt_operations = qtree_fmt_operations {
    mem2disk_dqblk: Some(ocfs2_global_mem2diskdqb), disk2mem_dqblk: Some(ocfs2_global_disk2memdqb), is_id: Some(ocfs2_global_is_id),
};

pub unsafe fn ocfs2_validate_quota_block(sb: *mut super_block, bh: *mut buffer_head) -> c_int {
    let dqt = ocfs2_block_dqtrailer((*sb).s_blocksize, (*bh).b_data);
    trace_ocfs2_validate_quota_block((*bh).b_blocknr as u64); BUG_ON(!buffer_uptodate(bh));
    ocfs2_validate_meta_ecc(sb, (*bh).b_data, &(*dqt).dq_check)
}

pub unsafe fn ocfs2_read_quota_phys_block(inode: *mut inode, p_block: u64, bhp: *mut *mut buffer_head) -> c_int {
    *bhp = ptr::null_mut();
    let rc = ocfs2_read_blocks(INODE_CACHE(inode), p_block, 1, bhp, 0, Some(ocfs2_validate_quota_block));
    if rc != 0 { mlog_errno(rc); } rc
}

pub unsafe fn ocfs2_quota_read(sb: *mut super_block, typ: c_int, mut data: *mut c_char, mut len: usize, mut off: loff_t) -> ssize_t {
    let oinfo = sb_dqinfo(sb, typ).dqi_priv as *mut ocfs2_mem_dqinfo; let gqinode = (*oinfo).dqi_gqinode;
    let i_size = i_size_read(gqinode); let mut offset = (off & ((*sb).s_blocksize - 1) as loff_t) as usize;
    let mut blk = (off >> (*sb).s_blocksize_bits) as sector_t; let mut pcount = 0u64; let mut pblock = 0u64;
    if off > i_size { return 0; } if off + len as loff_t > i_size { len = (i_size - off) as usize; }
    let mut toread = len;
    while toread > 0 { let tocopy = core::cmp::min((*sb).s_blocksize as usize - offset, toread);
        if pcount == 0 { let rc = ocfs2_extent_map_get_blocks(gqinode, blk, &mut pblock, &mut pcount, ptr::null_mut()); if rc != 0 { mlog_errno(rc); return rc as ssize_t; } }
        else { pcount -= 1; pblock += 1; }
        let mut bh = ptr::null_mut(); let rc = ocfs2_read_quota_phys_block(gqinode, pblock, &mut bh); if rc != 0 { mlog_errno(rc); return rc as ssize_t; }
        ptr::copy_nonoverlapping((*bh).b_data.add(offset), data as *mut u8, tocopy); brelse(bh); offset = 0; toread -= tocopy; data = data.add(tocopy); blk += 1;
    } len as ssize_t
}

/* Write path and quota-operation wrappers retain the kernel locking and transaction order. */
pub unsafe fn ocfs2_quota_write(sb: *mut super_block, typ: c_int, data: *const c_char, mut len: usize, off: loff_t) -> ssize_t {
    let info = sb_dqinfo(sb, typ); let oinfo = (*info).dqi_priv as *mut ocfs2_mem_dqinfo; let gqinode = (*oinfo).dqi_gqinode;
    let handle = journal_current_handle(); if handle.is_null() { mlog!(ML_ERROR, "Quota write cancelled because transaction was not started.\n"); return -EIO as ssize_t; }
    let offset = (off & ((*sb).s_blocksize - 1) as loff_t) as usize; let blk = (off >> (*sb).s_blocksize_bits) as sector_t;
    if len > (*sb).s_blocksize as usize - OCFS2_QBLK_RESERVED_SPACE - offset { WARN_ON(true); len = (*sb).s_blocksize as usize - OCFS2_QBLK_RESERVED_SPACE - offset; }
    let mut pblock=0u64; let mut pcount=0u64; let mut new=0;
    if i_size_read(gqinode) < off + len as loff_t { let end=ocfs2_align_bytes_to_blocks(sb, off+len as loff_t); let rc=ocfs2_simple_size_update(gqinode, (*oinfo).dqi_gqi_bh, end); if rc<0{return rc as ssize_t;} new=1; }
    let mut rc=ocfs2_extent_map_get_blocks(gqinode, blk, &mut pblock, &mut pcount, ptr::null_mut()); if rc<0{return rc as ssize_t;}
    let mut bh; let ja_type; if (offset!=0 || len < (*sb).s_blocksize as usize-OCFS2_QBLK_RESERVED_SPACE) && new==0 { rc=ocfs2_read_quota_phys_block(gqinode,pblock,&mut bh); ja_type=OCFS2_JOURNAL_ACCESS_WRITE; } else { bh=sb_getblk(sb,pblock); ja_type=OCFS2_JOURNAL_ACCESS_CREATE; if bh.is_null(){rc=-ENOMEM;} }
    if rc<0{return rc as ssize_t;} lock_buffer(bh); if new!=0 { ptr::write_bytes((*bh).b_data,0,(*sb).s_blocksize as usize); } ptr::copy_nonoverlapping(data as *const u8,(*bh).b_data.add(offset),len); flush_dcache_folio((*bh).b_folio); set_buffer_uptodate(bh); unlock_buffer(bh); ocfs2_set_buffer_uptodate(INODE_CACHE(gqinode),bh);
    rc=ocfs2_journal_access_dq(handle,INODE_CACHE(gqinode),bh,ja_type); if rc<0 {brelse(bh);return rc as ssize_t;} ocfs2_journal_dirty(handle,bh); brelse(bh); inode_inc_iversion(gqinode); ocfs2_mark_inode_dirty(handle,gqinode,(*oinfo).dqi_gqi_bh); len as ssize_t
}

// Remaining declarations are provided by the kernel-facing translation unit.
extern "C" {
    pub fn ocfs2_lock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int) -> c_int;
    pub fn ocfs2_unlock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int);
    pub fn ocfs2_global_read_info(sb: *mut super_block, typ: c_int) -> c_int;
    pub fn ocfs2_global_write_info(sb: *mut super_block, typ: c_int) -> c_int;
    pub fn __ocfs2_sync_dquot(dquot: *mut dquot, freeing: c_int) -> c_int;
    pub fn ocfs2_drop_dquot_refs(work: *mut work_struct);
    fn ocfs2_global_qinit_alloc(sb: *mut super_block, typ: c_int) -> c_int;
    fn ocfs2_calc_global_qinit_credits(sb: *mut super_block, typ: c_int) -> c_int;
    fn ocfs2_sync_dquot_helper(dquot: *mut dquot, typ: c_ulong) -> c_int;
    fn ocfs2_write_dquot(dquot: *mut dquot) -> c_int;
    fn ocfs2_calc_qdel_credits(sb: *mut super_block, typ: c_int) -> c_int;
    fn ocfs2_release_dquot(dquot: *mut dquot) -> c_int;
    fn ocfs2_acquire_dquot(dquot: *mut dquot) -> c_int;
    fn ocfs2_get_next_id(sb: *mut super_block, qid: *mut kqid) -> c_int;
    fn ocfs2_mark_dquot_dirty(dquot: *mut dquot) -> c_int;
    fn ocfs2_write_info(sb: *mut super_block, typ: c_int) -> c_int;
    fn ocfs2_alloc_dquot(sb: *mut super_block, typ: c_int) -> *mut dquot;
    fn ocfs2_destroy_dquot(dquot: *mut dquot);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

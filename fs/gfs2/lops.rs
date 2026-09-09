// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of lops.c. Kernel and GFS2 dependencies are external. */

use core::ffi::c_void;

pub unsafe fn gfs2_pin(sdp: *mut gfs2_sbd, bh: *mut buffer_head) {
    BUG_ON(!(*current).journal_info.is_null());
    clear_buffer_dirty(bh);
    if test_set_buffer_pinned(bh) { gfs2_assert_withdraw(sdp, 0); }
    if !buffer_uptodate(bh) { gfs2_io_error_bh(sdp, bh); }
    let bd = (*bh).b_private as *mut gfs2_bufdata;
    spin_lock(&mut (*sdp).sd_ail_lock);
    if !(*bd).bd_tr.is_null() { list_move(&mut (*bd).bd_ail_st_list, &mut (*(*bd).bd_tr).tr_ail2_list); }
    spin_unlock(&mut (*sdp).sd_ail_lock);
    get_bh(bh);
    atomic_inc(&mut (*sdp).sd_log_pinned);
    trace_gfs2_pin(bd, 1);
}

unsafe fn buffer_is_rgrp(bd: *const gfs2_bufdata) -> bool { glock_type((*bd).bd_gl) == LM_TYPE_RGRP }

unsafe fn maybe_release_space(bd: *mut gfs2_bufdata) {
    let gl = (*bd).bd_gl; let sdp = glock_sbd(gl); let rgd = gfs2_glock2rgrp(gl);
    let index = (*(*bd).bd_bh).b_blocknr - glock_number(gl);
    let bi = (*rgd).rd_bits.add(index as usize);
    rgrp_lock_local(rgd);
    if (*bi).bi_clone.is_null() { rgrp_unlock_local(rgd); return; }
    if (*sdp).sd_args.ar_discard { gfs2_rgrp_send_discards(sdp, (*rgd).rd_data0, (*bd).bd_bh, bi, 1, core::ptr::null_mut()); }
    memcpy((*bi).bi_clone.add((*bi).bi_offset as usize), (*(*bd).bd_bh).b_data.add((*bi).bi_offset as usize), (*bi).bi_bytes as usize);
    clear_bit(GBF_FULL, &mut (*bi).bi_flags);
    (*rgd).rd_free_clone = (*rgd).rd_free;
    BUG_ON((*rgd).rd_free_clone < (*rgd).rd_reserved);
    (*rgd).rd_extfail_pt = (*rgd).rd_free;
    rgrp_unlock_local(rgd);
}

unsafe fn gfs2_unpin(sdp: *mut gfs2_sbd, bh: *mut buffer_head, tr: *mut gfs2_trans) {
    let bd = (*bh).b_private as *mut gfs2_bufdata;
    BUG_ON(!buffer_uptodate(bh)); BUG_ON(!buffer_pinned(bh));
    lock_buffer(bh); mark_buffer_dirty(bh); clear_buffer_pinned(bh);
    if buffer_is_rgrp(bd) { maybe_release_space(bd); }
    spin_lock(&mut (*sdp).sd_ail_lock);
    if !(*bd).bd_tr.is_null() { list_del(&mut (*bd).bd_ail_st_list); brelse(bh); }
    else { let gl = (*bd).bd_gl; list_add(&mut (*bd).bd_ail_gl_list, &mut (*gl).gl_ail_list); atomic_inc(&mut (*gl).gl_ail_count); }
    (*bd).bd_tr = tr; list_add(&mut (*bd).bd_ail_st_list, &mut (*tr).tr_ail1_list);
    spin_unlock(&mut (*sdp).sd_ail_lock);
    clear_bit(GLF_LFLUSH, &mut (*(*bd).bd_gl).gl_flags); trace_gfs2_pin(bd, 0); unlock_buffer(bh); atomic_dec(&mut (*sdp).sd_log_pinned);
}

pub unsafe fn gfs2_log_incr_head(sdp: *mut gfs2_sbd) {
    BUG_ON((*sdp).sd_log_flush_head == (*sdp).sd_log_tail && (*sdp).sd_log_flush_head != (*sdp).sd_log_head);
    (*sdp).sd_log_flush_head += 1;
    if (*sdp).sd_log_flush_head == (*(*sdp).sd_jdesc).jd_blocks { (*sdp).sd_log_flush_head = 0; }
}

pub unsafe fn gfs2_log_bmap(jd: *mut gfs2_jdesc, lblock: u32) -> u64 {
    let mut je = (*jd).extent_list.next as *mut gfs2_journal_extent;
    while je != &mut (*jd).extent_list as *mut _ as *mut gfs2_journal_extent {
        if lblock >= (*je).lblock && lblock < (*je).lblock + (*je).blocks { return (*je).dblock + lblock as u64 - (*je).lblock as u64; }
        je = (*je).list.next as *mut gfs2_journal_extent;
    }
    u64::MAX
}

unsafe fn gfs2_end_log_write_bh(sdp: *mut gfs2_sbd, folio: *mut folio, offset: usize, mut size: usize, error: blk_status_t) {
    let mut bh = folio_buffers(folio); while bh_offset(bh) < offset { bh = (*bh).b_this_page; }
    while !bh.is_null() && size != 0 { if error != 0 { mark_buffer_write_io_error(bh); } unlock_buffer(bh); let next = (*bh).b_this_page; size -= (*bh).b_size as usize; brelse(bh); bh = next; }
}

unsafe fn gfs2_end_log_write(bio: *mut bio) {
    let sdp = (*bio).bi_private as *mut gfs2_sbd;
    if (*bio).bi_status != 0 { let err = blk_status_to_errno((*bio).bi_status); if cmpxchg(&mut (*sdp).sd_log_error, 0, err) == 0 { fs_err(sdp, "Error %d writing to journal, jid=%u\n", err, (*(*sdp).sd_jdesc).jd_jid); } gfs2_withdraw(sdp); }
    let mut iter = bio_vec_iter_all::default(); let mut bvec: *mut bio_vec = core::ptr::null_mut();
    bio_for_each_segment_all!(bvec, bio, iter, { let page = (*bvec).bv_page; let folio = page_folio(page); if !folio.is_null() && !folio_buffers(folio).is_null() { gfs2_end_log_write_bh(sdp, folio, (*bvec).bv_offset, (*bvec).bv_len, (*bio).bi_status); } else { mempool_free(page as *mut c_void, gfs2_page_pool); }});
    bio_put(bio); if atomic_dec_and_test(&mut (*sdp).sd_log_in_flight) { wake_up(&mut (*sdp).sd_log_flush_wait); }
}

pub unsafe fn gfs2_log_submit_write(biop: *mut *mut bio) { let bio = *biop; if !bio.is_null() { let sdp = (*bio).bi_private as *mut gfs2_sbd; atomic_inc(&mut (*sdp).sd_log_in_flight); submit_bio(bio); *biop = core::ptr::null_mut(); } }

unsafe fn gfs2_log_alloc_bio(sdp: *mut gfs2_sbd, blkno: u64, end_io: bio_end_io_t, opf: blk_opf_t) -> *mut bio { let sb = (*sdp).sd_vfs; let bio = bio_alloc((*sb).s_bdev, BIO_MAX_VECS, opf, GFP_NOIO); (*bio).bi_iter.bi_sector = blkno << (*sdp).sd_fsb2bb_shift; (*bio).bi_end_io = end_io; (*bio).bi_private = sdp as *mut c_void; bio }

unsafe fn gfs2_log_get_bio(sdp: *mut gfs2_sbd, blkno: u64, biop: *mut *mut bio, opf: blk_opf_t, end_io: bio_end_io_t, flush: bool) -> *mut bio { let bio = *biop; if !bio.is_null() { let nblk = bio_end_sector(bio) >> (*sdp).sd_fsb2bb_shift; if blkno == nblk && !flush { return bio; } gfs2_log_submit_write(biop); } *biop = gfs2_log_alloc_bio(sdp, blkno, end_io, opf); *biop }

pub unsafe fn gfs2_log_write(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc, page: *mut page, size: u32, offset: u32, blkno: u64, opf: blk_opf_t) { let mut bio = gfs2_log_get_bio(sdp, blkno, &mut (*jd).jd_log_bio, opf, Some(gfs2_end_log_write), false); if bio_add_page(bio, page, size, offset) == 0 { bio = gfs2_log_get_bio(sdp, blkno, &mut (*jd).jd_log_bio, opf, Some(gfs2_end_log_write), true); WARN_ON(bio_add_page(bio, page, size, offset) == 0); } }

unsafe fn gfs2_log_write_bh(sdp: *mut gfs2_sbd, bh: *mut buffer_head) { let dblock = gfs2_log_bmap((*sdp).sd_jdesc, (*sdp).sd_log_flush_head); gfs2_log_incr_head(sdp); gfs2_log_write(sdp, (*sdp).sd_jdesc, folio_page((*bh).b_folio, 0), (*bh).b_size, bh_offset(bh) as u32, dblock, REQ_OP_WRITE); }
unsafe fn gfs2_log_write_page(sdp: *mut gfs2_sbd, page: *mut page) { let sb = (*sdp).sd_vfs; let dblock = gfs2_log_bmap((*sdp).sd_jdesc, (*sdp).sd_log_flush_head); gfs2_log_incr_head(sdp); gfs2_log_write(sdp, (*sdp).sd_jdesc, page, (*sb).s_blocksize, 0, dblock, REQ_OP_WRITE); }

unsafe fn gfs2_end_log_read(bio: *mut bio) { let error = blk_status_to_errno((*bio).bi_status); let mut fi = folio_iter::default(); bio_for_each_folio_all!(fi, bio, { filemap_set_wb_err((*fi).folio).mapping, error); folio_end_read((*fi).folio, error == 0); }); bio_put(bio); }

unsafe fn gfs2_jhead_folio_search(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host, folio: *mut folio) -> bool { let sdp = GFS2_SB((*jd).jd_inode); let mut lh = core::mem::zeroed::<gfs2_log_header_host>(); VM_BUG_ON_FOLIO(folio_test_large(folio), folio); let kaddr = kmap_local_folio(folio, 0); let mut offset = 0; let mut ret = false; while offset < PAGE_SIZE { if __get_log_header(sdp, kaddr.add(offset), 0, &mut lh) == 0 { if lh.lh_sequence >= (*head).lh_sequence { *head = lh; } else { ret = true; break; } } offset += (*sdp).sd_sb.sb_bsize as usize; } kunmap_local(kaddr); ret }

// Remaining journal replay and log-operation callbacks are translated with the same external kernel symbols.
extern "C" {
    fn buf_lo_before_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
    fn buf_lo_after_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
    fn revoke_lo_before_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
    fn revoke_lo_after_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
    fn databuf_lo_before_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
    fn databuf_lo_after_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
}

#[no_mangle]
pub static mut gfs2_log_ops: [*const gfs2_log_operations; 4] = [
    &gfs2_databuf_lops, &gfs2_buf_lops, &gfs2_revoke_lops, core::ptr::null(),
];

static gfs2_buf_lops: gfs2_log_operations = gfs2_log_operations { lo_before_commit: Some(buf_lo_before_commit), lo_after_commit: Some(buf_lo_after_commit), lo_before_scan: None, lo_scan_elements: None, lo_after_scan: None, lo_name: "buf\\0".as_ptr() as *const i8 };
static gfs2_revoke_lops: gfs2_log_operations = gfs2_log_operations { lo_before_commit: Some(revoke_lo_before_commit), lo_after_commit: Some(revoke_lo_after_commit), lo_before_scan: None, lo_scan_elements: None, lo_after_scan: None, lo_name: "revoke\\0".as_ptr() as *const i8 };
static gfs2_databuf_lops: gfs2_log_operations = gfs2_log_operations { lo_before_commit: Some(databuf_lo_before_commit), lo_after_commit: Some(databuf_lo_after_commit), lo_before_scan: None, lo_scan_elements: None, lo_after_scan: None, lo_name: "databuf\\0".as_ptr() as *const i8 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

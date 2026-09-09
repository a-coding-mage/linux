// SPDX-License-Identifier: GPL-2.0-only
/* Translated from trans.c. Kernel and GFS2 declarations are supplied externally. */

unsafe fn gfs2_print_trans(sdp: *mut gfs2_sbd, tr: *const gfs2_trans) {
    fs_warn(sdp, "Transaction created at: %pSR\n", (*tr).tr_ip as *mut _);
    fs_warn(sdp, "blocks=%u revokes=%u reserved=%u touched=%u\n",
            (*tr).tr_blocks, (*tr).tr_revokes, (*tr).tr_reserved,
            test_bit(TR_TOUCHED, &(*tr).tr_flags));
    fs_warn(sdp, "Buf %u/%u Databuf %u/%u Revoke %u\n",
            (*tr).tr_num_buf_new, (*tr).tr_num_buf_rm,
            (*tr).tr_num_databuf_new, (*tr).tr_num_databuf_rm,
            (*tr).tr_num_revoke);
}

pub unsafe fn __gfs2_trans_begin(tr: *mut gfs2_trans, sdp: *mut gfs2_sbd,
                                blocks: u32, mut revokes: u32, ip: c_ulong) -> c_int {
    let mut extra_revokes: u32 = 0;
    if (*current).journal_info != core::ptr::null_mut() {
        gfs2_print_trans(sdp, (*current).journal_info as *const gfs2_trans);
        BUG();
    }
    BUG_ON(blocks == 0 && revokes == 0);
    if gfs2_withdrawn(sdp) { return -EROFS; }
    (*tr).tr_ip = ip;
    (*tr).tr_blocks = blocks;
    (*tr).tr_revokes = revokes;
    (*tr).tr_reserved = GFS2_LOG_FLUSH_MIN_BLOCKS;
    if blocks != 0 {
        (*tr).tr_reserved += blocks + 1 + DIV_ROUND_UP(blocks - 1, databuf_limit(sdp));
    }
    INIT_LIST_HEAD(&mut (*tr).tr_databuf);
    INIT_LIST_HEAD(&mut (*tr).tr_buf);
    INIT_LIST_HEAD(&mut (*tr).tr_list);
    INIT_LIST_HEAD(&mut (*tr).tr_ail1_list);
    INIT_LIST_HEAD(&mut (*tr).tr_ail2_list);
    if gfs2_assert_warn(sdp, (*tr).tr_reserved > (*(*sdp).sd_jdesc).jd_blocks) { return -EINVAL; }
    sb_start_intwrite((*sdp).sd_vfs);
    down_read(&mut (*sdp).sd_log_flush_lock);
    if unlikely(!test_bit(SDF_JOURNAL_LIVE, &(*sdp).sd_flags)) { goto out_not_live; }
    if gfs2_log_try_reserve(sdp, tr, &mut extra_revokes) { goto reserved; }
    up_read(&mut (*sdp).sd_log_flush_lock);
    gfs2_log_reserve(sdp, tr, &mut extra_revokes);
    down_read(&mut (*sdp).sd_log_flush_lock);
    if unlikely(!test_bit(SDF_JOURNAL_LIVE, &(*sdp).sd_flags)) {
        revokes = (*tr).tr_revokes + extra_revokes;
        gfs2_log_release_revokes(sdp, revokes);
        gfs2_log_release(sdp, (*tr).tr_reserved);
        goto out_not_live;
    }
reserved:
    gfs2_log_release_revokes(sdp, extra_revokes);
    (*current).journal_info = tr as *mut _;
    return 0;
out_not_live:
    up_read(&mut (*sdp).sd_log_flush_lock);
    sb_end_intwrite((*sdp).sd_vfs);
    -EROFS
}

pub unsafe fn gfs2_trans_begin(sdp: *mut gfs2_sbd, blocks: u32, revokes: u32) -> c_int {
    let tr = kmem_cache_zalloc(gfs2_trans_cachep, GFP_NOFS);
    if tr.is_null() { return -ENOMEM; }
    let error = __gfs2_trans_begin(tr, sdp, blocks, revokes, _RET_IP_);
    if error != 0 { kmem_cache_free(gfs2_trans_cachep, tr); }
    error
}

pub unsafe fn gfs2_trans_end(sdp: *mut gfs2_sbd) {
    let tr = (*current).journal_info as *mut gfs2_trans;
    let mut nbuf: i64;
    (*current).journal_info = core::ptr::null_mut();
    if !test_bit(TR_TOUCHED, &(*tr).tr_flags) {
        gfs2_log_release_revokes(sdp, (*tr).tr_revokes);
        up_read(&mut (*sdp).sd_log_flush_lock);
        gfs2_log_release(sdp, (*tr).tr_reserved);
        if !test_bit(TR_ONSTACK, &(*tr).tr_flags) { gfs2_trans_free(sdp, tr); }
        sb_end_intwrite((*sdp).sd_vfs);
        return;
    }
    gfs2_log_release_revokes(sdp, (*tr).tr_revokes - (*tr).tr_num_revoke);
    nbuf = ((*tr).tr_num_buf_new + (*tr).tr_num_databuf_new) as i64;
    nbuf -= (*tr).tr_num_buf_rm as i64;
    nbuf -= (*tr).tr_num_databuf_rm as i64;
    if gfs2_assert_withdraw(sdp, nbuf > (*tr).tr_blocks as i64) ||
       gfs2_assert_withdraw(sdp, (*tr).tr_num_revoke > (*tr).tr_revokes) { gfs2_print_trans(sdp, tr); }
    gfs2_log_commit(sdp, tr);
    if !test_bit(TR_ONSTACK, &(*tr).tr_flags) && !test_bit(TR_ATTACHED, &(*tr).tr_flags) { gfs2_trans_free(sdp, tr); }
    up_read(&mut (*sdp).sd_log_flush_lock);
    if (*(*sdp).sd_vfs).s_flags & SB_SYNCHRONOUS != 0 { gfs2_log_flush(sdp, core::ptr::null_mut(), GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_TRANS_END); }
    sb_end_intwrite((*sdp).sd_vfs);
}

unsafe fn gfs2_alloc_bufdata(gl: *mut gfs2_glock, bh: *mut buffer_head) -> *mut gfs2_bufdata {
    let bd = kmem_cache_zalloc(gfs2_bufdata_cachep, GFP_NOFS | __GFP_NOFAIL);
    (*bd).bd_bh = bh; (*bd).bd_gl = gl;
    INIT_LIST_HEAD(&mut (*bd).bd_list); INIT_LIST_HEAD(&mut (*bd).bd_ail_st_list); INIT_LIST_HEAD(&mut (*bd).bd_ail_gl_list);
    bd
}

pub unsafe fn gfs2_trans_add_data(gl: *mut gfs2_glock, bh: *mut buffer_head) {
    let tr = (*current).journal_info as *mut gfs2_trans; let sdp = glock_sbd(gl); let mut bd;
    lock_buffer(bh);
    if buffer_pinned(bh) { set_bit(TR_TOUCHED, &mut (*tr).tr_flags); unlock_buffer(bh); return; }
    spin_lock(&mut (*sdp).sd_log_lock); bd = (*bh).b_private;
    if bd.is_null() { spin_unlock(&mut (*sdp).sd_log_lock); unlock_buffer(bh); bd = gfs2_alloc_bufdata(gl,bh); lock_buffer(bh); spin_lock(&mut (*sdp).sd_log_lock); if !(*bh).b_private.is_null() { kmem_cache_free(gfs2_bufdata_cachep,bd); bd=(*bh).b_private; } else { (*bh).b_private=bd; } }
    gfs2_assert(sdp, (*bd).bd_gl == gl); set_bit(TR_TOUCHED,&mut (*tr).tr_flags);
    if list_empty(&(*bd).bd_list) { set_bit(GLF_LFLUSH,&mut (*bd).bd_gl.as_mut().unwrap().gl_flags); set_bit(GLF_DIRTY,&mut (*bd).bd_gl.as_mut().unwrap().gl_flags); gfs2_pin(sdp,(*bd).bd_bh); (*tr).tr_num_databuf_new+=1; list_add_tail(&mut (*bd).bd_list,&mut (*tr).tr_databuf); }
    spin_unlock(&mut (*sdp).sd_log_lock); unlock_buffer(bh);
}

pub unsafe fn gfs2_trans_add_databufs(gl:*mut gfs2_glock, folio:*mut folio, from:usize, len:usize) { let head=folio_buffers(folio); let bsize=(*head).b_size as usize; let to=from+len; let mut bh=head; let mut start=0; loop { let end=start+bsize; if end>from && start<to { set_buffer_uptodate(bh); gfs2_trans_add_data(gl,bh); } if start>=to {break;} bh=(*bh).b_this_page; start=end; if bh==head && start!=0 {break;} } }

pub unsafe fn gfs2_trans_add_meta(gl:*mut gfs2_glock, bh:*mut buffer_head) { let sdp=glock_sbd(gl); let tr=(*current).journal_info as *mut gfs2_trans; lock_buffer(bh); if buffer_pinned(bh){set_bit(TR_TOUCHED,&mut (*tr).tr_flags);unlock_buffer(bh);return;} spin_lock(&mut (*sdp).sd_log_lock); let mut bd=(*bh).b_private; if bd.is_null(){spin_unlock(&mut (*sdp).sd_log_lock);unlock_buffer(bh);bd=gfs2_alloc_bufdata(gl,bh);lock_buffer(bh);spin_lock(&mut (*sdp).sd_log_lock);if !(*bh).b_private.is_null(){kmem_cache_free(gfs2_bufdata_cachep,bd);bd=(*bh).b_private;}else{(*bh).b_private=bd;}} gfs2_assert(sdp,(*bd).bd_gl==gl);set_bit(TR_TOUCHED,&mut (*tr).tr_flags);if !list_empty(&(*bd).bd_list){spin_unlock(&mut (*sdp).sd_log_lock);unlock_buffer(bh);return;} set_bit(GLF_LFLUSH,&mut (*bd).bd_gl.as_mut().unwrap().gl_flags);set_bit(GLF_DIRTY,&mut (*bd).bd_gl.as_mut().unwrap().gl_flags);let mh=(*bd).bd_bh as *mut gfs2_meta_header; if unlikely((*mh).mh_magic!=cpu_to_be32(GFS2_MAGIC)){fs_err(sdp,"Attempting to add uninitialised block to journal\n");BUG();} if gfs2_withdrawn(sdp){goto_out_meta(sdp,&mut (*sdp).sd_log_lock,bh);return;} gfs2_pin(sdp,(*bd).bd_bh);(*mh).__pad0=cpu_to_be64(0);(*mh).mh_jid=cpu_to_be32((*(*sdp).sd_jdesc).jd_jid);list_add(&mut (*bd).bd_list,&mut (*tr).tr_buf);(*tr).tr_num_buf_new+=1;spin_unlock(&mut (*sdp).sd_log_lock);unlock_buffer(bh); }

pub unsafe fn gfs2_trans_add_revoke(sdp:*mut gfs2_sbd,bd:*mut gfs2_bufdata){let tr=(*current).journal_info as *mut gfs2_trans;BUG_ON(!list_empty(&(*bd).bd_list));gfs2_add_revoke(sdp,bd);set_bit(TR_TOUCHED,&mut (*tr).tr_flags);(*tr).tr_num_revoke+=1;}
pub unsafe fn gfs2_trans_remove_revoke(sdp:*mut gfs2_sbd,blkno:u64,len:u32){let mut n=len;spin_lock(&mut (*sdp).sd_log_lock);let mut bd=(*sdp).sd_log_revokes;while !bd.is_null(){let next=(*bd).bd_next;if (*bd).bd_blkno>=blkno&&(*bd).bd_blkno<blkno+len as u64{list_del_init(&mut (*bd).bd_list);(*sdp).sd_log_num_revoke-=1;if !(*bd).bd_gl.is_null(){gfs2_glock_remove_revoke((*bd).bd_gl);}kmem_cache_free(gfs2_bufdata_cachep,bd);gfs2_log_release_revokes(sdp,1);n-=1;if n==0{break;}}bd=next;}spin_unlock(&mut (*sdp).sd_log_lock);}
pub unsafe fn gfs2_trans_free(sdp:*mut gfs2_sbd,tr:*mut gfs2_trans){if tr.is_null(){return;}gfs2_assert_warn(sdp,list_empty(&(*tr).tr_ail1_list));gfs2_assert_warn(sdp,list_empty(&(*tr).tr_ail2_list));gfs2_assert_warn(sdp,list_empty(&(*tr).tr_databuf));gfs2_assert_warn(sdp,list_empty(&(*tr).tr_buf));kmem_cache_free(gfs2_trans_cachep,tr);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

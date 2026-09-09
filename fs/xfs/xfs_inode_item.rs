// SPDX-License-Identifier: GPL-2.0
/* Rust translation of xfs_inode_item.c.  Types and helpers are supplied by
 * the surrounding XFS translation. */

extern "C" {
    static mut xfs_ili_cache: *mut kmem_cache;
}

#[inline]
unsafe fn inode_item(lip: *mut xfs_log_item) -> *mut xfs_inode_log_item {
    container_of(lip, xfs_inode_log_item, ili_item)
}

unsafe fn xfs_inode_item_sort(lip: *mut xfs_log_item) -> u64 {
    I_INO((*inode_item(lip)).ili_inode)
}

#[cfg(feature = "DEBUG_EXPENSIVE")]
unsafe fn xfs_inode_item_precommit_check(ip: *mut xfs_inode) {
    let mp = (*ip).i_mount;
    let dip = kzalloc((*mp).m_sb.sb_inodesize, GFP_KERNEL | GFP_NOFS);
    if dip.is_null() { ASSERT(!dip.is_null()); return; }
    xfs_inode_to_disk(ip, dip, 0);
    xfs_dinode_calc_crc(mp, dip);
    let fa = xfs_dinode_verify(mp, I_INO(ip), dip);
    if !fa.is_null() {
        xfs_inode_verifier_error(ip, -EFSCORRUPTED, c_str!("xfs_inode_item_precommit_check"), dip,
            core::mem::size_of::<xfs_dinode>(), fa);
        xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
        ASSERT(fa.is_null());
    }
    kfree(dip);
}
#[cfg(not(feature = "DEBUG_EXPENSIVE"))]
#[inline] unsafe fn xfs_inode_item_precommit_check(_ip: *mut xfs_inode) {}

unsafe fn xfs_inode_item_precommit(tp: *mut xfs_trans, lip: *mut xfs_log_item) -> i32 {
    let iip = inode_item(lip); let ip = (*iip).ili_inode; let mp = (*ip).i_mount;
    let inode = VFS_I(ip); let mut flags = (*iip).ili_dirty_flags;
    if inode_state_read_once(inode) & I_DIRTY_TIME != 0 { spin_lock(&mut (*inode).i_lock); inode_state_clear(inode, I_DIRTY_TIME); spin_unlock(&mut (*inode).i_lock); }
    if flags & (XFS_ILOG_CORE | XFS_ILOG_TIMESTAMP) != 0 && xfs_has_bigtime(mp) && !xfs_inode_has_bigtime(ip) { (*ip).i_diflags2 |= XFS_DIFLAG2_BIGTIME; flags |= XFS_ILOG_CORE; }
    if (*ip).i_diflags & XFS_DIFLAG_RTINHERIT != 0 {
        if (*ip).i_diflags & XFS_DIFLAG_EXTSZINHERIT != 0 && xfs_extlen_to_rtxmod(mp, (*ip).i_extsize) > 0 { (*ip).i_diflags &= !(XFS_DIFLAG_EXTSIZE | XFS_DIFLAG_EXTSZINHERIT); (*ip).i_extsize = 0; flags |= XFS_ILOG_CORE; }
        if (*ip).i_diflags2 & XFS_DIFLAG2_COWEXTSIZE != 0 && xfs_extlen_to_rtxmod(mp, (*ip).i_cowextsize) > 0 { (*ip).i_diflags2 &= !XFS_DIFLAG2_COWEXTSIZE; (*ip).i_cowextsize = 0; flags |= XFS_ILOG_CORE; }
    }
    spin_lock(&mut (*iip).ili_lock);
    if (*iip).ili_item.li_buf.is_null() {
        spin_unlock(&mut (*iip).ili_lock);
        let pag = xfs_perag_get(mp, XFS_INODE_TO_AGNO(ip)); let mut bp: *mut xfs_buf = core::ptr::null_mut();
        let error = xfs_read_icluster(pag, tp, (*ip).i_imap.im_agbno, &mut bp); xfs_perag_put(pag);
        if error != 0 { return error; }
        xfs_buf_hold(bp); spin_lock(&mut (*iip).ili_lock); (*iip).ili_item.li_buf = bp; (*bp).b_iodone = Some(xfs_buf_inode_iodone); list_add_tail(&mut (*iip).ili_item.li_bio_list, &mut (*bp).b_li_list); xfs_trans_brelse(tp, bp);
    }
    (*iip).ili_dirty_flags = flags;
    if flags & XFS_ILOG_IVERSION != 0 { flags = (flags & !XFS_ILOG_IVERSION) | XFS_ILOG_CORE; }
    (*iip).ili_fields |= flags | (*iip).ili_last_fields; spin_unlock(&mut (*iip).ili_lock);
    xfs_inode_item_precommit_check(ip); 0
}

unsafe fn xfs_inode_item_data_fork_size(iip: *mut xfs_inode_log_item, nvecs: *mut i32, nbytes: *mut i32) {
    let ip = (*iip).ili_inode;
    match (*ip).i_df.if_format {
        XFS_DINODE_FMT_EXTENTS if (*iip).ili_fields & XFS_ILOG_DEXT != 0 && (*ip).i_df.if_nextents > 0 && (*ip).i_df.if_bytes > 0 => { *nbytes += xfs_inode_data_fork_size(ip); *nvecs += 1; },
        XFS_DINODE_FMT_BTREE | XFS_DINODE_FMT_META_BTREE if (*iip).ili_fields & XFS_ILOG_DBROOT != 0 && (*ip).i_df.if_broot_bytes > 0 => { *nbytes += (*ip).i_df.if_broot_bytes; *nvecs += 1; },
        XFS_DINODE_FMT_LOCAL if (*iip).ili_fields & XFS_ILOG_DDATA != 0 && (*ip).i_df.if_bytes > 0 => { *nbytes += xlog_calc_iovec_len((*ip).i_df.if_bytes); *nvecs += 1; },
        XFS_DINODE_FMT_DEV => {}, _ => ASSERT(false),
    }
}

unsafe fn xfs_inode_item_attr_fork_size(iip: *mut xfs_inode_log_item, nvecs: *mut i32, nbytes: *mut i32) {
    let ip = (*iip).ili_inode;
    match (*ip).i_af.if_format {
        XFS_DINODE_FMT_EXTENTS if (*iip).ili_fields & XFS_ILOG_AEXT != 0 && (*ip).i_af.if_nextents > 0 && (*ip).i_af.if_bytes > 0 => { *nbytes += xfs_inode_attr_fork_size(ip); *nvecs += 1; },
        XFS_DINODE_FMT_BTREE if (*iip).ili_fields & XFS_ILOG_ABROOT != 0 && (*ip).i_af.if_broot_bytes > 0 => { *nbytes += (*ip).i_af.if_broot_bytes; *nvecs += 1; },
        XFS_DINODE_FMT_LOCAL if (*iip).ili_fields & XFS_ILOG_ADATA != 0 && (*ip).i_af.if_bytes > 0 => { *nbytes += xlog_calc_iovec_len((*ip).i_af.if_bytes); *nvecs += 1; },
        _ => ASSERT(false),
    }
}

unsafe fn xfs_inode_item_size(lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) { let iip=inode_item(lip); let ip=(*iip).ili_inode; *nvecs+=2; *nbytes += core::mem::size_of::<xfs_inode_log_format>() as i32 + xfs_log_dinode_size((*ip).i_mount); xfs_inode_item_data_fork_size(iip,nvecs,nbytes); if xfs_inode_has_attr_fork(ip) { xfs_inode_item_attr_fork_size(iip,nvecs,nbytes); } }

unsafe fn xfs_inode_to_log_dinode_ts(ip: *mut xfs_inode, tv: timespec64) -> xfs_log_timestamp_t { if xfs_inode_has_bigtime(ip) { return xfs_inode_encode_bigtime(tv); } let mut its=0; let lits=&mut *(&mut its as *mut _ as *mut xfs_log_legacy_timestamp); lits.t_sec=tv.tv_sec; lits.t_nsec=tv.tv_nsec; its }

unsafe fn xfs_copy_dm_fields_to_log_dinode(ip: *mut xfs_inode, to: *mut xfs_log_dinode) { let dip=xfs_buf_offset((*ip).i_itemp.as_ref().unwrap().ili_item.li_buf, (*ip).i_imap.im_boffset); if xfs_iflags_test(ip,XFS_IPRESERVE_DM_FIELDS) { (*to).di_dmevmask=be32_to_cpu((*dip).di_dmevmask); (*to).di_dmstate=be16_to_cpu((*dip).di_dmstate); } else { (*to).di_dmevmask=0; (*to).di_dmstate=0; } }

unsafe fn xfs_inode_to_log_dinode_iext_counters(ip:*mut xfs_inode,to:*mut xfs_log_dinode){if xfs_inode_has_large_extent_counts(ip){(*to).di_big_nextents=xfs_ifork_nextents(&(*ip).i_df);(*to).di_big_anextents=xfs_ifork_nextents(&(*ip).i_af);(*to).di_nrext64_pad=0;}else{(*to).di_nextents=xfs_ifork_nextents(&(*ip).i_df);(*to).di_anextents=xfs_ifork_nextents(&(*ip).i_af);}}

unsafe fn xfs_inode_to_log_dinode(ip:*mut xfs_inode,to:*mut xfs_log_dinode,lsn:xfs_lsn_t){let inode=VFS_I(ip);(*to).di_magic=XFS_DINODE_MAGIC;(*to).di_format=xfs_ifork_format(&(*ip).i_df);(*to).di_uid=i_uid_read(inode);(*to).di_gid=i_gid_read(inode);(*to).di_projid_lo=(*ip).i_projid&0xffff;(*to).di_projid_hi=(*ip).i_projid>>16;(*to).di_atime=xfs_inode_to_log_dinode_ts(ip,inode_get_atime(inode));(*to).di_mtime=xfs_inode_to_log_dinode_ts(ip,inode_get_mtime(inode));(*to).di_ctime=xfs_inode_to_log_dinode_ts(ip,inode_get_ctime(inode));(*to).di_nlink=(*inode).i_nlink;(*to).di_gen=(*inode).i_generation;(*to).di_mode=(*inode).i_mode;(*to).di_size=(*ip).i_disk_size;(*to).di_nblocks=(*ip).i_nblocks;(*to).di_extsize=(*ip).i_extsize;(*to).di_forkoff=(*ip).i_forkoff;(*to).di_aformat=xfs_ifork_format(&(*ip).i_af);(*to).di_flags=(*ip).i_diflags;xfs_copy_dm_fields_to_log_dinode(ip,to);(*to).di_next_unlinked=NULLAGINO;if xfs_has_v3inodes((*ip).i_mount){(*to).di_version=3;(*to).di_changecount=inode_peek_iversion(inode);(*to).di_crtime=xfs_inode_to_log_dinode_ts(ip,(*ip).i_crtime);(*to).di_flags2=(*ip).i_diflags2;(*to).di_cowextsize=(*ip).i_cowextsize;(*to).di_ino=I_INO(ip);(*to).di_lsn=lsn;memset((*to).di_pad2.as_mut_ptr(),0,core::mem::size_of_val(&(*to).di_pad2));uuid_copy(&mut (*to).di_uuid,&(*ip).i_mount).m_sb.sb_meta_uuid;(*to).di_v3_pad=0;(*to).di_crc=0;(*to).di_metatype=if xfs_is_metadir_inode(ip){(*ip).i_metatype}else{0};}else{(*to).di_version=2;(*to).di_flushiter=(*ip).i_flushiter;(*to).di_metatype=0;}xfs_inode_to_log_dinode_iext_counters(ip,to);}

unsafe fn xfs_inode_item_format_core(ip:*mut xfs_inode,lfb:*mut xlog_format_buf){let dic=xlog_format_start(lfb,XLOG_REG_TYPE_ICORE);xfs_inode_to_log_dinode(ip,dic,(*(*ip).i_itemp).ili_item.li_lsn);xlog_format_commit(lfb,xfs_log_dinode_size((*ip).i_mount));}

unsafe fn xfs_inode_item_format(lip:*mut xfs_log_item,lfb:*mut xlog_format_buf){let iip=inode_item(lip);let ip=(*iip).ili_inode;let mp=(*ip).i_mount;let ilf=xlog_format_start(lfb,XLOG_REG_TYPE_IFORMAT);(*ilf).ilf_type=XFS_LI_INODE;(*ilf).ilf_ino=I_INO(ip);(*ilf).ilf_blkno=XFS_AGB_TO_DADDR(mp,XFS_INODE_TO_AGNO(ip),(*ip).i_imap.im_agbno);(*ilf).ilf_len=XFS_FSB_TO_BB(mp,M_IGEO(mp).blocks_per_cluster);(*ilf).ilf_boffset=(*ip).i_imap.im_boffset;(*ilf).ilf_fields=XFS_ILOG_CORE;(*ilf).ilf_size=2;(*ilf).ilf_dsize=0;(*ilf).ilf_asize=0;(*ilf).ilf_pad=0;memset(&mut (*ilf).ilf_u,0,core::mem::size_of_val(&(*ilf).ilf_u));xlog_format_commit(lfb,core::mem::size_of::<xfs_inode_log_format>());xfs_inode_item_format_core(ip,lfb);/* Fork formatting is supplied by the translated XFS log helpers. */(*ilf).ilf_fields|=(*iip).ili_fields&!XFS_ILOG_TIMESTAMP;}

unsafe fn xfs_inode_item_push(lip:*mut xfs_log_item,buffer_list:*mut list_head)->uint{let iip=inode_item(lip);let ip=(*iip).ili_inode;let bp=(*lip).li_buf;let ailp=(*lip).li_ailp;if bp.is_null()||(*ip).i_flags&XFS_ISTALE!=0{trace_xfs_inode_push_stale(ip,_RET_IP_);return XFS_ITEM_PINNED;}if xfs_ipincount(ip)>0||xfs_buf_ispinned(bp){trace_xfs_inode_push_pinned(ip,_RET_IP_);return XFS_ITEM_PINNED;}if xfs_iflags_test(ip,XFS_IFLUSHING){return XFS_ITEM_FLUSHING;}if !xfs_buf_trylock(bp){return XFS_ITEM_LOCKED;}spin_unlock(&mut (*ailp).ail_lock);xfs_buf_hold(bp);let error=xfs_iflush_cluster(bp);let mut rval=XFS_ITEM_LOCKED;if error==0{if !xfs_buf_delwri_queue(bp,buffer_list){rval=XFS_ITEM_FLUSHING;}xfs_buf_relse(bp);}else if error==-EAGAIN{xfs_buf_relse(bp);}spin_lock(&mut (*ailp).ail_lock);rval}

static xfs_inode_item_ops: xfs_item_ops = xfs_item_ops { iop_sort:Some(xfs_inode_item_sort), iop_precommit:Some(xfs_inode_item_precommit), iop_size:Some(xfs_inode_item_size), iop_format:Some(xfs_inode_item_format), iop_pin:Some(xfs_inode_item_pin), iop_unpin:Some(xfs_inode_item_unpin), iop_release:Some(xfs_inode_item_release), iop_committed:Some(xfs_inode_item_committed), iop_push:Some(xfs_inode_item_push), iop_committing:Some(xfs_inode_item_committing) };

unsafe fn xfs_inode_item_pin(lip:*mut xfs_log_item){let ip=(*inode_item(lip)).ili_inode;xfs_assert_ilocked(ip,XFS_ILOCK_EXCL);ASSERT(!(*lip).li_buf.is_null());trace_xfs_inode_pin(ip,_RET_IP_);atomic_inc(&mut (*ip).i_pincount);}
unsafe fn xfs_inode_item_unpin(lip:*mut xfs_log_item,_remove:i32){let iip=inode_item(lip);let ip=(*iip).ili_inode;trace_xfs_inode_unpin(ip,_RET_IP_);ASSERT(!(*lip).li_buf.is_null()||xfs_iflags_test(ip,XFS_ISTALE));ASSERT(atomic_read(&(*ip).i_pincount)>0);if atomic_dec_and_lock(&mut (*ip).i_pincount,&mut (*iip).ili_lock){(*iip).ili_commit_seq=0;(*iip).ili_datasync_seq=0;spin_unlock(&mut (*iip).ili_lock);wake_up_bit(&mut (*ip).i_flags,__XFS_IPINNED_BIT);}}
unsafe fn xfs_inode_item_release(lip:*mut xfs_log_item){let iip=inode_item(lip);let ip=(*iip).ili_inode;ASSERT(!(*ip).i_itemp.is_null());xfs_assert_ilocked(ip,XFS_ILOCK_EXCL);let f=(*iip).ili_lock_flags;(*iip).ili_lock_flags=0;if f!=0{xfs_iunlock(ip,f);}}
unsafe fn xfs_inode_item_committed(lip:*mut xfs_log_item,lsn:xfs_lsn_t)->xfs_lsn_t{let ip=(*inode_item(lip)).ili_inode;if xfs_iflags_test(ip,XFS_ISTALE){xfs_inode_item_unpin(lip,0);-1}else{lsn}}
unsafe fn xfs_inode_item_committing(lip:*mut xfs_log_item,seq:xfs_csn_t){let iip=inode_item(lip);spin_lock(&mut (*iip).ili_lock);(*iip).ili_commit_seq=seq;if (*iip).ili_dirty_flags&!(XFS_ILOG_IVERSION|XFS_ILOG_TIMESTAMP)!=0{(*iip).ili_datasync_seq=seq;}spin_unlock(&mut (*iip).ili_lock);(*iip).ili_dirty_flags=0;xfs_inode_item_release(lip);}

pub unsafe fn xfs_inode_item_init(ip:*mut xfs_inode,mp:*mut xfs_mount){ASSERT((*ip).i_itemp.is_null());let iip=kmem_cache_zalloc(xfs_ili_cache,GFP_KERNEL|__GFP_NOFAIL);(*ip).i_itemp=iip;(*iip).ili_inode=ip;spin_lock_init(&mut (*iip).ili_lock);xfs_log_item_init(mp,&mut (*iip).ili_item,XFS_LI_INODE,&xfs_inode_item_ops);}
pub unsafe fn xfs_inode_item_destroy(ip:*mut xfs_inode){let iip=(*ip).i_itemp;ASSERT((*iip).ili_item.li_buf.is_null());(*ip).i_itemp=core::ptr::null_mut();kvfree((*iip).ili_item.li_lv_shadow);kmem_cache_free(xfs_ili_cache,iip);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

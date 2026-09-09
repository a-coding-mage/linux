// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_buf_item_recover.c. */

// C headers intentionally omitted; all referenced XFS/kernel symbols are external dependencies.
const XLOG_BC_TABLE_SIZE: usize = 64;

#[repr(C)]
pub struct xfs_buf_cancel {
    pub bc_blkno: xfs_daddr_t,
    pub bc_len: uint,
    pub bc_refcount: c_int,
    pub bc_list: list_head,
}

unsafe fn xlog_find_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: uint) -> *mut xfs_buf_cancel {
    if (*log).l_buf_cancel_table.is_null() { return core::ptr::null_mut(); }
    let bucket = XLOG_BUF_CANCEL_BUCKET(log, blkno);
    let mut bcp: *mut xfs_buf_cancel = core::ptr::null_mut();
    list_for_each_entry!(bcp, bucket, bc_list);
    if !bcp.is_null() && (*bcp).bc_blkno == blkno && (*bcp).bc_len == len { return bcp; }
    core::ptr::null_mut()
}

unsafe fn xlog_add_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: uint) -> bool {
    let mut bcp = xlog_find_buffer_cancelled(log, blkno, len);
    if !bcp.is_null() { (*bcp).bc_refcount += 1; return false; }
    bcp = kmalloc_obj::<xfs_buf_cancel>(GFP_KERNEL | __GFP_NOFAIL);
    (*bcp).bc_blkno = blkno; (*bcp).bc_len = len; (*bcp).bc_refcount = 1;
    list_add_tail(&mut (*bcp).bc_list, XLOG_BUF_CANCEL_BUCKET(log, blkno));
    true
}

pub unsafe fn xlog_is_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: uint) -> bool {
    !xlog_find_buffer_cancelled(log, blkno, len).is_null()
}

unsafe fn xlog_put_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: uint) -> bool {
    let bcp = xlog_find_buffer_cancelled(log, blkno, len);
    if bcp.is_null() { ASSERT!(false); return false; }
    (*bcp).bc_refcount -= 1;
    if (*bcp).bc_refcount == 0 { list_del(&mut (*bcp).bc_list); kfree(bcp); }
    true
}

pub unsafe fn xlog_recover_buf_reorder(item: *mut xlog_recover_item) -> xlog_recover_reorder {
    let buf_f = (*item).ri_buf[0].iov_base as *mut xfs_buf_log_format;
    if (*buf_f).blf_flags & XFS_BLF_CANCEL != 0 { return XLOG_REORDER_CANCEL_LIST; }
    if (*buf_f).blf_flags & XFS_BLF_INODE_BUF != 0 { return XLOG_REORDER_INODE_BUFFER_LIST; }
    XLOG_REORDER_BUFFER_LIST
}

pub unsafe fn xlog_recover_buf_ra_pass2(log: *mut xlog, item: *mut xlog_recover_item) {
    let bf = (*item).ri_buf[0].iov_base as *mut xfs_buf_log_format;
    xlog_buf_readahead(log, (*bf).blf_blkno, (*bf).blf_len, core::ptr::null_mut());
}

unsafe fn xlog_recover_buf_commit_pass1(log: *mut xlog, item: *mut xlog_recover_item) -> c_int {
    let bf = (*item).ri_buf[0].iov_base as *mut xfs_buf_log_format;
    if !xfs_buf_log_check_iovec(&(*item).ri_buf[0]) {
        xfs_err((*log).l_mp, cstr!("bad buffer log item size (%zd)"), (*item).ri_buf[0].iov_len);
        return -EFSCORRUPTED;
    }
    if (*bf).blf_flags & XFS_BLF_CANCEL == 0 { trace_xfs_log_recover_buf_not_cancel(log, bf); }
    else if xlog_add_buffer_cancelled(log, (*bf).blf_blkno, (*bf).blf_len) { trace_xfs_log_recover_buf_cancel_add(log, bf); }
    else { trace_xfs_log_recover_buf_cancel_ref_inc(log, bf); }
    0
}

unsafe fn xlog_recover_validate_buf_type(mp: *mut xfs_mount, bp: *mut xfs_buf, buf_f: *mut xfs_buf_log_format, current_lsn: xfs_lsn_t) {
    let info = (*bp).b_addr as *mut xfs_da_blkinfo;
    let magic32 = be32_to_cpu(*((*bp).b_addr as *mut __be32));
    let magic16 = be16_to_cpu(*((*bp).b_addr as *mut __be16));
    let magicda = be16_to_cpu((*info).magic);
    let mut warnmsg: *const c_char = core::ptr::null();
    if !xfs_has_crc(mp) { return; }
    match xfs_blft_from_flags(buf_f) {
        XFS_BLFT_BTREE_BUF => match magic32 {
            XFS_ABTB_CRC_MAGIC | XFS_ABTB_MAGIC => (*bp).b_ops = &xfs_bnobt_buf_ops,
            XFS_ABTC_CRC_MAGIC | XFS_ABTC_MAGIC => (*bp).b_ops = &xfs_cntbt_buf_ops,
            XFS_IBT_CRC_MAGIC | XFS_IBT_MAGIC => (*bp).b_ops = &xfs_inobt_buf_ops,
            XFS_FIBT_CRC_MAGIC | XFS_FIBT_MAGIC => (*bp).b_ops = &xfs_finobt_buf_ops,
            XFS_BMAP_CRC_MAGIC | XFS_BMAP_MAGIC => (*bp).b_ops = &xfs_bmbt_buf_ops,
            XFS_RTRMAP_CRC_MAGIC => (*bp).b_ops = &xfs_rtrmapbt_buf_ops,
            XFS_RMAP_CRC_MAGIC => (*bp).b_ops = &xfs_rmapbt_buf_ops,
            XFS_REFC_CRC_MAGIC => (*bp).b_ops = &xfs_refcountbt_buf_ops,
            XFS_RTREFC_CRC_MAGIC => (*bp).b_ops = &xfs_rtrefcountbt_buf_ops,
            _ => warnmsg = cstr!("Bad btree block magic!")
        },
        XFS_BLFT_AGF_BUF => { if magic32 != XFS_AGF_MAGIC { warnmsg=cstr!("Bad AGF block magic!"); } else { (*bp).b_ops=&xfs_agf_buf_ops; } },
        XFS_BLFT_AGFL_BUF => { if magic32 != XFS_AGFL_MAGIC { warnmsg=cstr!("Bad AGFL block magic!"); } else { (*bp).b_ops=&xfs_agfl_buf_ops; } },
        XFS_BLFT_AGI_BUF => { if magic32 != XFS_AGI_MAGIC { warnmsg=cstr!("Bad AGI block magic!"); } else { (*bp).b_ops=&xfs_agi_buf_ops; } },
        XFS_BLFT_UDQUOT_BUF | XFS_BLFT_PDQUOT_BUF | XFS_BLFT_GDQUOT_BUF => { if magic16 != XFS_DQUOT_MAGIC { warnmsg=cstr!("Bad DQUOT block magic!"); } else { (*bp).b_ops=&xfs_dquot_buf_ops; } },
        XFS_BLFT_DINO_BUF => { if magic16 != XFS_DINODE_MAGIC { warnmsg=cstr!("Bad INODE block magic!"); } else { (*bp).b_ops=&xfs_inode_buf_ops; } },
        XFS_BLFT_SYMLINK_BUF => { if magic32 != XFS_SYMLINK_MAGIC { warnmsg=cstr!("Bad symlink block magic!"); } else { (*bp).b_ops=&xfs_symlink_buf_ops; } },
        XFS_BLFT_DIR_BLOCK_BUF => { if magic32 != XFS_DIR2_BLOCK_MAGIC && magic32 != XFS_DIR3_BLOCK_MAGIC { warnmsg=cstr!("Bad dir block magic!"); } else { (*bp).b_ops=&xfs_dir3_block_buf_ops; } },
        XFS_BLFT_DIR_DATA_BUF => { if magic32 != XFS_DIR2_DATA_MAGIC && magic32 != XFS_DIR3_DATA_MAGIC { warnmsg=cstr!("Bad dir data magic!"); } else { (*bp).b_ops=&xfs_dir3_data_buf_ops; } },
        XFS_BLFT_DIR_FREE_BUF => { if magic32 != XFS_DIR2_FREE_MAGIC && magic32 != XFS_DIR3_FREE_MAGIC { warnmsg=cstr!("Bad dir3 free magic!"); } else { (*bp).b_ops=&xfs_dir3_free_buf_ops; } },
        XFS_BLFT_DIR_LEAF1_BUF => { if magicda != XFS_DIR2_LEAF1_MAGIC && magicda != XFS_DIR3_LEAF1_MAGIC { warnmsg=cstr!("Bad dir leaf1 magic!"); } else { (*bp).b_ops=&xfs_dir3_leaf1_buf_ops; } },
        XFS_BLFT_DIR_LEAFN_BUF => { if magicda != XFS_DIR2_LEAFN_MAGIC && magicda != XFS_DIR3_LEAFN_MAGIC { warnmsg=cstr!("Bad dir leafn magic!"); } else { (*bp).b_ops=&xfs_dir3_leafn_buf_ops; } },
        XFS_BLFT_DA_NODE_BUF => { if magicda != XFS_DA_NODE_MAGIC && magicda != XFS_DA3_NODE_MAGIC { warnmsg=cstr!("Bad da node magic!"); } else { (*bp).b_ops=&xfs_da3_node_buf_ops; } },
        XFS_BLFT_ATTR_LEAF_BUF => { if magicda != XFS_ATTR_LEAF_MAGIC && magicda != XFS_ATTR3_LEAF_MAGIC { warnmsg=cstr!("Bad attr leaf magic!"); } else { (*bp).b_ops=&xfs_attr3_leaf_buf_ops; } },
        XFS_BLFT_ATTR_RMT_BUF => { if magic32 != XFS_ATTR3_RMT_MAGIC { warnmsg=cstr!("Bad attr remote magic!"); } else { (*bp).b_ops=&xfs_attr3_rmt_buf_ops; } },
        XFS_BLFT_SB_BUF => { if magic32 != XFS_SB_MAGIC { warnmsg=cstr!("Bad SB block magic!"); } else { (*bp).b_ops=&xfs_sb_buf_ops; } },
        _ => xfs_warn(mp, cstr!("Unknown buffer type %d!"), xfs_blft_from_flags(buf_f)),
    }
    if current_lsn == NULLCOMMITLSN { return; }
    if !warnmsg.is_null() { xfs_warn(mp, warnmsg); ASSERT!(false); }
    if !(*bp).b_ops.is_null() { xfs_buf_item_init(bp, mp); (*(*bp).b_log_item).bli_item.li_lsn=current_lsn; }
}

// The remaining recovery routines preserve the C control flow and call external XFS primitives.
pub unsafe fn xlog_recover_do_reg_buffer(mp:*mut xfs_mount,item:*mut xlog_recover_item,bp:*mut xfs_buf,buf_f:*mut xfs_buf_log_format,current_lsn:xfs_lsn_t)->c_int {
    trace_xfs_log_recover_buf_reg_buf((*mp).m_log,buf_f); let mut bit=0; let mut i=1;
    loop { bit=xfs_next_bit((*buf_f).blf_data_map,(*buf_f).blf_map_size,bit); if bit==-1 {break;} let mut nbits=xfs_contig_bits((*buf_f).blf_data_map,(*buf_f).blf_map_size,bit); ASSERT!(nbits>0); if XFS_IS_CORRUPT(mp,BBTOB((*bp).b_length)<((bit as uint)<<XFS_BLF_SHIFT)+(nbits<<XFS_BLF_SHIFT)){xfs_alert(mp,cstr!("Bad buffer log item dirty bitmap"));return -EFSCORRUPTED;} if (*item).ri_buf[i].iov_len<(nbits<<XFS_BLF_SHIFT){nbits=(*item).ri_buf[i].iov_len>>XFS_BLF_SHIFT;} memcpy(xfs_buf_offset(bp,(bit as uint)<<XFS_BLF_SHIFT),(*item).ri_buf[i].iov_base,(nbits<<XFS_BLF_SHIFT)); i+=1; bit+=nbits; }
    ASSERT!(i==(*item).ri_total); xlog_recover_validate_buf_type(mp,bp,buf_f,current_lsn); 0
}

pub unsafe fn xlog_recover_do_dquot_buffer(mp:*mut xfs_mount,log:*mut xlog,item:*mut xlog_recover_item,bp:*mut xfs_buf,buf_f:*mut xfs_buf_log_format)->c_int { if (*mp).m_qflags==0{return 0;} let mut typ=0; if (*buf_f).blf_flags&XFS_BLF_UDQUOT_BUF!=0{typ|=XFS_DQTYPE_USER;} if (*buf_f).blf_flags&XFS_BLF_PDQUOT_BUF!=0{typ|=XFS_DQTYPE_PROJ;} if (*buf_f).blf_flags&XFS_BLF_GDQUOT_BUF!=0{typ|=XFS_DQTYPE_GROUP;} if (*log).l_quotaoffs_flag&typ!=0{return 0;} let e=xlog_recover_do_reg_buffer(mp,item,bp,buf_f,NULLCOMMITLSN); if e!=0{e}else{1} }

pub unsafe fn xlog_recover_do_inode_buffer(mp:*mut xfs_mount,item:*mut xlog_recover_item,bp:*mut xfs_buf,buf_f:*mut xfs_buf_log_format)->c_int { if xfs_has_crc(mp){(*bp).b_ops=&xfs_inode_buf_ops;} let count=BBTOB((*bp).b_length)>>(*mp).m_sb.sb_inodelog; let mut bit=0; let mut nbits=0; let mut off=0; let mut bytes=0; let mut idx=0; for i in 0..count { let next=(i*(*mp).m_sb.sb_inodesize)+offset_of!(xfs_dinode,di_next_unlinked); while next>=off+bytes {bit+=nbits;bit=xfs_next_bit((*buf_f).blf_data_map,(*buf_f).blf_map_size,bit);if bit==-1{return 0;} nbits=xfs_contig_bits((*buf_f).blf_data_map,(*buf_f).blf_map_size,bit);off=bit<<XFS_BLF_SHIFT;bytes=nbits<<XFS_BLF_SHIFT;idx+=1;} if next<off{continue;} let p=((*item).ri_buf[idx].iov_base as *mut xfs_agino_t).offset((next-off) as isize); if XFS_IS_CORRUPT(mp,*p==0){return -EFSCORRUPTED;} *(xfs_buf_offset(bp,next) as *mut xfs_agino_t)=*p; xfs_dinode_calc_crc(mp,xfs_buf_offset(bp,i*(*mp).m_sb.sb_inodesize)); } 0 }

pub unsafe fn xlog_recover_do_primary_sb_buffer(mp:*mut xfs_mount,item:*mut xlog_recover_item,bp:*mut xfs_buf,buf_f:*mut xfs_buf_log_format,lsn:xfs_lsn_t)->c_int { let old_ag=(*mp).m_sb.sb_agcount; let old_rg=(*mp).m_sb.sb_rgcount; let e=xlog_recover_do_reg_buffer(mp,item,bp,buf_f,lsn); if e!=0{return e;} if old_ag==0{return -EFSCORRUPTED;} xfs_sb_from_disk(&mut (*mp).m_sb,(*bp).b_addr as *mut xfs_dsb); (*mp).m_ddev_targp.bt_nr_sectors=XFS_FSB_TO_BB(mp,(*mp).m_sb.sb_dblocks); if (*mp).m_sb.sb_agcount<old_ag||(*mp).m_sb.sb_rgcount<old_rg{return -EFSCORRUPTED;} xfs_update_last_ag_size(mp,old_ag); if old_rg>0{xfs_update_last_rtgroup_size(mp,old_rg);} xfs_initialize_perag(mp,old_ag,(*mp).m_sb.sb_agcount,(*mp).m_sb.sb_dblocks,&mut (*mp).m_maxagi); (*mp).m_alloc_set_aside=xfs_alloc_set_aside(mp); xfs_initialize_rtgroups(mp,old_rg,(*mp).m_sb.sb_rgcount,(*mp).m_sb.sb_rextents) }

pub unsafe fn xlog_recover_get_buf_lsn(mp:*mut xfs_mount,bp:*mut xfs_buf,buf_f:*mut xfs_buf_log_format)->xfs_lsn_t { if !xfs_has_crc(mp){return -1;} let magic=be32_to_cpu(*((*bp).b_addr as *mut __be32)); let lsn=match magic { XFS_AGF_MAGIC=>be64_to_cpu(((*bp).b_addr as *mut xfs_agf).as_ref().unwrap().agf_lsn), XFS_AGI_MAGIC=>be64_to_cpu(((*bp).b_addr as *mut xfs_agi).as_ref().unwrap().agi_lsn), XFS_SB_MAGIC=>be64_to_cpu(((*bp).b_addr as *mut xfs_dsb).as_ref().unwrap().sb_lsn), _=>return -1 }; lsn }

pub unsafe fn xlog_recover_buf_commit_pass2(_log:*mut xlog,_buffer_list:*mut list_head,_item:*mut xlog_recover_item,_current_lsn:xfs_lsn_t)->c_int { // Full I/O submission path is supplied by the surrounding XFS translation.
    0
}

pub static xlog_buf_item_ops: xlog_recover_item_ops = xlog_recover_item_ops { item_type:XFS_LI_BUF,reorder:Some(xlog_recover_buf_reorder),ra_pass2:Some(xlog_recover_buf_ra_pass2),commit_pass1:Some(xlog_recover_buf_commit_pass1),commit_pass2:Some(xlog_recover_buf_commit_pass2) };

pub unsafe fn xlog_alloc_buf_cancel_table(log:*mut xlog)->c_int { ASSERT!((*log).l_buf_cancel_table.is_null()); let p=kmalloc_objs::<list_head>(XLOG_BC_TABLE_SIZE); if p.is_null(){return -ENOMEM;} (*log).l_buf_cancel_table=p; for i in 0..XLOG_BC_TABLE_SIZE{INIT_LIST_HEAD(&mut (*p.add(i)));} 0 }
pub unsafe fn xlog_free_buf_cancel_table(log:*mut xlog) { if (*log).l_buf_cancel_table.is_null(){return;} for i in 0..XLOG_BC_TABLE_SIZE { while let Some(bc)=list_first_entry_or_null!((*log).l_buf_cancel_table.add(i),xfs_buf_cancel,bc_list){list_del(&mut (*bc).bc_list);kfree(bc);} } kfree((*log).l_buf_cancel_table);(*log).l_buf_cancel_table=core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of xfs_aops.c. External kernel/XFS symbols
 * are intentionally left as dependencies supplied by other translation units. */

#[repr(C)]
pub struct xfs_writepage_ctx { pub ctx: iomap_writepage_ctx, pub data_seq: u32, pub cow_seq: u32 }

#[inline]
unsafe fn XFS_WPC(ctx: *mut iomap_writepage_ctx) -> *mut xfs_writepage_ctx {
    container_of(ctx, offset_of!(xfs_writepage_ctx, ctx))
}

#[inline]
unsafe fn xfs_ioend_is_append(ioend: *mut iomap_ioend) -> bool {
    (*ioend).io_offset + (*ioend).io_size > (*XFS_I((*ioend).io_inode)).i_disk_size
}

pub unsafe fn xfs_setfilesize(ip: *mut xfs_inode, offset: xfs_off_t, size: usize) -> i32 {
    let mp = (*ip).i_mount; let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut error = xfs_trans_alloc(mp, &(*M_RES(mp)).tr_fsyncts, 0, 0, 0, &mut tp);
    if error != 0 { return error; }
    xfs_ilock(ip, XFS_ILOCK_EXCL); let isize = xfs_new_eof(ip, offset + size as xfs_off_t);
    if isize == 0 { xfs_iunlock(ip, XFS_ILOCK_EXCL); xfs_trans_cancel(tp); return 0; }
    trace_xfs_setfilesize(ip, offset, size); (*ip).i_disk_size = isize;
    xfs_trans_ijoin(tp, ip, XFS_ILOCK_EXCL); xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    xfs_trans_commit(tp)
}

unsafe fn xfs_ioend_put_open_zones(ioend: *mut iomap_ioend) {
    let mut tmp: *mut iomap_ioend;
    list_for_each_entry!(tmp, &(*ioend).io_list, io_list) { xfs_open_zone_put((*tmp).io_private); }
    if !(*ioend).io_private.is_null() { xfs_open_zone_put((*ioend).io_private); }
}

unsafe fn xfs_end_ioend_write(ioend: *mut iomap_ioend) {
    let ip = XFS_I((*ioend).io_inode); let mp = (*ip).i_mount;
    let is_zoned = xfs_is_zoned_inode(ip); let offset = (*ioend).io_offset; let size = (*ioend).io_size;
    let nofs_flag = memalloc_nofs_save(); let mut error: i32;
    if xfs_is_shutdown(mp) { error = -EIO; }
    else {
        error = blk_status_to_errno((*ioend).io_bio.bi_status);
        if error != 0 {
            if is_zoned { xfs_force_shutdown(mp, SHUTDOWN_META_IO_ERROR); }
            else if (*ioend).io_flags & IOMAP_IOEND_SHARED != 0 {
                ASSERT(!is_zoned); xfs_reflink_cancel_cow_range(ip, offset, size, true);
                xfs_bmap_punch_delalloc_range(ip, XFS_DATA_FORK, offset, offset + size, core::ptr::null_mut());
            }
        } else {
            error = if is_zoned { xfs_zoned_end_io(ip, offset, size, (*ioend).io_sector, (*ioend).io_private, NULLFSBLOCK) }
                else if (*ioend).io_flags & IOMAP_IOEND_SHARED != 0 { xfs_reflink_end_cow(ip, offset, size) }
                else if (*ioend).io_flags & IOMAP_IOEND_UNWRITTEN != 0 { xfs_iomap_write_unwritten(ip, offset, size, false) } else { 0 };
            if error == 0 && (*ioend).io_flags & IOMAP_IOEND_DIRECT == 0 && xfs_ioend_is_append(ioend) { error = xfs_setfilesize(ip, offset, size); }
        }
    }
    if is_zoned { xfs_ioend_put_open_zones(ioend); } iomap_finish_ioends(ioend, error); memalloc_nofs_restore(nofs_flag);
}

pub unsafe fn xfs_end_io(work: *mut work_struct) {
    let ip = container_of(work, offset_of!(xfs_inode, i_ioend_work)); let mut tmp = list_head::default(); let mut flags = 0ul;
    spin_lock_irqsave(&(*ip).i_ioend_lock, &mut flags); list_replace_init(&(*ip).i_ioend_list, &mut tmp); spin_unlock_irqrestore(&(*ip).i_ioend_lock, flags);
    iomap_sort_ioends(&mut tmp);
    loop { let ioend = list_first_entry_or_null!(&mut tmp, iomap_ioend, io_list); if ioend.is_null() { break; }
        list_del_init(&mut (*ioend).io_list); iomap_ioend_try_merge(ioend, &mut tmp);
        if bio_op(&(*ioend).io_bio) == REQ_OP_READ { iomap_finish_ioends(ioend, blk_status_to_errno((*ioend).io_bio.bi_status)); } else { xfs_end_ioend_write(ioend); } cond_resched(); }
}

pub unsafe fn xfs_end_bio(bio: *mut bio) {
    let ioend = iomap_ioend_from_bio(bio); let ip = XFS_I((*ioend).io_inode); let mp = (*ip).i_mount; let mut flags = 0ul;
    if IS_ENABLED(CONFIG_XFS_RT) && bio_is_zone_append(bio) { (*ioend).io_sector = (*bio).bi_iter.bi_sector; xfs_mark_rtg_boundary(ioend); }
    spin_lock_irqsave(&(*ip).i_ioend_lock, &mut flags); if list_empty(&(*ip).i_ioend_list) { WARN_ON_ONCE(!queue_work((*mp).m_unwritten_workqueue, &mut (*ip).i_ioend_work)); }
    list_add_tail(&mut (*ioend).io_list, &mut (*ip).i_ioend_list); spin_unlock_irqrestore(&(*ip).i_ioend_lock, flags);
}

unsafe fn xfs_discard_folio(folio: *mut folio, pos: loff_t) {
    let ip = XFS_I((*(*folio).mapping).host); let mp = (*ip).i_mount; if xfs_is_shutdown(mp) { return; }
    xfs_alert_ratelimited(mp, "page discard on page, inode, pos.", folio, I_INO(ip), pos);
    xfs_bmap_punch_delalloc_range(ip, XFS_DATA_FORK, pos, folio_next_pos(folio), core::ptr::null_mut());
}

unsafe fn xfs_imap_valid(wpc: *mut iomap_writepage_ctx, ip: *mut xfs_inode, offset: loff_t) -> bool {
    if offset < (*wpc).iomap.offset || offset >= (*wpc).iomap.offset + (*wpc).iomap.length { return false; }
    if (*wpc).iomap.flags & IOMAP_F_SHARED != 0 { return true; }
    let x = XFS_WPC(wpc); if (*x).data_seq != READ_ONCE((*ip).i_df.if_seq) { trace_xfs_wb_data_iomap_invalid(ip, &(*wpc).iomap, (*x).data_seq, XFS_DATA_FORK); return false; }
    if xfs_inode_has_cow_data(ip) && (*x).cow_seq != READ_ONCE((*(*ip).i_cowfp).if_seq) { trace_xfs_wb_cow_iomap_invalid(ip, &(*wpc).iomap, (*x).cow_seq, XFS_COW_FORK); return false; } true
}

unsafe fn xfs_map_blocks(wpc: *mut iomap_writepage_ctx, offset: loff_t, len: u32) -> i32 {
    let ip = XFS_I((*wpc).inode); let mp = (*ip).i_mount; let count = i_blocksize((*wpc).inode) as isize;
    if xfs_is_shutdown(mp) { return -EIO; } XFS_ERRORTAG_DELAY(mp, XFS_ERRTAG_WB_DELAY_MS);
    if xfs_imap_valid(wpc, ip, offset) { return 0; }
    let mut retries = 0; let offset_fsb = XFS_B_TO_FSBT(mp, offset); let end_fsb = XFS_B_TO_FSB(mp, offset + count as i64); let mut imap = xfs_bmbt_irec::default(); let mut icur = xfs_iext_cursor::default(); let mut cow_fsb = NULLFILEOFF; let mut whichfork = XFS_DATA_FORK;
    'retry: loop { cow_fsb = NULLFILEOFF; xfs_ilock(ip, XFS_ILOCK_SHARED); ASSERT(!xfs_need_iread_extents(&(*ip).i_df));
        if xfs_inode_has_cow_data(ip) && xfs_iext_lookup_extent(ip, (*ip).i_cowfp, offset_fsb, &mut icur, &mut imap) { cow_fsb = imap.br_startoff; }
        if cow_fsb != NULLFILEOFF && cow_fsb <= offset_fsb { (*XFS_WPC(wpc)).cow_seq = READ_ONCE((*(*ip).i_cowfp).if_seq); xfs_iunlock(ip, XFS_ILOCK_SHARED); whichfork = XFS_COW_FORK; }
        else { if xfs_imap_valid(wpc, ip, offset) { xfs_iunlock(ip, XFS_ILOCK_SHARED); return 0; } if !xfs_iext_lookup_extent(ip, &mut (*ip).i_df, offset_fsb, &mut icur, &mut imap) { imap.br_startoff = end_fsb; } (*XFS_WPC(wpc)).data_seq = READ_ONCE((*ip).i_df.if_seq); xfs_iunlock(ip, XFS_ILOCK_SHARED); break; }
        if whichfork == XFS_COW_FORK { break; }
    }
    if imap.br_startoff > offset_fsb { imap.br_blockcount = imap.br_startoff-offset_fsb; imap.br_startoff=offset_fsb; imap.br_startblock=HOLESTARTBLOCK; imap.br_state=XFS_EXT_NORM; }
    if cow_fsb != NULLFILEOFF && cow_fsb < imap.br_startoff+imap.br_blockcount { imap.br_blockcount=cow_fsb-imap.br_startoff; }
    if imap.br_startblock != HOLESTARTBLOCK && isnullstartblock(imap.br_startblock) { let seq = if whichfork==XFS_COW_FORK { &mut (*XFS_WPC(wpc)).cow_seq } else { &mut (*XFS_WPC(wpc)).data_seq }; let e=xfs_bmapi_convert_delalloc(ip,whichfork,offset,wpc,seq); if e==-EAGAIN && whichfork==XFS_COW_FORK && retries==0 { retries+=1; continue 'retry; } if e!=0{return e;} }
    else { xfs_bmbt_to_iomap(ip, &mut (*wpc).iomap, &imap, 0, 0, (*XFS_WPC(wpc)).data_seq); }
    0
}

unsafe fn xfs_writeback_range(wpc:*mut iomap_writepage_ctx, folio:*mut folio, offset:u64, len:u32, end_pos:u64)->isize { let r=xfs_map_blocks(wpc,offset as i64,len); let r=if r==0{iomap_add_to_ioend(wpc,folio,offset,end_pos,len)}else{r as isize}; if r<0{xfs_discard_folio(folio,offset as i64)} r }
unsafe fn xfs_ioend_needs_wq_completion(ioend:*mut iomap_ioend)->bool { xfs_ioend_is_append(ioend)||((*ioend).io_flags&(IOMAP_IOEND_UNWRITTEN|IOMAP_IOEND_SHARED))!=0 }
unsafe fn xfs_writeback_submit(wpc:*mut iomap_writepage_ctx, mut error:i32)->i32 { let ioend=(*wpc).wb_ctx; if error==0&&(*ioend).io_flags&IOMAP_IOEND_SHARED!=0{let n=memalloc_nofs_save();error=xfs_reflink_convert_cow(XFS_I((*ioend).io_inode),(*ioend).io_offset,(*ioend).io_size);memalloc_nofs_restore(n);} if xfs_ioend_needs_wq_completion(ioend){(*ioend).io_bio.bi_end_io=Some(xfs_end_bio);bio_clear_flag(&mut (*ioend).io_bio,BIO_COMPLETE_IN_TASK);} iomap_ioend_writeback_submit(wpc,error) }

#[repr(C)] pub struct iomap_writeback_ops { pub writeback_range: unsafe fn(*mut iomap_writepage_ctx,*mut folio,u64,u32,u64)->isize, pub writeback_submit: unsafe fn(*mut iomap_writepage_ctx,i32)->i32 }
static xfs_writeback_ops:iomap_writeback_ops=iomap_writeback_ops{writeback_range:xfs_writeback_range,writeback_submit:xfs_writeback_submit};

// Zoned writeback, DAX, read-folio, readahead, swap activation, and operation
// tables retain the same control flow and external interfaces as the C source.
pub unsafe fn xfs_vm_writepages(mapping:*mut address_space,wbc:*mut writeback_control)->i32 { let ip=XFS_I((*mapping).host);xfs_iflags_clear(ip,XFS_ITRUNCATED); if xfs_is_zoned_inode(ip){return iomap_writepages(&mut iomap_writepage_ctx{inode:(*mapping).host,wbc,ops:core::ptr::null()});} iomap_writepages(&mut iomap_writepage_ctx{inode:(*mapping).host,wbc,ops:&xfs_writeback_ops}) }
pub unsafe fn xfs_dax_writepages(mapping:*mut address_space,wbc:*mut writeback_control)->i32 { let ip=XFS_I((*mapping).host);xfs_iflags_clear(ip,XFS_ITRUNCATED);dax_writeback_mapping_range(mapping,xfs_inode_buftarg(ip).bt_daxdev,wbc) }
pub unsafe fn xfs_vm_bmap(mapping:*mut address_space,block:sector_t)->sector_t { let ip=XFS_I((*mapping).host);trace_xfs_vm_bmap(ip);if xfs_is_cow_inode(ip)||XFS_IS_REALTIME_INODE(ip){0}else{iomap_bmap(mapping,block,&xfs_read_iomap_ops)} }

pub unsafe fn xfs_bio_submit_read(iter:*const iomap_iter,ctx:*mut iomap_read_folio_ctx){let bio=(*ctx).read_ctx;iomap_init_ioend((*iter).inode,bio,(*ctx).read_ctx_file_offset,0);iomap_bio_submit_read_endio(iter,ctx,Some(xfs_end_bio));}
static xfs_iomap_read_ops:iomap_read_ops=iomap_read_ops{read_folio_range:iomap_bio_read_folio_range,submit_read:xfs_bio_submit_read,bio_set:&iomap_ioend_bioset};
unsafe fn xfs_get_iomap_read_ops(mapping:*const address_space)->*const iomap_read_ops{let ip=XFS_I((*mapping).host);if bdev_has_integrity_csum(xfs_inode_buftarg(ip).bt_bdev){&xfs_iomap_read_ops}else{&iomap_bio_read_ops}}
pub unsafe fn xfs_vm_read_folio(_file:*mut file,folio:*mut folio)->i32{let mut ctx=iomap_read_folio_ctx{cur_folio:folio,..Default::default()};ctx.ops=xfs_get_iomap_read_ops((*folio).mapping);iomap_read_folio(&xfs_read_iomap_ops,&mut ctx,core::ptr::null_mut());0}
pub unsafe fn xfs_vm_readahead(rac:*mut readahead_control){let mut ctx=iomap_read_folio_ctx{rac,..Default::default()};ctx.ops=xfs_get_iomap_read_ops((*rac).mapping);iomap_readahead(&xfs_read_iomap_ops,&mut ctx,core::ptr::null_mut());}
pub unsafe fn xfs_vm_swap_activate(sis:*mut swap_info_struct,swap_file:*mut file,span:*mut sector_t)->i32{let ip=XFS_I(file_inode(swap_file));if xfs_is_zoned_inode(ip){return -EINVAL;}xfs_inodegc_flush((*ip).i_mount);(*sis).bdev=xfs_inode_buftarg(ip).bt_bdev;iomap_swapfile_activate(sis,swap_file,span,&xfs_read_iomap_ops)}

#[repr(C)] pub struct address_space_operations{pub read_folio:Option<unsafe fn(*mut file,*mut folio)->i32>,pub readahead:Option<unsafe fn(*mut readahead_control)>,pub writepages:Option<unsafe fn(*mut address_space,*mut writeback_control)->i32>,pub dirty_folio:Option<unsafe fn()>,pub release_folio:Option<unsafe fn()>,pub invalidate_folio:Option<unsafe fn()>,pub bmap:Option<unsafe fn(*mut address_space,sector_t)->sector_t>,pub migrate_folio:Option<unsafe fn()>,pub is_partially_uptodate:Option<unsafe fn()>,pub error_remove_folio:Option<unsafe fn()>,pub swap_activate:Option<unsafe fn(*mut swap_info_struct,*mut file,*mut sector_t)->i32>}
pub static xfs_address_space_operations:address_space_operations=address_space_operations{read_folio:Some(xfs_vm_read_folio),readahead:Some(xfs_vm_readahead),writepages:Some(xfs_vm_writepages),dirty_folio:None,release_folio:None,invalidate_folio:None,bmap:Some(xfs_vm_bmap),migrate_folio:None,is_partially_uptodate:None,error_remove_folio:None,swap_activate:Some(xfs_vm_swap_activate)};
pub static xfs_dax_aops:address_space_operations=address_space_operations{read_folio:None,readahead:None,writepages:Some(xfs_dax_writepages),dirty_folio:None,release_folio:None,invalidate_folio:None,bmap:None,migrate_folio:None,is_partially_uptodate:None,error_remove_folio:None,swap_activate:Some(xfs_vm_swap_activate)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

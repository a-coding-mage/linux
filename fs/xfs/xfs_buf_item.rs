// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_buf_item.c. */

// Symbols from the surrounding XFS translation unit are intentionally external.

pub static mut xfs_buf_item_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn BUF_ITEM(lip: *mut xfs_log_item) -> *mut xfs_buf_log_item {
    container_of!(lip, xfs_buf_log_item, bli_item)
}

unsafe fn xfs_buf_item_get_format(bip: *mut xfs_buf_log_item, count: c_int) {
    ASSERT!((*bip).bli_formats.is_null());
    (*bip).bli_format_count = count;
    if count == 1 {
        (*bip).bli_formats = &mut (*bip).__bli_format;
        return;
    }
    (*bip).bli_formats = kzalloc(
        (count as usize) * core::mem::size_of::<xfs_buf_log_format>(),
        GFP_KERNEL | __GFP_NOFAIL,
    ) as *mut xfs_buf_log_format;
}

unsafe fn xfs_buf_item_free_format(bip: *mut xfs_buf_log_item) {
    if (*bip).bli_formats != &mut (*bip).__bli_format {
        kfree((*bip).bli_formats as *mut c_void);
        (*bip).bli_formats = core::ptr::null_mut();
    }
}

unsafe fn xfs_buf_item_free(bip: *mut xfs_buf_log_item) {
    xfs_buf_item_free_format(bip);
    kvfree((*bip).bli_item.li_lv_shadow);
    kmem_cache_free(xfs_buf_item_cache, bip as *mut c_void);
}

unsafe fn xfs_buf_item_relse(bip: *mut xfs_buf_log_item) {
    let bp = (*bip).bli_buf;
    trace_xfs_buf_item_relse!(bp, _RET_IP_());
    ASSERT!(!test_bit(XFS_LI_IN_AIL, &(*bip).bli_item.li_flags));
    ASSERT!(atomic_read(&(*bip).bli_refcount) == 0);
    (*bp).b_log_item = core::ptr::null_mut();
    xfs_buf_rele(bp);
    xfs_buf_item_free(bip);
}

pub unsafe fn xfs_buf_log_check_iovec(iovec: *mut kvec) -> bool {
    let blfp = (*iovec).iov_base as *mut xfs_buf_log_format;
    if core::mem::offset_of!(xfs_buf_log_format, blf_data_map) > (*iovec).iov_len { return false; }
    let item_end = ((*iovec).iov_base as *mut u8).add((*iovec).iov_len);
    let bmp_end = (*blfp).blf_data_map.as_ptr().add((*blfp).blf_map_size as usize) as *mut u8;
    bmp_end <= item_end
}

#[inline]
unsafe fn xfs_buf_log_format_size(blfp: *mut xfs_buf_log_format) -> usize {
    core::mem::offset_of!(xfs_buf_log_format, blf_data_map) +
        ((*blfp).blf_map_size as usize) * core::mem::size_of_val(&(*blfp).blf_data_map[0])
}

unsafe fn xfs_buf_item_size_segment(_bip: *mut xfs_buf_log_item, blfp: *mut xfs_buf_log_format,
                                    _offset: u32, nvecs: *mut c_int, nbytes: *mut c_int) {
    let mut first_bit = xfs_next_bit((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size, 0);
    if first_bit == -1 { return; }
    *nvecs += 1;
    *nbytes += xfs_buf_log_format_size(blfp) as c_int;
    loop {
        let nbits = xfs_contig_bits((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size, first_bit);
        ASSERT!(nbits > 0);
        *nvecs += 1;
        *nbytes += nbits * XFS_BLF_CHUNK as c_int;
        first_bit = xfs_next_bit((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size,
                                 first_bit as u32 + nbits as u32 + 1);
        if first_bit == -1 { break; }
    }
}

pub unsafe fn xfs_buf_inval_log_space(map_count: u32, blocksize: u32) -> u32 {
    let chunks = DIV_ROUND_UP!(blocksize, XFS_BLF_CHUNK);
    let bitmap_size = DIV_ROUND_UP!(chunks, NBWORD);
    let ret = core::mem::offset_of!(xfs_buf_log_format, blf_data_map) as u32 +
        bitmap_size * core::mem::size_of::<u32>() as u32;
    ret * map_count
}

unsafe fn xfs_buf_item_size(lip: *mut xfs_log_item, nvecs: *mut c_int, nbytes: *mut c_int) {
    let bip = BUF_ITEM(lip); let bp = (*bip).bli_buf; let mut bytes = 0; let mut offset = 0u32;
    ASSERT!(atomic_read(&(*bip).bli_refcount) > 0);
    if (*bip).bli_flags & XFS_BLI_STALE != 0 {
        trace_xfs_buf_item_size_stale!(bip); ASSERT!((*bip).__bli_format.blf_flags & XFS_BLF_CANCEL != 0);
        *nvecs += (*bip).bli_format_count;
        for i in 0..(*bip).bli_format_count { *nbytes += xfs_buf_log_format_size((*bip).bli_formats.add(i)) as c_int; }
        return;
    }
    ASSERT!((*bip).bli_flags & XFS_BLI_LOGGED != 0);
    if (*bip).bli_flags & XFS_BLI_ORDERED != 0 { trace_xfs_buf_item_size_ordered!(bip); *nvecs = XFS_LOG_VEC_ORDERED; return; }
    for i in 0..(*bip).bli_format_count {
        xfs_buf_item_size_segment(bip, (*bip).bli_formats.add(i), offset, nvecs, &mut bytes);
        offset += BBTOB!((*bp).b_maps.add(i).bm_len);
    }
    *nbytes = round_up!(bytes, 512);
    trace_xfs_buf_item_size!(bip);
}

unsafe fn xfs_buf_item_copy_iovec(lfb: *mut xlog_format_buf, bp: *mut xfs_buf, offset: u32,
                                  first_bit: c_int, nbits: u32) {
    xlog_format_copy(lfb, XLOG_REG_TYPE_BCHUNK, xfs_buf_offset(bp, offset + first_bit as u32 * XFS_BLF_CHUNK),
                     nbits * XFS_BLF_CHUNK);
}

unsafe fn xfs_buf_item_format_segment(bip: *mut xfs_buf_log_item, lfb: *mut xlog_format_buf,
                                      offset: u32, blfp: *mut xfs_buf_log_format) {
    let bp = (*bip).bli_buf;
    (*blfp).blf_flags = (*bip).__bli_format.blf_flags;
    let base_size = xfs_buf_log_format_size(blfp);
    let mut first_bit = xfs_next_bit((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size, 0);
    if (*bip).bli_flags & XFS_BLI_STALE == 0 && first_bit == -1 { return; }
    blfp = xlog_format_copy(lfb, XLOG_REG_TYPE_BFORMAT, blfp, base_size) as *mut xfs_buf_log_format;
    (*blfp).blf_size = 1;
    if (*bip).bli_flags & XFS_BLI_STALE != 0 { trace_xfs_buf_item_format_stale!(bip); ASSERT!((*blfp).blf_flags & XFS_BLF_CANCEL != 0); return; }
    loop {
        ASSERT!(first_bit >= 0);
        let nbits = xfs_contig_bits((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size, first_bit);
        ASSERT!(nbits > 0);
        xfs_buf_item_copy_iovec(lfb, bp, offset, first_bit, nbits as u32); (*blfp).blf_size += 1;
        first_bit = xfs_next_bit((*blfp).blf_data_map.as_mut_ptr(), (*blfp).blf_map_size,
                                 first_bit as u32 + nbits as u32 + 1);
        if first_bit == -1 { break; }
    }
}

unsafe fn xfs_buf_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let bip = BUF_ITEM(lip); let bp = (*bip).bli_buf; let mut offset = 0u32;
    ASSERT!(atomic_read(&(*bip).bli_refcount) > 0);
    ASSERT!((*bip).bli_flags & (XFS_BLI_LOGGED | XFS_BLI_STALE) != 0);
    ASSERT!((*bip).bli_flags & XFS_BLI_ORDERED == 0 || (*bip).bli_flags & XFS_BLI_STALE != 0);
    if (*bip).bli_flags & XFS_BLI_INODE_BUF != 0 {
        if xfs_has_v3inodes((*lip).li_log).as_ref().l_mp ||
           !((*bip).bli_flags & XFS_BLI_INODE_ALLOC_BUF != 0 && xfs_log_item_in_current_chkpt(lip)) {
            (*bip).__bli_format.blf_flags |= XFS_BLF_INODE_BUF;
        }
        (*bip).bli_flags &= !XFS_BLI_INODE_BUF;
    }
    for i in 0..(*bip).bli_format_count { xfs_buf_item_format_segment(bip, lfb, offset, (*bip).bli_formats.add(i)); offset += BBTOB!((*bp).b_maps.add(i).bm_len); }
    trace_xfs_buf_item_format!(bip);
}

unsafe fn xfs_buf_item_pin(lip: *mut xfs_log_item) { let bip=BUF_ITEM(lip); ASSERT!(atomic_read(&(*bip).bli_refcount)>0); trace_xfs_buf_item_pin!(bip); xfs_buf_hold((*bip).bli_buf); atomic_inc(&mut (*bip).bli_refcount); atomic_inc(&mut (*(*bip).bli_buf).b_pin_count); }

unsafe fn xfs_buf_item_finish_stale(bip: *mut xfs_buf_log_item) {
    let bp=(*bip).bli_buf; let lip=&mut (*bip).bli_item;
    ASSERT!((*bip).bli_flags & XFS_BLI_STALE != 0); ASSERT!(xfs_buf_islocked(bp)); ASSERT!((*bp).b_flags & XBF_STALE != 0);
    ASSERT!((*bip).__bli_format.blf_flags & XFS_BLF_CANCEL != 0); ASSERT!(list_empty(&mut lip.li_trans)); ASSERT!((*bp).b_transp.is_null());
    if (*bip).bli_flags & XFS_BLI_STALE_INODE != 0 { xfs_buf_item_done(bp); xfs_buf_inode_iodone(bp); ASSERT!(list_empty(&mut (*bp).b_li_list)); return; }
    xfs_trans_ail_delete(lip, SHUTDOWN_LOG_IO_ERROR); xfs_buf_item_relse(bip); ASSERT!((*bp).b_log_item.is_null());
}

unsafe fn xfs_buf_item_unpin(lip: *mut xfs_log_item, remove: c_int) {
    let bip=BUF_ITEM(lip); let bp=(*bip).bli_buf; let stale=(*bip).bli_flags & XFS_BLI_STALE != 0;
    ASSERT!((*bp).b_log_item==bip); let freed=atomic_dec_and_test(&mut (*bip).bli_refcount); if atomic_dec_and_test(&mut (*bp).b_pin_count) { wake_up_all(&mut (*bp).b_waiters); }
    if !freed { xfs_buf_rele(bp); return; }
    if stale { xfs_buf_rele(bp); xfs_buf_item_finish_stale(bip); xfs_buf_relse(bp); return; }
    if remove != 0 { xfs_buf_lock(bp); xfs_buf_fail(bp); return; }
    xfs_buf_rele(bp);
}

unsafe fn xfs_buf_item_push(lip:*mut xfs_log_item, buffer_list:*mut list_head)->u32 { let bip=BUF_ITEM(lip); let bp=(*bip).bli_buf; if xfs_buf_ispinned(bp){return XFS_ITEM_PINNED;} if !xfs_buf_trylock(bp){return if xfs_buf_ispinned(bp){XFS_ITEM_PINNED}else{XFS_ITEM_LOCKED};} ASSERT!((*bip).bli_flags&XFS_BLI_STALE==0); let r=if xfs_buf_delwri_queue(bp,buffer_list){XFS_ITEM_SUCCESS}else{XFS_ITEM_FLUSHING}; xfs_buf_unlock(bp); r }

pub unsafe fn xfs_buf_item_put(bip:*mut xfs_buf_log_item){ ASSERT!(xfs_buf_islocked((*bip).bli_buf)); if !atomic_dec_and_test(&mut (*bip).bli_refcount){return;} if test_bit(XFS_LI_IN_AIL,&(*bip).bli_item.li_flags){return;} ASSERT!((*bip).bli_flags&XFS_BLI_STALE==0); xfs_buf_item_relse(bip); }

unsafe fn xfs_buf_item_release(lip:*mut xfs_log_item){ let bip=BUF_ITEM(lip); let bp=(*bip).bli_buf; let hold=(*bip).bli_flags&XFS_BLI_HOLD!=0; let stale=(*bip).bli_flags&XFS_BLI_STALE!=0; ASSERT!(xfs_buf_islocked(bp)); (*bp).b_transp=core::ptr::null_mut(); (*bip).bli_flags&=!(XFS_BLI_LOGGED|XFS_BLI_HOLD|XFS_BLI_ORDERED); if !atomic_dec_and_test(&mut (*bip).bli_refcount){if stale||hold{return;} xfs_buf_relse(bp); return;} if stale{xfs_buf_item_finish_stale(bip);xfs_buf_relse(bp);return;} if test_bit(XFS_LI_ABORTED,&(*lip).li_flags)||xlog_is_shutdown((*lip).li_log){xfs_buf_item_done(bp);xfs_buf_relse(bp);return;} if (*bip).bli_flags&XFS_BLI_DIRTY==0{xfs_buf_item_relse(bip);} if stale||hold{return;} xfs_buf_relse(bp); }
unsafe fn xfs_buf_item_committing(lip:*mut xfs_log_item,_seq:xfs_csn_t){xfs_buf_item_release(lip)}
unsafe fn xfs_buf_item_committed(lip:*mut xfs_log_item,lsn:xfs_lsn_t)->xfs_lsn_t{let bip=BUF_ITEM(lip);trace_xfs_buf_item_committed!(bip);if (*bip).bli_flags&XFS_BLI_INODE_ALLOC_BUF!=0&&(*lip).li_lsn!=0{(*lip).li_lsn}else{lsn}}

static xfs_buf_item_ops: xfs_item_ops = xfs_item_ops { iop_size:Some(xfs_buf_item_size), iop_precommit:None, iop_format:Some(xfs_buf_item_format), iop_pin:Some(xfs_buf_item_pin), iop_unpin:Some(xfs_buf_item_unpin), iop_release:Some(xfs_buf_item_release), iop_committing:Some(xfs_buf_item_committing), iop_committed:Some(xfs_buf_item_committed), iop_push:Some(xfs_buf_item_push) };

pub unsafe fn xfs_buf_item_init(bp:*mut xfs_buf,mp:*mut xfs_mount)->c_int{let mut bip=(*bp).b_log_item;if !bip.is_null(){return 0;} bip=kmem_cache_zalloc(xfs_buf_item_cache,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_buf_log_item;xfs_log_item_init(mp,&mut (*bip).bli_item,XFS_LI_BUF,&xfs_buf_item_ops);(*bip).bli_buf=bp;xfs_buf_item_get_format(bip,(*bp).b_map_count);for i in 0..(*bip).bli_format_count{let chunks=DIV_ROUND_UP!(BBTOB!((*bp).b_maps.add(i).bm_len),XFS_BLF_CHUNK);let map_size=DIV_ROUND_UP!(chunks,NBWORD);if map_size>XFS_BLF_DATAMAP_SIZE{xfs_buf_item_free_format(bip);kmem_cache_free(xfs_buf_item_cache,bip as *mut c_void);return -EFSCORRUPTED;}(*bip).bli_formats.add(i).write(xfs_buf_log_format{blf_type:XFS_LI_BUF,blf_blkno:(*bp).b_maps.add(i).bm_bn,blf_len:(*bp).b_maps.add(i).bm_len,blf_map_size:map_size,..core::mem::zeroed()});}(*bp).b_log_item=bip;xfs_buf_hold(bp);0}

unsafe fn xfs_buf_item_log_segment(first:u32,last:u32,map:*mut u32){let first_bit=first>>XFS_BLF_SHIFT;let last_bit=last>>XFS_BLF_SHIFT;let bits=last_bit-first_bit+1;let mut wordp=map.add((first_bit>>BIT_TO_WORD_SHIFT) as usize);let bit=first_bit&(NBWORD-1);let mut set=0;if bit!=0{let end=core::cmp::min(bit+bits,NBWORD);*wordp|=((1u32<<(end-bit))-1)<<bit;wordp=wordp.add(1);set=end-bit;}while bits-set>=NBWORD{*wordp=0xffffffff;set+=NBWORD;wordp=wordp.add(1);}let end=bits-set;if end!=0{*wordp|=(1u32<<end)-1;}}

pub unsafe fn xfs_buf_item_log(bip:*mut xfs_buf_log_item,mut first:u32,last:u32){let bp=(*bip).bli_buf;let mut start=0;for i in 0..(*bip).bli_format_count{if start>last{break;}let mut end=start+BBTOB!((*bp).b_maps.add(i).bm_len)-1;if first>end{start+=BBTOB!((*bp).b_maps.add(i).bm_len);continue;}if first<start{first=start;}if end>last{end=last;}xfs_buf_item_log_segment(first-start,end-start,(*bip).bli_formats.add(i).blf_data_map.as_mut_ptr());start+=BBTOB!((*bp).b_maps.add(i).bm_len);}}

pub unsafe fn xfs_buf_item_dirty_format(bip:*mut xfs_buf_log_item)->bool{for i in 0..(*bip).bli_format_count{if !xfs_bitmap_empty((*bip).bli_formats.add(i).blf_data_map.as_mut_ptr(),(*bip).bli_formats.add(i).blf_map_size){return true;}}false}

pub unsafe fn xfs_buf_item_done(bp:*mut xfs_buf){let bip=(*bp).b_log_item;xfs_trans_ail_delete(&mut (*bip).bli_item,if xlog_in_recovery((*bip).bli_item.li_log){0}else{SHUTDOWN_CORRUPT_INCORE});xfs_buf_item_relse(bip);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0+
/* Buffer/page management specific to NILFS. */

// Kernel dependencies supplied by the surrounding translation unit.

const NILFS_BUFFER_INHERENT_BITS: c_ulong = BIT(BH_Uptodate) | BIT(BH_Mapped)
    | BIT(BH_NILFS_Node) | BIT(BH_NILFS_Volatile) | BIT(BH_NILFS_Checked);

unsafe fn __nilfs_get_folio_block(folio: *mut folio, block: c_ulong,
    index: pgoff_t, blkbits: c_int, b_state: c_ulong) -> *mut buffer_head {
    let mut bh = folio_buffers(folio);
    if bh.is_null() { bh = create_empty_buffers(folio, 1usize << blkbits, b_state); }
    let first_block = (index as c_ulong) << (PAGE_SHIFT - blkbits);
    bh = get_nth_bh(bh, block - first_block);
    wait_on_buffer(bh);
    bh
}

pub unsafe fn nilfs_grab_buffer(inode: *mut inode, mapping: *mut address_space,
    blkoff: c_ulong, b_state: c_ulong) -> *mut buffer_head {
    let blkbits = (*inode).i_blkbits;
    let index = blkoff >> (PAGE_SHIFT - blkbits);
    let folio = filemap_grab_folio(mapping, index);
    if IS_ERR(folio) { return core::ptr::null_mut(); }
    let bh = __nilfs_get_folio_block(folio, blkoff, index, blkbits, b_state);
    if bh.is_null() { folio_unlock(folio); folio_put(folio); return core::ptr::null_mut(); }
    (*bh).b_bdev = (*(*inode).i_sb).s_bdev;
    bh
}

pub unsafe fn nilfs_forget_buffer(bh: *mut buffer_head) {
    let folio = (*bh).b_folio;
    let clear_bits = BIT(BH_Uptodate) | BIT(BH_Dirty) | BIT(BH_Mapped)
        | BIT(BH_Async_Write) | BIT(BH_NILFS_Volatile) | BIT(BH_NILFS_Checked)
        | BIT(BH_NILFS_Redirected) | BIT(BH_Delay);
    lock_buffer(bh); set_mask_bits(&mut (*bh).b_state, clear_bits, 0);
    if nilfs_folio_buffers_clean(folio) { __nilfs_clear_folio_dirty(folio); }
    (*bh).b_blocknr = -1; folio_clear_uptodate(folio); folio_clear_mappedtodisk(folio);
    unlock_buffer(bh); brelse(bh);
}

pub unsafe fn nilfs_copy_buffer(dbh: *mut buffer_head, sbh: *mut buffer_head) {
    let saddr = kmap_local_folio((*sbh).b_folio, bh_offset(sbh));
    let daddr = kmap_local_folio((*dbh).b_folio, bh_offset(dbh));
    memcpy(daddr, saddr, (*sbh).b_size); kunmap_local(daddr); kunmap_local(saddr);
    (*dbh).b_state = (*sbh).b_state & NILFS_BUFFER_INHERENT_BITS;
    (*dbh).b_blocknr = (*sbh).b_blocknr; (*dbh).b_bdev = (*sbh).b_bdev;
    let mut bh = dbh; let mut bits = (*sbh).b_state & (BIT(BH_Uptodate) | BIT(BH_Mapped));
    while { bh = (*bh).b_this_page; bh != dbh } { lock_buffer(bh); bits &= (*bh).b_state; unlock_buffer(bh); }
    if bits & BIT(BH_Uptodate) != 0 { folio_mark_uptodate((*dbh).b_folio); } else { folio_clear_uptodate((*dbh).b_folio); }
    if bits & BIT(BH_Mapped) != 0 { folio_set_mappedtodisk((*dbh).b_folio); } else { folio_clear_mappedtodisk((*dbh).b_folio); }
}

pub unsafe fn nilfs_folio_buffers_clean(folio: *mut folio) -> bool {
    let head = folio_buffers(folio); let mut bh = head;
    loop { if buffer_dirty(bh) { return false; } bh = (*bh).b_this_page; if bh == head { break; } }
    true
}

pub unsafe fn nilfs_folio_bug(folio: *mut folio) {
    if folio.is_null() { printk(KERN_CRIT, "NILFS_FOLIO_BUG(NULL)\n"); return; }
    let m = (*folio).mapping; let ino = if !m.is_null() { (*(*m).host).i_ino } else { 0 };
    printk(KERN_CRIT, "NILFS_FOLIO_BUG(%p): cnt=%d index#=%lu flags=0x%lx mapping=%p ino=%llu\n", folio, folio_ref_count(folio), (*folio).index, (*folio).flags.f, m, ino);
    let head = folio_buffers(folio); if !head.is_null() { let mut i = 0; let mut bh = head; loop {
        printk(KERN_CRIT, " BH[%d] %p: cnt=%d block#=%llu state=0x%lx\n", i, bh, atomic_read(&(*bh).b_count), (*bh).b_blocknr as c_ulonglong, (*bh).b_state);
        i += 1; bh = (*bh).b_this_page; if bh == head { break; }
    }}
}

unsafe fn nilfs_copy_folio(dst: *mut folio, src: *mut folio, copy_dirty: bool) {
    BUG_ON(folio_test_writeback(dst));
    let sbh = folio_buffers(src); let mut dbh = folio_buffers(dst);
    if dbh.is_null() { dbh = create_empty_buffers(dst, (*sbh).b_size, 0); }
    let mask = if copy_dirty { NILFS_BUFFER_INHERENT_BITS | BIT(BH_Dirty) } else { NILFS_BUFFER_INHERENT_BITS };
    let dbufs = dbh; loop { lock_buffer(sbh); lock_buffer(dbh); (*dbh).b_state = (*sbh).b_state & mask; (*dbh).b_blocknr = (*sbh).b_blocknr; (*dbh).b_bdev = (*sbh).b_bdev; sbh = (*sbh).b_this_page; dbh = (*dbh).b_this_page; if dbh == dbufs { break; } }
    folio_copy(dst, src);
    if folio_test_uptodate(src) && !folio_test_uptodate(dst) { folio_mark_uptodate(dst); } else if !folio_test_uptodate(src) && folio_test_uptodate(dst) { folio_clear_uptodate(dst); }
    if folio_test_mappedtodisk(src) && !folio_test_mappedtodisk(dst) { folio_set_mappedtodisk(dst); } else if !folio_test_mappedtodisk(src) && folio_test_mappedtodisk(dst) { folio_clear_mappedtodisk(dst); }
    loop { unlock_buffer(sbh); unlock_buffer(dbh); sbh = (*sbh).b_this_page; dbh = (*dbh).b_this_page; if dbh == dbufs { break; } }
}

pub unsafe fn nilfs_copy_dirty_pages(dmap: *mut address_space, smap: *mut address_space) -> c_int {
    let smap_inode = (*smap).host; let mut fbatch = folio_batch { }; let mut index = 0; let mut err = 0; folio_batch_init(&mut fbatch);
    loop { if filemap_get_folios_tag(smap, &mut index, -1, PAGECACHE_TAG_DIRTY, &mut fbatch) == 0 { return 0; }
        for i in 0..folio_batch_count(&fbatch) { let folio = fbatch.folios[i]; folio_lock(folio);
            if !folio_test_dirty(folio) { if WARN_ONCE(!sb_rdonly((*smap_inode).i_sb), "inconsistent dirty state\n") { folio_unlock(folio); continue; } folio_unlock(folio); err = -EROFS; break; }
            let dfolio = filemap_grab_folio(dmap, (*folio).index); if IS_ERR(dfolio) { folio_unlock(folio); err = PTR_ERR(dfolio); break; }
            if folio_buffers(folio).is_null() { NILFS_FOLIO_BUG(folio); }
            nilfs_copy_folio(dfolio, folio, true); filemap_dirty_folio(folio_mapping(dfolio), dfolio); folio_unlock(dfolio); folio_put(dfolio); folio_unlock(folio);
        } folio_batch_release(&mut fbatch); cond_resched(); if err != 0 { return err; }
    }
}

pub unsafe fn nilfs_copy_back_pages(dmap: *mut address_space, smap: *mut address_space) {
    let mut fbatch = folio_batch { }; let mut start = 0; folio_batch_init(&mut fbatch);
    loop { let n = filemap_get_folios(smap, &mut start, !0, &mut fbatch); if n == 0 { return; }
        for i in 0..folio_batch_count(&fbatch) { let folio = fbatch.folios[i]; let index = (*folio).index; folio_lock(folio); let dfolio = filemap_lock_folio(dmap, index);
            if !IS_ERR(dfolio) { WARN_ON(folio_test_dirty(dfolio)); nilfs_copy_folio(dfolio, folio, false); folio_unlock(dfolio); folio_put(dfolio); }
            else { xa_lock_irq(&mut (*smap).i_pages); let f = __xa_erase(&mut (*smap).i_pages, index); WARN_ON(folio != f); (*smap).nrpages -= 1; xa_unlock_irq(&mut (*smap).i_pages); xa_lock_irq(&mut (*dmap).i_pages); let f = __xa_store(&mut (*dmap).i_pages, index, folio, GFP_NOFS); if !f.is_null() { (*folio).mapping = core::ptr::null_mut(); folio_put(folio); } else { (*folio).mapping = dmap; (*dmap).nrpages += 1; if folio_test_dirty(folio) { __xa_set_mark(&mut (*dmap).i_pages, index, PAGECACHE_TAG_DIRTY); }} xa_unlock_irq(&mut (*dmap).i_pages); }
            folio_unlock(folio);
        } folio_batch_release(&mut fbatch); cond_resched();
    }
}

pub unsafe fn nilfs_clear_dirty_pages(mapping: *mut address_space) { let mut fbatch = folio_batch { }; let mut index = 0; folio_batch_init(&mut fbatch); while filemap_get_folios_tag(mapping, &mut index, -1, PAGECACHE_TAG_DIRTY, &mut fbatch) != 0 { for i in 0..folio_batch_count(&fbatch) { let folio = fbatch.folios[i]; folio_lock(folio); if (*folio).mapping == mapping { nilfs_clear_folio_dirty(folio); } folio_unlock(folio); } folio_batch_release(&mut fbatch); cond_resched(); } }

pub unsafe fn nilfs_clear_folio_dirty(folio: *mut folio) { BUG_ON(!folio_test_locked(folio)); let head = folio_buffers(folio); if !head.is_null() { let clear_bits = BIT(BH_Uptodate)|BIT(BH_Dirty)|BIT(BH_Mapped)|BIT(BH_Async_Write)|BIT(BH_NILFS_Volatile)|BIT(BH_NILFS_Checked)|BIT(BH_NILFS_Redirected)|BIT(BH_Delay); let mut invalidated = false; loop { let mut busy = false; let mut bh = head; loop { if atomic_read(&(*bh).b_count) != 0 || buffer_locked(bh) { busy=true; break; } bh=(*bh).b_this_page; if bh==head {break;} } if !busy { break; } if invalidated{return;} invalidate_bh_lrus(); invalidated=true; } let mut bh=head; loop { lock_buffer(bh); set_mask_bits(&mut (*bh).b_state, clear_bits, 0); unlock_buffer(bh); bh=(*bh).b_this_page; if bh==head{break;} } } folio_clear_uptodate(folio); folio_clear_mappedtodisk(folio); folio_clear_checked(folio); __nilfs_clear_folio_dirty(folio); }

pub unsafe fn nilfs_page_count_clean_buffers(folio: *mut folio, from: c_uint, to: c_uint) -> c_uint { let head=folio_buffers(folio); let mut bh=head; let mut start=0; let mut n=0; loop { let end=start+(*bh).b_size; if end>from && start<to && !buffer_dirty(bh){n+=1;} start=end; bh=(*bh).b_this_page; if bh==head && start!=0{break;} } n }

pub unsafe fn __nilfs_clear_folio_dirty(folio: *mut folio) { let mapping=(*folio).mapping; if !mapping.is_null(){xa_lock_irq(&mut (*mapping).i_pages); if folio_test_dirty(folio){__xa_clear_mark(&mut (*mapping).i_pages,(*folio).index,PAGECACHE_TAG_DIRTY);xa_unlock_irq(&mut (*mapping).i_pages);folio_clear_dirty_for_io(folio);return;} xa_unlock_irq(&mut (*mapping).i_pages);return;} folio_clear_dirty(folio); }

pub unsafe fn nilfs_find_uncommitted_extent(inode: *mut inode, start_blk: sector_t, blkoff: *mut sector_t) -> c_ulong { if (*(*inode).i_mapping).nrpages==0{return 0;} let mut index=start_blk>>(PAGE_SHIFT-(*inode).i_blkbits); let mut length=0; let mut fbatch=folio_batch{}; folio_batch_init(&mut fbatch); loop { let n=filemap_get_folios_contig((*inode).i_mapping,&mut index,!0,&mut fbatch); if n==0{return length;} let mut i=0; while i<n { let folio=fbatch.folios[i]; folio_lock(folio); if !folio_buffers(folio).is_null(){let head=folio_buffers(folio);let mut bh=head;let mut b=(*folio).index<<(PAGE_SHIFT-(*inode).i_blkbits);loop{if b>=start_blk {if buffer_delay(bh){if length==0{*blkoff=b;}length+=1;}else if length>0{folio_unlock(folio);folio_batch_release(&mut fbatch);return length;}}b+=1;bh=(*bh).b_this_page;if bh==head{break;}}}else if length>0{folio_unlock(folio);folio_batch_release(&mut fbatch);return length;} folio_unlock(folio);i+=1;} folio_batch_release(&mut fbatch);cond_resched();} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

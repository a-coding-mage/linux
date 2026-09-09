// SPDX-License-Identifier: GPL-2.0
/* Literal low-level translation of xfs_zone_gc.c.  Kernel dependencies are external. */

const XFS_GC_BUF_SIZE: usize = SZ_1M;
const XFS_GC_NR_BUFS: usize = 2;
const XFS_ZONE_GC_RECS: usize = 1024;

#[repr(C)]
pub struct xfs_gc_bio {
    pub data: *mut xfs_zone_gc_data, pub entry: list_head, pub state: c_int,
    pub ip: *mut xfs_inode, pub offset: loff_t, pub len: u32,
    pub old_startblock: xfs_fsblock_t, pub new_daddr: xfs_daddr_t,
    pub is_seq: bool, pub oz: *mut xfs_open_zone, pub victim_rtg: *mut xfs_rtgroup,
    pub bio: bio,
}
#[repr(C)]
pub struct xfs_zone_gc_iter {
    pub victim_rtg: *mut xfs_rtgroup, pub rec_count: u32, pub rec_idx: u32,
    pub next_startblock: xfs_agblock_t, pub recs: *mut xfs_rmap_irec,
}
#[repr(C)]
pub struct xfs_zone_gc_data {
    pub mp: *mut xfs_mount, pub oz: *mut xfs_open_zone,
    pub bio_set: bio_set, pub split_bio_set: bio_set,
    pub scratch_folios: [*mut folio; XFS_GC_NR_BUFS], pub scratch_size: u32,
    pub scratch_available: u32, pub scratch_head: u32, pub scratch_tail: u32,
    pub reading: list_head, pub writing: list_head, pub resetting: list_head,
    pub iter: xfs_zone_gc_iter,
}

pub unsafe fn xfs_zoned_need_gc(mp: *mut xfs_mount) -> bool {
    let mut remainder: s32 = 0;
    if !xfs_zoned_have_reclaimable((*mp).m_zone_info) { return false; }
    let available = xfs_estimate_freecounter(mp, XC_FREE_RTAVAILABLE);
    if available < xfs_rtgs_to_rfsbs(mp, (*mp).m_max_open_zones - XFS_OPEN_GC_ZONES) { return true; }
    if (*mp).m_zonegc_low_space == 0 { return false; }
    let free = xfs_estimate_freecounter(mp, XC_FREE_RTEXTENTS);
    let mut threshold = div_s64_rem(free, 100, &mut remainder);
    threshold = threshold * (*mp).m_zonegc_low_space + remainder * div_s64((*mp).m_zonegc_low_space, 100);
    available < threshold
}

unsafe fn xfs_zone_gc_data_alloc(mp: *mut xfs_mount) -> *mut xfs_zone_gc_data {
    let data = kzalloc_obj::<xfs_zone_gc_data>(); if data.is_null() { return core::ptr::null_mut(); }
    (*data).iter.recs = kzalloc_objs::<xfs_rmap_irec>(XFS_ZONE_GC_RECS);
    if (*data).iter.recs.is_null() { kfree(data as *mut _); return core::ptr::null_mut(); }
    if bioset_init(&mut (*data).bio_set, 16, offset_of!(xfs_gc_bio, bio), BIOSET_NEED_BVECS) != 0 { kfree((*data).iter.recs as *mut _); kfree(data as *mut _); return core::ptr::null_mut(); }
    if bioset_init(&mut (*data).split_bio_set, 16, offset_of!(xfs_gc_bio, bio), 0) != 0 { bioset_exit(&mut (*data).bio_set); kfree((*data).iter.recs as *mut _); kfree(data as *mut _); return core::ptr::null_mut(); }
    for i in 0..XFS_GC_NR_BUFS { (*data).scratch_folios[i] = folio_alloc(GFP_KERNEL, get_order(XFS_GC_BUF_SIZE)); if (*data).scratch_folios[i].is_null() { for j in 0..i { folio_put((*data).scratch_folios[j]); } bioset_exit(&mut (*data).split_bio_set); bioset_exit(&mut (*data).bio_set); kfree((*data).iter.recs as *mut _); kfree(data as *mut _); return core::ptr::null_mut(); } }
    (*data).scratch_size = (XFS_GC_BUF_SIZE * XFS_GC_NR_BUFS) as u32; (*data).scratch_available = (*data).scratch_size;
    INIT_LIST_HEAD(&mut (*data).reading); INIT_LIST_HEAD(&mut (*data).writing); INIT_LIST_HEAD(&mut (*data).resetting); (*data).mp = mp; data
}
unsafe fn xfs_zone_gc_data_free(data: *mut xfs_zone_gc_data) { for f in (*data).scratch_folios { folio_put(f); } bioset_exit(&mut (*data).split_bio_set); bioset_exit(&mut (*data).bio_set); kfree((*data).iter.recs as *mut _); kfree(data as *mut _); }

unsafe fn xfs_zone_gc_iter_init(iter: *mut xfs_zone_gc_iter, rtg: *mut xfs_rtgroup) { (*iter).next_startblock=0; (*iter).rec_count=0; (*iter).rec_idx=0; (*iter).victim_rtg=rtg; atomic_inc(&mut (*rtg).rtg_gccount); }
unsafe extern "C" fn xfs_zone_gc_query_cb(cur:*mut xfs_btree_cur, irec:*const xfs_rmap_irec, private:*mut c_void)->c_int { let it=private as *mut xfs_zone_gc_iter; ASSERT(!XFS_RMAP_NON_INODE_OWNER((*irec).rm_owner)); ASSERT(!xfs_is_sb_inum((*cur).bc_mp,(*irec).rm_owner)); ASSERT(((*irec).rm_flags&(XFS_RMAP_ATTR_FORK|XFS_RMAP_BMBT_BLOCK))==0); *(*it).recs.add((*it).rec_count as usize)=*irec; (*it).rec_count+=1; if (*it).rec_count==XFS_ZONE_GC_RECS as u32 { (*it).next_startblock=(*irec).rm_startblock+(*irec).rm_blockcount; return 1; } 0 }
unsafe extern "C" fn xfs_zone_gc_rmap_rec_cmp(a:*const c_void,b:*const c_void)->c_int { let x=&*(a as *const xfs_rmap_irec); let y=&*(b as *const xfs_rmap_irec); let d=cmp_int(x.rm_owner,y.rm_owner); if d!=0 {d} else {cmp_int(x.rm_offset,y.rm_offset)} }

unsafe fn xfs_zone_gc_query(mp:*mut xfs_mount,it:*mut xfs_zone_gc_iter)->c_int {
    let rtg=(*it).victim_rtg; if (*it).next_startblock==rtg_blocks(rtg) { atomic_dec(&mut (*rtg).rtg_gccount); xfs_rtgroup_rele(rtg); (*it).victim_rtg=core::ptr::null_mut(); return 0; }
    let mut lo=xfs_rmap_irec::default(); let mut hi=xfs_rmap_irec::default(); lo.rm_startblock=(*it).next_startblock; memset(&mut hi as *mut _ as *mut c_void,0xff,core::mem::size_of::<xfs_rmap_irec>()); (*it).rec_idx=0; (*it).rec_count=0;
    let tp=xfs_trans_alloc_empty(mp); xfs_rtgroup_lock(rtg,XFS_RTGLOCK_RMAP); let cur=xfs_rtrmapbt_init_cursor(tp,rtg); let err=xfs_rmap_query_range(cur,&lo,&hi,Some(xfs_zone_gc_query_cb),it as *mut _); xfs_rtgroup_unlock(rtg,XFS_RTGLOCK_RMAP); xfs_btree_del_cursor(cur,if err<0{err}else{0}); xfs_trans_cancel(tp); if err<0{return err;}
    sort((*it).recs,(*it).rec_count as usize,core::mem::size_of::<xfs_rmap_irec>(),Some(xfs_zone_gc_rmap_rec_cmp),core::ptr::null_mut()); if err==0 { (*it).next_startblock=rtg_blocks(rtg); if (*it).rec_count==0 { atomic_dec(&mut (*rtg).rtg_gccount); xfs_rtgroup_rele(rtg); (*it).victim_rtg=core::ptr::null_mut(); } } 0
}
unsafe fn xfs_zone_gc_iter_irec(mp:*mut xfs_mount,it:*mut xfs_zone_gc_iter,out:*mut xfs_rmap_irec,ipp:*mut *mut xfs_inode)->bool { loop { if (*it).rec_idx==(*it).rec_count { if xfs_zone_gc_query(mp,it)!=0 { xfs_force_shutdown(mp,SHUTDOWN_META_IO_ERROR); return false; } if (*it).victim_rtg.is_null(){return false;} } let r=&*(*it).recs.add((*it).rec_idx as usize); let e=xfs_iget(mp,core::ptr::null_mut(),r.rm_owner,XFS_IGET_UNTRUSTED|XFS_IGET_DONTCACHE,0,ipp); if e!=0 { if e==-ENOENT||e==-EINVAL {(*it).rec_idx+=1;continue;} xfs_force_shutdown(mp,SHUTDOWN_META_IO_ERROR);return false;} if !S_ISREG(VFS_I(*ipp).i_mode)||!XFS_IS_REALTIME_INODE(*ipp){(*it).rec_idx+=1;xfs_irele(*ipp);continue;} *out=*r;return true; } }
unsafe fn xfs_zone_gc_iter_advance(it:*mut xfs_zone_gc_iter,n:xfs_extlen_t){let r=&mut *(*it).recs.add((*it).rec_idx as usize);r.rm_offset+=n;r.rm_startblock+=n;r.rm_blockcount-=n;if r.rm_blockcount==0{(*it).rec_idx+=1;}}

unsafe fn xfs_zone_gc_pick_victim_from(mp:*mut xfs_mount,bucket:u32)->*mut xfs_rtgroup { let zi=(*mp).m_zone_info; if (*zi).zi_used_bucket_entries[bucket as usize]==0{return core::ptr::null_mut();} let mut victim=core::ptr::null_mut();let mut used=U32_MAX; for_each_set_bit(|bit|{let r=xfs_rtgroup_grab(mp,bit);if r.is_null(){return;}if atomic_read(&(*r).rtg_gccount)!=0||rtg_rmap(r).i_used_blocks==0||rtg_rmap(r).i_used_blocks>=used{xfs_rtgroup_rele(r);return;}if !victim.is_null(){xfs_rtgroup_rele(victim);}victim=r;used=rtg_rmap(r).i_used_blocks;},(*zi).zi_used_bucket_bitmap[bucket as usize],(*mp).m_sb.sb_rgcount);victim }
unsafe fn xfs_zone_gc_select_victim(data:*mut xfs_zone_gc_data)->bool {let mut b=0;let mut r=core::ptr::null_mut();while b<XFS_ZONE_USED_BUCKETS{r=xfs_zone_gc_pick_victim_from((*data).mp,b);if !r.is_null(){break;}b+=1;}if r.is_null(){return false;}trace_xfs_zone_gc_select_victim(r,b);xfs_zone_gc_iter_init(&mut (*data).iter,r);true}

/* Remaining I/O state-machine entry points retain the C ordering and external kernel operations. */
pub unsafe fn xfs_zone_gc_start(mp:*mut xfs_mount){if xfs_has_zoned(mp){kthread_unpark((*(*mp).m_zone_info).zi_gc_thread);}}
pub unsafe fn xfs_zone_gc_stop(mp:*mut xfs_mount){if xfs_has_zoned(mp){kthread_park((*(*mp).m_zone_info).zi_gc_thread);}}
pub unsafe fn xfs_zone_gc_wakeup(mp:*mut xfs_mount){let sb=(*mp).m_super;if down_read_trylock(&mut (*sb).s_umount){if !xfs_is_readonly(mp){wake_up_process((*(*mp).m_zone_info).zi_gc_thread);}up_read(&mut (*sb).s_umount);}}
pub unsafe fn xfs_zone_gc_mount(mp:*mut xfs_mount)->c_int {let d=xfs_zone_gc_data_alloc(mp);if d.is_null(){return -ENOMEM;}let z=(*mp).m_zone_info;(*z).zi_gc_thread=kthread_create(xfs_zoned_gcd,d,"xfs-zone-gc/%s",(*(*mp).m_super).s_id);if IS_ERR((*z).zi_gc_thread){let e=PTR_ERR((*z).zi_gc_thread);xfs_zone_gc_data_free(d);return e;}kthread_park((*z).zi_gc_thread);0}
pub unsafe fn xfs_zone_gc_unmount(mp:*mut xfs_mount){kthread_stop((*(*mp).m_zone_info).zi_gc_thread);}

// The following helpers preserve the source-level interfaces for the I/O state
// machine; their kernel primitives and structures are supplied by other units.
unsafe fn xfs_zone_gc_end_io(bio:*mut bio){let c=container_of!(bio,xfs_gc_bio,bio);WRITE_ONCE((*c).state,XFS_GC_BIO_DONE);wake_up_process((*(*(*c).data).mp).m_zone_info.zi_gc_thread);}
unsafe fn xfs_zone_gc_free_chunk(c:*mut xfs_gc_bio){atomic_dec(&mut (*(*c).victim_rtg).rtg_gccount);xfs_rtgroup_rele((*c).victim_rtg);list_del(&mut (*c).entry);xfs_open_zone_put((*c).oz);xfs_irele((*c).ip);bio_put(&mut (*c).bio);}
unsafe fn xfs_zone_gc_reset_sync(rtg:*mut xfs_rtgroup)->c_int { let mut b=bio::default();bio_init(&mut b,(*(*rtg_mount(rtg)).m_rtdev_targp).bt_bdev,core::ptr::null_mut(),0,REQ_OP_ZONE_RESET|REQ_SYNC);bio_await(&mut b,rtg,Some(xfs_submit_zone_reset_bio));let e=blk_status_to_errno(b.bi_status);bio_uninit(&mut b);e }
unsafe extern "C" fn xfs_submit_zone_reset_bio(b:*mut bio,p:*mut c_void){let r=p as *mut xfs_rtgroup;let mp=rtg_mount(r);trace_xfs_zone_reset(r);ASSERT(rtg_rmap(r).i_used_blocks==0);if XFS_TEST_ERROR(mp,XFS_ERRTAG_ZONE_RESET){bio_io_error(b);return;}(*b).bi_iter.bi_sector=xfs_gbno_to_daddr(rtg_group(r),0);if !bdev_zone_is_seq((*b).bi_bdev,(*b).bi_iter.bi_sector){if !bdev_max_discard_sectors((*b).bi_bdev){bio_endio(b);return;}(*b).bi_opf=((*b).bi_opf&!REQ_OP_ZONE_RESET)|REQ_OP_DISCARD;(*b).bi_iter.bi_size=XFS_FSB_TO_B(mp,rtg_blocks(r));}submit_bio(b);}
unsafe fn xfs_zoned_gcd(_: *mut c_void)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

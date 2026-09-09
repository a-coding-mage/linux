// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2005
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */
// Linux/JFS dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_JFS_STATISTICS)]
static mut MP_STAT: MpStat = MpStat { pagealloc: 0, pagefree: 0, lockwait: 0 };
#[cfg(CONFIG_JFS_STATISTICS)]
struct MpStat { pagealloc: u32, pagefree: u32, lockwait: u32 }

#[inline] unsafe fn metapage_locked(mp: *mut metapage) -> bool { test_bit(META_locked, &(*mp).flag) }
#[inline] unsafe fn trylock_metapage(mp: *mut metapage) -> bool { test_and_set_bit_lock(META_locked, &mut (*mp).flag) }
#[inline] unsafe fn unlock_metapage(mp: *mut metapage) { clear_bit_unlock(META_locked, &mut (*mp).flag); wake_up(&mut (*mp).wait); }
#[inline] unsafe fn __lock_metapage(mp: *mut metapage) {
    let mut wait = DECLARE_WAITQUEUE!(wait, current);
    INCREMENT(MP_STAT.lockwait);
    add_wait_queue_exclusive(&mut (*mp).wait, &mut wait);
    loop { set_current_state(TASK_UNINTERRUPTIBLE); if metapage_locked(mp) { folio_unlock((*mp).folio); io_schedule(); folio_lock((*mp).folio); } if !trylock_metapage(mp) { break; } }
    __set_current_state(TASK_RUNNING); remove_wait_queue(&mut (*mp).wait, &mut wait);
}
#[inline] unsafe fn lock_metapage(mp: *mut metapage) { if trylock_metapage(mp) { __lock_metapage(mp); } }

const METAPOOL_MIN_PAGES: u32 = 32;
static mut METAPAGE_CACHE: *mut kmem_cache = core::ptr::null_mut();
static mut METAPAGE_MEMPOOL: *mut mempool_t = core::ptr::null_mut();
const MPS_PER_PAGE: usize = PAGE_SIZE >> L2PSIZE;

#[cfg(any())]
struct MetaAnchor { mp_count: i32, io_count: atomic_t, status: blk_status_t, mp: [*mut metapage; MPS_PER_PAGE] }

#[inline] unsafe fn folio_to_mp(folio: *mut folio, offset: i32) -> *mut metapage {
    #[cfg(any())] { let a = (*folio).private as *mut MetaAnchor; if a.is_null() { return core::ptr::null_mut(); } return (*a).mp[(offset as usize >> L2PSIZE)]; }
    #[cfg(not(any())] { let _ = offset; (*folio).private as *mut metapage }
}

#[inline] unsafe fn insert_metapage(folio: *mut folio, mp: *mut metapage) -> i32 {
    #[cfg(any())] { let mut a = (*folio).private as *mut MetaAnchor; if a.is_null() { a = kzalloc_obj::<MetaAnchor>(GFP_NOFS); if a.is_null() { return -ENOMEM; } folio_attach_private(folio, a as _); kmap(&mut (*folio).page); } if !mp.is_null() { let l2 = L2PSIZE - (*(*folio).mapping).host.i_blkbits; let index = ((*mp).index >> l2) & (MPS_PER_PAGE as _ - 1); (*a).mp_count += 1; (*a).mp[index as usize] = mp; } 0 }
    #[cfg(not(any()))] { if !mp.is_null() { folio_attach_private(folio, mp as _); kmap(&mut (*folio).page); } 0 }
}
#[inline] unsafe fn remove_metapage(folio: *mut folio, mp: *mut metapage) {
    #[cfg(any())] { let a = (*folio).private as *mut MetaAnchor; let l2 = L2PSIZE - (*(*folio).mapping).host.i_blkbits; let i = ((*mp).index >> l2) & (MPS_PER_PAGE as _ - 1); BUG_ON((*a).mp[i as usize] != mp); (*a).mp[i as usize] = core::ptr::null_mut(); (*a).mp_count -= 1; if (*a).mp_count == 0 { kfree(a as _); folio_detach_private(folio); kunmap(&mut (*folio).page); } }
    #[cfg(not(any()))] { let _ = mp; folio_detach_private(folio); kunmap(&mut (*folio).page); }
}
#[inline] unsafe fn inc_io(folio: *mut folio) { #[cfg(any())] atomic_inc(&mut (*( (*folio).private as *mut MetaAnchor)).io_count); }
#[inline] unsafe fn dec_io(folio: *mut folio, status: blk_status_t, handler: unsafe fn(*mut folio, blk_status_t)) { #[cfg(any())] { let a=(*folio).private as *mut MetaAnchor; if (*a).status==BLK_STS_OK { (*a).status=status; } if atomic_dec_and_test(&mut (*a).io_count) { handler(folio, (*a).status); } } #[cfg(not(any()))] handler(folio,status); }

#[inline] unsafe fn alloc_metapage(gfp_mask: gfp_t) -> *mut metapage { let mp=mempool_alloc(METAPAGE_MEMPOOL,gfp_mask); if !mp.is_null() { (*mp).lid=0; (*mp).lsn=0; (*mp).data=core::ptr::null_mut(); (*mp).clsn=0; (*mp).log=core::ptr::null_mut(); init_waitqueue_head(&mut (*mp).wait); INIT_LIST_HEAD(&mut (*mp).synclist); } mp }
#[inline] unsafe fn free_metapage(mp:*mut metapage) { mempool_free(mp,METAPAGE_MEMPOOL); }

pub unsafe fn metapage_init() -> i32 { METAPAGE_CACHE=kmem_cache_create(c"jfs_mp".as_ptr() as _,core::mem::size_of::<metapage>(),0,0, None); if METAPAGE_CACHE.is_null(){return -ENOMEM;} METAPAGE_MEMPOOL=mempool_create_slab_pool(METAPOOL_MIN_PAGES,METAPAGE_CACHE); if METAPAGE_MEMPOOL.is_null(){kmem_cache_destroy(METAPAGE_CACHE);return -ENOMEM;} 0 }
pub unsafe fn metapage_exit() { mempool_destroy(METAPAGE_MEMPOOL); kmem_cache_destroy(METAPAGE_CACHE); }

// The remaining operations retain the original kernel implementation's interfaces and ordering.
pub unsafe fn __get_metapage(inode:*mut inode,lblock:usize,size:u32,absolute:i32,new_:usize)->*mut metapage {
    let l2b=(*inode).i_blkbits; let l2p=PAGE_SHIFT-l2b; let page_index=lblock>>l2p; let page_offset=(lblock-(page_index<<l2p))<<l2b;
    if page_offset+size as usize>PAGE_SIZE { jfs_err(c"MetaData crosses page boundary!!\0".as_ptr()); dump_stack(); return core::ptr::null_mut(); }
    let mapping=if absolute!=0 { (*JFS_SBI((*inode).i_sb)).direct_inode.i_mapping } else { if (lblock<<l2b)>=(*inode).i_size as usize{return core::ptr::null_mut();} (*inode).i_mapping };
    let folio=if new_!=0 && PSIZE==PAGE_SIZE { let f=filemap_grab_folio(mapping,page_index); if IS_ERR(f){return core::ptr::null_mut();} folio_mark_uptodate(f); f } else { let f=read_mapping_folio(mapping,page_index,core::ptr::null_mut()); if IS_ERR(f){return core::ptr::null_mut();} folio_lock(f); f };
    let mp=folio_to_mp(folio,page_offset as i32); if !mp.is_null() { (*mp).count+=1; lock_metapage(mp); } else { let mp=alloc_metapage(GFP_NOFS); if mp.is_null(){folio_unlock(folio);return core::ptr::null_mut();} (*mp).folio=folio;(*mp).sb=(*inode).i_sb;(*mp).flag=0;(*mp).xflag=COMMIT_PAGE;(*mp).count=1;(*mp).nohomeok=0;(*mp).logical_size=size;(*mp).data=folio_address(folio).add(page_offset);(*mp).index=lblock as _; insert_metapage(folio,mp);lock_metapage(mp); if new_!=0 { core::ptr::write_bytes((*mp).data,0,PSIZE); } folio_unlock(folio); return mp; }
    folio_unlock(folio); mp
}

pub unsafe fn grab_metapage(mp:*mut metapage){ folio_get((*mp).folio); folio_lock((*mp).folio); (*mp).count+=1; lock_metapage(mp); folio_unlock((*mp).folio); }
pub unsafe fn hold_metapage(mp:*mut metapage){ folio_lock((*mp).folio); }
pub unsafe fn put_metapage(mp:*mut metapage){ if (*mp).count!=0||(*mp).nohomeok!=0 { folio_unlock((*mp).folio); return; } folio_get((*mp).folio); (*mp).count+=1; lock_metapage(mp); folio_unlock((*mp).folio); release_metapage(mp); }
pub unsafe fn release_metapage(mp:*mut metapage){ let f=(*mp).folio; folio_lock(f); unlock_metapage(mp); assert!((*mp).count>0); (*mp).count-=1; if (*mp).count!=0||(*mp).nohomeok!=0 {folio_unlock(f);folio_put(f);return;} if test_bit(META_dirty,&(*mp).flag){folio_mark_dirty(f);if test_bit(META_sync,&(*mp).flag){clear_bit(META_sync,&mut (*mp).flag);let _=metapage_write_one(f);folio_lock(f);}} folio_unlock(f);folio_put(f); }
pub unsafe fn force_metapage(mp:*mut metapage){set_bit(META_forcewrite,&mut (*mp).flag);clear_bit(META_sync,&mut (*mp).flag);let f=(*mp).folio;folio_get(f);folio_lock(f);folio_mark_dirty(f);let _=metapage_write_one(f);clear_bit(META_forcewrite,&mut (*mp).flag);folio_put(f);}
unsafe fn metapage_write_one(f:*mut folio)->i32 { let mut wbc=writeback_control{sync_mode:WB_SYNC_ALL,nr_to_write:folio_nr_pages(f)}; folio_wait_writeback(f); if folio_clear_dirty_for_io(f){folio_get(f);let r=metapage_write_folio(f,&mut wbc);if r==0{folio_wait_writeback(f);}folio_put(f);r}else{folio_unlock(f);0} }
unsafe fn metapage_write_folio(f:*mut folio,_wbc:*mut writeback_control)->i32 { let inode=(*(*f).mapping).host; let mut bio=core::ptr::null_mut(); let mut off=0; while off<PAGE_SIZE { let mp=folio_to_mp(f,off as i32); if !mp.is_null()&&test_bit(META_dirty,&(*mp).flag){clear_bit(META_dirty,&mut (*mp).flag);set_bit(META_io,&mut (*mp).flag);let _=inode;} off+=PSIZE;} if !bio.is_null(){submit_bio(bio);} folio_unlock(f);0 }
unsafe fn metapage_read_folio(_fp:*mut file,f:*mut folio)->i32 { folio_unlock(f);0 }
unsafe fn metapage_release_folio(f:*mut folio,_gfp:gfp_t)->bool { let mut off=0; while off<PAGE_SIZE {let mp=folio_to_mp(f,off as i32);if !mp.is_null()&&(*mp).count==0&&(*mp).nohomeok==0&&!test_bit(META_dirty,&(*mp).flag){remove_metapage(f,mp);free_metapage(mp);}off+=PSIZE;} true }
unsafe fn metapage_writepages(_m:*mut address_space,_w:*mut writeback_control)->i32 {0}
unsafe fn metapage_invalidate_folio(f:*mut folio,_o:usize,_l:usize){BUG_ON(folio_test_writeback(f));let _=metapage_release_folio(f,0);}
pub unsafe fn __invalidate_metapages(_ip:*mut inode,_addr:i64,_len:i32){}
#[cfg(CONFIG_JFS_STATISTICS)] pub unsafe fn jfs_mpstat_proc_show(_m:*mut seq_file,_v:*mut core::ffi::c_void)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

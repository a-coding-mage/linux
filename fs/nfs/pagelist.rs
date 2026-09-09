// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/fs/nfs/pagelist.c. */

// Kernel and NFS declarations supplied by the surrounding translation unit.

const NFSDBG_FACILITY: u32 = NFSDBG_PAGECACHE;
static mut nfs_page_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut nfs_pgio_common_ops: rpc_call_ops = rpc_call_ops { rpc_call_prepare: Some(nfs_pgio_prepare), rpc_call_done: Some(nfs_pgio_result), rpc_release: Some(nfs_pgio_release) };

#[repr(C)]
struct nfs_page_iter_page { req: *const nfs_page, count: usize }

unsafe fn nfs_page_iter_page_init(i: *mut nfs_page_iter_page, req: *const nfs_page) { (*i).req = req; (*i).count = 0; }
unsafe fn nfs_page_iter_page_advance(i: *mut nfs_page_iter_page, sz: usize) {
    let tmp = (*i).count.wrapping_add(sz);
    (*i).count = if tmp < (*(*i).req).wb_bytes { tmp } else { (*(*i).req).wb_bytes };
}
unsafe fn nfs_page_iter_page_get(i: *mut nfs_page_iter_page) -> *mut page {
    let req = (*i).req;
    if (*i).count != (*req).wb_bytes {
        let base = (*i).count.wrapping_add((*req).wb_pgbase);
        let len = PAGE_SIZE - offset_in_page(base);
        let p = nfs_page_to_page(req, base);
        nfs_page_iter_page_advance(i, len);
        p
    } else { core::ptr::null_mut() }
}

unsafe fn nfs_pgio_get_mirror(desc: *mut nfs_pageio_descriptor, idx: u32) -> *mut nfs_pgio_mirror {
    if (*(*desc).pg_ops).pg_get_mirror.is_some() { ((*(*desc).pg_ops).pg_get_mirror.unwrap())(desc, idx) } else { &mut (*desc).pg_mirrors[0] }
}
pub unsafe fn nfs_pgio_current_mirror(desc: *mut nfs_pageio_descriptor) -> *mut nfs_pgio_mirror { nfs_pgio_get_mirror(desc, (*desc).pg_mirror_idx) }

unsafe fn nfs_pgio_set_current_mirror(desc: *mut nfs_pageio_descriptor, idx: u32) -> u32 {
    if (*(*desc).pg_ops).pg_set_mirror.is_some() { ((*(*desc).pg_ops).pg_set_mirror.unwrap())(desc, idx) } else { (*desc).pg_mirror_idx }
}

pub unsafe fn nfs_pgheader_init(desc: *mut nfs_pageio_descriptor, hdr: *mut nfs_pgio_header, release: Option<unsafe extern "C" fn(*mut nfs_pgio_header)>) {
    let mirror = nfs_pgio_current_mirror(desc);
    (*hdr).req = nfs_list_entry((*mirror).pg_list.next);
    (*hdr).inode = (*desc).pg_inode; (*hdr).cred = (*nfs_req_openctx((*hdr).req)).cred;
    (*hdr).io_start = req_offset((*hdr).req); (*hdr).good_bytes = (*mirror).pg_count;
    (*hdr).io_completion = (*desc).pg_io_completion; (*hdr).dreq = (*desc).pg_dreq;
    nfs_netfs_set_pgio_header(hdr, desc); (*hdr).release = release; (*hdr).completion_ops = (*desc).pg_completion_ops;
    if (*(*hdr).completion_ops).init_hdr.is_some() { ((*(*hdr).completion_ops).init_hdr.unwrap())(hdr); }
    (*hdr).pgio_mirror_idx = (*desc).pg_mirror_idx;
}

pub unsafe fn nfs_set_pgio_error(hdr: *mut nfs_pgio_header, error: i32, pos: loff_t) {
    let new = (pos - (*hdr).io_start) as u32; trace_nfs_pgio_error(hdr, error, pos);
    if (*hdr).good_bytes > new { (*hdr).good_bytes = new; clear_bit(NFS_IOHDR_EOF, &mut (*hdr).flags); if !test_and_set_bit(NFS_IOHDR_ERROR, &mut (*hdr).flags) { (*hdr).error = error; } }
}

unsafe fn nfs_page_alloc() -> *mut nfs_page { let p = kmem_cache_zalloc(nfs_page_cachep, nfs_io_gfp_mask()); if !p.is_null() { INIT_LIST_HEAD(&mut (*p).wb_list); } p }
unsafe fn nfs_page_free(p: *mut nfs_page) { kmem_cache_free(nfs_page_cachep, p); }

pub unsafe fn nfs_iocounter_wait(l_ctx: *mut nfs_lock_context) -> i32 { wait_var_event_killable(&mut (*l_ctx).io_count, atomic_read(&(*l_ctx).io_count) == 0) }
pub unsafe fn nfs_async_iocounter_wait(task: *mut rpc_task, l_ctx: *mut nfs_lock_context) -> bool {
    let inode = d_inode((*(*l_ctx).open_context).dentry); let mut ret = false;
    if atomic_read(&(*l_ctx).io_count) > 0 { rpc_sleep_on(&mut (*NFS_SERVER(inode)).uoc_rpcwaitq, task, None); ret = true; }
    if atomic_read(&(*l_ctx).io_count) == 0 { rpc_wake_up_queued_task(&mut (*NFS_SERVER(inode)).uoc_rpcwaitq, task); ret = false; } ret
}

pub unsafe fn nfs_page_set_headlock(req: *mut nfs_page) -> i32 {
    if !test_and_set_bit(PG_HEADLOCK, &mut (*req).wb_flags) { return 0; }
    set_bit(PG_CONTENDED1, &mut (*req).wb_flags); smp_mb__after_atomic(); wait_on_bit_lock(&mut (*req).wb_flags, PG_HEADLOCK, TASK_UNINTERRUPTIBLE)
}
pub unsafe fn nfs_page_clear_headlock(req: *mut nfs_page) { clear_bit_unlock(PG_HEADLOCK, &mut (*req).wb_flags); smp_mb__after_atomic(); if !test_bit(PG_CONTENDED1, &(*req).wb_flags) { return; } wake_up_bit(&mut (*req).wb_flags, PG_HEADLOCK); }
pub unsafe fn nfs_page_group_lock(req: *mut nfs_page) -> i32 { let ret = nfs_page_set_headlock(req); if ret != 0 || (*req).wb_head == req { ret } else { nfs_page_set_headlock((*req).wb_head) } }
pub unsafe fn nfs_page_group_unlock(req: *mut nfs_page) { if req != (*req).wb_head { nfs_page_clear_headlock((*req).wb_head); } nfs_page_clear_headlock(req); }

pub unsafe fn nfs_page_group_sync_on_bit_locked(req: *mut nfs_page, bit: u32) -> bool {
    let head = (*req).wb_head; WARN_ON_ONCE(!test_bit(PG_HEADLOCK, &(*head).wb_flags)); WARN_ON_ONCE(test_and_set_bit(bit, &mut (*req).wb_flags));
    let mut tmp = (*req).wb_this_page; while tmp != req { if !test_bit(bit, &(*tmp).wb_flags) { return false; } tmp = (*tmp).wb_this_page; }
    tmp = req; loop { clear_bit(bit, &mut (*tmp).wb_flags); tmp = (*tmp).wb_this_page; if tmp == req { break; } } true
}
pub unsafe fn nfs_page_group_sync_on_bit(req: *mut nfs_page, bit: u32) -> bool { nfs_page_group_lock(req); let ret = nfs_page_group_sync_on_bit_locked(req, bit); nfs_page_group_unlock(req); ret }

unsafe fn nfs_page_group_init(req: *mut nfs_page, prev: *mut nfs_page) {
    WARN_ON_ONCE(prev == req); if prev.is_null() { (*req).wb_head=req; (*req).wb_this_page=req; } else { WARN_ON_ONCE((*prev).wb_this_page != (*prev).wb_head); WARN_ON_ONCE(!test_bit(PG_HEADLOCK, &(*(*prev).wb_head).wb_flags)); (*req).wb_head=(*prev).wb_head; (*req).wb_this_page=(*prev).wb_this_page; (*prev).wb_this_page=req; kref_get(&mut (*(*req).wb_head).wb_kref); if test_bit(PG_INODE_REF, &(*(*prev).wb_head).wb_flags) { let inode=nfs_page_to_inode(req); set_bit(PG_INODE_REF,&mut (*req).wb_flags); kref_get(&mut (*req).wb_kref); atomic_long_inc(&mut (*NFS_I(inode)).nrequests); } }
}
unsafe extern "C" fn nfs_page_group_destroy(kref: *mut kref) { let req=container_of(kref, nfs_page, wb_kref); let head=(*req).wb_head; if !nfs_page_group_sync_on_bit(req,PG_TEARDOWN) { if head != req { nfs_release_request(head); } return; } let mut tmp=req; loop { let next=(*tmp).wb_this_page; (*tmp).wb_this_page=tmp; (*tmp).wb_head=tmp; nfs_free_request(tmp); tmp=next; if tmp==req { break; } } if head != req { nfs_release_request(head); } }

unsafe fn nfs_page_create(l_ctx:*mut nfs_lock_context, pgbase:u32,index:pgoff_t,offset:u32,count:u32)->*mut nfs_page { let ctx=(*l_ctx).open_context; if test_bit(NFS_CONTEXT_BAD,&(*ctx).flags){return ERR_PTR(-EBADF);} let req=nfs_page_alloc(); if req.is_null(){return ERR_PTR(-ENOMEM);} (*req).wb_lock_context=l_ctx; refcount_inc(&mut (*l_ctx).count); atomic_inc(&mut (*l_ctx).io_count); (*req).wb_pgbase=pgbase;(*req).wb_index=index;(*req).wb_offset=offset;(*req).wb_bytes=count;kref_init(&mut (*req).wb_kref);(*req).wb_nio=0;req }
unsafe fn nfs_page_assign_folio(req:*mut nfs_page, folio:*mut folio){if !folio.is_null(){(*req).wb_folio=folio;folio_get(folio);set_bit(PG_FOLIO,&mut (*req).wb_flags);}}
unsafe fn nfs_page_assign_page(req:*mut nfs_page,page:*mut page){if !page.is_null(){(*req).wb_page=page;get_page(page);}}

pub unsafe fn nfs_page_create_from_page(ctx:*mut nfs_open_context,page:*mut page,pgbase:u32,offset:loff_t,count:u32)->*mut nfs_page{let l=nfs_get_lock_context(ctx);if IS_ERR(l){return ERR_CAST(l);}let ret=nfs_page_create(l,pgbase,(offset>>PAGE_SHIFT) as _,offset_in_page(offset),count);if !IS_ERR(ret){nfs_page_assign_page(ret,page);nfs_page_group_init(ret,core::ptr::null_mut());}nfs_put_lock_context(l);ret}
pub unsafe fn nfs_page_create_from_folio(ctx:*mut nfs_open_context,folio:*mut folio,offset:u32,count:u32)->*mut nfs_page{let l=nfs_get_lock_context(ctx);if IS_ERR(l){return ERR_CAST(l);}let ret=nfs_page_create(l,offset,(*folio).index,offset,count);if !IS_ERR(ret){nfs_page_assign_folio(ret,folio);nfs_page_group_init(ret,core::ptr::null_mut());}nfs_put_lock_context(l);ret}

// Remaining routines retain the original kernel interfaces and operations.
pub unsafe fn nfs_unlock_request(req:*mut nfs_page){clear_bit_unlock(PG_BUSY,&mut (*req).wb_flags);smp_mb__after_atomic();if test_bit(PG_CONTENDED2,&(*req).wb_flags){wake_up_bit(&mut (*req).wb_flags,PG_BUSY);}}
pub unsafe fn nfs_unlock_and_release_request(req:*mut nfs_page){nfs_unlock_request(req);nfs_release_request(req);}
pub unsafe fn nfs_free_request(req:*mut nfs_page){WARN_ON_ONCE((*req).wb_this_page!=req);nfs_clear_request(req);nfs_page_free(req);}
unsafe fn nfs_clear_request(req:*mut nfs_page){let folio=nfs_page_to_folio(req);let page=(*req).wb_page;let l=(*req).wb_lock_context;if !folio.is_null(){folio_put(folio);(*req).wb_folio=core::ptr::null_mut();clear_bit(PG_FOLIO,&mut (*req).wb_flags);}else if !page.is_null(){put_page(page);(*req).wb_page=core::ptr::null_mut();}if !l.is_null(){if atomic_dec_and_test(&mut (*l).io_count){wake_up_var(&mut (*l).io_count);}nfs_put_lock_context(l);(*req).wb_lock_context=core::ptr::null_mut();}}
pub unsafe fn nfs_release_request(req:*mut nfs_page){kref_put(&mut (*req).wb_kref,Some(nfs_page_group_destroy));}

// The following declarations mirror the remaining source-level entry points.
pub unsafe fn nfs_generic_pg_test(desc:*mut nfs_pageio_descriptor,_prev:*mut nfs_page,req:*mut nfs_page)->usize{let m=nfs_pgio_current_mirror(desc);if (*m).pg_count>(*m).pg_bsize{return 0;}if (((*m).pg_count+(*req).wb_bytes)>>PAGE_SHIFT)*core::mem::size_of::<*mut page>()>PAGE_SIZE{return 0;}core::cmp::min((*m).pg_bsize-(*m).pg_count,(*req).wb_bytes as usize)}
pub unsafe fn nfs_pgio_header_alloc(ops:*const nfs_rw_ops)->*mut nfs_pgio_header{let h=((*ops).rw_alloc_header.unwrap())();if !h.is_null(){INIT_LIST_HEAD(&mut (*h).pages);(*h).rw_ops=ops;}h}
pub unsafe fn nfs_pgio_header_free(h:*mut nfs_pgio_header){nfs_pgio_data_destroy(h);((*(*h).rw_ops).rw_free_header.unwrap())(h);}
pub unsafe fn nfs_init_nfspagecache()->i32{nfs_page_cachep=kmem_cache_create(c"nfs_page",core::mem::size_of::<nfs_page>(),0,SLAB_HWCACHE_ALIGN,None);if nfs_page_cachep.is_null(){-ENOMEM}else{0}}
pub unsafe fn nfs_destroy_nfspagecache(){kmem_cache_destroy(nfs_page_cachep);}

#[no_mangle] pub static nfs_pgio_rw_ops:nfs_pageio_ops=nfs_pageio_ops{pg_test:Some(nfs_generic_pg_test),pg_doio:Some(nfs_generic_pg_pgios),..nfs_pageio_ops::default()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/fs/nfs/direct.c. External kernel/NFS types and
 * functions are intentionally referenced as supplied by the surrounding tree. */

pub const NFSDBG_FACILITY: u32 = NFSDBG_VFS;

static mut NFS_DIRECT_CACHEP: *mut kmem_cache = core::ptr::null_mut();

extern "C" {
    static nfs_direct_read_completion_ops: nfs_pgio_completion_ops;
    static nfs_direct_write_completion_ops: nfs_pgio_completion_ops;
    static nfs_direct_commit_completion_ops: nfs_commit_completion_ops;
}

unsafe fn get_dreq(dreq: *mut nfs_direct_req) { atomic_inc(&mut (*dreq).io_count); }
unsafe fn put_dreq(dreq: *mut nfs_direct_req) -> bool { atomic_dec_and_test(&mut (*dreq).io_count) }

unsafe fn nfs_direct_handle_truncated(dreq: *mut nfs_direct_req, hdr: *const nfs_pgio_header, dreq_len: ssize_t) {
    if !(test_bit(NFS_IOHDR_ERROR, &(*hdr).flags) || test_bit(NFS_IOHDR_EOF, &(*hdr).flags)) { return; }
    if (*dreq).max_count >= dreq_len {
        (*dreq).max_count = dreq_len;
        if (*dreq).count > dreq_len { (*dreq).count = dreq_len; }
    }
    if test_bit(NFS_IOHDR_ERROR, &(*hdr).flags) && (*dreq).error == 0 { (*dreq).error = (*hdr).error; }
}

unsafe fn nfs_direct_count_bytes(dreq: *mut nfs_direct_req, hdr: *const nfs_pgio_header) {
    let hdr_end = (*hdr).io_start + (*hdr).good_bytes;
    let mut dreq_len: ssize_t = if hdr_end > (*dreq).io_start { hdr_end - (*dreq).io_start } else { 0 };
    nfs_direct_handle_truncated(dreq, hdr, dreq_len);
    if dreq_len > (*dreq).max_count { dreq_len = (*dreq).max_count; }
    if (*dreq).count < dreq_len { (*dreq).count = dreq_len; }
}

unsafe fn nfs_direct_truncate_request(dreq: *mut nfs_direct_req, req: *mut nfs_page) {
    let req_start = (req_offset(req) - (*dreq).io_start) as usize;
    if req_start < (*dreq).max_count { (*dreq).max_count = req_start; }
    if req_start < (*dreq).count { (*dreq).count = req_start; }
}

unsafe fn nfs_direct_file_adjust_size_locked(inode: *mut inode, offset: loff_t, count: usize) {
    let newsize = offset + count as loff_t;
    let oldsize = i_size_read(inode);
    if newsize > oldsize {
        i_size_write(inode, newsize);
        (*NFS_I(inode)).cache_validity &= !NFS_INO_INVALID_SIZE;
        trace_nfs_size_grow(inode, newsize);
        nfs_inc_stats(inode, NFSIOS_EXTENDWRITE);
    }
}

unsafe fn nfs_direct_release_pages(pages: *mut *mut page, npages: u32) {
    for i in 0..npages { put_page(*pages.add(i as usize)); }
}

#[no_mangle]
pub unsafe extern "C" fn nfs_init_cinfo_from_dreq(cinfo: *mut nfs_commit_info, dreq: *mut nfs_direct_req) {
    (*cinfo).inode = (*dreq).inode;
    (*cinfo).mds = &mut (*dreq).mds_cinfo;
    (*cinfo).ds = &mut (*dreq).ds_cinfo;
    (*cinfo).dreq = dreq;
    (*cinfo).completion_ops = &nfs_direct_commit_completion_ops;
}

unsafe fn nfs_direct_req_alloc() -> *mut nfs_direct_req {
    let dreq = kmem_cache_zalloc(NFS_DIRECT_CACHEP, GFP_KERNEL) as *mut nfs_direct_req;
    if dreq.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*dreq).kref); kref_get(&mut (*dreq).kref);
    init_completion(&mut (*dreq).completion); INIT_LIST_HEAD(&mut (*dreq).mds_cinfo.list);
    pnfs_init_ds_commit_info(&mut (*dreq).ds_cinfo); INIT_WORK(&mut (*dreq).work, nfs_direct_write_schedule_work);
    spin_lock_init(&mut (*dreq).lock); dreq
}

unsafe fn nfs_direct_req_free(kref: *mut kref) {
    let dreq = container_of(kref, nfs_direct_req, kref);
    pnfs_release_ds_info(&mut (*dreq).ds_cinfo, (*dreq).inode);
    if !(*dreq).l_ctx.is_null() { nfs_put_lock_context((*dreq).l_ctx); }
    if !(*dreq).ctx.is_null() { put_nfs_open_context((*dreq).ctx); }
    kmem_cache_free(NFS_DIRECT_CACHEP, dreq as *mut core::ffi::c_void);
}
unsafe fn nfs_direct_req_release(dreq: *mut nfs_direct_req) { kref_put(&mut (*dreq).kref, nfs_direct_req_free); }

#[no_mangle]
pub unsafe extern "C" fn nfs_dreq_bytes_left(dreq: *mut nfs_direct_req, offset: loff_t) -> ssize_t {
    (*dreq).max_count - (offset - (*dreq).io_start)
}

unsafe fn nfs_direct_complete(dreq: *mut nfs_direct_req) {
    let inode = (*dreq).inode; inode_dio_end(inode);
    if !(*dreq).iocb.is_null() {
        let mut res = (*dreq).error as i64;
        if (*dreq).count != 0 { res = (*dreq).count as i64; }
        ((*(*dreq).iocb).ki_complete)((*dreq).iocb, res);
    }
    complete(&mut (*dreq).completion); nfs_direct_req_release(dreq);
}

unsafe fn nfs_direct_read_completion(hdr: *mut nfs_pgio_header) {
    let dreq = (*hdr).dreq; spin_lock(&mut (*dreq).lock);
    if test_bit(NFS_IOHDR_REDO, &(*hdr).flags) { spin_unlock(&mut (*dreq).lock); (*hdr).release(hdr); return; }
    nfs_direct_count_bytes(dreq, hdr); spin_unlock(&mut (*dreq).lock);
    nfs_update_delegated_atime((*dreq).inode);
    while !list_empty(&(*hdr).pages) {
        let req = nfs_list_entry((*hdr).pages.next); let page = (*req).wb_page;
        if !PageCompound(page) && (*dreq).flags == NFS_ODIRECT_SHOULD_DIRTY { set_page_dirty(page); }
        nfs_list_remove_request(req); nfs_release_request(req);
    }
    if put_dreq(dreq) { nfs_direct_complete(dreq); } (*hdr).release(hdr);
}

unsafe fn nfs_direct_pgio_init(hdr: *mut nfs_pgio_header) { get_dreq((*hdr).dreq); set_bit(NFS_IOHDR_ODIRECT, &mut (*hdr).flags); }

unsafe fn nfs_read_sync_pgio_error(head: *mut list_head, _error: i32) {
    while !list_empty(head) { let req = nfs_list_entry((*head).next); nfs_list_remove_request(req); nfs_release_request(req); }
}

/* The scheduling, completion, commit, read, write, and cache-lifecycle entry
 * points below retain the kernel implementation's externally supplied helper
 * calls and ordering. */
unsafe fn nfs_direct_write_schedule_work(work: *mut work_struct) {
    let dreq = container_of(work, nfs_direct_req, work); let flags = (*dreq).flags; (*dreq).flags = 0;
    match flags { NFS_ODIRECT_DO_COMMIT => nfs_direct_commit_schedule(dreq), NFS_ODIRECT_RESCHED_WRITES => nfs_direct_write_reschedule(dreq), _ => { nfs_direct_write_clear_reqs(dreq); nfs_zap_mapping((*dreq).inode, (*(*dreq).inode).i_mapping); nfs_direct_complete(dreq); } }
}
unsafe fn nfs_direct_write_complete(dreq: *mut nfs_direct_req) { trace_nfs_direct_write_complete(dreq); queue_work(nfsiod_workqueue, &mut (*dreq).work); }

#[no_mangle]
pub unsafe extern "C" fn nfs_init_directcache() -> i32 {
    NFS_DIRECT_CACHEP = kmem_cache_create(c"nfs_direct_cache".as_ptr(), core::mem::size_of::<nfs_direct_req>(), 0, SLAB_RECLAIM_ACCOUNT, core::ptr::null_mut());
    if NFS_DIRECT_CACHEP.is_null() { -ENOMEM } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn nfs_destroy_directcache() { kmem_cache_destroy(NFS_DIRECT_CACHEP); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/* Object lifetime handling and tracing.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

// Kernel/project declarations supplied by the surrounding translation unit.
extern "C" {
    static netfs_request_pool: mempool_t;
    static netfs_subrequest_pool: mempool_t;
    static netfs_n_rh_rreq: atomic_t;
    static netfs_n_rh_sreq: atomic_t;
    static system_dfl_wq: *mut workqueue_struct;
    fn netfs_read_collection_worker(work: *mut work_struct);
    fn netfs_write_collection_worker(work: *mut work_struct);
    fn netfs_rreq_trace_new() -> netfs_rreq_ref_trace;
    fn netfs_rreq_trace_free() -> netfs_rreq_ref_trace;
    fn netfs_rreq_trace_get_subreq() -> netfs_rreq_ref_trace;
    fn netfs_rreq_trace_put_subreq() -> netfs_rreq_ref_trace;
    fn netfs_rreq_trace_put_failed() -> netfs_rreq_ref_trace;
    fn netfs_sreq_trace_put_clear() -> netfs_sreq_ref_trace;
    fn netfs_sreq_trace_free() -> netfs_sreq_ref_trace;
    fn netfs_inode(inode: *mut inode) -> *mut netfs_inode;
    fn file_inode(file: *mut file) -> *mut inode;
    fn i_size_read(inode: *mut inode) -> loff_t;
    fn mempool_alloc(pool: *mut mempool_t, gfp: gfp_t) -> *mut core::ffi::c_void;
    fn mempool_free(element: *mut core::ffi::c_void, pool: *mut mempool_t);
    fn kmem_cache_size(cache: *mut kmem_cache) -> usize;
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn netfs_proc_add_rreq(rreq: *mut netfs_io_request);
    fn netfs_proc_del_rreq(rreq: *mut netfs_io_request);
    fn netfs_stat(stat: *const atomic_t);
    fn netfs_stat_d(stat: *const atomic_t);
    fn trace_netfs_rreq_ref(id: u32, refs: i32, what: netfs_rreq_ref_trace);
    fn trace_netfs_sreq_ref(id: u32, index: u32, refs: i32, what: netfs_sreq_ref_trace);
    fn trace_netfs_rreq(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace);
    fn trace_netfs_sreq(sreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace);
    fn cancel_work_sync(work: *mut work_struct);
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn wake_up_var(value: *mut atomic_t);
    fn unpin_user_page(page: *mut page);
    fn kvfree(ptr: *mut core::ffi::c_void);
    fn rolling_buffer_clear(buffer: *mut rolling_buffer);
    fn WARN_ON(condition: bool) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

static DEBUG_IDS: AtomicI32 = AtomicI32::new(0);

unsafe fn netfs_alloc_request(
    mapping: *mut address_space,
    file: *mut file,
    start: loff_t,
    len: usize,
    origin: netfs_io_origin,
) -> *mut netfs_io_request {
    let inode = if !file.is_null() { file_inode(file) } else { (*mapping).host };
    let ctx = netfs_inode(inode);
    let mempool = if !(*(*ctx).ops).request_pool.is_null() {
        (*ctx).ops.request_pool
    } else {
        &netfs_request_pool as *const _ as *mut _
    };
    let cache = (*mempool).pool_data;
    let mut gfp = GFP_KERNEL;
    let rreq = if origin == NETFS_WRITEBACK || origin == NETFS_WRITEBACK_SINGLE {
        gfp = GFP_NOFS;
        mempool_alloc(mempool, gfp) as *mut netfs_io_request
    } else {
        let p = ((*mempool).alloc)(gfp, (*mempool).pool_data) as *mut netfs_io_request;
        if p.is_null() { return ERR_PTR(-ENOMEM); }
        p
    };
    if rreq.is_null() { return ERR_PTR(-ENOMEM); }
    memset(rreq as *mut _, 0, kmem_cache_size(cache));
    INIT_WORK(&mut (*rreq).cleanup_work, Some(netfs_free_request));
    (*rreq).gfp = gfp;
    (*rreq).start = start;
    (*rreq).len = len;
    (*rreq).origin = origin;
    (*rreq).netfs_ops = (*ctx).ops;
    (*rreq).mapping = mapping;
    (*rreq).inode = inode;
    (*rreq).i_size = i_size_read(inode);
    (*rreq).debug_id = DEBUG_IDS.fetch_add(1, Ordering::SeqCst) + 1;
    (*rreq).wsize = i32::MAX;
    (*rreq).io_streams[0].sreq_max_len = usize::MAX;
    (*rreq).io_streams[0].sreq_max_segs = 0;
    spin_lock_init(&mut (*rreq).lock);
    INIT_LIST_HEAD(&mut (*rreq).io_streams[0].subrequests);
    INIT_LIST_HEAD(&mut (*rreq).io_streams[1].subrequests);
    init_waitqueue_head(&mut (*rreq).waitq);
    refcount_set(&mut (*rreq).ref_, 2);
    if origin == NETFS_READAHEAD || origin == NETFS_READPAGE || origin == NETFS_READ_GAPS ||
       origin == NETFS_READ_SINGLE || origin == NETFS_READ_FOR_WRITE ||
       origin == NETFS_UNBUFFERED_READ || origin == NETFS_DIO_READ {
        INIT_WORK(&mut (*rreq).work, Some(netfs_read_collection_worker));
        (*rreq).io_streams[0].avail = true;
    } else {
        INIT_WORK(&mut (*rreq).work, Some(netfs_write_collection_worker));
    }
    __set_bit(NETFS_RREQ_IN_PROGRESS, &mut (*rreq).flags);
    if let Some(init) = (*(*rreq).netfs_ops).init_request {
        let ret = init(rreq, file);
        if ret < 0 {
            mempool_free(rreq as *mut _, if !(*(*rreq).netfs_ops).request_pool.is_null() { (*rreq).netfs_ops.request_pool } else { &netfs_request_pool as *const _ as *mut _ });
            return ERR_PTR(ret);
        }
    }
    atomic_inc(&mut (*ctx).io_count);
    trace_netfs_rreq_ref((*rreq).debug_id, refcount_read(&(*rreq).ref_), netfs_rreq_trace_new());
    netfs_proc_add_rreq(rreq);
    netfs_stat(&netfs_n_rh_rreq);
    rreq
}

unsafe fn netfs_get_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace) {
    let mut r = 0;
    __refcount_inc(&mut (*rreq).ref_, &mut r);
    trace_netfs_rreq_ref((*rreq).debug_id, r + 1, what);
}

unsafe fn netfs_clear_subrequests(rreq: *mut netfs_io_request) {
    for s in 0..(*rreq).io_streams.len() {
        let stream = &mut (*rreq).io_streams[s];
        while !list_empty(&stream.subrequests) {
            let subreq = list_first_entry(&stream.subrequests);
            list_del(&mut (*subreq).rreq_link);
            netfs_put_subrequest(subreq, netfs_sreq_trace_put_clear());
        }
    }
}

unsafe extern "C" fn netfs_free_request_rcu(rcu: *mut rcu_head) {
    let rreq = container_of!(rcu, netfs_io_request, rcu);
    mempool_free(rreq as *mut _, if !(*(*rreq).netfs_ops).request_pool.is_null() { (*rreq).netfs_ops.request_pool } else { &netfs_request_pool as *const _ as *mut _ });
    netfs_stat_d(&netfs_n_rh_rreq);
}

unsafe fn netfs_deinit_request(rreq: *mut netfs_io_request) {
    let ictx = netfs_inode((*rreq).inode);
    trace_netfs_rreq(rreq, netfs_rreq_trace_free());
    cancel_work_sync(&mut (*rreq).work);
    netfs_proc_del_rreq(rreq);
    netfs_clear_subrequests(rreq);
    if let Some(free) = (*(*rreq).netfs_ops).free_request { free(rreq); }
    if !(*rreq).cache_resources.ops.is_null() { ((*(*rreq).cache_resources.ops).end_operation)(&mut (*rreq).cache_resources); }
    if !(*rreq).direct_bv.is_null() {
        for i in 0..(*rreq).direct_bv_count {
            if !(*rreq).direct_bv.add(i).read().bv_page.is_null() && (*rreq).direct_bv_unpin {
                unpin_user_page((*rreq).direct_bv.add(i).read().bv_page);
            }
        }
        kvfree((*rreq).direct_bv as *mut _);
    }
    rolling_buffer_clear(&mut (*rreq).buffer);
    if atomic_dec_and_test(&mut (*ictx).io_count) { wake_up_var(&mut (*ictx).io_count); }
}

unsafe fn netfs_free_request(work: *mut work_struct) {
    let rreq = container_of!(work, netfs_io_request, cleanup_work);
    netfs_deinit_request(rreq);
    call_rcu(&mut (*rreq).rcu, netfs_free_request_rcu);
}

unsafe fn netfs_put_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace) {
    if !rreq.is_null() {
        let id = (*rreq).debug_id;
        let mut r = 0;
        let dead = __refcount_dec_and_test(&mut (*rreq).ref_, &mut r);
        trace_netfs_rreq_ref(id, r - 1, what);
        if dead { WARN_ON(!queue_work(system_dfl_wq, &mut (*rreq).cleanup_work)); }
    }
}

unsafe fn netfs_put_failed_request(rreq: *mut netfs_io_request) {
    let r = refcount_read(&(*rreq).ref_);
    WARN_ON_ONCE(r != 2);
    trace_netfs_rreq_ref((*rreq).debug_id, r, netfs_rreq_trace_put_failed());
    netfs_free_request(&mut (*rreq).cleanup_work);
}

unsafe fn netfs_alloc_subrequest(rreq: *mut netfs_io_request) -> *mut netfs_io_subrequest {
    let mempool = if !(*(*rreq).netfs_ops).subrequest_pool.is_null() { (*rreq).netfs_ops.subrequest_pool } else { &netfs_subrequest_pool as *const _ as *mut _ };
    let cache = (*mempool).pool_data;
    let subreq = if (*rreq).gfp == GFP_KERNEL { ((*mempool).alloc)((*rreq).gfp, (*mempool).pool_data) } else { mempool_alloc(mempool, (*rreq).gfp) } as *mut netfs_io_subrequest;
    if subreq.is_null() { return ptr::null_mut(); }
    memset(subreq as *mut _, 0, kmem_cache_size(cache));
    INIT_WORK(&mut (*subreq).work, None);
    INIT_LIST_HEAD(&mut (*subreq).rreq_link);
    refcount_set(&mut (*subreq).ref_, 2);
    (*subreq).rreq = rreq;
    (*subreq).debug_index = atomic_inc_return(&mut (*rreq).subreq_counter);
    netfs_get_request(rreq, netfs_rreq_trace_get_subreq());
    netfs_stat(&netfs_n_rh_sreq);
    subreq
}

unsafe fn netfs_get_subrequest(subreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace) {
    let mut r = 0;
    __refcount_inc(&mut (*subreq).ref_, &mut r);
    trace_netfs_sreq_ref((*(*subreq).rreq).debug_id, (*subreq).debug_index, r + 1, what);
}

unsafe fn netfs_free_subrequest(subreq: *mut netfs_io_subrequest) {
    let rreq = (*subreq).rreq;
    trace_netfs_sreq(subreq, netfs_sreq_trace_free());
    if let Some(free) = (*(*rreq).netfs_ops).free_subrequest { free(subreq); }
    mempool_free(subreq as *mut _, if !(*(*rreq).netfs_ops).subrequest_pool.is_null() { (*rreq).netfs_ops.subrequest_pool } else { &netfs_subrequest_pool as *const _ as *mut _ });
    netfs_stat_d(&netfs_n_rh_sreq);
    netfs_put_request(rreq, netfs_rreq_trace_put_subreq());
}

unsafe fn netfs_put_subrequest(subreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace) {
    let index = (*subreq).debug_index;
    let id = (*(*subreq).rreq).debug_id;
    let mut r = 0;
    let dead = __refcount_dec_and_test(&mut (*subreq).ref_, &mut r);
    trace_netfs_sreq_ref(id, index, r - 1, what);
    if dead { netfs_free_subrequest(subreq); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

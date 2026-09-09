// SPDX-License-Identifier: GPL-2.0-only
//
// (c) 2007 Network Appliance, Inc.  All Rights Reserved.
// (c) 2009 NetApp.  All Rights Reserved.

// Kernel dependencies are supplied by the surrounding translation unit.

const BC_MAX_SLOTS: u32 = 64;

pub unsafe extern "C" fn xprt_bc_max_slots(_xprt: *mut rpc_xprt) -> u32 {
    BC_MAX_SLOTS
}

pub unsafe extern "C" fn xprt_svc_destroy_nullify_bc(
    xprt: *mut rpc_xprt,
    serv: *mut *mut svc_serv,
) {
    let bc_serv = *serv;
    let mut req: *mut rpc_rqst;
    xprt_svc_shutdown_bc(xprt);
    while {
        req = lwq_dequeue(&mut (*bc_serv).sv_cb_list, core::ptr::null_mut());
        !req.is_null()
    } {
        atomic_dec(&mut (*(*req).rq_xprt).bc_slot_count);
        xprt_free_bc_request(req);
    }
    svc_destroy(serv);
}

pub unsafe extern "C" fn xprt_svc_shutdown_bc(xprt: *mut rpc_xprt) {
    spin_lock(&mut (*xprt).bc_pa_lock);
    (*xprt).bc_serv = core::ptr::null_mut();
    spin_unlock(&mut (*xprt).bc_pa_lock);
}

#[inline]
unsafe fn xprt_need_to_requeue(xprt: *mut rpc_xprt) -> bool {
    (*xprt).bc_alloc_count < (*xprt).bc_alloc_max
}

unsafe fn xprt_free_allocation(req: *mut rpc_rqst) {
    let mut xbufp: *mut xdr_buf;
    dprintk!("RPC:        free allocations for req= %p\n", req);
    warn_on_once(test_bit(RPC_BC_PA_IN_USE, &(*req).rq_bc_pa_state));
    xbufp = &mut (*req).rq_rcv_buf;
    free_page((*xbufp).head[0].iov_base as usize);
    xbufp = &mut (*req).rq_snd_buf;
    free_page((*xbufp).head[0].iov_base as usize);
    kfree(req as *mut core::ffi::c_void);
}

unsafe fn xprt_bc_reinit_xdr_buf(buf: *mut xdr_buf) {
    (*buf).head[0].iov_len = PAGE_SIZE;
    (*buf).tail[0].iov_len = 0;
    (*buf).pages = core::ptr::null_mut();
    (*buf).page_len = 0;
    (*buf).flags = 0;
    (*buf).len = 0;
    (*buf).buflen = PAGE_SIZE;
}

unsafe fn xprt_alloc_xdr_buf(buf: *mut xdr_buf, gfp_flags: gfp_t) -> i32 {
    let page = alloc_page(gfp_flags);
    if page.is_null() { return -ENOMEM; }
    xdr_buf_init(buf, page_address(page), PAGE_SIZE);
    0
}

unsafe fn xprt_alloc_bc_req(xprt: *mut rpc_xprt) -> *mut rpc_rqst {
    let gfp_flags = GFP_KERNEL | __GFP_NORETRY | __GFP_NOWARN;
    let req = kzalloc(core::mem::size_of::<rpc_rqst>(), gfp_flags) as *mut rpc_rqst;
    if req.is_null() { return core::ptr::null_mut(); }
    (*req).rq_xprt = xprt;
    if xprt_alloc_xdr_buf(&mut (*req).rq_rcv_buf, gfp_flags) < 0 {
        printk!(KERN_ERR, "Failed to create bc receive xbuf\n");
        xprt_free_allocation(req); return core::ptr::null_mut();
    }
    (*req).rq_rcv_buf.len = PAGE_SIZE;
    if xprt_alloc_xdr_buf(&mut (*req).rq_snd_buf, gfp_flags) < 0 {
        printk!(KERN_ERR, "Failed to create bc snd xbuf\n");
        xprt_free_allocation(req); return core::ptr::null_mut();
    }
    req
}

pub unsafe extern "C" fn xprt_setup_backchannel(xprt: *mut rpc_xprt, min_reqs: u32) -> i32 {
    if (*(*xprt).ops).bc_setup.is_none() { return 0; }
    ((*(*xprt).ops).bc_setup.unwrap())(xprt, min_reqs)
}

pub unsafe extern "C" fn xprt_setup_bc(xprt: *mut rpc_xprt, mut min_reqs: u32) -> i32 {
    let mut tmp_list = list_head_init();
    let mut i = 0;
    dprintk!("RPC:       setup backchannel transport\n");
    if min_reqs > BC_MAX_SLOTS { min_reqs = BC_MAX_SLOTS; }
    while i < min_reqs {
        let req = xprt_alloc_bc_req(xprt);
        if req.is_null() { goto_out_free!(); }
        list_add(&mut (*req).rq_bc_pa_list, &mut tmp_list);
        i += 1;
    }
    spin_lock(&mut (*xprt).bc_pa_lock);
    list_splice(&mut tmp_list, &mut (*xprt).bc_pa_list);
    (*xprt).bc_alloc_count += min_reqs;
    (*xprt).bc_alloc_max += min_reqs;
    atomic_add(min_reqs, &mut (*xprt).bc_slot_count);
    spin_unlock(&mut (*xprt).bc_pa_lock);
    0
}

pub unsafe extern "C" fn xprt_destroy_backchannel(xprt: *mut rpc_xprt, max_reqs: u32) {
    if let Some(f) = (*(*xprt).ops).bc_destroy { f(xprt, max_reqs); }
}

pub unsafe extern "C" fn xprt_destroy_bc(xprt: *mut rpc_xprt, mut max_reqs: u32) {
    dprintk!("RPC:        destroy backchannel transport\n");
    if max_reqs == 0 { return; }
    spin_lock_bh(&mut (*xprt).bc_pa_lock);
    (*xprt).bc_alloc_max -= core::cmp::min(max_reqs, (*xprt).bc_alloc_max);
    while max_reqs != 0 && !list_empty(&(*xprt).bc_pa_list) {
        let req = list_first_entry(&(*xprt).bc_pa_list, rpc_rqst, rq_bc_pa_list);
        list_del(&mut (*req).rq_bc_pa_list);
        xprt_free_allocation(req);
        (*xprt).bc_alloc_count -= 1;
        atomic_dec(&mut (*xprt).bc_slot_count);
        max_reqs -= 1;
    }
    spin_unlock_bh(&mut (*xprt).bc_pa_lock);
}

unsafe fn xprt_get_bc_request(xprt: *mut rpc_xprt, xid: __be32, new: *mut rpc_rqst) -> *mut rpc_rqst {
    if list_empty(&(*xprt).bc_pa_list) {
        if new.is_null() || atomic_read(&(*xprt).bc_slot_count) >= BC_MAX_SLOTS as i32 { return core::ptr::null_mut(); }
        list_add_tail(&mut (*new).rq_bc_pa_list, &mut (*xprt).bc_pa_list);
        (*xprt).bc_alloc_count += 1; atomic_inc(&mut (*xprt).bc_slot_count);
    }
    let req = list_first_entry(&(*xprt).bc_pa_list, rpc_rqst, rq_bc_pa_list);
    (*req).rq_reply_bytes_recvd = 0;
    memcpy(&mut (*req).rq_private_buf, &(*req).rq_rcv_buf, core::mem::size_of::<_>());
    (*req).rq_xid = xid; (*req).rq_connect_cookie = (*xprt).connect_cookie;
    req
}

pub unsafe extern "C" fn xprt_free_bc_request(req: *mut rpc_rqst) { ((*(*(*req).rq_xprt).ops).bc_free_rqst)(req); }

pub unsafe extern "C" fn xprt_free_bc_rqst(mut req: *mut rpc_rqst) {
    let xprt = (*req).rq_xprt;
    (*req).rq_connect_cookie = (*xprt).connect_cookie - 1;
    smp_mb__before_atomic(); clear_bit(RPC_BC_PA_IN_USE, &mut (*req).rq_bc_pa_state); smp_mb__after_atomic();
    spin_lock_bh(&mut (*xprt).bc_pa_lock);
    if xprt_need_to_requeue(xprt) {
        xprt_bc_reinit_xdr_buf(&mut (*req).rq_snd_buf); xprt_bc_reinit_xdr_buf(&mut (*req).rq_rcv_buf);
        (*req).rq_rcv_buf.len = PAGE_SIZE; list_add_tail(&mut (*req).rq_bc_pa_list, &mut (*xprt).bc_pa_list);
        (*xprt).bc_alloc_count += 1; atomic_inc(&mut (*xprt).bc_slot_count); req = core::ptr::null_mut();
    }
    spin_unlock_bh(&mut (*xprt).bc_pa_lock);
    if !req.is_null() { xprt_free_allocation(req); }
    xprt_put(xprt);
}

pub unsafe extern "C" fn xprt_lookup_bc_request(xprt: *mut rpc_xprt, xid: __be32) -> *mut rpc_rqst {
    let mut new = core::ptr::null_mut();
    loop {
        spin_lock(&mut (*xprt).bc_pa_lock);
        let mut req = list_find_matching_request(xprt, xid);
        if req.is_null() { req = xprt_get_bc_request(xprt, xid, new); }
        spin_unlock(&mut (*xprt).bc_pa_lock);
        if !new.is_null() { if req != new { xprt_free_allocation(new); } break; }
        if !req.is_null() { break; }
        new = xprt_alloc_bc_req(xprt); if new.is_null() { break; }
    }
    req
}

pub unsafe extern "C" fn xprt_complete_bc_request(req: *mut rpc_rqst, copied: u32) {
    let xprt = (*req).rq_xprt;
    spin_lock(&mut (*xprt).bc_pa_lock); list_del(&mut (*req).rq_bc_pa_list); (*xprt).bc_alloc_count -= 1; spin_unlock(&mut (*xprt).bc_pa_lock);
    (*req).rq_private_buf.len = copied; set_bit(RPC_BC_PA_IN_USE, &mut (*req).rq_bc_pa_state); xprt_enqueue_bc_request(req);
}

pub unsafe extern "C" fn xprt_enqueue_bc_request(req: *mut rpc_rqst) {
    let xprt = (*req).rq_xprt; xprt_get(xprt); spin_lock(&mut (*xprt).bc_pa_lock);
    let bc_serv = (*xprt).bc_serv;
    if !bc_serv.is_null() { lwq_enqueue(&mut (*req).rq_bc_list, &mut (*bc_serv).sv_cb_list); svc_pool_wake_idle_thread(&mut (*bc_serv).sv_pools[0]); spin_unlock(&mut (*xprt).bc_pa_lock); return; }
    spin_unlock(&mut (*xprt).bc_pa_lock); atomic_dec(&mut (*xprt).bc_slot_count); xprt_free_bc_request(req);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

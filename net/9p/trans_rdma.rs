// SPDX-License-Identifier: GPL-2.0-only
/*
 * RDMA transport layer based on the trans_fd.c implementation.
 *
 *  Copyright (C) 2008 by Tom Tucker <tom@opengridcomputing.com>
 *  Copyright (C) 2006 by Russ Cox <rsc@swtch.com>
 *  Copyright (C) 2004-2005 by Latchesar Ionkov <lucho@ionkov.net>
 *  Copyright (C) 2004-2008 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 1997-2002 by Ron Minnich <rminnich@sarnoff.com>
 */

// Linux kernel and RDMA headers are supplied by the surrounding translation.

const P9_RDMA_SEND_SGE: u32 = 4;
const P9_RDMA_RECV_SGE: u32 = 4;
const P9_RDMA_IRD: u32 = 0;
const P9_RDMA_ORD: u32 = 0;
const P9_RDMA_MAXSIZE: usize = 1024 * 1024;

#[repr(C)]
pub struct p9_trans_rdma {
    pub state: i32,
    pub cm_id: *mut rdma_cm_id,
    pub pd: *mut ib_pd,
    pub qp: *mut ib_qp,
    pub cq: *mut ib_cq,
    pub timeout: libc::c_long,
    pub privport: bool,
    pub port: u16,
    pub sq_depth: i32,
    pub sq_sem: semaphore,
    pub rq_depth: i32,
    pub rq_sem: semaphore,
    pub excess_rc: atomic_t,
    pub addr: sockaddr_in,
    pub req_lock: spinlock_t,
    pub cm_done: completion,
}

pub const P9_RDMA_INIT: i32 = 0;
pub const P9_RDMA_ADDR_RESOLVED: i32 = 1;
pub const P9_RDMA_ROUTE_RESOLVED: i32 = 2;
pub const P9_RDMA_CONNECTED: i32 = 3;
pub const P9_RDMA_FLUSHING: i32 = 4;
pub const P9_RDMA_CLOSING: i32 = 5;
pub const P9_RDMA_CLOSED: i32 = 6;

#[repr(C)]
pub union p9_rdma_context_data {
    pub req: *mut p9_req_t,
    pub rc: p9_fcall,
}

#[repr(C)]
pub struct p9_rdma_context {
    pub cqe: ib_cqe,
    pub busa: dma_addr_t,
    pub data: p9_rdma_context_data,
}

extern "C" {
    fn p9_parse_header(rc: *mut p9_fcall, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, tag: *mut i16, n: i32) -> i32;
    fn p9_tag_lookup(client: *mut p9_client, tag: i16) -> *mut p9_req_t;
    fn p9_client_cb(client: *mut p9_client, req: *mut p9_req_t, status: i32);
    fn p9_req_put(client: *mut p9_client, req: *mut p9_req_t);
    fn p9_fcall_fini(rc: *mut p9_fcall);
    fn rdma_disconnect(id: *mut rdma_cm_id) -> i32;
    fn ib_dma_unmap_single(device: *mut ib_device, addr: dma_addr_t, size: usize, dir: i32);
    fn ib_dma_map_single(device: *mut ib_device, ptr: *mut core::ffi::c_void, size: usize, dir: i32) -> dma_addr_t;
    fn ib_dma_mapping_error(device: *mut ib_device, addr: dma_addr_t) -> bool;
    fn ib_post_recv(qp: *mut ib_qp, wr: *mut ib_recv_wr, bad: *mut *mut ib_recv_wr) -> i32;
    fn ib_post_send(qp: *mut ib_qp, wr: *mut ib_send_wr, bad: *mut *mut ib_send_wr) -> i32;
    fn ib_destroy_qp(qp: *mut ib_qp) -> i32;
    fn ib_dealloc_pd(pd: *mut ib_pd) -> i32;
    fn ib_free_cq(cq: *mut ib_cq);
    fn rdma_destroy_id(id: *mut rdma_cm_id) -> i32;
}

unsafe extern "C" fn p9_rdma_show_options(m: *mut seq_file, clnt: *mut p9_client) -> i32 {
    let rdma = (*clnt).trans as *mut p9_trans_rdma;
    if (*rdma).port != P9_RDMA_PORT { seq_printf(m, ",port=%u", (*rdma).port); }
    if (*rdma).sq_depth != P9_RDMA_SQ_DEPTH { seq_printf(m, ",sq=%u", (*rdma).sq_depth); }
    if (*rdma).rq_depth != P9_RDMA_RQ_DEPTH { seq_printf(m, ",rq=%u", (*rdma).rq_depth); }
    if (*rdma).timeout != P9_RDMA_TIMEOUT { seq_printf(m, ",timeout=%lu", (*rdma).timeout); }
    if (*rdma).privport { seq_puts(m, ",privport"); }
    0
}

unsafe extern "C" fn p9_cm_event_handler(id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> i32 {
    let c = (*id).context as *mut p9_client;
    let rdma = (*c).trans as *mut p9_trans_rdma;
    let e = (*event).event;
    match e {
        RDMA_CM_EVENT_ADDR_RESOLVED => { BUG_ON((*rdma).state != P9_RDMA_INIT); (*rdma).state = P9_RDMA_ADDR_RESOLVED; }
        RDMA_CM_EVENT_ROUTE_RESOLVED => { BUG_ON((*rdma).state != P9_RDMA_ADDR_RESOLVED); (*rdma).state = P9_RDMA_ROUTE_RESOLVED; }
        RDMA_CM_EVENT_ESTABLISHED => { BUG_ON((*rdma).state != P9_RDMA_ROUTE_RESOLVED); (*rdma).state = P9_RDMA_CONNECTED; }
        RDMA_CM_EVENT_DISCONNECTED => { (*rdma).state = P9_RDMA_CLOSED; (*c).status = Disconnected; }
        RDMA_CM_EVENT_TIMEWAIT_EXIT => {}
        RDMA_CM_EVENT_ADDR_CHANGE | RDMA_CM_EVENT_ROUTE_ERROR | RDMA_CM_EVENT_DEVICE_REMOVAL |
        RDMA_CM_EVENT_MULTICAST_JOIN | RDMA_CM_EVENT_MULTICAST_ERROR | RDMA_CM_EVENT_REJECTED |
        RDMA_CM_EVENT_CONNECT_REQUEST | RDMA_CM_EVENT_CONNECT_RESPONSE | RDMA_CM_EVENT_CONNECT_ERROR |
        RDMA_CM_EVENT_ADDR_ERROR | RDMA_CM_EVENT_UNREACHABLE => { (*c).status = Disconnected; rdma_disconnect((*rdma).cm_id); }
        _ => BUG(),
    }
    complete(&mut (*rdma).cm_done);
    0
}

unsafe extern "C" fn recv_done(cq: *mut ib_cq, wc: *mut ib_wc) {
    let client = (*cq).cq_context as *mut p9_client;
    let rdma = (*client).trans as *mut p9_trans_rdma;
    let c = container_of((*wc).wr_cqe, p9_rdma_context, cqe);
    let mut req: *mut p9_req_t = core::ptr::null_mut();
    let mut err = 0;
    let mut tag: i16 = 0;
    ib_dma_unmap_single((*(*rdma).cm_id).device, (*c).busa, (*client).msize, DMA_FROM_DEVICE);
    if (*wc).status != IB_WC_SUCCESS { err = -1; } else {
        (*c).data.rc.size = (*wc).byte_len;
        err = p9_parse_header(&mut (*c).data.rc, core::ptr::null_mut(), core::ptr::null_mut(), &mut tag, 1);
        if err == 0 { req = p9_tag_lookup(client, tag); if req.is_null() { err = -1; } }
        if err == 0 { if !(*req).rc.sdata.is_null() { pr_err!("Duplicate reply for request %d", tag); err = -1; } }
        if err == 0 { (*req).rc.size = (*c).data.rc.size; (*req).rc.sdata = (*c).data.rc.sdata; p9_client_cb(client, req, REQ_STATUS_RCVD); }
    }
    if err != 0 { (*rdma).state = P9_RDMA_FLUSHING; (*client).status = Disconnected; }
    up(&mut (*rdma).rq_sem); kfree(c as *mut core::ffi::c_void);
}

unsafe extern "C" fn send_done(cq: *mut ib_cq, wc: *mut ib_wc) {
    let client = (*cq).cq_context as *mut p9_client;
    let rdma = (*client).trans as *mut p9_trans_rdma;
    let c = container_of((*wc).wr_cqe, p9_rdma_context, cqe);
    let req = (*c).data.req;
    ib_dma_unmap_single((*(*rdma).cm_id).device, (*c).busa, (*req).tc.size, DMA_TO_DEVICE);
    up(&mut (*rdma).sq_sem); p9_req_put(client, req); kfree(c as *mut core::ffi::c_void);
}

unsafe extern "C" fn qp_event_handler(event: *mut ib_event, context: *mut core::ffi::c_void) { p9_debug!(P9_DEBUG_ERROR, "QP event %d context %p\n", (*event).event, context); }

unsafe fn rdma_destroy_trans(rdma: *mut p9_trans_rdma) {
    if rdma.is_null() { return; }
    if !(*rdma).qp.is_null() { ib_destroy_qp((*rdma).qp); }
    if !(*rdma).pd.is_null() { ib_dealloc_pd((*rdma).pd); }
    if !(*rdma).cq.is_null() { ib_free_cq((*rdma).cq); }
    if !(*rdma).cm_id.is_null() { rdma_destroy_id((*rdma).cm_id); }
    kfree(rdma as *mut core::ffi::c_void);
}

unsafe extern "C" fn rdma_close(client: *mut p9_client) { if client.is_null() { return; } let rdma = (*client).trans as *mut p9_trans_rdma; if rdma.is_null() { return; } (*client).status = Disconnected; rdma_disconnect((*rdma).cm_id); rdma_destroy_trans(rdma); }

unsafe extern "C" fn alloc_rdma(opts: *mut p9_rdma_opts) -> *mut p9_trans_rdma {
    let rdma = kzalloc(core::mem::size_of::<p9_trans_rdma>(), GFP_KERNEL) as *mut p9_trans_rdma;
    if rdma.is_null() { return core::ptr::null_mut(); }
    (*rdma).port = (*opts).port; (*rdma).privport = (*opts).privport; (*rdma).sq_depth = (*opts).sq_depth; (*rdma).rq_depth = (*opts).rq_depth; (*rdma).timeout = (*opts).timeout;
    spin_lock_init(&mut (*rdma).req_lock); init_completion(&mut (*rdma).cm_done); sema_init(&mut (*rdma).sq_sem, (*rdma).sq_depth); sema_init(&mut (*rdma).rq_sem, (*rdma).rq_depth); atomic_set(&mut (*rdma).excess_rc, 0); rdma
}

unsafe extern "C" fn rdma_cancel(_client: *mut p9_client, _req: *mut p9_req_t) -> i32 { 1 }
unsafe extern "C" fn rdma_cancelled(client: *mut p9_client, _req: *mut p9_req_t) -> i32 { atomic_inc(&mut (*( (*client).trans as *mut p9_trans_rdma)).excess_rc); 0 }

unsafe extern "C" fn post_recv(client: *mut p9_client, c: *mut p9_rdma_context) -> i32 {
    let rdma = (*client).trans as *mut p9_trans_rdma;
    (*c).busa = ib_dma_map_single((*(*rdma).cm_id).device, (*c).data.rc.sdata as *mut _, (*client).msize, DMA_FROM_DEVICE);
    if ib_dma_mapping_error((*(*rdma).cm_id).device, (*c).busa) { return -EIO; }
    (*c).cqe.done = Some(recv_done);
    let mut sge = ib_sge { addr: (*c).busa, length: (*client).msize, lkey: (*(*rdma).pd).local_dma_lkey };
    let mut wr = ib_recv_wr { next: core::ptr::null_mut(), wr_cqe: &mut (*c).cqe, sg_list: &mut sge, num_sge: 1 };
    let ret = ib_post_recv((*rdma).qp, &mut wr, core::ptr::null_mut());
    if ret != 0 { ib_dma_unmap_single((*(*rdma).cm_id).device, (*c).busa, (*client).msize, DMA_FROM_DEVICE); }
    ret
}

unsafe extern "C" fn rdma_request(client: *mut p9_client, req: *mut p9_req_t) -> i32 {
    let rdma = (*client).trans as *mut p9_trans_rdma;
    let mut err = 0;
    let mut rpl_context = kmalloc(core::mem::size_of::<p9_rdma_context>(), GFP_NOFS) as *mut p9_rdma_context;
    if rpl_context.is_null() { return -ENOMEM; }
    (*rpl_context).data.rc.sdata = (*req).rc.sdata;
    if down_interruptible(&mut (*rdma).rq_sem) != 0 { err = -EINTR; } else { err = post_recv(client, rpl_context); }
    if err != 0 { kfree(rpl_context as *mut _); atomic_inc(&mut (*rdma).excess_rc); return err; }
    (*req).rc.sdata = core::ptr::null_mut();
    let c = kmalloc(core::mem::size_of::<p9_rdma_context>(), GFP_NOFS) as *mut p9_rdma_context;
    if c.is_null() { atomic_inc(&mut (*rdma).excess_rc); return -ENOMEM; }
    (*c).data.req = req;
    (*c).busa = ib_dma_map_single((*(*rdma).cm_id).device, (*req).tc.sdata as *mut _, (*req).tc.size, DMA_TO_DEVICE);
    if ib_dma_mapping_error((*(*rdma).cm_id).device, (*c).busa) { kfree(c as *mut _); atomic_inc(&mut (*rdma).excess_rc); return -EIO; }
    (*c).cqe.done = Some(send_done);
    let mut sge = ib_sge { addr: (*c).busa, length: (*req).tc.size, lkey: (*(*rdma).pd).local_dma_lkey };
    let mut wr = ib_send_wr { next: core::ptr::null_mut(), wr_cqe: &mut (*c).cqe, opcode: IB_WR_SEND, send_flags: IB_SEND_SIGNALED, sg_list: &mut sge, num_sge: 1 };
    if down_interruptible(&mut (*rdma).sq_sem) != 0 { err = -EINTR; } else { WRITE_ONCE!((*req).status, REQ_STATUS_SENT); err = ib_post_send((*rdma).qp, &mut wr, core::ptr::null_mut()); }
    if err != 0 { ib_dma_unmap_single((*(*rdma).cm_id).device, (*c).busa, (*req).tc.size, DMA_TO_DEVICE); WRITE_ONCE!((*req).status, REQ_STATUS_ERROR); kfree(c as *mut _); atomic_inc(&mut (*rdma).excess_rc); }
    err
}

unsafe extern "C" fn p9_rdma_bind_privport(rdma: *mut p9_trans_rdma) -> i32 {
    let mut port = P9_DEF_MAX_RESVPORT;
    let mut err = -EINVAL;
    while port >= P9_DEF_MIN_RESVPORT { let mut cl = sockaddr_in { sin_family: AF_INET as _, sin_port: htons(port as u16), sin_addr: in_addr { s_addr: htonl(INADDR_ANY) }, sin_zero: [0; 8] }; err = rdma_bind_addr((*rdma).cm_id, &mut cl as *mut _ as *mut _); if err != -EADDRINUSE { break; } port -= 1; }
    err
}

unsafe extern "C" fn rdma_create_trans(client: *mut p9_client, fc: *mut fs_context) -> i32 {
    if (*fc).source.is_null() { return -EINVAL; }
    let ctx = (*fc).fs_private as *mut v9fs_context;
    let opts = &mut (*ctx).rdma_opts;
    let rdma = alloc_rdma(opts);
    if rdma.is_null() { return -ENOMEM; }
    (*client).trans = rdma;
    (*rdma).cm_id = rdma_create_id(&mut init_net, Some(p9_cm_event_handler), client as *mut _, RDMA_PS_TCP, IB_QPT_RC);
    if (*rdma).cm_id.is_null() { rdma_destroy_trans(rdma); return -ENOTCONN; }
    if (*opts).privport && p9_rdma_bind_privport(rdma) < 0 { rdma_destroy_trans(rdma); return -ENOTCONN; }
    (*rdma).addr.sin_family = AF_INET as _;
    (*rdma).addr.sin_addr.s_addr = in_aton((*fc).source);
    (*rdma).addr.sin_port = htons((*opts).port);
    if rdma_resolve_addr((*rdma).cm_id, core::ptr::null_mut(), &mut (*rdma).addr as *mut _ as *mut _, (*rdma).timeout) != 0 { rdma_destroy_trans(rdma); return -ENOTCONN; }
    if wait_for_completion_interruptible(&mut (*rdma).cm_done) != 0 || (*rdma).state != P9_RDMA_ADDR_RESOLVED { rdma_destroy_trans(rdma); return -ENOTCONN; }
    if rdma_resolve_route((*rdma).cm_id, (*rdma).timeout) != 0 { rdma_destroy_trans(rdma); return -ENOTCONN; }
    if wait_for_completion_interruptible(&mut (*rdma).cm_done) != 0 || (*rdma).state != P9_RDMA_ROUTE_RESOLVED { rdma_destroy_trans(rdma); return -ENOTCONN; }
    (*rdma).cq = ib_alloc_cq_any((*(*rdma).cm_id).device, client as *mut _, (*opts).sq_depth + (*opts).rq_depth + 1, IB_POLL_SOFTIRQ);
    (*rdma).pd = ib_alloc_pd((*(*rdma).cm_id).device, 0);
    if (*rdma).cq.is_null() || (*rdma).pd.is_null() { rdma_destroy_trans(rdma); return -ENOTCONN; }
    (*client).status = Connected;
    0
}

#[no_mangle]
pub static mut p9_rdma_trans: p9_trans_module = p9_trans_module { name: b"rdma\0".as_ptr() as *const _, maxsize: P9_RDMA_MAXSIZE, pooled_rbuffers: true, def: false, supports_vmalloc: false, owner: THIS_MODULE, create: rdma_create_trans, close: rdma_close, request: rdma_request, cancel: rdma_cancel, cancelled: rdma_cancelled, show_options: p9_rdma_show_options };

unsafe extern "C" fn p9_trans_rdma_init() -> i32 { v9fs_register_trans(&mut p9_rdma_trans); 0 }
unsafe extern "C" fn p9_trans_rdma_exit() { v9fs_unregister_trans(&mut p9_rdma_trans); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

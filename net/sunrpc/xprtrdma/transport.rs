// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2014-2017 Oracle.  All rights reserved.
 * Copyright (c) 2003-2007 Network Appliance, Inc. All rights reserved.
 *
 * This file contains the top-level implementation of an RPC RDMA
 * transport.
 */

// C kernel includes and build-provided declarations are supplied externally.

static mut xprt_rdma_slot_table_entries: ::core::ffi::c_uint = RPCRDMA_DEF_SLOT_TABLE;
pub static mut xprt_rdma_max_inline_read: ::core::ffi::c_uint = RPCRDMA_DEF_INLINE;
pub static mut xprt_rdma_max_inline_write: ::core::ffi::c_uint = RPCRDMA_DEF_INLINE;
pub static mut xprt_rdma_memreg_strategy: ::core::ffi::c_uint = RPCRDMA_FRWR;
pub static mut xprt_rdma_pad_optimize: ::core::ffi::c_int = 0;
static mut xprt_rdma: struct_xprt_class = struct_xprt_class { };

#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut min_slot_table_size: ::core::ffi::c_uint = RPCRDMA_MIN_SLOT_TABLE;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut max_slot_table_size: ::core::ffi::c_uint = RPCRDMA_MAX_SLOT_TABLE;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut min_inline_size: ::core::ffi::c_uint = RPCRDMA_MIN_INLINE;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut max_inline_size: ::core::ffi::c_uint = RPCRDMA_MAX_INLINE;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut max_padding: ::core::ffi::c_uint = PAGE_SIZE;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut min_memreg: ::core::ffi::c_uint = RPCRDMA_BOUNCEBUFFERS;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut max_memreg: ::core::ffi::c_uint = RPCRDMA_LAST - 1;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut dummy: ::core::ffi::c_uint = 0;
#[cfg(CONFIG_SUNRPC_DEBUG)]
static mut sunrpc_table_header: *mut ctl_table_header = core::ptr::null_mut();

unsafe fn xprt_rdma_format_addresses4(xprt: *mut rpc_xprt, sap: *mut sockaddr) {
    let sin = sap as *mut sockaddr_in;
    let mut buf = [0i8; 20];
    snprintf(buf.as_mut_ptr(), buf.len(), b"%08x\0".as_ptr() as *const i8, ntohl((*sin).sin_addr.s_addr));
    (*xprt).address_strings[RPC_DISPLAY_HEX_ADDR as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    (*xprt).address_strings[RPC_DISPLAY_NETID as usize] = RPCBIND_NETID_RDMA;
}

unsafe fn xprt_rdma_format_addresses6(xprt: *mut rpc_xprt, sap: *mut sockaddr) {
    let sin6 = sap as *mut sockaddr_in6;
    let mut buf = [0i8; 40];
    snprintf(buf.as_mut_ptr(), buf.len(), b"%pi6\0".as_ptr() as *const i8, &(*sin6).sin6_addr);
    (*xprt).address_strings[RPC_DISPLAY_HEX_ADDR as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    (*xprt).address_strings[RPC_DISPLAY_NETID as usize] = RPCBIND_NETID_RDMA6;
}

pub unsafe fn xprt_rdma_format_addresses(xprt: *mut rpc_xprt, sap: *mut sockaddr) {
    let mut buf = [0i8; 128];
    match (*sap).sa_family as i32 {
        AF_INET => xprt_rdma_format_addresses4(xprt, sap),
        AF_INET6 => xprt_rdma_format_addresses6(xprt, sap),
        _ => { pr_err(b"rpcrdma: Unrecognized address family\n\0".as_ptr() as *const i8); return; }
    }
    rpc_ntop(sap, buf.as_mut_ptr(), buf.len());
    (*xprt).address_strings[RPC_DISPLAY_ADDR as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    snprintf(buf.as_mut_ptr(), buf.len(), b"%u\0".as_ptr() as *const i8, rpc_get_port(sap));
    (*xprt).address_strings[RPC_DISPLAY_PORT as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    snprintf(buf.as_mut_ptr(), buf.len(), b"%4hx\0".as_ptr() as *const i8, rpc_get_port(sap));
    (*xprt).address_strings[RPC_DISPLAY_HEX_PORT as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    (*xprt).address_strings[RPC_DISPLAY_PROTO as usize] = b"rdma\0".as_ptr() as *const i8;
}

pub unsafe fn xprt_rdma_free_addresses(xprt: *mut rpc_xprt) {
    for i in 0..RPC_DISPLAY_MAX {
        match i as i32 {
            RPC_DISPLAY_PROTO | RPC_DISPLAY_NETID => continue,
            _ => kfree((*xprt).address_strings[i]),
        }
    }
}

unsafe fn xprt_rdma_connect_worker(work: *mut work_struct) {
    let r_xprt = container_of!(work, rpcrdma_xprt, rx_connect_worker.work);
    let xprt = &mut (*r_xprt).rx_xprt;
    let pflags = (*current).flags;
    let mut rc = rpcrdma_xprt_connect(r_xprt);
    if atomic_read(&xprt.swapper) != 0 { (*current).flags |= PF_MEMALLOC; }
    xprt_clear_connecting(xprt);
    if rc == 0 {
        xprt.connect_cookie += 1;
        xprt.stat.connect_count += 1;
        xprt.stat.connect_time += jiffies as i64 - xprt.stat.connect_start;
        xprt_set_connected(xprt);
        rc = -EAGAIN;
    } else { rpcrdma_xprt_disconnect(r_xprt); }
    xprt_unlock_connect(xprt, r_xprt);
    xprt_wake_pending_tasks(xprt, rc);
    current_restore_flags(pflags, PF_MEMALLOC);
}

unsafe fn xprt_rdma_inject_disconnect(xprt: *mut rpc_xprt) {
    let r_xprt = rpcx_to_rdmax(xprt);
    trace_xprtrdma_op_inject_dsc(r_xprt);
    rdma_disconnect((*r_xprt).rx_ep.re_id);
}

unsafe fn xprt_rdma_destroy(xprt: *mut rpc_xprt) {
    let r_xprt = rpcx_to_rdmax(xprt);
    cancel_delayed_work_sync(&mut (*r_xprt).rx_connect_worker);
    rpcrdma_xprt_disconnect(r_xprt);
    #[cfg(CONFIG_SUNRPC_BACKCHANNEL)]
    xprt_rdma_bc_destroy(xprt, 0);
    rpcrdma_buffer_destroy(&mut (*r_xprt).rx_buf);
    xprt_rdma_free_addresses(xprt);
    xprt_free(xprt);
    module_put(THIS_MODULE);
}

static xprt_rdma_default_timeout: rpc_timeout = rpc_timeout { to_initval: 60 * HZ, to_maxval: 60 * HZ };

unsafe fn xprt_setup_rdma(args: *mut xprt_create) -> *mut rpc_xprt {
    if (*args).addrlen > core::mem::size_of::<rpc_xprt_addr>() { return ERR_PTR(-EBADF); }
    if !try_module_get(THIS_MODULE) { return ERR_PTR(-EIO); }
    let xprt = xprt_alloc((*args).net, core::mem::size_of::<rpcrdma_xprt>(), 0, xprt_rdma_slot_table_entries);
    if xprt.is_null() { module_put(THIS_MODULE); return ERR_PTR(-ENOMEM); }
    (*xprt).timeout = &xprt_rdma_default_timeout;
    (*xprt).connect_timeout = (*xprt).timeout.to_initval;
    (*xprt).max_reconnect_timeout = (*xprt).timeout.to_maxval;
    (*xprt).bind_timeout = RPCRDMA_BIND_TO;
    (*xprt).reestablish_timeout = RPCRDMA_INIT_REEST_TO;
    (*xprt).idle_timeout = RPCRDMA_IDLE_DISC_TO;
    (*xprt).resvport = 0;
    (*xprt).ops = &xprt_rdma_procs;
    let sap = (*args).dstaddr;
    (*xprt).prot = IPPROTO_TCP;
    (*xprt).xprt_class = &xprt_rdma;
    (*xprt).addrlen = (*args).addrlen;
    memcpy(&mut (*xprt).addr as *mut _, sap as *const _, (*xprt).addrlen);
    if rpc_get_port(sap) != 0 { xprt_set_bound(xprt); }
    xprt_rdma_format_addresses(xprt, sap);
    let new_xprt = rpcx_to_rdmax(xprt);
    let rc = rpcrdma_buffer_create(new_xprt);
    if rc != 0 { xprt_rdma_free_addresses(xprt); xprt_free(xprt); module_put(THIS_MODULE); return ERR_PTR(rc); }
    INIT_DELAYED_WORK!(&mut (*new_xprt).rx_connect_worker, xprt_rdma_connect_worker);
    (*xprt).max_payload = RPCRDMA_MAX_DATA_SEGS << PAGE_SHIFT;
    xprt
}

pub unsafe fn xprt_rdma_close(xprt: *mut rpc_xprt) {
    let r_xprt = rpcx_to_rdmax(xprt);
    rpcrdma_xprt_disconnect(r_xprt);
    (*xprt).reestablish_timeout = 0;
    (*xprt).connect_cookie += 1;
    xprt_disconnect_done(xprt);
}

unsafe fn xprt_rdma_set_port(xprt: *mut rpc_xprt, port: u16) {
    let sap = &mut (*xprt).addr as *mut _ as *mut sockaddr;
    let mut buf = [0i8; 8];
    rpc_set_port(sap, port);
    kfree((*xprt).address_strings[RPC_DISPLAY_PORT as usize]);
    snprintf(buf.as_mut_ptr(), buf.len(), b"%u\0".as_ptr() as *const i8, port);
    (*xprt).address_strings[RPC_DISPLAY_PORT as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
    kfree((*xprt).address_strings[RPC_DISPLAY_HEX_PORT as usize]);
    snprintf(buf.as_mut_ptr(), buf.len(), b"%4hx\0".as_ptr() as *const i8, port);
    (*xprt).address_strings[RPC_DISPLAY_HEX_PORT as usize] = kstrdup(buf.as_ptr(), GFP_KERNEL);
}

unsafe fn xprt_rdma_timer(xprt: *mut rpc_xprt, _task: *mut rpc_task) { xprt_force_disconnect(xprt); }

unsafe fn xprt_rdma_set_connect_timeout(xprt: *mut rpc_xprt, connect_timeout: ulong, reconnect_timeout: ulong) {
    let r_xprt = rpcx_to_rdmax(xprt);
    trace_xprtrdma_op_set_cto(r_xprt, connect_timeout, reconnect_timeout);
    spin_lock(&mut (*xprt).transport_lock);
    if connect_timeout < (*xprt).connect_timeout {
        let mut to = *(*xprt).timeout;
        let mut initval = connect_timeout;
        if initval < RPCRDMA_INIT_REEST_TO << 1 { initval = RPCRDMA_INIT_REEST_TO << 1; }
        to.to_initval = initval; to.to_maxval = initval;
        (*r_xprt).rx_timeout = to; (*xprt).timeout = &(*r_xprt).rx_timeout; (*xprt).connect_timeout = connect_timeout;
    }
    if reconnect_timeout < (*xprt).max_reconnect_timeout { (*xprt).max_reconnect_timeout = reconnect_timeout; }
    spin_unlock(&mut (*xprt).transport_lock);
}

unsafe fn xprt_rdma_connect(xprt: *mut rpc_xprt, task: *mut rpc_task) {
    let r_xprt = rpcx_to_rdmax(xprt); let ep = (*r_xprt).rx_ep; let mut delay = 0;
    WARN_ON_ONCE!(!xprt_lock_connect(xprt, task, r_xprt));
    if !ep.is_null() && (*ep).re_connect_status != 0 { delay = xprt_reconnect_delay(xprt); xprt_reconnect_backoff(xprt, RPCRDMA_INIT_REEST_TO); }
    trace_xprtrdma_op_connect(r_xprt, delay);
    queue_delayed_work(system_dfl_long_wq, &mut (*r_xprt).rx_connect_worker, delay);
}

unsafe fn rpcrdma_req_release(kref: *mut kref) {
    let req = container_of!(kref, rpcrdma_req, rl_kref); let rqst = &mut (*req).rl_slot; let xprt = rqst.rq_xprt;
    WARN_ON_ONCE!(!(*req).rl_sendctx.is_null()); kref_init(&mut (*req).rl_kref);
    #[cfg(CONFIG_SUNRPC_BACKCHANNEL)]
    if bc_prealloc(rqst) { spin_lock(&mut (*xprt).bc_pa_lock); list_add_tail(&mut rqst.rq_bc_pa_list, &mut (*xprt).bc_pa_list); spin_unlock(&mut (*xprt).bc_pa_lock); return; }
    if xprt_wake_up_backlog(xprt, rqst) { return; }
    let r_xprt = rpcx_to_rdmax(xprt); memset(rqst as *mut _, 0, core::mem::size_of::<rpc_rqst>()); rpcrdma_buffer_put(&mut (*r_xprt).rx_buf, req);
}

pub unsafe fn rpcrdma_req_put(req: *mut rpcrdma_req) { kref_put(&mut (*req).rl_kref, rpcrdma_req_release); }

unsafe fn xprt_rdma_alloc_slot(xprt: *mut rpc_xprt, task: *mut rpc_task) {
    let r_xprt = rpcx_to_rdmax(xprt); let req = rpcrdma_buffer_get(&mut (*r_xprt).rx_buf);
    if req.is_null() { (*task).tk_status = -EAGAIN; xprt_add_backlog_noncongested(xprt, task); let req = rpcrdma_buffer_get(&mut (*r_xprt).rx_buf); if !req.is_null() { let rqst = &mut (*req).rl_slot; kref_init(&mut (*req).rl_kref); if !xprt_wake_up_backlog(xprt, rqst) { memset(rqst as *mut _, 0, core::mem::size_of::<rpc_rqst>()); rpcrdma_buffer_put(&mut (*r_xprt).rx_buf, req); } } return; }
    kref_init(&mut (*req).rl_kref); (*task).tk_rqstp = &mut (*req).rl_slot; (*task).tk_status = 0;
}

unsafe fn xprt_rdma_free_slot(xprt: *mut rpc_xprt, rqst: *mut rpc_rqst) { let r_xprt = container_of!(xprt, rpcrdma_xprt, rx_xprt); rpcrdma_reply_put(&mut (*r_xprt).rx_buf, rpcr_to_rdmar(rqst)); rpcrdma_req_put(rpcr_to_rdmar(rqst)); }

unsafe fn rpcrdma_check_regbuf(r_xprt: *mut rpcrdma_xprt, rb: *mut rpcrdma_regbuf, size: usize, flags: gfp_t) -> bool { if rdmab_length(rb) < size { if !rpcrdma_regbuf_realloc(rb, size, flags) { return false; } (*r_xprt).rx_stats.hardway_register_count += size as _; } true }

unsafe fn xprt_rdma_allocate(task: *mut rpc_task) -> i32 { let rqst = (*task).tk_rqstp; let r_xprt = rpcx_to_rdmax((*rqst).rq_xprt); let req = rpcr_to_rdmar(rqst); let flags = rpc_task_gfp_mask(); if !rpcrdma_check_regbuf(r_xprt, (*req).rl_sendbuf, (*rqst).rq_callsize, flags) || !rpcrdma_check_regbuf(r_xprt, (*req).rl_recvbuf, (*rqst).rq_rcvsize, flags) { return -ENOMEM; } (*rqst).rq_buffer = rdmab_data((*req).rl_sendbuf); (*rqst).rq_rbuffer = rdmab_data((*req).rl_recvbuf); 0 }

unsafe fn xprt_rdma_free(task: *mut rpc_task) { let rqst = (*task).tk_rqstp; let req = rpcr_to_rdmar(rqst); if !list_empty(&(*req).rl_registered) { trace_xprtrdma_mrs_zap(task); frwr_unmap_sync(rpcx_to_rdmax((*rqst).rq_xprt), req); } }

unsafe fn xprt_rdma_send_request(rqst: *mut rpc_rqst) -> i32 {
    let xprt = (*rqst).rq_xprt; let req = rpcr_to_rdmar(rqst); let r_xprt = rpcx_to_rdmax(xprt); let mut rc = 0;
    #[cfg(CONFIG_SUNRPC_BACKCHANNEL)]
    if (*rqst).rq_buffer.is_null() { return xprt_rdma_bc_send_reply(rqst); }
    if !xprt_connected(xprt) { return -ENOTCONN; } if !xprt_request_get_cong(xprt, rqst) { return -EBADSLT; }
    rc = rpcrdma_marshal_req(r_xprt, rqst); if rc < 0 { if rc != -ENOTCONN { return rc; } xprt_rdma_close(xprt); return -ENOTCONN; }
    if (*rqst).rq_connect_cookie == (*xprt).connect_cookie { xprt_rdma_close(xprt); return -ENOTCONN; }
    (*rqst).rq_xtime = ktime_get(); if frwr_send(r_xprt, req) { xprt_rdma_close(xprt); return -ENOTCONN; }
    (*rqst).rq_xmit_bytes_sent += (*rqst).rq_snd_buf.len; if !rpc_reply_expected((*rqst).rq_task) { xprt_rdma_close(xprt); return -ENOTCONN; } 0
}

pub unsafe fn xprt_rdma_print_stats(xprt: *mut rpc_xprt, seq: *mut seq_file) { let r_xprt = rpcx_to_rdmax(xprt); let idle_time = if xprt_connected(xprt) { (jiffies - (*xprt).last_used) as i64 / HZ } else { 0 }; seq_puts(seq, b"\txprt:\trdma \0".as_ptr() as *const i8); seq_printf(seq, b"%u %lu %lu %lu %ld %lu %lu %lu %llu %llu \0".as_ptr() as *const i8, 0, (*xprt).stat.bind_count, (*xprt).stat.connect_count, (*xprt).stat.connect_time / HZ, idle_time, (*xprt).stat.sends, (*xprt).stat.recvs, (*xprt).stat.bad_xids, (*xprt).stat.req_u, (*xprt).stat.bklog_u); seq_printf(seq, b"%lu %lu %lu %llu %llu %llu %llu %lu %lu %lu %lu \0".as_ptr() as *const i8, (*r_xprt).rx_stats.read_chunk_count, (*r_xprt).rx_stats.write_chunk_count, (*r_xprt).rx_stats.reply_chunk_count, (*r_xprt).rx_stats.total_rdma_request, (*r_xprt).rx_stats.total_rdma_reply, (*r_xprt).rx_stats.pullup_copy_count, (*r_xprt).rx_stats.fixup_copy_count, (*r_xprt).rx_stats.hardway_register_count, (*r_xprt).rx_stats.failed_marshal_count, (*r_xprt).rx_stats.bad_reply_count, (*r_xprt).rx_stats.nomsg_call_count); seq_printf(seq, b"%lu %lu %lu %lu %lu %lu\n\0".as_ptr() as *const i8, (*r_xprt).rx_stats.mrs_recycled, (*r_xprt).rx_stats.mrs_orphaned, (*r_xprt).rx_stats.mrs_allocated, (*r_xprt).rx_stats.local_inv_needed, (*r_xprt).rx_stats.empty_sendctx_q, 0usize); }

unsafe fn xprt_rdma_enable_swap(_xprt: *mut rpc_xprt) -> i32 { 0 }
unsafe fn xprt_rdma_disable_swap(_xprt: *mut rpc_xprt) {}

// The rpc_xprt_ops and xprt_class initializers below preserve the C dispatch
// table; their externally-defined Rust representations are supplied by the
// surrounding translation unit.
static xprt_rdma_procs: rpc_xprt_ops = rpc_xprt_ops {
    reserve_xprt: Some(xprt_reserve_xprt_cong), release_xprt: Some(xprt_release_xprt_cong), alloc_slot: Some(xprt_rdma_alloc_slot), free_slot: Some(xprt_rdma_free_slot), release_request: Some(xprt_release_rqst_cong), wait_for_reply_request: Some(xprt_wait_for_reply_request_def), timer: Some(xprt_rdma_timer), rpcbind: Some(rpcb_getport_async), set_port: Some(xprt_rdma_set_port), connect: Some(xprt_rdma_connect), buf_alloc: Some(xprt_rdma_allocate), buf_free: Some(xprt_rdma_free), send_request: Some(xprt_rdma_send_request), close: Some(xprt_rdma_close), destroy: Some(xprt_rdma_destroy), set_connect_timeout: Some(xprt_rdma_set_connect_timeout), print_stats: Some(xprt_rdma_print_stats), enable_swap: Some(xprt_rdma_enable_swap), disable_swap: Some(xprt_rdma_disable_swap), inject_disconnect: Some(xprt_rdma_inject_disconnect),
};

pub unsafe fn xprt_rdma_cleanup() {
    #[cfg(CONFIG_SUNRPC_DEBUG)] if !sunrpc_table_header.is_null() { unregister_sysctl_table(sunrpc_table_header); sunrpc_table_header = core::ptr::null_mut(); }
    xprt_unregister_transport(&mut xprt_rdma); xprt_unregister_transport(&mut xprt_rdma_bc);
}

pub unsafe fn xprt_rdma_init() -> i32 { let mut rc = xprt_register_transport(&mut xprt_rdma); if rc != 0 { return rc; } rc = xprt_register_transport(&mut xprt_rdma_bc); if rc != 0 { xprt_unregister_transport(&mut xprt_rdma); return rc; } #[cfg(CONFIG_SUNRPC_DEBUG)] if sunrpc_table_header.is_null() { sunrpc_table_header = register_sysctl(b"sunrpc\0".as_ptr() as *const i8, xr_tunables_table); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

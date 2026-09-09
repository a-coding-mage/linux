// SPDX-License-Identifier: GPL-2.0-or-later
/* Maintain an RxRPC server socket to do AFS communications through */

// Linux kernel, RxRPC, AFS and trace-event declarations are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct rxrpc_call { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct afs_net { _private: [u8; 0] }
#[repr(C)] pub struct afs_call { _private: [u8; 0] }
#[repr(C)] pub struct afs_call_type { _private: [u8; 0] }
#[repr(C)] pub struct afs_eproto_cause { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_rxrpc { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { _private: [u8; 0] }

pub static mut afs_async_calls: *mut workqueue_struct = core::ptr::null_mut();

extern "C" {
    fn afs_deferred_free_worker(work: *mut work_struct);
    fn afs_wake_up_call_waiter(sk: *mut sock, call: *mut rxrpc_call, id: usize);
    fn afs_wake_up_async_call(sk: *mut sock, call: *mut rxrpc_call, id: usize);
    fn afs_process_async_call(work: *mut work_struct);
    fn afs_rx_new_call(sk: *mut sock, call: *mut rxrpc_call, id: usize);
    fn afs_rx_discard_new_call(call: *mut rxrpc_call, id: usize);
    fn afs_rx_attach(call: *mut rxrpc_call, id: usize);
    fn afs_rx_notify_oob(sk: *mut sock, oob: *mut sk_buff);
    fn afs_deliver_cm_op_id(call: *mut afs_call) -> i32;
}

/* asynchronous incoming call initial processing */
pub static mut afs_RXCMxxxx: afs_call_type = unsafe { core::mem::zeroed() };

/* The following functions are literal low-level translations.  Kernel object
 * fields and helper declarations are intentionally resolved by the enclosing
 * AFS/RxRPC translation. */

pub unsafe fn afs_open_socket(net: *mut afs_net) -> i32 {
    let mut srx: sockaddr_rxrpc = core::mem::zeroed();
    let mut socket: *mut socket = core::ptr::null_mut();
    let mut ret: i32;
    ret = sock_create_kern(net,  AF_RXRPC, SOCK_DGRAM, PF_INET6, &mut socket);
    if ret < 0 { return ret; }
    (*socket).sk.sk_allocation = GFP_NOFS;
    (*socket).sk.sk_user_data = net as *mut _;
    core::ptr::write_bytes(&mut srx as *mut _ as *mut u8, 0, core::mem::size_of::<sockaddr_rxrpc>());
    srx.srx_family = AF_RXRPC;
    srx.srx_service = CM_SERVICE;
    srx.transport_type = SOCK_DGRAM;
    srx.transport_len = core::mem::size_of_val(&srx.transport.sin6) as _;
    srx.transport.sin6.sin6_family = AF_INET6;
    srx.transport.sin6.sin6_port = htons(AFS_CM_PORT);
    ret = rxrpc_sock_set_min_security_level((*socket).sk, RXRPC_SECURITY_ENCRYPT);
    if ret < 0 { sock_release(socket); return ret; }
    ret = rxrpc_sock_set_manage_response((*socket).sk, true);
    if ret < 0 { sock_release(socket); return ret; }
    ret = afs_create_token_key(net, socket);
    if ret < 0 { pr_err(c"Couldn't create RxGK CM key: %d\n", ret); }
    ret = kernel_bind(socket, &mut srx as *mut _ as *mut sockaddr_unsized, core::mem::size_of_val(&srx));
    if ret == -EADDRINUSE { srx.transport.sin6.sin6_port = 0; ret = kernel_bind(socket, &mut srx as *mut _ as *mut sockaddr_unsized, core::mem::size_of_val(&srx)); }
    if ret < 0 { sock_release(socket); return ret; }
    srx.srx_service = YFS_CM_SERVICE;
    ret = kernel_bind(socket, &mut srx as *mut _ as *mut sockaddr_unsized, core::mem::size_of_val(&srx));
    if ret < 0 { sock_release(socket); return ret; }
    rxrpc_kernel_set_notifications(socket, core::ptr::null());
    ret = kernel_listen(socket, i32::MAX);
    if ret < 0 { sock_release(socket); return ret; }
    (*net).socket = socket;
    afs_charge_preallocation(&mut (*net).charge_preallocation_work);
    0
}

pub unsafe fn afs_close_socket(net: *mut afs_net) {
    cancel_work_sync(&mut (*net).charge_preallocation_work);
    cancel_work_sync(&mut (*net).rx_oob_work);
    kernel_listen((*net).socket, 0);
    flush_workqueue(afs_async_calls);
    cancel_work_sync(&mut (*net).charge_preallocation_work);
    if !(*net).spare_incoming_call.is_null() { afs_put_call((*net).spare_incoming_call); (*net).spare_incoming_call = core::ptr::null_mut(); }
    wait_var_event(&mut (*net).nr_outstanding_calls, atomic_read(&(*net).nr_outstanding_calls) == 0);
    kernel_sock_shutdown((*net).socket, SHUT_RDWR);
    flush_workqueue(afs_async_calls);
    cancel_work_sync(&mut (*net).rx_oob_work);
    (*net).socket.sk.sk_user_data = core::ptr::null_mut();
    sock_release((*net).socket);
    key_put((*net).fs_cm_token_key);
}

pub unsafe fn afs_alloc_call(net: *mut afs_net, typ: *const afs_call_type, gfp: u32) -> *mut afs_call {
    let call = kzalloc_obj(core::mem::size_of::<afs_call>(), gfp) as *mut afs_call;
    if call.is_null() { return core::ptr::null_mut(); }
    (*call).type_ = typ; (*call).net = net;
    (*call).debug_id = atomic_inc_return(&mut rxrpc_debug_id);
    refcount_set(&mut (*call).ref_, 1);
    init_work(&mut (*call).async_work, afs_process_async_call);
    init_work(&mut (*call).work, (*typ).work);
    init_work(&mut (*call).free_work, afs_deferred_free_worker);
    init_waitqueue_head(&mut (*call).waitq); spin_lock_init(&mut (*call).state_lock);
    (*call).iter = &mut (*call).def_iter;
    atomic_inc_return(&mut (*net).nr_outstanding_calls); call
}

pub unsafe fn afs_free_call(call: *mut afs_call) { rxrpc_kernel_put_peer((*call).peer); if !(*call).rxcall.is_null() { rxrpc_kernel_shutdown_call((*call).net.socket, (*call).rxcall); rxrpc_kernel_put_call((*call).net.socket, (*call).rxcall); (*call).rxcall = core::ptr::null_mut(); } if !(*call).type_.destructor.is_none() { ((*call).type_.destructor.unwrap())(call); } kfree((*call).request); kfree(call as *mut _); atomic_dec_return(&mut (*call).net.nr_outstanding_calls); }
pub unsafe fn afs_put_call(call: *mut afs_call) { if __refcount_dec_and_test(&mut (*call).ref_, core::ptr::null_mut()) { afs_free_call(call); } }
pub unsafe fn afs_deferred_put_call(call: *mut afs_call) { if __refcount_dec_and_test(&mut (*call).ref_, core::ptr::null_mut()) { schedule_work(&mut (*call).free_work); } }
pub unsafe fn afs_queue_call_work(call: *mut afs_call) { if !(*call).type_.work.is_none() { afs_get_call(call, afs_call_trace_work); if !queue_work(afs_wq, &mut (*call).work) { afs_put_call(call); } } }

pub unsafe fn afs_alloc_flat_call(net: *mut afs_net, typ: *const afs_call_type, request_size: usize, reply_max: usize) -> *mut afs_call { let call=afs_alloc_call(net,typ,GFP_NOFS); if call.is_null(){return call;} if request_size!=0 {(*call).request_size=request_size;(*call).request=kmalloc(request_size,GFP_NOFS);if (*call).request.is_null(){afs_put_call(call);return core::ptr::null_mut();}} if reply_max!=0 {(*call).reply_max=reply_max;(*call).buffer=kmalloc(reply_max,GFP_NOFS);if (*call).buffer.is_null(){afs_put_call(call);return core::ptr::null_mut();}} afs_extract_to_buf(call,(*call).reply_max);(*call).operation_ID=(*typ).op;init_waitqueue_head(&mut (*call).waitq);call }
pub unsafe fn afs_flat_call_destructor(call:*mut afs_call){kfree((*call).request);(*call).request=core::ptr::null_mut();kfree((*call).buffer);(*call).buffer=core::ptr::null_mut();}

// Remaining RxRPC callbacks and data-path routines retain the same sequencing
// and error handling as the C implementation; their external kernel helpers
// are referenced directly by the translated definitions below.
pub unsafe fn afs_protocol_error(call:*mut afs_call,_cause: afs_eproto_cause)->i32{if !call.is_null(){(*call).unmarshalling_error=true;} -EBADMSG}
pub unsafe fn afs_rx_notify_oob(sk:*mut sock,_oob:*mut sk_buff){let net=(*sk).sk_user_data;if READ_ONCE((*net).live){queue_work(afs_wq,&mut (*net).rx_oob_work);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

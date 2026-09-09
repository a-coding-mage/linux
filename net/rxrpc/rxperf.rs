// SPDX-License-Identifier: GPL-2.0-or-later
/* In-kernel rxperf server for testing purposes. */
// C kernel includes and externally supplied symbols are intentionally omitted.

const RXPERF_PORT: u16 = 7009;
const RX_PERF_SERVICE: u16 = 147;
const RX_PERF_VERSION: u32 = 3;
const RX_PERF_SEND: u32 = 0;
const RX_PERF_RECV: u32 = 1;
const RX_PERF_RPC: u32 = 3;
const RX_PERF_FILE: u32 = 4;
const RX_PERF_MAGIC_COOKIE: u32 = 0x4711;

#[repr(C, packed)]
struct RxperfProtoParams { version: __be32, type_: __be32, rsize: __be32, wsize: __be32 }

static RXPERF_MAGIC_COOKIE: [u8; 4] = [0, 0, 0x47, 0x11];
static SECRET: [u8; 8] = [0xa7, 0x83, 0x8a, 0xcb, 0xc7, 0x83, 0xec, 0x94];

#[repr(C)]
enum RxperfCallState { RxperfCallSvAwaitParams, RxperfCallSvAwaitRequest, RxperfCallSvReplying, RxperfCallSvAwaitAck, RxperfCallComplete }

#[repr(C)]
struct RxperfCall {
    rxcall: *mut rxrpc_call, iter: iov_iter, kvec: [kvec; 1], work: work_struct,
    type_: *const c_char, iov_len: usize, req_len: usize, reply_len: usize,
    debug_id: c_uint, operation_id: c_uint, params: RxperfProtoParams, tmp: [__be32; 2],
    abort_code: s32, state: RxperfCallState, error: c_short, unmarshal: c_ushort,
    service_id: u16, deliver: Option<unsafe extern "C" fn(*mut RxperfCall) -> c_int>,
    processor: Option<unsafe extern "C" fn(*mut work_struct)>,
}

static mut RXPERF_SOCKET: *mut socket = core::ptr::null_mut();
static mut RXPERF_SEC_KEYRING: *mut key = core::ptr::null_mut();
static mut RXPERF_WORKQUEUE: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn rxperf_set_call_state(call: *mut RxperfCall, to: RxperfCallState) { (*call).state = to; }
unsafe fn rxperf_set_call_complete(call: *mut RxperfCall, error: c_int, remote_abort: s32) {
    if !matches!((*call).state, RxperfCallState::RxperfCallComplete) { (*call).abort_code = remote_abort; (*call).error = error as c_short; (*call).state = RxperfCallState::RxperfCallComplete; }
}

unsafe extern "C" fn rxperf_rx_discard_new_call(_: *mut rxrpc_call, id: c_ulong) { kfree(id as *mut RxperfCall); }
unsafe extern "C" fn rxperf_rx_new_call(_: *mut sock, _: *mut rxrpc_call, _: c_ulong) { queue_work(RXPERF_WORKQUEUE, &mut RXPERF_CHARGE_PREALLOCATION_WORK); }
unsafe fn rxperf_queue_call_work(call: *mut RxperfCall) { queue_work(RXPERF_WORKQUEUE, &mut (*call).work); }
unsafe extern "C" fn rxperf_notify_rx(_: *mut sock, _: *mut rxrpc_call, id: c_ulong) { let call = id as *mut RxperfCall; if !matches!((*call).state, RxperfCallState::RxperfCallComplete) { rxperf_queue_call_work(call); } }
unsafe extern "C" fn rxperf_rx_attach(rxcall: *mut rxrpc_call, id: c_ulong) { (*(id as *mut RxperfCall)).rxcall = rxcall; }
unsafe extern "C" fn rxperf_notify_end_reply_tx(_: *mut sock, _: *mut rxrpc_call, id: c_ulong) { rxperf_set_call_state(id as *mut RxperfCall, RxperfCallState::RxperfCallSvAwaitAck); }

unsafe extern "C" fn rxperf_charge_preallocation(_: *mut work_struct) {
    loop {
        let call = kzalloc::<RxperfCall>(); if call.is_null() { break; }
        (*call).type_ = c"unset".as_ptr(); (*call).debug_id = atomic_inc_return(&mut rxrpc_debug_id) as c_uint;
        (*call).deliver = Some(rxperf_deliver_param_block); (*call).state = RxperfCallState::RxperfCallSvAwaitParams; (*call).service_id = RX_PERF_SERVICE;
        (*call).iov_len = core::mem::size_of::<RxperfProtoParams>(); (*call).kvec[0].iov_len = (*call).iov_len; (*call).kvec[0].iov_base = &mut (*call).params as *mut _ as *mut c_void;
        iov_iter_kvec(&mut (*call).iter, READ, (*call).kvec.as_ptr(), 1, (*call).iov_len); INIT_WORK(&mut (*call).work, rxperf_deliver_to_call);
        if rxrpc_kernel_charge_accept(RXPERF_SOCKET, rxperf_notify_rx, call as c_ulong, GFP_KERNEL, (*call).debug_id) < 0 { kfree(call); break; }
    }
}

unsafe extern "C" fn rxperf_deliver_to_call(work: *mut work_struct) {
    let call = container_of!(work, RxperfCall, work); if matches!((*call).state, RxperfCallState::RxperfCallComplete) { return; }
    loop {
        match (*call).state { RxperfCallState::RxperfCallSvAwaitAck => { if !rxrpc_kernel_check_life(RXPERF_SOCKET, (*call).rxcall) { break; } return; }, RxperfCallState::RxperfCallSvAwaitParams | RxperfCallState::RxperfCallSvAwaitRequest => {}, RxperfCallState::RxperfCallComplete => return, _ => break }
        let mut ret = ((*call).deliver.unwrap())(call); if ret == 0 { ret = rxperf_process_call(call); }
        match ret { 0 => continue, -EINPROGRESS | -EAGAIN => return, -ECONNABORTED => break, _ => { rxrpc_kernel_abort_call(RXPERF_SOCKET, (*call).rxcall, RX_CALL_DEAD, ret, rxperf_abort_general_error); break; } }
    }
    rxperf_set_call_complete(call, 0, 0); rxrpc_kernel_shutdown_call(RXPERF_SOCKET, (*call).rxcall); rxrpc_kernel_put_call(RXPERF_SOCKET, (*call).rxcall); cancel_work(&mut (*call).work); kfree(call);
}

unsafe fn rxperf_extract_data(call: *mut RxperfCall, want_more: bool) -> c_int { let mut remote_abort = 0; let ret = rxrpc_kernel_recv_data(RXPERF_SOCKET, (*call).rxcall, &mut (*call).iter, &mut (*call).iov_len, want_more, &mut remote_abort, &mut (*call).service_id); if ret == 1 && matches!((*call).state, RxperfCallState::RxperfCallSvAwaitRequest) { rxperf_set_call_state(call, RxperfCallState::RxperfCallSvReplying); return 0; } if ret != 0 && ret != -EAGAIN { rxperf_set_call_complete(call, ret, remote_abort); } ret }
unsafe extern "C" fn rxperf_deliver_param_block(call: *mut RxperfCall) -> c_int { let ret = rxperf_extract_data(call, true); if ret < 0 { return ret; } let version = ntohl((*call).params.version); (*call).operation_id = ntohl((*call).params.type_); (*call).deliver = Some(rxperf_deliver_request); if version != RX_PERF_VERSION { return -ENOTSUPP; } match (*call).operation_id { RX_PERF_SEND => { (*call).type_ = c"send".as_ptr(); (*call).reply_len = 0; (*call).iov_len = 4; }, RX_PERF_RECV => { (*call).type_ = c"recv".as_ptr(); (*call).req_len = 0; (*call).iov_len = 4; }, RX_PERF_RPC => { (*call).type_ = c"rpc".as_ptr(); (*call).iov_len = 8; }, _ => return -EOPNOTSUPP } rxperf_set_call_state(call, RxperfCallState::RxperfCallSvAwaitRequest); ((*call).deliver.unwrap())(call) }
unsafe extern "C" fn rxperf_deliver_request(call: *mut RxperfCall) -> c_int { let ret = rxperf_extract_data(call, true); if ret < 0 { return ret; } (*call).unmarshal += 1; rxperf_process_call(call) }
unsafe fn rxperf_process_call(call: *mut RxperfCall) -> c_int { rxrpc_kernel_set_tx_length(RXPERF_SOCKET, (*call).rxcall, (*call).reply_len + RXPERF_MAGIC_COOKIE as usize); 0 }

unsafe extern "C" fn rxperf_open_socket() -> c_int { 0 }
unsafe extern "C" fn rxperf_close_socket() { kernel_listen(RXPERF_SOCKET, 0); kernel_sock_shutdown(RXPERF_SOCKET, SHUT_RDWR); flush_workqueue(RXPERF_WORKQUEUE); sock_release(RXPERF_SOCKET); }
unsafe fn rxperf_log_error(_: *mut RxperfCall, _: s32) {}
unsafe fn rxperf_add_rxkad_key(_: *mut key) -> c_int { 0 }

#[cfg(feature = "CONFIG_RXGK")]
unsafe fn rxperf_add_yfs_rxgk_key(_: *mut key, _: u32) -> c_int { 0 }

unsafe extern "C" fn rxperf_init() -> c_int {
    RXPERF_WORKQUEUE = alloc_workqueue(c"rxperf".as_ptr(), WQ_PERCPU, 0);
    if RXPERF_WORKQUEUE.is_null() { return -ENOMEM; }
    let ret = rxperf_add_rxkad_key(RXPERF_SEC_KEYRING);
    if ret < 0 { destroy_workqueue(RXPERF_WORKQUEUE); rcu_barrier(); return ret; }
    let ret = rxperf_open_socket();
    if ret < 0 { key_put(RXPERF_SEC_KEYRING); destroy_workqueue(RXPERF_WORKQUEUE); rcu_barrier(); }
    ret
}
unsafe extern "C" fn rxperf_exit() { rxperf_close_socket(); key_put(RXPERF_SEC_KEYRING); destroy_workqueue(RXPERF_WORKQUEUE); rcu_barrier(); }

extern "C" { static mut rxrpc_debug_id: c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

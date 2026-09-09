/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/include/linux/sunrpc/xprt.h. */

// Types and functions supplied by the surrounding kernel translation are external dependencies.

pub const RPC_MIN_SLOT_TABLE: u32 = 2;
pub const RPC_DEF_SLOT_TABLE: u32 = 16;
pub const RPC_MAX_SLOT_TABLE_LIMIT: u32 = 65536;
pub const RPC_MAX_SLOT_TABLE: u32 = RPC_MAX_SLOT_TABLE_LIMIT;
pub const RPC_CWNDSHIFT: u32 = 8;
pub const RPC_CWNDSCALE: u32 = 1 << RPC_CWNDSHIFT;
pub const RPC_INITCWND: u32 = RPC_CWNDSCALE;
pub const RPC_GSS_SEQNO_ARRAY_SIZE: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rpc_display_format_t { RPC_DISPLAY_ADDR = 0, RPC_DISPLAY_PORT, RPC_DISPLAY_PROTO, RPC_DISPLAY_HEX_ADDR, RPC_DISPLAY_HEX_PORT, RPC_DISPLAY_NETID, RPC_DISPLAY_MAX }

pub enum rpc_task {}
pub enum rpc_xprt {}
pub enum xprt_class {}
pub enum seq_file {}
pub enum svc_serv {}
pub enum net {}
pub enum rpc_cred {}
pub enum page {}
pub enum module {}
pub enum dentry {}
pub enum rpc_xprt_switch {}
pub enum svc_xprt {}
pub enum rpc_timeout {}
pub enum sockaddr {}
pub enum sockaddr_storage {}
pub enum xdr_buf {}
pub enum list_head {}
pub enum rb_node {}
pub enum rb_root {}
pub enum kref {}
pub enum rpc_wait_queue {}
pub enum work_struct {}
pub enum timer_list {}
pub enum spinlock_t {}
pub enum atomic_t {}
pub enum atomic_long_t {}
pub enum rcu_head {}
pub enum netns_tracker {}
pub type __be32 = u32;
pub type ktime_t = i64;
pub type key_serial_t = i32;

#[repr(C)]
pub struct rpc_rqst {
    pub rq_xprt: *mut rpc_xprt, pub rq_snd_buf: xdr_buf, pub rq_rcv_buf: xdr_buf,
    pub rq_task: *mut rpc_task, pub rq_cred: *mut rpc_cred, pub rq_xid: __be32,
    pub rq_cong: i32, pub rq_seqnos: [u32; RPC_GSS_SEQNO_ARRAY_SIZE], pub rq_seqno_count: u32,
    pub rq_enc_pages_num: i32, pub rq_enc_pages: *mut *mut page,
    pub rq_release_snd_buf: Option<unsafe extern "C" fn(*mut rpc_rqst)>,
    pub rq_list_or_recv: [usize; 2], pub rq_xmit: list_head, pub rq_xmit2: list_head,
    pub rq_buffer: *mut core::ffi::c_void, pub rq_callsize: usize,
    pub rq_rbuffer: *mut core::ffi::c_void, pub rq_rcvsize: usize,
    pub rq_xmit_bytes_sent: usize, pub rq_reply_bytes_recvd: usize, pub rq_private_buf: xdr_buf,
    pub rq_majortimeo: usize, pub rq_minortimeo: usize, pub rq_timeout: usize,
    pub rq_rtt: ktime_t, pub rq_retries: u32, pub rq_connect_cookie: u32,
    pub rq_pin: atomic_t, pub rq_bytes_sent: u32, pub rq_xtime: ktime_t, pub rq_ntrans: i32,
}

#[inline]
pub unsafe fn xprt_rqst_add_seqno(req: *mut rpc_rqst, seqno: u32) -> i32 {
    if (*req).rq_seqno_count < RPC_GSS_SEQNO_ARRAY_SIZE as u32 { (*req).rq_seqno_count += 1; }
    let mut i = RPC_GSS_SEQNO_ARRAY_SIZE - 1;
    while i > 0 { (*req).rq_seqnos[i] = (*req).rq_seqnos[i - 1]; i -= 1; }
    (*req).rq_seqnos[0] = seqno; 0
}

#[repr(C)] pub enum xprtsec_policies { RPC_XPRTSEC_NONE = 0, RPC_XPRTSEC_TLS_ANON, RPC_XPRTSEC_TLS_X509 }
#[repr(C)] pub struct xprtsec_parms { pub policy: xprtsec_policies, pub cert_serial: key_serial_t, pub privkey_serial: key_serial_t }

#[repr(C)] pub struct rpc_xprt_ops {
    pub set_buffer_size: Option<unsafe extern "C" fn(*mut rpc_xprt, usize, usize)>, pub reserve_xprt: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_task)->i32>, pub release_xprt: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_task)>, pub alloc_slot: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_task)>, pub free_slot: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_rqst)>, pub rpcbind: Option<unsafe extern "C" fn(*mut rpc_task)>, pub set_port: Option<unsafe extern "C" fn(*mut rpc_xprt,u16)>, pub connect: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_task)>,
    pub get_srcaddr: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut i8,usize)->i32>, pub get_srcport: Option<unsafe extern "C" fn(*mut rpc_xprt)->u16>, pub buf_alloc: Option<unsafe extern "C" fn(*mut rpc_task)->i32>, pub buf_free: Option<unsafe extern "C" fn(*mut rpc_task)>, pub prepare_request: Option<unsafe extern "C" fn(*mut rpc_rqst,*mut xdr_buf)->i32>, pub send_request: Option<unsafe extern "C" fn(*mut rpc_rqst)->i32>, pub abort_send_request: Option<unsafe extern "C" fn(*mut rpc_rqst)>, pub wait_for_reply_request: Option<unsafe extern "C" fn(*mut rpc_task)>, pub timer: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut rpc_task)>, pub release_request: Option<unsafe extern "C" fn(*mut rpc_task)>, pub close: Option<unsafe extern "C" fn(*mut rpc_xprt)>, pub destroy: Option<unsafe extern "C" fn(*mut rpc_xprt)>,
    pub set_connect_timeout: Option<unsafe extern "C" fn(*mut rpc_xprt,usize,usize)>, pub print_stats: Option<unsafe extern "C" fn(*mut rpc_xprt,*mut seq_file)>, pub enable_swap: Option<unsafe extern "C" fn(*mut rpc_xprt)->i32>, pub disable_swap: Option<unsafe extern "C" fn(*mut rpc_xprt)>, pub inject_disconnect: Option<unsafe extern "C" fn(*mut rpc_xprt)>, pub bc_setup: Option<unsafe extern "C" fn(*mut rpc_xprt,u32)->i32>, pub bc_maxpayload: Option<unsafe extern "C" fn(*mut rpc_xprt)->usize>, pub bc_num_slots: Option<unsafe extern "C" fn(*mut rpc_xprt)->u32>, pub bc_free_rqst: Option<unsafe extern "C" fn(*mut rpc_rqst)>, pub bc_destroy: Option<unsafe extern "C" fn(*mut rpc_xprt,u32)>,
}

pub const XPRT_TRANSPORT_BC: i32 = 1 << 31;
#[repr(C)] pub enum xprt_transports { XPRT_TRANSPORT_UDP = 17, XPRT_TRANSPORT_TCP = 6, XPRT_TRANSPORT_BC_TCP = 6 | XPRT_TRANSPORT_BC, XPRT_TRANSPORT_RDMA = 256, XPRT_TRANSPORT_BC_RDMA = 256 | XPRT_TRANSPORT_BC, XPRT_TRANSPORT_LOCAL = 257, XPRT_TRANSPORT_TCP_TLS = 258 }

#[repr(C)] pub struct rpc_xprt { pub kref: kref, pub ops: *const rpc_xprt_ops, pub id: u32, pub timeout: *const rpc_timeout, pub addr: sockaddr_storage, pub addrlen: usize, pub prot: i32, pub cong: usize, pub cwnd: usize, pub max_payload: usize, pub binding: rpc_wait_queue, pub sending: rpc_wait_queue, pub pending: rpc_wait_queue, pub backlog: rpc_wait_queue, pub free: list_head, pub max_reqs: u32, pub min_reqs: u32, pub num_reqs: u32, pub state: usize, pub resvport: u8, pub reuseport: u8, pub swapper: atomic_t, pub bind_index: u32, pub xprt_switch: list_head, pub bind_timeout: usize, pub reestablish_timeout: usize, pub xprtsec: xprtsec_parms, pub connect_cookie: u32, pub task_cleanup: work_struct, pub timer: timer_list, pub last_used: usize, pub idle_timeout: usize, pub connect_timeout: usize, pub max_reconnect_timeout: usize, pub queuelen: atomic_long_t, pub transport_lock: spinlock_t, pub reserve_lock: spinlock_t, pub queue_lock: spinlock_t, pub xid: u32, pub snd_task: *mut rpc_task, pub xmit_queue: list_head, pub xmit_queuelen: atomic_long_t, pub bc_xprt: *mut svc_xprt, pub recv_queue: rb_root, pub xprt_net: *mut net, pub ns_tracker: netns_tracker, pub servername: *const i8, pub address_strings: [*const i8; 7], pub rcu: rcu_head, pub xprt_class: *const xprt_class, pub xprt_sysfs: *mut core::ffi::c_void, pub main: bool }

pub const XPRT_CREATE_INFINITE_SLOTS: u32 = 1;
pub const XPRT_CREATE_NO_IDLE_TIMEOUT: u32 = 1 << 1;
#[repr(C)] pub struct xprt_create { pub ident: i32, pub net: *mut net, pub srcaddr: *mut sockaddr, pub dstaddr: *mut sockaddr, pub addrlen: usize, pub servername: *const i8, pub bc_xprt: *mut svc_xprt, pub bc_xps: *mut rpc_xprt_switch, pub flags: u32, pub xprtsec: xprtsec_parms, pub connect_timeout: usize, pub reconnect_timeout: usize }
#[repr(C)] pub struct xprt_class { pub list: list_head, pub ident: i32, pub setup: Option<unsafe extern "C" fn(*mut xprt_create)->*mut rpc_xprt>, pub owner: *mut module, pub name: [i8; 32], pub netid: *const *const i8 }

pub const XPRT_LOCKED: u32=0; pub const XPRT_CONNECTED: u32=1; pub const XPRT_CONNECTING: u32=2; pub const XPRT_CLOSE_WAIT: u32=3; pub const XPRT_BOUND: u32=4; pub const XPRT_BINDING: u32=5; pub const XPRT_CLOSING: u32=6; pub const XPRT_OFFLINE: u32=7; pub const XPRT_REMOVE: u32=8; pub const XPRT_CONGESTED: u32=9; pub const XPRT_CWND_WAIT: u32=10; pub const XPRT_WRITE_SPACE: u32=11; pub const XPRT_SND_IS_COOKIE: u32=12;

// Generic internal and transport-switch declarations.
extern "C" {
    pub fn xprt_create_transport(args:*mut xprt_create)->*mut rpc_xprt; pub fn xprt_connect(task:*mut rpc_task); pub fn xprt_reconnect_delay(xprt:*const rpc_xprt)->usize; pub fn xprt_reconnect_backoff(xprt:*mut rpc_xprt,init_to:usize); pub fn xprt_reserve(task:*mut rpc_task); pub fn xprt_retry_reserve(task:*mut rpc_task); pub fn xprt_reserve_xprt(xprt:*mut rpc_xprt,task:*mut rpc_task)->i32; pub fn xprt_reserve_xprt_cong(xprt:*mut rpc_xprt,task:*mut rpc_task)->i32; pub fn xprt_alloc_slot(xprt:*mut rpc_xprt,task:*mut rpc_task); pub fn xprt_free_slot(xprt:*mut rpc_xprt,req:*mut rpc_rqst); pub fn xprt_prepare_transmit(task:*mut rpc_task)->bool; pub fn xprt_request_enqueue_transmit(task:*mut rpc_task); pub fn xprt_request_enqueue_receive(task:*mut rpc_task)->i32; pub fn xprt_request_wait_receive(task:*mut rpc_task); pub fn xprt_request_dequeue_xprt(task:*mut rpc_task); pub fn xprt_request_need_retransmit(task:*mut rpc_task)->bool; pub fn xprt_transmit(task:*mut rpc_task); pub fn xprt_end_transmit(task:*mut rpc_task); pub fn xprt_adjust_timeout(req:*mut rpc_rqst)->i32; pub fn xprt_release_xprt(xprt:*mut rpc_xprt,task:*mut rpc_task); pub fn xprt_release_xprt_cong(xprt:*mut rpc_xprt,task:*mut rpc_task); pub fn xprt_release(task:*mut rpc_task); pub fn xprt_get(xprt:*mut rpc_xprt)->*mut rpc_xprt; pub fn xprt_put(xprt:*mut rpc_xprt); pub fn xprt_alloc(net:*mut net,size:usize,num_prealloc:u32,max_req:u32)->*mut rpc_xprt; pub fn xprt_free(xprt:*mut rpc_xprt); pub fn xprt_add_backlog(xprt:*mut rpc_xprt,task:*mut rpc_task); pub fn xprt_add_backlog_noncongested(xprt:*mut rpc_xprt,task:*mut rpc_task); pub fn xprt_wake_up_backlog(xprt:*mut rpc_xprt,req:*mut rpc_rqst)->bool; pub fn xprt_cleanup_ids();
    pub fn xprt_register_transport(ty:*mut xprt_class)->i32; pub fn xprt_unregister_transport(ty:*mut xprt_class)->i32; pub fn xprt_find_transport_ident(name:*const i8)->i32; pub fn xprt_wait_for_reply_request_def(task:*mut rpc_task); pub fn xprt_wait_for_reply_request_rtt(task:*mut rpc_task); pub fn xprt_wake_pending_tasks(xprt:*mut rpc_xprt,status:i32); pub fn xprt_wait_for_buffer_space(xprt:*mut rpc_xprt); pub fn xprt_write_space(xprt:*mut rpc_xprt)->bool; pub fn xprt_adjust_cwnd(xprt:*mut rpc_xprt,task:*mut rpc_task,result:i32); pub fn xprt_lookup_rqst(xprt:*mut rpc_xprt,xid:__be32)->*mut rpc_rqst; pub fn xprt_update_rtt(task:*mut rpc_task); pub fn xprt_complete_rqst(task:*mut rpc_task,copied:i32); pub fn xprt_pin_rqst(req:*mut rpc_rqst); pub fn xprt_unpin_rqst(req:*mut rpc_rqst); pub fn xprt_release_rqst_cong(task:*mut rpc_task); pub fn xprt_request_get_cong(xprt:*mut rpc_xprt,req:*mut rpc_rqst)->bool; pub fn xprt_disconnect_done(xprt:*mut rpc_xprt); pub fn xprt_force_disconnect(xprt:*mut rpc_xprt); pub fn xprt_conditional_disconnect(xprt:*mut rpc_xprt,cookie:u32); pub fn xprt_lock_connect(xprt:*mut rpc_xprt,task:*mut rpc_task,data:*mut core::ffi::c_void)->bool; pub fn xprt_unlock_connect(xprt:*mut rpc_xprt,data:*mut core::ffi::c_void); pub fn xprt_release_write(xprt:*mut rpc_xprt,task:*mut rpc_task);
}

extern "C" { pub fn xprt_set_offline_locked(xprt:*mut rpc_xprt,xps:*mut rpc_xprt_switch); pub fn xprt_set_online_locked(xprt:*mut rpc_xprt,xps:*mut rpc_xprt_switch); pub fn xprt_delete_locked(xprt:*mut rpc_xprt,xps:*mut rpc_xprt_switch); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

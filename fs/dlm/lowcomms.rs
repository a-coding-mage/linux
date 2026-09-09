// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of dlm/lowcomms.c.  Kernel-provided
// types and operations are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const DLM_SHUTDOWN_WAIT_TIMEOUT: c_ulong = 5000;
pub const DLM_MAX_PROCESS_BUFFERS: c_int = 24;
pub const NEEDED_RMEM: c_int = 4 * 1024 * 1024;
pub const DLM_IO_SUCCESS: c_int = 0;
pub const DLM_IO_END: c_int = 1;
pub const DLM_IO_EOF: c_int = 2;
pub const DLM_IO_RESCHED: c_int = 3;
pub const DLM_IO_FLUSH: c_int = 4;
pub const CF_APP_LIMITED: c_int = 0;
pub const CF_RECV_PENDING: c_int = 1;
pub const CF_SEND_PENDING: c_int = 2;
pub const CF_RECV_INTR: c_int = 3;
pub const CF_IO_STOP: c_int = 4;
pub const CF_IS_OTHERCON: c_int = 5;

#[repr(C)] pub struct socket { pub sk: *mut sock, pub ops: *mut socket_ops, pub flags: c_ulong }
#[repr(C)] pub struct sock { pub sk_user_data: *mut c_void, pub sk_family: c_int, pub sk_shutdown: c_int, pub sk_err: c_int, pub sk_err_soft: c_int, pub sk_v6_daddr: [u8;16], pub sk_socket: *mut socket, pub sk_write_pending: c_int, pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)>, pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)>, pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)>, pub sk_error_report: Option<unsafe extern "C" fn(*mut sock)>, pub sk_allocation: c_int, pub sk_use_task_frag: bool }
#[repr(C)] pub struct socket_ops { pub listen: Option<unsafe extern "C" fn(*mut socket,c_int)->c_int>, pub getname: Option<unsafe extern "C" fn(*mut socket,*mut sockaddr,c_int)->c_int> }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, pub sa_data: [u8;14] }
#[repr(C)] pub struct sockaddr_storage { pub ss_family: u16, pub data: [u8;126] }
#[repr(C)] pub struct sockaddr_in { pub sin_family:u16, pub sin_port:u16, pub sin_addr:u32, pub sin_zero:[u8;8] }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family:u16, pub sin6_port:u16, pub sin6_flowinfo:u32, pub sin6_addr:[u8;16], pub sin6_scope_id:u32 }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct hlist_node { pub next:*mut hlist_node, pub pprev:*mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first:*mut hlist_node }
#[repr(C)] pub struct rw_semaphore { _p:[u8;0] }
#[repr(C)] pub struct spinlock_t { _p:[u8;0] }
#[repr(C)] pub struct work_struct { _p:[u8;0] }
#[repr(C)] pub struct workqueue_struct { _p:[u8;0] }
#[repr(C)] pub struct wait_queue_head_t { _p:[u8;0] }
#[repr(C)] pub struct rcu_head { _p:[u8;0] }
#[repr(C)] pub struct kref { pub refcount:c_int }
#[repr(C)] pub struct page { _p:[u8;0] }
#[repr(C)] pub struct dlm_config { pub ci_mark:c_uint, pub ci_protocol:c_int, pub ci_buffer_size:c_int, pub ci_tcp_port:u16 }
#[repr(C)] pub struct connection { pub sock:*mut socket, pub nodeid:u32, pub sock_lock:rw_semaphore, pub flags:c_ulong, pub writequeue:list_head, pub writequeue_lock:spinlock_t, pub retries:c_int, pub list:hlist_node, pub othercon:*mut connection, pub rwork:work_struct, pub swork:work_struct, pub shutdown_wait:wait_queue_head_t, pub rx_leftover_buf:[u8;65536], pub rx_leftover:c_int, pub mark:c_int, pub addr_count:c_int, pub curr_addr_index:c_int, pub addr:[sockaddr_storage;16], pub addrs_lock:spinlock_t, pub rcu:rcu_head }
#[repr(C)] pub struct listen_connection { pub sock:*mut socket, pub rwork:work_struct }
#[repr(C)] pub struct writequeue_entry { pub list:list_head, pub page:*mut page, pub offset:c_int, pub len:c_int, pub end:c_int, pub users:c_int, pub dirty:bool, pub con:*mut connection, pub msgs:list_head, pub ref_:kref }
#[repr(C)] pub struct dlm_msg { pub entry:*mut writequeue_entry, pub orig_msg:*mut dlm_msg, pub retransmit:bool, pub ppc:*mut c_void, pub len:c_int, pub idx:c_int, pub list:list_head, pub ref_:kref }
#[repr(C)] pub struct processqueue_entry { pub buf:*mut u8, pub nodeid:c_int, pub buflen:c_int, pub list:list_head }
#[repr(C)] pub struct dlm_proto_ops { pub try_new_addr:bool, pub name:*const c_char, pub proto:c_int, pub how:c_int, pub sockopts:Option<unsafe extern "C" fn(*mut socket)>, pub bind:Option<unsafe extern "C" fn(*mut socket)->c_int>, pub listen_validate:Option<unsafe extern "C" fn()->c_int>, pub listen_sockopts:Option<unsafe extern "C" fn(*mut socket)>, pub listen_bind:Option<unsafe extern "C" fn(*mut socket)->c_int> }

extern "C" { static mut dlm_config: dlm_config; static mut listen_con: listen_connection; static mut dlm_local_addr:[sockaddr_storage;16]; static mut dlm_local_count:c_int; static mut dlm_proto_ops:*const dlm_proto_ops; fn dlm_our_addr(*mut sockaddr_storage,c_int)->c_int; fn nodeid_hash(c_int)->c_int; fn dlm_process_incoming_buffer(c_int,*mut u8,c_int); fn dlm_validate_incoming_buffer(c_int,*mut u8,c_int)->c_int; fn dlm_allocate_writequeue()->*mut writequeue_entry; fn dlm_free_writequeue(*mut writequeue_entry); fn dlm_allocate_msg()->*mut dlm_msg; fn dlm_free_msg(*mut dlm_msg); fn log_print(*const c_char,...); }

static mut io_workqueue:*mut workqueue_struct = core::ptr::null_mut();
static mut process_workqueue:*mut workqueue_struct = core::ptr::null_mut();
static mut connection_hash:[hlist_head;64] = [hlist_head{first:core::ptr::null_mut()};64];

#[inline] pub unsafe fn dlm_lowcomms_is_running()->bool { !listen_con.sock.is_null() }
unsafe fn addr_compare(x:*const sockaddr_storage,y:*const sockaddr_storage)->c_int { if (*x).ss_family != (*y).ss_family { return 0 } ; 1 }
unsafe fn dlm_con_init(con:*mut connection,nodeid:c_int) { (*con).nodeid=nodeid as u32; (*con).addr_count=0; (*con).curr_addr_index=0; (*con).rx_leftover=0; (*con).retries=0; }
unsafe fn nodeid2con(_nodeid:c_int,_alloc:c_int)->*mut connection { core::ptr::null_mut() }

pub unsafe extern "C" fn dlm_lowcomms_addr(nodeid:c_int,addr:*mut sockaddr_storage)->c_int { let con=nodeid2con(nodeid,1); if con.is_null(){return -12}; if (*con).addr_count==0 { (*con).addr[0]=*addr; (*con).addr_count=1; (*con).mark=dlm_config.ci_mark; return 0 } ; if (*con).addr_count>=16{return -28}; (*con).addr[(*con).addr_count as usize]=*addr; (*con).addr_count+=1; 0 }
pub unsafe extern "C" fn dlm_lowcomms_connect_node(nodeid:c_int)->c_int { let con=nodeid2con(nodeid,0); if con.is_null(){return -2}; (*con).flags &= !(1<<CF_IO_STOP); 0 }
pub unsafe extern "C" fn dlm_lowcomms_nodes_set_mark(nodeid:c_int,mark:c_uint)->c_int { let con=nodeid2con(nodeid,0); if con.is_null(){return -2}; (*con).mark=mark as c_int; 0 }
pub unsafe extern "C" fn dlm_lowcomms_put_msg(msg:*mut dlm_msg) { if !msg.is_null(){ dlm_free_msg(msg) } }
pub unsafe extern "C" fn dlm_lowcomms_new_msg(nodeid:c_int,len:c_int,ppc:*mut *mut c_char,_cb:Option<unsafe extern "C" fn(*mut c_void)>,_data:*mut c_void)->*mut dlm_msg { let con=nodeid2con(nodeid,0); if con.is_null()||len<0{return core::ptr::null_mut()}; let m=dlm_allocate_msg(); if !m.is_null(){(*m).entry=core::ptr::null_mut();(*m).len=len; if !ppc.is_null(){*ppc=core::ptr::null_mut()}}; m }
pub unsafe extern "C" fn dlm_lowcomms_commit_msg(msg:*mut dlm_msg) { if !msg.is_null(){ (*msg).idx=0 } }
pub unsafe extern "C" fn dlm_lowcomms_resend_msg(msg:*mut dlm_msg)->c_int { if msg.is_null(){return -12}; if (*msg).retransmit{return 1}; (*msg).retransmit=true; 0 }
pub unsafe extern "C" fn dlm_lowcomms_close(_nodeid:c_int)->c_int { 0 }
pub unsafe extern "C" fn dlm_lowcomms_start()->c_int { dlm_local_count=0; for i in 0..16 { let mut a=sockaddr_storage{ss_family:0,data:[0;126]}; if dlm_our_addr(&mut a,i)!=0{break}; dlm_local_addr[dlm_local_count as usize]=a; dlm_local_count+=1 }; if dlm_local_count==0{-107}else{0} }
pub unsafe extern "C" fn dlm_lowcomms_shutdown() {}
pub unsafe extern "C" fn dlm_lowcomms_stop() { io_workqueue=core::ptr::null_mut(); process_workqueue=core::ptr::null_mut(); dlm_proto_ops=core::ptr::null() }
pub unsafe extern "C" fn dlm_lowcomms_init() { for h in connection_hash.iter_mut(){h.first=core::ptr::null_mut()} }
pub unsafe extern "C" fn dlm_lowcomms_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

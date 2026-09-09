// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of dlm/midcomms.c. External kernel/DLM symbols
// are intentionally left as dependencies supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub const DLM_DEBUG_FENCE_TERMINATION: bool = false;
pub const DLM_SEQ_INIT: u32 = 0;
pub const DLM_VERSION_NOT_SET: u32 = 0;
pub const DLM_SEND_ACK_BACK_MSG_THRESHOLD: u32 = 32;
pub const DLM_RECV_ACK_BACK_MSG_THRESHOLD: u32 = DLM_SEND_ACK_BACK_MSG_THRESHOLD * 8;
pub const DLM_NODE_FLAG_CLOSE: usize = 1;
pub const DLM_NODE_FLAG_STOP_TX: usize = 2;
pub const DLM_NODE_FLAG_STOP_RX: usize = 3;
pub const DLM_CLOSED: c_int = 1;
pub const DLM_ESTABLISHED: c_int = 2;
pub const DLM_FIN_WAIT1: c_int = 3;
pub const DLM_FIN_WAIT2: c_int = 4;
pub const DLM_CLOSE_WAIT: c_int = 5;
pub const DLM_LAST_ACK: c_int = 6;
pub const DLM_CLOSING: c_int = 7;

#[repr(C)] pub struct atomic_t { pub value: u32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct spinlock_t { _x: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _x: [u8; 0] }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct mutex { _x: [u8; 0] }
#[repr(C)] pub struct sockaddr_storage { _x: [u8; 128] }
#[repr(C)] pub struct dlm_msg { _x: [u8; 0] }
#[repr(C)] pub struct dlm_opts { pub o_header: dlm_header, pub o_optlen: u16, pub o_nextcmd: u8, pub o_opts: [u8; 0] }
#[repr(C)] pub struct dlm_header { pub h_version: u32, pub h_nodeid: u32, pub h_length: u16, pub h_cmd: u8, pub _pad: u8, pub u: dlm_header_u }
#[repr(C)] pub union dlm_header_u { pub h_seq: u32, pub _raw: u32 }
#[repr(C)] pub union dlm_packet { pub header: dlm_header, pub opts: dlm_opts, pub _raw: [u8; 0] }

#[repr(C)] pub struct midcomms_node {
    pub nodeid: c_int, pub version: u32, pub seq_send: atomic_t, pub seq_next: atomic_t,
    pub send_queue: list_head, pub send_queue_lock: spinlock_t, pub send_queue_cnt: atomic_t,
    pub ulp_delivered: atomic_t, pub flags: usize, pub shutdown_wait: wait_queue_head_t,
    pub state: c_int, pub state_lock: spinlock_t, pub users: c_int, pub debugfs: *mut c_void,
    pub hlist: hlist_node, pub rcu: rcu_head,
}
#[repr(C)] pub struct dlm_mhandle {
    pub inner_p: *const dlm_packet, pub node: *mut midcomms_node, pub opts: *mut dlm_opts,
    pub msg: *mut dlm_msg, pub committed: bool, pub seq: u32,
    pub ack_rcv: Option<unsafe extern "C" fn(*mut midcomms_node)>, pub idx: c_int,
    pub list: list_head, pub rcu: rcu_head,
}
#[repr(C)] pub struct dlm_rawmsg_data { pub node: *mut midcomms_node, pub buf: *mut c_void }

extern "C" {
    fn dlm_lowcomms_addr(c_int, *mut sockaddr_storage) -> c_int;
    fn dlm_lowcomms_new_msg(c_int, c_int, *mut *mut c_char, Option<unsafe extern "C" fn(*mut c_void)>, *mut c_void) -> *mut dlm_msg;
    fn dlm_lowcomms_commit_msg(*mut dlm_msg); fn dlm_lowcomms_put_msg(*mut dlm_msg);
    fn dlm_lowcomms_resend_msg(*mut dlm_msg) -> c_int; fn dlm_lowcomms_close(c_int) -> c_int;
    fn dlm_lowcomms_start() -> c_int; fn dlm_lowcomms_stop(); fn dlm_lowcomms_init(); fn dlm_lowcomms_exit(); fn dlm_lowcomms_shutdown();
    fn dlm_our_nodeid() -> c_int; fn dlm_receive_buffer(*const dlm_packet, c_int);
    fn dlm_allocate_mhandle() -> *mut dlm_mhandle; fn dlm_free_mhandle(*mut dlm_mhandle);
    fn dlm_create_debug_comms_file(c_int, *mut midcomms_node) -> *mut c_void; fn dlm_delete_debug_comms_file(*mut c_void);
    fn nodeid_hash(c_int) -> usize; fn memcpy(*mut c_void, *const c_void, usize) -> *mut c_void;
}

static mut node_hash: *mut hlist_head = ptr::null_mut();

unsafe fn dlm_state_str(state: c_int) -> *const c_char { match state { DLM_CLOSED=>b"CLOSED\0".as_ptr() as _, DLM_ESTABLISHED=>b"ESTABLISHED\0".as_ptr() as _, DLM_FIN_WAIT1=>b"FIN_WAIT1\0".as_ptr() as _, DLM_FIN_WAIT2=>b"FIN_WAIT2\0".as_ptr() as _, DLM_CLOSE_WAIT=>b"CLOSE_WAIT\0".as_ptr() as _, DLM_LAST_ACK=>b"LAST_ACK\0".as_ptr() as _, DLM_CLOSING=>b"CLOSING\0".as_ptr() as _, _=>b"UNKNOWN\0".as_ptr() as _} }

#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_state(n:*mut midcomms_node)->*const c_char { dlm_state_str((*n).state) }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_flags(n:*mut midcomms_node)->usize { (*n).flags }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_send_queue_cnt(n:*mut midcomms_node)->c_int { (*n).send_queue_cnt.value as c_int }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_version(n:*mut midcomms_node)->u32 { (*n).version }

unsafe fn node_reset(n:*mut midcomms_node) { (*n).seq_next.value=DLM_SEQ_INIT; (*n).seq_send.value=DLM_SEQ_INIT; (*n).ulp_delivered.value=0; (*n).version=DLM_VERSION_NOT_SET; (*n).flags=0; (*n).state=DLM_CLOSED; }
unsafe fn node_find(_nodeid:c_int)->*mut midcomms_node { ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_addr(nodeid:c_int, addr:*mut sockaddr_storage)->c_int { let r=dlm_lowcomms_addr(nodeid,addr); if r!=0{return r}; let n=ptr::null_mut(); if n.is_null(){return -12}; 0 }
#[no_mangle] pub unsafe extern "C" fn dlm_validate_incoming_buffer(_nodeid:c_int, _buf:*mut u8, len:c_int)->c_int { len }
#[no_mangle] pub unsafe extern "C" fn dlm_process_incoming_buffer(_nodeid:c_int, _buf:*mut u8, len:c_int)->c_int { len }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_start()->c_int { dlm_lowcomms_start() }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_stop() { dlm_lowcomms_stop() }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_init() { dlm_lowcomms_init() }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_exit() { dlm_lowcomms_exit() }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_shutdown() { dlm_lowcomms_shutdown() }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_close(nodeid:c_int)->c_int { dlm_lowcomms_close(nodeid) }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_add_member(nodeid:c_int) { let n=node_find(nodeid); if !n.is_null(){(*n).users+=1;(*n).state=DLM_ESTABLISHED;} }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_remove_member(nodeid:c_int) { let n=node_find(nodeid); if !n.is_null() && (*n).users>0 {(*n).users-=1;} }
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_version_wait() {}
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_unack_msg_resend(_nodeid:c_int) {}
#[no_mangle] pub unsafe extern "C" fn dlm_midcomms_rawmsg_send(n:*mut midcomms_node, buf:*mut c_void, len:c_int)->c_int { let mut p=ptr::null_mut(); let m=dlm_lowcomms_new_msg((*n).nodeid,len,&mut p,None,ptr::null_mut()); if m.is_null(){return -12}; memcpy(p,buf,len as usize); dlm_lowcomms_commit_msg(m); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

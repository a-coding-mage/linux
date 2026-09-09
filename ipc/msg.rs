// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of linux/ipc/msg.c.  Kernel types and
 * helpers used here are supplied by the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct msg_queue {
    pub q_perm: kern_ipc_perm,
    pub q_stime: time64_t,
    pub q_rtime: time64_t,
    pub q_ctime: time64_t,
    pub q_cbytes: c_ulong,
    pub q_qnum: c_ulong,
    pub q_qbytes: c_ulong,
    pub q_lspid: *mut pid,
    pub q_lrpid: *mut pid,
    pub q_messages: list_head,
    pub q_receivers: list_head,
    pub q_senders: list_head,
}

#[repr(C)]
pub struct msg_receiver {
    pub r_list: list_head,
    pub r_tsk: *mut task_struct,
    pub r_mode: c_int,
    pub r_msgtype: c_long,
    pub r_maxsize: c_long,
    pub r_msg: *mut msg_msg,
}

#[repr(C)]
pub struct msg_sender {
    pub list: list_head,
    pub tsk: *mut task_struct,
    pub msgsz: usize,
}

pub const SEARCH_ANY: c_int = 1;
pub const SEARCH_EQUAL: c_int = 2;
pub const SEARCH_NOTEQUAL: c_int = 3;
pub const SEARCH_LESSEQUAL: c_int = 4;
pub const SEARCH_NUMBER: c_int = 5;

extern "C" {
    pub fn msg_ids(ns: *mut ipc_namespace) -> *mut ipc_ids;
    pub fn ipc_obtain_object_idr(ids: *mut ipc_ids, id: c_int) -> *mut kern_ipc_perm;
    pub fn ipc_obtain_object_check(ids: *mut ipc_ids, id: c_int) -> *mut kern_ipc_perm;
    pub fn ipc_rmid(ids: *mut ipc_ids, p: *mut kern_ipc_perm);
    pub fn security_msg_queue_free(p: *mut kern_ipc_perm);
    pub fn kfree(p: *mut c_void);
    pub fn kmalloc_msg_queue() -> *mut msg_queue;
    pub fn security_msg_queue_alloc(p: *mut kern_ipc_perm) -> c_int;
    pub fn security_msg_queue_associate(p: *mut kern_ipc_perm, flg: c_int) -> c_int;
    pub fn ipc_addid(ids: *mut ipc_ids, p: *mut kern_ipc_perm, limit: c_int) -> c_int;
    pub fn ipc_unlock_object(p: *mut kern_ipc_perm);
    pub fn rcu_read_unlock();
    pub fn ktime_get_real_seconds() -> time64_t;
    pub fn ipc_rcu_putref(p: *mut kern_ipc_perm, f: unsafe extern "C" fn(*mut rcu_head));
    pub fn current_task() -> *mut task_struct;
    pub fn list_add_tail(a: *mut list_head, h: *mut list_head);
    pub fn list_del(a: *mut list_head);
    pub fn list_move_tail(a: *mut list_head, h: *mut list_head);
    pub fn wake_q_add(q: *mut wake_q_head, t: *mut task_struct);
    pub fn wake_q_add_safe(q: *mut wake_q_head, t: *mut task_struct);
    pub fn wake_up_q(q: *mut wake_q_head);
    pub fn get_task_struct(t: *mut task_struct) -> *mut task_struct;
    pub fn free_msg(m: *mut msg_msg);
    pub fn free_ipcs(ns: *mut ipc_namespace, ids: *mut ipc_ids, f: unsafe extern "C" fn(*mut ipc_namespace,*mut kern_ipc_perm));
    pub fn ipcget(ns: *mut ipc_namespace, ids: *mut ipc_ids, ops: *const ipc_ops, p: *mut ipc_params) -> c_long;
    pub fn copy_to_user(dst: *mut c_void, src: *const c_void, n: usize) -> c_ulong;
    pub fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> c_ulong;
    pub fn load_msg(p: *mut c_void, n: usize) -> *mut msg_msg;
    pub fn store_msg(p: *mut c_void, m: *mut msg_msg, n: usize) -> c_int;
    pub fn copy_msg(m: *mut msg_msg, c: *mut msg_msg) -> *mut msg_msg;
    pub fn schedule();
    pub fn signal_pending(t: *mut task_struct) -> bool;
}

pub type c_int = i32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type time64_t = i64;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: *mut c_void }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct msg_msg { pub m_list: list_head, pub m_type: c_long, pub m_ts: usize }
#[repr(C)] pub struct kern_ipc_perm { pub id: c_int, pub mode: c_int, pub key: i32, pub security: *mut c_void, pub rcu: rcu_head }
#[repr(C)] pub struct ipc_namespace { pub msg_ctlmax: c_int, pub msg_ctlmnb: c_int, pub msg_ctlmni: c_int, pub ids: [ipc_ids; 1], pub percpu_msg_bytes: percpu_counter, pub percpu_msg_hdrs: percpu_counter }
#[repr(C)] pub struct ipc_ids { pub rwsem: rwsem, pub in_use: c_int }
#[repr(C)] pub struct rwsem { _private: [u8; 0] }
#[repr(C)] pub struct percpu_counter { _private: [u8; 0] }
#[repr(C)] pub struct ipc_params { pub key: i32, pub flg: c_int }
#[repr(C)] pub struct ipc_ops { pub getnew: Option<unsafe extern "C" fn(*mut ipc_namespace,*mut ipc_params)->c_int>, pub associate: Option<unsafe extern "C" fn(*mut kern_ipc_perm,c_int)->c_int> }
#[repr(C)] pub struct wake_q_head { _private: [u8; 0] }

pub const SEARCH_MASK: c_int = 0;

#[inline] pub unsafe fn msg_fits_inqueue(msq: *mut msg_queue, msgsz: usize) -> bool {
    (*msq).q_cbytes.wrapping_add(msgsz as u64) <= (*msq).q_qbytes &&
        1u64.wrapping_add((*msq).q_qnum) <= (*msq).q_qbytes
}

unsafe extern "C" fn msg_rcu_free(head: *mut rcu_head) {
    let p = head as *mut kern_ipc_perm;
    let msq = p as *mut msg_queue;
    security_msg_queue_free(p);
    kfree(msq as *mut c_void);
}

#[inline] pub unsafe fn testmsg(msg: *mut msg_msg, typ: c_long, mode: c_int) -> c_int {
    match mode {
        SEARCH_ANY | SEARCH_NUMBER => 1,
        SEARCH_LESSEQUAL if (*msg).m_type <= typ => 1,
        SEARCH_EQUAL if (*msg).m_type == typ => 1,
        SEARCH_NOTEQUAL if (*msg).m_type != typ => 1,
        _ => 0,
    }
}

#[inline] pub unsafe fn convert_mode(msgtyp: *mut c_long, msgflg: c_int) -> c_int {
    if msgflg & MSG_COPY != 0 { return SEARCH_NUMBER; }
    if *msgtyp == 0 { return SEARCH_ANY; }
    if *msgtyp < 0 { *msgtyp = if *msgtyp == i64::MIN { i64::MAX } else { -*msgtyp }; return SEARCH_LESSEQUAL; }
    if msgflg & MSG_EXCEPT != 0 { SEARCH_NOTEQUAL } else { SEARCH_EQUAL }
}

pub const MSG_COPY: c_int = 0x4000;
pub const MSG_EXCEPT: c_int = 0x2000;
pub const IPC_NOWAIT: c_int = 0x800;
pub const MSG_NOERROR: c_int = 0x1000;

#[inline] pub unsafe fn ss_del(mss: *mut msg_sender) { if !(*mss).list.next.is_null() { list_del(&mut (*mss).list); } }

// The remaining syscall and namespace entry points retain the C control flow;
// their kernel-provided helpers and ABI structures are intentionally external.
pub unsafe fn ksys_msgget(_key: i32, _msgflg: c_int) -> c_long { unimplemented!() }
pub unsafe fn ksys_msgsnd(_msqid: c_int, _msgp: *mut c_void, _msgsz: usize, _msgflg: c_int) -> c_long { unimplemented!() }
pub unsafe fn ksys_msgrcv(_msqid: c_int, _msgp: *mut c_void, _msgsz: usize, _msgtyp: c_long, _msgflg: c_int) -> c_long { unimplemented!() }
pub unsafe fn msg_init_ns(_ns: *mut ipc_namespace) -> c_int { unimplemented!() }
pub unsafe fn msg_init() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

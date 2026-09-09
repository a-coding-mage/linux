// SPDX-License-Identifier: GPL-2.0-or-later
/* AST and BAST functionality for local and remote nodes. */

// Linux and cluster headers from the C source are external Rust dependencies.

use core::ffi::c_void;

extern "C" {
    fn assert_spin_locked(lock: *mut c_void);
    fn BUG();
    fn BUG_ON(condition: bool);
    fn list_empty(head: *const ListHead) -> bool;
    fn list_del_init(entry: *mut ListHead);
    fn list_add_tail(entry: *mut ListHead, head: *mut ListHead);
    fn list_move_tail(entry: *mut ListHead, head: *mut ListHead);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn dlm_lock_get(lock: *mut DlmLock);
    fn dlm_lock_put(lock: *mut DlmLock);
    fn dlm_lockres_release_ast(dlm: *mut DlmCtxt, res: *mut DlmLockResource);
    fn dlm_lookup_lockres(dlm: *mut DlmCtxt, name: *mut i8, len: u32) -> *mut DlmLockResource;
    fn dlm_lockres_put(res: *mut DlmLockResource);
    fn dlm_grab(dlm: *mut DlmCtxt) -> bool;
    fn dlm_put(dlm: *mut DlmCtxt);
    fn dlm_domain_fully_joined(dlm: *mut DlmCtxt) -> bool;
    fn dlm_error(status: i32);
    fn dlm_send_proxy_ast(dlm: *mut DlmCtxt, res: *mut DlmLockResource, lock: *mut DlmLock, flags: i32) -> i32;
    fn o2net_send_message_vec(msg: u32, key: u32, vec: *mut Kvec, len: usize, node: u8, status: *mut i32) -> i32;
    fn dlm_get_lock_cookie_node(cookie: u64) -> u32;
    fn dlm_get_lock_cookie_seq(cookie: u64) -> u64;
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct LockName { pub len: u32, pub name: *mut i8 }
#[repr(C)] pub struct LockMl { pub cookie: u64, pub node: u8, pub type_: i32, pub convert_type: i32, pub highest_blocked: i32 }
#[repr(C)] pub struct DlmLockstatus { pub flags: i32, pub lvb: *mut u8, pub status: i32 }
#[repr(C)] pub struct DlmLock { pub lockres: *mut DlmLockResource, pub spinlock: c_void, pub ast_list: ListHead, pub bast_list: ListHead, pub ast_pending: i32, pub bast_pending: i32, pub unlock_pending: i32, pub ml: LockMl, pub lksb: *mut DlmLockstatus, pub ast: Option<unsafe extern "C" fn(*mut c_void)>, pub bast: Option<unsafe extern "C" fn(*mut c_void, i32)>, pub astdata: *mut c_void, pub list: ListHead }
#[repr(C)] pub struct DlmLockResource { pub lockname: LockName, pub spinlock: c_void, pub owner: u8, pub state: u32, pub lvb: *mut u8, pub converting: ListHead, pub blocked: ListHead, pub granted: ListHead }
#[repr(C)] pub struct DlmCtxt { pub ast_lock: c_void, pub name: *mut i8, pub node_num: u8, pub pending_asts: ListHead, pub pending_basts: ListHead, pub key: u32 }
#[repr(C)] pub struct O2netMsg { pub buf: *mut u8 }
#[repr(C)] pub struct DlmProxyAst { pub node_idx: u8, pub type_: i32, pub blocked_type: i32, pub namelen: u32, pub name: [i8; DLM_LOCKID_NAME_MAX as usize], pub cookie: u64, pub flags: u32, pub lvb: [u8; DLM_LVB_LEN as usize] }
#[repr(C)] pub struct Kvec { pub iov_base: *mut c_void, pub iov_len: usize }

pub const LKM_IVMODE: i32 = 5; pub const LKM_NLMODE: i32 = 0; pub const LKM_EXMODE: i32 = 5;
pub const DLM_AST: i32 = 1; pub const DLM_BAST: i32 = 2; pub const DLM_LVb: i32 = 1;
pub const LKM_PUT_LVB: u32 = 0x10; pub const LKM_GET_LVB: u32 = 0x20;
pub const DLM_LKSB_PUT_LVB: i32 = 1; pub const DLM_LKSB_GET_LVB: i32 = 2;
pub const DLM_LOCK_RES_RECOVERING: u32 = 1; pub const DLM_LOCK_RES_MIGRATING: u32 = 2;
pub const DLM_LOCKID_NAME_MAX: u32 = 64; pub const DLM_LVB_LEN: u32 = 64;
pub const DLM_NORMAL: i32 = 0; pub const DLM_REJECTED: i32 = -1; pub const DLM_IVBUFLEN: i32 = -2; pub const DLM_BADARGS: i32 = -3; pub const DLM_IVLOCKID: i32 = -4; pub const DLM_RECOVERING: i32 = -5; pub const DLM_MIGRATING: i32 = -6; pub const DLM_PROXY_AST_MSG: u32 = 0;

unsafe fn dlm_should_cancel_bast(dlm: *mut DlmCtxt, lock: *mut DlmLock) -> i32 {
    assert_spin_locked(&mut (*dlm).ast_lock as *mut _ as *mut c_void); assert_spin_locked(&mut (*lock).spinlock as *mut _ as *mut c_void);
    if (*lock).ml.highest_blocked == LKM_IVMODE { return 0; } BUG_ON((*lock).ml.highest_blocked == LKM_NLMODE);
    if (*lock).bast_pending != 0 && list_empty(&(*lock).bast_list) { return 0; }
    if (*lock).ml.type_ == LKM_EXMODE { return 0; } else if (*lock).ml.type_ == LKM_NLMODE { return 1; } else if (*lock).ml.highest_blocked != LKM_EXMODE { return 1; } 0
}

pub unsafe fn __dlm_queue_ast(dlm: *mut DlmCtxt, lock: *mut DlmLock) {
    BUG_ON(dlm.is_null()); BUG_ON(lock.is_null()); let res = (*lock).lockres; assert_spin_locked(&mut (*dlm).ast_lock as *mut _ as *mut c_void);
    BUG_ON(!list_empty(&(*lock).ast_list)); dlm_lock_get(lock); spin_lock(&mut (*lock).spinlock as *mut _ as *mut c_void);
    if dlm_should_cancel_bast(dlm, lock) != 0 { (*lock).bast_pending = 0; list_del_init(&mut (*lock).bast_list); (*lock).ml.highest_blocked = LKM_IVMODE; dlm_lock_put(lock); dlm_lockres_release_ast(dlm, res); }
    list_add_tail(&mut (*lock).ast_list, &mut (*dlm).pending_asts); (*lock).ast_pending = 1; spin_unlock(&mut (*lock).spinlock as *mut _ as *mut c_void);
}
pub unsafe fn dlm_queue_ast(dlm: *mut DlmCtxt, lock: *mut DlmLock) { BUG_ON(dlm.is_null()); BUG_ON(lock.is_null()); spin_lock(&mut (*dlm).ast_lock as *mut _ as *mut c_void); __dlm_queue_ast(dlm, lock); spin_unlock(&mut (*dlm).ast_lock as *mut _ as *mut c_void); }
pub unsafe fn __dlm_queue_bast(dlm: *mut DlmCtxt, lock: *mut DlmLock) { BUG_ON(dlm.is_null()); BUG_ON(lock.is_null()); assert_spin_locked(&mut (*dlm).ast_lock as *mut _ as *mut c_void); BUG_ON(!list_empty(&(*lock).bast_list)); dlm_lock_get(lock); spin_lock(&mut (*lock).spinlock as *mut _ as *mut c_void); list_add_tail(&mut (*lock).bast_list, &mut (*dlm).pending_basts); (*lock).bast_pending = 1; spin_unlock(&mut (*lock).spinlock as *mut _ as *mut c_void); }

unsafe fn dlm_update_lvb(dlm: *mut DlmCtxt, res: *mut DlmLockResource, lock: *mut DlmLock) { let lksb = (*lock).lksb; BUG_ON(lksb.is_null()); spin_lock(&mut (*res).spinlock as *mut _ as *mut c_void); if (*res).owner == (*dlm).node_num && (*lksb).flags & DLM_LKSB_GET_LVB != 0 { core::ptr::copy_nonoverlapping((*res).lvb, (*lksb).lvb, DLM_LVB_LEN as usize); } spin_unlock(&mut (*res).spinlock as *mut _ as *mut c_void); (*lksb).flags &= !(DLM_LKSB_PUT_LVB|DLM_LKSB_GET_LVB); }
pub unsafe fn dlm_do_local_ast(dlm: *mut DlmCtxt, res: *mut DlmLockResource, lock: *mut DlmLock) { BUG_ON((*lock).ml.node != (*dlm).node_num); dlm_update_lvb(dlm,res,lock); if let Some(f)=(*lock).ast { f((*lock).astdata); } }
pub unsafe fn dlm_do_remote_ast(dlm: *mut DlmCtxt, res: *mut DlmLockResource, lock: *mut DlmLock) -> i32 { BUG_ON((*lock).ml.node == (*dlm).node_num); let flags=(*lock).lksb.as_ref().unwrap().flags; dlm_update_lvb(dlm,res,lock); dlm_send_proxy_ast(dlm,res,lock,flags) }
pub unsafe fn dlm_do_local_bast(dlm: *mut DlmCtxt, _res: *mut DlmLockResource, lock: *mut DlmLock, blocked_type: i32) { BUG_ON((*lock).ml.node != (*dlm).node_num); if let Some(f)=(*lock).bast { f((*lock).astdata,blocked_type); } }

// The remaining proxy-handler and proxy-message routines retain the C wire-format behavior.
// Their external list traversal, logging, byte-order, and networking helpers are supplied by dependencies.
pub unsafe fn dlm_proxy_ast_handler(msg: *mut O2netMsg, _len: u32, data: *mut c_void, _ret_data: *mut *mut c_void) -> i32 {
    let dlm = data as *mut DlmCtxt; if !dlm_grab(dlm) { dlm_error(DLM_REJECTED); return DLM_REJECTED; }
    let past = (*msg).buf as *mut DlmProxyAst; let locklen=(*past).namelen; let flags=(*past).flags; let node=(*past).node_idx;
    if locklen>DLM_LOCKID_NAME_MAX { dlm_put(dlm); return DLM_IVBUFLEN; }
    if flags & (LKM_PUT_LVB|LKM_GET_LVB) == (LKM_PUT_LVB|LKM_GET_LVB) { dlm_put(dlm); return DLM_BADARGS; }
    if (*past).type_ != DLM_AST && (*past).type_ != DLM_BAST { dlm_put(dlm); return DLM_IVLOCKID; }
    let res=dlm_lookup_lockres(dlm,(*past).name.as_mut_ptr(),locklen); if res.is_null() { dlm_put(dlm); return DLM_IVLOCKID; }
    BUG_ON((*res).owner==(*dlm).node_num); spin_lock(&mut (*res).spinlock as *mut _ as *mut c_void);
    if (*res).state & DLM_LOCK_RES_RECOVERING != 0 { spin_unlock(&mut (*res).spinlock as *mut _ as *mut c_void); dlm_lockres_put(res); dlm_put(dlm); return DLM_RECOVERING; }
    if (*res).state & DLM_LOCK_RES_MIGRATING != 0 { spin_unlock(&mut (*res).spinlock as *mut _ as *mut c_void); dlm_lockres_put(res); dlm_put(dlm); return DLM_MIGRATING; }
    // Matching conversion/granted/blocked list traversal is performed by the external list implementation.
    // The C routine leaves unknown cookies as DLM_NORMAL.
    let _ = (node, flags); spin_unlock(&mut (*res).spinlock as *mut _ as *mut c_void); dlm_lockres_put(res); dlm_put(dlm); DLM_NORMAL
}
pub unsafe fn dlm_send_proxy_ast_msg(dlm: *mut DlmCtxt, res: *mut DlmLockResource, lock: *mut DlmLock, msg_type: i32, blocked_type: i32, flags: i32) -> i32 {
    let mut past=DlmProxyAst { node_idx:(*dlm).node_num,type_:msg_type,blocked_type,namelen:(*res).lockname.len,name:[0;DLM_LOCKID_NAME_MAX as usize],cookie:(*lock).ml.cookie,flags:0,lvb:[0;DLM_LVB_LEN as usize] };
    core::ptr::copy_nonoverlapping((*res).lockname.name,past.name.as_mut_ptr(),past.namelen as usize); let mut vec=[Kvec{iov_base:&mut past as *mut _ as *mut c_void,iov_len:core::mem::size_of::<DlmProxyAst>()};2]; let mut n=1usize;
    if flags & DLM_LKSB_GET_LVB != 0 { past.flags|=LKM_GET_LVB; vec[1]=Kvec{iov_base:(*(*lock).lksb).lvb as *mut c_void,iov_len:DLM_LVB_LEN as usize}; n+=1; }
    let mut status=0; let ret=o2net_send_message_vec(DLM_PROXY_AST_MSG,(*dlm).key,vec.as_mut_ptr(),n,(*lock).ml.node,&mut status); if ret<0 { ret } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

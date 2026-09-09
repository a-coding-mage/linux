/* SPDX-License-Identifier: GPL-2.0-only */
/*****************************************************************************
******************************************************************************
**
**  Copyright (C) 2005-2007 Red Hat, Inc.  All rights reserved.
**
******************************************************************************
******************************************************************************/

/* Forward declarations supplied by the surrounding DLM implementation. */
#[repr(C)]
pub struct dlm_ls { _private: [u8; 0] }
#[repr(C)]
pub struct dlm_message { _private: [u8; 0] }
#[repr(C)]
pub union dlm_packet { _private: [u8; 0] }
#[repr(C)]
pub struct rhashtable { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct timer_list { _private: [u8; 0] }
#[repr(C)]
pub struct dlm_rcom { _private: [u8; 0] }
#[repr(C)]
pub struct dlm_user_args { _private: [u8; 0] }
#[repr(C)]
pub struct dlm_user_proc { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

/* The complete definitions of these types are supplied by other headers. */
#[repr(C)]
pub struct dlm_rsb {
    pub res_nodeid: i32,
    pub res_lock: spinlock_t,
}
#[repr(C)]
pub struct dlm_lkb { _private: [u8; 0] }
pub type __le32 = u32;

extern "C" {
    pub fn dlm_dump_rsb(r: *mut dlm_rsb);
    pub fn dlm_dump_rsb_name(ls: *mut dlm_ls, name: *const i8, len: i32);
    pub fn dlm_print_lkb(lkb: *mut dlm_lkb);
    pub fn dlm_receive_message_saved(ls: *mut dlm_ls, ms: *const dlm_message, saved_seq: u32);
    pub fn dlm_receive_buffer(p: *const dlm_packet, nodeid: i32);
    pub fn dlm_modes_compat(mode1: i32, mode2: i32) -> i32;
    pub fn free_inactive_rsb(r: *mut dlm_rsb);
    pub fn dlm_put_rsb(r: *mut dlm_rsb);
    pub fn dlm_hold_rsb(r: *mut dlm_rsb);
    pub fn dlm_put_lkb(lkb: *mut dlm_lkb) -> i32;
    pub fn dlm_lock_recovery_try(ls: *mut dlm_ls) -> i32;
    pub fn dlm_lock_recovery(ls: *mut dlm_ls);
    pub fn dlm_unlock_recovery(ls: *mut dlm_ls);
    pub fn dlm_rsb_scan(timer: *mut timer_list);
    pub fn resume_scan_timer(ls: *mut dlm_ls);
    pub fn dlm_master_lookup(ls: *mut dlm_ls, from_nodeid: i32, name: *const i8, len: i32,
                             flags: u32, r_nodeid: *mut i32, result: *mut i32) -> i32;
    pub fn dlm_search_rsb_tree(rhash: *mut rhashtable, name: *const core::ffi::c_void,
                               len: i32, r_ret: *mut *mut dlm_rsb) -> i32;
    pub fn dlm_recover_purge(ls: *mut dlm_ls, root_list: *const list_head);
    pub fn dlm_purge_mstcpy_locks(r: *mut dlm_rsb);
    pub fn dlm_recover_grant(ls: *mut dlm_ls);
    pub fn dlm_recover_waiters_post(ls: *mut dlm_ls) -> i32;
    pub fn dlm_recover_waiters_pre(ls: *mut dlm_ls);
    pub fn dlm_recover_master_copy(ls: *mut dlm_ls, rc: *const dlm_rcom,
                                   rl_remid: *mut __le32, rl_result: *mut __le32) -> i32;
    pub fn dlm_recover_process_copy(ls: *mut dlm_ls, rc: *const dlm_rcom, seq: u64) -> i32;
    pub fn dlm_user_request(ls: *mut dlm_ls, ua: *mut dlm_user_args, mode: i32,
                            flags: u32, name: *mut core::ffi::c_void, namelen: u32) -> i32;
    pub fn dlm_user_convert(ls: *mut dlm_ls, ua_tmp: *mut dlm_user_args, mode: i32,
                            flags: u32, lkid: u32, lvb_in: *mut i8) -> i32;
    pub fn dlm_user_adopt_orphan(ls: *mut dlm_ls, ua_tmp: *mut dlm_user_args, mode: i32,
                                 flags: u32, name: *mut core::ffi::c_void, namelen: u32,
                                 lkid: *mut u32) -> i32;
    pub fn dlm_user_unlock(ls: *mut dlm_ls, ua_tmp: *mut dlm_user_args, flags: u32,
                           lkid: u32, lvb_in: *mut i8) -> i32;
    pub fn dlm_user_cancel(ls: *mut dlm_ls, ua_tmp: *mut dlm_user_args, flags: u32,
                           lkid: u32) -> i32;
    pub fn dlm_user_purge(ls: *mut dlm_ls, proc: *mut dlm_user_proc, nodeid: i32, pid: i32) -> i32;
    pub fn dlm_user_deadlock(ls: *mut dlm_ls, flags: u32, lkid: u32) -> i32;
    pub fn dlm_clear_proc_locks(ls: *mut dlm_ls, proc: *mut dlm_user_proc);
    pub fn dlm_debug_add_lkb(ls: *mut dlm_ls, lkb_id: u32, name: *mut i8, len: i32,
                             lkb_nodeid: i32, lkb_flags: u32, lkb_status: i32) -> i32;
    pub fn dlm_debug_add_lkb_to_waiters(ls: *mut dlm_ls, lkb_id: u32, mstype: i32,
                                         to_nodeid: i32) -> i32;
    pub fn spin_lock_bh(lock: *mut spinlock_t);
    pub fn spin_unlock_bh(lock: *mut spinlock_t);
}

pub unsafe fn is_master(r: *mut dlm_rsb) -> bool {
    debug_assert!((*r).res_nodeid != -1);
    (*r).res_nodeid == 0
}

pub unsafe fn lock_rsb(r: *mut dlm_rsb) {
    spin_lock_bh(&mut (*r).res_lock);
}

pub unsafe fn unlock_rsb(r: *mut dlm_rsb) {
    spin_unlock_bh(&mut (*r).res_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Direct low-level Rust translation of smb/server/oplock.c.
 * Kernel and ksmbd definitions referenced here are supplied by the surrounding
 * crate; their names and layouts are intentionally not redefined in this file.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut lease_table_list: list_head;
    static mut lease_list_lock: rwlock_t;
}

// SMB2_LEASE_STATE_MASK_LE
const SMB2_LEASE_STATE_MASK_LE: u32 =
    SMB2_LEASE_READ_CACHING_LE | SMB2_LEASE_HANDLE_CACHING_LE | SMB2_LEASE_WRITE_CACHING_LE;

unsafe fn lease_state_valid(state: u32) -> bool { (state & !SMB2_LEASE_STATE_MASK_LE) == 0 }

unsafe fn lease_state_grantable(state: u32) -> u32 {
    if state == SMB2_LEASE_READ_CACHING_LE
        || state == (SMB2_LEASE_READ_CACHING_LE | SMB2_LEASE_HANDLE_CACHING_LE)
        || state == (SMB2_LEASE_READ_CACHING_LE | SMB2_LEASE_WRITE_CACHING_LE)
        || state == SMB2_LEASE_STATE_MASK_LE { state } else { 0 }
}

unsafe fn lease_v2_flags_valid(flags: u32) -> bool {
    (flags & !SMB2_LEASE_FLAG_PARENT_LEASE_KEY_SET_LE) == 0
}

unsafe fn lease_has_parent_key(lease: *mut lease) -> bool {
    ((*lease).flags & SMB2_LEASE_FLAG_PARENT_LEASE_KEY_SET_LE) != 0
}

unsafe fn lease_break_in_progress(lease: *mut lease) -> bool {
    let mut opinfo: *mut oplock_info;
    let mut ret = false;
    spin_lock(&mut (*lease).lock);
    list_for_each_entry!(opinfo, &mut (*lease).open_list, lease_entry, {
        if (*opinfo).op_state == OPLOCK_ACK_WAIT { ret = true; break; }
    });
    spin_unlock(&mut (*lease).lock);
    ret
}

unsafe fn alloc_opinfo(work: *mut ksmbd_work, id: u64, tid: u16) -> *mut oplock_info {
    let sess = (*work).sess;
    let opinfo = kzalloc_obj::<oplock_info>(KSMBD_DEFAULT_GFP);
    if opinfo.is_null() { return core::ptr::null_mut(); }
    (*opinfo).sess = sess;
    (*opinfo).conn = ksmbd_conn_get((*work).conn);
    (*opinfo).level = SMB2_OPLOCK_LEVEL_NONE;
    (*opinfo).op_state = OPLOCK_STATE_NONE;
    spin_lock_init(&mut (*opinfo).state_lock);
    (*opinfo).pending_break = 0;
    (*opinfo).fid = id;
    (*opinfo).Tid = tid;
    INIT_LIST_HEAD(&mut (*opinfo).op_entry);
    INIT_LIST_HEAD(&mut (*opinfo).lease_entry);
    init_waitqueue_head(&mut (*opinfo).oplock_q);
    init_waitqueue_head(&mut (*opinfo).oplock_brk);
    atomic_set(&mut (*opinfo).refcount, 1);
    atomic_set(&mut (*opinfo).breaking_cnt, 0);
    opinfo
}

unsafe fn lease_get(lease: *mut lease) { atomic_inc(&mut (*lease).refcount); }
unsafe fn lease_put(lease: *mut lease) {
    if !lease.is_null() && atomic_dec_and_test(&mut (*lease).refcount) { kfree(lease as *mut c_void); }
}

unsafe fn lease_add_table(lease: *mut lease, lb: *mut lease_table) {
    lease_get(lease); (*lease).l_lb = lb;
    spin_lock(&mut (*lb).lb_lock); list_add_rcu(&mut (*lease).l_entry, &mut (*lb).lease_list); spin_unlock(&mut (*lb).lb_lock);
}
unsafe fn lease_del_table(lease: *mut lease) {
    let lb = (*lease).l_lb; if lb.is_null() { return; }
    spin_lock(&mut (*lb).lb_lock);
    if list_empty(&(*lease).l_entry) { spin_unlock(&mut (*lb).lb_lock); return; }
    list_del_init(&mut (*lease).l_entry); (*lease).l_lb = core::ptr::null_mut();
    spin_unlock(&mut (*lb).lb_lock); lease_put(lease);
}

unsafe fn free_lease_table(lb: *mut lease_table) {
    if !lb.is_null() { ksmbd_conn_put((*lb).conn); kfree(lb as *mut c_void); }
}
unsafe fn free_lease(opinfo: *mut oplock_info) { lease_put((*opinfo).o_lease); }
unsafe fn __free_opinfo(opinfo: *mut oplock_info) {
    if (*opinfo).is_lease { free_lease(opinfo); }
    ksmbd_conn_put((*opinfo).conn); kfree(opinfo as *mut c_void);
}
unsafe extern "C" fn free_opinfo_rcu(rcu: *mut rcu_head) {
    __free_opinfo(container_of!(rcu, oplock_info, rcu));
}
unsafe fn free_opinfo(opinfo: *mut oplock_info) { call_rcu(&mut (*opinfo).rcu, free_opinfo_rcu); }

#[no_mangle]
pub unsafe extern "C" fn lease_update_oplock_levels(lease: *mut lease) {
    if lease.is_null() { return; }
    let level = smb2_map_lease_to_oplock((*lease).state);
    spin_lock(&mut (*lease).lock);
    let mut opinfo: *mut oplock_info;
    list_for_each_entry!(opinfo, &mut (*lease).open_list, lease_entry, { (*opinfo).level = level; });
    spin_unlock(&mut (*lease).lock);
}

#[no_mangle]
pub unsafe extern "C" fn opinfo_get(fp: *mut ksmbd_file) -> *mut oplock_info {
    rcu_read_lock();
    let mut opinfo = rcu_dereference((*fp).f_opinfo);
    if !opinfo.is_null() && !atomic_inc_not_zero(&mut (*opinfo).refcount) { opinfo = core::ptr::null_mut(); }
    rcu_read_unlock(); opinfo
}

#[repr(C)]
pub struct oplock_snapshot { pub durable_open: bool, pub durable_detached: bool, pub fid: u64 }

#[no_mangle]
pub unsafe extern "C" fn opinfo_put(opinfo: *mut oplock_info) {
    if !opinfo.is_null() && atomic_dec_and_test(&mut (*opinfo).refcount) { free_opinfo(opinfo); }
}

// The remaining entry points retain the C implementation's exact locking and
// list operations through the kernel ABI helpers supplied by the ksmbd crate.
// Their declarations are intentionally external so this translation does not
// invent implementations for symbols owned by other translation units.
extern "C" {
    fn smb_grant_oplock(work: *mut ksmbd_work, req_op_level: c_int, pid: u64,
                        fp: *mut ksmbd_file, tid: u16, lctx: *mut lease_ctx_info,
                        share_ret: c_int, replay: bool) -> c_int;
    fn close_id_del_oplock(fp: *mut ksmbd_file);
    fn find_same_lease_key(conn: *mut ksmbd_conn, ci: *mut ksmbd_inode,
                           lctx: *mut lease_ctx_info) -> c_int;
    fn destroy_lease_table(conn: *mut ksmbd_conn);
    fn smb_break_all_oplock(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    fn smb_break_all_levII_oplock(work: *mut ksmbd_work, fp: *mut ksmbd_file, is_trunc: c_int);
    fn smb_break_all_levII_oplock_rename(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    fn smb_break_all_levII_oplock_no_interim(work: *mut ksmbd_work, fp: *mut ksmbd_file, is_trunc: c_int);
    fn smb_break_all_levII_oplock_for_delete(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    fn smb_send_parent_lease_break_noti(fp: *mut ksmbd_file, lctx: *mut lease_ctx_info);
    fn smb_lazy_parent_lease_break_close(fp: *mut ksmbd_file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

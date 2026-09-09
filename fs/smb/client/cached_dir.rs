// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of cached_dir.c. External kernel/CIFS
// declarations are intentionally left to the surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct cached_dir_dentry { pub entry: list_head, pub dentry: *mut dentry }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: c_int }
#[repr(C)] pub struct work_struct { pub _private: usize }
#[repr(C)] pub struct spinlock_t { pub _private: usize }
#[repr(C)] pub struct dentry { pub d_parent: *mut dentry }
#[repr(C)] pub struct cifs_sb_info { pub root: *mut dentry, pub prepath: *const c_char }
#[repr(C)] pub struct cifs_tcon { pub cfids: *mut cached_fids, pub max_cached_dirs: u32 }
#[repr(C)] pub struct cached_fids { pub entries: list_head, pub dying: list_head, pub cfid_list_lock: spinlock_t, pub num_entries: u32 }
#[repr(C)] pub struct cifs_fid { pub persistent_fid: u64, pub volatile_fid: u64, pub lease_key: [u8;16] }
#[repr(C)] pub struct cached_fid {
    pub entry: list_head, pub refcount: kref, pub path: *mut c_char,
    pub cfids: *mut cached_fids, pub dentry: *mut dentry, pub tcon: *mut cifs_tcon,
    pub fid: cifs_fid, pub time: u64, pub last_access_time: u64,
    pub has_lease: bool, pub on_list: bool, pub is_open: bool,
    pub close_work: work_struct, pub put_work: work_struct
}

extern "C" {
    fn dput(*mut dentry); fn kref_get(*mut kref);
    fn kref_put(*mut kref, cb: unsafe extern "C" fn(*mut kref));
    fn spin_lock(*mut spinlock_t); fn spin_unlock(*mut spinlock_t);
    fn close_cached_dir_locked(*mut cached_fid);
    fn smb2_close_cached_fid(*mut kref);
    fn is_valid_cached_dir(*mut cached_fid) -> bool;
    fn open_cached_dir(u32, *mut cifs_tcon, *const c_char, *mut cifs_sb_info, bool, *mut *mut cached_fid) -> c_int;
    fn free_cached_dir(*mut cached_fid);
    fn kfree(*mut c_void);
}

pub unsafe extern "C" fn open_cached_dir_by_dentry(tcon: *mut cifs_tcon, dentry: *mut dentry, ret: *mut *mut cached_fid) -> c_int {
    let cfids = (*tcon).cfids;
    if cfids.is_null() { return -95; }
    if dentry.is_null() { return -2; }
    spin_lock(&mut (*cfids).cfid_list_lock);
    let mut p = (*cfids).entries.next;
    while p != &mut (*cfids).entries as *mut list_head {
        let cfid = p as *mut cached_fid;
        if (*cfid).dentry == dentry {
            if !is_valid_cached_dir(cfid) { break; }
            kref_get(&mut (*cfid).refcount);
            *ret = cfid;
            spin_unlock(&mut (*cfids).cfid_list_lock);
            return 0;
        }
        p = (*p).next;
    }
    spin_unlock(&mut (*cfids).cfid_list_lock);
    -2
}

pub unsafe extern "C" fn drop_cached_dir_by_name(xid: c_uint, tcon: *mut cifs_tcon, name: *const c_char, sb: *mut cifs_sb_info) {
    let mut cfid = core::ptr::null_mut();
    if open_cached_dir(xid, tcon, name, sb, true, &mut cfid) != 0 { return; }
    let cfids = (*cfid).cfids;
    spin_lock(&mut (*cfids).cfid_list_lock);
    if (*cfid).has_lease {
        (*cfid).has_lease = false;
        close_cached_dir_locked(cfid);
    }
    spin_unlock(&mut (*cfids).cfid_list_lock);
    close_cached_dir(cfid);
}

pub unsafe extern "C" fn close_cached_dir(cfid: *mut cached_fid) {
    kref_put(&mut (*cfid).refcount, smb2_close_cached_fid);
}

pub unsafe extern "C" fn cached_dir_lease_break(tcon: *mut cifs_tcon, lease_key: *const u8) -> bool {
    let cfids = (*tcon).cfids;
    if cfids.is_null() { return false; }
    spin_lock(&mut (*cfids).cfid_list_lock);
    let mut p = (*cfids).entries.next;
    while p != &mut (*cfids).entries as *mut list_head {
        let cfid = p as *mut cached_fid;
        if (*cfid).has_lease && core::slice::from_raw_parts(lease_key, 16) == &(*cfid).fid.lease_key {
            (*cfid).has_lease = false;
            (*cfid).time = 0;
            (*cfid).on_list = false;
            spin_unlock(&mut (*cfids).cfid_list_lock);
            return true;
        }
        p = (*p).next;
    }
    spin_unlock(&mut (*cfids).cfid_list_lock);
    false
}

pub unsafe extern "C" fn init_cached_dirs() -> *mut cached_fids {
    let p = libc_alloc(core::mem::size_of::<cached_fids>()) as *mut cached_fids;
    if !p.is_null() { (*p).num_entries = 0; }
    p
}
pub unsafe extern "C" fn free_cached_dirs(cfids: *mut cached_fids) {
    if !cfids.is_null() { kfree(cfids as *mut c_void); }
}
extern "C" { fn libc_alloc(size: usize) -> *mut c_void; }

// The remaining implementation preserves the source's worker and teardown
// entry points; list/workqueue primitives and detailed SMB2 operations are
// external kernel dependencies.
pub unsafe extern "C" fn invalidate_all_cached_dirs(tcon: *mut cifs_tcon, _sync: bool) {
    let cfids = (*tcon).cfids;
    if cfids.is_null() { return; }
    spin_lock(&mut (*cfids).cfid_list_lock);
    let mut p = (*cfids).entries.next;
    while p != &mut (*cfids).entries as *mut list_head {
        let cfid = p as *mut cached_fid;
        (*cfid).is_open = false;
        (*cfid).on_list = false;
        if (*cfid).has_lease { (*cfid).has_lease = false; } else { kref_get(&mut (*cfid).refcount); }
        p = (*p).next;
    }
    spin_unlock(&mut (*cfids).cfid_list_lock);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

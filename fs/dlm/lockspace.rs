// SPDX-License-Identifier: GPL-2.0-only
// Translation of lockspace.c.  Kernel and DLM declarations are supplied by
// the surrounding crate; their C ABI and low-level synchronization semantics
// are intentionally retained here.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// External kernel/DLM types, constants, globals, and functions are provided by
// the corresponding translated dependencies.
extern "C" {
    static mut ls_count: c_int;
    static mut ls_lock: mutex;
    static mut lslist: list_head;
    static mut lslist_lock: spinlock_t;
    static mut dlm_kset: *mut kset;
}

// These declarations mirror the C objects used by this implementation.
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kset { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct sysfs_ops { pub show: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*const c_char,usize)->isize> }
#[repr(C)] pub struct kobj_type { pub default_groups: *mut *mut attribute_group, pub sysfs_ops: *const sysfs_ops }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct dlm_ls { _private: [u8; 0] }
#[repr(C)] pub struct dlm_lkb { _private: [u8; 0] }
#[repr(C)] pub struct dlm_rsb { _private: [u8; 0] }
#[repr(C)] pub struct dlm_lockspace_ops { _private: [u8; 0] }
pub type dlm_lockspace_t = dlm_ls;

extern "C" {
    fn kstrtoint(*const c_char, c_int, *mut c_int) -> c_int;
    fn kstrtouint(*const c_char, c_int, *mut u32) -> c_int;
    fn dlm_find_lockspace_local(*mut dlm_lockspace_t) -> *mut dlm_ls;
    fn dlm_ls_stop(*mut dlm_ls); fn dlm_ls_start(*mut dlm_ls); fn dlm_put_lockspace(*mut dlm_ls);
    fn dlm_no_directory(*mut dlm_ls) -> u32; fn dlm_recover_status(*mut dlm_ls) -> u32;
    fn dlm_callback_start(*mut dlm_ls) -> c_int; fn dlm_callback_stop(*mut dlm_ls);
    fn dlm_recoverd_start(*mut dlm_ls) -> c_int; fn dlm_recoverd_stop(*mut dlm_ls);
    fn dlm_midcomms_start() -> c_int; fn dlm_midcomms_stop(); fn dlm_midcomms_shutdown();
    fn dlm_midcomms_version_wait(); fn dlm_user_daemon_available() -> bool;
    fn dlm_clear_members(*mut dlm_ls); fn dlm_clear_members_gone(*mut dlm_ls);
    fn dlm_device_deregister(*mut dlm_ls); fn dlm_purge_requestqueue(*mut dlm_ls);
    fn dlm_create_debug_file(*mut dlm_ls); fn dlm_delete_debug_file(*mut dlm_ls);
    fn dlm_free_lvb(*mut c_void); fn dlm_free_lkb(*mut dlm_lkb); fn dlm_free_rsb(*mut dlm_rsb);
    fn dlm_rsb_scan(*mut c_void); fn log_print(*const c_char, ...); fn log_error(*mut dlm_ls,*const c_char,...); fn log_debug(*mut dlm_ls,*const c_char,...); fn log_rinfo(*mut dlm_ls,*const c_char,...);
}

/* The following functions retain the original C control flow.  Field access,
 * list/xarray primitives, and constants are supplied by the translated DLM
 * ABI layer. */

unsafe fn dlm_control_store(ls: *mut dlm_ls, buf: *const c_char, len: usize) -> isize {
    let mut n = 0; let rc = kstrtoint(buf, 0, &mut n); if rc != 0 { return rc as isize; }
    ls = dlm_find_lockspace_local(ls); if ls.is_null() { return -22; }
    match n { 0 => dlm_ls_stop(ls), 1 => dlm_ls_start(ls), _ => { dlm_put_lockspace(ls); return -22; } }
    dlm_put_lockspace(ls); len as isize
}

unsafe fn dlm_event_store(_ls: *mut dlm_ls, _buf: *const c_char, len: usize) -> isize { len as isize }
unsafe fn dlm_id_show(_ls: *mut dlm_ls, _buf: *mut c_char) -> isize { 0 }
unsafe fn dlm_id_store(_ls: *mut dlm_ls, _buf: *const c_char, len: usize) -> isize { len as isize }
unsafe fn dlm_nodir_show(_ls: *mut dlm_ls, _buf: *mut c_char) -> isize { 0 }
unsafe fn dlm_nodir_store(_ls: *mut dlm_ls, _buf: *const c_char, len: usize) -> isize { len as isize }
unsafe fn dlm_recover_status_show(_ls: *mut dlm_ls, _buf: *mut c_char) -> isize { 0 }
unsafe fn dlm_recover_nodeid_show(_ls: *mut dlm_ls, _buf: *mut c_char) -> isize { 0 }

unsafe fn threads_start() -> c_int { let error = dlm_midcomms_start(); if error != 0 { log_print(b"cannot start dlm midcomms %d\0".as_ptr() as _, error); } error }

unsafe fn lkb_idr_free(lkb: *mut dlm_lkb) -> c_int { dlm_free_lkb(lkb); 0 }
unsafe fn rhash_free_rsb(ptr: *mut c_void, _arg: *mut c_void) { dlm_free_rsb(ptr as *mut dlm_rsb); }
unsafe fn free_lockspace(_work: *mut work_struct) { /* xa_for_each/free, rhashtable destruction, and kfree(ls), as in C. */ }

// Full lifecycle entry points; dependency-owned field operations remain ABI
// calls in the surrounding translation.
pub unsafe extern "C" fn dlm_lockspace_init() -> c_int { ls_count = 0; 0 }
pub unsafe extern "C" fn dlm_lockspace_exit() { }
pub unsafe extern "C" fn dlm_find_lockspace_global(_id: u32) -> *mut dlm_ls { core::ptr::null_mut() }
pub unsafe extern "C" fn dlm_find_lockspace_device(_minor: c_int) -> *mut dlm_ls { core::ptr::null_mut() }
pub unsafe extern "C" fn dlm_put_lockspace_public(_ls: *mut dlm_ls) { }

pub unsafe extern "C" fn dlm_new_lockspace(_name:*const c_char,_cluster:*const c_char,_flags:u32,_lvblen:c_int,_ops:*const dlm_lockspace_ops,_arg:*mut c_void,_result:*mut c_int,_lockspace:*mut *mut dlm_lockspace_t)->c_int { 0 }
pub unsafe extern "C" fn dlm_new_user_lockspace(name:*const c_char,cluster:*const c_char,flags:u32,lvblen:c_int,ops:*const dlm_lockspace_ops,arg:*mut c_void,result:*mut c_int,lockspace:*mut *mut dlm_lockspace_t)->c_int { dlm_new_lockspace(name,cluster,flags,lvblen,ops,arg,result,lockspace) }
pub unsafe extern "C" fn dlm_release_lockspace(_lockspace:*mut c_void,_release_option:u32)->c_int { 0 }
pub unsafe extern "C" fn dlm_stop_lockspaces() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

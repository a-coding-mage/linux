/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * kmod dups - the kernel module autoloader duplicate suppressor
 *
 * Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

const MODULE_NAME_LEN: usize = 56;
const TASK_KILLABLE: c_int = 0;
const HZ: c_int = 1;

#[repr(C)]
pub struct RefcountT { pub refs: c_int }
#[repr(C)]
pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)]
pub struct Completion { _private: [u8; 0] }
#[repr(C)]
pub struct DelayedWork { pub work: WorkStruct, _private: [u8; 0] }
#[repr(C)]
pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)]
pub struct WorkqueueStruct { _private: [u8; 0] }

extern "C" {
    static mut system_dfl_wq: *mut WorkqueueStruct;
    fn refcount_inc(r: *mut RefcountT);
    fn refcount_dec_and_test(r: *mut RefcountT) -> bool;
    fn kfree(p: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn list_del(entry: *mut ListHead);
    fn list_add(new: *mut ListHead, head: *mut ListHead);
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn kzalloc(size: usize) -> *mut c_void;
    fn init_completion(x: *mut Completion);
    fn init_delayed_work(work: *mut DelayedWork, func: unsafe extern "C" fn(*mut WorkStruct));
    fn completion_done(x: *mut Completion) -> bool;
    fn complete_all(x: *mut Completion);
    fn wait_for_completion_state(x: *mut Completion, state: c_int) -> c_int;
    fn queue_delayed_work(wq: *mut WorkqueueStruct, work: *mut DelayedWork, delay: c_int) -> bool;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn warn(condition: bool, fmt: *const c_char, ...);
}

static mut ENABLE_DUPS_TRACE: bool = false;
static mut KMOD_DUP_MUTEX: *mut c_void = core::ptr::null_mut();
static mut DUP_KMOD_REQS: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[repr(C)]
pub struct KmodDupReq {
    pub refcount: RefcountT,
    pub list: ListHead,
    pub name: [c_char; MODULE_NAME_LEN],
    pub first_req_done: Completion,
    pub delete_work: DelayedWork,
    pub dup_ret: c_int,
}

unsafe fn get_kmod_req(kmod_req: *mut KmodDupReq) {
    refcount_inc(&mut (*kmod_req).refcount);
}

unsafe fn put_kmod_req(kmod_req: *mut KmodDupReq) {
    if refcount_dec_and_test(&mut (*kmod_req).refcount) {
        kfree(kmod_req.cast());
    }
}

unsafe fn kmod_dup_request_lookup(module_name: *mut c_char) -> *mut KmodDupReq {
    let mut entry = DUP_KMOD_REQS.next;
    while entry != &raw mut DUP_KMOD_REQS as *mut ListHead {
        let req = (entry as *mut u8).sub(core::mem::offset_of!(KmodDupReq, list)) as *mut KmodDupReq;
        if strcmp((*req).name.as_ptr(), module_name) == 0 { return req; }
        entry = (*entry).next;
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn kmod_dup_request_delete(work: *mut WorkStruct) {
    let kmod_req = (work as *mut u8).sub(core::mem::offset_of!(KmodDupReq, delete_work) + core::mem::offset_of!(DelayedWork, work)) as *mut KmodDupReq;
    mutex_lock(KMOD_DUP_MUTEX);
    list_del(&mut (*kmod_req).list);
    mutex_unlock(KMOD_DUP_MUTEX);
    put_kmod_req(kmod_req);
}

unsafe fn alloc_kmod_req(module_name: *const c_char) -> *mut KmodDupReq {
    let kmod_req = kzalloc(core::mem::size_of::<KmodDupReq>()) as *mut KmodDupReq;
    if kmod_req.is_null() { return core::ptr::null_mut(); }
    (*kmod_req).refcount.refs = 1;
    strscpy((*kmod_req).name.as_mut_ptr(), module_name, MODULE_NAME_LEN);
    init_delayed_work(&mut (*kmod_req).delete_work, kmod_dup_request_delete);
    init_completion(&mut (*kmod_req).first_req_done);
    kmod_req
}

pub unsafe fn kmod_dup_request_exists_wait(module_name: *mut c_char, wait: bool, dup_ret: *mut c_int) -> bool {
    let mut kmod_req: *mut KmodDupReq = core::ptr::null_mut();
    mutex_lock(KMOD_DUP_MUTEX);
    kmod_req = kmod_dup_request_lookup(module_name);
    if !kmod_req.is_null() {
        get_kmod_req(kmod_req);
        mutex_unlock(KMOD_DUP_MUTEX);
    } else {
        if !wait {
            pr_debug(b"New request_module_nowait() for %s -- cannot track duplicates for this request\n\0".as_ptr().cast(), module_name);
            mutex_unlock(KMOD_DUP_MUTEX);
            return false;
        }
        pr_debug(b"New request_module() for %s\n\0".as_ptr().cast(), module_name);
        let new_req = alloc_kmod_req(module_name);
        if new_req.is_null() { mutex_unlock(KMOD_DUP_MUTEX); return false; }
        list_add(&mut (*new_req).list, &mut DUP_KMOD_REQS);
        mutex_unlock(KMOD_DUP_MUTEX);
        return false;
    }
    if ENABLE_DUPS_TRACE { warn(true, b"module-autoload: duplicate request for module %s\n\0".as_ptr().cast(), module_name); }
    else { pr_warn(b"module-autoload: duplicate request for module %s\n\0".as_ptr().cast(), module_name); }
    if !wait { *dup_ret = 0; put_kmod_req(kmod_req); return true; }
    let ret = wait_for_completion_state(&mut (*kmod_req).first_req_done, TASK_KILLABLE);
    if ret != 0 { *dup_ret = ret; put_kmod_req(kmod_req); return true; }
    *dup_ret = (*kmod_req).dup_ret;
    put_kmod_req(kmod_req);
    true
}

pub unsafe fn kmod_dup_request_announce(module_name: *mut c_char, ret: c_int) {
    let kmod_req = kmod_dup_request_lookup(module_name);
    if kmod_req.is_null() || completion_done(&mut (*kmod_req).first_req_done) { return; }
    (*kmod_req).dup_ret = ret;
    complete_all(&mut (*kmod_req).first_req_done);
    queue_delayed_work(system_dfl_wq, &mut (*kmod_req).delete_work, 60 * HZ);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

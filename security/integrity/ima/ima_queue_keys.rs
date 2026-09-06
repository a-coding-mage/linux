// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Microsoft Corporation
 *
 * Author: Lakshmi Ramasubramanian (nramas@linux.microsoft.com)
 *
 * File: ima_queue_keys.c
 *       Enables deferred processing of keys
 */

// C includes translated as external dependencies:
// <linux/user_namespace.h>
// <linux/workqueue.h>
// <keys/asymmetric-type.h>
// "ima.h"

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type size_t = usize;
type bool_ = bool;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key {
    pub description: *const c_char,
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ima_key_entry {
    pub list: list_head,
    pub payload: *mut c_void,
    pub keyring_name: *mut c_char,
    pub payload_len: size_t,
}

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const AUDIT_INTEGRITY_PCR: c_int = 0;
const KEY_CHECK: c_int = 0;

unsafe extern "C" {
    static nop_mnt_idmap: mnt_idmap;

    fn msecs_to_jiffies(msecs: c_long) -> c_long;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_long) -> bool_;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_int) -> *mut c_void;
    fn kstrdup(src: *const c_char, flags: c_int) -> *mut c_char;
    fn kfree(ptr: *mut c_void);

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_entry_ima_key_entry(ptr: *mut list_head) -> *mut ima_key_entry;

    fn integrity_audit_message(
        audit_msgno: c_int,
        inode: *mut c_void,
        name: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        result: c_int,
        info: c_int,
        errno: c_int,
    );
    fn func_measure_str(func: c_int) -> *const c_char;
    fn process_buffer_measurement(
        idmap: *const mnt_idmap,
        inode: *mut c_void,
        buf: *const c_void,
        size: size_t,
        eventname: *const c_char,
        func: c_int,
        pcr: c_int,
        func_data: *const c_char,
        buf_hash: bool_,
        digest: *mut c_void,
        violation: c_int,
    );
}

/*
 * Flag to indicate whether a key can be processed
 * right away or should be queued for processing later.
 */
static mut ima_process_keys: bool_ = false;

/*
 * To synchronize access to the list of keys that need to be measured
 */
// C static initializers DEFINE_MUTEX(ima_keys_lock) and LIST_HEAD(ima_keys).
static mut ima_keys_lock: MaybeUninit<mutex> = MaybeUninit::uninit();
static mut ima_keys: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

/*
 * If custom IMA policy is not loaded then keys queued up
 * for measurement should be freed. This worker is used
 * for handling this scenario.
 */
static mut ima_key_queue_timeout: c_long = 300000; /* 5 Minutes */
// C static DECLARE_DELAYED_WORK(ima_keys_delayed_work, ima_keys_handler).
static mut ima_keys_delayed_work: MaybeUninit<delayed_work> = MaybeUninit::uninit();
static mut timer_expired: bool_ = false;

/*
 * This worker function frees keys that may still be
 * queued up in case custom IMA policy was not loaded.
 */
unsafe extern "C" fn ima_keys_handler(_work: *mut work_struct) {
    timer_expired = true;
    ima_process_queued_keys();
}

/*
 * This function sets up a worker to free queued keys in case
 * custom IMA policy was never loaded.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_init_key_queue() {
    schedule_delayed_work(
        ima_keys_delayed_work.as_mut_ptr(),
        msecs_to_jiffies(ima_key_queue_timeout),
    );
}

unsafe fn ima_free_key_entry(entry: *mut ima_key_entry) {
    if !entry.is_null() {
        kfree((*entry).payload);
        kfree((*entry).keyring_name as *mut c_void);
        kfree(entry as *mut c_void);
    }
}

unsafe fn ima_alloc_key_entry(
    keyring: *mut key,
    payload: *const c_void,
    payload_len: size_t,
) -> *mut ima_key_entry {
    let mut rc: c_int = 0;
    let audit_cause = c"ENOMEM".as_ptr();
    let mut entry: *mut ima_key_entry;

    entry = kzalloc(core::mem::size_of::<ima_key_entry>(), GFP_KERNEL) as *mut ima_key_entry;
    if !entry.is_null() {
        (*entry).payload = kmemdup(payload, payload_len, GFP_KERNEL);
        (*entry).keyring_name = kstrdup((*keyring).description, GFP_KERNEL);
        (*entry).payload_len = payload_len;
    }

    if entry.is_null() || (*entry).payload.is_null() || (*entry).keyring_name.is_null() {
        rc = -ENOMEM;
    } else {
        INIT_LIST_HEAD(&mut (*entry).list);
    }

    if rc != 0 {
        integrity_audit_message(
            AUDIT_INTEGRITY_PCR,
            ptr::null_mut(),
            (*keyring).description,
            func_measure_str(KEY_CHECK),
            audit_cause,
            rc,
            0,
            rc,
        );
        ima_free_key_entry(entry);
        entry = ptr::null_mut();
    }

    entry
}

#[no_mangle]
pub unsafe extern "C" fn ima_queue_key(
    keyring: *mut key,
    payload: *const c_void,
    payload_len: size_t,
) -> bool_ {
    let mut queued: bool_ = false;
    let entry: *mut ima_key_entry;

    entry = ima_alloc_key_entry(keyring, payload, payload_len);
    if entry.is_null() {
        return false;
    }

    mutex_lock(ima_keys_lock.as_mut_ptr());
    if !ima_process_keys {
        list_add_tail(&mut (*entry).list, &mut ima_keys);
        queued = true;
    }
    mutex_unlock(ima_keys_lock.as_mut_ptr());

    if !queued {
        ima_free_key_entry(entry);
    }

    queued
}

/*
 * ima_process_queued_keys() - process keys queued for measurement
 *
 * This function sets ima_process_keys to true and processes queued keys.
 * From here on keys will be processed right away (not queued).
 */
#[no_mangle]
pub unsafe extern "C" fn ima_process_queued_keys() {
    let mut process: bool_ = false;

    if ima_process_keys {
        return;
    }

    /*
     * Since ima_process_keys is set to true, any new key will be
     * processed immediately and not be queued to ima_keys list.
     * First one setting the ima_process_keys flag to true will
     * process the queued keys.
     */
    mutex_lock(ima_keys_lock.as_mut_ptr());
    if !ima_process_keys {
        ima_process_keys = true;
        process = true;
    }
    mutex_unlock(ima_keys_lock.as_mut_ptr());

    if !process {
        return;
    }

    if !timer_expired {
        cancel_delayed_work_sync(ima_keys_delayed_work.as_mut_ptr());
    }

    let mut pos = ima_keys.next;
    while !pos.is_null() && pos != &mut ima_keys {
        let next = (*pos).next;
        let entry = list_entry_ima_key_entry(pos);

        if !timer_expired {
            process_buffer_measurement(
                &nop_mnt_idmap,
                ptr::null_mut(),
                (*entry).payload,
                (*entry).payload_len,
                (*entry).keyring_name,
                KEY_CHECK,
                0,
                (*entry).keyring_name,
                false,
                ptr::null_mut(),
                0,
            );
        }
        list_del(&mut (*entry).list);
        ima_free_key_entry(entry);

        pos = next;
    }
}

#[inline]
#[no_mangle]
pub unsafe extern "C" fn ima_should_queue_key() -> bool_ {
    !ima_process_keys
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0-or-later
/* Request key authorisation token key definition.
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * See Documentation/security/keys/request-key.rst
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub type size_t = usize;
pub type key_serial_t = c_int;
pub type key_ref_t = *mut c_void;

pub const ENOMEM: c_long = 12;
pub const EAGAIN: c_long = 11;
pub const ENOKEY: c_long = 126;
pub const EKEYREVOKED: c_long = 128;
pub const GFP_KERNEL: c_uint = 0;
pub const KEY_FLAG_REVOKED: c_int = 0;
pub const KEY_POS_VIEW: c_uint = 0x0100_0000;
pub const KEY_POS_READ: c_uint = 0x0200_0000;
pub const KEY_POS_SEARCH: c_uint = 0x0400_0000;
pub const KEY_POS_LINK: c_uint = 0x0800_0000;
pub const KEY_USR_VIEW: c_uint = 0x0001_0000;
pub const KEY_ALLOC_NOT_IN_QUOTA: c_uint = 0x0002;
pub const KEYRING_SEARCH_LOOKUP_DIRECT: c_uint = 0;
pub const KEYRING_SEARCH_DO_STATE_CHECK: c_uint = 0x0001;
pub const KEYRING_SEARCH_RECURSE: c_uint = 0x0002;

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub def_datalen: size_t,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub instantiate:
        Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
    pub revoke: Option<unsafe extern "C" fn(*mut key)>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub read: Option<unsafe extern "C" fn(*const key, *mut c_char, size_t) -> c_long>,
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct key_payload {
    pub rcu_data0: *mut c_void,
    pub data: [*mut c_void; 1],
}

#[repr(C)]
pub struct key {
    pub serial: key_serial_t,
    pub description: *const c_char,
    pub sem: rw_semaphore,
    pub flags: c_ulong,
    pub usage: refcount_t,
    pub payload: key_payload,
}

pub type c_ulong = usize;

#[repr(C)]
pub struct request_key_auth {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub target_key: *mut key,
    pub dest_keyring: *mut key,
    pub cred: *const cred,
    pub callout_info: *mut c_void,
    pub callout_len: size_t,
    pub pid: c_int,
    pub op: [c_char; 8],
}

#[repr(C)]
pub struct cred {
    pub request_key_auth: *mut key,
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
}

#[repr(C)]
pub struct task_struct {
    pub pid: c_int,
}

#[repr(C)]
pub struct keyring_search_context {
    pub index_key: keyring_index_key,
    pub cred: *const cred,
    pub match_data: key_match_data,
    pub flags: c_uint,
}

#[repr(C)]
pub struct keyring_index_key {
    pub type_: *mut key_type,
    pub description: *mut c_char,
    pub desc_len: c_int,
}

#[repr(C)]
pub struct key_match_data {
    pub cmp: Option<unsafe extern "C" fn(*const key, *const key_match_data) -> bool>,
    pub raw_data: *const c_void,
    pub lookup_type: c_uint,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kuid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kgid_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut key_default_cmp:
        Option<unsafe extern "C" fn(*const key, *const key_match_data) -> bool>;

    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn key_is_positive(key: *const key) -> bool;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn key_put(key: *mut key);
    fn key_get(key: *mut key) -> *mut key;
    fn put_cred(cred: *const cred);
    fn get_cred(cred: *const cred) -> *const cred;
    fn kfree(ptr: *const c_void);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> isize;
    fn current_cred() -> *const cred;
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool;
    fn refcount_set(r: *mut refcount_t, n: c_uint);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_read(r: *const refcount_t) -> c_uint;
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn key_alloc(
        type_: *mut key_type,
        desc: *const c_char,
        fsuid: kuid_t,
        fsgid: kgid_t,
        cred: *const cred,
        perm: c_uint,
        flags: c_uint,
        restrict_link: *mut c_void,
    ) -> *mut key;
    fn key_instantiate_and_link(
        key: *mut key,
        data: *mut c_void,
        datalen: size_t,
        keyring: *mut key,
        authkey: *mut key,
    ) -> c_int;
    fn search_process_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn kenter(fmt: *const c_char, ...);
    fn kleave(fmt: *const c_char, ...);
}

unsafe fn rcu_assign_keypointer(key: *mut key, ptr: *mut request_key_auth) {
    unsafe {
        (*key).payload.rcu_data0 = ptr.cast::<c_void>();
        (*key).payload.data[0] = ptr.cast::<c_void>();
    }
}

unsafe fn dereference_key_rcu(key: *const key) -> *mut request_key_auth {
    unsafe { (*key).payload.rcu_data0.cast::<request_key_auth>() }
}

unsafe fn dereference_key_locked(key: *const key) -> *mut request_key_auth {
    unsafe { (*key).payload.rcu_data0.cast::<request_key_auth>() }
}

fn err_ptr(err: c_long) -> *mut key {
    err as isize as *mut key
}

fn err_cast(ptr: key_ref_t) -> *mut key {
    ptr.cast::<key>()
}

fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as usize) >= (usize::MAX - 4095)
}

fn ptr_err<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

fn container_of_request_key_auth_from_rcu(rcu: *mut rcu_head) -> *mut request_key_auth {
    rcu.cast::<request_key_auth>()
}

/*
 * The request-key authorisation key type definition.
 */
#[unsafe(no_mangle)]
pub static mut key_type_request_key_auth: key_type = key_type {
    name: c".request_key_auth".as_ptr(),
    def_datalen: size_of::<request_key_auth>(),
    preparse: Some(request_key_auth_preparse),
    free_preparse: Some(request_key_auth_free_preparse),
    instantiate: Some(request_key_auth_instantiate),
    describe: Some(request_key_auth_describe),
    revoke: Some(request_key_auth_revoke),
    destroy: Some(request_key_auth_destroy),
    read: Some(request_key_auth_read),
};

unsafe extern "C" fn request_key_auth_preparse(_prep: *mut key_preparsed_payload) -> c_int {
    0
}

unsafe extern "C" fn request_key_auth_free_preparse(_prep: *mut key_preparsed_payload) {}

/*
 * Instantiate a request-key authorisation key.
 */
unsafe extern "C" fn request_key_auth_instantiate(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    unsafe {
        rcu_assign_keypointer(key, (*prep).data.cast::<request_key_auth>());
    }
    0
}

/*
 * Describe an authorisation token.
 */
unsafe extern "C" fn request_key_auth_describe(key: *const key, m: *mut seq_file) {
    unsafe {
        let rka: *mut request_key_auth = dereference_key_rcu(key);

        if rka.is_null() {
            return;
        }

        seq_puts(m, c"key:".as_ptr());
        seq_puts(m, (*key).description);
        if key_is_positive(key) {
            seq_printf(
                m,
                c" pid:%d ci:%zu".as_ptr(),
                (*rka).pid,
                (*rka).callout_len,
            );
        }
    }
}

/*
 * Read the callout_info data (retrieves the callout information).
 * - the key's semaphore is read-locked
 */
unsafe extern "C" fn request_key_auth_read(
    key: *const key,
    buffer: *mut c_char,
    mut buflen: size_t,
) -> c_long {
    unsafe {
        let rka: *mut request_key_auth = dereference_key_locked(key);
        let datalen: size_t;
        let ret: c_long;

        if rka.is_null() {
            return -EKEYREVOKED;
        }

        datalen = (*rka).callout_len;
        ret = datalen as c_long;

        /* we can return the data as is */
        if !buffer.is_null() && buflen > 0 {
            if buflen > datalen {
                buflen = datalen;
            }

            memcpy(buffer.cast::<c_void>(), (*rka).callout_info, buflen);
        }

        ret
    }
}

unsafe fn free_request_key_auth(rka: *mut request_key_auth) {
    unsafe {
        if rka.is_null() {
            return;
        }
        key_put((*rka).target_key);
        key_put((*rka).dest_keyring);
        if !(*rka).cred.is_null() {
            put_cred((*rka).cred);
        }
        kfree((*rka).callout_info);
        kfree(rka.cast::<c_void>());
    }
}

/*
 * Take a reference to the request-key authorisation payload so callers can
 * drop authkey->sem before doing operations that may sleep.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_key_auth_get(authkey: *mut key) -> *mut request_key_auth {
    unsafe {
        let mut rka: *mut request_key_auth;

        down_read(&mut (*authkey).sem);
        rka = dereference_key_locked(authkey);
        if !rka.is_null() && !test_bit(KEY_FLAG_REVOKED, &(*authkey).flags) {
            refcount_inc(&mut (*rka).usage);
        } else {
            rka = ptr::null_mut();
        }
        up_read(&mut (*authkey).sem);

        rka
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_key_auth_put(rka: *mut request_key_auth) {
    unsafe {
        if !rka.is_null() && refcount_dec_and_test(&mut (*rka).usage) {
            call_rcu(&mut (*rka).rcu, request_key_auth_rcu_disposal);
        }
    }
}

/*
 * Dispose of the request_key_auth record under RCU conditions
 */
unsafe extern "C" fn request_key_auth_rcu_disposal(rcu: *mut rcu_head) {
    unsafe {
        let rka: *mut request_key_auth = container_of_request_key_auth_from_rcu(rcu);

        free_request_key_auth(rka);
    }
}

/*
 * Handle revocation of an authorisation token key.
 *
 * Called with the key sem write-locked.
 */
unsafe extern "C" fn request_key_auth_revoke(key: *mut key) {
    unsafe {
        let rka: *mut request_key_auth = dereference_key_locked(key);

        kenter(c"{%d}".as_ptr(), (*key).serial);
        if rka.is_null() {
            return;
        }
        rcu_assign_keypointer(key, ptr::null_mut());
        request_key_auth_put(rka);
    }
}

/*
 * Destroy an instantiation authorisation token key.
 */
unsafe extern "C" fn request_key_auth_destroy(key: *mut key) {
    unsafe {
        let rka: *mut request_key_auth = (*key).payload.rcu_data0.cast::<request_key_auth>();

        kenter(c"{%d}".as_ptr(), (*key).serial);
        if !rka.is_null() {
            rcu_assign_keypointer(key, ptr::null_mut());
            request_key_auth_put(rka);
        }
    }
}

/*
 * Create an authorisation token for /sbin/request-key or whoever to gain
 * access to the caller's security data.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_key_auth_new(
    target: *mut key,
    op: *const c_char,
    callout_info: *const c_void,
    callout_len: size_t,
    dest_keyring: *mut key,
) -> *mut key {
    unsafe {
        let rka: *mut request_key_auth;
        let irka: *mut request_key_auth;
        let cred: *const cred = current_cred();
        let mut authkey: *mut key = ptr::null_mut();
        let mut desc: [c_char; 20] = [0; 20];
        let mut ret: c_int = -(ENOMEM as c_int);

        kenter(c"%d,".as_ptr(), (*target).serial);

        /* allocate a auth record */
        rka = kzalloc(size_of::<request_key_auth>(), GFP_KERNEL).cast::<request_key_auth>();
        if rka.is_null() {
            kleave(c"= %d".as_ptr(), ret);
            return err_ptr(ret as c_long);
        }
        refcount_set(&mut (*rka).usage, 1);
        (*rka).callout_info = kmemdup(callout_info, callout_len, GFP_KERNEL);
        if (*rka).callout_info.is_null() {
            free_request_key_auth(rka);
            kleave(c"= %d".as_ptr(), ret);
            return err_ptr(ret as c_long);
        }
        (*rka).callout_len = callout_len;
        strscpy((*rka).op.as_mut_ptr(), op, (*rka).op.len());

        /* see if the calling process is already servicing the key request of
         * another process */
        if !(*cred).request_key_auth.is_null() {
            /* it is - use that instantiation context here too */
            down_read(&mut (*(*cred).request_key_auth).sem);

            /* if the auth key has been revoked, then the key we're
             * servicing is already instantiated */
            if test_bit(KEY_FLAG_REVOKED, &(*(*cred).request_key_auth).flags) {
                up_read(&mut (*(*cred).request_key_auth).sem);
                ret = -(EKEYREVOKED as c_int);
                free_request_key_auth(rka);
                kleave(c"= %d".as_ptr(), ret);
                return err_ptr(ret as c_long);
            }

            irka = (*(*cred).request_key_auth).payload.data[0].cast::<request_key_auth>();
            (*rka).cred = get_cred((*irka).cred);
            (*rka).pid = (*irka).pid;

            up_read(&mut (*(*cred).request_key_auth).sem);
        } else {
            /* it isn't - use this process as the context */
            (*rka).cred = get_cred(cred);
            (*rka).pid = (*current).pid;
        }

        (*rka).target_key = key_get(target);
        (*rka).dest_keyring = key_get(dest_keyring);

        /* allocate the auth key */
        sprintf(desc.as_mut_ptr(), c"%x".as_ptr(), (*target).serial);

        authkey = key_alloc(
            &mut key_type_request_key_auth,
            desc.as_ptr(),
            ptr::read(&(*cred).fsuid),
            ptr::read(&(*cred).fsgid),
            cred,
            KEY_POS_VIEW | KEY_POS_READ | KEY_POS_SEARCH | KEY_POS_LINK | KEY_USR_VIEW,
            KEY_ALLOC_NOT_IN_QUOTA,
            ptr::null_mut(),
        );
        if is_err(authkey) {
            ret = ptr_err(authkey);
            free_request_key_auth(rka);
            kleave(c"= %d".as_ptr(), ret);
            return err_ptr(ret as c_long);
        }

        /* construct the auth key */
        ret = key_instantiate_and_link(
            authkey,
            rka.cast::<c_void>(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if ret < 0 {
            key_put(authkey);
            free_request_key_auth(rka);
            kleave(c"= %d".as_ptr(), ret);
            return err_ptr(ret as c_long);
        }

        kleave(
            c" = {%d,%d}".as_ptr(),
            (*authkey).serial,
            refcount_read(&(*authkey).usage),
        );
        authkey
    }
}

/*
 * Search the current process's keyrings for the authorisation key for
 * instantiation of a key.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_get_instantiation_authkey(target_id: key_serial_t) -> *mut key {
    unsafe {
        let mut description: [c_char; 16] = [0; 16];
        let mut ctx: keyring_search_context = keyring_search_context {
            index_key: keyring_index_key {
                type_: &mut key_type_request_key_auth,
                description: description.as_mut_ptr(),
                desc_len: 0,
            },
            cred: current_cred(),
            match_data: key_match_data {
                cmp: key_default_cmp,
                raw_data: description.as_ptr().cast::<c_void>(),
                lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
            },
            flags: KEYRING_SEARCH_DO_STATE_CHECK | KEYRING_SEARCH_RECURSE,
        };
        let mut authkey: *mut key;
        let authkey_ref: key_ref_t;

        ctx.index_key.desc_len = sprintf(description.as_mut_ptr(), c"%x".as_ptr(), target_id);

        rcu_read_lock();
        authkey_ref = search_process_keyrings_rcu(&mut ctx);
        rcu_read_unlock();

        if is_err(authkey_ref) {
            authkey = err_cast(authkey_ref);
            if authkey == err_ptr(-EAGAIN) {
                authkey = err_ptr(-ENOKEY);
            }
            return authkey;
        }

        authkey = key_ref_to_ptr(authkey_ref);
        if test_bit(KEY_FLAG_REVOKED, &(*authkey).flags) {
            key_put(authkey);
            authkey = err_ptr(-EKEYREVOKED);
        }

        authkey
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

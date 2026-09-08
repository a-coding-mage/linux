// SPDX-License-Identifier: GPL-2.0-or-later
/* Basic authentication token and access key management
 *
 * Copyright (C) 2004-2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type size_t = usize;
type bool_t = bool;
type key_serial_t = c_int;
type key_perm_t = c_uint;
type key_ref_t = *mut c_void;
type time64_t = i64;
type kuid_t = c_uint;
type kgid_t = c_uint;

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}
#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct lock_class_key {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cred {
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
}
#[repr(C)]
pub struct keyring_index_key {
    pub type_: *mut key_type,
    pub description: *const c_char,
    pub desc_len: size_t,
}
#[repr(C)]
pub struct key_payload {
    pub data: [*mut c_void; 4],
}
#[repr(C)]
pub struct key_preparsed_payload {
    pub orig_description: *const c_char,
    pub description: *const c_char,
    pub data: *const c_void,
    pub datalen: size_t,
    pub quotalen: size_t,
    pub expiry: time64_t,
    pub payload: key_payload,
}
#[repr(C)]
pub struct key_restriction {
    pub check: Option<unsafe extern "C" fn(*mut key, *mut key_type, *mut key_payload, *mut key) -> c_int>,
    pub key: *mut key,
}
#[repr(C)]
pub struct key_user {
    pub usage: refcount_t,
    pub nkeys: atomic_t,
    pub nikeys: atomic_t,
    pub uid: kuid_t,
    pub qnkeys: c_uint,
    pub qnbytes: c_uint,
    pub lock: spinlock_t,
    pub cons_lock: mutex,
    pub node: rb_node,
}
#[repr(C)]
pub struct domain_tag {
    pub usage: refcount_t,
}
#[repr(C)]
pub struct key {
    pub index_key: keyring_index_key,
    pub usage: refcount_t,
    pub sem: rw_semaphore,
    pub user: *mut key_user,
    pub quotalen: size_t,
    pub datalen: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub perm: key_perm_t,
    pub expiry: time64_t,
    pub restrict_link: *mut key_restriction,
    pub last_used_at: time64_t,
    pub flags: c_ulong,
    pub magic: c_uint,
    pub domain_tag: *mut domain_tag,
    pub serial: key_serial_t,
    pub serial_node: rb_node,
    pub description: *mut c_char,
    pub type_: *mut key_type,
    pub state: c_int,
    pub payload: key_payload,
    pub revoked_at: time64_t,
}
#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub def_datalen: size_t,
    pub vet_description: Option<unsafe extern "C" fn(*const c_char) -> c_int>,
    pub instantiate: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub update: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub read: Option<unsafe extern "C" fn()>,
    pub revoke: Option<unsafe extern "C" fn(*mut key)>,
    pub link: list_head,
    pub lock_class: lock_class_key,
}
#[repr(C)]
pub struct assoc_array_edit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const EDQUOT: c_int = 122;
pub const EBUSY: c_int = 16;
pub const EPERM: c_int = 1;
pub const ENOKEY: c_int = 126;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EEXIST: c_int = 17;
pub const EOPNOTSUPP: c_int = 95;
pub const GFP_KERNEL: c_uint = 0;
pub const SLAB_HWCACHE_ALIGN: c_uint = 0;
pub const SLAB_PANIC: c_uint = 0;
pub const SLAB_NO_MERGE: c_uint = 0;
pub const TIME64_MAX: time64_t = i64::MAX;
pub const GLOBAL_ROOT_UID: kuid_t = 0;
pub const KEY_DEBUG_MAGIC: c_uint = 0;
pub const KEY_FLAG_USER_ALIVE: c_int = 0;
pub const KEY_FLAG_IN_QUOTA: c_int = 1;
pub const KEY_FLAG_BUILTIN: c_int = 2;
pub const KEY_FLAG_UID_KEYRING: c_int = 3;
pub const KEY_FLAG_KEEP: c_int = 4;
pub const KEY_FLAG_USER_CONSTRUCT: c_int = 5;
pub const KEY_FLAG_REVOKED: c_int = 6;
pub const KEY_FLAG_INVALIDATED: c_int = 7;
pub const KEY_ALLOC_NOT_IN_QUOTA: c_ulong = 1 << 0;
pub const KEY_ALLOC_QUOTA_OVERRUN: c_ulong = 1 << 1;
pub const KEY_ALLOC_BUILT_IN: c_ulong = 1 << 2;
pub const KEY_ALLOC_UID_KEYRING: c_ulong = 1 << 3;
pub const KEY_ALLOC_SET_KEEP: c_ulong = 1 << 4;
pub const KEY_ALLOC_BYPASS_RESTRICTION: c_ulong = 1 << 5;
pub const KEY_IS_POSITIVE: c_int = 1;
pub const KEY_IS_UNINSTANTIATED: c_int = 0;
pub const NOTIFY_KEY_INSTANTIATED: c_int = 0;
pub const NOTIFY_KEY_UPDATED: c_int = 1;
pub const NOTIFY_KEY_REVOKED: c_int = 2;
pub const NOTIFY_KEY_INVALIDATED: c_int = 3;
pub const KEY_NEED_WRITE: c_int = 0;
pub const KEY_PERM_UNDEF: key_perm_t = !0;
pub const KEY_POS_VIEW: key_perm_t = 1 << 0;
pub const KEY_POS_SEARCH: key_perm_t = 1 << 1;
pub const KEY_POS_LINK: key_perm_t = 1 << 2;
pub const KEY_POS_SETATTR: key_perm_t = 1 << 3;
pub const KEY_USR_VIEW: key_perm_t = 1 << 4;
pub const KEY_POS_READ: key_perm_t = 1 << 5;
pub const KEY_POS_WRITE: key_perm_t = 1 << 6;

extern "C" {
    pub static mut key_jar: *mut kmem_cache;
    pub static mut key_serial_tree: rb_root;
    pub static mut key_serial_lock: spinlock_t;
    pub static mut key_user_tree: rb_root;
    pub static mut key_user_lock: spinlock_t;
    pub static mut key_quota_root_maxkeys: c_uint;
    pub static mut key_quota_root_maxbytes: c_uint;
    pub static mut key_quota_maxkeys: c_uint;
    pub static mut key_quota_maxbytes: c_uint;
    static mut key_types_list: list_head;
    static mut key_types_sem: rw_semaphore;
    static mut key_construction_mutex: mutex;
    static mut key_gc_work: work_struct;
    static mut key_type_keyring: key_type;
    static mut key_type_dead: key_type;
    static mut key_type_user: key_type;
    static mut key_type_logon: key_type;
    static mut root_key_user: key_user;
    static mut key_gc_delay: time64_t;

    fn printk(fmt: *const c_char, ...) -> c_int;
    fn BUG() -> !;
    fn kmalloc_obj_key_user() -> *mut key_user;
    fn kfree(ptr: *mut c_void);
    fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: c_uint) -> *mut key;
    fn kmem_cache_free(cache: *mut kmem_cache, ptr: *mut c_void);
    fn kmem_cache_create(name: *const c_char, size: size_t, align: size_t, flags: c_uint, ctor: *mut c_void) -> *mut kmem_cache;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn get_random_bytes(buf: *mut c_void, nbytes: size_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn init_rwsem(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn down_write_nested(sem: *mut rw_semaphore, subclass: c_int);
    fn up_write(sem: *mut rw_semaphore);
    fn downgrade_write(sem: *mut rw_semaphore);
    fn lockdep_set_class(sem: *mut rw_semaphore, key: *mut lock_class_key);
    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool_t;
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool_t;
    fn refcount_inc_not_zero(r: *mut refcount_t) -> bool_t;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_inc(v: *mut atomic_t);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn uid_lt(a: kuid_t, b: kuid_t) -> bool_t;
    fn uid_gt(a: kuid_t, b: kuid_t) -> bool_t;
    fn uid_eq(a: kuid_t, b: kuid_t) -> bool_t;
    fn key_set_index_key(index_key: *mut keyring_index_key);
    fn security_key_alloc(key: *mut key, cred: *const cred, flags: c_ulong) -> c_int;
    fn security_key_post_create_or_update(keyring: *mut key, key: *mut key, payload: *const c_void, plen: size_t, flags: c_ulong, create: bool_t);
    fn ktime_get_real_seconds() -> time64_t;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool_t;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit_unlock(nr: c_int, addr: *mut c_ulong);
    fn test_and_clear_bit(nr: c_int, addr: *mut c_ulong) -> bool_t;
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> bool_t;
    fn smp_store_release(p: *mut c_int, v: c_int);
    fn notify_key(key: *mut key, event: c_int, aux: c_int);
    fn __key_link(keyring: *mut key, key: *mut key, edit: *mut *mut assoc_array_edit);
    fn __key_link_lock(keyring: *mut key, index_key: *mut keyring_index_key) -> c_int;
    fn __key_link_begin(keyring: *mut key, index_key: *mut keyring_index_key, edit: *mut *mut assoc_array_edit) -> c_int;
    fn __key_link_end(keyring: *mut key, index_key: *mut keyring_index_key, edit: *mut assoc_array_edit);
    fn key_set_expiry(key: *mut key, expiry: time64_t);
    fn wake_up_bit(word: *mut c_ulong, bit: c_int);
    fn schedule_work(work: *mut work_struct);
    fn key_permission(key_ref: key_ref_t, perm: c_int) -> c_int;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn key_ref_put(key_ref: key_ref_t);
    fn find_key_to_update(keyring_ref: key_ref_t, index_key: *mut keyring_index_key) -> key_ref_t;
    fn current_cred() -> *const cred;
    fn make_key_ref(key: *mut key, possessed: bool_t) -> key_ref_t;
    fn is_key_possessed(key_ref: key_ref_t) -> bool_t;
    fn wait_for_key_construction(key: *mut key, intr: bool_t) -> c_int;
    fn key_serial(key: *mut key) -> key_serial_t;
    fn key_schedule_gc(time: time64_t);
    fn key_schedule_gc_links();
    fn rcu_assign_keypointer(key: *mut key, ptr: *mut c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn key_gc_keytype(ktype: *mut key_type);
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_devel(fmt: *const c_char, ...);
    fn kenter(fmt: *const c_char, ...);
}

#[inline]
unsafe fn ERR_PTR<T>(err: c_long) -> *mut T {
    err as isize as *mut T
}
#[inline]
unsafe fn ERR_CAST<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}
#[inline]
unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}
#[inline]
unsafe fn unlikely(x: bool) -> bool {
    x
}
#[inline]
unsafe fn rb_entry_key_user_node(node: *mut rb_node) -> *mut key_user {
    (node as *mut u8).sub(core::mem::offset_of!(key_user, node)) as *mut key_user
}
#[inline]
unsafe fn rb_entry_key_serial_node(node: *mut rb_node) -> *mut key {
    (node as *mut u8).sub(core::mem::offset_of!(key, serial_node)) as *mut key
}
#[inline]
unsafe fn key_check(_key: *mut key) {}

/* KEY_DEBUGGING conditional in C. */
#[no_mangle]
pub unsafe extern "C" fn __key_check(key: *const key) {
    printk(
        b"__key_check: key %p {%08x} should be {%08x}\n\0".as_ptr() as *const c_char,
        key,
        (*key).magic,
        KEY_DEBUG_MAGIC,
    );
    BUG();
}

/*
 * Get the key quota record for a user, allocating a new record if one doesn't
 * already exist.
 */
#[no_mangle]
pub unsafe extern "C" fn key_user_lookup(uid: kuid_t) -> *mut key_user {
    let mut candidate: *mut key_user = null_mut();
    let mut user: *mut key_user;
    let mut parent: *mut rb_node;
    let mut p: *mut *mut rb_node;

    loop {
        parent = null_mut();
        p = &mut key_user_tree.rb_node;
        spin_lock(&mut key_user_lock);

        while !(*p).is_null() {
            parent = *p;
            user = rb_entry_key_user_node(parent);

            if uid_lt(uid, (*user).uid) {
                p = &mut (**p).rb_left;
            } else if uid_gt(uid, (*user).uid) {
                p = &mut (**p).rb_right;
            } else {
                refcount_inc(&mut (*user).usage);
                spin_unlock(&mut key_user_lock);
                kfree(candidate as *mut c_void);
                return user;
            }
        }

        if candidate.is_null() {
            spin_unlock(&mut key_user_lock);
            candidate = kmalloc_obj_key_user();
            if unlikely(candidate.is_null()) {
                return null_mut();
            }
            continue;
        }

        refcount_set(&mut (*candidate).usage, 1);
        atomic_set(&mut (*candidate).nkeys, 0);
        atomic_set(&mut (*candidate).nikeys, 0);
        (*candidate).uid = uid;
        (*candidate).qnkeys = 0;
        (*candidate).qnbytes = 0;
        spin_lock_init(&mut (*candidate).lock);
        mutex_init(&mut (*candidate).cons_lock);

        rb_link_node(&mut (*candidate).node, parent, p);
        rb_insert_color(&mut (*candidate).node, &mut key_user_tree);
        spin_unlock(&mut key_user_lock);
        return candidate;
    }
}

/*
 * Dispose of a user structure
 */
#[no_mangle]
pub unsafe extern "C" fn key_user_put(user: *mut key_user) {
    if refcount_dec_and_lock(&mut (*user).usage, &mut key_user_lock) {
        rb_erase(&mut (*user).node, &mut key_user_tree);
        spin_unlock(&mut key_user_lock);
        kfree(user as *mut c_void);
    }
}

/*
 * Allocate a serial number for a key.  These are assigned randomly to avoid
 * security issues through covert channel problems.
 */
#[inline]
unsafe fn key_alloc_serial(key: *mut key) {
    let mut parent: *mut rb_node;
    let mut p: *mut *mut rb_node;
    let mut xkey: *mut key;

    loop {
        get_random_bytes(&mut (*key).serial as *mut _ as *mut c_void, size_of::<key_serial_t>());
        (*key).serial >>= 1;
        if (*key).serial >= 3 {
            break;
        }
    }

    spin_lock(&mut key_serial_lock);

    'attempt_insertion: loop {
        parent = null_mut();
        p = &mut key_serial_tree.rb_node;

        while !(*p).is_null() {
            parent = *p;
            xkey = rb_entry_key_serial_node(parent);

            if (*key).serial < (*xkey).serial {
                p = &mut (**p).rb_left;
            } else if (*key).serial > (*xkey).serial {
                p = &mut (**p).rb_right;
            } else {
                loop {
                    (*key).serial = (*key).serial.wrapping_add(1);
                    if (*key).serial < 3 {
                        (*key).serial = 3;
                        continue 'attempt_insertion;
                    }
                    parent = rb_next(parent);
                    if parent.is_null() {
                        continue 'attempt_insertion;
                    }
                    xkey = rb_entry_key_serial_node(parent);
                    if (*key).serial < (*xkey).serial {
                        continue 'attempt_insertion;
                    }
                }
            }
        }

        rb_link_node(&mut (*key).serial_node, parent, p);
        rb_insert_color(&mut (*key).serial_node, &mut key_serial_tree);
        spin_unlock(&mut key_serial_lock);
        return;
    }
}

#[no_mangle]
pub unsafe extern "C" fn key_alloc(type_: *mut key_type, desc: *const c_char, uid: kuid_t, gid: kgid_t, cred: *const cred, perm: key_perm_t, flags: c_ulong, restrict_link: *mut key_restriction) -> *mut key {
    let mut user: *mut key_user = null_mut();
    let mut keyp: *mut key;
    let desclen: size_t;
    let quotalen: size_t;
    let mut ret: c_int;
    let irqflags: c_ulong = 0;

    keyp = ERR_PTR(-(EINVAL as c_long));
    if desc.is_null() || *desc == 0 {
        return keyp;
    }

    if let Some(vet_description) = (*type_).vet_description {
        ret = vet_description(desc);
        if ret < 0 {
            return ERR_PTR(ret as c_long);
        }
    }

    desclen = strlen(desc);
    quotalen = desclen + 1 + (*type_).def_datalen;

    user = key_user_lookup(uid);
    if user.is_null() {
        return ERR_PTR(-(ENOMEM as c_long));
    }

    if flags & KEY_ALLOC_NOT_IN_QUOTA == 0 {
        let maxkeys = if uid_eq(uid, GLOBAL_ROOT_UID) { key_quota_root_maxkeys } else { key_quota_maxkeys };
        let maxbytes = if uid_eq(uid, GLOBAL_ROOT_UID) { key_quota_root_maxbytes } else { key_quota_maxbytes };

        spin_lock_irqsave(&mut (*user).lock, irqflags);
        if flags & KEY_ALLOC_QUOTA_OVERRUN == 0 {
            if (*user).qnkeys + 1 > maxkeys ||
                (*user).qnbytes.wrapping_add(quotalen as c_uint) > maxbytes ||
                (*user).qnbytes.wrapping_add(quotalen as c_uint) < (*user).qnbytes {
                spin_unlock_irqrestore(&mut (*user).lock, irqflags);
                key_user_put(user);
                return ERR_PTR(-(EDQUOT as c_long));
            }
        }
        (*user).qnkeys += 1;
        (*user).qnbytes = (*user).qnbytes.wrapping_add(quotalen as c_uint);
        spin_unlock_irqrestore(&mut (*user).lock, irqflags);
    }

    keyp = kmem_cache_zalloc(key_jar, GFP_KERNEL);
    if keyp.is_null() {
        if flags & KEY_ALLOC_NOT_IN_QUOTA == 0 {
            spin_lock_irqsave(&mut (*user).lock, irqflags);
            (*user).qnkeys -= 1;
            (*user).qnbytes = (*user).qnbytes.wrapping_sub(quotalen as c_uint);
            spin_unlock_irqrestore(&mut (*user).lock, irqflags);
        }
        key_user_put(user);
        return ERR_PTR(-(ENOMEM as c_long));
    }

    (*keyp).index_key.desc_len = desclen;
    (*keyp).index_key.description = kmemdup(desc as *const c_void, desclen + 1, GFP_KERNEL);
    if (*keyp).index_key.description.is_null() {
        kmem_cache_free(key_jar, keyp as *mut c_void);
        if flags & KEY_ALLOC_NOT_IN_QUOTA == 0 {
            spin_lock_irqsave(&mut (*user).lock, irqflags);
            (*user).qnkeys -= 1;
            (*user).qnbytes = (*user).qnbytes.wrapping_sub(quotalen as c_uint);
            spin_unlock_irqrestore(&mut (*user).lock, irqflags);
        }
        key_user_put(user);
        return ERR_PTR(-(ENOMEM as c_long));
    }
    (*keyp).index_key.type_ = type_;
    key_set_index_key(&mut (*keyp).index_key);

    refcount_set(&mut (*keyp).usage, 1);
    init_rwsem(&mut (*keyp).sem);
    lockdep_set_class(&mut (*keyp).sem, &mut (*type_).lock_class);
    (*keyp).user = user;
    (*keyp).quotalen = quotalen;
    (*keyp).datalen = (*type_).def_datalen as c_int;
    (*keyp).uid = uid;
    (*keyp).gid = gid;
    (*keyp).perm = perm;
    (*keyp).expiry = TIME64_MAX;
    (*keyp).restrict_link = restrict_link;
    (*keyp).last_used_at = ktime_get_real_seconds();

    (*keyp).flags |= 1 << KEY_FLAG_USER_ALIVE;
    if flags & KEY_ALLOC_NOT_IN_QUOTA == 0 {
        (*keyp).flags |= 1 << KEY_FLAG_IN_QUOTA;
    }
    if flags & KEY_ALLOC_BUILT_IN != 0 {
        (*keyp).flags |= 1 << KEY_FLAG_BUILTIN;
    }
    if flags & KEY_ALLOC_UID_KEYRING != 0 {
        (*keyp).flags |= 1 << KEY_FLAG_UID_KEYRING;
    }
    if flags & KEY_ALLOC_SET_KEEP != 0 {
        (*keyp).flags |= 1 << KEY_FLAG_KEEP;
    }
    (*keyp).magic = KEY_DEBUG_MAGIC;

    ret = security_key_alloc(keyp, cred, flags);
    if ret < 0 {
        kfree((*keyp).description as *mut c_void);
        kmem_cache_free(key_jar, keyp as *mut c_void);
        if flags & KEY_ALLOC_NOT_IN_QUOTA == 0 {
            spin_lock_irqsave(&mut (*user).lock, irqflags);
            (*user).qnkeys -= 1;
            (*user).qnbytes = (*user).qnbytes.wrapping_sub(quotalen as c_uint);
            spin_unlock_irqrestore(&mut (*user).lock, irqflags);
        }
        key_user_put(user);
        return ERR_PTR(ret as c_long);
    }

    refcount_inc(&mut (*(*keyp).domain_tag).usage);
    atomic_inc(&mut (*user).nkeys);
    key_alloc_serial(keyp);
    keyp
}

#[no_mangle]
pub unsafe extern "C" fn key_payload_reserve(key: *mut key, datalen: size_t) -> c_int {
    let delta: c_int = datalen as c_int - (*key).datalen;
    let mut ret: c_int = 0;

    key_check(key);
    if delta != 0 && test_bit(KEY_FLAG_IN_QUOTA, &(*key).flags) {
        let maxbytes = if uid_eq((*(*key).user).uid, GLOBAL_ROOT_UID) { key_quota_root_maxbytes } else { key_quota_maxbytes };
        let flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*(*key).user).lock, flags);
        if delta > 0 &&
            ((*(*key).user).qnbytes.wrapping_add(delta as c_uint) > maxbytes ||
             (*(*key).user).qnbytes.wrapping_add(delta as c_uint) < (*(*key).user).qnbytes) {
            ret = -EDQUOT;
        } else {
            (*(*key).user).qnbytes = (*(*key).user).qnbytes.wrapping_add(delta as c_uint);
            (*key).quotalen = ((*key).quotalen as isize + delta as isize) as size_t;
        }
        spin_unlock_irqrestore(&mut (*(*key).user).lock, flags);
    }

    if ret == 0 {
        (*key).datalen = datalen as c_int;
    }
    ret
}

unsafe fn mark_key_instantiated(key: *mut key, reject_error: c_int) {
    smp_store_release(&mut (*key).state, if reject_error < 0 { reject_error } else { KEY_IS_POSITIVE });
}

unsafe fn __key_instantiate_and_link(key: *mut key, prep: *mut key_preparsed_payload, keyring: *mut key, authkey: *mut key, edit: *mut *mut assoc_array_edit) -> c_int {
    let mut ret: c_int;
    let mut awaken: c_int = 0;
    key_check(key);
    key_check(keyring);
    ret = -EBUSY;
    mutex_lock(&mut key_construction_mutex);
    if (*key).state == KEY_IS_UNINSTANTIATED {
        ret = ((*(*key).type_).instantiate.unwrap())(key, prep);
        if ret == 0 {
            atomic_inc(&mut (*(*key).user).nikeys);
            mark_key_instantiated(key, 0);
            notify_key(key, NOTIFY_KEY_INSTANTIATED, 0);
            if test_and_clear_bit(KEY_FLAG_USER_CONSTRUCT, &mut (*key).flags) {
                awaken = 1;
            }
            if !keyring.is_null() {
                if test_bit(KEY_FLAG_KEEP, &(*keyring).flags) {
                    set_bit(KEY_FLAG_KEEP, &mut (*key).flags);
                }
                __key_link(keyring, key, edit);
            }
            if !authkey.is_null() {
                key_invalidate(authkey);
            }
            if (*prep).expiry != TIME64_MAX {
                key_set_expiry(key, (*prep).expiry);
            }
        }
    }
    mutex_unlock(&mut key_construction_mutex);
    if awaken != 0 {
        wake_up_bit(&mut (*key).flags, KEY_FLAG_USER_CONSTRUCT);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn key_instantiate_and_link(key: *mut key, data: *const c_void, datalen: size_t, keyring: *mut key, authkey: *mut key) -> c_int {
    let mut prep: key_preparsed_payload = core::mem::zeroed();
    let mut edit: *mut assoc_array_edit = null_mut();
    let mut ret: c_int;

    prep.orig_description = (*key).description;
    prep.data = data;
    prep.datalen = datalen;
    prep.quotalen = (*(*key).type_).def_datalen;
    prep.expiry = TIME64_MAX;
    if let Some(preparse) = (*(*key).type_).preparse {
        ret = preparse(&mut prep);
        if ret < 0 {
            return ret;
        }
    }
    if !keyring.is_null() {
        ret = __key_link_lock(keyring, &mut (*key).index_key);
        if ret < 0 {
            if let Some(free_preparse) = (*(*key).type_).free_preparse { free_preparse(&mut prep); }
            return ret;
        }
        ret = __key_link_begin(keyring, &mut (*key).index_key, &mut edit);
        if ret < 0 {
            __key_link_end(keyring, &mut (*key).index_key, edit);
            if let Some(free_preparse) = (*(*key).type_).free_preparse { free_preparse(&mut prep); }
            return ret;
        }
        if !(*keyring).restrict_link.is_null() {
            let keyres = (*keyring).restrict_link;
            if let Some(check) = (*keyres).check {
                ret = check(keyring, (*key).type_, &mut prep.payload, (*keyres).key);
                if ret < 0 {
                    __key_link_end(keyring, &mut (*key).index_key, edit);
                    if let Some(free_preparse) = (*(*key).type_).free_preparse { free_preparse(&mut prep); }
                    return ret;
                }
            }
        }
    }
    ret = __key_instantiate_and_link(key, &mut prep, keyring, authkey, &mut edit);
    if !keyring.is_null() {
        __key_link_end(keyring, &mut (*key).index_key, edit);
    }
    if let Some(free_preparse) = (*(*key).type_).free_preparse {
        if (*(*key).type_).preparse.is_some() {
            free_preparse(&mut prep);
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn key_reject_and_link(key: *mut key, timeout: c_uint, error: c_uint, keyring: *mut key, authkey: *mut key) -> c_int {
    let mut edit: *mut assoc_array_edit = null_mut();
    let mut ret: c_int = -EBUSY;
    let mut awaken: c_int = 0;
    let mut link_ret: c_int = 0;
    key_check(key);
    key_check(keyring);
    if !keyring.is_null() {
        if !(*keyring).restrict_link.is_null() {
            return -EPERM;
        }
        link_ret = __key_link_lock(keyring, &mut (*key).index_key);
        if link_ret == 0 {
            link_ret = __key_link_begin(keyring, &mut (*key).index_key, &mut edit);
            if link_ret < 0 {
                __key_link_end(keyring, &mut (*key).index_key, edit);
            }
        }
    }
    mutex_lock(&mut key_construction_mutex);
    if (*key).state == KEY_IS_UNINSTANTIATED {
        atomic_inc(&mut (*(*key).user).nikeys);
        mark_key_instantiated(key, -(error as c_int));
        notify_key(key, NOTIFY_KEY_INSTANTIATED, -(error as c_int));
        key_set_expiry(key, ktime_get_real_seconds() + timeout as time64_t);
        if test_and_clear_bit(KEY_FLAG_USER_CONSTRUCT, &mut (*key).flags) {
            awaken = 1;
        }
        ret = 0;
        if !keyring.is_null() && link_ret == 0 {
            __key_link(keyring, key, &mut edit);
        }
        if !authkey.is_null() {
            key_invalidate(authkey);
        }
    }
    mutex_unlock(&mut key_construction_mutex);
    if !keyring.is_null() && link_ret == 0 {
        __key_link_end(keyring, &mut (*key).index_key, edit);
    }
    if awaken != 0 {
        wake_up_bit(&mut (*key).flags, KEY_FLAG_USER_CONSTRUCT);
    }
    if ret == 0 { link_ret } else { ret }
}

#[no_mangle]
pub unsafe extern "C" fn key_put(key: *mut key) {
    if !key.is_null() {
        key_check(key);
        if refcount_dec_and_test(&mut (*key).usage) {
            let flags: c_ulong = 0;
            if test_bit(KEY_FLAG_IN_QUOTA, &(*key).flags) {
                spin_lock_irqsave(&mut (*(*key).user).lock, flags);
                (*(*key).user).qnkeys -= 1;
                (*(*key).user).qnbytes = (*(*key).user).qnbytes.wrapping_sub((*key).quotalen as c_uint);
                spin_unlock_irqrestore(&mut (*(*key).user).lock, flags);
            }
            clear_bit_unlock(KEY_FLAG_USER_ALIVE, &mut (*key).flags);
            schedule_work(&mut key_gc_work);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn key_lookup(id: key_serial_t) -> *mut key {
    let mut n: *mut rb_node;
    let mut keyp: *mut key;
    spin_lock(&mut key_serial_lock);
    n = key_serial_tree.rb_node;
    while !n.is_null() {
        keyp = rb_entry_key_serial_node(n);
        if id < (*keyp).serial {
            n = (*n).rb_left;
        } else if id > (*keyp).serial {
            n = (*n).rb_right;
        } else {
            if !refcount_inc_not_zero(&mut (*keyp).usage) {
                spin_unlock(&mut key_serial_lock);
                return ERR_PTR(-(ENOKEY as c_long));
            }
            spin_unlock(&mut key_serial_lock);
            return keyp;
        }
    }
    spin_unlock(&mut key_serial_lock);
    ERR_PTR(-(ENOKEY as c_long))
}

#[no_mangle]
pub unsafe extern "C" fn key_type_lookup(type_: *const c_char) -> *mut key_type {
    let mut ktype: *mut key_type;
    down_read(&mut key_types_sem);
    ktype = key_types_list.next as *mut key_type;
    while (ktype as *mut list_head) != &mut key_types_list {
        if strcmp((*ktype).name, type_) == 0 {
            return ktype;
        }
        ktype = (*ktype).link.next as *mut key_type;
    }
    up_read(&mut key_types_sem);
    ERR_PTR(-(ENOKEY as c_long))
}

#[no_mangle]
pub unsafe extern "C" fn key_set_timeout(key: *mut key, timeout: c_uint) {
    let mut expiry: time64_t = TIME64_MAX;
    down_write(&mut (*key).sem);
    if timeout > 0 {
        expiry = ktime_get_real_seconds() + timeout as time64_t;
    }
    key_set_expiry(key, expiry);
    up_write(&mut (*key).sem);
}

#[no_mangle]
pub unsafe extern "C" fn key_type_put(_ktype: *mut key_type) {
    up_read(&mut key_types_sem);
}

unsafe fn __key_update(mut key_ref: key_ref_t, prep: *mut key_preparsed_payload) -> key_ref_t {
    let key = key_ref_to_ptr(key_ref);
    let mut ret = key_permission(key_ref, KEY_NEED_WRITE);
    if ret < 0 {
        key_put(key);
        return ERR_PTR(ret as c_long);
    }
    ret = -EEXIST;
    if (*(*key).type_).update.is_none() {
        key_put(key);
        return ERR_PTR(ret as c_long);
    }
    down_write(&mut (*key).sem);
    ret = ((*(*key).type_).update.unwrap())(key, prep);
    if ret == 0 {
        mark_key_instantiated(key, 0);
        notify_key(key, NOTIFY_KEY_UPDATED, 0);
    }
    up_write(&mut (*key).sem);
    if ret < 0 {
        key_put(key);
        key_ref = ERR_PTR(ret as c_long);
    }
    key_ref
}

unsafe fn __key_create_or_update(keyring_ref: key_ref_t, type_: *const c_char, description: *const c_char, payload: *const c_void, plen: size_t, mut perm: key_perm_t, flags: c_ulong, allow_update: bool_t) -> key_ref_t {
    let mut index_key: keyring_index_key = core::mem::zeroed();
    index_key.description = description;
    let mut prep: key_preparsed_payload = core::mem::zeroed();
    let mut edit: *mut assoc_array_edit = null_mut();
    let cred = current_cred();
    let keyring: *mut key;
    let mut keyp: *mut key;
    let mut key_ref: key_ref_t;
    let mut ret: c_int;
    let mut restrict_link: *mut key_restriction = null_mut();

    index_key.type_ = key_type_lookup(type_);
    if IS_ERR(index_key.type_) {
        return ERR_PTR(-(ENODEV as c_long));
    }
    if (*index_key.type_).instantiate.is_none() || (index_key.description.is_null() && (*index_key.type_).preparse.is_none()) {
        key_type_put(index_key.type_);
        return ERR_PTR(-(EINVAL as c_long));
    }
    keyring = key_ref_to_ptr(keyring_ref);
    key_check(keyring);
    if flags & KEY_ALLOC_BYPASS_RESTRICTION == 0 {
        restrict_link = (*keyring).restrict_link;
    }
    if (*keyring).type_ != &mut key_type_keyring {
        key_type_put(index_key.type_);
        return ERR_PTR(-(ENOTDIR as c_long));
    }
    prep.orig_description = description;
    prep.data = payload;
    prep.datalen = plen;
    prep.quotalen = (*index_key.type_).def_datalen;
    prep.expiry = TIME64_MAX;
    if let Some(preparse) = (*index_key.type_).preparse {
        ret = preparse(&mut prep);
        if ret < 0 {
            if let Some(free_preparse) = (*index_key.type_).free_preparse { free_preparse(&mut prep); }
            key_type_put(index_key.type_);
            return ERR_PTR(ret as c_long);
        }
        if index_key.description.is_null() {
            index_key.description = prep.description;
        }
        if index_key.description.is_null() {
            if let Some(free_preparse) = (*index_key.type_).free_preparse { free_preparse(&mut prep); }
            key_type_put(index_key.type_);
            return ERR_PTR(-(EINVAL as c_long));
        }
    }
    index_key.desc_len = strlen(index_key.description);
    key_set_index_key(&mut index_key);

    ret = __key_link_lock(keyring, &mut index_key);
    if ret < 0 {
        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
        key_type_put(index_key.type_);
        return ERR_PTR(ret as c_long);
    }
    ret = __key_link_begin(keyring, &mut index_key, &mut edit);
    if ret < 0 {
        __key_link_end(keyring, &mut index_key, edit);
        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
        key_type_put(index_key.type_);
        return ERR_PTR(ret as c_long);
    }
    if !restrict_link.is_null() {
        if let Some(check) = (*restrict_link).check {
            ret = check(keyring, index_key.type_, &mut prep.payload, (*restrict_link).key);
            if ret < 0 {
                key_ref = ERR_PTR(ret as c_long);
                __key_link_end(keyring, &mut index_key, edit);
                if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
                key_type_put(index_key.type_);
                return key_ref;
            }
        }
    }
    ret = key_permission(keyring_ref, KEY_NEED_WRITE);
    if ret < 0 {
        __key_link_end(keyring, &mut index_key, edit);
        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
        key_type_put(index_key.type_);
        return ERR_PTR(ret as c_long);
    }
    if allow_update {
        if (*index_key.type_).update.is_some() {
            key_ref = find_key_to_update(keyring_ref, &mut index_key);
            if !key_ref.is_null() {
                __key_link_end(keyring, &mut index_key, edit);
                keyp = key_ref_to_ptr(key_ref);
                if test_bit(KEY_FLAG_USER_CONSTRUCT, &(*keyp).flags) {
                    ret = wait_for_key_construction(keyp, true);
                    if ret < 0 {
                        key_ref_put(key_ref);
                        key_ref = ERR_PTR(ret as c_long);
                        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
                        key_type_put(index_key.type_);
                        return key_ref;
                    }
                }
                key_ref = __key_update(key_ref, &mut prep);
                if !IS_ERR(key_ref) {
                    security_key_post_create_or_update(keyring, keyp, payload, plen, flags, false);
                }
                if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
                key_type_put(index_key.type_);
                return key_ref;
            }
        }
    } else {
        key_ref = find_key_to_update(keyring_ref, &mut index_key);
        if !key_ref.is_null() {
            key_ref_put(key_ref);
            __key_link_end(keyring, &mut index_key, edit);
            if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
            key_type_put(index_key.type_);
            return ERR_PTR(-(EEXIST as c_long));
        }
    }
    if perm == KEY_PERM_UNDEF {
        perm = KEY_POS_VIEW | KEY_POS_SEARCH | KEY_POS_LINK | KEY_POS_SETATTR;
        perm |= KEY_USR_VIEW;
        if (*index_key.type_).read.is_some() {
            perm |= KEY_POS_READ;
        }
        if index_key.type_ == &mut key_type_keyring || (*index_key.type_).update.is_some() {
            perm |= KEY_POS_WRITE;
        }
    }
    keyp = key_alloc(index_key.type_, index_key.description, (*cred).fsuid, (*cred).fsgid, cred, perm, flags, null_mut());
    if IS_ERR(keyp) {
        __key_link_end(keyring, &mut index_key, edit);
        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
        key_type_put(index_key.type_);
        return ERR_CAST(keyp);
    }
    ret = __key_instantiate_and_link(keyp, &mut prep, keyring, null_mut(), &mut edit);
    if ret < 0 {
        key_put(keyp);
        __key_link_end(keyring, &mut index_key, edit);
        if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
        key_type_put(index_key.type_);
        return ERR_PTR(ret as c_long);
    }
    security_key_post_create_or_update(keyring, keyp, payload, plen, flags, true);
    key_ref = make_key_ref(keyp, is_key_possessed(keyring_ref));
    __key_link_end(keyring, &mut index_key, edit);
    if let Some(free_preparse) = (*index_key.type_).free_preparse { if (*index_key.type_).preparse.is_some() { free_preparse(&mut prep); } }
    key_type_put(index_key.type_);
    key_ref
}

#[no_mangle]
pub unsafe extern "C" fn key_create_or_update(keyring_ref: key_ref_t, type_: *const c_char, description: *const c_char, payload: *const c_void, plen: size_t, perm: key_perm_t, flags: c_ulong) -> key_ref_t {
    __key_create_or_update(keyring_ref, type_, description, payload, plen, perm, flags, true)
}

#[no_mangle]
pub unsafe extern "C" fn key_create(keyring_ref: key_ref_t, type_: *const c_char, description: *const c_char, payload: *const c_void, plen: size_t, perm: key_perm_t, flags: c_ulong) -> key_ref_t {
    __key_create_or_update(keyring_ref, type_, description, payload, plen, perm, flags, false)
}

#[no_mangle]
pub unsafe extern "C" fn key_update(key_ref: key_ref_t, payload: *const c_void, plen: size_t) -> c_int {
    let mut prep: key_preparsed_payload = core::mem::zeroed();
    let key = key_ref_to_ptr(key_ref);
    let mut ret: c_int;
    key_check(key);
    ret = key_permission(key_ref, KEY_NEED_WRITE);
    if ret < 0 {
        return ret;
    }
    if (*(*key).type_).update.is_none() {
        return -EOPNOTSUPP;
    }
    prep.data = payload;
    prep.datalen = plen;
    prep.quotalen = (*(*key).type_).def_datalen;
    prep.expiry = TIME64_MAX;
    if let Some(preparse) = (*(*key).type_).preparse {
        ret = preparse(&mut prep);
        if ret < 0 {
            return ret;
        }
    }
    down_write(&mut (*key).sem);
    ret = ((*(*key).type_).update.unwrap())(key, &mut prep);
    if ret == 0 {
        mark_key_instantiated(key, 0);
        notify_key(key, NOTIFY_KEY_UPDATED, 0);
    }
    up_write(&mut (*key).sem);
    if let Some(free_preparse) = (*(*key).type_).free_preparse {
        if (*(*key).type_).preparse.is_some() {
            free_preparse(&mut prep);
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn key_revoke(key: *mut key) {
    let time: time64_t;
    key_check(key);
    down_write_nested(&mut (*key).sem, 1);
    if !test_and_set_bit(KEY_FLAG_REVOKED, &mut (*key).flags) {
        notify_key(key, NOTIFY_KEY_REVOKED, 0);
        if let Some(revoke) = (*(*key).type_).revoke {
            revoke(key);
        }
        time = ktime_get_real_seconds();
        if (*key).revoked_at == 0 || (*key).revoked_at > time {
            (*key).revoked_at = time;
            key_schedule_gc((*key).revoked_at + key_gc_delay);
        }
    }
    up_write(&mut (*key).sem);
}

#[no_mangle]
pub unsafe extern "C" fn key_invalidate(key: *mut key) {
    kenter(b"%d\0".as_ptr() as *const c_char, key_serial(key));
    key_check(key);
    if !test_bit(KEY_FLAG_INVALIDATED, &(*key).flags) {
        down_write_nested(&mut (*key).sem, 1);
        if !test_and_set_bit(KEY_FLAG_INVALIDATED, &mut (*key).flags) {
            notify_key(key, NOTIFY_KEY_INVALIDATED, 0);
            key_schedule_gc_links();
        }
        up_write(&mut (*key).sem);
    }
}

#[no_mangle]
pub unsafe extern "C" fn generic_key_instantiate(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    let ret: c_int;
    pr_devel(b"==>%s()\n\0".as_ptr() as *const c_char, b"generic_key_instantiate\0".as_ptr() as *const c_char);
    ret = key_payload_reserve(key, (*prep).quotalen);
    if ret == 0 {
        rcu_assign_keypointer(key, (*prep).payload.data[0]);
        (*key).payload.data[1] = (*prep).payload.data[1];
        (*key).payload.data[2] = (*prep).payload.data[2];
        (*key).payload.data[3] = (*prep).payload.data[3];
        (*prep).payload.data[0] = null_mut();
        (*prep).payload.data[1] = null_mut();
        (*prep).payload.data[2] = null_mut();
        (*prep).payload.data[3] = null_mut();
    }
    pr_devel(b"<==%s() = %d\n\0".as_ptr() as *const c_char, b"generic_key_instantiate\0".as_ptr() as *const c_char, ret);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn register_key_type(ktype: *mut key_type) -> c_int {
    let mut p: *mut key_type;
    let mut ret: c_int;
    memset(&mut (*ktype).lock_class as *mut _ as *mut c_void, 0, size_of::<lock_class_key>());
    ret = -EEXIST;
    down_write(&mut key_types_sem);
    p = key_types_list.next as *mut key_type;
    while (p as *mut list_head) != &mut key_types_list {
        if strcmp((*p).name, (*ktype).name) == 0 {
            up_write(&mut key_types_sem);
            return ret;
        }
        p = (*p).link.next as *mut key_type;
    }
    list_add(&mut (*ktype).link, &mut key_types_list);
    pr_notice(b"Key type %s registered\n\0".as_ptr() as *const c_char, (*ktype).name);
    ret = 0;
    up_write(&mut key_types_sem);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn unregister_key_type(ktype: *mut key_type) {
    down_write(&mut key_types_sem);
    list_del_init(&mut (*ktype).link);
    downgrade_write(&mut key_types_sem);
    key_gc_keytype(ktype);
    pr_notice(b"Key type %s unregistered\n\0".as_ptr() as *const c_char, (*ktype).name);
    up_read(&mut key_types_sem);
}

/*
 * Initialise the key management state.
 */
#[no_mangle]
pub unsafe extern "C" fn key_init() {
    key_jar = kmem_cache_create(
        b"key_jar\0".as_ptr() as *const c_char,
        size_of::<key>(),
        0,
        SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_NO_MERGE,
        null_mut(),
    );
    list_add_tail(&mut key_type_keyring.link, &mut key_types_list);
    list_add_tail(&mut key_type_dead.link, &mut key_types_list);
    list_add_tail(&mut key_type_user.link, &mut key_types_list);
    list_add_tail(&mut key_type_logon.link, &mut key_types_list);
    rb_link_node(&mut root_key_user.node, null_mut(), &mut key_user_tree.rb_node);
    rb_insert_color(&mut root_key_user.node, &mut key_user_tree);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

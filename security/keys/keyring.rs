// SPDX-License-Identifier: GPL-2.0-or-later
/* Keyring handling
 *
 * Copyright (C) 2004-2005, 2008, 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type bool_t = bool;
type size_t = usize;
type u8_t = u8;
type u32_t = u32;
type u64_t = u64;
type time64_t = i64;
type key_serial_t = i32;
type key_perm_t = u32;
type key_ref_t = *mut c_void;
type kuid_t = u32;
type kgid_t = u32;
type key_restrict_link_func_t = Option<
    unsafe extern "C" fn(
        *mut key,
        *const key_type,
        *const key_payload,
        *mut key,
    ) -> c_int,
>;

const KEYRING_SEARCH_MAX_DEPTH: usize = 6;
const KEYRING_PTR_SUBTYPE: c_ulong = 0x2;

const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const EKEYREVOKED: c_int = 128;
const EKEYEXPIRED: c_int = 127;
const EACCES: c_int = 13;
const EAGAIN: c_int = 11;
const ENOKEY: c_int = 126;
const ENOTDIR: c_int = 20;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const EDEADLK: c_int = 35;
const ELOOP: c_int = 40;

const KEY_FLAG_INVALIDATED: c_int = 0;
const KEY_FLAG_REVOKED: c_int = 1;
const KEY_FLAG_UID_KEYRING: c_int = 2;
const KEY_TYPE_NET_DOMAIN: c_uint = 0x1;
const KEY_NEED_SEARCH: key_perm_t = 0x1;
const KEYQUOTA_LINK_BYTES: c_uint = 4;
const KEYCTL_MOVE_EXCL: c_uint = 0x1;
const NOTIFY_KEY_SETATTR: c_uint = 0;
const NOTIFY_KEY_LINKED: c_uint = 1;
const NOTIFY_KEY_UNLINKED: c_uint = 2;
const NOTIFY_KEY_CLEARED: c_uint = 3;
const KEYRING_SEARCH_NO_STATE_CHECK: c_uint = 0x0001;
const KEYRING_SEARCH_DO_STATE_CHECK: c_uint = 0x0002;
const KEYRING_SEARCH_SKIP_EXPIRED: c_uint = 0x0004;
const KEYRING_SEARCH_NO_CHECK_PERM: c_uint = 0x0008;
const KEYRING_SEARCH_RECURSE: c_uint = 0x0010;
const KEYRING_SEARCH_DETECT_TOO_DEEP: c_uint = 0x0020;
const KEYRING_SEARCH_NO_UPDATE_TIME: c_uint = 0x0040;
const KEYRING_SEARCH_LOOKUP_DIRECT: c_uint = 0;
const KEYRING_SEARCH_LOOKUP_ITERATE: c_uint = 1;
const ASSOC_ARRAY_LEVEL_STEP: c_uint = 4;
const ASSOC_ARRAY_FAN_MASK: c_ulong = 15;
const ASSOC_ARRAY_FAN_OUT: usize = 16;
const ASSOC_ARRAY_KEY_CHUNK_SIZE: c_uint = 32;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct refcount_t {
    refs: c_int,
}

#[repr(C)]
pub struct key_tag {
    usage: refcount_t,
    removed: bool_t,
    rcu: rcu_head,
}

#[repr(C)]
pub struct rcu_head {
    next: *mut c_void,
    func: *mut c_void,
}

#[repr(C)]
pub struct assoc_array_ptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct assoc_array_node {
    back_pointer: *mut assoc_array_ptr,
    parent_slot: c_int,
    slots: [*mut assoc_array_ptr; ASSOC_ARRAY_FAN_OUT],
}

#[repr(C)]
pub struct assoc_array_shortcut {
    back_pointer: *mut assoc_array_ptr,
    parent_slot: c_int,
    next_node: *mut assoc_array_ptr,
    index_key: [c_ulong; ASSOC_ARRAY_FAN_OUT],
}

#[repr(C)]
pub struct assoc_array {
    root: *mut assoc_array_ptr,
    nr_leaves_on_tree: c_ulong,
}

#[repr(C)]
pub struct assoc_array_edit {
    dead_leaf: *mut c_void,
}

#[repr(C)]
pub struct assoc_array_ops {
    get_key_chunk: Option<unsafe extern "C" fn(*const c_void, c_int) -> c_ulong>,
    get_object_key_chunk: Option<unsafe extern "C" fn(*const c_void, c_int) -> c_ulong>,
    compare_object: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> bool_t>,
    diff_objects: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    free_object: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct keyring_index_key {
    type_: *mut key_type,
    domain_tag: *mut key_tag,
    description: *const c_char,
    desc_len: size_t,
    hash: c_ulong,
    x: c_ulong,
    desc: [c_char; 16],
}

#[repr(C)]
pub union key_payload {
    data: *mut c_void,
}

#[repr(C)]
pub struct key_restriction {
    check: key_restrict_link_func_t,
    key: *mut key,
    keytype: *mut key_type,
}

#[repr(C)]
pub struct key {
    type_: *mut key_type,
    serial: key_serial_t,
    flags: c_ulong,
    expiry: time64_t,
    description: *const c_char,
    index_key: keyring_index_key,
    keys: assoc_array,
    name_link: list_head,
    restrict_link: *mut key_restriction,
    payload: key_payload,
    sem: rw_semaphore,
    datalen: c_uint,
    usage: refcount_t,
    last_used_at: time64_t,
    user: *mut key_user,
}

#[repr(C)]
pub struct key_user {
    uid: kuid_t,
}

#[repr(C)]
pub struct key_preparsed_payload {
    datalen: size_t,
}

#[repr(C)]
pub struct key_match_data {
    cmp: Option<unsafe extern "C" fn(*const key, *const key_match_data) -> bool_t>,
    raw_data: *const c_void,
    lookup_type: c_uint,
}

#[repr(C)]
pub struct keyring_search_context {
    index_key: keyring_index_key,
    cred: *const cred,
    match_data: key_match_data,
    flags: c_uint,
    iterator: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    possessed: bool_t,
    result: key_ref_t,
    now: time64_t,
    skipped_ret: c_int,
}

#[repr(C)]
pub struct key_type {
    name: *const c_char,
    def_datalen: c_uint,
    preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> c_int>,
    free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    instantiate: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    revoke: Option<unsafe extern "C" fn(*mut key)>,
    destroy: Option<unsafe extern "C" fn(*mut key)>,
    describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
    read: Option<unsafe extern "C" fn(*const key, *mut c_char, size_t) -> c_long>,
    flags: c_uint,
    match_preparse: Option<unsafe extern "C" fn(*mut key_match_data) -> c_int>,
    match_free: Option<unsafe extern "C" fn(*mut key_match_data)>,
    lookup_restriction: Option<unsafe extern "C" fn(*const c_char) -> *mut key_restriction>,
}

#[repr(C)]
pub struct user_namespace {
    keyring_name_list: list_head,
    user_keyring_register: *mut key,
    persistent_keyring_register: *mut key,
}

#[repr(C)]
pub struct nsproxy {
    net_ns: *mut net,
}

#[repr(C)]
pub struct net {
    key_domain: *mut key_tag,
}

#[repr(C)]
pub struct task_struct {
    nsproxy: *mut nsproxy,
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rwlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;

    fn current_user_ns() -> *mut user_namespace;
    fn current_cred() -> *const cred;
    fn key_put(key: *mut key);
    fn key_get(key: *mut key) -> *mut key;
    fn __key_get(key: *const key);
    fn key_check(key: *const key);
    fn key_read_state(key: *const key) -> i16;
    fn key_is_positive(key: *const key) -> bool_t;
    fn key_is_dead(key: *const key, limit: time64_t) -> bool_t;
    fn key_serial(key: *const key) -> key_serial_t;
    fn key_alloc(
        type_: *mut key_type,
        description: *const c_char,
        uid: kuid_t,
        gid: kgid_t,
        cred: *const cred,
        perm: key_perm_t,
        flags: c_ulong,
        restrict_link: *mut key_restriction,
    ) -> *mut key;
    fn key_instantiate_and_link(
        key: *mut key,
        data: *const c_void,
        datalen: size_t,
        dest: *mut key,
        authkey: *mut key,
    ) -> c_int;
    fn key_task_permission(key_ref: key_ref_t, cred: *const cred, perm: key_perm_t) -> c_int;
    fn key_permission(key_ref: key_ref_t, perm: key_perm_t) -> c_int;
    fn key_payload_reserve(key: *mut key, datalen: c_uint) -> c_int;
    fn key_type_lookup(type_: *const c_char) -> *mut key_type;
    fn key_type_put(type_: *mut key_type);
    fn notify_key(key: *mut key, op: c_uint, aux: key_serial_t);
    fn key_schedule_gc_links();

    fn assoc_array_init(array: *mut assoc_array);
    fn assoc_array_destroy(array: *mut assoc_array, ops: *const assoc_array_ops);
    fn assoc_array_find(
        array: *const assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const keyring_index_key,
    ) -> *const c_void;
    fn assoc_array_iterate(
        array: *const assoc_array,
        iterator: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn assoc_array_insert(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const keyring_index_key,
        object: *const c_void,
    ) -> *mut assoc_array_edit;
    fn assoc_array_insert_set_object(edit: *mut assoc_array_edit, object: *mut c_void);
    fn assoc_array_apply_edit(edit: *mut assoc_array_edit);
    fn assoc_array_cancel_edit(edit: *mut assoc_array_edit);
    fn assoc_array_delete(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const keyring_index_key,
    ) -> *mut assoc_array_edit;
    fn assoc_array_clear(array: *mut assoc_array, ops: *const assoc_array_ops) -> *mut assoc_array_edit;
    fn assoc_array_gc(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        select: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool_t>,
        data: *mut c_void,
    );
    fn assoc_array_ptr_to_leaf(ptr: *const assoc_array_ptr) -> *mut c_void;
    fn assoc_array_ptr_is_shortcut(ptr: *const assoc_array_ptr) -> bool_t;
    fn assoc_array_ptr_to_shortcut(ptr: *const assoc_array_ptr) -> *mut assoc_array_shortcut;
    fn assoc_array_ptr_to_node(ptr: *const assoc_array_ptr) -> *mut assoc_array_node;
    fn assoc_array_ptr_is_meta(ptr: *const assoc_array_ptr) -> bool_t;
    fn assoc_array_ptr_is_node(ptr: *const assoc_array_ptr) -> bool_t;

    fn write_lock(lock: *mut rwlock_t);
    fn write_unlock(lock: *mut rwlock_t);
    fn read_lock(lock: *mut rwlock_t);
    fn read_unlock(lock: *mut rwlock_t);
    fn down_write(sem: *mut rw_semaphore);
    fn down_write_nested(sem: *mut rw_semaphore, subclass: c_int);
    fn up_write(sem: *mut rw_semaphore);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn rcu_read_lock();
    fn rcu_read_unlock();

    fn list_del_init(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool_t;
    fn list_del(entry: *mut list_head);
    fn kuid_has_mapping(ns: *mut user_namespace, uid: kuid_t) -> bool_t;

    fn refcount_dec_and_test(r: *mut refcount_t) -> bool_t;
    fn refcount_inc_not_zero(r: *mut refcount_t) -> bool_t;
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kfree_rcu(tag: *mut key_tag, rcu: *mut rcu_head);
    fn kzalloc_obj_key_restriction() -> *mut key_restriction;
    fn ktime_get_real_seconds() -> time64_t;
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

static mut keyring_name_lock: rwlock_t = rwlock_t { _private: [] };
static mut keyring_serialise_link_lock: mutex = mutex { _private: [] };
static mut keyring_serialise_restrict_sem: rw_semaphore = rw_semaphore { _private: [] };

static KEYRING_NAME: &[u8] = b"keyring\0";

#[unsafe(no_mangle)]
pub static mut key_type_keyring: key_type = key_type {
    name: KEYRING_NAME.as_ptr() as *const c_char,
    def_datalen: 0,
    preparse: Some(keyring_preparse),
    free_preparse: Some(keyring_free_preparse),
    instantiate: Some(keyring_instantiate),
    revoke: Some(keyring_revoke),
    destroy: Some(keyring_destroy),
    describe: Some(keyring_describe),
    read: Some(keyring_read),
    flags: 0,
    match_preparse: None,
    match_free: None,
    lookup_restriction: None,
};

#[inline]
unsafe fn ERR_PTR(err: c_long) -> *mut c_void {
    err as isize as *mut c_void
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as isize as c_long
}

#[inline]
unsafe fn IS_ERR<T>(ptr: *const T) -> bool_t {
    (ptr as usize) >= (!4095usize)
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

#[inline]
unsafe fn test_bit(bit: c_int, addr: *const c_ulong) -> bool_t {
    ((*addr & (1usize << bit) as c_ulong) != 0)
}

#[inline]
unsafe fn make_key_ref(key: *const key, possessed: bool_t) -> key_ref_t {
    ((key as usize) | possessed as usize) as key_ref_t
}

#[inline]
unsafe fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key {
    ((key_ref as usize) & !1usize) as *mut key
}

#[inline]
unsafe fn is_key_possessed(key_ref: key_ref_t) -> bool_t {
    ((key_ref as usize) & 1) != 0
}

#[inline]
unsafe fn keyring_ptr_is_keyring(x: *const assoc_array_ptr) -> bool_t {
    ((x as c_ulong) & KEYRING_PTR_SUBTYPE) != 0
}

#[inline]
unsafe fn keyring_ptr_to_key(x: *const c_void) -> *mut key {
    let object = assoc_array_ptr_to_leaf(x as *const assoc_array_ptr);
    ((object as c_ulong) & !KEYRING_PTR_SUBTYPE) as *mut key
}

#[inline]
unsafe fn keyring_key_to_ptr(key: *mut key) -> *mut c_void {
    if (*key).type_ == &raw mut key_type_keyring {
        return ((key as c_ulong) | KEYRING_PTR_SUBTYPE) as *mut c_void;
    }
    key as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_free_user_ns(ns: *mut user_namespace) {
    write_lock(&raw mut keyring_name_lock);
    list_del_init(&mut (*ns).keyring_name_list);
    write_unlock(&raw mut keyring_name_lock);
    key_put((*ns).user_keyring_register);
    /* CONFIG_PERSISTENT_KEYRINGS */
    key_put((*ns).persistent_keyring_register);
}

unsafe extern "C" fn keyring_publish_name(keyring: *mut key) {
    let ns = current_user_ns();
    if !(*keyring).description.is_null()
        && *(*keyring).description != 0
        && *(*keyring).description != b'.' as c_char
    {
        write_lock(&raw mut keyring_name_lock);
        list_add_tail(&mut (*keyring).name_link, &mut (*ns).keyring_name_list);
        write_unlock(&raw mut keyring_name_lock);
    }
}

unsafe extern "C" fn keyring_preparse(prep: *mut key_preparsed_payload) -> c_int {
    if (*prep).datalen != 0 { -EINVAL } else { 0 }
}

unsafe extern "C" fn keyring_free_preparse(_prep: *mut key_preparsed_payload) {}

unsafe extern "C" fn keyring_instantiate(keyring: *mut key, _prep: *mut key_preparsed_payload) -> c_int {
    assoc_array_init(&mut (*keyring).keys);
    keyring_publish_name(keyring);
    0
}

unsafe fn mult_64x32_and_fold(x: u64_t, y: u32_t) -> u64_t {
    let hi = ((x >> 32) as u32_t as u64_t).wrapping_mul(y as u64_t);
    let lo = (x as u32_t as u64_t).wrapping_mul(y as u64_t);
    lo.wrapping_add((hi as u32_t as u64_t) << 32)
        .wrapping_add((hi >> 32) as u32_t as u64_t)
}

unsafe fn hash_key_type_and_desc(index_key: *mut keyring_index_key) {
    let level_shift = ASSOC_ARRAY_LEVEL_STEP;
    let fan_mask = ASSOC_ARRAY_FAN_MASK;
    let mut description = (*index_key).description;
    let mut hash: c_ulong;
    let type_ = (*index_key).type_ as c_ulong;
    let mut piece: u32_t;
    let mut desc_len = (*index_key).desc_len as c_int;
    let mut acc = mult_64x32_and_fold(type_ as u64_t, (desc_len + 13) as u32_t);
    acc = mult_64x32_and_fold(acc, 9207);
    piece = (*index_key).domain_tag as c_ulong as u32_t;
    acc = mult_64x32_and_fold(acc, piece);
    acc = mult_64x32_and_fold(acc, 9207);
    loop {
        let mut n = desc_len;
        if n <= 0 { break; }
        if n > 4 { n = 4; }
        piece = 0;
        memcpy(&mut piece as *mut _ as *mut c_void, description as *const c_void, n as size_t);
        description = description.add(n as usize);
        desc_len -= n;
        acc = mult_64x32_and_fold(acc, piece);
        acc = mult_64x32_and_fold(acc, 9207);
    }
    hash = acc as c_ulong;
    if ASSOC_ARRAY_KEY_CHUNK_SIZE == 32 {
        hash ^= (acc >> 32) as c_ulong;
    }
    if (*index_key).type_ != &raw mut key_type_keyring && (hash & fan_mask) == 0 {
        hash |= (hash >> (ASSOC_ARRAY_KEY_CHUNK_SIZE - level_shift)) | 1;
    } else if (*index_key).type_ == &raw mut key_type_keyring && (hash & fan_mask) != 0 {
        hash = hash.wrapping_add(hash << level_shift) & !fan_mask;
    }
    (*index_key).hash = hash;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_set_index_key(index_key: *mut keyring_index_key) {
    static mut DEFAULT_DOMAIN_TAG: key_tag = key_tag {
        usage: refcount_t { refs: 1 },
        removed: false,
        rcu: rcu_head { next: null_mut(), func: null_mut() },
    };
    let n = core::cmp::min((*index_key).desc_len, size_of_val(&(*index_key).desc));
    memcpy((*index_key).desc.as_mut_ptr() as *mut c_void, (*index_key).description as *const c_void, n);
    if (*index_key).domain_tag.is_null() {
        if ((*(*index_key).type_).flags & KEY_TYPE_NET_DOMAIN) != 0 {
            (*index_key).domain_tag = (*(*(*current).nsproxy).net_ns).key_domain;
        } else {
            (*index_key).domain_tag = &raw mut DEFAULT_DOMAIN_TAG;
        }
    }
    hash_key_type_and_desc(index_key);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_put_tag(tag: *mut key_tag) -> bool_t {
    if refcount_dec_and_test(&mut (*tag).usage) {
        kfree_rcu(tag, &mut (*tag).rcu);
        return true;
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_remove_domain(domain_tag: *mut key_tag) {
    (*domain_tag).removed = true;
    if !key_put_tag(domain_tag) {
        key_schedule_gc_links();
    }
}

unsafe extern "C" fn keyring_get_key_chunk(data: *const c_void, mut level: c_int) -> c_ulong {
    let index_key = data as *const keyring_index_key;
    let mut chunk: c_ulong = 0;
    let mut desc_len = (*index_key).desc_len as c_int;
    let n = size_of::<c_ulong>() as c_int;
    level /= ASSOC_ARRAY_KEY_CHUNK_SIZE as c_int;
    match level {
        0 => (*index_key).hash,
        1 => (*index_key).x,
        2 => (*index_key).type_ as c_ulong,
        3 => (*index_key).domain_tag as c_ulong,
        _ => {
            level -= 4;
            let offset = size_of_val(&(*index_key).desc) + level as usize * size_of::<c_long>();
            if desc_len <= offset as c_int { return 0; }
            let mut d = (*index_key).description.add(offset);
            desc_len -= offset as c_int;
            if desc_len > n { desc_len = n; }
            d = d.add(desc_len as usize);
            loop {
                chunk <<= 8;
                d = d.sub(1);
                chunk |= *d as u8_t as c_ulong;
                desc_len -= 1;
                if desc_len <= 0 { break; }
            }
            chunk
        }
    }
}

unsafe extern "C" fn keyring_get_object_key_chunk(object: *const c_void, level: c_int) -> c_ulong {
    let key = keyring_ptr_to_key(object);
    keyring_get_key_chunk(&(*key).index_key as *const _ as *const c_void, level)
}

unsafe extern "C" fn keyring_compare_object(object: *const c_void, data: *const c_void) -> bool_t {
    let index_key = data as *const keyring_index_key;
    let key = keyring_ptr_to_key(object);
    (*key).index_key.type_ == (*index_key).type_
        && (*key).index_key.domain_tag == (*index_key).domain_tag
        && (*key).index_key.desc_len == (*index_key).desc_len
        && memcmp(
            (*key).index_key.description as *const c_void,
            (*index_key).description as *const c_void,
            (*index_key).desc_len,
        ) == 0
}

unsafe extern "C" fn keyring_diff_objects(object: *const c_void, data: *const c_void) -> c_int {
    let key_a = keyring_ptr_to_key(object);
    let a = &(*key_a).index_key as *const keyring_index_key;
    let b = data as *const keyring_index_key;
    let mut level: c_int = 0;
    let mut seg_a = (*a).hash;
    let mut seg_b = (*b).hash;
    if (seg_a ^ seg_b) == 0 {
        level += (ASSOC_ARRAY_KEY_CHUNK_SIZE / 8) as c_int;
        seg_a = (*a).x;
        seg_b = (*b).x;
        if (seg_a ^ seg_b) == 0 {
            level += size_of::<c_ulong>() as c_int;
            seg_a = (*a).type_ as c_ulong;
            seg_b = (*b).type_ as c_ulong;
            if (seg_a ^ seg_b) == 0 {
                level += size_of::<c_ulong>() as c_int;
                seg_a = (*a).domain_tag as c_ulong;
                seg_b = (*b).domain_tag as c_ulong;
                if (seg_a ^ seg_b) == 0 {
                    level += size_of::<c_ulong>() as c_int;
                    let mut i = size_of_val(&(*a).desc) as c_int;
                    if (*a).desc_len <= i as usize { return -1; }
                    while i < (*a).desc_len as c_int {
                        seg_a = *(*a).description.add(i as usize) as u8_t as c_ulong;
                        seg_b = *(*b).description.add(i as usize) as u8_t as c_ulong;
                        if (seg_a ^ seg_b) != 0 {
                            level += i - size_of_val(&(*a).desc) as c_int;
                            return level * 8 + (seg_a ^ seg_b).trailing_zeros() as c_int;
                        }
                        i += 1;
                    }
                    return -1;
                }
            }
        }
    }
    level * 8 + (seg_a ^ seg_b).trailing_zeros() as c_int
}

unsafe extern "C" fn keyring_free_object(object: *mut c_void) {
    key_put(keyring_ptr_to_key(object));
}

static keyring_assoc_array_ops: assoc_array_ops = assoc_array_ops {
    get_key_chunk: Some(keyring_get_key_chunk),
    get_object_key_chunk: Some(keyring_get_object_key_chunk),
    compare_object: Some(keyring_compare_object),
    diff_objects: Some(keyring_diff_objects),
    free_object: Some(keyring_free_object),
};

unsafe extern "C" fn keyring_destroy(keyring: *mut key) {
    if !(*keyring).description.is_null() {
        write_lock(&raw mut keyring_name_lock);
        if !(*keyring).name_link.next.is_null() && !list_empty(&(*keyring).name_link) {
            list_del(&mut (*keyring).name_link);
        }
        write_unlock(&raw mut keyring_name_lock);
    }
    if !(*keyring).restrict_link.is_null() {
        let keyres = (*keyring).restrict_link;
        key_put((*keyres).key);
        kfree(keyres as *mut c_void);
    }
    assoc_array_destroy(&mut (*keyring).keys, &keyring_assoc_array_ops);
}

unsafe extern "C" fn keyring_describe(keyring: *const key, m: *mut seq_file) {
    if !(*keyring).description.is_null() {
        seq_puts(m, (*keyring).description);
    } else {
        seq_puts(m, c"[anon]".as_ptr());
    }
    if key_is_positive(keyring) {
        if (*keyring).keys.nr_leaves_on_tree != 0 {
            seq_printf(m, c": %lu".as_ptr(), (*keyring).keys.nr_leaves_on_tree);
        } else {
            seq_puts(m, c": empty".as_ptr());
        }
    }
}

#[repr(C)]
struct keyring_read_iterator_context {
    buflen: size_t,
    count: size_t,
    buffer: *mut key_serial_t,
}

unsafe extern "C" fn keyring_read_iterator(object: *const c_void, data: *mut c_void) -> c_int {
    let ctx = data as *mut keyring_read_iterator_context;
    let key = keyring_ptr_to_key(object);
    if (*ctx).count >= (*ctx).buflen { return 1; }
    *(*ctx).buffer = (*key).serial;
    (*ctx).buffer = (*ctx).buffer.add(1);
    (*ctx).count += size_of::<key_serial_t>();
    0
}

unsafe extern "C" fn keyring_read(keyring: *const key, buffer: *mut c_char, buflen: size_t) -> c_long {
    if (buflen & (size_of::<key_serial_t>() - 1)) != 0 { return -EINVAL as c_long; }
    if !buffer.is_null() && buflen != 0 {
        let mut ctx = keyring_read_iterator_context {
            buffer: buffer as *mut key_serial_t,
            buflen,
            count: 0,
        };
        let ret = assoc_array_iterate(&(*keyring).keys, Some(keyring_read_iterator), &mut ctx as *mut _ as *mut c_void);
        if ret < 0 { return ret as c_long; }
    }
    ((*keyring).keys.nr_leaves_on_tree as c_long) * size_of::<key_serial_t>() as c_long
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_alloc(
    description: *const c_char,
    uid: kuid_t,
    gid: kgid_t,
    cred: *const cred,
    perm: key_perm_t,
    flags: c_ulong,
    restrict_link: *mut key_restriction,
    dest: *mut key,
) -> *mut key {
    let mut keyring = key_alloc(&raw mut key_type_keyring, description, uid, gid, cred, perm, flags, restrict_link);
    if !IS_ERR(keyring) {
        let ret = key_instantiate_and_link(keyring, null(), 0, dest, null_mut());
        if ret < 0 {
            key_put(keyring);
            keyring = ERR_PTR(ret as c_long) as *mut key;
        }
    }
    keyring
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restrict_link_reject(
    _keyring: *mut key,
    _type: *const key_type,
    _payload: *const key_payload,
    _restriction_key: *mut key,
) -> c_int {
    -EPERM
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_default_cmp(key: *const key, match_data: *const key_match_data) -> bool_t {
    strcmp((*key).description, (*match_data).raw_data as *const c_char) == 0
}

unsafe extern "C" fn keyring_search_iterator(object: *const c_void, iterator_data: *mut c_void) -> c_int {
    let ctx = iterator_data as *mut keyring_search_context;
    let key = keyring_ptr_to_key(object);
    let kflags = READ_ONCE(&(*key).flags);
    let state = key_read_state(key);
    if (*key).type_ != (*ctx).index_key.type_ { return 0; }
    if ((*ctx).flags & KEYRING_SEARCH_DO_STATE_CHECK) != 0 {
        let expiry = READ_ONCE(&(*key).expiry);
        if (kflags & (((1 << KEY_FLAG_INVALIDATED) | (1 << KEY_FLAG_REVOKED)) as c_ulong)) != 0 {
            (*ctx).result = ERR_PTR(-EKEYREVOKED as c_long);
            return (*ctx).skipped_ret;
        }
        if expiry != 0 && (*ctx).now >= expiry {
            if ((*ctx).flags & KEYRING_SEARCH_SKIP_EXPIRED) == 0 {
                (*ctx).result = ERR_PTR(-EKEYEXPIRED as c_long);
            }
            return (*ctx).skipped_ret;
        }
    }
    if ((*ctx).match_data.cmp.unwrap())(key, &(*ctx).match_data) == false { return 0; }
    if ((*ctx).flags & KEYRING_SEARCH_NO_CHECK_PERM) == 0
        && key_task_permission(make_key_ref(key, (*ctx).possessed), (*ctx).cred, KEY_NEED_SEARCH) < 0
    {
        (*ctx).result = ERR_PTR(-EACCES as c_long);
        return (*ctx).skipped_ret;
    }
    if ((*ctx).flags & KEYRING_SEARCH_DO_STATE_CHECK) != 0 && state < 0 {
        (*ctx).result = ERR_PTR(state as c_long);
        return (*ctx).skipped_ret;
    }
    (*ctx).result = make_key_ref(key, (*ctx).possessed);
    1
}

unsafe fn search_keyring(keyring: *mut key, ctx: *mut keyring_search_context) -> c_int {
    if (*ctx).match_data.lookup_type == KEYRING_SEARCH_LOOKUP_DIRECT {
        let object = assoc_array_find(&(*keyring).keys, &keyring_assoc_array_ops, &(*ctx).index_key);
        if !object.is_null() { ((*ctx).iterator.unwrap())(object, ctx as *mut c_void) } else { 0 }
    } else {
        assoc_array_iterate(&(*keyring).keys, (*ctx).iterator, ctx as *mut c_void)
    }
}

#[repr(C)]
struct keyring_search_stack {
    keyring: *mut key,
    node: *mut assoc_array_node,
    slot: c_int,
}

unsafe fn search_nested_keyrings(mut keyring: *mut key, ctx: *mut keyring_search_context) -> bool_t {
    let mut stack: [keyring_search_stack; KEYRING_SEARCH_MAX_DEPTH] = core::mem::zeroed();
    let mut sp: c_int = 0;
    let mut node: *mut assoc_array_node;
    let mut ptr: *mut assoc_array_ptr;
    let mut slot: c_int;
    let mut key_: *mut key;
    if !(*ctx).index_key.description.is_null() {
        key_set_index_key(&mut (*ctx).index_key);
    }
    if (*ctx).match_data.lookup_type == KEYRING_SEARCH_LOOKUP_ITERATE
        || keyring_compare_object(keyring as *const c_void, &(*ctx).index_key as *const _ as *const c_void)
    {
        (*ctx).skipped_ret = 2;
        match ((*ctx).iterator.unwrap())(keyring_key_to_ptr(keyring), ctx as *mut c_void) {
            1 => goto_found(keyring, ctx, &mut stack, &mut sp),
            2 => return false,
            _ => {}
        }
    }
    (*ctx).skipped_ret = 0;
    'descend_to_keyring: loop {
        if ((*keyring).flags & (((1 << KEY_FLAG_INVALIDATED) | (1 << KEY_FLAG_REVOKED)) as c_ulong)) != 0 {
            break 'descend_to_keyring;
        }
        if search_keyring(keyring, ctx) != 0 {
            return goto_found(keyring, ctx, &mut stack, &mut sp);
        }
        if ((*ctx).flags & KEYRING_SEARCH_RECURSE) == 0 { break 'descend_to_keyring; }
        ptr = READ_ONCE(&(*keyring).keys.root);
        if ptr.is_null() { break 'descend_to_keyring; }
        if assoc_array_ptr_is_shortcut(ptr) {
            let shortcut = assoc_array_ptr_to_shortcut(ptr);
            if ((*shortcut).index_key[0] & ASSOC_ARRAY_FAN_MASK) != 0 { break 'descend_to_keyring; }
            ptr = READ_ONCE(&(*shortcut).next_node);
            node = assoc_array_ptr_to_node(ptr);
        } else {
            node = assoc_array_ptr_to_node(ptr);
            ptr = (*node).slots[0];
            if assoc_array_ptr_is_meta(ptr) {
                loop {
                    if assoc_array_ptr_is_shortcut(ptr) {
                        let shortcut = assoc_array_ptr_to_shortcut(ptr);
                        ptr = READ_ONCE(&(*shortcut).next_node);
                    }
                    node = assoc_array_ptr_to_node(ptr);
                    break;
                }
            }
        }
        slot = 0;
        loop {
            while slot < ASSOC_ARRAY_FAN_OUT as c_int {
                ptr = READ_ONCE(&(*node).slots[slot as usize]);
                if assoc_array_ptr_is_meta(ptr) {
                    if !(*node).back_pointer.is_null() || assoc_array_ptr_is_shortcut(ptr) {
                        if assoc_array_ptr_is_shortcut(ptr) {
                            let shortcut = assoc_array_ptr_to_shortcut(ptr);
                            ptr = READ_ONCE(&(*shortcut).next_node);
                        }
                        node = assoc_array_ptr_to_node(ptr);
                        slot = 0;
                        continue;
                    }
                }
                if keyring_ptr_is_keyring(ptr) {
                    key_ = keyring_ptr_to_key(ptr as *const c_void);
                    if sp >= KEYRING_SEARCH_MAX_DEPTH as c_int {
                        if ((*ctx).flags & KEYRING_SEARCH_DETECT_TOO_DEEP) != 0 {
                            (*ctx).result = ERR_PTR(-ELOOP as c_long);
                            return false;
                        }
                        break;
                    }
                    if ((*ctx).flags & KEYRING_SEARCH_NO_CHECK_PERM) != 0
                        || key_task_permission(make_key_ref(key_, (*ctx).possessed), (*ctx).cred, KEY_NEED_SEARCH) >= 0
                    {
                        stack[sp as usize].keyring = keyring;
                        stack[sp as usize].node = node;
                        stack[sp as usize].slot = slot;
                        sp += 1;
                        keyring = key_;
                        continue 'descend_to_keyring;
                    }
                }
                slot += 1;
            }
            ptr = READ_ONCE(&(*node).back_pointer);
            slot = (*node).parent_slot;
            if !ptr.is_null() && assoc_array_ptr_is_shortcut(ptr) {
                let shortcut = assoc_array_ptr_to_shortcut(ptr);
                ptr = READ_ONCE(&(*shortcut).back_pointer);
                slot = (*shortcut).parent_slot;
            }
            if ptr.is_null() { break; }
            node = assoc_array_ptr_to_node(ptr);
            slot += 1;
            if !(*node).back_pointer.is_null() { continue; }
            break;
        }
        break;
    }
    while sp > 0 {
        sp -= 1;
        keyring = stack[sp as usize].keyring;
        node = stack[sp as usize].node;
        slot = stack[sp as usize].slot + 1;
        loop {
            while slot < ASSOC_ARRAY_FAN_OUT as c_int {
                ptr = READ_ONCE(&(*node).slots[slot as usize]);
                if keyring_ptr_is_keyring(ptr) {
                    key_ = keyring_ptr_to_key(ptr as *const c_void);
                    stack[sp as usize].keyring = keyring;
                    stack[sp as usize].node = node;
                    stack[sp as usize].slot = slot;
                    sp += 1;
                    keyring = key_;
                    continue 'descend_to_keyring;
                }
                slot += 1;
            }
            break;
        }
    }
    false
}

unsafe fn goto_found(keyring: *mut key, ctx: *mut keyring_search_context, stack: &mut [keyring_search_stack; KEYRING_SEARCH_MAX_DEPTH], sp: &mut c_int) -> bool_t {
    let key = key_ref_to_ptr((*ctx).result);
    key_check(key);
    if ((*ctx).flags & KEYRING_SEARCH_NO_UPDATE_TIME) == 0 {
        (*key).last_used_at = (*ctx).now;
        (*keyring).last_used_at = (*ctx).now;
        while *sp > 0 {
            *sp -= 1;
            (*stack[*sp as usize].keyring).last_used_at = (*ctx).now;
        }
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_search_rcu(keyring_ref: key_ref_t, ctx: *mut keyring_search_context) -> key_ref_t {
    (*ctx).iterator = Some(keyring_search_iterator);
    (*ctx).possessed = is_key_possessed(keyring_ref);
    (*ctx).result = ERR_PTR(-EAGAIN as c_long);
    let keyring = key_ref_to_ptr(keyring_ref);
    key_check(keyring);
    if (*keyring).type_ != &raw mut key_type_keyring { return ERR_PTR(-ENOTDIR as c_long); }
    if ((*ctx).flags & KEYRING_SEARCH_NO_CHECK_PERM) == 0 {
        let err = key_task_permission(keyring_ref, (*ctx).cred, KEY_NEED_SEARCH);
        if err < 0 { return ERR_PTR(err as c_long); }
    }
    (*ctx).now = ktime_get_real_seconds();
    if search_nested_keyrings(keyring, ctx) {
        __key_get(key_ref_to_ptr((*ctx).result));
    }
    (*ctx).result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_search(
    keyring: key_ref_t,
    type_: *mut key_type,
    description: *const c_char,
    recurse: bool_t,
) -> key_ref_t {
    let mut ctx: keyring_search_context = core::mem::zeroed();
    ctx.index_key.type_ = type_;
    ctx.index_key.description = description;
    ctx.index_key.desc_len = strlen(description);
    ctx.cred = current_cred();
    ctx.match_data.cmp = Some(key_default_cmp);
    ctx.match_data.raw_data = description as *const c_void;
    ctx.match_data.lookup_type = KEYRING_SEARCH_LOOKUP_DIRECT;
    ctx.flags = KEYRING_SEARCH_DO_STATE_CHECK;
    if recurse { ctx.flags |= KEYRING_SEARCH_RECURSE; }
    if let Some(match_preparse) = (*type_).match_preparse {
        let ret = match_preparse(&mut ctx.match_data);
        if ret < 0 { return ERR_PTR(ret as c_long); }
    }
    rcu_read_lock();
    let keyref = keyring_search_rcu(keyring, &mut ctx);
    rcu_read_unlock();
    if let Some(match_free) = (*type_).match_free {
        match_free(&mut ctx.match_data);
    }
    keyref
}

unsafe fn keyring_restriction_alloc(check: key_restrict_link_func_t) -> *mut key_restriction {
    let keyres = kzalloc_obj_key_restriction();
    if keyres.is_null() { return ERR_PTR(-ENOMEM as c_long) as *mut key_restriction; }
    (*keyres).check = check;
    keyres
}

unsafe fn keyring_detect_restriction_cycle(dest_keyring: *const key, mut keyres: *mut key_restriction) -> bool_t {
    while !keyres.is_null() && !(*keyres).key.is_null() && (*(*keyres).key).type_ == &raw mut key_type_keyring {
        if (*keyres).key == dest_keyring as *mut key { return true; }
        keyres = (*(*keyres).key).restrict_link;
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_restrict(keyring_ref: key_ref_t, type_: *const c_char, restriction: *const c_char) -> c_int {
    let keyring = key_ref_to_ptr(keyring_ref);
    key_check(keyring);
    if (*keyring).type_ != &raw mut key_type_keyring { return -ENOTDIR; }
    let mut restrict_type: *mut key_type = null_mut();
    let restrict_link: *mut key_restriction;
    let mut ret = 0;
    if type_.is_null() {
        restrict_link = keyring_restriction_alloc(Some(restrict_link_reject));
    } else {
        restrict_type = key_type_lookup(type_);
        if IS_ERR(restrict_type) { return PTR_ERR(restrict_type) as c_int; }
        if (*restrict_type).lookup_restriction.is_none() {
            ret = -ENOENT;
            if !restrict_type.is_null() { key_type_put(restrict_type); }
            return ret;
        }
        restrict_link = ((*restrict_type).lookup_restriction.unwrap())(restriction);
    }
    if IS_ERR(restrict_link) {
        ret = PTR_ERR(restrict_link) as c_int;
    } else {
        down_write(&mut (*keyring).sem);
        down_write(&raw mut keyring_serialise_restrict_sem);
        if !(*keyring).restrict_link.is_null() {
            ret = -EEXIST;
        } else if keyring_detect_restriction_cycle(keyring, restrict_link) {
            ret = -EDEADLK;
        } else {
            (*keyring).restrict_link = restrict_link;
            notify_key(keyring, NOTIFY_KEY_SETATTR, 0);
        }
        up_write(&raw mut keyring_serialise_restrict_sem);
        up_write(&mut (*keyring).sem);
        if ret < 0 {
            key_put((*restrict_link).key);
            kfree(restrict_link as *mut c_void);
        }
    }
    if !restrict_type.is_null() { key_type_put(restrict_type); }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_key_to_update(keyring_ref: key_ref_t, index_key: *const keyring_index_key) -> key_ref_t {
    let keyring = key_ref_to_ptr(keyring_ref);
    rcu_read_lock();
    let object = assoc_array_find(&(*keyring).keys, &keyring_assoc_array_ops, index_key);
    rcu_read_unlock();
    if object.is_null() { return null_mut(); }
    let key = keyring_ptr_to_key(object);
    if ((*key).flags & (((1 << KEY_FLAG_INVALIDATED) | (1 << KEY_FLAG_REVOKED)) as c_ulong)) != 0 {
        return null_mut();
    }
    __key_get(key);
    make_key_ref(key, is_key_possessed(keyring_ref))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_keyring_by_name(name: *const c_char, uid_keyring: bool_t) -> *mut key {
    let ns = current_user_ns();
    if name.is_null() { return ERR_PTR(-EINVAL as c_long) as *mut key; }
    read_lock(&raw mut keyring_name_lock);
    let mut pos = (*ns).keyring_name_list.next;
    while pos != &mut (*ns).keyring_name_list && !pos.is_null() {
        let keyring = (pos as *mut u8).sub(core::mem::offset_of!(key, name_link)) as *mut key;
        if kuid_has_mapping(ns, (*(*keyring).user).uid)
            && !test_bit(KEY_FLAG_REVOKED, &(*keyring).flags)
            && strcmp((*keyring).description, name) == 0
            && ((uid_keyring && test_bit(KEY_FLAG_UID_KEYRING, &(*keyring).flags))
                || (!uid_keyring && key_permission(make_key_ref(keyring, false), KEY_NEED_SEARCH) >= 0))
            && refcount_inc_not_zero(&mut (*keyring).usage)
        {
            (*keyring).last_used_at = ktime_get_real_seconds();
            read_unlock(&raw mut keyring_name_lock);
            return keyring;
        }
        pos = (*pos).next;
    }
    read_unlock(&raw mut keyring_name_lock);
    ERR_PTR(-ENOKEY as c_long) as *mut key
}

unsafe extern "C" fn keyring_detect_cycle_iterator(object: *const c_void, iterator_data: *mut c_void) -> c_int {
    let ctx = iterator_data as *mut keyring_search_context;
    let key = keyring_ptr_to_key(object);
    if key != (*ctx).match_data.raw_data as *mut key { return 0; }
    (*ctx).result = ERR_PTR(-EDEADLK as c_long);
    1
}

unsafe fn keyring_detect_cycle(A: *mut key, B: *mut key) -> c_int {
    let mut ctx: keyring_search_context = core::mem::zeroed();
    ctx.index_key = core::ptr::read(&(*A).index_key);
    ctx.match_data.raw_data = A as *const c_void;
    ctx.match_data.lookup_type = KEYRING_SEARCH_LOOKUP_DIRECT;
    ctx.iterator = Some(keyring_detect_cycle_iterator);
    ctx.flags = KEYRING_SEARCH_NO_STATE_CHECK
        | KEYRING_SEARCH_NO_UPDATE_TIME
        | KEYRING_SEARCH_NO_CHECK_PERM
        | KEYRING_SEARCH_DETECT_TOO_DEEP
        | KEYRING_SEARCH_RECURSE;
    ctx.result = ERR_PTR(-EAGAIN as c_long);
    rcu_read_lock();
    search_nested_keyrings(B, &mut ctx);
    rcu_read_unlock();
    if PTR_ERR(ctx.result) == -EAGAIN as c_long { 0 } else { PTR_ERR(ctx.result) as c_int }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_link_lock(keyring: *mut key, index_key: *const keyring_index_key) -> c_int {
    if (*keyring).type_ != &raw mut key_type_keyring { return -ENOTDIR; }
    down_write(&mut (*keyring).sem);
    if (*index_key).type_ == &raw mut key_type_keyring { mutex_lock(&raw mut keyring_serialise_link_lock); }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_move_lock(l_keyring: *mut key, u_keyring: *mut key, index_key: *const keyring_index_key) -> c_int {
    if (*l_keyring).type_ != &raw mut key_type_keyring || (*u_keyring).type_ != &raw mut key_type_keyring { return -ENOTDIR; }
    if (l_keyring as usize) < (u_keyring as usize) {
        down_write(&mut (*l_keyring).sem);
        down_write_nested(&mut (*u_keyring).sem, 1);
    } else {
        down_write(&mut (*u_keyring).sem);
        down_write_nested(&mut (*l_keyring).sem, 1);
    }
    if (*index_key).type_ == &raw mut key_type_keyring { mutex_lock(&raw mut keyring_serialise_link_lock); }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_link_begin(keyring: *mut key, index_key: *const keyring_index_key, _edit: *mut *mut assoc_array_edit) -> c_int {
    *_edit = null_mut();
    let mut ret = -EKEYREVOKED;
    if test_bit(KEY_FLAG_REVOKED, &(*keyring).flags) { return ret; }
    let edit = assoc_array_insert(&mut (*keyring).keys, &keyring_assoc_array_ops, index_key, null());
    if IS_ERR(edit) { return PTR_ERR(edit) as c_int; }
    if (*edit).dead_leaf.is_null() {
        ret = key_payload_reserve(keyring, (*keyring).datalen + KEYQUOTA_LINK_BYTES);
        if ret < 0 {
            assoc_array_cancel_edit(edit);
            return ret;
        }
    }
    *_edit = edit;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_link_check_live_key(keyring: *mut key, key: *mut key) -> c_int {
    if (*key).type_ == &raw mut key_type_keyring { keyring_detect_cycle(keyring, key) } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_link(keyring: *mut key, key: *mut key, _edit: *mut *mut assoc_array_edit) {
    __key_get(key);
    assoc_array_insert_set_object(*_edit, keyring_key_to_ptr(key));
    assoc_array_apply_edit(*_edit);
    *_edit = null_mut();
    notify_key(keyring, NOTIFY_KEY_LINKED, key_serial(key));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __key_link_end(keyring: *mut key, index_key: *const keyring_index_key, edit: *mut assoc_array_edit) {
    if !edit.is_null() {
        if (*edit).dead_leaf.is_null() {
            key_payload_reserve(keyring, (*keyring).datalen - KEYQUOTA_LINK_BYTES);
        }
        assoc_array_cancel_edit(edit);
    }
    up_write(&mut (*keyring).sem);
    if (*index_key).type_ == &raw mut key_type_keyring { mutex_unlock(&raw mut keyring_serialise_link_lock); }
}

unsafe fn __key_link_check_restriction(keyring: *mut key, key: *mut key) -> c_int {
    if (*keyring).restrict_link.is_null() || (*(*keyring).restrict_link).check.is_none() {
        return 0;
    }
    ((*(*keyring).restrict_link).check.unwrap())(
        keyring,
        (*key).type_,
        &(*key).payload,
        (*(*keyring).restrict_link).key,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_link(keyring: *mut key, key: *mut key) -> c_int {
    key_check(keyring);
    key_check(key);
    let mut edit: *mut assoc_array_edit = null_mut();
    let mut ret = __key_link_lock(keyring, &(*key).index_key);
    if ret < 0 { return ret; }
    ret = __key_link_begin(keyring, &(*key).index_key, &mut edit);
    if ret >= 0 {
        ret = __key_link_check_restriction(keyring, key);
        if ret == 0 { ret = __key_link_check_live_key(keyring, key); }
        if ret == 0 { __key_link(keyring, key, &mut edit); }
    }
    __key_link_end(keyring, &(*key).index_key, edit);
    ret
}

unsafe fn __key_unlink_lock(keyring: *mut key) -> c_int {
    if (*keyring).type_ != &raw mut key_type_keyring { return -ENOTDIR; }
    down_write(&mut (*keyring).sem);
    0
}

unsafe fn __key_unlink_begin(keyring: *mut key, key: *mut key, _edit: *mut *mut assoc_array_edit) -> c_int {
    let edit = assoc_array_delete(&mut (*keyring).keys, &keyring_assoc_array_ops, &(*key).index_key);
    if IS_ERR(edit) { return PTR_ERR(edit) as c_int; }
    if edit.is_null() { return -ENOENT; }
    *_edit = edit;
    0
}

unsafe fn __key_unlink(keyring: *mut key, key: *mut key, _edit: *mut *mut assoc_array_edit) {
    assoc_array_apply_edit(*_edit);
    notify_key(keyring, NOTIFY_KEY_UNLINKED, key_serial(key));
    *_edit = null_mut();
    key_payload_reserve(keyring, (*keyring).datalen - KEYQUOTA_LINK_BYTES);
}

unsafe fn __key_unlink_end(keyring: *mut key, _key: *mut key, edit: *mut assoc_array_edit) {
    if !edit.is_null() { assoc_array_cancel_edit(edit); }
    up_write(&mut (*keyring).sem);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_unlink(keyring: *mut key, key: *mut key) -> c_int {
    key_check(keyring);
    key_check(key);
    let mut edit: *mut assoc_array_edit = null_mut();
    let ret = __key_unlink_lock(keyring);
    if ret < 0 { return ret; }
    let ret = __key_unlink_begin(keyring, key, &mut edit);
    if ret == 0 { __key_unlink(keyring, key, &mut edit); }
    __key_unlink_end(keyring, key, edit);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_move(key: *mut key, from_keyring: *mut key, to_keyring: *mut key, flags: c_uint) -> c_int {
    let mut from_edit: *mut assoc_array_edit = null_mut();
    let mut to_edit: *mut assoc_array_edit = null_mut();
    if from_keyring == to_keyring { return 0; }
    key_check(key);
    key_check(from_keyring);
    key_check(to_keyring);
    let mut ret = __key_move_lock(from_keyring, to_keyring, &(*key).index_key);
    if ret < 0 { return ret; }
    ret = __key_unlink_begin(from_keyring, key, &mut from_edit);
    if ret >= 0 { ret = __key_link_begin(to_keyring, &(*key).index_key, &mut to_edit); }
    if ret >= 0 {
        ret = -EEXIST;
        if !(*to_edit).dead_leaf.is_null() && (flags & KEYCTL_MOVE_EXCL) != 0 {
        } else {
            ret = __key_link_check_restriction(to_keyring, key);
            if ret >= 0 { ret = __key_link_check_live_key(to_keyring, key); }
            if ret >= 0 {
                __key_unlink(from_keyring, key, &mut from_edit);
                __key_link(to_keyring, key, &mut to_edit);
            }
        }
    }
    __key_link_end(to_keyring, &(*key).index_key, to_edit);
    __key_unlink_end(from_keyring, key, from_edit);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_clear(keyring: *mut key) -> c_int {
    if (*keyring).type_ != &raw mut key_type_keyring { return -ENOTDIR; }
    down_write(&mut (*keyring).sem);
    let edit = assoc_array_clear(&mut (*keyring).keys, &keyring_assoc_array_ops);
    let ret;
    if IS_ERR(edit) {
        ret = PTR_ERR(edit) as c_int;
    } else {
        if !edit.is_null() { assoc_array_apply_edit(edit); }
        notify_key(keyring, NOTIFY_KEY_CLEARED, 0);
        key_payload_reserve(keyring, 0);
        ret = 0;
    }
    up_write(&mut (*keyring).sem);
    ret
}

unsafe extern "C" fn keyring_revoke(keyring: *mut key) {
    let edit = assoc_array_clear(&mut (*keyring).keys, &keyring_assoc_array_ops);
    if !IS_ERR(edit) {
        if !edit.is_null() { assoc_array_apply_edit(edit); }
        key_payload_reserve(keyring, 0);
    }
}

unsafe extern "C" fn keyring_gc_select_iterator(object: *mut c_void, iterator_data: *mut c_void) -> bool_t {
    let key = keyring_ptr_to_key(object);
    let limit = iterator_data as *mut time64_t;
    if key_is_dead(key, *limit) { return false; }
    key_get(key);
    true
}

unsafe extern "C" fn keyring_gc_check_iterator(object: *const c_void, iterator_data: *mut c_void) -> c_int {
    let key = keyring_ptr_to_key(object);
    let limit = iterator_data as *mut time64_t;
    key_check(key);
    key_is_dead(key, *limit) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_gc(keyring: *mut key, mut limit: time64_t) {
    if ((*keyring).flags & (((1 << KEY_FLAG_INVALIDATED) | (1 << KEY_FLAG_REVOKED)) as c_ulong)) != 0 {
        return;
    }
    rcu_read_lock();
    let result = assoc_array_iterate(&(*keyring).keys, Some(keyring_gc_check_iterator), &mut limit as *mut _ as *mut c_void);
    rcu_read_unlock();
    if result == true as c_int {
        down_write(&mut (*keyring).sem);
        assoc_array_gc(
            &mut (*keyring).keys,
            &keyring_assoc_array_ops,
            Some(keyring_gc_select_iterator),
            &mut limit as *mut _ as *mut c_void,
        );
        up_write(&mut (*keyring).sem);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyring_restriction_gc(keyring: *mut key, dead_type: *mut key_type) {
    if dead_type.is_null()
        || (*keyring).restrict_link.is_null()
        || (*(*keyring).restrict_link).keytype != dead_type
    {
        return;
    }
    down_write(&mut (*keyring).sem);
    let keyres = (*keyring).restrict_link;
    (*keyres).check = Some(restrict_link_reject);
    key_put((*keyres).key);
    (*keyres).key = null_mut();
    (*keyres).keytype = null_mut();
    up_write(&mut (*keyring).sem);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

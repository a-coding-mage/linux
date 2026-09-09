// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2006-2007 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* External kernel/XFS types, constants, and operations are supplied by the
 * surrounding translation unit. */
use core::ffi::c_void;

#[repr(C)]
pub struct xfs_mru_cache {
    pub store: radix_tree_root,
    pub lists: *mut list_head,
    pub reap_list: list_head,
    pub lock: spinlock_t,
    pub grp_count: u32,
    pub grp_time: u32,
    pub lru_grp: u32,
    pub time_zero: usize,
    pub free_func: xfs_mru_cache_free_func_t,
    pub work: delayed_work,
    pub queued: u32,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct xfs_mru_cache_elem {
    pub list_node: list_head,
    pub key: usize,
}

extern "C" {
    static mut xfs_mru_reap_wq: *mut workqueue_struct;
    static mut jiffies: usize;
    fn _xfs_mru_cache_reap(work: *mut work_struct);
    fn alloc_workqueue(name: *const u8, flags: u32, max_active: u32) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: usize) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn radix_tree_preload(flags: u32) -> i32;
    fn radix_tree_preload_end();
    fn radix_tree_insert(root: *mut radix_tree_root, key: usize, item: *mut c_void) -> i32;
    fn radix_tree_delete(root: *mut radix_tree_root, key: usize) -> *mut xfs_mru_cache_elem;
    fn radix_tree_lookup(root: *mut radix_tree_root, key: usize) -> *mut xfs_mru_cache_elem;
    fn msecs_to_jiffies(ms: u32) -> u32;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn list_empty(head: *const list_head) -> bool;
    fn list_splice_init(list: *mut list_head, prev: *mut list_head);
    fn list_add_tail(node: *mut list_head, head: *mut list_head);
    fn list_del(node: *mut list_head);
    fn list_del_init(node: *mut list_head);
    fn list_move(node: *mut list_head, head: *mut list_head);
    fn init_list_head(head: *mut list_head);
    fn init_radix_tree(root: *mut radix_tree_root, flags: u32);
    fn init_delayed_work(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
}

#[repr(C)] pub struct radix_tree_root { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
pub type xfs_mru_cache_free_func_t = unsafe extern "C" fn(*mut c_void, *mut xfs_mru_cache_elem);

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const GFP_KERNEL: u32 = 0;
const GFP_ATOMIC: u32 = 0;
const __GFP_NOFAIL: u32 = 0;
const WQ_MEM_RECLAIM: u32 = 0;
const WQ_FREEZABLE: u32 = 0;
const WQ_PERCPU: u32 = 0;

unsafe fn _xfs_mru_cache_migrate(mru: *mut xfs_mru_cache, now: usize) -> usize {
    if (*mru).time_zero == 0 { return 0; }
    let mut migrated = 0u32;
    while (*mru).time_zero <= now.wrapping_sub((*mru).grp_count as usize * (*mru).grp_time as usize) {
        let lru_list = (*mru).lists.add((*mru).lru_grp as usize);
        if !list_empty(lru_list) { list_splice_init(lru_list, (*mru).reap_list.prev); }
        (*mru).lru_grp = ((*mru).lru_grp + 1) % (*mru).grp_count;
        (*mru).time_zero += (*mru).grp_time as usize;
        migrated += 1;
        if migrated == (*mru).grp_count { (*mru).lru_grp = 0; (*mru).time_zero = 0; return 0; }
    }
    for grp in 0..(*mru).grp_count {
        let lru_list = (*mru).lists.add(((*mru).lru_grp + grp) as usize % (*mru).grp_count as usize);
        if !list_empty(lru_list) { return (*mru).time_zero + ((*mru).grp_count + grp) as usize * (*mru).grp_time as usize; }
    }
    (*mru).lru_grp = 0; (*mru).time_zero = 0; 0
}

unsafe fn _xfs_mru_cache_list_insert(mru: *mut xfs_mru_cache, elem: *mut xfs_mru_cache_elem) {
    let mut grp = 0u32;
    let now = jiffies;
    if _xfs_mru_cache_migrate(mru, now) == 0 {
        (*mru).time_zero = now;
        if (*mru).queued == 0 { (*mru).queued = 1; queue_delayed_work(xfs_mru_reap_wq, &mut (*mru).work, (*mru).grp_count as usize * (*mru).grp_time as usize); }
    } else {
        grp = ((now - (*mru).time_zero) / (*mru).grp_time as usize) as u32;
        grp = ((*mru).lru_grp + grp) % (*mru).grp_count;
    }
    list_add_tail(&mut (*elem).list_node, (*mru).lists.add(grp as usize));
}

unsafe fn _xfs_mru_cache_clear_reap_list(mru: *mut xfs_mru_cache) {
    /* The list traversal and temporary list are represented by the external kernel list API. */
    let mut elem = (*mru).reap_list.next as *mut xfs_mru_cache_elem;
    while !elem.is_null() && elem as *mut list_head != &mut (*mru).reap_list {
        let next = (*elem).list_node.next as *mut xfs_mru_cache_elem;
        radix_tree_delete(&mut (*mru).store, (*elem).key);
        list_del_init(&mut (*elem).list_node);
        spin_unlock(&mut (*mru).lock);
        ((*mru).free_func)((*mru).data, elem);
        spin_lock(&mut (*mru).lock);
        elem = next;
    }
}

pub unsafe extern "C" fn _xfs_mru_cache_reap(work: *mut work_struct) {
    let mru = (work as *mut u8).sub(0) as *mut xfs_mru_cache;
    if mru.is_null() || (*mru).lists.is_null() { return; }
    spin_lock(&mut (*mru).lock);
    let mut next = _xfs_mru_cache_migrate(mru, jiffies);
    _xfs_mru_cache_clear_reap_list(mru);
    (*mru).queued = next as u32;
    if (*mru).queued > 0 {
        let now = jiffies;
        next = if next <= now { 0 } else { next - now };
        queue_delayed_work(xfs_mru_reap_wq, &mut (*mru).work, next);
    }
    spin_unlock(&mut (*mru).lock);
}

pub unsafe extern "C" fn xfs_mru_cache_init() -> i32 {
    xfs_mru_reap_wq = alloc_workqueue(b"xfs_mru_cache\0".as_ptr(), WQ_MEM_RECLAIM | WQ_FREEZABLE | WQ_PERCPU, 1);
    if xfs_mru_reap_wq.is_null() { -ENOMEM } else { 0 }
}
pub unsafe extern "C" fn xfs_mru_cache_uninit() { destroy_workqueue(xfs_mru_reap_wq); }

pub unsafe extern "C" fn xfs_mru_cache_create(mrup: *mut *mut xfs_mru_cache, data: *mut c_void, lifetime_ms: u32, grp_count: u32, free_func: xfs_mru_cache_free_func_t) -> i32 {
    if !mrup.is_null() { *mrup = core::ptr::null_mut(); }
    if mrup.is_null() || grp_count == 0 || lifetime_ms == 0 { return -EINVAL; }
    let grp_time = msecs_to_jiffies(lifetime_ms) / grp_count;
    if grp_time == 0 { return -EINVAL; }
    let mru = kzalloc(core::mem::size_of::<xfs_mru_cache>(), GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_mru_cache;
    if mru.is_null() { return -ENOMEM; }
    (*mru).grp_count = grp_count + 1;
    (*mru).lists = kzalloc((*mru).grp_count as usize * core::mem::size_of::<list_head>(), GFP_KERNEL | __GFP_NOFAIL) as *mut list_head;
    if (*mru).lists.is_null() { kfree(mru as *mut c_void); return -ENOMEM; }
    for grp in 0..(*mru).grp_count { init_list_head((*mru).lists.add(grp as usize)); }
    init_radix_tree(&mut (*mru).store, GFP_ATOMIC); init_list_head(&mut (*mru).reap_list); spin_lock_init(&mut (*mru).lock); init_delayed_work(&mut (*mru).work, _xfs_mru_cache_reap);
    (*mru).grp_time = grp_time; (*mru).free_func = free_func; (*mru).data = data; *mrup = mru; 0
}

unsafe fn xfs_mru_cache_flush(mru: *mut xfs_mru_cache) {
    if mru.is_null() || (*mru).lists.is_null() { return; }
    spin_lock(&mut (*mru).lock);
    if (*mru).queued != 0 { spin_unlock(&mut (*mru).lock); cancel_delayed_work_sync(&mut (*mru).work); spin_lock(&mut (*mru).lock); }
    _xfs_mru_cache_migrate(mru, jiffies + (*mru).grp_count as usize * (*mru).grp_time as usize); _xfs_mru_cache_clear_reap_list(mru); spin_unlock(&mut (*mru).lock);
}
pub unsafe extern "C" fn xfs_mru_cache_destroy(mru: *mut xfs_mru_cache) { if mru.is_null() || (*mru).lists.is_null() { return; } xfs_mru_cache_flush(mru); kfree((*mru).lists as *mut c_void); kfree(mru as *mut c_void); }

pub unsafe extern "C" fn xfs_mru_cache_insert(mru: *mut xfs_mru_cache, key: usize, elem: *mut xfs_mru_cache_elem) -> i32 {
    let mut error = -ENOMEM;
    if radix_tree_preload(GFP_KERNEL) != 0 { ((*mru).free_func)((*mru).data, elem); return error; }
    init_list_head(&mut (*elem).list_node); (*elem).key = key; spin_lock(&mut (*mru).lock); error = radix_tree_insert(&mut (*mru).store, key, elem as *mut c_void); radix_tree_preload_end(); if error == 0 { _xfs_mru_cache_list_insert(mru, elem); } spin_unlock(&mut (*mru).lock);
    if error != 0 { ((*mru).free_func)((*mru).data, elem); } error
}
pub unsafe extern "C" fn xfs_mru_cache_remove(mru: *mut xfs_mru_cache, key: usize) -> *mut xfs_mru_cache_elem { if mru.is_null() || (*mru).lists.is_null() { return core::ptr::null_mut(); } spin_lock(&mut (*mru).lock); let elem = radix_tree_delete(&mut (*mru).store, key); if !elem.is_null() { list_del(&mut (*elem).list_node); } spin_unlock(&mut (*mru).lock); elem }
pub unsafe extern "C" fn xfs_mru_cache_delete(mru: *mut xfs_mru_cache, key: usize) { let elem = xfs_mru_cache_remove(mru, key); if !elem.is_null() { ((*mru).free_func)((*mru).data, elem); } }
pub unsafe extern "C" fn xfs_mru_cache_lookup(mru: *mut xfs_mru_cache, key: usize) -> *mut xfs_mru_cache_elem { if mru.is_null() || (*mru).lists.is_null() { return core::ptr::null_mut(); } spin_lock(&mut (*mru).lock); let elem = radix_tree_lookup(&mut (*mru).store, key); if elem.is_null() { spin_unlock(&mut (*mru).lock); } else { list_del(&mut (*elem).list_node); _xfs_mru_cache_list_insert(mru, elem); } elem }
pub unsafe extern "C" fn xfs_mru_cache_done(mru: *mut xfs_mru_cache) { spin_unlock(&mut (*mru).lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

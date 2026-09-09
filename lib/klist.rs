// SPDX-License-Identifier: GPL-2.0-only
/*
 * klist.c - Routines for manipulating klists.
 *
 * Copyright (C) 2005 Patrick Mochel
 *
 * This klist interface provides a couple of structures that wrap around
 * struct list_head to provide explicit list "head" (struct klist) and list
 * "node" (struct klist_node) objects. For struct klist, a spinlock is
 * included that protects access to the actual list itself. struct
 * klist_node provides a pointer to the klist that owns it and a kref
 * reference count that indicates the number of current users of that node
 * in the list.
 *
 * The entire point is to provide an interface for iterating over a list
 * that is safe and allows for modification of the list during the
 * iteration (e.g. insertion and removal), including modification of the
 * current node on the list.
 *
 * It works using a 3rd object type - struct klist_iter - that is declared
 * and initialized before an iteration. klist_next() is used to acquire the
 * next element in the list. It returns NULL if there are no more items.
 * Internally, that routine takes the klist's lock, decrements the
 * reference count of the previous klist_node and increments the count of
 * the next klist_node. It then drops the lock and returns.
 *
 * There are primitives for adding and removing nodes to/from a klist.
 * When deleting, klist_del() will simply decrement the reference count.
 * Only when the count goes to 0 is the node removed from the list.
 * klist_remove() will try to delete the node from the list and block until
 * it is actually removed. This is useful for objects (like devices) that
 * have been removed from the system and must be freed (but must wait until
 * all accessors have finished).
 */

// External kernel declarations supplied by the surrounding implementation.
use core::ffi::c_void;

const KNODE_DEAD: usize = 1;
const KNODE_KLIST_MASK: usize = !KNODE_DEAD;

unsafe fn knode_klist(knode: *mut klist_node) -> *mut klist {
    ((*knode).n_klist as usize & KNODE_KLIST_MASK) as *mut klist
}

unsafe fn knode_dead(knode: *mut klist_node) -> bool {
    (*knode).n_klist as usize & KNODE_DEAD != 0
}

unsafe fn knode_set_klist(knode: *mut klist_node, klist: *mut klist) {
    (*knode).n_klist = klist;
    // no knode deserves to start its life dead
    WARN_ON(knode_dead(knode));
}

unsafe fn knode_kill(knode: *mut klist_node) {
    // and no knode should die twice ever either, see we're very humane
    WARN_ON(knode_dead(knode));
    *(core::ptr::addr_of_mut!((*knode).n_klist) as *mut usize) |= KNODE_DEAD;
}

pub unsafe extern "C" fn klist_init(
    k: *mut klist,
    get: Option<unsafe extern "C" fn(*mut klist_node)>,
    put: Option<unsafe extern "C" fn(*mut klist_node)>,
) {
    INIT_LIST_HEAD(&mut (*k).k_list);
    spin_lock_init(&mut (*k).k_lock);
    (*k).get = get;
    (*k).put = put;
}

unsafe fn add_head(k: *mut klist, n: *mut klist_node) {
    spin_lock(&mut (*k).k_lock);
    list_add(&mut (*n).n_node, &mut (*k).k_list);
    spin_unlock(&mut (*k).k_lock);
}

unsafe fn add_tail(k: *mut klist, n: *mut klist_node) {
    spin_lock(&mut (*k).k_lock);
    list_add_tail(&mut (*n).n_node, &mut (*k).k_list);
    spin_unlock(&mut (*k).k_lock);
}

unsafe fn klist_node_init(k: *mut klist, n: *mut klist_node) {
    INIT_LIST_HEAD(&mut (*n).n_node);
    kref_init(&mut (*n).n_ref);
    knode_set_klist(n, k);
    if let Some(get) = (*k).get {
        get(n);
    }
}

pub unsafe extern "C" fn klist_add_head(n: *mut klist_node, k: *mut klist) {
    klist_node_init(k, n);
    add_head(k, n);
}

pub unsafe extern "C" fn klist_add_tail(n: *mut klist_node, k: *mut klist) {
    klist_node_init(k, n);
    add_tail(k, n);
}

pub unsafe extern "C" fn klist_add_behind(n: *mut klist_node, pos: *mut klist_node) {
    let k = knode_klist(pos);
    klist_node_init(k, n);
    spin_lock(&mut (*k).k_lock);
    list_add(&mut (*n).n_node, &mut (*pos).n_node);
    spin_unlock(&mut (*k).k_lock);
}

pub unsafe extern "C" fn klist_add_before(n: *mut klist_node, pos: *mut klist_node) {
    let k = knode_klist(pos);
    klist_node_init(k, n);
    spin_lock(&mut (*k).k_lock);
    list_add_tail(&mut (*n).n_node, &mut (*pos).n_node);
    spin_unlock(&mut (*k).k_lock);
}

#[repr(C)]
struct klist_waiter {
    list: list_head,
    node: *mut klist_node,
    process: *mut task_struct,
    woken: i32,
}

static mut klist_remove_lock: spinlock_t = spinlock_t::ZERO;
static mut klist_remove_waiters: list_head = list_head::ZERO;

unsafe fn klist_release(kref: *mut kref) {
    let mut waiter: *mut klist_waiter;
    let mut tmp: *mut klist_waiter;
    let n = container_of(kref, core::mem::offset_of!(klist_node, n_ref), klist_node);

    WARN_ON(!knode_dead(n));
    list_del(&mut (*n).n_node);
    spin_lock(&mut klist_remove_lock);
    list_for_each_entry_safe!(waiter, tmp, &mut klist_remove_waiters, list, klist_waiter, {
        if (*waiter).node == n {
            list_del(&mut (*waiter).list);
            (*waiter).woken = 1;
            mb();
            wake_up_process((*waiter).process);
        }
    });
    spin_unlock(&mut klist_remove_lock);
    knode_set_klist(n, core::ptr::null_mut());
}

unsafe fn klist_dec_and_del(n: *mut klist_node) -> i32 {
    kref_put(&mut (*n).n_ref, Some(klist_release))
}

unsafe fn klist_put(n: *mut klist_node, kill: bool) {
    let k = knode_klist(n);
    let mut put = (*k).put;
    spin_lock(&mut (*k).k_lock);
    if kill { knode_kill(n); }
    if klist_dec_and_del(n) == 0 { put = None; }
    spin_unlock(&mut (*k).k_lock);
    if let Some(put) = put { put(n); }
}

pub unsafe extern "C" fn klist_del(n: *mut klist_node) { klist_put(n, true); }

pub unsafe extern "C" fn klist_remove(n: *mut klist_node) {
    let mut waiter = klist_waiter {
        list: list_head::ZERO, node: n, process: current(), woken: 0,
    };
    spin_lock(&mut klist_remove_lock);
    list_add(&mut waiter.list, &mut klist_remove_waiters);
    spin_unlock(&mut klist_remove_lock);
    klist_del(n);
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if waiter.woken != 0 { break; }
        schedule();
    }
    __set_current_state(TASK_RUNNING);
}

pub unsafe extern "C" fn klist_node_attached(n: *mut klist_node) -> i32 {
    ((*n).n_klist != core::ptr::null_mut()) as i32
}

pub unsafe extern "C" fn klist_iter_init_node(k: *mut klist, i: *mut klist_iter, n: *mut klist_node) {
    (*i).i_klist = k;
    (*i).i_cur = core::ptr::null_mut();
    if !n.is_null() && kref_get_unless_zero(&mut (*n).n_ref) { (*i).i_cur = n; }
}

pub unsafe extern "C" fn klist_iter_init(k: *mut klist, i: *mut klist_iter) {
    klist_iter_init_node(k, i, core::ptr::null_mut());
}

pub unsafe extern "C" fn klist_iter_exit(i: *mut klist_iter) {
    if !(*i).i_cur.is_null() {
        klist_put((*i).i_cur, false);
        (*i).i_cur = core::ptr::null_mut();
    }
}

unsafe fn to_klist_node(n: *mut list_head) -> *mut klist_node {
    container_of(n, core::mem::offset_of!(klist_node, n_node), klist_node)
}

pub unsafe extern "C" fn klist_prev(i: *mut klist_iter) -> *mut klist_node {
    let mut put = (*(*i).i_klist).put;
    let last = (*i).i_cur;
    let mut prev;
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*(*i).i_klist).k_lock, &mut flags);
    if !last.is_null() {
        prev = to_klist_node((*last).n_node.prev);
        if klist_dec_and_del(last) == 0 { put = None; }
    } else { prev = to_klist_node((*(*i).i_klist).k_list.prev); }
    (*i).i_cur = core::ptr::null_mut();
    while prev != to_klist_node(&mut (*(*i).i_klist).k_list) {
        if !knode_dead(prev) { kref_get(&mut (*prev).n_ref); (*i).i_cur = prev; break; }
        prev = to_klist_node((*prev).n_node.prev);
    }
    spin_unlock_irqrestore(&mut (*(*i).i_klist).k_lock, flags);
    if let Some(put) = put { if !last.is_null() { put(last); } }
    (*i).i_cur
}

pub unsafe extern "C" fn klist_next(i: *mut klist_iter) -> *mut klist_node {
    let mut put = (*(*i).i_klist).put;
    let last = (*i).i_cur;
    let mut next;
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*(*i).i_klist).k_lock, &mut flags);
    if !last.is_null() {
        next = to_klist_node((*last).n_node.next);
        if klist_dec_and_del(last) == 0 { put = None; }
    } else { next = to_klist_node((*(*i).i_klist).k_list.next); }
    (*i).i_cur = core::ptr::null_mut();
    while next != to_klist_node(&mut (*(*i).i_klist).k_list) {
        if !knode_dead(next) { kref_get(&mut (*next).n_ref); (*i).i_cur = next; break; }
        next = to_klist_node((*next).n_node.next);
    }
    spin_unlock_irqrestore(&mut (*(*i).i_klist).k_lock, flags);
    if let Some(put) = put { if !last.is_null() { put(last); } }
    (*i).i_cur
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

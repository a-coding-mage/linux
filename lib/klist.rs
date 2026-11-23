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

// Dependencies: linux/klist.h, linux/export.h, linux/sched.h.

use core::mem::offset_of;
use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile, write_volatile};
use kernel::bindings::{get_current, klist, klist_iter, klist_node, kref, list_head, mb, schedule,
                       task_struct, wake_up_process};
use kernel::ffi::{c_int, c_ulong, c_void};

/// Callback type of `struct klist`'s `get` and `put` members.
type KlistNodeFn = Option<unsafe extern "C" fn(*mut klist_node)>;

extern "C" {
    fn lupos_klist_lock_init(k: *mut klist);
    fn lupos_klist_lock(k: *mut klist);
    fn lupos_klist_unlock(k: *mut klist);
    fn lupos_klist_lock_irqsave(k: *mut klist) -> c_ulong;
    fn lupos_klist_unlock_irqrestore(k: *mut klist, flags: c_ulong);
    fn lupos_klist_remove_lock();
    fn lupos_klist_remove_unlock();
    fn lupos_klist_init_list_head(list: *mut list_head);
    fn lupos_klist_list_add(new: *mut list_head, head: *mut list_head);
    fn lupos_klist_list_add_tail(new: *mut list_head, head: *mut list_head);
    fn lupos_klist_list_del(entry: *mut list_head);
    fn lupos_klist_kref_init(kref: *mut kref);
    fn lupos_klist_kref_get(kref: *mut kref);
    fn lupos_klist_kref_get_unless_zero(kref: *mut kref) -> c_int;
    fn lupos_klist_kref_dec_and_test(kref: *mut kref) -> bool;
    fn lupos_klist_set_current_uninterruptible();
    fn lupos_klist_set_current_running();
    fn lupos_klist_warn_set_dead();
    fn lupos_klist_warn_kill_dead();
    fn lupos_klist_warn_release_live();
}

/*
 * Use the lowest bit of n_klist to mark deleted nodes and exclude
 * dead ones from iteration.
 */
const KNODE_DEAD: c_ulong = 1;
const KNODE_KLIST_MASK: c_ulong = !KNODE_DEAD;

/// # Safety
/// `knode` points to a valid `struct klist_node`.
unsafe fn knode_klist(knode: *mut klist_node) -> *mut klist {
    // SAFETY: knode is valid; the tagged pointer is only reinterpreted.
    unsafe { ((*knode).n_klist as c_ulong & KNODE_KLIST_MASK) as *mut klist }
}

/// # Safety
/// `knode` points to a valid `struct klist_node`.
unsafe fn knode_dead(knode: *mut klist_node) -> bool {
    // SAFETY: knode is valid.
    unsafe { (*knode).n_klist as c_ulong & KNODE_DEAD != 0 }
}

/// # Safety
/// `knode` points to a valid `struct klist_node`.
unsafe fn knode_set_klist(knode: *mut klist_node, klist: *mut klist) {
    // SAFETY: knode is valid.
    unsafe {
        (*knode).n_klist = klist.cast::<c_void>();
        /* no knode deserves to start its life dead */
        if knode_dead(knode) {
            lupos_klist_warn_set_dead();
        }
    }
}

/// # Safety
/// `knode` points to a valid `struct klist_node`.
unsafe fn knode_kill(knode: *mut klist_node) {
    // SAFETY: knode is valid; C updates the pointer through unsigned long.
    unsafe {
        /* and no knode should die twice ever either, see we're very humane */
        if knode_dead(knode) {
            lupos_klist_warn_kill_dead();
        }
        *addr_of_mut!((*knode).n_klist).cast::<c_ulong>() |= KNODE_DEAD;
    }
}

/**
 * klist_init - Initialize a klist structure.
 * @k: The klist we're initializing.
 * @get: The get function for the embedding object (NULL if none)
 * @put: The put function for the embedding object (NULL if none)
 *
 * Initialises the klist structure.  If the klist_node structures are
 * going to be embedded in refcounted objects (necessary for safe
 * deletion) then the get/put arguments are used to initialise
 * functions that take and release references on the embedding
 * objects.
 */
/// # Safety
/// `k` points to writable storage for a `struct klist`.
#[no_mangle]
pub unsafe extern "C" fn klist_init(k: *mut klist, get: KlistNodeFn, put: KlistNodeFn) {
    // SAFETY: k is writable klist storage owned by the caller.
    unsafe {
        lupos_klist_init_list_head(addr_of_mut!((*k).k_list));
        lupos_klist_lock_init(k);
        (*k).get = get;
        (*k).put = put;
    }
}
// EXPORT_SYMBOL_GPL(klist_init);

/// # Safety
/// `k` is an initialized klist and `n` a node being added to it.
unsafe fn add_head(k: *mut klist, n: *mut klist_node) {
    // SAFETY: the list is modified only under k->k_lock.
    unsafe {
        lupos_klist_lock(k);
        lupos_klist_list_add(addr_of_mut!((*n).n_node), addr_of_mut!((*k).k_list));
        lupos_klist_unlock(k);
    }
}

/// # Safety
/// `k` is an initialized klist and `n` a node being added to it.
unsafe fn add_tail(k: *mut klist, n: *mut klist_node) {
    // SAFETY: the list is modified only under k->k_lock.
    unsafe {
        lupos_klist_lock(k);
        lupos_klist_list_add_tail(addr_of_mut!((*n).n_node), addr_of_mut!((*k).k_list));
        lupos_klist_unlock(k);
    }
}

/// # Safety
/// `k` is an initialized klist and `n` points to writable node storage.
unsafe fn klist_node_init(k: *mut klist, n: *mut klist_node) {
    // SAFETY: n is the caller's new node; k->get is the klist's callback.
    unsafe {
        lupos_klist_init_list_head(addr_of_mut!((*n).n_node));
        lupos_klist_kref_init(addr_of_mut!((*n).n_ref));
        knode_set_klist(n, k);
        if let Some(get) = (*k).get {
            get(n);
        }
    }
}

/**
 * klist_add_head - Initialize a klist_node and add it to front.
 * @n: node we're adding.
 * @k: klist it's going on.
 */
/// # Safety
/// `k` is an initialized klist and `n` points to writable node storage.
#[no_mangle]
pub unsafe extern "C" fn klist_add_head(n: *mut klist_node, k: *mut klist) {
    // SAFETY: forwarded caller contract.
    unsafe {
        klist_node_init(k, n);
        add_head(k, n);
    }
}
// EXPORT_SYMBOL_GPL(klist_add_head);

/**
 * klist_add_tail - Initialize a klist_node and add it to back.
 * @n: node we're adding.
 * @k: klist it's going on.
 */
/// # Safety
/// `k` is an initialized klist and `n` points to writable node storage.
#[no_mangle]
pub unsafe extern "C" fn klist_add_tail(n: *mut klist_node, k: *mut klist) {
    // SAFETY: forwarded caller contract.
    unsafe {
        klist_node_init(k, n);
        add_tail(k, n);
    }
}
// EXPORT_SYMBOL_GPL(klist_add_tail);

/**
 * klist_add_behind - Init a klist_node and add it after an existing node
 * @n: node we're adding.
 * @pos: node to put @n after
 */
/// # Safety
/// `pos` is a node on an initialized klist and `n` writable node storage.
#[no_mangle]
pub unsafe extern "C" fn klist_add_behind(n: *mut klist_node, pos: *mut klist_node) {
    // SAFETY: pos belongs to a live klist whose lock protects the insertion.
    unsafe {
        let k = knode_klist(pos);

        klist_node_init(k, n);
        lupos_klist_lock(k);
        lupos_klist_list_add(addr_of_mut!((*n).n_node), addr_of_mut!((*pos).n_node));
        lupos_klist_unlock(k);
    }
}
// EXPORT_SYMBOL_GPL(klist_add_behind);

/**
 * klist_add_before - Init a klist_node and add it before an existing node
 * @n: node we're adding.
 * @pos: node to put @n after
 */
/// # Safety
/// `pos` is a node on an initialized klist and `n` writable node storage.
#[no_mangle]
pub unsafe extern "C" fn klist_add_before(n: *mut klist_node, pos: *mut klist_node) {
    // SAFETY: pos belongs to a live klist whose lock protects the insertion.
    unsafe {
        let k = knode_klist(pos);

        klist_node_init(k, n);
        lupos_klist_lock(k);
        lupos_klist_list_add_tail(addr_of_mut!((*n).n_node), addr_of_mut!((*pos).n_node));
        lupos_klist_unlock(k);
    }
}
// EXPORT_SYMBOL_GPL(klist_add_before);

/// `struct klist_waiter`, private to the original translation unit.
#[repr(C)]
struct KlistWaiter {
    list: list_head,
    node: *mut klist_node,
    process: *mut task_struct,
    woken: c_int,
}

// static DEFINE_SPINLOCK(klist_remove_lock) keeps its static lockdep
// initializer in lib/klist_helpers.c.
static mut KLIST_REMOVE_WAITERS: list_head = list_head {
    next: addr_of_mut!(KLIST_REMOVE_WAITERS),
    prev: addr_of_mut!(KLIST_REMOVE_WAITERS),
};

/// The kref release callback of klist nodes.
///
/// # Safety
/// `kref` is the `n_ref` member of a dead node whose klist lock is held.
unsafe fn klist_release(kref: *mut kref) {
    // SAFETY: kref is embedded in a klist_node; the waiter list is
    // traversed and modified under klist_remove_lock.
    unsafe {
        let n = kref.byte_sub(offset_of!(klist_node, n_ref)).cast::<klist_node>();

        if !knode_dead(n) {
            lupos_klist_warn_release_live();
        }
        lupos_klist_list_del(addr_of_mut!((*n).n_node));
        lupos_klist_remove_lock();
        let head = addr_of_mut!(KLIST_REMOVE_WAITERS);
        let mut pos = (*head).next;
        while pos != head {
            let tmp = (*pos).next;
            let waiter = pos.byte_sub(offset_of!(KlistWaiter, list)).cast::<KlistWaiter>();

            if (*waiter).node == n {
                lupos_klist_list_del(addr_of_mut!((*waiter).list));
                write_volatile(addr_of_mut!((*waiter).woken), 1);
                mb();
                wake_up_process((*waiter).process);
            }
            pos = tmp;
        }
        lupos_klist_remove_unlock();
        knode_set_klist(n, null_mut());
    }
}

/// # Safety
/// `n` is a node on a klist whose lock is held.
unsafe fn klist_dec_and_del(n: *mut klist_node) -> c_int {
    // SAFETY: kref_put() expansion on the node's embedded kref.
    unsafe {
        if lupos_klist_kref_dec_and_test(addr_of_mut!((*n).n_ref)) {
            klist_release(addr_of_mut!((*n).n_ref));
            1
        } else {
            0
        }
    }
}

/// # Safety
/// `n` is a node on an initialized klist.
unsafe fn klist_put(n: *mut klist_node, kill: bool) {
    // SAFETY: n is attached to the klist returned by knode_klist().
    unsafe {
        let k = knode_klist(n);
        let mut put: KlistNodeFn = (*k).put;

        lupos_klist_lock(k);
        if kill {
            knode_kill(n);
        }
        if klist_dec_and_del(n) == 0 {
            put = None;
        }
        lupos_klist_unlock(k);
        if let Some(put) = put {
            put(n);
        }
    }
}

/**
 * klist_del - Decrement the reference count of node and try to remove.
 * @n: node we're deleting.
 */
/// # Safety
/// `n` is a node on an initialized klist.
#[no_mangle]
pub unsafe extern "C" fn klist_del(n: *mut klist_node) {
    // SAFETY: forwarded caller contract.
    unsafe { klist_put(n, true) };
}
// EXPORT_SYMBOL_GPL(klist_del);

/**
 * klist_remove - Decrement the refcount of node and wait for it to go away.
 * @n: node we're removing.
 */
/// # Safety
/// `n` is a node on an initialized klist; the caller may sleep.
#[no_mangle]
pub unsafe extern "C" fn klist_remove(n: *mut klist_node) {
    let mut waiter = KlistWaiter {
        list: list_head {
            next: null_mut(),
            prev: null_mut(),
        },
        node: n,
        // SAFETY: get_current() returns the running task.
        process: unsafe { get_current() },
        woken: 0,
    };

    // SAFETY: the waiter stays on this stack until klist_release() has
    // unlinked it and set woken, which this loop waits for.
    unsafe {
        lupos_klist_remove_lock();
        lupos_klist_list_add(addr_of_mut!(waiter.list), addr_of_mut!(KLIST_REMOVE_WAITERS));
        lupos_klist_remove_unlock();

        klist_del(n);

        loop {
            lupos_klist_set_current_uninterruptible();
            if read_volatile(addr_of!(waiter.woken)) != 0 {
                break;
            }
            schedule();
        }
        lupos_klist_set_current_running();
    }
}
// EXPORT_SYMBOL_GPL(klist_remove);

/**
 * klist_node_attached - Say whether a node is bound to a list or not.
 * @n: Node that we're testing.
 */
/// # Safety
/// `n` points to a valid `struct klist_node`.
#[no_mangle]
pub unsafe extern "C" fn klist_node_attached(n: *mut klist_node) -> c_int {
    // SAFETY: n is valid.
    unsafe { c_int::from(!(*n).n_klist.is_null()) }
}
// EXPORT_SYMBOL_GPL(klist_node_attached);

/**
 * klist_iter_init_node - Initialize a klist_iter structure.
 * @k: klist we're iterating.
 * @i: klist_iter we're filling.
 * @n: node to start with.
 *
 * Similar to klist_iter_init(), but starts the action off with @n,
 * instead of with the list head.
 */
/// # Safety
/// `k` is an initialized klist, `i` writable iterator storage and `n` NULL
/// or a node on `k`.
#[no_mangle]
pub unsafe extern "C" fn klist_iter_init_node(k: *mut klist, i: *mut klist_iter, n: *mut klist_node) {
    // SAFETY: forwarded caller contract.
    unsafe {
        (*i).i_klist = k;
        (*i).i_cur = null_mut();
        if !n.is_null() && lupos_klist_kref_get_unless_zero(addr_of_mut!((*n).n_ref)) != 0 {
            (*i).i_cur = n;
        }
    }
}
// EXPORT_SYMBOL_GPL(klist_iter_init_node);

/**
 * klist_iter_init - Iniitalize a klist_iter structure.
 * @k: klist we're iterating.
 * @i: klist_iter structure we're filling.
 *
 * Similar to klist_iter_init_node(), but start with the list head.
 */
/// # Safety
/// `k` is an initialized klist and `i` writable iterator storage.
#[no_mangle]
pub unsafe extern "C" fn klist_iter_init(k: *mut klist, i: *mut klist_iter) {
    // SAFETY: forwarded caller contract.
    unsafe { klist_iter_init_node(k, i, null_mut()) };
}
// EXPORT_SYMBOL_GPL(klist_iter_init);

/**
 * klist_iter_exit - Finish a list iteration.
 * @i: Iterator structure.
 *
 * Must be called when done iterating over list, as it decrements the
 * refcount of the current node. Necessary in case iteration exited before
 * the end of the list was reached, and always good form.
 */
/// # Safety
/// `i` is an iterator initialized by `klist_iter_init*()`.
#[no_mangle]
pub unsafe extern "C" fn klist_iter_exit(i: *mut klist_iter) {
    // SAFETY: i_cur, when set, holds a reference taken by the iterator.
    unsafe {
        if !(*i).i_cur.is_null() {
            klist_put((*i).i_cur, false);
            (*i).i_cur = null_mut();
        }
    }
}
// EXPORT_SYMBOL_GPL(klist_iter_exit);

/// container_of(n, struct klist_node, n_node), including for the list head.
fn to_klist_node(n: *mut list_head) -> *mut klist_node {
    // The list head is not embedded in a node; C forms the pointer anyway.
    n.wrapping_byte_sub(offset_of!(klist_node, n_node)).cast::<klist_node>()
}

/**
 * klist_prev - Ante up prev node in list.
 * @i: Iterator structure.
 *
 * First grab list lock. Decrement the reference count of the previous
 * node, if there was one. Grab the prev node, increment its reference
 * count, drop the lock, and return that prev node.
 */
/// # Safety
/// `i` is an iterator initialized by `klist_iter_init*()`.
#[no_mangle]
pub unsafe extern "C" fn klist_prev(i: *mut klist_iter) -> *mut klist_node {
    // SAFETY: the iterator's klist is live; nodes are walked under its lock.
    unsafe {
        let mut put: KlistNodeFn = (*(*i).i_klist).put;
        let last = (*i).i_cur;
        let mut prev: *mut klist_node;

        let flags = lupos_klist_lock_irqsave((*i).i_klist);

        if !last.is_null() {
            prev = to_klist_node((*last).n_node.prev);
            if klist_dec_and_del(last) == 0 {
                put = None;
            }
        } else {
            prev = to_klist_node((*(*i).i_klist).k_list.prev);
        }

        (*i).i_cur = null_mut();
        while prev != to_klist_node(addr_of_mut!((*(*i).i_klist).k_list)) {
            if !knode_dead(prev) {
                lupos_klist_kref_get(addr_of_mut!((*prev).n_ref));
                (*i).i_cur = prev;
                break;
            }
            prev = to_klist_node((*prev).n_node.prev);
        }

        lupos_klist_unlock_irqrestore((*i).i_klist, flags);

        if let Some(put) = put {
            if !last.is_null() {
                put(last);
            }
        }
        (*i).i_cur
    }
}
// EXPORT_SYMBOL_GPL(klist_prev);

/**
 * klist_next - Ante up next node in list.
 * @i: Iterator structure.
 *
 * First grab list lock. Decrement the reference count of the previous
 * node, if there was one. Grab the next node, increment its reference
 * count, drop the lock, and return that next node.
 */
/// # Safety
/// `i` is an iterator initialized by `klist_iter_init*()`.
#[no_mangle]
pub unsafe extern "C" fn klist_next(i: *mut klist_iter) -> *mut klist_node {
    // SAFETY: the iterator's klist is live; nodes are walked under its lock.
    unsafe {
        let mut put: KlistNodeFn = (*(*i).i_klist).put;
        let last = (*i).i_cur;
        let mut next: *mut klist_node;

        let flags = lupos_klist_lock_irqsave((*i).i_klist);

        if !last.is_null() {
            next = to_klist_node((*last).n_node.next);
            if klist_dec_and_del(last) == 0 {
                put = None;
            }
        } else {
            next = to_klist_node((*(*i).i_klist).k_list.next);
        }

        (*i).i_cur = null_mut();
        while next != to_klist_node(addr_of_mut!((*(*i).i_klist).k_list)) {
            if !knode_dead(next) {
                lupos_klist_kref_get(addr_of_mut!((*next).n_ref));
                (*i).i_cur = next;
                break;
            }
            next = to_klist_node((*next).n_node.next);
        }

        lupos_klist_unlock_irqrestore((*i).i_klist, flags);

        if let Some(put) = put {
            if !last.is_null() {
                put(last);
            }
        }
        (*i).i_cur
    }
}
// EXPORT_SYMBOL_GPL(klist_next);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

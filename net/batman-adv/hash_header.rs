/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Simon Wunderlich, Marek Lindner
 */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

pub type batadv_hashdata_compare_cb =
    unsafe extern "C" fn(node: *const hlist_node, key: *const core::ffi::c_void) -> bool;
pub type batadv_hashdata_choose_cb =
    unsafe extern "C" fn(key: *const core::ffi::c_void, size: u32) -> u32;
pub type batadv_hashdata_free_cb =
    unsafe extern "C" fn(node: *mut hlist_node, arg: *mut core::ffi::c_void);

/** Wrapper of simple hlist based hashtable. */
#[repr(C)]
pub struct batadv_hashtable {
    /** the hashtable itself with the buckets */
    pub table: *mut hlist_head,
    /** spinlock for each hash list entry */
    pub list_locks: *mut spinlock_t,
    /** size of hashtable */
    pub size: u32,
    /** current (generation) sequence number */
    pub generation: atomic_t,
}

extern "C" {
    pub fn batadv_hash_new(size: u32) -> *mut batadv_hashtable;
    pub fn batadv_hash_set_lock_class(hash: *mut batadv_hashtable,
                                      key: *mut lock_class_key);
    pub fn batadv_hash_destroy(hash: *mut batadv_hashtable);
}

pub unsafe fn batadv_hash_add(
    hash: *mut batadv_hashtable,
    compare: batadv_hashdata_compare_cb,
    choose: batadv_hashdata_choose_cb,
    data: *const core::ffi::c_void,
    data_node: *mut hlist_node,
) -> i32 {
    let mut ret: i32 = -1;
    let index: u32;

    if hash.is_null() {
        return ret;
    }

    index = choose(data, (*hash).size);
    let head = (*hash).table.add(index as usize);
    let list_lock = (*hash).list_locks.add(index as usize);

    spin_lock_bh(list_lock);

    /* hlist_for_each(node, head) */
    let mut node: *mut hlist_node = (*head).first;
    while !node.is_null() {
        if compare(node, data) {
            ret = 1;
            break;
        }
        node = (*node).next;
    }

    if ret != 1 {
        hlist_add_head_rcu(data_node, head);
        atomic_inc(&mut (*hash).generation);
        ret = 0;
    }

    spin_unlock_bh(list_lock);
    ret
}

pub unsafe fn batadv_hash_remove(
    hash: *mut batadv_hashtable,
    compare: batadv_hashdata_compare_cb,
    choose: batadv_hashdata_choose_cb,
    data: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let index = choose(data, (*hash).size);
    let head = (*hash).table.add(index as usize);
    let mut data_save: *mut core::ffi::c_void = core::ptr::null_mut();

    spin_lock_bh((*hash).list_locks.add(index as usize));
    let mut node: *mut hlist_node = (*head).first;
    while !node.is_null() {
        if compare(node, data) {
            data_save = node as *mut core::ffi::c_void;
            hlist_del_rcu(node);
            atomic_inc(&mut (*hash).generation);
            break;
        }
        node = (*node).next;
    }
    spin_unlock_bh((*hash).list_locks.add(index as usize));

    data_save
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

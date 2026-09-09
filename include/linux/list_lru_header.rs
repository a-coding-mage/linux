/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2013 Red Hat, Inc. and Parallels Inc. All rights reserved.
 * Authors: David Chinner and Glauber Costa
 *
 * Generic LRU infrastructure
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum lru_status {
    LRU_REMOVED,
    LRU_REMOVED_RETRY,
    LRU_ROTATE,
    LRU_SKIP,
    LRU_RETRY,
    LRU_STOP,
}

#[repr(C)]
pub struct mem_cgroup;

#[repr(C)]
pub struct list_lru_one {
    pub list: list_head,
    /* may become negative during memcg reparenting */
    pub nr_items: core::ffi::c_long,
    /* protects all fields above */
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct list_lru_memcg {
    pub rcu: rcu_head,
    /* array of per cgroup per node lists, indexed by node id */
    pub node: [list_lru_one; 0],
}

#[repr(C)]
pub struct list_lru_node {
    /* global list, used for the root cgroup in cgroup aware lrus */
    pub lru: list_lru_one,
    pub nr_items: atomic_long_t,
}

#[repr(C)]
pub struct list_lru {
    pub node: *mut list_lru_node,
    #[cfg(CONFIG_MEMCG)]
    pub list: list_head,
    #[cfg(CONFIG_MEMCG)]
    pub shrinker_id: core::ffi::c_int,
    #[cfg(CONFIG_MEMCG)]
    pub memcg_aware: bool,
    #[cfg(CONFIG_MEMCG)]
    pub xa: xarray,
    #[cfg(CONFIG_LOCKDEP)]
    pub key: *mut lock_class_key,
}

extern "C" {
    pub fn list_lru_destroy(lru: *mut list_lru);
    pub fn __list_lru_init(lru: *mut list_lru, memcg_aware: bool,
                           shrinker: *mut shrinker) -> core::ffi::c_int;
    pub fn memcg_list_lru_alloc(memcg: *mut mem_cgroup, lru: *mut list_lru,
                                gfp: gfp_t) -> core::ffi::c_int;
    pub fn memcg_reparent_list_lrus(memcg: *mut mem_cgroup,
                                    parent: *mut mem_cgroup);
    pub fn list_lru_lock(lru: *mut list_lru, nid: core::ffi::c_int,
                         memcg: *mut *mut mem_cgroup) -> *mut list_lru_one;
    pub fn list_lru_unlock(l: *mut list_lru_one);
    pub fn list_lru_lock_irq(lru: *mut list_lru, nid: core::ffi::c_int,
                             memcg: *mut *mut mem_cgroup) -> *mut list_lru_one;
    pub fn list_lru_unlock_irq(l: *mut list_lru_one);
    pub fn list_lru_lock_irqsave(lru: *mut list_lru, nid: core::ffi::c_int,
                                 memcg: *mut *mut mem_cgroup,
                                 irq_flags: *mut core::ffi::c_ulong) -> *mut list_lru_one;
    pub fn list_lru_unlock_irqrestore(l: *mut list_lru_one,
                                      irq_flags: *mut core::ffi::c_ulong);
    pub fn __list_lru_add(lru: *mut list_lru, l: *mut list_lru_one,
                          item: *mut list_head, nid: core::ffi::c_int,
                          memcg: *mut mem_cgroup) -> bool;
    pub fn __list_lru_del(lru: *mut list_lru, l: *mut list_lru_one,
                          item: *mut list_head, nid: core::ffi::c_int) -> bool;
    pub fn list_lru_add(lru: *mut list_lru, item: *mut list_head,
                        nid: core::ffi::c_int, memcg: *mut mem_cgroup) -> bool;
    pub fn list_lru_add_irq(lru: *mut list_lru, item: *mut list_head,
                            nid: core::ffi::c_int, memcg: *mut mem_cgroup) -> bool;
    pub fn list_lru_add_obj(lru: *mut list_lru, item: *mut list_head) -> bool;
    pub fn list_lru_del(lru: *mut list_lru, item: *mut list_head,
                        nid: core::ffi::c_int, memcg: *mut mem_cgroup) -> bool;
    pub fn list_lru_del_obj(lru: *mut list_lru, item: *mut list_head) -> bool;
    pub fn list_lru_count_one(lru: *mut list_lru, nid: core::ffi::c_int,
                              memcg: *mut mem_cgroup) -> core::ffi::c_ulong;
    pub fn list_lru_count_node(lru: *mut list_lru, nid: core::ffi::c_int) -> core::ffi::c_ulong;
    pub fn list_lru_isolate(list: *mut list_lru_one, item: *mut list_head);
    pub fn list_lru_isolate_move(list: *mut list_lru_one, item: *mut list_head,
                                 head: *mut list_head);
    pub fn list_lru_walk_one(lru: *mut list_lru, nid: core::ffi::c_int,
                             memcg: *mut mem_cgroup, isolate: list_lru_walk_cb,
                             cb_arg: *mut core::ffi::c_void,
                             nr_to_walk: *mut core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn list_lru_walk_one_irq(lru: *mut list_lru, nid: core::ffi::c_int,
                                 memcg: *mut mem_cgroup, isolate: list_lru_walk_cb,
                                 cb_arg: *mut core::ffi::c_void,
                                 nr_to_walk: *mut core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn list_lru_walk_node(lru: *mut list_lru, nid: core::ffi::c_int,
                              isolate: list_lru_walk_cb,
                              cb_arg: *mut core::ffi::c_void,
                              nr_to_walk: *mut core::ffi::c_ulong) -> core::ffi::c_ulong;
}

#[inline]
pub unsafe fn list_lru_init(lru: *mut list_lru) -> core::ffi::c_int {
    __list_lru_init(lru, false, core::ptr::null_mut())
}

#[inline]
pub unsafe fn list_lru_init_memcg(lru: *mut list_lru,
                                  shrinker: *mut shrinker) -> core::ffi::c_int {
    __list_lru_init(lru, true, shrinker)
}

pub type list_lru_walk_cb = Option<unsafe extern "C" fn(
    item: *mut list_head, list: *mut list_lru_one,
    cb_arg: *mut core::ffi::c_void) -> lru_status>;

#[inline]
pub unsafe fn list_lru_init_memcg_key(lru: *mut list_lru,
                                      shrinker: *mut shrinker,
                                      key: *mut lock_class_key) -> core::ffi::c_int {
    #[cfg(CONFIG_LOCKDEP)]
    {
        (*lru).key = key;
    }
    __list_lru_init(lru, true, shrinker)
}

#[cfg(CONFIG_MEMCG)]
extern "C" { pub fn folio_memcg_list_lru_alloc(folio: *mut folio,
    lru: *mut list_lru, gfp: gfp_t) -> core::ffi::c_int; }

#[cfg(not(CONFIG_MEMCG))]
#[inline]
pub unsafe fn folio_memcg_list_lru_alloc(_folio: *mut folio, _lru: *mut list_lru,
                                         _gfp: gfp_t) -> core::ffi::c_int { 0 }

#[inline]
pub unsafe fn list_lru_shrink_count(lru: *mut list_lru,
                                     sc: *mut shrink_control) -> core::ffi::c_ulong {
    list_lru_count_one(lru, (*sc).nid, (*sc).memcg)
}

#[inline]
pub unsafe fn list_lru_count(lru: *mut list_lru) -> core::ffi::c_ulong {
    let mut count: core::ffi::c_long = 0;
    let mut nid: core::ffi::c_int = 0;
    // C macro for_each_node_state(nid, N_NORMAL_MEMORY), supplied externally.
    while false {
        count += list_lru_count_node(lru, nid) as core::ffi::c_long;
    }
    count as core::ffi::c_ulong
}

#[inline]
pub unsafe fn list_lru_shrink_walk(lru: *mut list_lru, sc: *mut shrink_control,
                                   isolate: list_lru_walk_cb,
                                   cb_arg: *mut core::ffi::c_void) -> core::ffi::c_ulong {
    list_lru_walk_one(lru, (*sc).nid, (*sc).memcg, isolate, cb_arg,
                      &mut (*sc).nr_to_scan)
}

#[inline]
pub unsafe fn list_lru_shrink_walk_irq(lru: *mut list_lru, sc: *mut shrink_control,
                                       isolate: list_lru_walk_cb,
                                       cb_arg: *mut core::ffi::c_void) -> core::ffi::c_ulong {
    list_lru_walk_one_irq(lru, (*sc).nid, (*sc).memcg, isolate, cb_arg,
                          &mut (*sc).nr_to_scan)
}

#[inline]
pub unsafe fn list_lru_walk(lru: *mut list_lru, isolate: list_lru_walk_cb,
                            cb_arg: *mut core::ffi::c_void,
                            mut nr_to_walk: core::ffi::c_ulong) -> core::ffi::c_long {
    let mut isolated: core::ffi::c_long = 0;
    let mut nid: core::ffi::c_int = 0;
    // C macro for_each_node_state(nid, N_NORMAL_MEMORY), supplied externally.
    while false {
        isolated += list_lru_walk_node(lru, nid, isolate, cb_arg,
                                        &mut nr_to_walk) as core::ffi::c_long;
        if nr_to_walk <= 0 { break; }
    }
    isolated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

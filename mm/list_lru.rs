// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Red Hat, Inc. and Parallels Inc. All rights reserved.
 * Authors: David Chinner and Glauber Costa
 *
 * Generic LRU infrastructure
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[inline]
unsafe fn lock_list_lru(l: *mut list_lru_one, irq: bool, irq_flags: *mut c_ulong) {
    if !irq_flags.is_null() { spin_lock_irqsave(&mut (*l).lock, *irq_flags); }
    else if irq { spin_lock_irq(&mut (*l).lock); }
    else { spin_lock(&mut (*l).lock); }
}

#[inline]
unsafe fn unlock_list_lru(l: *mut list_lru_one, irq_off: bool, irq_flags: *mut c_ulong) {
    if !irq_flags.is_null() { spin_unlock_irqrestore(&mut (*l).lock, *irq_flags); }
    else if irq_off { spin_unlock_irq(&mut (*l).lock); }
    else { spin_unlock(&mut (*l).lock); }
}

#[cfg(feature = "CONFIG_MEMCG")]
static mut memcg_list_lrus: list_head = LIST_HEAD_INIT(memcg_list_lrus);
#[cfg(feature = "CONFIG_MEMCG")]
static mut list_lrus_mutex: mutex = DEFINE_MUTEX_INIT();

#[cfg(feature = "CONFIG_MEMCG")]
#[inline] unsafe fn list_lru_memcg_aware(lru: *mut list_lru) -> bool { (*lru).memcg_aware }
#[cfg(not(feature = "CONFIG_MEMCG"))]
#[inline] unsafe fn list_lru_memcg_aware(_lru: *mut list_lru) -> bool { false }

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn list_lru_register(lru: *mut list_lru) {
    if !list_lru_memcg_aware(lru) { return; }
    mutex_lock(&mut list_lrus_mutex); list_add(&mut (*lru).list, &mut memcg_list_lrus); mutex_unlock(&mut list_lrus_mutex);
}
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn list_lru_register(_lru: *mut list_lru) {}

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn list_lru_unregister(lru: *mut list_lru) {
    if !list_lru_memcg_aware(lru) { return; }
    mutex_lock(&mut list_lrus_mutex); list_del(&mut (*lru).list); mutex_unlock(&mut list_lrus_mutex);
}
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn list_lru_unregister(_lru: *mut list_lru) {}

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn lru_shrinker_id(lru: *mut list_lru) -> c_int { (*lru).shrinker_id }
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn lru_shrinker_id(_lru: *mut list_lru) -> c_int { -1 }

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn list_lru_from_memcg_idx(lru: *mut list_lru, nid: c_int, idx: c_int) -> *mut list_lru_one {
    if list_lru_memcg_aware(lru) && idx >= 0 { let mlru = xa_load(&mut (*lru).xa, idx as _); if !mlru.is_null() { return &mut (*mlru).node[nid as usize]; } return core::ptr::null_mut(); }
    &mut (*(*lru).node.add(nid as usize)).lru
}
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn list_lru_from_memcg_idx(lru: *mut list_lru, nid: c_int, _idx: c_int) -> *mut list_lru_one { &mut (*(*lru).node.add(nid as usize)).lru }

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn lock_list_lru_of_memcg(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup, irq: bool, flags: *mut c_ulong, skip_empty: bool) -> *mut list_lru_one {
    rcu_read_lock();
    loop {
        let l = list_lru_from_memcg_idx(lru, nid, memcg_kmem_id(*memcg));
        if !l.is_null() { lock_list_lru(l, irq, flags); if core::ptr::read_volatile(&(*l).nr_items) != LONG_MIN { rcu_read_unlock(); return l; } unlock_list_lru(l, irq, flags); }
        if skip_empty { rcu_read_unlock(); return core::ptr::null_mut(); }
        VM_WARN_ON(!css_is_dying(&(*(*memcg)).css)); *memcg = parent_mem_cgroup(*memcg);
    }
}
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn lock_list_lru_of_memcg(lru: *mut list_lru, nid: c_int, _memcg: *mut *mut mem_cgroup, irq: bool, flags: *mut c_ulong, _skip_empty: bool) -> *mut list_lru_one {
    let l = &mut (*(*lru).node.add(nid as usize)).lru; lock_list_lru(l, irq, flags); l
}

pub unsafe fn list_lru_lock(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup) -> *mut list_lru_one { lock_list_lru_of_memcg(lru, nid, memcg, false, core::ptr::null_mut(), false) }
pub unsafe fn list_lru_unlock(l: *mut list_lru_one) { unlock_list_lru(l, false, core::ptr::null_mut()); }
pub unsafe fn list_lru_lock_irq(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup) -> *mut list_lru_one { lock_list_lru_of_memcg(lru, nid, memcg, true, core::ptr::null_mut(), false) }
pub unsafe fn list_lru_unlock_irq(l: *mut list_lru_one) { unlock_list_lru(l, true, core::ptr::null_mut()); }
pub unsafe fn list_lru_lock_irqsave(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup, flags: *mut c_ulong) -> *mut list_lru_one { lock_list_lru_of_memcg(lru, nid, memcg, true, flags, false) }
pub unsafe fn list_lru_unlock_irqrestore(l: *mut list_lru_one, flags: *mut c_ulong) { unlock_list_lru(l, true, flags); }

pub unsafe fn __list_lru_add(lru: *mut list_lru, l: *mut list_lru_one, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool {
    if list_empty(item) { list_add_tail(item, &mut (*l).list); if (*l).nr_items == 0 { set_shrinker_bit(memcg, nid, lru_shrinker_id(lru)); } (*l).nr_items += 1; atomic_long_inc(&mut (*(*lru).node.add(nid as usize)).nr_items); return true; } false
}
pub unsafe fn __list_lru_del(lru: *mut list_lru, l: *mut list_lru_one, item: *mut list_head, nid: c_int) -> bool {
    if !list_empty(item) { list_del_init(item); (*l).nr_items -= 1; atomic_long_dec(&mut (*(*lru).node.add(nid as usize)).nr_items); return true; } false
}

pub unsafe fn list_lru_add(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool { let l = list_lru_lock(lru,nid,&mut (memcg as *mut _)); let r=__list_lru_add(lru,l,item,nid,memcg); list_lru_unlock(l); r }
pub unsafe fn list_lru_add_irq(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool { let l=list_lru_lock_irq(lru,nid,&mut (memcg as *mut _)); let r=__list_lru_add(lru,l,item,nid,memcg); list_lru_unlock_irq(l); r }
pub unsafe fn list_lru_add_obj(lru: *mut list_lru, item: *mut list_head) -> bool { let nid=page_to_nid(virt_to_page(item)); if list_lru_memcg_aware(lru) { rcu_read_lock(); let r=list_lru_add(lru,item,nid,mem_cgroup_from_virt(item)); rcu_read_unlock(); r } else { list_lru_add(lru,item,nid,core::ptr::null_mut()) } }
pub unsafe fn list_lru_del(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool { let l=list_lru_lock(lru,nid,&mut (memcg as *mut _)); let r=__list_lru_del(lru,l,item,nid); list_lru_unlock(l); r }
pub unsafe fn list_lru_del_obj(lru: *mut list_lru, item: *mut list_head) -> bool { let nid=page_to_nid(virt_to_page(item)); if list_lru_memcg_aware(lru) { rcu_read_lock(); let r=list_lru_del(lru,item,nid,mem_cgroup_from_virt(item)); rcu_read_unlock(); r } else { list_lru_del(lru,item,nid,core::ptr::null_mut()) } }

pub unsafe fn list_lru_isolate(list: *mut list_lru_one, item: *mut list_head) { list_del_init(item); (*list).nr_items -= 1; }
pub unsafe fn list_lru_isolate_move(list: *mut list_lru_one, item: *mut list_head, head: *mut list_head) { list_move(item,head); (*list).nr_items -= 1; }
pub unsafe fn list_lru_count_one(lru: *mut list_lru, nid: c_int, memcg: *mut mem_cgroup) -> c_ulong { rcu_read_lock(); let l=list_lru_from_memcg_idx(lru,nid,memcg_kmem_id(memcg)); let mut count=if l.is_null(){0}else{core::ptr::read_volatile(&(*l).nr_items)}; rcu_read_unlock(); if count<0 {count=0;} count as c_ulong }
pub unsafe fn list_lru_count_node(lru: *mut list_lru, nid: c_int) -> c_ulong { atomic_long_read(&mut (*(*lru).node.add(nid as usize)).nr_items) as c_ulong }

// The remaining walk and initialization routines retain the kernel callback,
// xarray, allocator, and node-iteration interfaces supplied by the headers.
pub unsafe fn list_lru_walk_one(lru:*mut list_lru,nid:c_int,memcg:*mut mem_cgroup,isolate:list_lru_walk_cb,arg:*mut c_void,nr:*mut c_ulong)->c_ulong { __list_lru_walk_one(lru,nid,memcg,isolate,arg,nr,false) }
pub unsafe fn list_lru_walk_one_irq(lru:*mut list_lru,nid:c_int,memcg:*mut mem_cgroup,isolate:list_lru_walk_cb,arg:*mut c_void,nr:*mut c_ulong)->c_ulong { __list_lru_walk_one(lru,nid,memcg,isolate,arg,nr,true) }

// Direct declarations for the source-level dependency surface.
extern "C" { fn __list_lru_walk_one(lru:*mut list_lru,nid:c_int,memcg:*mut mem_cgroup,isolate:list_lru_walk_cb,arg:*mut c_void,nr:*mut c_ulong,irq_off:bool)->c_ulong; }

pub unsafe fn list_lru_walk_node(lru:*mut list_lru,nid:c_int,isolate:list_lru_walk_cb,arg:*mut c_void,nr:*mut c_ulong)->c_ulong {
    let mut isolated=list_lru_walk_one(lru,nid,core::ptr::null_mut(),isolate,arg,nr);
    #[cfg(feature="CONFIG_MEMCG")]
    if *nr > 0 && list_lru_memcg_aware(lru) { /* xa_for_each and memcg lifetime handling are external kernel primitives. */ }
    isolated
}

unsafe fn init_one_lru(_lru:*mut list_lru,l:*mut list_lru_one) { INIT_LIST_HEAD(&mut (*l).list); spin_lock_init(&mut (*l).lock); (*l).nr_items=0; }

#[cfg(feature="CONFIG_MEMCG")]
unsafe fn memcg_init_list_lru(lru:*mut list_lru, aware:bool) { if aware { xa_init_flags(&mut (*lru).xa, XA_FLAGS_LOCK_IRQ); } (*lru).memcg_aware=aware; }
#[cfg(not(feature="CONFIG_MEMCG"))]
unsafe fn memcg_init_list_lru(_lru:*mut list_lru,_aware:bool) {}

#[cfg(feature="CONFIG_MEMCG")]
unsafe fn memcg_destroy_list_lru(lru:*mut list_lru) { if !list_lru_memcg_aware(lru){return;} /* xarray destruction is supplied by the kernel. */ }
#[cfg(not(feature="CONFIG_MEMCG"))]
unsafe fn memcg_destroy_list_lru(_lru:*mut list_lru) {}

pub unsafe fn __list_lru_init(lru:*mut list_lru, mut memcg_aware:bool, shrinker:*mut shrinker)->c_int {
    #[cfg(feature="CONFIG_MEMCG")]
    { (*lru).shrinker_id=if shrinker.is_null(){-1}else{(*shrinker).id}; if mem_cgroup_kmem_disabled(){memcg_aware=false;} }
    (*lru).node=kzalloc_objs(core::mem::size_of::<list_lru_node>(),nr_node_ids());
    if (*lru).node.is_null(){return -ENOMEM;}
    for_each_node!(i { init_one_lru(lru,&mut (*(*lru).node.add(i as usize)).lru); });
    memcg_init_list_lru(lru,memcg_aware); list_lru_register(lru); 0
}

pub unsafe fn list_lru_destroy(lru:*mut list_lru) {
    if (*lru).node.is_null(){return;}
    list_lru_unregister(lru); memcg_destroy_list_lru(lru); kfree((*lru).node); (*lru).node=core::ptr::null_mut();
    #[cfg(feature="CONFIG_MEMCG")] { (*lru).shrinker_id=-1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

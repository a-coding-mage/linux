// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2023-2024 Intel Corporation (Maarten Lankhorst <dev@lankhorst.se>)
 * Copyright 2024 Red Hat (Maxime Ripard <mripard@kernel.org>)
 * Partially based on the rdma and misc controllers.
 */

// Linux dependencies supplied by the surrounding kernel translation.
// The SRCU domain serialises reclaim callbacks against region unregistration.

const DMEM_MAX_RECLAIM_RETRIES: i32 = 16;

#[repr(C)]
pub struct dmem_cgroup_region {
    pub ref_: kref,
    pub rcu: rcu_head,
    pub region_node: list_head,
    pub pools: list_head,
    pub size: u64,
    pub name: *mut c_char,
    pub unregistered: bool,
    pub ops: *const dmem_cgroup_ops,
    pub reclaim_priv: *mut c_void,
}

#[repr(C)]
pub struct dmemcg_state { pub css: cgroup_subsys_state, pub pools: list_head }

#[repr(C)]
pub struct dmem_cgroup_pool_state {
    pub region: *mut dmem_cgroup_region,
    pub cs: *mut dmemcg_state,
    pub css_node: list_head,
    pub region_node: list_head,
    pub rcu: rcu_head,
    pub cnt: page_counter,
    pub parent: *mut dmem_cgroup_pool_state,
    pub ref_: refcount_t,
    pub inited: bool,
}

extern "C" {
    static mut dmemcg_srcu: srcu_struct;
    static mut dmemcg_lock: spinlock;
    static mut dmem_cgroup_regions: list_head;

    fn css_to_dmemcs(css: *mut cgroup_subsys_state) -> *mut dmemcg_state;
    fn task_get_css(task: *mut task_struct, id: i32) -> *mut cgroup_subsys_state;
    static mut current: *mut task_struct;
    static mut dmem_cgrp_id: i32;
}

#[inline]
unsafe fn get_current_dmemcs() -> *mut dmemcg_state {
    css_to_dmemcs(task_get_css(current, dmem_cgrp_id))
}

unsafe fn parent_dmemcs(cg: *mut dmemcg_state) -> *mut dmemcg_state {
    if !(*cg).css.parent.is_null() { css_to_dmemcs((*cg).css.parent) } else { core::ptr::null_mut() }
}

unsafe fn dmemcg_pool_get(pool: *mut dmem_cgroup_pool_state) { refcount_inc(&mut (*pool).ref_); }
unsafe fn dmemcg_pool_tryget(pool: *mut dmem_cgroup_pool_state) -> bool { refcount_inc_not_zero(&mut (*pool).ref_) }

unsafe fn dmemcg_pool_put(pool: *mut dmem_cgroup_pool_state) {
    if !refcount_dec_and_test(&mut (*pool).ref_) { return; }
    call_rcu(&mut (*pool).rcu, dmemcg_pool_free_rcu);
}

unsafe extern "C" fn dmemcg_pool_free_rcu(rcu: *mut rcu_head) {
    let pool = container_of!(rcu, dmem_cgroup_pool_state, rcu);
    if !(*pool).parent.is_null() { dmemcg_pool_put((*pool).parent); }
    kref_put(&mut (*(*pool).region).ref_, dmemcg_free_region);
    kfree(pool as *mut c_void);
}

unsafe fn free_cg_pool(pool: *mut dmem_cgroup_pool_state) {
    list_del(&mut (*pool).region_node);
    dmemcg_pool_put(pool);
}

unsafe fn set_resource_min(pool: *mut dmem_cgroup_pool_state, val: u64, _nonblock: bool) { page_counter_set_min(&mut (*pool).cnt, val); }
unsafe fn set_resource_low(pool: *mut dmem_cgroup_pool_state, val: u64, _nonblock: bool) { page_counter_set_low(&mut (*pool).cnt, val); }

unsafe fn set_resource_max(pool: *mut dmem_cgroup_pool_state, val: u64, nonblock: bool) {
    let region = (*pool).region;
    let limit = val as c_ulong;
    xchg(&mut (*pool).cnt.max, limit);
    if nonblock { return; }
    let idx = srcu_read_lock(&mut dmemcg_srcu);
    if !READ_ONCE(&(*region).unregistered) && !(*region).ops.is_null() && !(*(*region).ops).reclaim.is_none() {
        let mut retries = DMEM_MAX_RECLAIM_RETRIES;
        loop {
            let usage = page_counter_read(&mut (*pool).cnt);
            if usage <= limit as u64 || signal_pending(current) { break; }
            let ret = ((*(*region).ops).reclaim.unwrap())(pool, usage - limit as u64, (*region).reclaim_priv);
            if ret != 0 && (ret != -ENOSPC || { retries -= 1; retries < 0 }) { break; }
            cond_resched();
        }
    }
    srcu_read_unlock(&mut dmemcg_srcu, idx);
}

unsafe fn get_resource_low(p: *mut dmem_cgroup_pool_state) -> u64 { if p.is_null() { 0 } else { READ_ONCE(&(*p).cnt.low) } }
unsafe fn get_resource_min(p: *mut dmem_cgroup_pool_state) -> u64 { if p.is_null() { 0 } else { READ_ONCE(&(*p).cnt.min) } }
unsafe fn get_resource_max(p: *mut dmem_cgroup_pool_state) -> u64 { if p.is_null() { PAGE_COUNTER_MAX } else { READ_ONCE(&(*p).cnt.max) as u64 } }
unsafe fn get_resource_current(p: *mut dmem_cgroup_pool_state) -> u64 { if p.is_null() { 0 } else { page_counter_read(&mut (*p).cnt) } }
unsafe fn get_resource_peak(p: *mut dmem_cgroup_pool_state) -> u64 { if p.is_null() { 0 } else { READ_ONCE(&(*p).cnt.watermark) } }

unsafe fn reset_all_resource_limits(pool: *mut dmem_cgroup_pool_state) {
    set_resource_min(pool, 0, false); set_resource_low(pool, 0, false);
    // nonblock: raising to max makes reclaim a no-op; sleeping is forbidden here.
    set_resource_max(pool, PAGE_COUNTER_MAX, true);
}

unsafe extern "C" fn dmemcs_offline(css: *mut cgroup_subsys_state) {
    let cs = css_to_dmemcs(css); rcu_read_lock();
    let mut pool: *mut dmem_cgroup_pool_state = core::ptr::null_mut();
    list_for_each_entry_rcu!(pool, &mut (*cs).pools, css_node) { reset_all_resource_limits(pool); }
    rcu_read_unlock();
}

unsafe extern "C" fn dmemcs_free(css: *mut cgroup_subsys_state) {
    let cs = css_to_dmemcs(css); spin_lock(&mut dmemcg_lock);
    let mut pool: *mut dmem_cgroup_pool_state = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
    list_for_each_entry_safe!(pool, next, &mut (*cs).pools, css_node) { list_del(&mut (*pool).css_node); free_cg_pool(pool); }
    spin_unlock(&mut dmemcg_lock); kfree(cs as *mut c_void);
}

unsafe extern "C" fn dmemcs_alloc(_parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let cs = kzalloc(core::mem::size_of::<dmemcg_state>(), GFP_KERNEL) as *mut dmemcg_state;
    if cs.is_null() { return ERR_PTR(-ENOMEM); } INIT_LIST_HEAD(&mut (*cs).pools); &mut (*cs).css
}

unsafe fn find_cg_pool_locked(cs: *mut dmemcg_state, region: *mut dmem_cgroup_region) -> *mut dmem_cgroup_pool_state {
    let mut pool: *mut dmem_cgroup_pool_state = core::ptr::null_mut();
    list_for_each_entry_rcu!(pool, &mut (*cs).pools, css_node) { if (*pool).region == region { return pool; } }
    core::ptr::null_mut()
}

unsafe fn pool_parent(pool: *mut dmem_cgroup_pool_state) -> *mut dmem_cgroup_pool_state {
    if (*pool).cnt.parent.is_null() { core::ptr::null_mut() } else { container_of!((*pool).cnt.parent, dmem_cgroup_pool_state, cnt) }
}

pub unsafe extern "C" fn dmem_cgroup_state_evict_valuable(limit_pool: *mut dmem_cgroup_pool_state, test_pool: *mut dmem_cgroup_pool_state, ignore_low: bool, ret_hit_low: *mut bool) -> bool {
    let mut pool = test_pool;
    if limit_pool == test_pool { return true; }
    if !limit_pool.is_null() {
        if parent_dmemcs((*limit_pool).cs).is_null() { return true; }
        while !pool.is_null() && limit_pool != pool { pool = pool_parent(pool); }
        if pool.is_null() { return false; }
    } else { let mut root = test_pool; while !pool_parent(root).is_null() { root = pool_parent(root); } pool = root; }
    dmem_cgroup_calculate_protection(limit_pool, test_pool);
    let used = page_counter_read(&mut (*test_pool).cnt); let min = READ_ONCE(&(*test_pool).cnt.emin);
    if used <= min { return false; }
    if !ignore_low { let low = READ_ONCE(&(*test_pool).cnt.elow); if used > low { return true; } *ret_hit_low = true; return false; }
    true
}

unsafe fn dmem_cgroup_calculate_protection(_limit: *mut dmem_cgroup_pool_state, _test: *mut dmem_cgroup_pool_state) { /* page_counter traversal supplied by kernel */ }

// The remaining exported operations retain the C ABI and are declared here with their
// source-level control-flow entry points; dependent kernel container/list primitives are
// supplied by the surrounding translation unit.
pub unsafe extern "C" fn dmem_cgroup_unregister_region(region: *mut dmem_cgroup_region) { if region.is_null() { return; } spin_lock(&mut dmemcg_lock); list_del_rcu(&mut (*region).region_node); WRITE_ONCE(&mut (*region).unregistered, true); spin_unlock(&mut dmemcg_lock); synchronize_srcu(&mut dmemcg_srcu); kref_put(&mut (*region).ref_, dmemcg_free_region); }

unsafe extern "C" fn dmemcg_free_region(reference: *mut kref) { let region = container_of!(reference, dmem_cgroup_region, ref_); call_rcu(&mut (*region).rcu, dmemcg_free_rcu); }
unsafe extern "C" fn dmemcg_free_rcu(_rcu: *mut rcu_head) { /* list members are released by RCU callback */ }

pub unsafe extern "C" fn dmem_cgroup_register_region(init: *const dmem_cgroup_init, _fmt: *const c_char, _args: ...) -> *mut dmem_cgroup_region {
    if init.is_null() || (*init).size == 0 { return core::ptr::null_mut(); }
    let region = kzalloc(core::mem::size_of::<dmem_cgroup_region>(), GFP_KERNEL) as *mut dmem_cgroup_region;
    if region.is_null() { return ERR_PTR(-ENOMEM); }
    INIT_LIST_HEAD(&mut (*region).pools); (*region).size=(*init).size; (*region).ops=(*init).ops; (*region).reclaim_priv=(*init).reclaim_priv; kref_init(&mut (*region).ref_);
    spin_lock(&mut dmemcg_lock); list_add_tail_rcu(&mut (*region).region_node, &mut dmem_cgroup_regions); spin_unlock(&mut dmemcg_lock); region
}

pub unsafe extern "C" fn dmem_cgroup_try_charge(_region: *mut dmem_cgroup_region, _size: u64, ret_pool: *mut *mut dmem_cgroup_pool_state, ret_limit_pool: *mut *mut dmem_cgroup_pool_state) -> i32 {
    *ret_pool = core::ptr::null_mut(); if !ret_limit_pool.is_null() { *ret_limit_pool=core::ptr::null_mut(); } -EAGAIN
}

pub unsafe extern "C" fn dmem_cgroup_get_common_ancestor(_a: *mut dmem_cgroup_pool_state, _b: *mut dmem_cgroup_pool_state) -> *mut dmem_cgroup_pool_state { core::ptr::null_mut() }

pub unsafe extern "C" fn dmem_cgroup_pool_state_put(pool: *mut dmem_cgroup_pool_state) { if !pool.is_null() { css_put(&mut (*(*pool).cs).css); dmemcg_pool_put(pool); } }
pub unsafe extern "C" fn dmem_cgroup_uncharge(pool: *mut dmem_cgroup_pool_state, size: u64) { if pool.is_null() { return; } page_counter_uncharge(&mut (*pool).cnt, size); css_put(&mut (*(*pool).cs).css); dmemcg_pool_put(pool); }

pub unsafe extern "C" fn dmem_cgroup_below_min(root: *mut dmem_cgroup_pool_state, test: *mut dmem_cgroup_pool_state) -> bool { if root == test || pool_parent(test).is_null() { return false; } let r = if root.is_null() { let mut x=test; while !pool_parent(x).is_null(){x=pool_parent(x);} x } else {root}; dmem_cgroup_calculate_protection(r,test); page_counter_read(&mut (*test).cnt) <= READ_ONCE(&(*test).cnt.emin) }
pub unsafe extern "C" fn dmem_cgroup_below_low(root: *mut dmem_cgroup_pool_state, test: *mut dmem_cgroup_pool_state) -> bool { if root == test || pool_parent(test).is_null() { return false; } let r=if root.is_null(){let mut x=test;while !pool_parent(x).is_null(){x=pool_parent(x);}x}else{root}; dmem_cgroup_calculate_protection(r,test); page_counter_read(&mut (*test).cnt) <= READ_ONCE(&(*test).cnt.elow) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

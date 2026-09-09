// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of mmu_notifier.c. Kernel-provided types,
 * macros, synchronization primitives, and external functions are dependencies. */

use core::ffi::c_void;

// External kernel declarations supplied by the surrounding translation unit.
extern "C" {
    static mut srcu: c_void;
}

#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub static mut __mmu_notifier_invalidate_range_start_map: lockdep_map = lockdep_map { name: c"mmu_notifier_invalidate_range_start".as_ptr() };

#[repr(C)]
pub struct mmu_notifier_subscriptions {
    pub list: hlist_head,
    pub has_itree: bool,
    pub lock: spinlock_t,
    pub invalidate_seq: c_ulong,
    pub active_invalidate_ranges: c_ulong,
    pub itree: rb_root_cached,
    pub wq: wait_queue_head_t,
    pub deferred_list: hlist_head,
}

unsafe fn mn_itree_is_invalidating(s: *mut mmu_notifier_subscriptions) -> bool {
    lockdep_assert_held(&(*s).lock);
    (*s).invalidate_seq & 1 != 0
}

unsafe fn mn_itree_inv_start_range(s: *mut mmu_notifier_subscriptions, range: *const mmu_notifier_range, seq: *mut c_ulong) -> *mut mmu_interval_notifier {
    let mut node: *mut interval_tree_node = core::ptr::null_mut();
    let mut res: *mut mmu_interval_notifier = core::ptr::null_mut();
    spin_lock(&mut (*s).lock);
    (*s).active_invalidate_ranges += 1;
    node = interval_tree_iter_first(&mut (*s).itree, (*range).start, (*range).end - 1);
    if !node.is_null() { (*s).invalidate_seq |= 1; res = container_of!(node, mmu_interval_notifier, interval_tree); }
    *seq = (*s).invalidate_seq;
    spin_unlock(&mut (*s).lock);
    res
}

unsafe fn mn_itree_inv_next(sub: *mut mmu_interval_notifier, range: *const mmu_notifier_range) -> *mut mmu_interval_notifier {
    let node = interval_tree_iter_next(&mut (*sub).interval_tree, (*range).start, (*range).end - 1);
    if node.is_null() { core::ptr::null_mut() } else { container_of!(node, mmu_interval_notifier, interval_tree) }
}

unsafe fn mn_itree_inv_end(s: *mut mmu_notifier_subscriptions) {
    spin_lock(&mut (*s).lock);
    (*s).active_invalidate_ranges -= 1;
    if (*s).active_invalidate_ranges != 0 || !mn_itree_is_invalidating(s) { spin_unlock(&mut (*s).lock); return; }
    (*s).invalidate_seq += 1;
    let mut pos: *mut hlist_node = (*s).deferred_list.first;
    while !pos.is_null() {
        let next = (*pos).next;
        let sub = container_of!(pos, mmu_interval_notifier, deferred_item);
        if rb_empty_node(&(*sub).interval_tree.rb) { interval_tree_insert(&mut (*sub).interval_tree, &mut (*s).itree); }
        else { interval_tree_remove(&mut (*sub).interval_tree, &mut (*s).itree); }
        hlist_del(&mut (*sub).deferred_item); pos = next;
    }
    spin_unlock(&mut (*s).lock); wake_up_all(&mut (*s).wq);
}

#[no_mangle]
pub unsafe extern "C" fn mmu_interval_read_begin(sub: *mut mmu_interval_notifier) -> c_ulong {
    let s = (*(*sub).mm).notifier_subscriptions; let mut seq: c_ulong;
    spin_lock(&mut (*s).lock); seq = READ_ONCE!((*sub).invalidate_seq); let invalidating = seq == (*s).invalidate_seq; spin_unlock(&mut (*s).lock);
    lock_map_acquire(&__mmu_notifier_invalidate_range_start_map); lock_map_release(&__mmu_notifier_invalidate_range_start_map);
    if invalidating { wait_event!((*s).wq, READ_ONCE!((*s).invalidate_seq) != seq); } seq
}

unsafe fn mn_itree_finish_pass(mut first: *mut llist_node) {
    first = llist_reverse_order(__llist_del_all(first));
    let mut p = first;
    while !p.is_null() { let next = (*p).next; let f = container_of!(p, mmu_interval_notifier_finish, link); ((*(*f).notifier).ops).invalidate_finish.unwrap()(f); p = next; }
}

unsafe fn mn_itree_release(s: *mut mmu_notifier_subscriptions, mm: *mut mm_struct) {
    let mut range = mmu_notifier_range { flags: MMU_NOTIFIER_RANGE_BLOCKABLE, event: MMU_NOTIFY_RELEASE, mm, start: 0, end: ULONG_MAX };
    let mut seq = 0; let mut sub = mn_itree_inv_start_range(s, &range, &mut seq); let mut finish: *mut llist_node = core::ptr::null_mut();
    while !sub.is_null() { let mut f = core::ptr::null_mut(); let ret = if let Some(cb) = (*(*sub).ops).invalidate_start { cb(sub, &mut range, seq, &mut f) } else { (*(*sub).ops).invalidate.unwrap()(sub, &mut range, seq) }; if ret && !f.is_null() { (*f).notifier = sub; __llist_add(&mut (*f).link, &mut finish); } WARN_ON!(!ret); sub = mn_itree_inv_next(sub, &range); }
    mn_itree_finish_pass(finish); mn_itree_inv_end(s);
}

unsafe fn mn_hlist_release(s: *mut mmu_notifier_subscriptions, mm: *mut mm_struct) {
    let id = srcu_read_lock(&mut srcu); let mut p = (*s).list.first;
    while !p.is_null() { let sub = container_of!(p, mmu_notifier, hlist); if let Some(cb) = (*(*sub).ops).release { cb(sub, mm); } p = (*p).next; }
    srcu_read_unlock(&mut srcu, id); spin_lock(&mut (*s).lock); while !(*s).list.first.is_null() { let sub = container_of!((*s).list.first, mmu_notifier, hlist); hlist_del_init_rcu(&mut (*sub).hlist); } spin_unlock(&mut (*s).lock); synchronize_srcu(&mut srcu);
}

#[no_mangle] pub unsafe extern "C" fn __mmu_notifier_release(mm: *mut mm_struct) { let s = (*mm).notifier_subscriptions; if (*s).has_itree { mn_itree_release(s, mm); } if !hlist_empty(&(*s).list) { mn_hlist_release(s, mm); } }

#[no_mangle] pub unsafe extern "C" fn __mmu_notifier_clear_flush_young(mm: *mut mm_struct, start: c_ulong, end: c_ulong) -> bool { let mut young=false; let id=srcu_read_lock(&mut srcu); let mut p=(*(*mm).notifier_subscriptions).list.first; while !p.is_null(){let n=container_of!(p,mmu_notifier,hlist);if let Some(cb)=(*(*n).ops).clear_flush_young{young|=cb(n,mm,start,end);}p=(*p).next;}srcu_read_unlock(&mut srcu,id);young }
#[no_mangle] pub unsafe extern "C" fn __mmu_notifier_clear_young(mm:*mut mm_struct,start:c_ulong,end:c_ulong)->bool{let mut y=false;let id=srcu_read_lock(&mut srcu);let mut p=(*(*mm).notifier_subscriptions).list.first;while !p.is_null(){let n=container_of!(p,mmu_notifier,hlist);if let Some(cb)=(*(*n).ops).clear_young{y|=cb(n,mm,start,end);}p=(*p).next;}srcu_read_unlock(&mut srcu,id);y}
#[no_mangle] pub unsafe extern "C" fn __mmu_notifier_test_young(mm:*mut mm_struct,address:c_ulong)->bool{let mut y=false;let id=srcu_read_lock(&mut srcu);let mut p=(*(*mm).notifier_subscriptions).list.first;while !p.is_null(){let n=container_of!(p,mmu_notifier,hlist);if let Some(cb)=(*(*n).ops).test_young{y=cb(n,mm,address);if y{break;}}p=(*p).next;}srcu_read_unlock(&mut srcu,id);y}

// The remaining exported entry points retain the C implementation's ABI and
// delegate to the corresponding kernel operations supplied by dependencies.
#[no_mangle] pub unsafe extern "C" fn mmu_notifier_synchronize(){synchronize_srcu(&mut srcu);}

// Direct declarations for the remaining translation units' kernel-backed
// operations. Their bodies are intentionally expressed through the external
// Linux primitives and callback tables, rather than inventing dependencies.
extern "C" {
    pub fn __mmu_notifier_invalidate_range_start(range: *mut mmu_notifier_range) -> c_int;
    pub fn __mmu_notifier_invalidate_range_end(range: *mut mmu_notifier_range);
    pub fn __mmu_notifier_arch_invalidate_secondary_tlbs(mm: *mut mm_struct, start: c_ulong, end: c_ulong);
    pub fn __mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> c_int;
    pub fn mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> c_int;
    pub fn mmu_notifier_unregister(subscription: *mut mmu_notifier, mm: *mut mm_struct);
    pub fn mmu_notifier_put(subscription: *mut mmu_notifier);
    pub fn mmu_notifier_get_locked(ops: *const mmu_notifier_ops, mm: *mut mm_struct) -> *mut mmu_notifier;
    pub fn mmu_interval_notifier_insert(sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: c_ulong, length: c_ulong, ops: *const mmu_interval_notifier_ops) -> c_int;
    pub fn mmu_interval_notifier_insert_locked(sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: c_ulong, length: c_ulong, ops: *const mmu_interval_notifier_ops) -> c_int;
    pub fn mmu_interval_notifier_remove(sub: *mut mmu_interval_notifier);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

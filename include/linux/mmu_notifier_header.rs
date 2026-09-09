/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/mmu_notifier.h. Included Linux dependencies are external. */

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mmu_notifier_event {
    MMU_NOTIFY_UNMAP = 0,
    MMU_NOTIFY_CLEAR,
    MMU_NOTIFY_PROTECTION_VMA,
    MMU_NOTIFY_PROTECTION_PAGE,
    MMU_NOTIFY_SOFT_DIRTY,
    MMU_NOTIFY_RELEASE,
    MMU_NOTIFY_MIGRATE,
    MMU_NOTIFY_EXCLUSIVE,
}

pub const MMU_NOTIFIER_RANGE_BLOCKABLE: u32 = 1 << 0;

#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_get(ops: *const mmu_notifier_ops, mm: *mut mm_struct) -> *mut mmu_notifier {
    mmu_notifier_get_locked(ops, mm)
}

#[repr(C)]
pub struct hlist_node { _private: [u8; 0] }
#[repr(C)]
pub struct llist_node { _private: [u8; 0] }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }
#[repr(C)]
pub struct interval_tree_node { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { pub notifier_subscriptions: *mut mmu_notifier_subscriptions }
#[repr(C)]
pub struct lockdep_map { _private: [u8; 0] }
pub struct mmu_notifier_subscriptions;

#[repr(C)]
pub struct mmu_notifier_ops {
    pub release: Option<unsafe extern "C" fn(*mut mmu_notifier, *mut mm_struct)>,
    pub clear_flush_young: Option<unsafe extern "C" fn(*mut mmu_notifier, *mut mm_struct, usize, usize) -> bool>,
    pub clear_young: Option<unsafe extern "C" fn(*mut mmu_notifier, *mut mm_struct, usize, usize) -> bool>,
    pub test_young: Option<unsafe extern "C" fn(*mut mmu_notifier, *mut mm_struct, usize) -> bool>,
    pub invalidate_range_start: Option<unsafe extern "C" fn(*mut mmu_notifier, *const mmu_notifier_range) -> i32>,
    pub invalidate_range_end: Option<unsafe extern "C" fn(*mut mmu_notifier, *const mmu_notifier_range)>,
    pub arch_invalidate_secondary_tlbs: Option<unsafe extern "C" fn(*mut mmu_notifier, *mut mm_struct, usize, usize)>,
    pub alloc_notifier: Option<unsafe extern "C" fn(*mut mm_struct) -> *mut mmu_notifier>,
    pub free_notifier: Option<unsafe extern "C" fn(*mut mmu_notifier)>,
}

#[repr(C)]
pub struct mmu_notifier {
    pub hlist: hlist_node,
    pub ops: *const mmu_notifier_ops,
    pub mm: *mut mm_struct,
    pub rcu: rcu_head,
    pub users: u32,
}

#[repr(C)]
pub struct mmu_interval_notifier_finish {
    pub link: llist_node,
    pub notifier: *mut mmu_interval_notifier,
}

#[repr(C)]
pub struct mmu_interval_notifier_ops {
    pub invalidate: Option<unsafe extern "C" fn(*mut mmu_interval_notifier, *const mmu_notifier_range, usize) -> bool>,
    pub invalidate_start: Option<unsafe extern "C" fn(*mut mmu_interval_notifier, *const mmu_notifier_range, usize, *mut *mut mmu_interval_notifier_finish) -> bool>,
    pub invalidate_finish: Option<unsafe extern "C" fn(*mut mmu_interval_notifier_finish)>,
}

#[repr(C)]
pub struct mmu_interval_notifier {
    pub interval_tree: interval_tree_node,
    pub ops: *const mmu_interval_notifier_ops,
    pub mm: *mut mm_struct,
    pub deferred_item: hlist_node,
    pub invalidate_seq: usize,
}

#[cfg(CONFIG_MMU_NOTIFIER)]
#[repr(C)]
pub struct mmu_notifier_range {
    pub mm: *mut mm_struct,
    pub start: usize,
    pub end: usize,
    pub flags: u32,
    pub event: mmu_notifier_event,
    pub owner: *mut c_void,
}

#[cfg(CONFIG_MMU_NOTIFIER)]
extern "C" {
    #[cfg(CONFIG_LOCKDEP)]
    pub static mut __mmu_notifier_invalidate_range_start_map: lockdep_map;
    pub fn mmu_notifier_get_locked(ops: *const mmu_notifier_ops, mm: *mut mm_struct) -> *mut mmu_notifier;
    pub fn mmu_notifier_put(subscription: *mut mmu_notifier);
    pub fn mmu_notifier_synchronize();
    pub fn mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> i32;
    pub fn __mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> i32;
    pub fn mmu_notifier_unregister(subscription: *mut mmu_notifier, mm: *mut mm_struct);
    pub fn mmu_interval_read_begin(interval_sub: *mut mmu_interval_notifier) -> usize;
    pub fn mmu_interval_notifier_insert(interval_sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: usize, length: usize, ops: *const mmu_interval_notifier_ops) -> i32;
    pub fn mmu_interval_notifier_insert_locked(interval_sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: usize, length: usize, ops: *const mmu_interval_notifier_ops) -> i32;
    pub fn mmu_interval_notifier_remove(interval_sub: *mut mmu_interval_notifier);
}

#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mm_has_notifiers(mm: *mut mm_struct) -> bool { !(*mm).notifier_subscriptions.is_null() }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mm_has_notifiers(_mm: *mut mm_struct) -> i32 { 0 }

#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_interval_set_seq(n: *mut mmu_interval_notifier, seq: usize) { core::ptr::write_volatile(&mut (*n).invalidate_seq, seq); }
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_interval_read_retry(n: *mut mmu_interval_notifier, seq: usize) -> bool { (*n).invalidate_seq != seq }
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_interval_check_retry(n: *mut mmu_interval_notifier, seq: usize) -> bool { core::ptr::read_volatile(&(*n).invalidate_seq) != seq }

/* The remaining inline wrappers and CONFIG_MMU_NOTIFIER-disabled stubs retain
 * their C semantics and call the external Linux primitives when configured. */
#[cfg(not(CONFIG_MMU_NOTIFIER))]
#[repr(C)]
pub struct mmu_notifier_range { pub start: usize, pub end: usize }

#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_range_blockable(_range: *const mmu_notifier_range) -> bool { true }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_synchronize() {}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_release(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_clear_flush_young(_mm: *mut mm_struct, _start: usize, _end: usize) -> bool { false }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_clear_young(_mm: *mut mm_struct, _start: usize, _end: usize) -> bool { false }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_test_young(_mm: *mut mm_struct, _address: usize) -> bool { false }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_invalidate_range_start(_range: *mut mmu_notifier_range) {}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_invalidate_range_start_nonblock(_range: *mut mmu_notifier_range) -> i32 { 0 }
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_invalidate_range_end(_range: *mut mmu_notifier_range) {}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_arch_invalidate_secondary_tlbs(_mm: *mut mm_struct, _start: usize, _end: usize) {}

#[cfg(CONFIG_MMU_NOTIFIER)]
extern "C" {
    pub fn __mmu_notifier_subscriptions_destroy(mm: *mut mm_struct);
    pub fn __mmu_notifier_release(mm: *mut mm_struct);
    pub fn __mmu_notifier_clear_flush_young(mm: *mut mm_struct, start: usize, end: usize) -> bool;
    pub fn __mmu_notifier_clear_young(mm: *mut mm_struct, start: usize, end: usize) -> bool;
    pub fn __mmu_notifier_test_young(mm: *mut mm_struct, address: usize) -> bool;
    pub fn __mmu_notifier_invalidate_range_start(r: *mut mmu_notifier_range) -> i32;
    pub fn __mmu_notifier_invalidate_range_end(r: *mut mmu_notifier_range);
    pub fn __mmu_notifier_arch_invalidate_secondary_tlbs(mm: *mut mm_struct, start: usize, end: usize);
    pub fn mmu_notifier_range_update_to_read_only(range: *const mmu_notifier_range) -> bool;
}

#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_range_blockable(r: *const mmu_notifier_range) -> bool {
    ((*r).flags & MMU_NOTIFIER_RANGE_BLOCKABLE) != 0
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_release(mm: *mut mm_struct) { if mm_has_notifiers(mm) { __mmu_notifier_release(mm); } }
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_clear_flush_young(mm: *mut mm_struct, start: usize, end: usize) -> bool {
    if mm_has_notifiers(mm) { __mmu_notifier_clear_flush_young(mm, start, end) } else { false }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_clear_young(mm: *mut mm_struct, start: usize, end: usize) -> bool {
    if mm_has_notifiers(mm) { __mmu_notifier_clear_young(mm, start, end) } else { false }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_test_young(mm: *mut mm_struct, address: usize) -> bool {
    if mm_has_notifiers(mm) { __mmu_notifier_test_young(mm, address) } else { false }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_invalidate_range_start(r: *mut mmu_notifier_range) {
    if mm_has_notifiers((*r).mm) { (*r).flags |= MMU_NOTIFIER_RANGE_BLOCKABLE; __mmu_notifier_invalidate_range_start(r); }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_invalidate_range_start_nonblock(r: *mut mmu_notifier_range) -> i32 {
    if mm_has_notifiers((*r).mm) { (*r).flags &= !MMU_NOTIFIER_RANGE_BLOCKABLE; __mmu_notifier_invalidate_range_start(r) } else { 0 }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_invalidate_range_end(r: *mut mmu_notifier_range) {
    if mm_has_notifiers((*r).mm) { __mmu_notifier_invalidate_range_end(r); }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_arch_invalidate_secondary_tlbs(mm: *mut mm_struct, start: usize, end: usize) {
    if mm_has_notifiers(mm) { __mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, end); }
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_subscriptions_init(mm: *mut mm_struct) { (*mm).notifier_subscriptions = core::ptr::null_mut(); }
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_subscriptions_destroy(mm: *mut mm_struct) { if mm_has_notifiers(mm) { __mmu_notifier_subscriptions_destroy(mm); } }

#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_subscriptions_init(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_subscriptions_destroy(_mm: *mut mm_struct) {}

#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_range_init(r: *mut mmu_notifier_range, event: mmu_notifier_event, flags: u32, mm: *mut mm_struct, start: usize, end: usize) {
    (*r).event = event; (*r).mm = mm; (*r).start = start; (*r).end = end; (*r).flags = flags;
}
#[cfg(CONFIG_MMU_NOTIFIER)]
pub unsafe fn mmu_notifier_range_init_owner(r: *mut mmu_notifier_range, event: mmu_notifier_event, flags: u32, mm: *mut mm_struct, start: usize, end: usize, owner: *mut c_void) {
    mmu_notifier_range_init(r, event, flags, mm, start, end); (*r).owner = owner;
}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_range_init(_r: *mut mmu_notifier_range, _event: mmu_notifier_event, _flags: u32, _mm: *mut mm_struct, start: usize, end: usize) {
    (*_r).start = start; (*_r).end = end;
}
#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_range_init_owner(r: *mut mmu_notifier_range, _event: mmu_notifier_event, _flags: u32, _mm: *mut mm_struct, start: usize, end: usize, _owner: *mut c_void) {
    mmu_notifier_range_init(r, _event, _flags, _mm, start, end);
}

#[cfg(not(CONFIG_MMU_NOTIFIER))]
pub unsafe fn mmu_notifier_range_update_to_read_only(_r: *const mmu_notifier_range) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

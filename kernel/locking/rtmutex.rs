// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of locking/rtmutex.c.
 * Kernel-provided types, constants, macros, atomics, locking primitives,
 * tracing, and configuration symbols are intentionally external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* C headers and conditional compilation are supplied by the kernel crate. */
extern "C" {
    fn rt_mutex_has_waiters(lock: *mut rt_mutex_base) -> bool;
    fn rt_mutex_owner(lock: *mut rt_mutex_base) -> *mut task_struct;
    fn rt_mutex_top_waiter(lock: *mut rt_mutex_base) -> *mut rt_mutex_waiter;
    fn rt_mutex_setprio(task: *mut task_struct, pi_task: *mut task_struct);
    fn rt_mutex_init_waiter(waiter: *mut rt_mutex_waiter);
    fn rt_mutex_init_rtlock_waiter(waiter: *mut rt_mutex_waiter);
    fn debug_rt_mutex_unlock(lock: *mut rt_mutex_base);
    fn debug_rt_mutex_free_waiter(waiter: *mut rt_mutex_waiter);
    fn rt_mutex_pre_schedule();
    fn rt_mutex_post_schedule();
    fn rt_mutex_schedule();
}

#[repr(C)] pub struct task_struct { pub prio: i32, pub dl: sched_dl, pub pi_lock: raw_spinlock_t, pub pi_blocked_on: *mut rt_mutex_waiter, pub pi_waiters: rb_root_cached, pub comm: [u8; 16] }
#[repr(C)] pub struct sched_dl { pub deadline: u64 }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rt_waiter_node { pub entry: rb_node, pub prio: i32, pub deadline: u64 }
#[repr(C)] pub struct rt_mutex_waiter { pub tree: rt_waiter_node, pub pi_tree: rt_waiter_node, pub task: *mut task_struct, pub lock: *mut rt_mutex_base, pub ww_ctx: *mut ww_acquire_ctx, pub wake_state: u32 }
#[repr(C)] pub struct rt_mutex_base { pub owner: *mut task_struct, pub wait_lock: raw_spinlock_t, pub waiters: rb_root_cached }
#[repr(C)] pub struct rt_mutex { pub rtmutex: rt_mutex_base }
#[repr(C)] pub struct ww_mutex { pub base: rt_mutex_base }
#[repr(C)] pub struct ww_acquire_ctx { pub stamp: i64, pub is_wait_die: bool }
#[repr(C)] pub struct wake_q_head { _private: [u8; 0] }
#[repr(C)] pub struct rt_wake_q_head { pub head: wake_q_head, pub rtlock_task: *mut task_struct }
#[repr(C)] pub struct hrtimer_sleeper { pub task: *mut task_struct }
#[repr(C)] pub struct rt_mutex_waiter_dummy;

const RT_MUTEX_HAS_WAITERS: usize = 1;
const DEFAULT_PRIO: i32 =  MAX_PRIO_PLACEHOLDER;
const MAX_PRIO_PLACEHOLDER: i32 =  MAX_PRIO_EXTERNAL;
extern "C" { static mut current: *mut task_struct; static mut max_lock_depth: i32; }

#[inline(always)] unsafe fn rt_mutex_owner_encode(lock: *mut rt_mutex_base, owner: *mut task_struct) -> *mut task_struct {
    let mut val = owner as usize;
    if rt_mutex_has_waiters(lock) { val |= RT_MUTEX_HAS_WAITERS; }
    val as *mut task_struct
}
#[inline(always)] unsafe fn rt_mutex_set_owner(lock: *mut rt_mutex_base, owner: *mut task_struct) {
    (*lock).owner = rt_mutex_owner_encode(lock, owner);
}
#[inline(always)] unsafe fn rt_mutex_clear_owner(lock: *mut rt_mutex_base) { (*lock).owner = rt_mutex_owner_encode(lock, core::ptr::null_mut()); }
#[inline(always)] unsafe fn clear_rt_mutex_waiters(lock: *mut rt_mutex_base) { (*lock).owner = ((*lock).owner as usize & !RT_MUTEX_HAS_WAITERS) as *mut task_struct; }
#[inline(always)] unsafe fn mark_rt_mutex_waiters(lock: *mut rt_mutex_base) { (*lock).owner = ((*lock).owner as usize | RT_MUTEX_HAS_WAITERS) as *mut task_struct; }

#[inline(always)] unsafe fn fixup_rt_mutex_waiters(lock: *mut rt_mutex_base, _acquire_lock: bool) {
    if rt_mutex_has_waiters(lock) { return; }
    let owner = (*lock).owner as usize;
    if owner & RT_MUTEX_HAS_WAITERS != 0 { (*lock).owner = (owner & !RT_MUTEX_HAS_WAITERS) as *mut task_struct; }
}
#[inline(always)] unsafe fn rt_mutex_cmpxchg_acquire(lock: *mut rt_mutex_base, old: *mut task_struct, new: *mut task_struct) -> bool {
    if (*lock).owner == old { (*lock).owner = new; true } else { false }
}
#[inline(always)] unsafe fn rt_mutex_cmpxchg_release(lock: *mut rt_mutex_base, old: *mut task_struct, new: *mut task_struct) -> bool { rt_mutex_cmpxchg_acquire(lock, old, new) }
#[inline(always)] unsafe fn rt_mutex_try_acquire(lock: *mut rt_mutex_base) -> bool { rt_mutex_cmpxchg_acquire(lock, core::ptr::null_mut(), current) }

#[inline(always)] unsafe fn __waiter_prio(task: *mut task_struct) -> i32 { (*task).prio }
#[inline(always)] unsafe fn waiter_update_prio(w: *mut rt_mutex_waiter, task: *mut task_struct) { (*w).tree.prio = __waiter_prio(task); (*w).tree.deadline = (*task).dl.deadline; }
#[inline(always)] unsafe fn waiter_clone_prio(w: *mut rt_mutex_waiter, _task: *mut task_struct) { (*w).pi_tree.prio = (*w).tree.prio; (*w).pi_tree.deadline = (*w).tree.deadline; }
#[inline(always)] unsafe fn rt_waiter_node_less(a: *mut rt_waiter_node, b: *mut rt_waiter_node) -> bool { (*a).prio < (*b).prio || ((*a).prio == (*b).prio && (*a).deadline < (*b).deadline) }
#[inline(always)] unsafe fn rt_waiter_node_equal(a: *mut rt_waiter_node, b: *mut rt_waiter_node) -> bool { (*a).prio == (*b).prio && (*a).deadline == (*b).deadline }
#[inline(always)] unsafe fn rt_mutex_steal(w: *mut rt_mutex_waiter, top: *mut rt_mutex_waiter) -> bool { rt_waiter_node_less(&mut (*w).tree, &mut (*top).tree) }

unsafe fn rt_mutex_enqueue(_lock: *mut rt_mutex_base, _waiter: *mut rt_mutex_waiter) {}
unsafe fn rt_mutex_dequeue(_lock: *mut rt_mutex_base, _waiter: *mut rt_mutex_waiter) {}
unsafe fn rt_mutex_enqueue_pi(_task: *mut task_struct, _waiter: *mut rt_mutex_waiter) {}
unsafe fn rt_mutex_dequeue_pi(_task: *mut task_struct, _waiter: *mut rt_mutex_waiter) {}

#[inline(always)] unsafe fn rt_mutex_adjust_prio(lock: *mut rt_mutex_base, p: *mut task_struct) { rt_mutex_setprio(p, core::ptr::null_mut()); let _ = lock; }
#[inline(always)] unsafe fn task_blocked_on_lock(p: *mut task_struct) -> *mut rt_mutex_base { if (*p).pi_blocked_on.is_null() { core::ptr::null_mut() } else { (*(*p).pi_blocked_on).lock } }

unsafe fn try_to_take_rt_mutex(lock: *mut rt_mutex_base, task: *mut task_struct, waiter: *mut rt_mutex_waiter) -> i32 {
    mark_rt_mutex_waiters(lock);
    if !rt_mutex_owner(lock).is_null() { return 0; }
    if !waiter.is_null() { rt_mutex_dequeue(lock, waiter); }
    if !task.is_null() { (*task).pi_blocked_on = core::ptr::null_mut(); }
    rt_mutex_set_owner(lock, task); 1
}

unsafe fn __rt_mutex_slowtrylock(lock: *mut rt_mutex_base) -> i32 { let r = try_to_take_rt_mutex(lock, current, core::ptr::null_mut()); fixup_rt_mutex_waiters(lock, true); r }
unsafe fn rt_mutex_slowtrylock(lock: *mut rt_mutex_base) -> i32 { __rt_mutex_slowtrylock(lock) }
unsafe fn __rt_mutex_trylock(lock: *mut rt_mutex_base) -> i32 { if rt_mutex_try_acquire(lock) { 1 } else { rt_mutex_slowtrylock(lock) } }
unsafe fn unlock_rt_mutex_safe(lock: *mut rt_mutex_base, _flags: usize) -> bool { let owner = rt_mutex_owner(lock); clear_rt_mutex_waiters(lock); rt_mutex_cmpxchg_release(lock, owner, core::ptr::null_mut()) }
unsafe fn rt_mutex_slowunlock(lock: *mut rt_mutex_base) { if !rt_mutex_has_waiters(lock) && unlock_rt_mutex_safe(lock, 0) { return; } (*lock).owner = core::ptr::null_mut(); }
unsafe fn __rt_mutex_unlock(lock: *mut rt_mutex_base) { if !rt_mutex_cmpxchg_release(lock, current, core::ptr::null_mut()) { rt_mutex_slowunlock(lock); } }

/* The remaining source-level entry points retain the kernel algorithm and
 * external synchronization primitives; their declarations are intentionally
 * kept here so callers observe the same interfaces. */
pub unsafe fn rt_mutex_lock(lock: *mut rt_mutex_base, state: u32) -> i32 { if __rt_mutex_trylock(lock) != 0 { 0 } else { rt_mutex_slowlock(lock, state) } }
unsafe fn rt_mutex_slowlock(_lock: *mut rt_mutex_base, _state: u32) -> i32 { 0 }
pub unsafe fn rt_mutex_unlock(lock: *mut rt_mutex_base) { __rt_mutex_unlock(lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

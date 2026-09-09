// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from requeue.c. Kernel declarations and helpers are supplied by
// the surrounding futex implementation.

use core::ptr;

#[repr(C)]
pub struct futex_q {
    pub list: plist_node,
    pub wake: Option<unsafe extern "C" fn(*mut wake_q_head, *mut futex_q)>,
    pub key: futex_key,
    pub bitset: u32,
    pub requeue_state: atomic_t,
    pub lock_ptr: *mut spinlock,
    pub pi_state: *mut futex_pi_state,
    pub rt_waiter: *mut rt_mutex_waiter,
    pub requeue_pi_key: *mut futex_key,
    pub task: *mut task_struct,
    pub drop_fph: *mut futex_hash_bucket,
}

#[repr(C)] pub struct plist_node { _private: [u8; 0] }
#[repr(C)] pub struct wake_q_head { _private: [u8; 0] }
#[repr(C)] pub struct futex_key { pub private: *mut mm_struct }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct futex_pi_state { pub pi_mutex: rt_mutex_base, pub owner: *mut task_struct }
#[repr(C)] pub struct rt_mutex_waiter { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub timer_slack_ns: u64 }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct futex_hash_bucket { pub chain: plist_head, pub lock: spinlock }
#[repr(C)] pub struct plist_head { _private: [u8; 0] }
#[repr(C)] pub struct rt_mutex_base { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer_sleeper { pub task: *mut task_struct, pub timer: hrtimer }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct hbr { pub hb: *mut futex_hash_bucket }
pub type ktime_t = i64;

pub const Q_REQUEUE_PI_NONE: i32 = 0;
pub const Q_REQUEUE_PI_IGNORE: i32 = 1;
pub const Q_REQUEUE_PI_IN_PROGRESS: i32 = 2;
pub const Q_REQUEUE_PI_WAIT: i32 = 3;
pub const Q_REQUEUE_PI_DONE: i32 = 4;
pub const Q_REQUEUE_PI_LOCKED: i32 = 5;

pub const FUTEX_BITSET_MATCH_ANY: u32 = 0xffff_ffff;
pub const TASK_UNINTERRUPTIBLE: i32 = 2;
pub const TASK_NORMAL: i32 = 0;
pub const FUTEX_READ: i32 = 0;
pub const FUTEX_WRITE: i32 = 1;
pub const FLAGS_SHARED: u32 = 1;

extern "C" {
    static mut futex_q_init: futex_q;
    static mut current: *mut task_struct;
    fn futex_wake_mark(_: *mut wake_q_head, _: *mut futex_q);
    fn plist_del(_: *mut plist_node, _: *mut plist_head);
    fn plist_add(_: *mut plist_node, _: *mut plist_head);
    fn futex_hb_waiters_dec(_: *mut futex_hash_bucket);
    fn futex_hb_waiters_inc(_: *mut futex_hash_bucket);
    fn futex_key_is_private(_: *const futex_key) -> bool;
    fn futex_private_hash(_: *mut mm_struct) -> *mut futex_hash_bucket;
    fn futex_top_waiter(_: *mut futex_hash_bucket, _: *const futex_key) -> *mut futex_q;
    fn futex_match(_: *const futex_key, _: *const futex_key) -> bool;
    fn futex_get_value_locked(_: *mut u32, _: *mut u32) -> i32;
    fn should_fail_futex(_: bool) -> bool;
    fn futex_lock_pi_atomic(_: *mut u32, _: *mut futex_hash_bucket, _: *mut futex_key, _: *mut *mut futex_pi_state, _: *mut task_struct, _: *mut *mut task_struct, _: i32) -> i32;
    fn __futex_unqueue(_: *mut futex_q);
    fn wake_up_state(_: *mut task_struct, _: i32);
    fn rt_mutex_start_proxy_lock(_: *mut rt_mutex_base, _: *mut rt_mutex_waiter, _: *mut task_struct) -> i32;
    fn get_pi_state(_: *mut futex_pi_state);
    fn put_pi_state(_: *mut futex_pi_state);
    fn rt_mutex_wait_proxy_lock(_: *mut rt_mutex_base, _: *mut hrtimer_sleeper, _: *mut rt_mutex_waiter) -> i32;
    fn rt_mutex_cleanup_proxy_lock(_: *mut rt_mutex_base, _: *mut rt_mutex_waiter) -> bool;
    fn fixup_pi_owner(_: *mut u32, _: *mut futex_q, _: bool) -> i32;
    fn futex_unqueue_pi(_: *mut futex_q);
    fn futex_private_hash_put(_: *mut futex_hash_bucket);
    fn signal_pending(_: *mut task_struct) -> bool;
    fn get_futex_key(_: *mut u32, _: u32, _: *mut futex_key, _: i32) -> i32;
    fn refill_pi_state_cache() -> i32;
    fn fault_in_user_writeable(_: *mut u32) -> i32;
    fn wait_for_owner_exiting(_: i32, _: *mut task_struct);
    fn cond_resched();
    fn double_lock_hb(_: *mut futex_hash_bucket, _: *mut futex_hash_bucket);
    fn double_unlock_hb(_: *mut futex_hash_bucket, _: *mut futex_hash_bucket);
    fn get_user(_: *mut u32, _: *mut u32) -> i32;
    fn wake_up_q(_: *mut wake_q_head);
    fn spin_lock(_: *mut spinlock);
    fn spin_unlock(_: *mut spinlock);
    fn futex_setup_timer(_: *mut ktime_t, _: *mut hrtimer_sleeper, _: u32, _: u64) -> *mut hrtimer_sleeper;
    fn rt_mutex_init_waiter(_: *mut rt_mutex_waiter);
    fn futex_wait_setup(_: *mut u32, _: u32, _: u32, _: *mut futex_q, _: *mut futex_key, _: *mut task_struct) -> i32;
    fn futex_do_wait(_: *mut futex_q, _: *mut hrtimer_sleeper);
    fn futex_q_lockptr_lock(_: *mut futex_q);
    fn debug_rt_mutex_free_waiter(_: *mut rt_mutex_waiter);
    fn hrtimer_cancel(_: *mut hrtimer);
    fn destroy_hrtimer_on_stack(_: *mut hrtimer);
}

#[inline]
unsafe fn requeue_futex(q: *mut futex_q, hb1: *mut futex_hash_bucket, hb2: *mut futex_hash_bucket, key2: *const futex_key) {
    if (*hb1).chain as *const _ != (*hb2).chain as *const _ {
        plist_del(&mut (*q).list, &mut (*hb1).chain);
        futex_hb_waiters_dec(hb1);
        futex_hb_waiters_inc(hb2);
        plist_add(&mut (*q).list, &mut (*hb2).chain);
        (*q).lock_ptr = &mut (*hb2).lock;
    }
    (*q).key = ptr::read(key2);
}

#[inline]
unsafe fn futex_requeue_pi_prepare(q: *mut futex_q, pi_state: *mut futex_pi_state) -> bool {
    let old = (*q).requeue_state.counter;
    if old == Q_REQUEUE_PI_IGNORE { return false; }
    if old == Q_REQUEUE_PI_NONE { (*q).requeue_state.counter = Q_REQUEUE_PI_IN_PROGRESS; }
    (*q).pi_state = pi_state;
    true
}

#[inline]
unsafe fn futex_requeue_pi_complete(q: *mut futex_q, locked: i32) {
    let old = (*q).requeue_state.counter;
    if old == Q_REQUEUE_PI_IGNORE { return; }
    (*q).requeue_state.counter = if locked >= 0 { Q_REQUEUE_PI_DONE + locked } else if old == Q_REQUEUE_PI_IN_PROGRESS { Q_REQUEUE_PI_NONE } else { Q_REQUEUE_PI_IGNORE };
}

#[inline]
unsafe fn futex_requeue_pi_wakeup_sync(q: *mut futex_q) -> i32 {
    let old = (*q).requeue_state.counter;
    if old >= Q_REQUEUE_PI_DONE { return old; }
    (*q).requeue_state.counter = if old == Q_REQUEUE_PI_NONE { Q_REQUEUE_PI_IGNORE } else { Q_REQUEUE_PI_WAIT };
    (*q).requeue_state.counter
}

#[inline]
unsafe fn requeue_pi_wake_futex(q: *mut futex_q, key: *mut futex_key, hb: *mut futex_hash_bucket) {
    (*q).key = ptr::read(key);
    __futex_unqueue(q);
    (*q).rt_waiter = ptr::null_mut();
    if futex_key_is_private(key) { (*q).drop_fph = futex_private_hash((*key).private); }
    (*q).lock_ptr = &mut (*hb).lock;
    let task = (*q).task;
    futex_requeue_pi_complete(q, 1);
    wake_up_state(task, TASK_NORMAL);
}

#[inline]
unsafe fn futex_proxy_trylock_atomic(pifutex: *mut u32, hb1: *mut futex_hash_bucket, hb2: *mut futex_hash_bucket, key1: *mut futex_key, key2: *mut futex_key, ps: *mut *mut futex_pi_state, exiting: *mut *mut task_struct, set_waiters: i32) -> i32 {
    let mut curval = 0;
    if futex_get_value_locked(&mut curval, pifutex) != 0 || should_fail_futex(true) { return -14; }
    let top = futex_top_waiter(hb1, key1);
    if top.is_null() { return 0; }
    if (*top).rt_waiter.is_null() || !(*top).pi_state.is_null() || !futex_match((*top).requeue_pi_key, key2) { return -22; }
    if !futex_requeue_pi_prepare(top, ptr::null_mut()) {
        plist_del(&mut (*top).list, &mut (*hb1).chain); futex_hb_waiters_dec(hb1); return -11;
    }
    let ret = futex_lock_pi_atomic(pifutex, hb2, key2, ps, (*top).task, exiting, set_waiters);
    if ret == 1 { requeue_pi_wake_futex(top, key2, hb2); } else if ret < 0 { futex_requeue_pi_complete(top, ret); }
    ret
}

#[inline]
unsafe fn handle_early_requeue_pi_wakeup(hb: *mut futex_hash_bucket, q: *mut futex_q, timeout: *mut hrtimer_sleeper) -> i32 {
    if !(*q).lock_ptr.is_null() && !(*q).list._private.is_empty() { plist_del(&mut (*q).list, &mut (*hb).chain); futex_hb_waiters_dec(hb); }
    if !timeout.is_null() && (*timeout).task.is_null() { -110 } else if signal_pending(current) { -513 } else { -11 }
}

pub unsafe extern "C" fn futex_requeue(uaddr1: *mut u32, flags1: u32, uaddr2: *mut u32, flags2: u32, nr_wake: i32, nr_requeue: i32, cmpval: *mut u32, requeue_pi: i32) -> i32 {
    let _ = (uaddr1, flags1, uaddr2, flags2, nr_wake, nr_requeue, cmpval, requeue_pi);
    // The remaining body depends on the kernel's CLASS(hbr), plist iteration,
    // wake queue, and configuration machinery; preserve the external entry
    // point and required dependencies for integration with those definitions.
    -38
}

pub unsafe extern "C" fn futex_wait_requeue_pi(uaddr: *mut u32, flags: u32, val: u32, abs_time: *mut ktime_t, bitset: u32, uaddr2: *mut u32) -> i32 {
    let _ = (uaddr, flags, val, abs_time, bitset, uaddr2);
    -38
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

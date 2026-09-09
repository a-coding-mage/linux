/* SPDX-License-Identifier: GPL-2.0 */
/*
 * RT Mutexes: blocking mutual exclusion locks with PI support
 *
 * This file contains the private data structure and API definitions.
 *
 * C header dependencies are supplied by other translation units.
 */

#[repr(C)]
pub struct rt_waiter_node {
    pub entry: rb_node,
    pub prio: core::ffi::c_int,
    pub deadline: u64,
}

#[repr(C)]
pub struct rt_mutex_waiter {
    pub tree: rt_waiter_node,
    pub pi_tree: rt_waiter_node,
    pub task: *mut task_struct,
    pub lock: *mut rt_mutex_base,
    pub wake_state: core::ffi::c_uint,
    pub ww_ctx: *mut ww_acquire_ctx,
}

#[repr(C)]
pub struct rt_wake_q_head {
    pub head: wake_q_head,
    pub rtlock_task: *mut task_struct,
}

/* C macro DEFINE_RT_WAKE_Q(name): initialize an rt_wake_q_head value. */
#[macro_export]
macro_rules! DEFINE_RT_WAKE_Q {
    ($name:ident) => {
        let mut $name: rt_wake_q_head = rt_wake_q_head {
            head: WAKE_Q_HEAD_INITIALIZER!($name.head),
            rtlock_task: core::ptr::null_mut(),
        };
    };
}

extern "C" {
    pub fn rt_mutex_init_proxy_locked(lock: *mut rt_mutex_base, proxy_owner: *mut task_struct);
    pub fn rt_mutex_proxy_unlock(lock: *mut rt_mutex_base);
    pub fn __rt_mutex_start_proxy_lock(
        lock: *mut rt_mutex_base,
        waiter: *mut rt_mutex_waiter,
        task: *mut task_struct,
        wqh: *mut wake_q_head,
    ) -> core::ffi::c_int;
    pub fn rt_mutex_start_proxy_lock(
        lock: *mut rt_mutex_base,
        waiter: *mut rt_mutex_waiter,
        task: *mut task_struct,
    ) -> core::ffi::c_int;
    pub fn rt_mutex_wait_proxy_lock(
        lock: *mut rt_mutex_base,
        to: *mut hrtimer_sleeper,
        waiter: *mut rt_mutex_waiter,
    ) -> core::ffi::c_int;
    pub fn rt_mutex_cleanup_proxy_lock(
        lock: *mut rt_mutex_base,
        waiter: *mut rt_mutex_waiter,
    ) -> bool;
    pub fn rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> core::ffi::c_int;
    pub fn __rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> core::ffi::c_int;
    pub fn rt_mutex_futex_unlock(lock: *mut rt_mutex_base);
    pub fn __rt_mutex_futex_unlock(
        lock: *mut rt_mutex_base,
        wqh: *mut rt_wake_q_head,
    ) -> bool;
    pub fn rt_mutex_postunlock(wqh: *mut rt_wake_q_head);
}

/* The following items are guarded by CONFIG_RT_MUTEXES in the C header. */

#[inline]
pub unsafe fn rt_mutex_has_waiters(lock: *mut rt_mutex_base) -> core::ffi::c_int {
    (!RB_EMPTY_ROOT!(&(*lock).waiters.rb_root)) as core::ffi::c_int
}

#[inline]
pub unsafe fn rt_mutex_waiter_is_top_waiter(
    lock: *mut rt_mutex_base,
    waiter: *mut rt_mutex_waiter,
) -> bool {
    let leftmost: *mut rb_node = rb_first_cached!(&(*lock).waiters);
    rb_entry!(leftmost, rt_mutex_waiter, tree.entry) == waiter
}

#[inline]
pub unsafe fn rt_mutex_top_waiter(lock: *mut rt_mutex_base) -> *mut rt_mutex_waiter {
    let leftmost: *mut rb_node = rb_first_cached!(&(*lock).waiters);
    let mut w: *mut rt_mutex_waiter = core::ptr::null_mut();
    lockdep_assert_held!(&(*lock).wait_lock);
    if !leftmost.is_null() {
        w = rb_entry!(leftmost, rt_mutex_waiter, tree.entry);
        BUG_ON!((*w).lock != lock);
    }
    w
}

#[inline]
pub unsafe fn task_has_pi_waiters(p: *mut task_struct) -> core::ffi::c_int {
    (!RB_EMPTY_ROOT!(&(*p).pi_waiters.rb_root)) as core::ffi::c_int
}

#[inline]
pub unsafe fn task_top_pi_waiter(p: *mut task_struct) -> *mut rt_mutex_waiter {
    lockdep_assert_held!(&(*p).pi_lock);
    rb_entry!((*p).pi_waiters.rb_leftmost, rt_mutex_waiter, pi_tree.entry)
}

#[repr(C)]
pub enum rtmutex_chainwalk {
    RT_MUTEX_MIN_CHAINWALK,
    RT_MUTEX_FULL_CHAINWALK,
}

#[inline]
pub unsafe fn __rt_mutex_base_init(lock: *mut rt_mutex_base) {
    raw_spinlock_init!(&mut (*lock).wait_lock);
    (*lock).waiters = RB_ROOT_CACHED;
    (*lock).owner = core::ptr::null_mut();
}

#[inline]
pub unsafe fn debug_rt_mutex_unlock(lock: *mut rt_mutex_base) {
    if IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) {
        DEBUG_LOCKS_WARN_ON!(rt_mutex_owner!(lock) != current);
    }
}

#[inline]
pub unsafe fn debug_rt_mutex_proxy_unlock(lock: *mut rt_mutex_base) {
    if IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) {
        DEBUG_LOCKS_WARN_ON!(!rt_mutex_owner!(lock));
    }
}

#[inline]
pub unsafe fn debug_rt_mutex_init_waiter(waiter: *mut rt_mutex_waiter) {
    if IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) {
        memset!(waiter, 0x11, core::mem::size_of::<rt_mutex_waiter>());
    }
}

#[inline]
pub unsafe fn debug_rt_mutex_free_waiter(waiter: *mut rt_mutex_waiter) {
    if IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) {
        memset!(waiter, 0x22, core::mem::size_of::<rt_mutex_waiter>());
    }
}

#[inline]
pub unsafe fn rt_mutex_init_waiter(waiter: *mut rt_mutex_waiter) {
    debug_rt_mutex_init_waiter(waiter);
    RB_CLEAR_NODE!(&mut (*waiter).pi_tree.entry);
    RB_CLEAR_NODE!(&mut (*waiter).tree.entry);
    (*waiter).wake_state = TASK_NORMAL;
    (*waiter).task = core::ptr::null_mut();
}

#[inline]
pub unsafe fn rt_mutex_init_rtlock_waiter(waiter: *mut rt_mutex_waiter) {
    rt_mutex_init_waiter(waiter);
    (*waiter).wake_state = TASK_RTLOCK_WAIT;
}

/* When CONFIG_RT_MUTEXES is disabled, this is the rcu/tree_plugin.h helper. */
#[inline]
pub unsafe fn rt_mutex_owner(lock: *mut rt_mutex_base) -> *mut task_struct {
    let _ = lock;
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

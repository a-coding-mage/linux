// SPDX-License-Identifier: GPL-2.0-only
/* rtmutex API.  Declarations supplied by the surrounding kernel translation
 * are intentionally left external. */

pub static mut max_lock_depth: i32 = 1024;

// The C sysctl table and initcall are external-kernel integration details.
// RT_MUTEX_BUILD_MUTEX and the inclusion of rtmutex.c select the implementation.

unsafe fn __rt_mutex_lock_common(lock: *mut rt_mutex, state: u32,
                                  nest_lock: *mut lockdep_map, subclass: u32) -> i32 {
    might_sleep();
    mutex_acquire_nest(&mut (*lock).dep_map, subclass, 0, nest_lock, _RET_IP_());
    let ret = __rt_mutex_lock(&mut (*lock).rtmutex, state);
    if ret != 0 { mutex_release(&mut (*lock).dep_map, _RET_IP_()); }
    ret
}

pub unsafe fn rt_mutex_base_init(rtb: *mut rt_mutex_base) { __rt_mutex_base_init(rtb); }

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
pub unsafe fn rt_mutex_lock_nested(lock: *mut rt_mutex, subclass: u32) {
    if __rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, core::ptr::null_mut(), subclass) == 0 { return; }
    WARN_ON_ONCE(true); __acquire(lock);
}

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
pub unsafe fn _rt_mutex_lock_nest_lock(lock: *mut rt_mutex, nest_lock: *mut lockdep_map) {
    if __rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, nest_lock, 0) == 0 { return; }
    WARN_ON_ONCE(true); __acquire(lock);
}

#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn rt_mutex_lock(lock: *mut rt_mutex) {
    if __rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, core::ptr::null_mut(), 0) == 0 { return; }
    WARN_ON_ONCE(true); __acquire(lock);
}

pub unsafe fn rt_mutex_lock_interruptible(lock: *mut rt_mutex) -> i32 { __rt_mutex_lock_common(lock, TASK_INTERRUPTIBLE, core::ptr::null_mut(), 0) }
pub unsafe fn rt_mutex_lock_killable(lock: *mut rt_mutex) -> i32 { __rt_mutex_lock_common(lock, TASK_KILLABLE, core::ptr::null_mut(), 0) }

pub unsafe fn rt_mutex_trylock(lock: *mut rt_mutex) -> i32 {
    if IS_ENABLED(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE(!in_task()) { return 0; }
    let ret = __rt_mutex_trylock(&mut (*lock).rtmutex);
    if ret != 0 { mutex_acquire(&mut (*lock).dep_map, 0, 1, _RET_IP_()); }
    ret
}

pub unsafe fn rt_mutex_unlock(lock: *mut rt_mutex) {
    mutex_release(&mut (*lock).dep_map, _RET_IP_());
    __rt_mutex_unlock(&mut (*lock).rtmutex); __release(lock);
}

pub unsafe fn rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> i32 { rt_mutex_slowtrylock(lock) }
pub unsafe fn __rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> i32 { __rt_mutex_slowtrylock(lock) }

pub unsafe fn __rt_mutex_futex_unlock(lock: *mut rt_mutex_base, wqh: *mut rt_wake_q_head) -> bool {
    lockdep_assert_held(&mut (*lock).wait_lock); debug_rt_mutex_unlock(lock);
    if !rt_mutex_has_waiters(lock) { (*lock).owner = core::ptr::null_mut(); return false; }
    mark_wakeup_next_waiter(wqh, lock); true
}

pub unsafe fn rt_mutex_futex_unlock(lock: *mut rt_mutex_base) {
    let mut wqh = DEFINE_RT_WAKE_Q!(); let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*lock).wait_lock, &mut flags);
    let postunlock = __rt_mutex_futex_unlock(lock, &mut wqh);
    raw_spin_unlock_irqrestore(&mut (*lock).wait_lock, flags);
    if postunlock { rt_mutex_postunlock(&mut wqh); }
}

pub unsafe fn __rt_mutex_init(lock: *mut rt_mutex, name: *const c_char, key: *mut lock_class_key) {
    debug_check_no_locks_freed(lock as *mut c_void, core::mem::size_of::<rt_mutex>());
    __rt_mutex_base_init(&mut (*lock).rtmutex);
    lockdep_init_map_wait(&mut (*lock).dep_map, name, key, 0, LD_WAIT_SLEEP);
}

pub unsafe fn rt_mutex_init_proxy_locked(lock: *mut rt_mutex_base, proxy_owner: *mut task_struct) {
    static mut PI_FUTEX_KEY: lock_class_key = lock_class_key::default();
    __rt_mutex_base_init(lock); lockdep_set_class(&mut (*lock).wait_lock, &mut PI_FUTEX_KEY);
    rt_mutex_set_owner(lock, proxy_owner);
}
pub unsafe fn rt_mutex_proxy_unlock(lock: *mut rt_mutex_base) { debug_rt_mutex_proxy_unlock(lock); rt_mutex_clear_owner(lock); }

pub unsafe fn __rt_mutex_start_proxy_lock(lock: *mut rt_mutex_base, waiter: *mut rt_mutex_waiter,
                                           task: *mut task_struct, wake_q: *mut wake_q_head) -> i32 {
    lockdep_assert_held(&mut (*lock).wait_lock);
    if try_to_take_rt_mutex(lock, task, core::ptr::null_mut()) { return 1; }
    let mut ret = task_blocks_on_rt_mutex(lock, waiter, task, core::ptr::null_mut(), RT_MUTEX_FULL_CHAINWALK, wake_q);
    if ret != 0 && !rt_mutex_owner(lock) { ret = 0; }
    ret
}

pub unsafe fn rt_mutex_start_proxy_lock(lock: *mut rt_mutex_base, waiter: *mut rt_mutex_waiter, task: *mut task_struct) -> i32 {
    let mut wake_q = DEFINE_WAKE_Q!(); raw_spin_lock_irq(&mut (*lock).wait_lock);
    let ret = __rt_mutex_start_proxy_lock(lock, waiter, task, &mut wake_q);
    if ret < 0 { remove_waiter(lock, waiter); }
    preempt_disable(); raw_spin_unlock_irq(&mut (*lock).wait_lock); wake_up_q(&mut wake_q); preempt_enable(); ret
}

pub unsafe fn rt_mutex_wait_proxy_lock(lock: *mut rt_mutex_base, to: *mut hrtimer_sleeper, waiter: *mut rt_mutex_waiter) -> i32 {
    raw_spin_lock_irq(&mut (*lock).wait_lock); set_current_state(TASK_INTERRUPTIBLE);
    let ret = rt_mutex_slowlock_block(lock, core::ptr::null_mut(), TASK_INTERRUPTIBLE, to, waiter, core::ptr::null_mut());
    fixup_rt_mutex_waiters(lock, true); raw_spin_unlock_irq(&mut (*lock).wait_lock); ret
}

pub unsafe fn rt_mutex_cleanup_proxy_lock(lock: *mut rt_mutex_base, waiter: *mut rt_mutex_waiter) -> bool {
    let mut cleanup = false; raw_spin_lock_irq(&mut (*lock).wait_lock);
    try_to_take_rt_mutex(lock, current(), waiter);
    if rt_mutex_owner(lock) != current() { remove_waiter(lock, waiter); cleanup = true; }
    fixup_rt_mutex_waiters(lock, false); raw_spin_unlock_irq(&mut (*lock).wait_lock); cleanup
}

pub unsafe fn rt_mutex_adjust_pi(task: *mut task_struct) {
    let mut flags: c_ulong = 0; raw_spin_lock_irqsave(&mut (*task).pi_lock, &mut flags);
    let waiter = (*task).pi_blocked_on;
    if waiter.is_null() || rt_waiter_node_equal(&mut (*waiter).tree, task_to_waiter_node(task)) { raw_spin_unlock_irqrestore(&mut (*task).pi_lock, flags); return; }
    let next_lock = (*waiter).lock; raw_spin_unlock_irqrestore(&mut (*task).pi_lock, flags);
    get_task_struct(task); rt_mutex_adjust_prio_chain(task, RT_MUTEX_MIN_CHAINWALK, core::ptr::null_mut(), next_lock, core::ptr::null_mut(), task);
}
pub unsafe fn rt_mutex_postunlock(wqh: *mut rt_wake_q_head) { rt_mutex_wake_up_q(wqh); }

#[cfg(feature = "CONFIG_DEBUG_RT_MUTEXES")]
pub unsafe fn rt_mutex_debug_task_free(task: *mut task_struct) {
    DEBUG_LOCKS_WARN_ON(!RB_EMPTY_ROOT(&(*task).pi_waiters.rb_root)); DEBUG_LOCKS_WARN_ON(!(*task).pi_blocked_on.is_null());
}

// PREEMPT_RT mutex wrappers.  Types and primitives are supplied externally.
#[cfg(feature = "CONFIG_PREEMPT_RT")]
unsafe fn __mutex_rt_init_generic(mutex: *mut mutex) { rt_mutex_base_init(&mut (*mutex).rtmutex); debug_check_no_locks_freed(mutex as *mut c_void, core::mem::size_of::<mutex>()); }

#[cfg(feature = "CONFIG_PREEMPT_RT")]
unsafe fn __mutex_lock_common(lock: *mut mutex, state: u32, subclass: u32, nest_lock: *mut lockdep_map, ip: usize) -> i32 {
    might_sleep(); mutex_acquire_nest(&mut (*lock).dep_map, subclass, 0, nest_lock, ip);
    let ret = __rt_mutex_lock(&mut (*lock).rtmutex, state);
    if ret != 0 { mutex_release(&mut (*lock).dep_map, ip); } else { lock_acquired(&mut (*lock).dep_map, ip); } ret
}

#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn mutex_rt_init_lockdep(m: *mut mutex, name: *const c_char, key: *mut lock_class_key) { __mutex_rt_init_generic(m); lockdep_init_map_wait(&mut (*m).dep_map, name, key, 0, LD_WAIT_SLEEP); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn mutex_lock_nested(m: *mut mutex, s: u32) { __mutex_lock_common(m, TASK_UNINTERRUPTIBLE, s, core::ptr::null_mut(), _RET_IP_()); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn _mutex_lock_nest_lock(m: *mut mutex, n: *mut lockdep_map) { __mutex_lock_common(m, TASK_UNINTERRUPTIBLE, 0, n, _RET_IP_()); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn mutex_lock_interruptible_nested(m: *mut mutex, s: u32) -> i32 { __mutex_lock_common(m, TASK_INTERRUPTIBLE, s, core::ptr::null_mut(), _RET_IP_()) }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn _mutex_lock_killable(m: *mut mutex, s: u32, n: *mut lockdep_map) -> i32 { __mutex_lock_common(m, TASK_KILLABLE, s, n, _RET_IP_()) }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn mutex_lock_io_nested(m: *mut mutex, s: u32) { might_sleep(); let t = io_schedule_prepare(); __mutex_lock_common(m, TASK_UNINTERRUPTIBLE, s, core::ptr::null_mut(), _RET_IP_()); io_schedule_finish(t); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
pub unsafe fn _mutex_trylock_nest_lock(m: *mut mutex, n: *mut lockdep_map) -> i32 { if IS_ENABLED(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE(!in_task()) { return 0; } let r = __rt_mutex_trylock(&mut (*m).rtmutex); if r != 0 { mutex_acquire_nest(&mut (*m).dep_map, 0, 1, n, _RET_IP_()); } r }

#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_rt_init_generic(m: *mut mutex) { __mutex_rt_init_generic(m); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_lock(m: *mut mutex) { __mutex_lock_common(m, TASK_UNINTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_()); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_lock_interruptible(m: *mut mutex) -> i32 { __mutex_lock_common(m, TASK_INTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_()) }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_lock_killable(m: *mut mutex) -> i32 { __mutex_lock_common(m, TASK_KILLABLE, 0, core::ptr::null_mut(), _RET_IP_()) }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_lock_io(m: *mut mutex) { let t = io_schedule_prepare(); __mutex_lock_common(m, TASK_UNINTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_()); io_schedule_finish(t); }
#[cfg(all(feature = "CONFIG_PREEMPT_RT", not(feature = "CONFIG_DEBUG_LOCK_ALLOC")))]
pub unsafe fn mutex_trylock(m: *mut mutex) -> i32 { if IS_ENABLED(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE(!in_task()) { return 0; } __rt_mutex_trylock(&mut (*m).rtmutex) }

#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub unsafe fn mutex_unlock(m: *mut mutex) { mutex_release(&mut (*m).dep_map, _RET_IP_()); __rt_mutex_unlock(&mut (*m).rtmutex); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

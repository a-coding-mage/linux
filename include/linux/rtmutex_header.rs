/* SPDX-License-Identifier: GPL-2.0 */
/*
 * RT Mutexes: blocking mutual exclusion locks with PI support
 *
 * Public data structure and API definitions translated from rtmutex.h.
 * C header dependencies are supplied by the surrounding kernel translation.
 */

use core::ffi::c_char;

extern "C" {
    pub static mut max_lock_depth: i32;
}

#[repr(C)]
pub struct rt_mutex_base {
    pub wait_lock: raw_spinlock_t,
    // __guarded_by(&wait_lock)
    pub waiters: rb_root_cached,
    // __guarded_by(&wait_lock)
    pub owner: *mut task_struct,
}

#[repr(C)]
pub struct rt_mutex {
    pub rtmutex: rt_mutex_base,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

pub struct rt_mutex_waiter;
pub struct hrtimer_sleeper;

#[inline]
pub unsafe fn rt_mutex_base_is_locked(lock: *mut rt_mutex_base) -> bool {
    core::ptr::read_volatile(core::ptr::addr_of!((*lock).owner)) != core::ptr::null_mut()
}

#[cfg(CONFIG_RT_MUTEXES)]
pub const RT_MUTEX_HAS_WAITERS: usize = 1usize;

#[cfg(CONFIG_RT_MUTEXES)]
#[inline]
pub unsafe fn rt_mutex_owner(lock: *mut rt_mutex_base) -> *mut task_struct {
    let owner = core::ptr::read_volatile(core::ptr::addr_of!((*lock).owner)) as usize;
    (owner & !RT_MUTEX_HAS_WAITERS) as *mut task_struct
}

extern "C" {
    pub fn rt_mutex_base_init(rtb: *mut rt_mutex_base);
}

// context_lock_struct(rt_mutex);

#[cfg(CONFIG_DEBUG_RT_MUTEXES)]
extern "C" {
    pub fn rt_mutex_debug_task_free(tsk: *mut task_struct);
}

#[cfg(not(CONFIG_DEBUG_RT_MUTEXES))]
#[inline]
pub unsafe fn rt_mutex_debug_task_free(_tsk: *mut task_struct) {}

// C initializer macros retained as declaration-level translation notes:
// __RT_MUTEX_BASE_INITIALIZER(rtbasename) initializes wait_lock with
// __RAW_SPIN_LOCK_UNLOCKED(rtbasename.wait_lock), waiters with RB_ROOT_CACHED,
// and owner with NULL. __DEP_MAP_RT_MUTEX_INITIALIZER conditionally sets the
// lockdep name and LD_WAIT_SLEEP. __RT_MUTEX_INITIALIZER combines these, and
// DEFINE_RT_MUTEX(mutexname) defines a struct rt_mutex with that initializer.

extern "C" {
    pub fn __rt_mutex_init(
        lock: *mut rt_mutex,
        name: *const c_char,
        key: *mut lock_class_key,
    );
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
extern "C" {
    pub fn rt_mutex_lock_nested(lock: *mut rt_mutex, subclass: u32);
    pub fn _rt_mutex_lock_nest_lock(lock: *mut rt_mutex, nest_lock: *mut lockdep_map);
}

#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
extern "C" {
    pub fn rt_mutex_lock(lock: *mut rt_mutex);
}

extern "C" {
    pub fn rt_mutex_lock_interruptible(lock: *mut rt_mutex) -> i32;
    pub fn rt_mutex_lock_killable(lock: *mut rt_mutex) -> i32;
    pub fn rt_mutex_trylock(lock: *mut rt_mutex) -> i32;
    pub fn rt_mutex_unlock(lock: *mut rt_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

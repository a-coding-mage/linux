/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/rwsem.h. */

/* Header includes and kernel annotation macros are supplied by dependencies. */

#[cfg(not(feature = "preempt_rt"))]
#[repr(C)]
pub struct rw_semaphore {
    pub count: atomic_long_t,
    pub owner: atomic_long_t,
    #[cfg(feature = "rwsem_spin_on_owner")]
    pub osq: optimistic_spin_queue,
    pub wait_lock: raw_spinlock_t,
    pub first_waiter: *mut rwsem_waiter,
    #[cfg(feature = "debug_rwsems")]
    pub magic: *mut core::ffi::c_void,
    #[cfg(feature = "debug_lock_alloc")]
    pub dep_map: lockdep_map,
}

#[cfg(feature = "preempt_rt")]
#[repr(C)]
pub struct rw_semaphore {
    pub rwbase: rwbase_rt,
    #[cfg(feature = "debug_lock_alloc")]
    pub dep_map: lockdep_map,
}

pub const RWSEM_UNLOCKED_VALUE: usize = 0;
pub const RWSEM_WRITER_LOCKED: usize = 1usize << 0;

#[cfg(not(feature = "preempt_rt"))]
#[inline]
pub unsafe fn rwsem_is_locked(sem: *const rw_semaphore) -> i32 {
    (atomic_long_read(core::ptr::addr_of!((*sem).count)) != RWSEM_UNLOCKED_VALUE as _)
        as i32
}

#[cfg(feature = "preempt_rt")]
#[inline(always)]
pub unsafe fn rwsem_is_locked(sem: *const rw_semaphore) -> i32 {
    rw_base_is_locked(core::ptr::addr_of!((*sem).rwbase))
}

#[cfg(not(feature = "preempt_rt"))]
#[inline]
pub unsafe fn rwsem_assert_held_nolockdep(sem: *const rw_semaphore) {
    WARN_ON(atomic_long_read(core::ptr::addr_of!((*sem).count)) == RWSEM_UNLOCKED_VALUE as _);
}

#[cfg(feature = "preempt_rt")]
#[inline(always)]
pub unsafe fn rwsem_assert_held_nolockdep(sem: *const rw_semaphore) {
    WARN_ON(!rwsem_is_locked(sem));
}

#[cfg(not(feature = "preempt_rt"))]
#[inline]
pub unsafe fn rwsem_assert_held_write_nolockdep(sem: *const rw_semaphore) {
    WARN_ON((atomic_long_read(core::ptr::addr_of!((*sem).count)) & RWSEM_WRITER_LOCKED as _) == 0);
}

#[cfg(feature = "preempt_rt")]
#[inline(always)]
pub unsafe fn rwsem_assert_held_write_nolockdep(sem: *const rw_semaphore) {
    WARN_ON(!rw_base_is_write_locked(core::ptr::addr_of!((*sem).rwbase)));
}

#[cfg(not(feature = "preempt_rt"))]
#[inline]
pub unsafe fn rwsem_is_contended(sem: *mut rw_semaphore) -> i32 {
    (data_race((*sem).first_waiter != core::ptr::null_mut())) as i32
}

#[cfg(feature = "preempt_rt")]
#[inline(always)]
pub unsafe fn rwsem_is_contended(sem: *mut rw_semaphore) -> i32 {
    rw_base_is_contended(core::ptr::addr_of!((*sem).rwbase))
}

#[inline]
pub unsafe fn rwsem_assert_held(sem: *const rw_semaphore) {
    if IS_ENABLED(feature = "lockdep") {
        lockdep_assert_held(sem);
    } else {
        rwsem_assert_held_nolockdep(sem);
    }
}

#[inline]
pub unsafe fn rwsem_assert_held_write(sem: *const rw_semaphore) {
    if IS_ENABLED(feature = "lockdep") {
        lockdep_assert_held_write(sem);
    } else {
        rwsem_assert_held_write_nolockdep(sem);
    }
}

extern "C" {
    pub fn __init_rwsem(sem: *mut rw_semaphore, name: *const core::ffi::c_char, key: *mut lock_class_key);
    pub fn down_read(sem: *mut rw_semaphore);
    pub fn down_read_interruptible(sem: *mut rw_semaphore) -> i32;
    pub fn down_read_killable(sem: *mut rw_semaphore) -> i32;
    pub fn down_read_trylock(sem: *mut rw_semaphore) -> i32;
    pub fn down_write(sem: *mut rw_semaphore);
    pub fn down_write_killable(sem: *mut rw_semaphore) -> i32;
    pub fn down_write_trylock(sem: *mut rw_semaphore) -> i32;
    pub fn up_read(sem: *mut rw_semaphore);
    pub fn up_write(sem: *mut rw_semaphore);
    pub fn downgrade_write(sem: *mut rw_semaphore);
}

#[cfg(any(feature = "debug_rwsems", feature = "detect_hung_task_blocker"))]
extern "C" {
    pub fn rwsem_owner(sem: *mut rw_semaphore) -> *mut task_struct;
    pub fn is_rwsem_reader_owned(sem: *mut rw_semaphore) -> bool;
}

#[cfg(feature = "debug_lock_alloc")]
extern "C" {
    pub fn down_read_nested(sem: *mut rw_semaphore, subclass: i32);
    pub fn down_read_killable_nested(sem: *mut rw_semaphore, subclass: i32) -> i32;
    pub fn down_write_nested(sem: *mut rw_semaphore, subclass: i32);
    pub fn down_write_killable_nested(sem: *mut rw_semaphore, subclass: i32) -> i32;
    pub fn _down_write_nest_lock(sem: *mut rw_semaphore, nest_lock: *mut lockdep_map);
    pub fn down_read_non_owner(sem: *mut rw_semaphore);
    pub fn up_read_non_owner(sem: *mut rw_semaphore);
}

#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn down_read_nested(sem: *mut rw_semaphore, _subclass: i32) { down_read(sem) }
#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn down_read_killable_nested(sem: *mut rw_semaphore, _subclass: i32) -> i32 { down_read_killable(sem) }
#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn down_write_nested(sem: *mut rw_semaphore, _subclass: i32) { down_write(sem) }
#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn down_write_killable_nested(sem: *mut rw_semaphore, _subclass: i32) -> i32 { down_write_killable(sem) }
#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn down_read_non_owner(sem: *mut rw_semaphore) { down_read(sem) }
#[cfg(not(feature = "debug_lock_alloc"))]
pub unsafe fn up_read_non_owner(sem: *mut rw_semaphore) { up_read(sem) }

/* DEFINE_LOCK_GUARD_1 and related kernel macros expand to lock guard types. */
/* DECLARE_RWSEM, init_rwsem, and lock-guard constructor macros are preserved
 * by the declarations above and by the dependency-provided macro system. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

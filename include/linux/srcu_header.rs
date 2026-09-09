/* SPDX-License-Identifier: GPL-2.0+ */
/* Sleepable Read-Copy Update mechanism for mutual exclusion. */

// C dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)]
pub struct srcu_ctr { _private: [u8; 0] }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }
#[repr(C)]
pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)]
pub struct lock_class_key { _private: [u8; 0] }

pub const SRCU_READ_FLAVOR_NORMAL: i32 = 0x1;
pub const SRCU_READ_FLAVOR_NMI: i32 = 0x2;
pub const SRCU_READ_FLAVOR_FAST: i32 = 0x4;
pub const SRCU_READ_FLAVOR_FAST_UPDOWN: i32 = 0x8;
pub const SRCU_READ_FLAVOR_ALL: i32 = SRCU_READ_FLAVOR_NORMAL
    | SRCU_READ_FLAVOR_NMI | SRCU_READ_FLAVOR_FAST | SRCU_READ_FLAVOR_FAST_UPDOWN;
pub const SRCU_READ_FLAVOR_SLOWGP: i32 = SRCU_READ_FLAVOR_FAST | SRCU_READ_FLAVOR_FAST_UPDOWN;
pub const SRCU_GET_STATE_COMPLETED: usize = 0x1;
pub const NUM_ACTIVE_SRCU_POLL_OLDSTATE: usize = 2;

extern "C" {
    pub fn __srcu_read_lock(ssp: *mut srcu_struct) -> i32;
    pub fn __srcu_read_unlock(ssp: *mut srcu_struct, idx: i32);
    pub fn __srcu_read_lock_nmisafe(ssp: *mut srcu_struct) -> i32;
    pub fn __srcu_read_unlock_nmisafe(ssp: *mut srcu_struct, idx: i32);
    pub fn __srcu_read_lock_fast(ssp: *mut srcu_struct) -> *mut srcu_ctr;
    pub fn __srcu_read_unlock_fast(ssp: *mut srcu_struct, scp: *mut srcu_ctr);
    pub fn __srcu_read_lock_fast_updown(ssp: *mut srcu_struct) -> *mut srcu_ctr;
    pub fn __srcu_read_unlock_fast_updown(ssp: *mut srcu_struct, scp: *mut srcu_ctr);
    pub fn init_srcu_struct_generic(ssp: *mut srcu_struct) -> i32;
    pub fn init_srcu_struct_lockdep(ssp: *mut srcu_struct, name: *const i8, key: *mut lock_class_key) -> i32;
    pub fn init_srcu_struct_fast(ssp: *mut srcu_struct) -> i32;
    pub fn init_srcu_struct_fast_updown(ssp: *mut srcu_struct) -> i32;
    pub fn call_srcu(ssp: *mut srcu_struct, head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    pub fn cleanup_srcu_struct(ssp: *mut srcu_struct);
    pub fn synchronize_srcu(ssp: *mut srcu_struct);
    pub fn get_state_synchronize_srcu(ssp: *mut srcu_struct) -> usize;
    pub fn start_poll_synchronize_srcu(ssp: *mut srcu_struct) -> usize;
    pub fn poll_state_synchronize_srcu(ssp: *mut srcu_struct, cookie: usize) -> bool;
    pub fn srcu_init();
    pub fn debug_lockdep_rcu_enabled() -> bool;
    pub fn lock_is_held(map: *const lockdep_map) -> i32;
}

#[inline]
pub unsafe fn __init_srcu_struct(ssp: *mut srcu_struct, name: *const i8,
                                  key: *mut lock_class_key) -> i32 {
    // CONFIG_DEBUG_LOCK_ALLOC selects the lockdep initializer; otherwise the
    // generic initializer ignores name and key.
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    { init_srcu_struct_lockdep(ssp, name, key) }
    #[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
    { let _ = (name, key); init_srcu_struct_generic(ssp) }
}

#[inline]
pub unsafe fn init_srcu_struct(ssp: *mut srcu_struct, name: *const i8,
                               key: *mut lock_class_key) -> i32 {
    __init_srcu_struct(ssp, name, key)
}

#[inline]
pub unsafe fn srcu_read_lock_held(ssp: *const srcu_struct) -> i32 {
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    {
        if !debug_lockdep_rcu_enabled() { return 1; }
        // dep_map is an external field supplied by the translated struct.
        return lock_is_held(ssp as *const lockdep_map);
    }
    #[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
    { let _ = ssp; 1 }
}

#[inline]
pub unsafe fn __srcu_read_lock_must_hold(_ssp: *const srcu_struct) {}

// srcu_dereference_check(), srcu_dereference(), and
// srcu_dereference_notrace() are expression macros around the external
// __rcu_dereference_check() and lock-context annotations.  Their dependency
// symbols are intentionally left external, as in the C header.

#[inline]
pub const fn get_completed_synchronize_srcu() -> usize { SRCU_GET_STATE_COMPLETED }

#[inline]
pub const fn same_state_synchronize_srcu(oldstate1: usize, oldstate2: usize) -> bool {
    oldstate1 == oldstate2
}

#[inline]
pub unsafe fn srcu_read_lock(ssp: *mut srcu_struct) -> i32 {
    __srcu_read_lock(ssp)
}
#[inline]
pub unsafe fn srcu_read_lock_fast(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_read_lock_fast(ssp)
}
#[inline]
pub unsafe fn srcu_read_lock_fast_updown(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_read_lock_fast_updown(ssp)
}
#[inline]
pub unsafe fn srcu_read_lock_fast_notrace(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_read_lock_fast(ssp)
}
#[inline]
pub unsafe fn srcu_down_read_fast(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_read_lock_fast_updown(ssp)
}
#[inline]
pub unsafe fn srcu_read_lock_nmisafe(ssp: *mut srcu_struct) -> i32 {
    __srcu_read_lock_nmisafe(ssp)
}
#[inline]
pub unsafe fn srcu_read_lock_notrace(ssp: *mut srcu_struct) -> i32 {
    __srcu_read_lock(ssp)
}
#[inline]
pub unsafe fn srcu_down_read(ssp: *mut srcu_struct) -> i32 { __srcu_read_lock(ssp) }

#[inline]
pub unsafe fn srcu_read_unlock(ssp: *mut srcu_struct, idx: i32) { __srcu_read_unlock(ssp, idx) }
#[inline]
pub unsafe fn srcu_read_unlock_fast(ssp: *mut srcu_struct, scp: *mut srcu_ctr) { __srcu_read_unlock_fast(ssp, scp) }
#[inline]
pub unsafe fn srcu_read_unlock_fast_updown(ssp: *mut srcu_struct, scp: *mut srcu_ctr) { __srcu_read_unlock_fast_updown(ssp, scp) }
#[inline]
pub unsafe fn srcu_read_unlock_fast_notrace(ssp: *mut srcu_struct, scp: *mut srcu_ctr) { __srcu_read_unlock_fast(ssp, scp) }
#[inline]
pub unsafe fn srcu_up_read_fast(ssp: *mut srcu_struct, scp: *mut srcu_ctr) { __srcu_read_unlock_fast_updown(ssp, scp) }
#[inline]
pub unsafe fn srcu_read_unlock_nmisafe(ssp: *mut srcu_struct, idx: i32) { __srcu_read_unlock_nmisafe(ssp, idx) }
#[inline]
pub unsafe fn srcu_read_unlock_notrace(ssp: *mut srcu_struct, idx: i32) { __srcu_read_unlock(ssp, idx) }
#[inline]
pub unsafe fn srcu_up_read(ssp: *mut srcu_struct, idx: i32) { __srcu_read_unlock(ssp, idx) }

#[inline] pub fn smp_mb__after_srcu_read_unlock() {}
#[inline] pub fn smp_mb__after_srcu_read_lock() {}

// The following C lock-guard and lockdep annotation macros have no direct
// file-local Rust representation; their operational read-lock/unlock bodies
// are provided above and retain the same external names and pointer behavior.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

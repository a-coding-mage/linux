// SPDX-License-Identifier: GPL-2.0-only

/*
 * rcuref - A scalable reference count implementation for RCU managed objects
 *
 * This file is a direct Rust translation of rcuref.c. Kernel-provided types,
 * constants, and primitives are declared below as external dependencies.
 */

use core::ffi::{c_char, c_uint, c_void};

#[repr(C)]
pub struct rcuref_t {
    pub refcnt: c_uint,
}

// Values supplied by <linux/rcuref.h>.
pub const RCUREF_ONEREF: c_uint = 0x0000_0001;
pub const RCUREF_MAXREF: c_uint = 0x7fff_ffff;
pub const RCUREF_SATURATED: c_uint = 0xa000_0000;
pub const RCUREF_RELEASED: c_uint = 0xc000_0000;
pub const RCUREF_DEAD: c_uint = 0xe000_0000;
pub const RCUREF_NOREF: c_uint = 0xffff_ffff;

extern "C" {
    fn atomic_read(v: *const c_uint) -> c_uint;
    fn atomic_set(v: *mut c_uint, value: c_uint);
    fn atomic_add_negative(v: c_int, i: *mut c_uint) -> bool;
    fn atomic_try_cmpxchg_release(
        v: *mut c_uint,
        old: *mut c_uint,
        new: c_uint,
    ) -> bool;
    fn smp_acquire__after_ctrl_dep();
    fn warn_once(condition: bool, fmt: *const c_char) -> bool;
}

type c_int = i32;

/// rcuref_get_slowpath - Slowpath of rcuref_get()
/// @ref: Pointer to the reference count
pub unsafe extern "C" fn rcuref_get_slowpath(ref_: *mut rcuref_t) -> bool {
    let cnt = atomic_read(core::ptr::addr_of!((*ref_).refcnt));

    /*
     * If the reference count was already marked dead, undo the
     * increment so it stays in the middle of the dead zone and return
     * fail.
     */
    if cnt >= RCUREF_RELEASED {
        atomic_set(core::ptr::addr_of_mut!((*ref_).refcnt), RCUREF_DEAD);
        return false;
    }

    /*
     * If it was saturated, warn and mark it so. In case the increment
     * was already on a saturated value restore the saturation
     * marker. This keeps it in the middle of the saturation zone and
     * prevents the reference count from overflowing. This leaks the
     * object memory, but prevents the obvious reference count overflow
     * damage.
     */
    if warn_once(cnt > RCUREF_MAXREF, c"rcuref saturated - leaking memory\0".as_ptr()) {
        atomic_set(core::ptr::addr_of_mut!((*ref_).refcnt), RCUREF_SATURATED);
    }
    true
}

/// rcuref_put_slowpath - Slowpath of __rcuref_put()
/// @ref: Pointer to the reference count
/// @cnt: The resulting value of the fastpath decrement
pub unsafe extern "C" fn rcuref_put_slowpath(ref_: *mut rcuref_t, mut cnt: c_uint) -> bool {
    /* Did this drop the last reference? */
    if cnt == RCUREF_NOREF {
        /*
         * Carefully try to set the reference count to RCUREF_DEAD.
         *
         * This can fail if a concurrent get() operation has
         * elevated it again or the corresponding put() even marked
         * it dead already. Both are valid situations and do not
         * require a retry. If this fails the caller is not
         * allowed to deconstruct the object.
         */
        if !atomic_try_cmpxchg_release(
            core::ptr::addr_of_mut!((*ref_).refcnt),
            &mut cnt,
            RCUREF_DEAD,
        ) {
            return false;
        }

        /* The caller can safely schedule the object for deconstruction. */
        smp_acquire__after_ctrl_dep();
        return true;
    }

    /*
     * If the reference count was already in the dead zone, then this
     * put() operation is imbalanced. Warn, put the reference count back to
     * DEAD and tell the caller to not deconstruct the object.
     */
    if warn_once(cnt >= RCUREF_RELEASED, c"rcuref - imbalanced put()\0".as_ptr()) {
        atomic_set(core::ptr::addr_of_mut!((*ref_).refcnt), RCUREF_DEAD);
        return false;
    }

    /* Restore the mean saturation value for a saturated refcount. */
    if cnt > RCUREF_MAXREF {
        atomic_set(core::ptr::addr_of_mut!((*ref_).refcnt), RCUREF_SATURATED);
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

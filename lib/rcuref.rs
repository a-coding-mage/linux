// SPDX-License-Identifier: GPL-2.0-only

/*
 * rcuref - A scalable reference count implementation for RCU managed objects
 *
 * rcuref is provided to replace open coded reference count implementations
 * based on atomic_t. It protects explicitely RCU managed objects which can
 * be visible even after the last reference has been dropped and the object
 * is heading towards destruction.
 *
 * The documentation of the algorithm, its states and the fast paths is kept
 * with the original C unit, lib/rcuref.c, and linux/rcuref.h.
 */

// Dependencies: linux/rcuref.h.

use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use kernel::bindings::{atomic_t, rcuref_t, smp_rmb};
use kernel::ffi::c_int;

extern "C" {
    fn lupos_rcuref_try_cmpxchg_release(v: *mut atomic_t, old: *mut c_int, new: c_int) -> bool;
    fn lupos_rcuref_warn_saturated();
    fn lupos_rcuref_warn_imbalanced_put();
}

/// RCUREF_* zone markers from linux/rcuref.h; lib/rcuref_helpers.c asserts
/// them.
const RCUREF_MAXREF: u32 = 0x7FFF_FFFF;
const RCUREF_SATURATED: u32 = 0xA000_0000;
const RCUREF_RELEASED: u32 = 0xC000_0000;
const RCUREF_DEAD: u32 = 0xE000_0000;
const RCUREF_NOREF: u32 = 0xFFFF_FFFF;

/// atomic_set(&ref->refcnt, value).
///
/// # Safety
/// `r` points to a valid rcuref.
unsafe fn refcnt_set(r: *mut rcuref_t, value: u32) {
    // SAFETY: atomic_set() is a volatile store of the counter.
    unsafe { write_volatile(addr_of_mut!((*r).refcnt.counter), value as c_int) };
}

/**
 * rcuref_get_slowpath - Slowpath of rcuref_get()
 * @ref:	Pointer to the reference count
 *
 * Invoked when the reference count is outside of the valid zone.
 *
 * Return:
 *	False if the reference count was already marked dead
 *
 *	True if the reference count is saturated, which prevents the
 *	object from being deconstructed ever.
 */
/// # Safety
/// `r` points to a valid rcuref.
#[no_mangle]
pub unsafe extern "C" fn rcuref_get_slowpath(r: *mut rcuref_t) -> bool {
    // SAFETY: r is valid; atomic_read() is a volatile load.
    unsafe {
        let cnt = read_volatile(addr_of_mut!((*r).refcnt.counter)) as u32;

        /*
         * If the reference count was already marked dead, undo the
         * increment so it stays in the middle of the dead zone and return
         * fail.
         */
        if cnt >= RCUREF_RELEASED {
            refcnt_set(r, RCUREF_DEAD);
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
        if cnt > RCUREF_MAXREF {
            lupos_rcuref_warn_saturated();
            refcnt_set(r, RCUREF_SATURATED);
        }
    }
    true
}
// EXPORT_SYMBOL_GPL(rcuref_get_slowpath);

/**
 * rcuref_put_slowpath - Slowpath of __rcuref_put()
 * @ref:	Pointer to the reference count
 * @cnt:	The resulting value of the fastpath decrement
 *
 * Invoked when the reference count is outside of the valid zone.
 *
 * Return:
 *	True if this was the last reference with no future references
 *	possible. This signals the caller that it can safely schedule the
 *	object, which is protected by the reference counter, for
 *	deconstruction.
 *
 *	False if there are still active references or the put() raced
 *	with a concurrent get()/put() pair. Caller is not allowed to
 *	deconstruct the protected object.
 */
/// # Safety
/// `r` points to a valid rcuref.
#[no_mangle]
pub unsafe extern "C" fn rcuref_put_slowpath(r: *mut rcuref_t, cnt: u32) -> bool {
    // SAFETY: r is valid; the DEAD transition uses the native release
    // cmpxchg and smp_acquire__after_ctrl_dep() (smp_rmb()).
    unsafe {
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
            let mut old = cnt as c_int;
            if !lupos_rcuref_try_cmpxchg_release(addr_of_mut!((*r).refcnt), &mut old, RCUREF_DEAD as c_int) {
                return false;
            }

            /*
             * The caller can safely schedule the object for
             * deconstruction. Provide acquire ordering.
             */
            smp_rmb();
            return true;
        }

        /*
         * If the reference count was already in the dead zone, then this
         * put() operation is imbalanced. Warn, put the reference count back to
         * DEAD and tell the caller to not deconstruct the object.
         */
        if cnt >= RCUREF_RELEASED {
            lupos_rcuref_warn_imbalanced_put();
            refcnt_set(r, RCUREF_DEAD);
            return false;
        }

        /*
         * This is a put() operation on a saturated refcount. Restore the
         * mean saturation value and tell the caller to not deconstruct the
         * object.
         */
        if cnt > RCUREF_MAXREF {
            refcnt_set(r, RCUREF_SATURATED);
        }
    }
    false
}
// EXPORT_SYMBOL_GPL(rcuref_put_slowpath);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

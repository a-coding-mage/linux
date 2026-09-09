/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MCS lock defines
 *
 * This file contains the main data structure and API definitions of MCS lock.
 *
 * The MCS lock (proposed by Mellor-Crummey and Scott) is a simple spin-lock
 * with the desirable properties of being fair, and with each cpu trying to
 * acquire the lock spinning on a local variable.
 * It avoids expensive cache bounces that common test-and-set spin-lock
 * implementations incur.
 */

// Architecture-specific MCS lock support is supplied by the corresponding
// dependency, as in <asm/mcs_spinlock.h>.

#[repr(C)]
pub struct mcs_spinlock {
    pub next: *mut mcs_spinlock,
    pub locked: i32,
}

/*
 * Note: the smp_load_acquire/smp_store_release pair is not
 * sufficient to form a full memory barrier across
 * cpus for many architectures (except x86) for mcs_unlock and mcs_lock.
 * For applications that need a full barrier across multiple cpus
 * with mcs_unlock and mcs_lock pair, smp_mb__after_unlock_lock() should be
 * used after mcs_lock.
 */

extern "C" {
    fn xchg(lock: *mut *mut mcs_spinlock, node: *mut mcs_spinlock) -> *mut mcs_spinlock;
    fn cmpxchg_release(
        lock: *mut *mut mcs_spinlock,
        old: *mut mcs_spinlock,
        new: *mut mcs_spinlock,
    ) -> *mut mcs_spinlock;
    fn cpu_relax();
    fn smp_cond_load_acquire(ptr: *mut i32, val: i32);
    fn smp_store_release(ptr: *mut i32, val: i32);
}

#[inline]
pub unsafe fn mcs_spin_lock(lock: *mut *mut mcs_spinlock, node: *mut mcs_spinlock) {
    let prev: *mut mcs_spinlock;

    /* Init node */
    (*node).locked = 0;
    (*node).next = core::ptr::null_mut();

    /*
     * We rely on the full barrier with global transitivity implied by the
     * below xchg() to order the initialization stores above against any
     * observation of @node. And to provide the ACQUIRE ordering associated
     * with a LOCK primitive.
     */
    prev = xchg(lock, node);
    if prev.is_null() {
        /* Lock acquired; this thread does not spin on its own node. */
        return;
    }
    core::ptr::write_volatile(&mut (*prev).next, node);

    /* Wait until the lock holder passes the lock down. */
    smp_cond_load_acquire(&mut (*node).locked, 1);
}

/*
 * Releases the lock. The caller should pass in the corresponding node that
 * was used to acquire the lock.
 */
#[inline]
pub unsafe fn mcs_spin_unlock(lock: *mut *mut mcs_spinlock, node: *mut mcs_spinlock) {
    let mut next: *mut mcs_spinlock = core::ptr::read_volatile(&(*node).next);

    if next.is_null() {
        /* Release the lock by setting it to NULL. */
        if cmpxchg_release(lock, node, core::ptr::null_mut()) == node {
            return;
        }
        /* Wait until the next pointer is set. */
        while {
            next = core::ptr::read_volatile(&(*node).next);
            next.is_null()
        } {
            cpu_relax();
        }
    }

    /* Pass lock to next waiter. */
    smp_store_release(&mut (*next).locked, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

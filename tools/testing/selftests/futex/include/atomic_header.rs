/* SPDX-License-Identifier: GPL-2.0-or-later */
/******************************************************************************
 *
 *   Copyright (c) International Business Machines  Corp., 2009
 *
 * DESCRIPTION
 *      GCC atomic builtin wrappers
 *      http://gcc.gnu.org/onlinedocs/gcc-4.1.0/gcc/Atomic-Builtins.html
 *
 * AUTHOR
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2009-Nov-17: Initial version by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

use core::ffi::c_int;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

#[repr(C)]
pub struct atomic_t {
    pub val: c_int,
}

pub const ATOMIC_INITIALIZER: atomic_t = atomic_t { val: 0 };

/**
 * atomic_cmpxchg() - Atomic compare and exchange
 * @uaddr:	The address of the futex to be modified
 * @oldval:	The expected value of the futex
 * @newval:	The new value to try and assign the futex
 *
 * Return the old value of addr->val.
 */
#[inline]
pub unsafe fn atomic_cmpxchg(addr: *mut atomic_t, oldval: c_int, newval: c_int) -> c_int {
    let val = unsafe { &*ptr::addr_of_mut!((*addr).val).cast::<AtomicI32>() };

    match val.compare_exchange(
        oldval as i32,
        newval as i32,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(previous) | Err(previous) => previous as c_int,
    }
}

/**
 * atomic_inc() - Atomic incrememnt
 * @addr:	Address of the variable to increment
 *
 * Return the new value of addr->val.
 */
#[inline]
pub unsafe fn atomic_inc(addr: *mut atomic_t) -> c_int {
    let val = unsafe { &*ptr::addr_of_mut!((*addr).val).cast::<AtomicI32>() };

    val.fetch_add(1, Ordering::SeqCst).wrapping_add(1) as c_int
}

/**
 * atomic_dec() - Atomic decrement
 * @addr:	Address of the variable to decrement
 *
 * Return the new value of addr-val.
 */
#[inline]
pub unsafe fn atomic_dec(addr: *mut atomic_t) -> c_int {
    let val = unsafe { &*ptr::addr_of_mut!((*addr).val).cast::<AtomicI32>() };

    val.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1) as c_int
}

/**
 * atomic_set() - Atomic set
 * @addr:	Address of the variable to set
 * @newval:	New value for the atomic_t
 *
 * Return the new value of addr->val.
 */
#[inline]
pub unsafe fn atomic_set(addr: *mut atomic_t, newval: c_int) -> c_int {
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!((*addr).val), newval);
    }

    newval
}

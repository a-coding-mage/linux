/* SPDX-License-Identifier: GPL-2.0-or-later */
/******************************************************************************
 *
 *   Copyright (c) International Business Machines  Corp., 2009
 *
 * DESCRIPTION
 *      Glibc independent futex library for testing kernel functionality.
 *
 * AUTHOR
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2009-Nov-6: Initial version by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

/* C dependencies removed from executable Rust:
 *   <unistd.h>, <sys/syscall.h>, <sys/types.h>, <linux/futex.h>
 */

use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use std::os::raw::{c_int, c_long};

pub type u_int32_t = u32;
pub type futex_t = u_int32_t;
pub const FUTEX_INITIALIZER: futex_t = 0;

/*
 * Opaque declaration for struct timespec supplied by the system C library.
 */
#[repr(C)]
pub struct timespec {
    _private: [u8; 0],
}

/*
 * Futex constants are supplied by <linux/futex.h> in C. The newer op codes are
 * defined here as fallbacks when the system header file is not up to date.
 */
pub const FUTEX_WAIT_BITSET: c_int = 9;
pub const FUTEX_WAKE_BITSET: c_int = 10;
pub const FUTEX_WAIT_REQUEUE_PI: c_int = 11;
pub const FUTEX_CMP_REQUEUE_PI: c_int = 12;
pub const FUTEX_ROBUST_UNLOCK: c_int = 512;
pub const FUTEX_ROBUST_LIST32: c_int = 1024;

/*
 * Expected external constants from <linux/futex.h> and <sys/syscall.h>.
 */
extern "C" {
    pub static SYS_futex: c_long;
    pub static FUTEX_WAIT: c_int;
    pub static FUTEX_WAKE: c_int;
    pub static FUTEX_PRIVATE_FLAG: c_int;
    pub static FUTEX_LOCK_PI: c_int;
    pub static FUTEX_UNLOCK_PI: c_int;
    pub static FUTEX_WAKE_OP: c_int;
    pub static FUTEX_REQUEUE: c_int;
    pub static FUTEX_CMP_REQUEUE: c_int;
}

/*
 * C preprocessor fallback:
 *
 *   #ifndef FUTEX_WAIT_REQUEUE_PI_PRIVATE
 *   #define FUTEX_WAIT_REQUEUE_PI_PRIVATE \
 *           (FUTEX_WAIT_REQUEUE_PI | FUTEX_PRIVATE_FLAG)
 *   #endif
 *
 *   #ifndef FUTEX_REQUEUE_PI_PRIVATE
 *   #define FUTEX_CMP_REQUEUE_PI_PRIVATE \
 *           (FUTEX_CMP_REQUEUE_PI | FUTEX_PRIVATE_FLAG)
 *   #endif
 *
 * This cannot be a Rust const while FUTEX_PRIVATE_FLAG is an external C static.
 */

/*
 * SYS_futex is expected from system C library, in glibc some 32-bit
 * architectures (e.g. RV32) are using 64-bit time_t, therefore it doesn't have
 * SYS_futex defined but just SYS_futex_time64. Define SYS_futex as
 * SYS_futex_time64 in this situation to ensure the compilation and the
 * compatibility.
 *
 * On 32bit systems if we use "-D_FILE_OFFSET_BITS=64 -D_TIME_BITS=64" or if
 * we are using a newer compiler then the size of the timestamps will be 64bit,
 * however, the SYS_futex will still point to the 32bit futex system call.
 *
 * These build-time C preprocessor conditions are preserved as dependency
 * intent; the actual syscall number is expected from the target environment.
 */

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

/**
 * futex() - SYS_futex syscall wrapper
 * @uaddr:	address of first futex
 * @op:		futex op code
 * @val:	typically expected value of uaddr, but varies by op
 * @timeout:	typically an absolute struct timespec (except where noted
 *              otherwise). Overloaded by some ops
 * @uaddr2:	address of second futex for some ops\
 * @val3:	varies by op
 * @opflags:	flags to be bitwise OR'd with op, such as FUTEX_PRIVATE_FLAG
 *
 * futex() is used by all the following futex op wrappers. It can also be
 * used for misuse and abuse testing. Generally, the specific op wrappers
 * should be used instead. It is a macro instead of an static inline function as
 * some of the types over overloaded (timeout is used for nr_requeue for
 * example).
 *
 * These argument descriptions are the defaults for all
 * like-named arguments in the following wrappers except where noted below.
 */
pub unsafe fn futex(
    uaddr: *mut futex_t,
    op: c_int,
    val: c_int,
    timeout: *mut c_void,
    uaddr2: *mut futex_t,
    val3: c_int,
    opflags: c_int,
) -> c_int {
    syscall(
        SYS_futex,
        uaddr,
        op | opflags,
        val,
        timeout,
        uaddr2,
        val3,
    ) as c_int
}

/**
 * futex_wait() - block on uaddr with optional timeout
 * @timeout:	relative timeout
 */
pub unsafe fn futex_wait(
    uaddr: *mut futex_t,
    val: futex_t,
    timeout: *mut timespec,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_WAIT,
        val as c_int,
        timeout as *mut c_void,
        ptr::null_mut(),
        0,
        opflags,
    )
}

/**
 * futex_wake() - wake one or more tasks blocked on uaddr
 * @nr_wake:	wake up to this many tasks
 */
pub unsafe fn futex_wake(uaddr: *mut futex_t, nr_wake: c_int, opflags: c_int) -> c_int {
    futex(
        uaddr,
        FUTEX_WAKE,
        nr_wake,
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        opflags,
    )
}

/**
 * futex_wait_bitset() - block on uaddr with bitset
 * @bitset:	bitset to be used with futex_wake_bitset
 */
pub unsafe fn futex_wait_bitset(
    uaddr: *mut futex_t,
    val: futex_t,
    timeout: *mut timespec,
    bitset: u_int32_t,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_WAIT_BITSET,
        val as c_int,
        timeout as *mut c_void,
        ptr::null_mut(),
        bitset as c_int,
        opflags,
    )
}

/**
 * futex_wake_bitset() - wake one or more tasks blocked on uaddr with bitset
 * @bitset:	bitset to compare with that used in futex_wait_bitset
 */
pub unsafe fn futex_wake_bitset(
    uaddr: *mut futex_t,
    nr_wake: c_int,
    bitset: u_int32_t,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_WAKE_BITSET,
        nr_wake,
        ptr::null_mut(),
        ptr::null_mut(),
        bitset as c_int,
        opflags,
    )
}

/**
 * futex_lock_pi() - block on uaddr as a PI mutex
 * @detect:	whether (1) or not (0) to perform deadlock detection
 */
pub unsafe fn futex_lock_pi(
    uaddr: *mut futex_t,
    timeout: *mut timespec,
    detect: c_int,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_LOCK_PI,
        detect,
        timeout as *mut c_void,
        ptr::null_mut(),
        0,
        opflags,
    )
}

/**
 * futex_unlock_pi() - release uaddr as a PI mutex, waking the top waiter
 */
pub unsafe fn futex_unlock_pi(uaddr: *mut futex_t, opflags: c_int) -> c_int {
    futex(
        uaddr,
        FUTEX_UNLOCK_PI,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        opflags,
    )
}

/**
 * futex_wake_op() - FIXME: COME UP WITH A GOOD ONE LINE DESCRIPTION
 */
pub unsafe fn futex_wake_op(
    uaddr: *mut futex_t,
    uaddr2: *mut futex_t,
    nr_wake: c_int,
    nr_wake2: c_int,
    wake_op: c_int,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_WAKE_OP,
        nr_wake,
        nr_wake2 as *mut c_void,
        uaddr2,
        wake_op,
        opflags,
    )
}

/**
 * futex_requeue() - requeue without expected value comparison, deprecated
 * @nr_wake:	wake up to this many tasks
 * @nr_requeue:	requeue up to this many tasks
 *
 * Due to its inherently racy implementation, futex_requeue() is deprecated in
 * favor of futex_cmp_requeue().
 */
pub unsafe fn futex_requeue(
    uaddr: *mut futex_t,
    uaddr2: *mut futex_t,
    nr_wake: c_int,
    nr_requeue: c_int,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_REQUEUE,
        nr_wake,
        nr_requeue as *mut c_void,
        uaddr2,
        0,
        opflags,
    )
}

/**
 * futex_cmp_requeue() - requeue tasks from uaddr to uaddr2
 * @nr_wake:	wake up to this many tasks
 * @nr_requeue:	requeue up to this many tasks
 */
pub unsafe fn futex_cmp_requeue(
    uaddr: *mut futex_t,
    val: futex_t,
    uaddr2: *mut futex_t,
    nr_wake: c_int,
    nr_requeue: c_int,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_CMP_REQUEUE,
        nr_wake,
        nr_requeue as *mut c_void,
        uaddr2,
        val as c_int,
        opflags,
    )
}

/**
 * futex_wait_requeue_pi() - block on uaddr and prepare to requeue to uaddr2
 * @uaddr:	non-PI futex source
 * @uaddr2:	PI futex target
 *
 * This is the first half of the requeue_pi mechanism. It shall always be
 * paired with futex_cmp_requeue_pi().
 */
pub unsafe fn futex_wait_requeue_pi(
    uaddr: *mut futex_t,
    val: futex_t,
    uaddr2: *mut futex_t,
    timeout: *mut timespec,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_WAIT_REQUEUE_PI,
        val as c_int,
        timeout as *mut c_void,
        uaddr2,
        0,
        opflags,
    )
}

/**
 * futex_cmp_requeue_pi() - requeue tasks from uaddr to uaddr2 (PI aware)
 * @uaddr:	non-PI futex source
 * @uaddr2:	PI futex target
 * @nr_wake:	wake up to this many tasks
 * @nr_requeue:	requeue up to this many tasks
 */
pub unsafe fn futex_cmp_requeue_pi(
    uaddr: *mut futex_t,
    val: futex_t,
    uaddr2: *mut futex_t,
    nr_wake: c_int,
    nr_requeue: c_int,
    opflags: c_int,
) -> c_int {
    futex(
        uaddr,
        FUTEX_CMP_REQUEUE_PI,
        nr_wake,
        nr_requeue as *mut c_void,
        uaddr2,
        val as c_int,
        opflags,
    )
}

/**
 * futex_cmpxchg() - atomic compare and exchange
 * @uaddr:	The address of the futex to be modified
 * @oldval:	The expected value of the futex
 * @newval:	The new value to try and assign the futex
 *
 * Implement cmpxchg using gcc atomic builtins.
 * http://gcc.gnu.org/onlinedocs/gcc-4.1.0/gcc/Atomic-Builtins.html
 *
 * Return the old futex value.
 */
pub unsafe fn futex_cmpxchg(uaddr: *mut futex_t, oldval: u_int32_t, newval: u_int32_t) -> u_int32_t {
    let atomic = &*(uaddr as *const AtomicU32);
    match atomic.compare_exchange(oldval, newval, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(previous) => previous,
        Err(previous) => previous,
    }
}

/**
 * futex_dec() - atomic decrement of the futex value
 * @uaddr:	The address of the futex to be modified
 *
 * Return the new futex value.
 */
pub unsafe fn futex_dec(uaddr: *mut futex_t) -> u_int32_t {
    let atomic = &*(uaddr as *const AtomicU32);
    atomic.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/**
 * futex_inc() - atomic increment of the futex value
 * @uaddr:	the address of the futex to be modified
 *
 * Return the new futex value.
 */
pub unsafe fn futex_inc(uaddr: *mut futex_t) -> u_int32_t {
    let atomic = &*(uaddr as *const AtomicU32);
    atomic.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/**
 * futex_set() - atomic decrement of the futex value
 * @uaddr:	the address of the futex to be modified
 * @newval:	New value for the atomic_t
 *
 * Return the new futex value.
 */
pub unsafe fn futex_set(uaddr: *mut futex_t, newval: u_int32_t) -> u_int32_t {
    ptr::write_volatile(uaddr, newval);
    newval
}

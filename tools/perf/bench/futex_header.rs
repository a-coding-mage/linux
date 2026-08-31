/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Glibc independent futex library for testing kernel functionality.
 * Shamelessly stolen from Darren Hart <dvhltc@us.ibm.com>
 *    http://git.kernel.org/cgit/linux/kernel/git/dvhart/futextest.git/
 */

use std::ffi::{c_int, c_long};
use std::ptr;

pub type u_int32_t = u32;

// C dependencies from <sys/syscall.h>, <linux/futex.h>, and struct timespec.
unsafe extern "C" {
    pub static SYS_futex: c_long;
    pub static FUTEX_WAIT: c_int;
    pub static FUTEX_WAKE: c_int;
    pub static FUTEX_LOCK_PI: c_int;
    pub static FUTEX_UNLOCK_PI: c_int;
    pub static FUTEX_CMP_REQUEUE: c_int;
    pub static FUTEX_WAIT_REQUEUE_PI: c_int;
    pub static FUTEX_CMP_REQUEUE_PI: c_int;

    pub fn syscall(num: c_long, ...) -> c_long;
}

#[repr(C)]
pub struct timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench_futex_parameters {
    pub silent: bool,
    pub fshared: bool,
    pub mlockall: bool,
    pub multi: bool,     /* lock-pi */
    pub pi: bool,        /* requeue-pi */
    pub broadcast: bool, /* requeue */
    pub runtime: u32,    /* seconds*/
    pub nthreads: u32,
    pub nfutexes: u32,
    pub nwakes: u32,
    pub nrequeue: u32,
    pub nbuckets: c_int,
}

/**
 * futex_syscall() - SYS_futex syscall wrapper
 * @uaddr:	address of first futex
 * @op:		futex op code
 * @val:	typically expected value of uaddr, but varies by op
 * @timeout:	typically an absolute struct timespec (except where noted
 *		otherwise). Overloaded by some ops
 * @uaddr2:	address of second futex for some ops
 * @val3:	varies by op
 * @opflags:	flags to be bitwise OR'd with op, such as FUTEX_PRIVATE_FLAG
 *
 * futex_syscall() is used by all the following futex op wrappers. It can also be
 * used for misuse and abuse testing. Generally, the specific op wrappers
 * should be used instead.
 *
 * These argument descriptions are the defaults for all
 * like-named arguments in the following wrappers except where noted below.
 */
pub unsafe fn futex_syscall(
    uaddr: *mut u_int32_t,
    op: c_int,
    val: u_int32_t,
    timeout: *mut timespec,
    uaddr2: *mut u_int32_t,
    val3: c_int,
    opflags: c_int,
) -> c_int {
    unsafe { syscall(SYS_futex, uaddr, op | opflags, val, timeout, uaddr2, val3) as c_int }
}

pub unsafe fn futex_syscall_nr_requeue(
    uaddr: *mut u_int32_t,
    op: c_int,
    val: u_int32_t,
    nr_requeue: c_int,
    uaddr2: *mut u_int32_t,
    val3: c_int,
    opflags: c_int,
) -> c_int {
    unsafe {
        syscall(
            SYS_futex,
            uaddr,
            op | opflags,
            val,
            nr_requeue,
            uaddr2,
            val3,
        ) as c_int
    }
}

/**
 * futex_wait() - block on uaddr with optional timeout
 * @timeout:	relative timeout
 */
pub unsafe fn futex_wait(
    uaddr: *mut u_int32_t,
    val: u_int32_t,
    timeout: *mut timespec,
    opflags: c_int,
) -> c_int {
    unsafe { futex_syscall(uaddr, FUTEX_WAIT, val, timeout, ptr::null_mut(), 0, opflags) }
}

/**
 * futex_wake() - wake one or more tasks blocked on uaddr
 * @nr_wake:	wake up to this many tasks
 */
pub unsafe fn futex_wake(uaddr: *mut u_int32_t, nr_wake: c_int, opflags: c_int) -> c_int {
    unsafe {
        futex_syscall(
            uaddr,
            FUTEX_WAKE,
            nr_wake as u_int32_t,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            opflags,
        )
    }
}

/**
 * futex_lock_pi() - block on uaddr as a PI mutex
 */
pub unsafe fn futex_lock_pi(
    uaddr: *mut u_int32_t,
    timeout: *mut timespec,
    opflags: c_int,
) -> c_int {
    unsafe { futex_syscall(uaddr, FUTEX_LOCK_PI, 0, timeout, ptr::null_mut(), 0, opflags) }
}

/**
 * futex_unlock_pi() - release uaddr as a PI mutex, waking the top waiter
 */
pub unsafe fn futex_unlock_pi(uaddr: *mut u_int32_t, opflags: c_int) -> c_int {
    unsafe {
        futex_syscall(
            uaddr,
            FUTEX_UNLOCK_PI,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            opflags,
        )
    }
}

/**
* futex_cmp_requeue() - requeue tasks from uaddr to uaddr2
* @nr_wake:        wake up to this many tasks
* @nr_requeue:     requeue up to this many tasks
*/
pub unsafe fn futex_cmp_requeue(
    uaddr: *mut u_int32_t,
    val: u_int32_t,
    uaddr2: *mut u_int32_t,
    nr_wake: c_int,
    nr_requeue: c_int,
    opflags: c_int,
) -> c_int {
    unsafe {
        futex_syscall_nr_requeue(
            uaddr,
            FUTEX_CMP_REQUEUE,
            nr_wake as u_int32_t,
            nr_requeue,
            uaddr2,
            val as c_int,
            opflags,
        )
    }
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
    uaddr: *mut u_int32_t,
    val: u_int32_t,
    uaddr2: *mut u_int32_t,
    timeout: *mut timespec,
    opflags: c_int,
) -> c_int {
    unsafe {
        futex_syscall(
            uaddr,
            FUTEX_WAIT_REQUEUE_PI,
            val,
            timeout,
            uaddr2,
            0,
            opflags,
        )
    }
}

/**
 * futex_cmp_requeue_pi() - requeue tasks from uaddr to uaddr2
 * @uaddr:	non-PI futex source
 * @uaddr2:	PI futex target
 * @nr_requeue:	requeue up to this many tasks
 *
 * This is the second half of the requeue_pi mechanism. It shall always be
 * paired with futex_wait_requeue_pi(). The first waker is always awoken.
 */
pub unsafe fn futex_cmp_requeue_pi(
    uaddr: *mut u_int32_t,
    val: u_int32_t,
    uaddr2: *mut u_int32_t,
    nr_requeue: c_int,
    opflags: c_int,
) -> c_int {
    unsafe {
        futex_syscall_nr_requeue(
            uaddr,
            FUTEX_CMP_REQUEUE_PI,
            1,
            nr_requeue,
            uaddr2,
            val as c_int,
            opflags,
        )
    }
}

unsafe extern "C" {
    pub fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);
    pub fn futex_print_nbuckets(params: *mut bench_futex_parameters);
}

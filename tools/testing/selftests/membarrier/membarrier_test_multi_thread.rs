// SPDX-License-Identifier: GPL-2.0
// C dependency intent: _GNU_SOURCE, <linux/membarrier.h>, <syscall.h>,
// <stdio.h>, <errno.h>, <string.h>, <pthread.h>, and
// "membarrier_test_impl.h".

use core::ffi::c_void;

static mut thread_ready: libc::c_int = 0;
static mut thread_quit: libc::c_int = 0;
static mut test_membarrier_thread_mutex: libc::pthread_mutex_t =
    libc::PTHREAD_MUTEX_INITIALIZER;
static mut test_membarrier_thread_cond: libc::pthread_cond_t =
    libc::PTHREAD_COND_INITIALIZER;

extern "C" {
    fn test_membarrier_fail();
    fn test_membarrier_success();
    fn test_membarrier_query();
    fn ksft_print_header();
    fn ksft_set_plan(plan: libc::c_uint);
    fn ksft_exit_pass() -> !;
}

pub unsafe extern "C" fn test_membarrier_thread(arg: *mut c_void) -> *mut c_void {
    let _ = arg;

    libc::pthread_mutex_lock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));
    thread_ready = 1;
    libc::pthread_cond_broadcast(core::ptr::addr_of_mut!(test_membarrier_thread_cond));
    libc::pthread_mutex_unlock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));

    libc::pthread_mutex_lock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));
    while thread_quit == 0 {
        libc::pthread_cond_wait(
            core::ptr::addr_of_mut!(test_membarrier_thread_cond),
            core::ptr::addr_of_mut!(test_membarrier_thread_mutex),
        );
    }
    libc::pthread_mutex_unlock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));

    core::ptr::null_mut()
}

unsafe fn test_mt_membarrier() -> libc::c_int {
    let mut i: libc::c_int;
    let mut test_thread: libc::pthread_t = core::mem::zeroed();

    let _ = core::ptr::addr_of_mut!(i);

    libc::pthread_create(
        core::ptr::addr_of_mut!(test_thread),
        core::ptr::null(),
        Some(test_membarrier_thread),
        core::ptr::null_mut(),
    );

    libc::pthread_mutex_lock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));
    while thread_ready == 0 {
        libc::pthread_cond_wait(
            core::ptr::addr_of_mut!(test_membarrier_thread_cond),
            core::ptr::addr_of_mut!(test_membarrier_thread_mutex),
        );
    }
    libc::pthread_mutex_unlock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));

    test_membarrier_fail();

    test_membarrier_success();

    libc::pthread_mutex_lock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));
    thread_quit = 1;
    libc::pthread_cond_broadcast(core::ptr::addr_of_mut!(test_membarrier_thread_cond));
    libc::pthread_mutex_unlock(core::ptr::addr_of_mut!(test_membarrier_thread_mutex));

    libc::pthread_join(test_thread, core::ptr::null_mut());

    0
}

pub unsafe fn main(argc: libc::c_int, argv: *mut *mut libc::c_char) {
    let _ = argc;
    let _ = argv;

    ksft_print_header();
    ksft_set_plan(16);

    test_membarrier_query();

    /* Multi-threaded */
    test_mt_membarrier();

    ksft_exit_pass();
}

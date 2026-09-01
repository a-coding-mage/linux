// SPDX-License-Identifier: GPL-2.0
// C dependencies: pthread.h, stdlib.h, signal.h, unistd.h, linux/compiler.h, ../tests.h

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

type sig_atomic_t = c_int;
type pthread_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const SIGINT: c_int = 2;
const SIGALRM: c_int = 14;

static mut a: c_int = 0;
static mut done: sig_atomic_t = 0;

unsafe extern "C" {
    fn pthread_self() -> pthread_t;
    fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn alarm(seconds: c_uint) -> c_uint;
}

unsafe extern "C" fn sighandler(_sig: c_int) {
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!(done), 1);
    }
}

#[inline(always)]
unsafe fn leaf(b: c_int) {
    loop {
        unsafe {
            let current = ptr::read_volatile(ptr::addr_of!(a));
            ptr::write_volatile(ptr::addr_of_mut!(a), current.wrapping_add(b));

            if ptr::read_volatile(ptr::addr_of!(done)) != 0 {
                break;
            }
        }
    }
}

#[inline(always)]
unsafe fn middle(b: c_int) {
    unsafe {
        leaf(b);
    }
}

#[inline(never)]
unsafe fn parent(b: c_int) {
    unsafe {
        middle(b);
    }
}

unsafe extern "C" fn inlineloop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut sec: c_int = 1;

    unsafe {
        pthread_setname_np(pthread_self(), c"perf-inlineloop".as_ptr());
        if argc > 0 {
            sec = atoi(*argv.offset(0));
        }

        signal(SIGINT, Some(sighandler));
        signal(SIGALRM, Some(sighandler));
        alarm(sec as c_uint);

        parent(sec);
    }

    0
}

// DEFINE_WORKLOAD(inlineloop);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

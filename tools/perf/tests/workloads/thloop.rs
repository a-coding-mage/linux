/* SPDX-License-Identifier: GPL-2.0 */
// C dependencies: pthread.h, stdlib.h, signal.h, unistd.h, linux/compiler.h,
// and ../tests.h.

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type sig_atomic_t = c_int;
type pthread_t = c_ulong;
type useconds_t = c_uint;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const SIGINT: c_int = 2;
const SIGALRM: c_int = 14;

static mut done: sig_atomic_t = 0;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn atof(nptr: *const c_char) -> c_double;
    fn atoi(nptr: *const c_char) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn alarm(seconds: c_uint) -> c_uint;
    fn ualarm(usecs: useconds_t, interval: useconds_t) -> useconds_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

/* We want to check this symbol in perf report */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_loop() {
    while unsafe { core::ptr::read_volatile(core::ptr::addr_of!(done)) } == 0 {}
}

extern "C" fn sighandler(_sig: c_int) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(done), 1);
    }
}

extern "C" fn thfunc(arg: *mut c_void) -> *mut c_void {
    let loop_fn: extern "C" fn() = unsafe { mem::transmute(arg) };

    loop_fn();
    ptr::null_mut()
}

static ERROR_SECONDS_MUST_BE_GT_0: &[u8] = b"Error: seconds (%f) must be > 0\n\0";
static ERROR_MALLOC_FAILED_FOR_THREADS: &[u8] = b"Error: malloc failed for %d threads\n\0";
static ERROR_THREAD_COUNT_MUST_BE_GE_1: &[u8] =
    b"Error: thread count (%d) must be >= 1\n\0";
static ERROR_FAILED_TO_CREATE_THREAD: &[u8] = b"Error: failed to create thread %d\n\0";

unsafe extern "C" fn thloop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut nt: c_int = 2;
    let mut err: c_int = 1;
    let mut sec: c_double = 1.0;
    let mut thread_list: *mut pthread_t = ptr::null_mut();

    if argc > 0 {
        sec = unsafe { atof(*argv.offset(0)) };
    }

    if !(sec > 0.0) {
        unsafe {
            fprintf(
                stderr,
                ERROR_SECONDS_MUST_BE_GT_0.as_ptr() as *const c_char,
                sec,
            );
        }
        return 1;
    }

    if argc > 1 {
        nt = unsafe { atoi(*argv.offset(1)) };
    }

    if nt <= 0 {
        unsafe {
            fprintf(
                stderr,
                ERROR_THREAD_COUNT_MUST_BE_GE_1.as_ptr() as *const c_char,
                nt,
            );
        }
        return 1;
    }

    unsafe {
        signal(SIGINT, sighandler);
        signal(SIGALRM, sighandler);
    }

    thread_list = unsafe { calloc(nt as usize, mem::size_of::<pthread_t>()) as *mut pthread_t };
    if thread_list.is_null() {
        unsafe {
            fprintf(
                stderr,
                ERROR_MALLOC_FAILED_FOR_THREADS.as_ptr() as *const c_char,
                nt,
            );
        }
        // goto out;
    } else {
        let mut i: c_int = 1;
        while i < nt {
            let ret: c_int = unsafe {
                pthread_create(
                    thread_list.offset(i as isize),
                    ptr::null(),
                    thfunc,
                    test_loop as *mut c_void,
                )
            };

            if ret != 0 {
                unsafe {
                    fprintf(
                        stderr,
                        ERROR_FAILED_TO_CREATE_THREAD.as_ptr() as *const c_char,
                        i,
                    );
                    core::ptr::write_volatile(core::ptr::addr_of_mut!(done), 1);
                } // Ensure started threads terminate.
                break;
            }
            i += 1;
        }

        if i == nt {
            if sec < 1.0 {
                let usecs: useconds_t = (sec * 1000000.0) as useconds_t;

                unsafe {
                    ualarm(if usecs > 0 { usecs } else { 1 }, 0);
                }
            } else {
                unsafe {
                    alarm(sec as c_uint);
                }
            }
            unsafe {
                test_loop();
            }
            err = 0;
        }
    }

    let mut i: c_int = 1;
    while i < nt {
        if !thread_list.is_null() && unsafe { *thread_list.offset(i as isize) } != 0 {
            unsafe {
                pthread_join(*thread_list.offset(i as isize), ptr::null_mut());
            }
        }
        i += 1;
    }
    unsafe {
        free(thread_list as *mut c_void);
    }
    err
}

// DEFINE_WORKLOAD(thloop);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

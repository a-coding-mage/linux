// SPDX-License-Identifier: GPL-2.0
// C dependencies: errno.h, limits.h, pthread.h, stdio.h, stdlib.h, string.h,
// linux/compiler.h, ../tests.h

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const MAX_THREADS: usize = 25;

static mut iterations: c_int = 500;

#[no_mangle]
pub static mut named_threads_work: c_int = 1234;

type pthread_t = usize;
type thread_fn_t = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

unsafe extern "C" {
    fn pthread_self() -> pthread_t;
    fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: thread_fn_t,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;
}

macro_rules! define_thread {
    ($func:ident, $name:expr) => {
        #[inline(never)]
        #[no_mangle]
        pub unsafe extern "C" fn $func(_arg: *mut c_void) -> *mut c_void {
            unsafe {
                pthread_setname_np(pthread_self(), $name.as_ptr() as *const c_char);
                for _i in 0..iterations {
                    named_threads_work += 3;
                }
            }

            ptr::null_mut()
        }
    };
}

define_thread!(named_threads_thread1, b"thread1\0");
define_thread!(named_threads_thread2, b"thread2\0");
define_thread!(named_threads_thread3, b"thread3\0");
define_thread!(named_threads_thread4, b"thread4\0");
define_thread!(named_threads_thread5, b"thread5\0");
define_thread!(named_threads_thread6, b"thread6\0");
define_thread!(named_threads_thread7, b"thread7\0");
define_thread!(named_threads_thread8, b"thread8\0");
define_thread!(named_threads_thread9, b"thread9\0");
define_thread!(named_threads_thread10, b"thread10\0");
define_thread!(named_threads_thread11, b"thread11\0");
define_thread!(named_threads_thread12, b"thread12\0");
define_thread!(named_threads_thread13, b"thread13\0");
define_thread!(named_threads_thread14, b"thread14\0");
define_thread!(named_threads_thread15, b"thread15\0");
define_thread!(named_threads_thread16, b"thread16\0");
define_thread!(named_threads_thread17, b"thread17\0");
define_thread!(named_threads_thread18, b"thread18\0");
define_thread!(named_threads_thread19, b"thread19\0");
define_thread!(named_threads_thread20, b"thread20\0");
define_thread!(named_threads_thread21, b"thread21\0");
define_thread!(named_threads_thread22, b"thread22\0");
define_thread!(named_threads_thread23, b"thread23\0");
define_thread!(named_threads_thread24, b"thread24\0");
define_thread!(named_threads_thread25, b"thread25\0");

static thread_fns: [thread_fn_t; MAX_THREADS] = [
    named_threads_thread1,
    named_threads_thread2,
    named_threads_thread3,
    named_threads_thread4,
    named_threads_thread5,
    named_threads_thread6,
    named_threads_thread7,
    named_threads_thread8,
    named_threads_thread9,
    named_threads_thread10,
    named_threads_thread11,
    named_threads_thread12,
    named_threads_thread13,
    named_threads_thread14,
    named_threads_thread15,
    named_threads_thread16,
    named_threads_thread17,
    named_threads_thread18,
    named_threads_thread19,
    named_threads_thread20,
    named_threads_thread21,
    named_threads_thread22,
    named_threads_thread23,
    named_threads_thread24,
    named_threads_thread25,
];

/*
 * Creates argv[0] threads that run a unique function named "thread[x]" which performs
 * a multiplication in a loop for argv[1] loops.
 */
unsafe extern "C" fn named_threads(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut threads: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];
    let mut nr_threads: c_int = 1;
    let err: c_int = 0;

    unsafe {
        if argc > 0 {
            nr_threads = atoi(*argv.add(0));
        }

        if nr_threads <= 0 || nr_threads > MAX_THREADS as c_int {
            fprintf(
                stderr,
                b"Error: num threads must be 1 - %d\n\0".as_ptr() as *const c_char,
                MAX_THREADS as c_int,
            );
            return 1;
        }

        if argc > 1 {
            iterations = atoi(*argv.add(1));
        }

        if iterations < 0 {
            fprintf(
                stderr,
                b"Error: iterations must be non-negative\n\0".as_ptr() as *const c_char,
            );
            return 1;
        }

        for i in 0..nr_threads {
            let ret: c_int;

            ret = pthread_create(
                &mut threads[i as usize],
                ptr::null(),
                thread_fns[i as usize],
                ptr::null_mut(),
            );
            if ret != 0 {
                fprintf(
                    stderr,
                    b"Error: failed to create thread%d: %s\n\0".as_ptr() as *const c_char,
                    i + 1,
                    strerror(ret),
                );
                return 1;
            }
        }

        for i in 0..nr_threads {
            pthread_join(threads[i as usize], ptr::null_mut());
        }
    }

    err
}

// DEFINE_WORKLOAD(named_threads);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

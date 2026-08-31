/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: pthread.h, stdlib.h, signal.h, unistd.h, linux/compiler.h,
// and ../tests.h.

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_double = f64;
type sig_atomic_t = c_int;
type useconds_t = c_uint;
type pthread_t = c_ulong;

const SIGINT: c_int = 2;
const SIGALRM: c_int = 14;

static mut done: sig_atomic_t = 0;

unsafe extern "C" {
    fn pthread_self() -> pthread_t;
    fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    fn atof(nptr: *const c_char) -> c_double;
    fn fprintf(stream: *mut core::ffi::c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut core::ffi::c_void;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> unsafe extern "C" fn(c_int);
    fn ualarm(usecs: useconds_t, interval: useconds_t) -> useconds_t;
    fn alarm(seconds: c_uint) -> c_uint;
}

unsafe extern "C" fn sighandler(_sig: c_int) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(done), 1);
    }
}

unsafe extern "C" fn noploop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut sec: c_double = 1.0;

    unsafe {
        pthread_setname_np(pthread_self(), c"perf-noploop".as_ptr());
    }
    if argc > 0 {
        unsafe {
            sec = atof(*argv.offset(0));
        }
    }

    if !(sec > 0.0) {
        unsafe {
            fprintf(
                stderr,
                c"Error: seconds (%f) must be > 0\n".as_ptr(),
                sec,
            );
        }
        return 1;
    }

    unsafe {
        signal(SIGINT, sighandler);
        signal(SIGALRM, sighandler);
    }

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

    while unsafe { core::ptr::read_volatile(core::ptr::addr_of!(done)) } == 0 {
        continue;
    }

    return 0;
}

// DEFINE_WORKLOAD(noploop);

// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/mutex.c.
// Dependencies originally came from "mutex.h", "debug.h", linux/string.h, and errno.h.

use libc::{
    c_char, c_int, pthread_cond_t, pthread_condattr_t, pthread_mutex_t, pthread_mutexattr_t, size_t,
    EBUSY, PTHREAD_MUTEX_ERRORCHECK, PTHREAD_MUTEX_RECURSIVE, PTHREAD_PROCESS_SHARED,
};

pub const STRERR_BUFSIZE: usize = 128;

#[repr(C)]
pub struct mutex {
    pub lock: pthread_mutex_t,
}

#[repr(C)]
pub struct cond {
    pub cond: pthread_cond_t,
}

unsafe extern "C" {
    fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, type_: c_int) -> c_int;
    fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: c_int) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> c_int;
    fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;

    fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: c_int) -> c_int;
    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;
    fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;

    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn check_err(fn_: *const c_char, err: c_int) {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    if err == 0 {
        return;
    }

    unsafe {
        pr_err(
            b"%s error: '%s'\n\0".as_ptr() as *const c_char,
            fn_,
            str_error_r(err, sbuf.as_mut_ptr(), sbuf.len() as size_t),
        );
    }
}

unsafe fn __mutex_init(mtx: *mut mutex, pshared: bool, recursive: bool) {
    let mut attr = unsafe { core::mem::zeroed::<pthread_mutexattr_t>() };

    unsafe {
        check_err(b"__mutex_init\0".as_ptr() as *const c_char, pthread_mutexattr_init(&mut attr));

        // In normal builds enable error checking, such as recursive usage.
        // Original C only applies this when NDEBUG is not defined.
        #[cfg(debug_assertions)]
        check_err(
            b"__mutex_init\0".as_ptr() as *const c_char,
            pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_ERRORCHECK),
        );

        if recursive {
            check_err(
                b"__mutex_init\0".as_ptr() as *const c_char,
                pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE),
            );
        }
        if pshared {
            check_err(
                b"__mutex_init\0".as_ptr() as *const c_char,
                pthread_mutexattr_setpshared(&mut attr, PTHREAD_PROCESS_SHARED),
            );
        }
        check_err(
            b"__mutex_init\0".as_ptr() as *const c_char,
            pthread_mutex_init(&mut (*mtx).lock, &attr),
        );
        check_err(
            b"__mutex_init\0".as_ptr() as *const c_char,
            pthread_mutexattr_destroy(&mut attr),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_init(mtx: *mut mutex) {
    unsafe {
        __mutex_init(mtx, false, false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_init_pshared(mtx: *mut mutex) {
    unsafe {
        __mutex_init(mtx, true, false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_init_recursive(mtx: *mut mutex) {
    unsafe {
        __mutex_init(mtx, false, true);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_destroy(mtx: *mut mutex) {
    unsafe {
        check_err(
            b"mutex_destroy\0".as_ptr() as *const c_char,
            pthread_mutex_destroy(&mut (*mtx).lock),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_lock(mtx: *mut mutex) {
    // Original C uses NO_THREAD_SAFETY_ANALYSIS.
    unsafe {
        check_err(
            b"mutex_lock\0".as_ptr() as *const c_char,
            pthread_mutex_lock(&mut (*mtx).lock),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_unlock(mtx: *mut mutex) {
    // Original C uses NO_THREAD_SAFETY_ANALYSIS.
    unsafe {
        check_err(
            b"mutex_unlock\0".as_ptr() as *const c_char,
            pthread_mutex_unlock(&mut (*mtx).lock),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mutex_trylock(mtx: *mut mutex) -> bool {
    let ret = unsafe { pthread_mutex_trylock(&mut (*mtx).lock) };

    if ret == 0 {
        return true; /* Lock acquired. */
    }

    if ret == EBUSY {
        return false; /* Lock busy. */
    }

    /* Print error. */
    unsafe {
        check_err(b"mutex_trylock\0".as_ptr() as *const c_char, ret);
    }
    false
}

unsafe fn __cond_init(cnd: *mut cond, pshared: bool) {
    let mut attr = unsafe { core::mem::zeroed::<pthread_condattr_t>() };

    unsafe {
        check_err(b"__cond_init\0".as_ptr() as *const c_char, pthread_condattr_init(&mut attr));
        if pshared {
            check_err(
                b"__cond_init\0".as_ptr() as *const c_char,
                pthread_condattr_setpshared(&mut attr, PTHREAD_PROCESS_SHARED),
            );
        }

        check_err(
            b"__cond_init\0".as_ptr() as *const c_char,
            pthread_cond_init(&mut (*cnd).cond, &attr),
        );
        check_err(
            b"__cond_init\0".as_ptr() as *const c_char,
            pthread_condattr_destroy(&mut attr),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_init(cnd: *mut cond) {
    unsafe {
        __cond_init(cnd, false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_init_pshared(cnd: *mut cond) {
    unsafe {
        __cond_init(cnd, true);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_destroy(cnd: *mut cond) {
    unsafe {
        check_err(
            b"cond_destroy\0".as_ptr() as *const c_char,
            pthread_cond_destroy(&mut (*cnd).cond),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_wait(cnd: *mut cond, mtx: *mut mutex) {
    unsafe {
        check_err(
            b"cond_wait\0".as_ptr() as *const c_char,
            pthread_cond_wait(&mut (*cnd).cond, &mut (*mtx).lock),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_signal(cnd: *mut cond) {
    unsafe {
        check_err(
            b"cond_signal\0".as_ptr() as *const c_char,
            pthread_cond_signal(&mut (*cnd).cond),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cond_broadcast(cnd: *mut cond) {
    unsafe {
        check_err(
            b"cond_broadcast\0".as_ptr() as *const c_char,
            pthread_cond_broadcast(&mut (*cnd).cond),
        );
    }
}

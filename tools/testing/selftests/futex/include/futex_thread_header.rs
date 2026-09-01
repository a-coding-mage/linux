/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header dependencies: errno.h, pthread.h, stdio.h, string.h, unistd.h,
// and "kselftest_harness.h".

pub const USEC_PER_SEC: libc::c_long = 1000000;
pub const WAIT_FOR_THREAD_SECS: libc::c_uint = 1;
pub const WAIT_FOR_THREAD_USECS: libc::c_long =
    (WAIT_FOR_THREAD_SECS as libc::c_long) * USEC_PER_SEC;
pub const WAIT_THREAD_RETRIES: libc::c_uint = 100;

#[repr(C)]
pub struct futex_thread {
    pub thread: libc::pthread_t,
    pub barrier: libc::pthread_barrier_t,
    pub tid: libc::pid_t,
    pub threadfn: Option<unsafe extern "C" fn(arg: *mut libc::c_void) -> libc::c_int>,
    pub arg: *mut libc::c_void,
    pub retval: libc::c_int,
}

extern "C" {
    pub fn gettid() -> libc::pid_t;
}

#[inline]
pub unsafe fn __wait_for_thread(
    fp: *mut libc::FILE,
    _metadata: *mut __test_metadata,
) -> libc::c_int {
    let sleep_time_us: libc::c_uint =
        (WAIT_FOR_THREAD_USECS / WAIT_THREAD_RETRIES as libc::c_long) as libc::c_uint;
    let mut buf: [libc::c_char; 80] = [0; 80];

    for _i in 0..WAIT_THREAD_RETRIES {
        if libc::fgets(buf.as_mut_ptr(), buf.len() as libc::c_int, fp).is_null() {
            return libc::EIO;
        }
        if libc::strncmp(
            buf.as_ptr(),
            b"futex\0".as_ptr() as *const libc::c_char,
            5,
        ) == 0
        {
            return 0;
        }
        libc::usleep(sleep_time_us);
        libc::rewind(fp);
    }

    TH_LOG!(
        "/proc/$PID/wchan contains \"%s\". Trying to continue.",
        buf.as_ptr()
    );
    0
}

pub unsafe extern "C" fn __futex_thread_fn(arg: *mut libc::c_void) -> *mut libc::c_void {
    let t: *mut futex_thread = arg as *mut futex_thread;

    (*t).tid = gettid();
    libc::pthread_barrier_wait(&mut (*t).barrier);
    (*t).retval = ((*t).threadfn.expect("threadfn is NULL"))((*t).arg);
    core::ptr::null_mut()
}

/**
 * futex_wait_for_thread - Wait for the child thread to sleep in the futex context
 * @t:          Thread handle.
 * @_metadata:	Test metadata for TH_LOG() context
 */
#[inline]
pub unsafe fn futex_wait_for_thread(
    t: *mut futex_thread,
    _metadata: *mut __test_metadata,
) -> libc::c_int {
    let mut fname: [libc::c_char; 80] = [0; 80];
    let fp: *mut libc::FILE;
    let res: libc::c_int;

    libc::snprintf(
        fname.as_mut_ptr(),
        fname.len(),
        b"/proc/%d/wchan\0".as_ptr() as *const libc::c_char,
        (*t).tid,
    );
    fp = libc::fopen(
        fname.as_ptr(),
        b"r\0".as_ptr() as *const libc::c_char,
    );
    if fp.is_null() {
        /* If /proc/... is not available, sleep */
        if *__errno_location() != libc::ENOENT {
            return *__errno_location();
        }
        TH_LOG!("/proc/$PID/wchan not accessible, continue with sleep()");
        libc::sleep(WAIT_FOR_THREAD_SECS);
        return 0;
    }

    res = __wait_for_thread(fp, _metadata);
    libc::fclose(fp);
    res
}

/**
 * futex_thread_create - Create a new thread for testing.
 * @t:        The handle of the newly created thread.
 * @threadfn: The new thread starts execution by invoking threadfn
 * @arg:      The parameters passed to threadfn.
 */
#[inline]
pub unsafe fn futex_thread_create(
    t: *mut futex_thread,
    threadfn: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    arg: *mut libc::c_void,
) -> libc::c_int {
    libc::pthread_barrier_init(&mut (*t).barrier, core::ptr::null(), 2);

    (*t).tid = 0;
    (*t).threadfn = threadfn;
    (*t).arg = arg;

    if libc::pthread_create(
        &mut (*t).thread,
        core::ptr::null(),
        __futex_thread_fn,
        t as *mut libc::c_void,
    ) < 0
    {
        let ret: libc::c_int = *__errno_location();
        libc::pthread_barrier_destroy(&mut (*t).barrier);
        return ret;
    }

    libc::pthread_barrier_wait(&mut (*t).barrier);
    0
}

/**
 * futex_thread_destroy - Wait for and reclaim the resources of the thread.
 * @t:      Thread handle.
 */
#[inline]
pub unsafe fn futex_thread_destroy(t: *mut futex_thread) -> libc::c_int {
    libc::pthread_join((*t).thread, core::ptr::null_mut());
    libc::pthread_barrier_destroy(&mut (*t).barrier);
    (*t).retval
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

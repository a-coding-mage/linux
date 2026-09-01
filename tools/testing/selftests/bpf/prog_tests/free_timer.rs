// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025. Huawei Technologies Co., Ltd */

use core::ffi::{c_int, c_long, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct free_timer {
    pub obj: *mut bpf_object,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: size_of::<Self>(),
            retval: 0,
        }
    }
}

#[repr(C)]
struct run_ctx {
    start_prog: *mut bpf_program,
    overwrite_prog: *mut bpf_program,
    notify: libc::pthread_barrier_t,
    loop_: c_int,
    start: AtomicBool,
    stop: AtomicBool,
}

unsafe extern "C" {
    fn usleep(usec: libc::useconds_t) -> c_int;
    fn pthread_self() -> libc::pthread_t;
    fn pthread_setaffinity_np(
        thread: libc::pthread_t,
        cpusetsize: usize,
        cpuset: *const libc::cpu_set_t,
    ) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut libc::pthread_barrier_t,
        attr: *const libc::pthread_barrierattr_t,
        count: libc::c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut libc::pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut c_void) -> c_int;

    fn free_timer__open_and_load() -> *mut free_timer;
    fn free_timer__destroy(obj: *mut free_timer);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const libc::c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn test__skip();
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const libc::c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const libc::c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const libc::c_char) -> bool;
}

const EOPNOTSUPP: c_int = 95;

unsafe fn CPU_ZERO(set: *mut libc::cpu_set_t) {
    ptr::write_bytes(set.cast::<u8>(), 0, size_of::<libc::cpu_set_t>());
}

unsafe fn CPU_SET(cpu: usize, set: *mut libc::cpu_set_t) {
    let bits_per_word = 8 * size_of::<libc::c_ulong>();
    let words = set.cast::<libc::c_ulong>();
    *words.add(cpu / bits_per_word) |= (1 as libc::c_ulong) << (cpu % bits_per_word);
}

fn start_threads(ctx: *mut run_ctx) {
    unsafe {
        (*ctx).start.store(true, Ordering::Relaxed);
    }
}

fn stop_threads(ctx: *mut run_ctx) {
    unsafe {
        (*ctx).stop.store(true, Ordering::Relaxed);
        /* Guarantee the order between ->stop and ->start */
        (*ctx).start.store(true, Ordering::Release);
    }
}

unsafe fn wait_for_start(ctx: *mut run_ctx) -> c_int {
    while !(*ctx).start.load(Ordering::Acquire) {
        usleep(10);
    }

    (*ctx).stop.load(Ordering::Relaxed) as c_int
}

unsafe extern "C" fn overwrite_timer_fn(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut run_ctx;
    let mut loop_: c_int;
    let fd: c_int;
    let mut err: c_int;
    let mut cpuset: libc::cpu_set_t = zeroed();
    let mut ret: c_long = 0;

    /* Pin on CPU 0 */
    CPU_ZERO(&mut cpuset);
    CPU_SET(0, &mut cpuset);
    pthread_setaffinity_np(pthread_self(), size_of::<libc::cpu_set_t>(), &cpuset);

    /* Is the thread being stopped ? */
    err = wait_for_start(ctx);
    if err != 0 {
        return ptr::null_mut();
    }

    fd = bpf_program__fd((*ctx).overwrite_prog);
    loop_ = (*ctx).loop_;
    while {
        let old = loop_;
        loop_ -= 1;
        old > 0
    } {
        let mut opts = bpf_test_run_opts::default();

        /* Wait for start thread to complete */
        pthread_barrier_wait(&mut (*ctx).notify);

        /* Overwrite timers */
        err = bpf_prog_test_run_opts(fd, &mut opts);
        if err != 0 {
            ret |= 1;
        } else if opts.retval != 0 {
            ret |= 2;
        }

        /* Notify start thread to start timers */
        pthread_barrier_wait(&mut (*ctx).notify);
    }

    ret as usize as *mut c_void
}

unsafe extern "C" fn start_timer_fn(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut run_ctx;
    let mut loop_: c_int;
    let fd: c_int;
    let mut err: c_int;
    let mut cpuset: libc::cpu_set_t = zeroed();
    let mut ret: c_long = 0;

    /* Pin on CPU 1 */
    CPU_ZERO(&mut cpuset);
    CPU_SET(1, &mut cpuset);
    pthread_setaffinity_np(pthread_self(), size_of::<libc::cpu_set_t>(), &cpuset);

    /* Is the thread being stopped ? */
    err = wait_for_start(ctx);
    if err != 0 {
        return ptr::null_mut();
    }

    fd = bpf_program__fd((*ctx).start_prog);
    loop_ = (*ctx).loop_;
    while {
        let old = loop_;
        loop_ -= 1;
        old > 0
    } {
        let mut opts = bpf_test_run_opts::default();

        /* Run the prog to start timer */
        err = bpf_prog_test_run_opts(fd, &mut opts);
        if err != 0 {
            ret |= 4;
        } else if opts.retval != 0 {
            ret |= 8;
        }

        /* Notify overwrite thread to do overwrite */
        pthread_barrier_wait(&mut (*ctx).notify);

        /* Wait for overwrite thread to complete */
        pthread_barrier_wait(&mut (*ctx).notify);
    }

    ret as usize as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn test_free_timer() {
    let mut skel: *mut free_timer;
    let mut prog: *mut bpf_program;
    let mut ctx: run_ctx = zeroed();
    let mut tid: [libc::pthread_t; 2] = zeroed();
    let mut ret: *mut c_void;
    let mut err: c_int;

    skel = free_timer__open_and_load();
    if skel.is_null() && *libc::__errno_location() == EOPNOTSUPP {
        test__skip();
        return;
    }
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"open_load".as_ptr()) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, c"start_timer".as_ptr());
    if !ASSERT_OK_PTR(prog.cast::<c_void>(), c"find start prog".as_ptr()) {
        goto_out(skel);
        return;
    }
    ctx.start_prog = prog;

    prog = bpf_object__find_program_by_name((*skel).obj, c"overwrite_timer".as_ptr());
    if !ASSERT_OK_PTR(prog.cast::<c_void>(), c"find overwrite prog".as_ptr()) {
        goto_out(skel);
        return;
    }
    ctx.overwrite_prog = prog;

    pthread_barrier_init(&mut ctx.notify, ptr::null(), 2);
    ctx.loop_ = 10;

    err = pthread_create(
        &mut tid[0],
        ptr::null(),
        start_timer_fn,
        (&mut ctx as *mut run_ctx).cast::<c_void>(),
    );
    if !ASSERT_OK(err, c"create start_timer".as_ptr()) {
        goto_out(skel);
        return;
    }

    err = pthread_create(
        &mut tid[1],
        ptr::null(),
        overwrite_timer_fn,
        (&mut ctx as *mut run_ctx).cast::<c_void>(),
    );
    if !ASSERT_OK(err, c"create overwrite_timer".as_ptr()) {
        stop_threads(&mut ctx);
        goto_out(skel);
        return;
    }

    start_threads(&mut ctx);

    ret = ptr::null_mut();
    err = pthread_join(tid[0], &mut ret);
    ASSERT_EQ(err as c_long | ret as c_long, 0, c"start_timer".as_ptr());
    ret = ptr::null_mut();
    err = pthread_join(tid[1], &mut ret);
    ASSERT_EQ(err as c_long | ret as c_long, 0, c"overwrite_timer".as_ptr());

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut free_timer) {
    free_timer__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

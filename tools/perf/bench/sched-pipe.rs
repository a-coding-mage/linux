// SPDX-License-Identifier: GPL-2.0
/*
 *
 * sched-pipe.c
 *
 * pipe: Benchmark for pipe()
 *
 * Based on pipe-test-1m.c by Ingo Molnar <mingo@redhat.com>
 *  http://people.redhat.com/mingo/cfs-scheduler/tools/pipe-test-1m.c
 * Ported to perf by Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
 */

use libc::{
    c_char, c_int, c_long, c_uint, c_void, pid_t, pthread_t, timeval, EACCES, EINTR, ENOENT,
    EWOULDBLOCK, F_SETPIPE_SZ, INT_MAX, O_NONBLOCK, O_WRONLY, PATH_MAX, POLLIN, POLLOUT,
};

#[repr(C)]
struct option {
    _unused: [u8; 0],
}

#[repr(C)]
struct cgroup {
    fd: c_int,
    name: *mut c_char,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: u64,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct thread_data {
    nr: c_int,
    pipe_read: c_int,
    pipe_write: c_int,
    epoll_ev: epoll_event,
    epoll_fd: c_int,
    cgroup_failed: bool,
    pthread: pthread_t,
    buf: *mut c_char,
}

const LOOPS_DEFAULT: c_int = 1000000;
const USEC_PER_SEC: u64 = 1000000;
const USEC_PER_MSEC: c_long = 1000;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLLIN: u32 = POLLIN as u32;
const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;

static mut loops: c_int = LOOPS_DEFAULT;

/* Use processes by default: */
static mut threaded: bool = false;

static mut nonblocking: bool = false;
static mut write_size: c_uint = core::mem::size_of::<c_int>() as c_uint;
static mut cgrp_names: [*mut c_char; 2] = [core::ptr::null_mut(); 2];
static mut cgrps: [*mut cgroup; 2] = [core::ptr::null_mut(); 2];

unsafe extern "C" {
    static mut bench_format: c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn cgroup__new(name: *const c_char, do_open: bool) -> *mut cgroup;
    fn cgroup__put(cgrp: *mut cgroup);
    fn cgroupfs_find_mountpoint(buf: *mut c_char, maxlen: usize, type_: *const c_char) -> c_int;

    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut libc::FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn getpid() -> pid_t;
    fn geteuid() -> libc::uid_t;
    fn poll(fds: *mut pollfd, nfds: libc::nfds_t, timeout: c_int) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        libc::abort();
    }
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1000000;
    }
}

unsafe extern "C" fn parse_two_cgroups(
    _opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let p = strdup(str_);
    let q: *mut c_char;
    let mut ret: c_int = -1;

    if p.is_null() {
        fprintf(
            libc::stderr,
            c"memory allocation failure\n".as_ptr(),
        );
        return -1;
    }

    q = strchr(p, ',' as c_int);
    if q.is_null() {
        fprintf(
            libc::stderr,
            c"it should have two cgroup names: %s\n".as_ptr(),
            p,
        );
        goto_out(p, ret);
        return ret;
    }
    *q = '\0' as c_char;

    cgrp_names[0] = strdup(p);
    cgrp_names[1] = strdup(q.add(1));

    if cgrp_names[0].is_null() || cgrp_names[1].is_null() {
        fprintf(libc::stderr, c"memory allocation failure\n".as_ptr());
        goto_out(p, ret);
        return ret;
    }
    ret = 0;

    goto_out(p, ret)
}

unsafe fn goto_out(p: *mut c_char, ret: c_int) -> c_int {
    free(p as *mut c_void);
    ret
}

/*
 * C source defines this with parse-options OPT_* macros:
 * OPT_BOOLEAN('n', "nonblocking", &nonblocking, ...)
 * OPT_INTEGER('l', "loop", &loops, ...)
 * OPT_BOOLEAN('T', "threaded", &threaded, ...)
 * OPT_UINTEGER('s', "write-size", &write_size, ...)
 * OPT_CALLBACK('G', "cgroups", NULL, "SEND,RECV", ..., parse_two_cgroups)
 * OPT_END()
 */
static options: [option; 1] = [option { _unused: [] }];

static bench_sched_pipe_usage_0: &[u8] = b"perf bench sched pipe <options>\0";
static bench_sched_pipe_usage_1: &[u8] = b"\0";
static bench_sched_pipe_usage: [*const c_char; 2] = [
    bench_sched_pipe_usage_0.as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe fn enter_cgroup(nr: c_int) -> c_int {
    let mut buf: [c_char; 32] = [0; 32];
    let mut fd: c_int;
    let len: c_int;
    let ret: isize;
    let saved_errno: c_int;
    let cgrp: *mut cgroup;
    let pid: pid_t;

    if cgrp_names[nr as usize].is_null() {
        return 0;
    }

    if cgrps[nr as usize].is_null() {
        cgrps[nr as usize] = cgroup__new(cgrp_names[nr as usize], true);
        if cgrps[nr as usize].is_null() {
            return err(nr);
        }
    }
    cgrp = cgrps[nr as usize];

    if threaded {
        pid = syscall(libc::SYS_gettid as c_long) as pid_t;
    } else {
        pid = getpid();
    }

    snprintf(buf.as_mut_ptr(), buf.len(), c"%d\n".as_ptr(), pid);
    len = strlen(buf.as_ptr()) as c_int;

    /* try cgroup v2 interface first */
    if threaded {
        fd = openat((*cgrp).fd, c"cgroup.threads".as_ptr(), O_WRONLY);
    } else {
        fd = openat((*cgrp).fd, c"cgroup.procs".as_ptr(), O_WRONLY);
    }

    /* try cgroup v1 if failed */
    if fd < 0 && errno() == ENOENT {
        fd = openat((*cgrp).fd, c"tasks".as_ptr(), O_WRONLY);
    }

    if fd < 0 {
        return err(nr);
    }

    ret = write(fd, buf.as_ptr() as *const c_void, len as usize);
    close(fd);

    if ret != len as isize {
        printf(c"Cannot enter to cgroup: %s\n".as_ptr(), (*cgrp).name);
        return -1;
    }
    return 0;

    unsafe fn err(nr: c_int) -> c_int {
        let saved_errno = errno();
        printf(
            c"Failed to open cgroup file in %s\n".as_ptr(),
            cgrp_names[nr as usize],
        );

        if saved_errno == ENOENT {
            let mut mnt: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];

            if cgroupfs_find_mountpoint(
                mnt.as_mut_ptr(),
                mnt.len(),
                c"perf_event".as_ptr(),
            ) == 0
            {
                printf(
                    c" Hint: create the cgroup first, like 'mkdir %s/%s'\n".as_ptr(),
                    mnt.as_ptr(),
                    cgrp_names[nr as usize],
                );
            }
        } else if saved_errno == EACCES && geteuid() > 0 {
            printf(c" Hint: try to run as root\n".as_ptr());
        }

        -1
    }
}

unsafe fn exit_cgroup(nr: c_int) {
    cgroup__put(cgrps[nr as usize]);
    free(cgrp_names[nr as usize] as *mut c_void);
}

/* Sleep until @fd is writable, so we don't busy-spin on EWOULDBLOCK. */
unsafe fn wait_writable(fd: c_int) {
    let mut pfd = pollfd {
        fd,
        events: POLLOUT,
        revents: 0,
    };

    poll(&mut pfd, 1, -1);
}

/*
 * Loop on short read()/write(): the kernel may return fewer bytes than
 * requested, retry on EINTR, and in non-blocking mode wait via poll()
 * when the writer transiently hits EWOULDBLOCK while the peer is still
 * draining a full pipe (capacity is sized to write_size).
 */
unsafe fn write_pipe(td: *mut thread_data) -> c_int {
    let mut done: c_uint = 0;
    let mut ret: isize;

    while done < write_size {
        ret = write(
            (*td).pipe_write,
            (*td).buf.add(done as usize) as *const c_void,
            (write_size - done) as usize,
        );
        if ret < 0 {
            if errno() == EINTR {
                continue;
            }
            if nonblocking && errno() == EWOULDBLOCK {
                wait_writable((*td).pipe_write);
                continue;
            }
            return ret as c_int;
        }
        done = done.wrapping_add(ret as c_uint);
    }
    done as c_int
}

unsafe fn read_pipe(td: *mut thread_data) -> c_int {
    let mut done: c_uint = 0;
    let mut ret: isize;

    while done < write_size {
        if nonblocking {
            ret = epoll_wait((*td).epoll_fd, &mut (*td).epoll_ev, 1, -1) as isize;
            if ret < 0 {
                if errno() == EINTR {
                    continue;
                }
                return ret as c_int;
            }
        }
        ret = read(
            (*td).pipe_read,
            (*td).buf.add(done as usize) as *mut c_void,
            (write_size - done) as usize,
        );
        if ret < 0 {
            if errno() == EINTR {
                continue;
            }
            if nonblocking && errno() == EWOULDBLOCK {
                continue;
            }
            return ret as c_int;
        }
        if ret == 0 {
            return done as c_int;
        }
        done = done.wrapping_add(ret as c_uint);
    }
    done as c_int
}

unsafe extern "C" fn worker_thread(__tdata: *mut c_void) -> *mut c_void {
    let td = __tdata as *mut thread_data;
    let mut i: c_int;
    let mut ret: c_int;

    ret = enter_cgroup((*td).nr);
    if ret < 0 {
        (*td).cgroup_failed = true;
        return core::ptr::null_mut();
    }

    if nonblocking {
        (*td).epoll_ev.events = EPOLLIN;
        (*td).epoll_fd = epoll_create(1);
        BUG_ON((*td).epoll_fd < 0);
        BUG_ON(epoll_ctl((*td).epoll_fd, EPOLL_CTL_ADD, (*td).pipe_read, &mut (*td).epoll_ev) < 0);
    }

    i = 0;
    while i < loops {
        ret = write_pipe(td);
        BUG_ON(ret != write_size as c_int);
        ret = read_pipe(td);
        BUG_ON(ret != write_size as c_int);
        i += 1;
    }

    core::ptr::null_mut()
}

/*
 * On a custom write_size, resize the pipes so a single payload fits.
 */
unsafe fn resize_pipes(wfd1: c_int, wfd2: c_int) -> c_int {
    let r1: c_int;
    let r2: c_int;

    if write_size <= core::mem::size_of::<c_int>() as c_uint {
        return 0;
    }

    r1 = fcntl(wfd1, F_SETPIPE_SZ, write_size);
    r2 = fcntl(wfd2, F_SETPIPE_SZ, write_size);
    if r1 < 0 || r2 < 0 || r1 as c_uint < write_size || r2 as c_uint < write_size {
        fprintf(
            libc::stderr,
            c"--write-size %u exceeds /proc/sys/fs/pipe-max-size\n".as_ptr(),
            write_size,
        );
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bench_sched_pipe(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut threads: [thread_data; 2] = core::mem::zeroed();
    let mut td: *mut thread_data;
    let mut pipe_1: [c_int; 2] = [0; 2];
    let mut pipe_2: [c_int; 2] = [0; 2];
    let mut start: timeval = core::mem::zeroed();
    let mut stop: timeval = core::mem::zeroed();
    let mut diff: timeval = core::mem::zeroed();
    let mut result_usec: u64 = 0;
    let nr_threads: c_int = 2;
    let mut t: c_int;

    /*
     * why does "ret" exist?
     * discarding returned value of read(), write()
     * causes error in building environment for perf
     */
    let mut ret: c_int;
    let mut wait_stat: c_int = 0;
    let mut flags: c_int = 0;
    let pid: pid_t;
    let retpid: pid_t;

    let _argc = parse_options(argc, argv, options.as_ptr(), bench_sched_pipe_usage.as_ptr(), 0);

    /*
     * The error paths below return early without closing the pipes or
     * freeing the cgroup state. That is fine: bench_sched_pipe() runs
     * once and the process exits right after it returns, so these are
     * not real leaks.
     */
    if write_size == 0 || write_size > INT_MAX as c_uint {
        fprintf(
            libc::stderr,
            c"--write-size must be in 1..%d\n".as_ptr(),
            INT_MAX,
        );
        return -1;
    }

    if nonblocking {
        flags |= O_NONBLOCK;
    }

    BUG_ON(pipe2(pipe_1.as_mut_ptr(), flags) != 0);
    BUG_ON(pipe2(pipe_2.as_mut_ptr(), flags) != 0);

    if resize_pipes(pipe_1[1], pipe_2[1]) < 0 {
        return -1;
    }

    t = 0;
    while t < nr_threads {
        threads[t as usize].buf = calloc(1, write_size as usize) as *mut c_char;
        BUG_ON(threads[t as usize].buf.is_null());
        t += 1;
    }

    gettimeofday(&mut start, core::ptr::null_mut());

    t = 0;
    while t < nr_threads {
        td = threads.as_mut_ptr().add(t as usize);

        (*td).nr = t;

        if t == 0 {
            (*td).pipe_read = pipe_1[0];
            (*td).pipe_write = pipe_2[1];
        } else {
            (*td).pipe_write = pipe_1[1];
            (*td).pipe_read = pipe_2[0];
        }
        t += 1;
    }

    if threaded {
        t = 0;
        while t < nr_threads {
            td = threads.as_mut_ptr().add(t as usize);

            ret = pthread_create(
                &mut (*td).pthread,
                core::ptr::null(),
                worker_thread,
                td as *mut c_void,
            );
            BUG_ON(ret != 0);
            t += 1;
        }

        t = 0;
        while t < nr_threads {
            td = threads.as_mut_ptr().add(t as usize);

            ret = pthread_join((*td).pthread, core::ptr::null_mut());
            BUG_ON(ret != 0);
            t += 1;
        }
    } else {
        pid = fork();
        assert!(pid >= 0);

        if pid == 0 {
            worker_thread(threads.as_mut_ptr().add(0) as *mut c_void);
            exit(0);
        } else {
            worker_thread(threads.as_mut_ptr().add(1) as *mut c_void);
        }

        retpid = waitpid(pid, &mut wait_stat, 0);
        assert!((retpid == pid) && WIFEXITED(wait_stat));
    }

    gettimeofday(&mut stop, core::ptr::null_mut());
    timersub(&stop, &start, &mut diff);

    t = 0;
    while t < nr_threads {
        free(threads[t as usize].buf as *mut c_void);
        t += 1;
    }

    exit_cgroup(0);
    exit_cgroup(1);

    if threads[0].cgroup_failed || threads[1].cgroup_failed {
        return 0;
    }

    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            printf(
                c"# Executed %d pipe operations between two %s\n\n".as_ptr(),
                loops,
                if threaded {
                    c"threads".as_ptr()
                } else {
                    c"processes".as_ptr()
                },
            );

            result_usec = (diff.tv_sec as u64).wrapping_mul(USEC_PER_SEC);
            result_usec = result_usec.wrapping_add(diff.tv_usec as u64);

            printf(
                c" %14s: %lu.%03lu [sec]\n\n".as_ptr(),
                c"Total time".as_ptr(),
                diff.tv_sec as libc::c_ulong,
                (diff.tv_usec / USEC_PER_MSEC) as libc::c_ulong,
            );

            printf(
                c" %14lf usecs/op\n".as_ptr(),
                result_usec as f64 / loops as f64,
            );
            printf(
                c" %14d ops/sec\n".as_ptr(),
                (loops as f64 / (result_usec as f64 / USEC_PER_SEC as f64)) as c_int,
            );
        }

        BENCH_FORMAT_SIMPLE => {
            printf(
                c"%lu.%03lu\n".as_ptr(),
                diff.tv_sec as libc::c_ulong,
                (diff.tv_usec / USEC_PER_MSEC) as libc::c_ulong,
            );
        }

        _ => {
            /* reaching here is something disaster */
            fprintf(
                libc::stderr,
                c"Unknown format:%d\n".as_ptr(),
                bench_format,
            );
            exit(1);
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

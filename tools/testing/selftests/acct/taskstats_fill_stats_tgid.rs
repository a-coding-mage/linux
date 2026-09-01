// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included Linux netlink/taskstats, pthread,
// libc, netlink_helper.h, and kselftest.h dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const BUSY_NS: u64 = 200u64 * 1000 * 1000;

const CLOCK_MONOTONIC: c_int = 1;
const EXIT_FAILURE: c_int = 1;
const NLM_F_REQUEST: u16 = 1;
const NLMSG_ERROR: u16 = 2;
const ENOENT: c_int = 2;
const TASKSTATS_CMD_GET: u8 = 1;
const TASKSTATS_CMD_ATTR_TGID: u16 = 3;
const TASKSTATS_TYPE_AGGR_PID: u16 = 4;
const TASKSTATS_TYPE_AGGR_TGID: u16 = 5;
const TASKSTATS_TYPE_STATS: u16 = 1;
const TASKSTATS_GENL_NAME: *const c_char = b"TASKSTATS\0".as_ptr() as *const c_char;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;

#[repr(C)]
struct pthread_mutex_t {
    __data: [u8; 40],
}

#[repr(C)]
struct pthread_cond_t {
    __data: [u8; 48],
}

type pthread_t = c_ulong;
type pid_t = c_int;
type ssize_t = isize;

#[repr(C)]
struct worker_ctx {
    lock: pthread_mutex_t,
    cond: pthread_cond_t,
    ready: bool,
    release: bool,
}

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct genlmsghdr {
    cmd: u8,
    version: u8,
    reserved: u16,
}

#[repr(C)]
struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct taskstats {
    _opaque: [u8; 0],
    ac_utime: u64,
    ac_stime: u64,
    nvcsw: u64,
    nivcsw: u64,
}

#[repr(C)]
struct request {
    nlh: nlmsghdr,
    genl: genlmsghdr,
    buf: [c_char; 256],
}

static mut busy_sink: c_ulong = 0;

unsafe extern "C" {
    static mut errno: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn getpid() -> pid_t;
    fn geteuid() -> c_uint;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn send_request(fd: c_int, msg: *const c_void, len: c_int) -> c_int;
    fn netlink_open() -> c_int;
    fn get_family_id(fd: c_int, family_name: *const c_char) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_finished();
    fn ksft_get_fail_cnt() -> c_int;
}

const fn nlmsg_align(len: u32) -> u32 {
    (len + 4 - 1) & !(4 - 1)
}

const fn nla_align(len: u16) -> u16 {
    (len + 4 - 1) & !(4 - 1)
}

const NLMSG_HDRLEN: u32 = nlmsg_align(size_of::<nlmsghdr>() as u32);
const GENL_HDRLEN: u32 = nlmsg_align(size_of::<genlmsghdr>() as u32);
const NLA_HDRLEN: u16 = nla_align(size_of::<nlattr>() as u16);

const fn nlmsg_length(len: u32) -> u32 {
    len + NLMSG_HDRLEN
}

unsafe fn nlmsg_ok(nlh: *const nlmsghdr, len: c_int) -> bool {
    len >= size_of::<nlmsghdr>() as c_int
        && (*nlh).nlmsg_len >= size_of::<nlmsghdr>() as u32
        && (*nlh).nlmsg_len <= len as u32
}

unsafe fn nlmsg_next(nlh: *mut nlmsghdr, len: *mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(NLMSG_HDRLEN as usize) as *mut c_void
}

unsafe fn nla_ok(nla: *const nlattr, rem: c_int) -> bool {
    rem >= size_of::<nlattr>() as c_int
        && (*nla).nla_len >= size_of::<nlattr>() as u16
        && (*nla).nla_len <= rem as u16
}

unsafe fn nla_next(nla: *mut nlattr, rem: *mut c_int) -> *mut nlattr {
    let aligned = nla_align((*nla).nla_len) as c_int;
    *rem -= aligned;
    (nla as *mut u8).add(aligned as usize) as *mut nlattr
}

unsafe fn nla_data(nla: *mut nlattr) -> *mut c_void {
    (nla as *mut u8).add(NLA_HDRLEN as usize) as *mut c_void
}

unsafe fn timespec_diff_ns(start: *const timespec, end: *const timespec) -> u64 {
    ((*end).tv_sec - (*start).tv_sec) as u64 * 1000000000u64
        + ((*end).tv_nsec - (*start).tv_nsec) as u64
}

unsafe fn burn_cpu_for_ns(runtime_ns: u64) {
    let mut start: timespec = zeroed();
    let mut now: timespec = zeroed();
    let mut acc: c_ulong = 0;

    if clock_gettime(CLOCK_MONOTONIC, &mut start) != 0 {
        perror(c"clock_gettime".as_ptr());
        exit(EXIT_FAILURE);
    }

    loop {
        for i in 0..100000 {
            acc = acc.wrapping_add(i as c_ulong);
        }
        if clock_gettime(CLOCK_MONOTONIC, &mut now) != 0 {
            perror(c"clock_gettime".as_ptr());
            exit(EXIT_FAILURE);
        }
        if timespec_diff_ns(&start, &now) >= runtime_ns {
            break;
        }
    }

    busy_sink = acc;
}

unsafe fn get_taskstats(
    fd: c_int,
    family_id: c_int,
    attr_type: u16,
    id: u32,
    stats: *mut taskstats,
) -> c_int {
    let mut req: request = zeroed();
    let mut resp = [0u8; 16384];
    let mut nlh: *mut nlmsghdr;
    let mut genl: *mut genlmsghdr;
    let mut na: *mut nlattr;
    let mut nested: *mut nlattr;
    let mut len: c_int;
    let mut rem: c_int;
    let mut nrem: c_int;
    let mut ret: c_int;

    memset(
        stats as *mut c_void,
        0,
        size_of::<taskstats>(),
    );

    req.nlh.nlmsg_len = nlmsg_length(GENL_HDRLEN);
    req.nlh.nlmsg_type = family_id as u16;
    req.nlh.nlmsg_flags = NLM_F_REQUEST;
    req.nlh.nlmsg_seq = 2;
    req.nlh.nlmsg_pid = getpid() as u32;

    req.genl.cmd = TASKSTATS_CMD_GET;
    req.genl.version = 1;

    na = (&mut req as *mut request as *mut u8).add(nlmsg_align(req.nlh.nlmsg_len) as usize)
        as *mut nlattr;
    (*na).nla_type = attr_type;
    (*na).nla_len = NLA_HDRLEN + size_of::<u32>() as u16;
    memcpy(
        nla_data(na),
        &id as *const u32 as *const c_void,
        size_of::<u32>(),
    );
    req.nlh.nlmsg_len = nlmsg_align(req.nlh.nlmsg_len) + nla_align((*na).nla_len) as u32;

    ret = send_request(fd, &req as *const request as *const c_void, req.nlh.nlmsg_len as c_int);
    if ret != 0 {
        return ret;
    }

    len = recv(fd, resp.as_mut_ptr() as *mut c_void, resp.len(), 0) as c_int;
    if len < 0 {
        return -errno;
    }

    nlh = resp.as_mut_ptr() as *mut nlmsghdr;
    while nlmsg_ok(nlh, len) {
        if (*nlh).nlmsg_type == NLMSG_ERROR {
            let err = nlmsg_data(nlh) as *mut nlmsgerr;

            return if (*err).error != 0 { (*err).error } else { -ENOENT };
        }

        genl = nlmsg_data(nlh) as *mut genlmsghdr;
        rem = (*nlh).nlmsg_len as c_int - NLMSG_HDRLEN as c_int - GENL_HDRLEN as c_int;
        na = (genl as *mut u8).add(GENL_HDRLEN as usize) as *mut nlattr;
        while nla_ok(na, rem) {
            if (*na).nla_type == TASKSTATS_TYPE_AGGR_PID || (*na).nla_type == TASKSTATS_TYPE_AGGR_TGID {
                nested = nla_data(na) as *mut nlattr;
                nrem = (*na).nla_len as c_int - NLA_HDRLEN as c_int;
                while nla_ok(nested, nrem) {
                    if (*nested).nla_type == TASKSTATS_TYPE_STATS {
                        memcpy(
                            stats as *mut c_void,
                            nla_data(nested),
                            size_of::<taskstats>(),
                        );
                        return 0;
                    }
                    nested = nla_next(nested, &mut nrem);
                }
            }
            na = nla_next(na, &mut rem);
        }

        nlh = nlmsg_next(nlh, &mut len);
    }

    -ENOENT
}

unsafe fn cpu_total(stats: *const taskstats) -> u64 {
    (*stats).ac_utime as u64 + (*stats).ac_stime as u64
}

unsafe fn print_stats(label: *const c_char, stats: *const taskstats) {
    ksft_print_msg(
        c"%s: cpu_total=%llu nvcsw=%llu nivcsw=%llu\n".as_ptr(),
        label,
        cpu_total(stats) as u64,
        (*stats).nvcsw as u64,
        (*stats).nivcsw as u64,
    );
}

unsafe extern "C" fn worker_thread(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut worker_ctx;

    burn_cpu_for_ns(BUSY_NS);

    pthread_mutex_lock(&mut (*ctx).lock);
    (*ctx).ready = true;
    pthread_cond_broadcast(&mut (*ctx).cond);
    while !(*ctx).release {
        pthread_cond_wait(&mut (*ctx).cond, &mut (*ctx).lock);
    }
    pthread_mutex_unlock(&mut (*ctx).lock);

    ptr::null_mut()
}

unsafe fn rust_main() -> c_int {
    let mut ctx: worker_ctx = zeroed();
    let mut before: taskstats = zeroed();
    let mut after: taskstats = zeroed();
    let mut thread: pthread_t = 0;
    let tgid: pid_t = getpid();
    let mut family_id: c_int;
    let fd: c_int;
    let mut ret: c_int;

    ksft_print_header();
    ksft_set_plan(1);

    if geteuid() != 0 {
        ksft_exit_skip(c"taskstats_fill_stats_tgid needs root\n".as_ptr());
    }

    fd = netlink_open();
    if fd < 0 {
        ksft_exit_skip(
            c"failed to open generic netlink socket: %s\n".as_ptr(),
            strerror(-fd),
        );
    }

    family_id = get_family_id(fd, TASKSTATS_GENL_NAME);
    if family_id < 0 {
        ksft_exit_skip(
            c"taskstats generic netlink family unavailable: %s\n".as_ptr(),
            strerror(-family_id),
        );
    }

    /* Create worker thread that burns 200ms of CPU */
    if pthread_create(
        &mut thread,
        ptr::null(),
        worker_thread,
        &mut ctx as *mut worker_ctx as *mut c_void,
    ) != 0
    {
        ksft_exit_fail_msg(c"pthread_create failed: %s\n".as_ptr(), strerror(errno));
    }

    /* Wait for worker to finish generating activity */
    pthread_mutex_lock(&mut ctx.lock);
    while !ctx.ready {
        pthread_cond_wait(&mut ctx.cond, &mut ctx.lock);
    }
    pthread_mutex_unlock(&mut ctx.lock);

    /*
     * Snapshot A: TGID stats while worker is alive and sleeping.
     * Contains main thread + worker contributions.
     */
    ret = get_taskstats(
        fd,
        family_id,
        TASKSTATS_CMD_ATTR_TGID,
        tgid as u32,
        &mut before,
    );
    if ret != 0 {
        ksft_exit_fail_msg(
            c"TGID query before exit failed: %s\n".as_ptr(),
            strerror(-ret),
        );
    }

    /* Release worker so it can exit, then join (deterministic wait).
     *
     * Kernel exit path ordering guarantees:
     *   do_exit()
     *     taskstats_exit() -> fill_tgid_exit()  (accumulates worker into signal->stats)
     *     exit_notify()                         (releases the thread)
     *     do_task_dead() -> __schedule()        (wakes joiner)
     *
     * So pthread_join() returns only after fill_tgid_exit() has completed.
     */
    pthread_mutex_lock(&mut ctx.lock);
    ctx.release = true;
    pthread_cond_broadcast(&mut ctx.cond);
    pthread_mutex_unlock(&mut ctx.lock);

    pthread_join(thread, ptr::null_mut());

    /*
     * Snapshot B: TGID stats after worker has exited.
     * fill_stats_for_tgid() does:
     *   memcpy(signal->stats)   <- includes fill_tgid_exit accumulation
     *   + scan live threads      <- only main thread now
     */
    ret = get_taskstats(
        fd,
        family_id,
        TASKSTATS_CMD_ATTR_TGID,
        tgid as u32,
        &mut after,
    );
    if ret != 0 {
        ksft_exit_fail_msg(
            c"TGID query after exit failed: %s\n".as_ptr(),
            strerror(-ret),
        );
    }

    print_stats(c"TGID before worker exit".as_ptr(), &before);
    print_stats(c"TGID after  worker exit".as_ptr(), &after);

    /*
     * The worker burned 200ms of CPU before the first snapshot.
     * If the kernel correctly retained its contribution via
     * fill_tgid_exit(), then the TGID CPU total after exit must be at
     * least as large as the TGID CPU total before exit.
     */
    ksft_test_result(
        cpu_total(&after) >= cpu_total(&before),
        c"TGID CPU stats should not regress after thread exit\n".as_ptr(),
    );

    close(fd);
    ksft_finished();
    if ksft_get_fail_cnt() != 0 {
        KSFT_FAIL
    } else {
        KSFT_PASS
    }
}

fn main() {
    unsafe {
        let status = rust_main();
        if status != 0 {
            core::process::exit(status);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// <asm/unistd.h>, <linux/time_types.h>, <poll.h>, <unistd.h>, <assert.h>,
// <signal.h>, <pthread.h>, <sys/epoll.h>, <sys/socket.h>, <sys/eventfd.h>,
// and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long, c_short, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

type size_t = usize;
type ssize_t = isize;
type pthread_t = c_ulong;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
#[derive(Copy, Clone)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct __kernel_timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct epoll_mtcontext {
    efd: [c_int; 3],
    sfd: [c_int; 4],
    count: c_int,
    main: pthread_t,
    waiter: pthread_t,
}

#[repr(C)]
struct epoll60_ctx {
    stopped: c_int,
    ready: c_int,
    waiters: c_int,
    epfd: c_int,
    evfd: [c_int; EPOLL60_EVENTS_NR],
}

#[repr(C)]
struct epoll61_ctx {
    epfd: c_int,
    evfd: c_int,
}

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const POLLIN: c_short = 0x001;
const SIGUSR1: c_int = 10;
const SIG_SETMASK: c_int = 2;
const EINTR: c_int = 4;
const ENODATA: usize = 61;
const CLOCK_REALTIME: c_int = 0;
const EFD_NONBLOCK: c_int = 0x800;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_MOD: c_int = 3;
const EPOLLIN: u32 = 0x001;
const EPOLLERR: u32 = 0x008;
const EPOLLHUP: u32 = 0x010;
const EPOLLET: u32 = 1u32 << 31;
const __NR_epoll_pwait2: c_long = -1;
const EPOLL60_EVENTS_NR: usize = 10;

unsafe extern "C" {
    static mut errno: c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn usleep(usec: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn epoll_pwait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int, sigmask: *const sigset_t) -> c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_tryjoin_np(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
}

macro_rules! ASSERT_EQ {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b)
    };
}
macro_rules! ASSERT_GE {
    ($a:expr, $b:expr) => {
        assert!($a >= $b)
    };
}
macro_rules! ASSERT_GT {
    ($a:expr, $b:expr) => {
        assert!($a > $b)
    };
}
macro_rules! EXPECT_EQ {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b)
    };
}
macro_rules! EXPECT_GE {
    ($a:expr, $b:expr) => {
        assert!($a >= $b)
    };
}
macro_rules! EXPECT_TRUE {
    ($a:expr) => {
        assert!($a)
    };
}

unsafe fn atomic_fetch_add_int(p: *mut c_int, v: c_int) -> c_int {
    (*(p as *mut AtomicI32)).fetch_add(v, Ordering::SeqCst)
}

unsafe fn atomic_fetch_or_int(p: *mut c_int, v: c_int) -> c_int {
    (*(p as *mut AtomicI32)).fetch_or(v, Ordering::SeqCst)
}

unsafe fn atomic_fetch_sub_int(p: *mut c_int, v: c_int) -> c_int {
    (*(p as *mut AtomicI32)).fetch_sub(v, Ordering::Acquire)
}

unsafe fn atomic_load_int(p: *const c_int) -> c_int {
    (*(p as *const AtomicI32)).load(Ordering::Acquire)
}

unsafe fn zeroed<T>() -> T {
    mem::zeroed()
}

unsafe fn sys_epoll_pwait2(
    fd: c_int,
    events: *mut epoll_event,
    maxevents: c_int,
    timeout: *const __kernel_timespec,
    sigset: *const sigset_t,
    sigsetsize: size_t,
) -> c_int {
    syscall(__NR_epoll_pwait2, fd, events, maxevents, timeout, sigset, sigsetsize) as c_int
}

unsafe extern "C" fn signal_handler(_signum: c_int) {}

unsafe fn kill_timeout(ctx: *mut epoll_mtcontext) {
    usleep(1000000);
    pthread_kill((*ctx).main, SIGUSR1);
    pthread_kill((*ctx).waiter, SIGUSR1);
}

unsafe extern "C" fn waiter_entry1a(data: *mut c_void) -> *mut c_void {
    let mut e: epoll_event = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    if epoll_wait((*ctx).efd[0], &mut e, 1, -1) > 0 {
        atomic_fetch_add_int(&mut (*ctx).count, 1);
    }
    ptr::null_mut()
}

unsafe extern "C" fn waiter_entry1ap(data: *mut c_void) -> *mut c_void {
    let mut pfd: pollfd = zeroed();
    let mut e: epoll_event = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    pfd.fd = (*ctx).efd[0];
    pfd.events = POLLIN;
    if poll(&mut pfd, 1, -1) > 0 {
        if epoll_wait((*ctx).efd[0], &mut e, 1, 0) > 0 {
            atomic_fetch_add_int(&mut (*ctx).count, 1);
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn waiter_entry1o(data: *mut c_void) -> *mut c_void {
    let mut e: epoll_event = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    if epoll_wait((*ctx).efd[0], &mut e, 1, -1) > 0 {
        atomic_fetch_or_int(&mut (*ctx).count, 1);
    }
    ptr::null_mut()
}

unsafe extern "C" fn waiter_entry1op(data: *mut c_void) -> *mut c_void {
    let mut pfd: pollfd = zeroed();
    let mut e: epoll_event = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    pfd.fd = (*ctx).efd[0];
    pfd.events = POLLIN;
    if poll(&mut pfd, 1, -1) > 0 {
        if epoll_wait((*ctx).efd[0], &mut e, 1, 0) > 0 {
            atomic_fetch_or_int(&mut (*ctx).count, 1);
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn waiter_entry2a(data: *mut c_void) -> *mut c_void {
    let mut events: [epoll_event; 2] = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    if epoll_wait((*ctx).efd[0], events.as_mut_ptr(), 2, -1) > 0 {
        atomic_fetch_add_int(&mut (*ctx).count, 1);
    }
    ptr::null_mut()
}

unsafe extern "C" fn waiter_entry2ap(data: *mut c_void) -> *mut c_void {
    let mut pfd: pollfd = zeroed();
    let mut events: [epoll_event; 2] = zeroed();
    let ctx = data as *mut epoll_mtcontext;
    pfd.fd = (*ctx).efd[0];
    pfd.events = POLLIN;
    if poll(&mut pfd, 1, -1) > 0 {
        if epoll_wait((*ctx).efd[0], events.as_mut_ptr(), 2, 0) > 0 {
            atomic_fetch_add_int(&mut (*ctx).count, 1);
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn emitter_entry1(data: *mut c_void) -> *mut c_void {
    let ctx = data as *mut epoll_mtcontext;
    usleep(100000);
    write((*ctx).sfd[1], b"w".as_ptr() as *const c_void, 1);
    kill_timeout(ctx);
    ptr::null_mut()
}

unsafe extern "C" fn emitter_entry2(data: *mut c_void) -> *mut c_void {
    let ctx = data as *mut epoll_mtcontext;
    usleep(100000);
    write((*ctx).sfd[1], b"w".as_ptr() as *const c_void, 1);
    write((*ctx).sfd[3], b"w".as_ptr() as *const c_void, 1);
    kill_timeout(ctx);
    ptr::null_mut()
}

unsafe fn finish_emitter(emitter: pthread_t) {
    if pthread_tryjoin_np(emitter, ptr::null_mut()) < 0 {
        pthread_kill(emitter, SIGUSR1);
        pthread_join(emitter, ptr::null_mut());
    }
}

unsafe fn close_all(fds: &[c_int]) {
    for &fd in fds {
        close(fd);
    }
}

unsafe fn simple_socket_epoll(two: bool, ctl_et: bool, via_poll: bool, expect2: c_int) {
    let mut efd: c_int;
    let mut sfd = [0; 4];
    let mut events: [epoll_event; 2] = zeroed();
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr()), 0);
    if two {
        ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr().add(2)), 0);
    }
    efd = epoll_create(1);
    ASSERT_GE!(efd, 0);
    events[0].events = EPOLLIN | if ctl_et { EPOLLET } else { 0 };
    ASSERT_EQ!(epoll_ctl(efd, EPOLL_CTL_ADD, sfd[0], events.as_mut_ptr()), 0);
    if two {
        events[0].events = EPOLLIN | if ctl_et { EPOLLET } else { 0 };
        ASSERT_EQ!(epoll_ctl(efd, EPOLL_CTL_ADD, sfd[2], events.as_mut_ptr()), 0);
    }
    ASSERT_EQ!(write(sfd[1], b"w".as_ptr() as *const c_void, 1), 1);
    if two {
        ASSERT_EQ!(write(sfd[3], b"w".as_ptr() as *const c_void, 1), 1);
    }
    if via_poll {
        let mut pfd: pollfd = zeroed();
        pfd.fd = efd;
        pfd.events = POLLIN;
        EXPECT_EQ!(poll(&mut pfd, 1, 0), 1);
    }
    EXPECT_EQ!(epoll_wait(efd, events.as_mut_ptr(), if two { 2 } else { 1 }, 0), if two { 2 } else { 1 });
    if via_poll {
        let mut pfd: pollfd = zeroed();
        pfd.fd = efd;
        pfd.events = POLLIN;
        EXPECT_EQ!(poll(&mut pfd, 1, 0), expect2);
    }
    EXPECT_EQ!(epoll_wait(efd, events.as_mut_ptr(), if two { 2 } else { 1 }, 0), expect2 * if two { 2 } else { 1 });
    close(efd);
    close_all(if two { &sfd[..4] } else { &sfd[..2] });
}

/*          t0
 *           | (ew)
 *          e0
 *           | (lt)
 *          s0
 */
unsafe fn epoll1() { simple_socket_epoll(false, false, false, 1); }

/*          t0
 *           | (ew)
 *          e0
 *           | (et)
 *          s0
 */
unsafe fn epoll2() { simple_socket_epoll(false, true, false, 0); }

/*           t0
 *            | (ew)
 *           e0
 *     (lt) /  \ (lt)
 *        s0    s2
 */
unsafe fn epoll3() { simple_socket_epoll(true, false, false, 1); }

/*           t0
 *            | (ew)
 *           e0
 *     (et) /  \ (et)
 *        s0    s2
 */
unsafe fn epoll4() { simple_socket_epoll(true, true, false, 0); }

/*          t0
 *           | (p)
 *          e0
 *           | (lt)
 *          s0
 */
unsafe fn epoll5() { simple_socket_epoll(false, false, true, 1); }

/*          t0
 *           | (p)
 *          e0
 *           | (et)
 *          s0
 */
unsafe fn epoll6() { simple_socket_epoll(false, true, true, 0); }

/*           t0
 *            | (p)
 *           e0
 *     (lt) /  \ (lt)
 *        s0    s2
 */
unsafe fn epoll7() { simple_socket_epoll(true, false, true, 1); }

/*           t0
 *            | (p)
 *           e0
 *     (et) /  \ (et)
 *        s0    s2
 */
unsafe fn epoll8() { simple_socket_epoll(true, true, true, 0); }

unsafe fn threaded_socket_epoll(
    two_sockets: bool,
    nested: bool,
    split_top_wait: bool,
    ctl1_et: bool,
    ctl0_et: bool,
    waiter: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    emitter_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    main_poll: bool,
    main_fd_index: usize,
    main_or: c_int,
    expected: c_int,
    expected_alt: c_int,
) {
    let mut emitter: pthread_t = 0;
    let mut e: epoll_event = zeroed();
    let mut events: [epoll_event; 2] = zeroed();
    let mut ctx: epoll_mtcontext = zeroed();
    signal(SIGUSR1, Some(signal_handler));
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, ctx.sfd.as_mut_ptr()), 0);
    if two_sockets {
        ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, ctx.sfd.as_mut_ptr().add(2)), 0);
    }
    ctx.efd[0] = epoll_create(1);
    ASSERT_GE!(ctx.efd[0], 0);
    if nested {
        ctx.efd[1] = epoll_create(1);
        ASSERT_GE!(ctx.efd[1], 0);
        e.events = EPOLLIN | if ctl1_et { EPOLLET } else { 0 };
        ASSERT_EQ!(epoll_ctl(ctx.efd[1], EPOLL_CTL_ADD, ctx.sfd[0], &mut e), 0);
        if two_sockets {
            ctx.efd[2] = epoll_create(1);
            ASSERT_GE!(ctx.efd[2], 0);
            e.events = EPOLLIN;
            ASSERT_EQ!(epoll_ctl(ctx.efd[2], EPOLL_CTL_ADD, ctx.sfd[2], &mut e), 0);
        }
        e.events = EPOLLIN | if ctl0_et { EPOLLET } else { 0 };
        ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.efd[1], &mut e), 0);
        if two_sockets {
            e.events = EPOLLIN | if ctl0_et { EPOLLET } else { 0 };
            ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.efd[2], &mut e), 0);
        }
    } else {
        events[0].events = EPOLLIN | if ctl0_et { EPOLLET } else { 0 };
        ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.sfd[0], events.as_mut_ptr()), 0);
        if two_sockets {
            events[0].events = EPOLLIN | if ctl0_et { EPOLLET } else { 0 };
            ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.sfd[2], events.as_mut_ptr()), 0);
        }
    }
    ctx.main = pthread_self();
    ASSERT_EQ!(pthread_create(&mut ctx.waiter, ptr::null(), waiter, &mut ctx as *mut _ as *mut c_void), 0);
    ASSERT_EQ!(pthread_create(&mut emitter, ptr::null(), emitter_fn, &mut ctx as *mut _ as *mut c_void), 0);
    if main_poll {
        let mut pfd: pollfd = zeroed();
        pfd.fd = ctx.efd[main_fd_index];
        pfd.events = POLLIN;
        if poll(&mut pfd, 1, -1) > 0 {
            if epoll_wait(ctx.efd[main_fd_index], &mut e, 1, 0) > 0 {
                if main_or != 0 { atomic_fetch_or_int(&mut ctx.count, main_or); } else { atomic_fetch_add_int(&mut ctx.count, 1); }
            }
        }
    } else if split_top_wait {
        if epoll_wait(ctx.efd[1], &mut e, 1, -1) > 0 {
            if main_or != 0 { atomic_fetch_or_int(&mut ctx.count, main_or); } else { atomic_fetch_add_int(&mut ctx.count, 1); }
        }
    } else if two_sockets {
        if epoll_wait(ctx.efd[0], events.as_mut_ptr(), if ctl0_et { 1 } else { 2 }, -1) > 0 {
            atomic_fetch_add_int(&mut ctx.count, 1);
        }
    } else if epoll_wait(ctx.efd[0], &mut e, 1, -1) > 0 {
        atomic_fetch_add_int(&mut ctx.count, 1);
    }
    ASSERT_EQ!(pthread_join(ctx.waiter, ptr::null_mut()), 0);
    if expected_alt >= 0 {
        EXPECT_TRUE!(ctx.count == expected || ctx.count == expected_alt);
    } else {
        EXPECT_EQ!(ctx.count, expected);
    }
    finish_emitter(emitter);
    close(ctx.efd[0]);
    if nested { close(ctx.efd[1]); }
    if nested && two_sockets { close(ctx.efd[2]); }
    close(ctx.sfd[0]);
    close(ctx.sfd[1]);
    if two_sockets {
        close(ctx.sfd[2]);
        close(ctx.sfd[3]);
    }
}

unsafe fn epoll9() { threaded_socket_epoll(false, false, false, false, false, waiter_entry1a, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll10() { threaded_socket_epoll(false, false, false, false, true, waiter_entry1a, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll11() { threaded_socket_epoll(true, false, false, false, false, waiter_entry2a, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll12() { threaded_socket_epoll(true, false, false, false, true, waiter_entry1a, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll13() { threaded_socket_epoll(false, false, false, false, false, waiter_entry1ap, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll14() { threaded_socket_epoll(false, false, false, false, true, waiter_entry1ap, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll15() { threaded_socket_epoll(true, false, false, false, false, waiter_entry2ap, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll16() { threaded_socket_epoll(true, false, false, false, true, waiter_entry1ap, emitter_entry2, false, 0, 0, 2, -1); }

unsafe fn nested_simple_epoll(top_et: bool, leaf_et: bool, via_poll: bool, expect2: c_int) {
    let mut efd = [0; 3];
    let mut sfd = [0; 4];
    let mut e: epoll_event = zeroed();
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr()), 0);
    efd[0] = epoll_create(1); ASSERT_GE!(efd[0], 0);
    efd[1] = epoll_create(1); ASSERT_GE!(efd[1], 0);
    e.events = EPOLLIN | if leaf_et { EPOLLET } else { 0 };
    ASSERT_EQ!(epoll_ctl(efd[1], EPOLL_CTL_ADD, sfd[0], &mut e), 0);
    e.events = EPOLLIN | if top_et { EPOLLET } else { 0 };
    ASSERT_EQ!(epoll_ctl(efd[0], EPOLL_CTL_ADD, efd[1], &mut e), 0);
    ASSERT_EQ!(write(sfd[1], b"w".as_ptr() as *const c_void, 1), 1);
    if via_poll { let mut pfd: pollfd = zeroed(); pfd.fd = efd[0]; pfd.events = POLLIN; EXPECT_EQ!(poll(&mut pfd, 1, 0), 1); }
    EXPECT_EQ!(epoll_wait(efd[0], &mut e, 1, 0), 1);
    if via_poll { let mut pfd: pollfd = zeroed(); pfd.fd = efd[0]; pfd.events = POLLIN; EXPECT_EQ!(poll(&mut pfd, 1, 0), expect2); }
    EXPECT_EQ!(epoll_wait(efd[0], &mut e, 1, 0), expect2);
    close_all(&efd[..2]);
    close_all(&sfd[..2]);
}

unsafe fn epoll17() { nested_simple_epoll(false, false, false, 1); }
unsafe fn epoll18() { nested_simple_epoll(false, true, false, 1); }
unsafe fn epoll19() { nested_simple_epoll(true, false, false, 0); }
unsafe fn epoll20() { nested_simple_epoll(true, true, false, 0); }
unsafe fn epoll21() { nested_simple_epoll(false, false, true, 1); }
unsafe fn epoll22() { nested_simple_epoll(false, true, true, 1); }
unsafe fn epoll23() { nested_simple_epoll(true, false, true, 0); }
unsafe fn epoll24() { nested_simple_epoll(true, true, true, 0); }

unsafe fn epoll25() { threaded_socket_epoll(false, true, false, false, false, waiter_entry1a, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll26() { threaded_socket_epoll(false, true, false, true, false, waiter_entry1a, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll27() { threaded_socket_epoll(false, true, false, false, true, waiter_entry1a, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll28() { threaded_socket_epoll(false, true, false, true, true, waiter_entry1a, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll29() { threaded_socket_epoll(false, true, false, false, false, waiter_entry1ap, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll30() { threaded_socket_epoll(false, true, false, true, false, waiter_entry1ap, emitter_entry1, false, 0, 0, 2, -1); }
unsafe fn epoll31() { threaded_socket_epoll(false, true, false, false, true, waiter_entry1ap, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll32() { threaded_socket_epoll(false, true, false, true, true, waiter_entry1ap, emitter_entry1, false, 0, 0, 1, -1); }
unsafe fn epoll33() { threaded_socket_epoll(false, true, true, false, false, waiter_entry1a, emitter_entry1, false, 1, 0, 2, -1); }
unsafe fn epoll34() { threaded_socket_epoll(false, true, true, true, false, waiter_entry1o, emitter_entry1, false, 1, 2, 2, 3); }
unsafe fn epoll35() { threaded_socket_epoll(false, true, true, false, true, waiter_entry1a, emitter_entry1, false, 1, 0, 2, -1); }
unsafe fn epoll36() { threaded_socket_epoll(false, true, true, true, true, waiter_entry1o, emitter_entry1, false, 1, 2, 2, 3); }
unsafe fn epoll37() { threaded_socket_epoll(false, true, false, false, false, waiter_entry1a, emitter_entry1, true, 1, 0, 2, -1); }
unsafe fn epoll38() { threaded_socket_epoll(false, true, false, true, false, waiter_entry1o, emitter_entry1, true, 1, 2, 2, 3); }
unsafe fn epoll39() { threaded_socket_epoll(false, true, false, false, true, waiter_entry1a, emitter_entry1, true, 1, 0, 2, -1); }
unsafe fn epoll40() { threaded_socket_epoll(false, true, false, true, true, waiter_entry1o, emitter_entry1, true, 1, 2, 2, 3); }
unsafe fn epoll41() { threaded_socket_epoll(false, true, true, false, false, waiter_entry1ap, emitter_entry1, false, 1, 0, 2, -1); }
unsafe fn epoll42() { threaded_socket_epoll(false, true, true, true, false, waiter_entry1op, emitter_entry1, false, 1, 2, 2, 3); }
unsafe fn epoll43() { threaded_socket_epoll(false, true, true, false, true, waiter_entry1ap, emitter_entry1, false, 1, 0, 2, -1); }
unsafe fn epoll44() { threaded_socket_epoll(false, true, true, true, true, waiter_entry1op, emitter_entry1, false, 1, 2, 2, 3); }
unsafe fn epoll45() { threaded_socket_epoll(false, true, false, false, false, waiter_entry1ap, emitter_entry1, true, 1, 0, 2, -1); }
unsafe fn epoll46() { threaded_socket_epoll(false, true, true, true, false, waiter_entry1op, emitter_entry1, false, 1, 2, 2, 3); }
unsafe fn epoll47() { threaded_socket_epoll(false, true, false, false, true, waiter_entry1ap, emitter_entry1, true, 1, 0, 2, -1); }
unsafe fn epoll48() { threaded_socket_epoll(false, true, true, true, true, waiter_entry1op, emitter_entry1, false, 1, 2, 2, 3); }

unsafe fn nested_two_simple_epoll(top_et: bool, via_poll: bool, expect2: c_int) {
    let mut efd = [0; 3];
    let mut sfd = [0; 4];
    let mut events: [epoll_event; 2] = zeroed();
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr()), 0);
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr().add(2)), 0);
    for i in 0..3 { efd[i] = epoll_create(1); ASSERT_GE!(efd[i], 0); }
    events[0].events = EPOLLIN; ASSERT_EQ!(epoll_ctl(efd[1], EPOLL_CTL_ADD, sfd[0], events.as_mut_ptr()), 0);
    events[0].events = EPOLLIN; ASSERT_EQ!(epoll_ctl(efd[2], EPOLL_CTL_ADD, sfd[2], events.as_mut_ptr()), 0);
    events[0].events = EPOLLIN | if top_et { EPOLLET } else { 0 }; ASSERT_EQ!(epoll_ctl(efd[0], EPOLL_CTL_ADD, efd[1], events.as_mut_ptr()), 0);
    events[0].events = EPOLLIN | if top_et { EPOLLET } else { 0 }; ASSERT_EQ!(epoll_ctl(efd[0], EPOLL_CTL_ADD, efd[2], events.as_mut_ptr()), 0);
    ASSERT_EQ!(write(sfd[1], b"w".as_ptr() as *const c_void, 1), 1);
    ASSERT_EQ!(write(sfd[3], b"w".as_ptr() as *const c_void, 1), 1);
    if via_poll { let mut pfd: pollfd = zeroed(); pfd.fd = efd[0]; pfd.events = POLLIN; EXPECT_EQ!(poll(&mut pfd, 1, 0), 1); }
    EXPECT_EQ!(epoll_wait(efd[0], events.as_mut_ptr(), 2, 0), 2);
    if via_poll { let mut pfd: pollfd = zeroed(); pfd.fd = efd[0]; pfd.events = POLLIN; EXPECT_EQ!(poll(&mut pfd, 1, 0), expect2); }
    EXPECT_EQ!(epoll_wait(efd[0], events.as_mut_ptr(), 2, 0), expect2 * 2);
    close_all(&efd);
    close_all(&sfd);
}

unsafe fn epoll49() { nested_two_simple_epoll(false, false, 1); }
unsafe fn epoll50() { nested_two_simple_epoll(true, false, 0); }
unsafe fn epoll51() { nested_two_simple_epoll(false, true, 1); }
unsafe fn epoll52() { nested_two_simple_epoll(true, true, 0); }
unsafe fn epoll53() { threaded_socket_epoll(true, true, false, false, false, waiter_entry1a, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll54() { threaded_socket_epoll(true, true, false, false, true, waiter_entry1a, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll55() { threaded_socket_epoll(true, true, false, false, false, waiter_entry1ap, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll56() { threaded_socket_epoll(true, true, false, false, true, waiter_entry1ap, emitter_entry2, false, 0, 0, 2, -1); }
unsafe fn epoll57() { threaded_socket_epoll(true, true, false, false, false, waiter_entry1ap, emitter_entry2, true, 0, 0, 2, -1); }
unsafe fn epoll58() { threaded_socket_epoll(true, true, false, false, true, waiter_entry1ap, emitter_entry2, true, 0, 0, 2, -1); }

unsafe extern "C" fn epoll59_thread(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll_mtcontext;
    let mut e: epoll_event = zeroed();
    let mut i = 0;
    while i < 100000 {
        while (*ctx).count == 0 {}
        e.events = EPOLLIN | EPOLLERR | EPOLLET;
        epoll_ctl((*ctx).efd[0], EPOLL_CTL_MOD, (*ctx).sfd[0], &mut e);
        (*ctx).count = 0;
        i += 1;
    }
    ptr::null_mut()
}

/*        t0
 *      (p) \
 *           e0
 *     (et) /
 *        e0
 *
 * Based on https://bugzilla.kernel.org/show_bug.cgi?id=205933
 */
unsafe fn epoll59() {
    let mut emitter: pthread_t = 0;
    let mut e: epoll_event = zeroed();
    let mut ctx: epoll_mtcontext = zeroed();
    signal(SIGUSR1, Some(signal_handler));
    ctx.efd[0] = epoll_create1(0);
    ASSERT_GE!(ctx.efd[0], 0);
    ctx.sfd[0] = eventfd(1, 0);
    ASSERT_GE!(ctx.sfd[0], 0);
    e.events = EPOLLIN | EPOLLERR | EPOLLET;
    ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.sfd[0], &mut e), 0);
    ASSERT_EQ!(pthread_create(&mut emitter, ptr::null(), epoll59_thread, &mut ctx as *mut _ as *mut c_void), 0);
    for _i in 0..100000 {
        let ret = epoll_wait(ctx.efd[0], &mut e, 1, 1000);
        ASSERT_GT!(ret, 0);
        while ctx.count != 0 {}
        ctx.count = 1;
    }
    finish_emitter(emitter);
    close(ctx.efd[0]);
    close(ctx.sfd[0]);
}

unsafe extern "C" fn epoll60_wait_thread(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll60_ctx;
    let mut e: epoll_event = zeroed();
    let mut sigmask: sigset_t = zeroed();
    let mut v: u64 = 0;
    sigemptyset(&mut sigmask);
    sigaddset(&mut sigmask, SIGUSR1);
    sigprocmask(SIG_SETMASK, &sigmask, ptr::null_mut());
    sigemptyset(&mut sigmask);
    while (*ctx).stopped == 0 {
        atomic_fetch_add_int(&mut (*ctx).ready, 1);
        while atomic_load_int(&(*ctx).ready) != 0 && (*ctx).stopped == 0 {}
        atomic_fetch_add_int(&mut (*ctx).waiters, 1);
        let ret = epoll_pwait((*ctx).epfd, &mut e, 1, 2000, &sigmask);
        if ret != 1 {
            assert!(ret < 0 && errno == EINTR && (*ctx).stopped != 0, "Lost wakeup!\n");
            break;
        }
        let ret = read(e.data.fd, &mut v as *mut _ as *mut c_void, mem::size_of_val(&v));
        assert_eq!(ret, mem::size_of_val(&v) as isize);
        atomic_fetch_sub_int(&mut (*ctx).waiters, 1);
    }
    ptr::null_mut()
}

unsafe fn msecs() -> u64 {
    let mut ts: timespec = zeroed();
    clock_gettime(CLOCK_REALTIME, &mut ts);
    (ts.tv_sec as u64) * 1000u64 + (ts.tv_nsec as u64) / 1000000u64
}

unsafe fn count_waiters(ctx: *mut epoll60_ctx) -> c_int {
    atomic_load_int(&(*ctx).waiters)
}

unsafe fn epoll60() {
    let mut ctx: epoll60_ctx = zeroed();
    let mut waiters = [0 as pthread_t; EPOLL60_EVENTS_NR];
    let mut e: epoll_event = zeroed();
    signal(SIGUSR1, Some(signal_handler));
    ctx.epfd = epoll_create1(0);
    ASSERT_GE!(ctx.epfd, 0);
    for i in 0..ctx.evfd.len() {
        ctx.evfd[i] = eventfd(0, EFD_NONBLOCK);
        ASSERT_GE!(ctx.evfd[i], 0);
        e.events = EPOLLIN | EPOLLET;
        e.data.fd = ctx.evfd[i];
        ASSERT_EQ!(epoll_ctl(ctx.epfd, EPOLL_CTL_ADD, ctx.evfd[i], &mut e), 0);
    }
    for i in 0..waiters.len() {
        ASSERT_EQ!(pthread_create(&mut waiters[i], ptr::null(), epoll60_wait_thread, &mut ctx as *mut _ as *mut c_void), 0);
    }
    for _i in 0..300 {
        let v: u64 = 1;
        while atomic_load_int(&ctx.ready) != ctx.evfd.len() as c_int {}
        atomic_fetch_sub_int(&mut ctx.ready, ctx.evfd.len() as c_int);
        while count_waiters(&mut ctx) != ctx.evfd.len() as c_int {}
        usleep(1000);
        for n in 0..ctx.evfd.len() {
            let ret = write(ctx.evfd[n], &v as *const _ as *const c_void, mem::size_of_val(&v));
            ASSERT_EQ!(ret, mem::size_of_val(&v) as isize);
        }
        let ms = msecs();
        while count_waiters(&mut ctx) != 0 && msecs() < ms + 1000 {}
        ASSERT_EQ!(count_waiters(&mut ctx), 0);
    }
    ctx.stopped = 1;
    for i in 0..waiters.len() {
        let _ret = pthread_kill(waiters[i], SIGUSR1);
    }
    for i in 0..waiters.len() {
        pthread_join(waiters[i], ptr::null_mut());
    }
    for i in 0..waiters.len() {
        close(ctx.evfd[i]);
    }
    close(ctx.epfd);
}

unsafe extern "C" fn epoll61_write_eventfd(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll61_ctx;
    let l: i64 = 1;
    usleep(10950);
    write((*ctx).evfd, &l as *const _ as *const c_void, mem::size_of_val(&l));
    ptr::null_mut()
}

unsafe extern "C" fn epoll61_epoll_with_timeout(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll61_ctx;
    let mut events: [epoll_event; 1] = zeroed();
    let n = epoll_wait((*ctx).epfd, events.as_mut_ptr(), 1, 11);
    /*
     * If epoll returned the eventfd, write on the eventfd to wake up the
     * blocking poller.
     */
    if n == 1 {
        let l: i64 = 1;
        write((*ctx).evfd, &l as *const _ as *const c_void, mem::size_of_val(&l));
    }
    ptr::null_mut()
}

unsafe extern "C" fn epoll61_blocking_epoll(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll61_ctx;
    let mut events: [epoll_event; 1] = zeroed();
    epoll_wait((*ctx).epfd, events.as_mut_ptr(), 1, -1);
    ptr::null_mut()
}

unsafe fn epoll61() {
    let mut ctx: epoll61_ctx = zeroed();
    let mut ev: epoll_event = zeroed();
    ctx.epfd = epoll_create1(0);
    ASSERT_GE!(ctx.epfd, 0);
    ctx.evfd = eventfd(0, EFD_NONBLOCK);
    ASSERT_GE!(ctx.evfd, 0);
    ev.events = EPOLLIN | EPOLLET | EPOLLERR | EPOLLHUP;
    ev.data.ptr = ptr::null_mut();
    let r = epoll_ctl(ctx.epfd, EPOLL_CTL_ADD, ctx.evfd, &mut ev);
    ASSERT_EQ!(r, 0);
    /*
     * We are testing a race.  Repeat the test case 1000 times to make it
     * more likely to fail in case of a bug.
     */
    for _i in 0..1000 {
        let mut threads = [0 as pthread_t; 3];
        ASSERT_EQ!(pthread_create(&mut threads[0], ptr::null(), epoll61_write_eventfd, &mut ctx as *mut _ as *mut c_void), 0);
        ASSERT_EQ!(pthread_create(&mut threads[1], ptr::null(), epoll61_epoll_with_timeout, &mut ctx as *mut _ as *mut c_void), 0);
        ASSERT_EQ!(pthread_create(&mut threads[2], ptr::null(), epoll61_blocking_epoll, &mut ctx as *mut _ as *mut c_void), 0);
        for n in 0..threads.len() {
            ASSERT_EQ!(pthread_join(threads[n], ptr::null_mut()), 0);
        }
    }
    close(ctx.epfd);
    close(ctx.evfd);
}

/* Equivalent to basic test epoll1, but exercising epoll_pwait2. */
unsafe fn epoll62() {
    let mut efd: c_int;
    let mut sfd = [0; 2];
    let mut e: epoll_event = zeroed();
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr()), 0);
    efd = epoll_create(1);
    ASSERT_GE!(efd, 0);
    e.events = EPOLLIN;
    ASSERT_EQ!(epoll_ctl(efd, EPOLL_CTL_ADD, sfd[0], &mut e), 0);
    ASSERT_EQ!(write(sfd[1], b"w".as_ptr() as *const c_void, 1), 1);
    EXPECT_EQ!(sys_epoll_pwait2(efd, &mut e, 1, ptr::null(), ptr::null(), 0), 1);
    EXPECT_EQ!(sys_epoll_pwait2(efd, &mut e, 1, ptr::null(), ptr::null(), 0), 1);
    close(efd);
    close_all(&sfd);
}

/* Epoll_pwait2 basic timeout test. */
unsafe fn epoll63() {
    let cfg_delay_ms: c_int = 10;
    let mut ts: __kernel_timespec = zeroed();
    let mut sfd = [0; 2];
    let mut e: epoll_event = zeroed();
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sfd.as_mut_ptr()), 0);
    let efd = epoll_create(1);
    ASSERT_GE!(efd, 0);
    e.events = EPOLLIN;
    ASSERT_EQ!(epoll_ctl(efd, EPOLL_CTL_ADD, sfd[0], &mut e), 0);
    ts.tv_sec = 0;
    ts.tv_nsec = (cfg_delay_ms * 1000 * 1000) as i64;
    let mut tdiff = msecs();
    EXPECT_EQ!(sys_epoll_pwait2(efd, &mut e, 1, &ts, ptr::null(), 0), 0);
    tdiff = msecs() - tdiff;
    EXPECT_GE!(tdiff, cfg_delay_ms as u64);
    close(efd);
    close_all(&sfd);
}

unsafe fn epoll64() {
    let mut waiter = [0 as pthread_t; 2];
    let mut e: epoll_event = zeroed();
    let mut ctx: epoll_mtcontext = zeroed();
    signal(SIGUSR1, Some(signal_handler));
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, ctx.sfd.as_mut_ptr()), 0);
    ctx.efd[0] = epoll_create(1);
    ASSERT_GE!(ctx.efd[0], 0);
    e.events = EPOLLIN;
    ASSERT_EQ!(epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.sfd[0], &mut e), 0);
    /*
     * main will act as the emitter once both waiter threads are
     * blocked and expects to both be awoken upon the ready event.
     */
    ctx.main = pthread_self();
    ASSERT_EQ!(pthread_create(&mut waiter[0], ptr::null(), waiter_entry1a, &mut ctx as *mut _ as *mut c_void), 0);
    ASSERT_EQ!(pthread_create(&mut waiter[1], ptr::null(), waiter_entry1a, &mut ctx as *mut _ as *mut c_void), 0);
    usleep(100000);
    ASSERT_EQ!(write(ctx.sfd[1], b"w".as_ptr() as *const c_void, 1), 1);
    ASSERT_EQ!(pthread_join(waiter[0], ptr::null_mut()), 0);
    ASSERT_EQ!(pthread_join(waiter[1], ptr::null_mut()), 0);
    EXPECT_EQ!(ctx.count, 2);
    close(ctx.efd[0]);
    close(ctx.sfd[0]);
    close(ctx.sfd[1]);
}

unsafe extern "C" fn epoll65_wait(ctx_: *mut c_void) -> *mut c_void {
    let ctx = ctx_ as *mut epoll_mtcontext;
    let mut event: epoll_event = zeroed();
    for _i in 0..100000 {
        if epoll_wait((*ctx).efd[0], &mut event, 1, 0) == 0 {
            return ENODATA as *mut c_void;
        }
    }
    0usize as *mut c_void
}

unsafe fn epoll65() {
    let mut ctx: epoll_mtcontext = zeroed();
    let mut event: epoll_event = zeroed();
    let dummy_data: i64 = 99;
    let mut threads = [0 as pthread_t; 64];
    ctx.efd[0] = epoll_create(1);
    ASSERT_GE!(ctx.efd[0], 0);
    ctx.efd[1] = eventfd(0, 0);
    ASSERT_GE!(ctx.efd[1], 0);
    event.events = EPOLLIN;
    let err = epoll_ctl(ctx.efd[0], EPOLL_CTL_ADD, ctx.efd[1], &mut event);
    ASSERT_EQ!(err, 0);
    write(ctx.efd[1], &dummy_data as *const _ as *const c_void, mem::size_of_val(&dummy_data));
    for i in 0..threads.len() {
        ASSERT_EQ!(pthread_create(&mut threads[i], ptr::null(), epoll65_wait, &mut ctx as *mut _ as *mut c_void), 0);
    }
    for i in 0..threads.len() {
        let mut ret: *mut c_void = ptr::null_mut();
        ASSERT_EQ!(pthread_join(threads[i], &mut ret), 0);
        ASSERT_EQ!(ret as usize, 0);
    }
    close(ctx.efd[0]);
    close(ctx.efd[1]);
}

unsafe fn epoll66() {
    let mut event: epoll_event = zeroed();
    let mut pfd = [0; 2];
    ASSERT_EQ!(pipe(pfd.as_mut_ptr()), 0);
    let efd = epoll_create1(0);
    ASSERT_GE!(efd, 0);
    event.events = EPOLLIN | EPOLLET;
    ASSERT_EQ!(epoll_ctl(efd, EPOLL_CTL_ADD, pfd[0], &mut event), 0);
    for _i in 0..2 {
        ASSERT_EQ!(write(pfd[1], b"".as_ptr() as *const c_void, 1), 1);
        EXPECT_EQ!(epoll_wait(efd, &mut event, 1, 0), 1);
    }
    close(pfd[0]);
    close(pfd[1]);
    close(efd);
}

fn main() {
    unsafe {
        epoll1(); epoll2(); epoll3(); epoll4(); epoll5(); epoll6(); epoll7(); epoll8();
        epoll9(); epoll10(); epoll11(); epoll12(); epoll13(); epoll14(); epoll15(); epoll16();
        epoll17(); epoll18(); epoll19(); epoll20(); epoll21(); epoll22(); epoll23(); epoll24();
        epoll25(); epoll26(); epoll27(); epoll28(); epoll29(); epoll30(); epoll31(); epoll32();
        epoll33(); epoll34(); epoll35(); epoll36(); epoll37(); epoll38(); epoll39(); epoll40();
        epoll41(); epoll42(); epoll43(); epoll44(); epoll45(); epoll46(); epoll47(); epoll48();
        epoll49(); epoll50(); epoll51(); epoll52(); epoll53(); epoll54(); epoll55(); epoll56();
        epoll57(); epoll58(); epoll59(); epoll60(); epoll61(); epoll62(); epoll63(); epoll64();
        epoll65(); epoll66();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

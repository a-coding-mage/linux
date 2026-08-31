// SPDX-License-Identifier: GPL-2.0
/*
 *
 * sched-messaging.c
 *
 * messaging: Benchmark for scheduler and IPC mechanisms
 *
 * Based on hackbench by Rusty Russell <rusty@rustcorp.com.au>
 * Ported to perf by Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
 *
 */

/* Dependencies originally included from:
 * <subcmd/parse-options.h>, "bench.h", pthread/libc/socket/wait/time/poll,
 * <linux/list.h>, and <linux/time64.h>.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

const DATASIZE: usize = 100;
const EXIT_FAILURE: c_int = 1;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const POLLIN: i16 = 0x0001;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGKILL: c_int = 9;
const PTHREAD_STACK_MIN: usize = 16384;
const USEC_PER_MSEC: c_long = 1000;

static mut use_pipes: bool = false;
static mut nr_loops: c_uint = 100;
static mut thread_mode: bool = false;
static mut num_groups: c_uint = 10;
static mut total_children: c_uint = 0;
static mut sender_contexts: list_head = list_head {
    next: ptr::addr_of_mut!(sender_contexts),
    prev: ptr::addr_of_mut!(sender_contexts),
};
static mut receiver_contexts: list_head = list_head {
    next: ptr::addr_of_mut!(receiver_contexts),
    prev: ptr::addr_of_mut!(receiver_contexts),
};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
struct sender_context {
    list: list_head,
    num_fds: c_uint,
    ready_out: c_int,
    wakefd: c_int,
    out_fds: [c_int; 0],
}

#[repr(C)]
struct receiver_context {
    list: list_head,
    num_packets: c_uint,
    in_fds: [c_int; 2],
    ready_out: c_int,
    wakefd: c_int,
}

#[repr(C)]
union messaging_worker {
    thread: pthread_t,
    pid: pid_t,
}

static mut worker_tab: *mut messaging_worker = ptr::null_mut();

type pthread_t = usize;
type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
struct pthread_attr_t {
    _private: [usize; 7],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct option {
    _private: [usize; 0],
}

unsafe extern "C" {
    static mut bench_format: c_int;
    static BENCH_FORMAT_DEFAULT: c_int;
    static BENCH_FORMAT_SIMPLE: c_int;

    fn pipe(fds: *mut c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...);
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: size_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn printf(format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);

    static mut stderr: *mut c_void;

    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1_000_000;
    }
}

unsafe fn sender_out_fds(ctx: *mut sender_context) -> *mut c_int {
    ptr::addr_of_mut!((*ctx).out_fds) as *mut c_int
}

unsafe fn fdpair(fds: *mut c_int) {
    if use_pipes {
        if pipe(fds) == 0 {
            return;
        }
    } else if socketpair(AF_UNIX, SOCK_STREAM, 0, fds) == 0 {
        return;
    }

    err(
        EXIT_FAILURE,
        if use_pipes {
            c"pipe()".as_ptr()
        } else {
            c"socketpair()".as_ptr()
        },
    );
}

/* Block until we're ready to go */
unsafe fn ready(ready_out: c_int, wakefd: c_int) {
    let mut pollfd = pollfd {
        fd: wakefd,
        events: POLLIN,
        revents: 0,
    };

    /* Tell them we're ready. */
    if write(ready_out, c"R".as_ptr() as *const c_void, 1) != 1 {
        err(EXIT_FAILURE, c"CLIENT: ready write".as_ptr());
    }

    /* Wait for "GO" signal */
    if poll(&mut pollfd, 1, -1) != 1 {
        err(EXIT_FAILURE, c"poll".as_ptr());
    }
}

/* Sender sprays nr_loops messages down each file descriptor */
unsafe extern "C" fn sender(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut sender_context;
    let mut data = [0 as c_char; DATASIZE];
    let mut i: c_uint;
    let mut j: c_uint;

    ready((*ctx).ready_out, (*ctx).wakefd);
    memset(data.as_mut_ptr() as *mut c_void, b'S' as c_int, mem::size_of_val(&data));

    /* Now pump to every receiver. */
    i = 0;
    while i < nr_loops {
        j = 0;
        while j < (*ctx).num_fds {
            let mut ret: c_int;
            let mut done: c_int = 0;

            loop {
                ret = write(
                    *sender_out_fds(ctx).add(j as usize),
                    data.as_mut_ptr().add(done as usize) as *const c_void,
                    DATASIZE - done as usize,
                ) as c_int;
                if ret < 0 {
                    err(EXIT_FAILURE, c"SENDER: write".as_ptr());
                }
                done += ret;
                if done >= DATASIZE as c_int {
                    break;
                }
            }
            j += 1;
        }
        i += 1;
    }

    ptr::null_mut()
}

/* One receiver per fd */
unsafe extern "C" fn receiver(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut receiver_context;
    let mut i: c_uint;

    if !thread_mode {
        close((*ctx).in_fds[1]);
    }

    /* Wait for start... */
    ready((*ctx).ready_out, (*ctx).wakefd);

    /* Receive them all */
    i = 0;
    while i < (*ctx).num_packets {
        let mut data = [0 as c_char; DATASIZE];
        let mut ret: c_int;
        let mut done: c_int = 0;

        loop {
            ret = read(
                (*ctx).in_fds[0],
                data.as_mut_ptr().add(done as usize) as *mut c_void,
                DATASIZE - done as usize,
            ) as c_int;
            if ret < 0 {
                err(EXIT_FAILURE, c"SERVER: read".as_ptr());
            }
            done += ret;
            if done >= DATASIZE as c_int {
                break;
            }
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn create_thread_worker(
    worker: *mut messaging_worker,
    ctx: *mut c_void,
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
) {
    let mut attr: pthread_attr_t = mem::zeroed();
    let ret: c_int;

    if pthread_attr_init(&mut attr) != 0 {
        err(EXIT_FAILURE, c"pthread_attr_init:".as_ptr());
    }

    /* Original C excludes this pthread_attr_setstacksize call on __ia64__. */
    if pthread_attr_setstacksize(&mut attr, PTHREAD_STACK_MIN) != 0 {
        err(EXIT_FAILURE, c"pthread_attr_setstacksize".as_ptr());
    }

    ret = pthread_create(ptr::addr_of_mut!((*worker).thread), &attr, func, ctx);
    if ret != 0 {
        err(EXIT_FAILURE, c"pthread_create failed".as_ptr());
    }

    pthread_attr_destroy(&mut attr);
}

unsafe fn create_process_worker(
    worker: *mut messaging_worker,
    ctx: *mut c_void,
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
) {
    /* Fork the receiver. */
    (*worker).pid = fork();

    if (*worker).pid == -1 {
        err(EXIT_FAILURE, c"fork()".as_ptr());
    } else if (*worker).pid == 0 {
        func(ctx);
        exit(0);
    }
}

unsafe fn create_worker(
    worker: *mut messaging_worker,
    ctx: *mut c_void,
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
) {
    if !thread_mode {
        create_process_worker(worker, ctx, func);
    } else {
        create_thread_worker(worker, ctx, func);
    }
}

unsafe fn reap_worker(worker: *mut messaging_worker) {
    let mut proc_status: c_int = 0;
    let mut thread_status: *mut c_void = ptr::null_mut();

    if !thread_mode {
        /* process mode */
        wait(&mut proc_status);
        if !WIFEXITED(proc_status) {
            exit(1);
        }
    } else {
        pthread_join((*worker).thread, &mut thread_status);
    }
}

/* One group of senders and receivers */
unsafe fn group(
    worker: *mut messaging_worker,
    num_fds: c_uint,
    ready_out: c_int,
    wakefd: c_int,
) -> c_uint {
    let mut i: c_uint;
    let snd_ctx = malloc(mem::size_of::<sender_context>() + num_fds as usize * mem::size_of::<c_int>())
        as *mut sender_context;

    if snd_ctx.is_null() {
        err(EXIT_FAILURE, c"malloc()".as_ptr());
    }

    list_add(ptr::addr_of_mut!((*snd_ctx).list), ptr::addr_of_mut!(sender_contexts));
    i = 0;
    while i < num_fds {
        let mut fds = [0 as c_int; 2];
        let ctx = malloc(mem::size_of::<receiver_context>()) as *mut receiver_context;

        if ctx.is_null() {
            err(EXIT_FAILURE, c"malloc()".as_ptr());
        }

        list_add(
            ptr::addr_of_mut!((*ctx).list),
            ptr::addr_of_mut!(receiver_contexts),
        );

        /* Create the pipe between client and server */
        fdpair(fds.as_mut_ptr());

        (*ctx).num_packets = num_fds * nr_loops;
        (*ctx).in_fds[0] = fds[0];
        (*ctx).in_fds[1] = fds[1];
        (*ctx).ready_out = ready_out;
        (*ctx).wakefd = wakefd;

        create_worker(worker.add(i as usize), ctx as *mut c_void, receiver);

        *sender_out_fds(snd_ctx).add(i as usize) = fds[1];
        if !thread_mode {
            close(fds[0]);
        }
        i += 1;
    }

    /* Now we have all the fds, fork the senders */
    i = 0;
    while i < num_fds {
        (*snd_ctx).ready_out = ready_out;
        (*snd_ctx).wakefd = wakefd;
        (*snd_ctx).num_fds = num_fds;

        create_worker(worker.add((num_fds + i) as usize), snd_ctx as *mut c_void, sender);
        i += 1;
    }

    /* Close the fds we have left */
    if !thread_mode {
        i = 0;
        while i < num_fds {
            close(*sender_out_fds(snd_ctx).add(i as usize));
            i += 1;
        }
    }

    /* Return number of children to reap */
    num_fds * 2
}

unsafe extern "C" fn sig_handler(_sig: c_int) {
    let mut i: c_uint;

    /*
     * When exit abnormally, kill all forked child processes.
     */
    i = 0;
    while i < total_children {
        kill((*worker_tab.add(i as usize)).pid, SIGKILL);
        i += 1;
    }
}

/* Original C option table:
 * OPT_BOOLEAN('p', "pipe", &use_pipes, "Use pipe() instead of socketpair()"),
 * OPT_BOOLEAN('t', "thread", &thread_mode, "Be multi thread instead of multi process"),
 * OPT_UINTEGER('g', "group", &num_groups, "Specify number of groups"),
 * OPT_UINTEGER('l', "nr_loops", &nr_loops, "Specify the number of loops to run (default: 100)"),
 * OPT_END()
 */
unsafe extern "C" {
    static options: [option; 0];
}

static bench_sched_message_usage_0: &[u8] = b"perf bench sched messaging <options>\0";
static bench_sched_message_usage_1: *const c_char = ptr::null();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_sched_messaging(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut i: c_uint;
    let mut start: timeval = mem::zeroed();
    let mut stop: timeval = mem::zeroed();
    let mut diff: timeval = mem::zeroed();
    let num_fds: c_uint = 20;
    let mut readyfds = [0 as c_int; 2];
    let mut wakefds = [0 as c_int; 2];
    let mut dummy: c_char = 0;
    let mut pos: *mut sender_context;
    let mut n: *mut sender_context;

    let bench_sched_message_usage = [
        bench_sched_message_usage_0.as_ptr() as *const c_char,
        bench_sched_message_usage_1,
    ];

    argc = parse_options(argc, argv, options.as_ptr(), bench_sched_message_usage.as_ptr(), 0);

    worker_tab = calloc(
        (num_fds * 2 * num_groups) as usize,
        mem::size_of::<messaging_worker>(),
    ) as *mut messaging_worker;
    if worker_tab.is_null() {
        err(EXIT_FAILURE, c"main:malloc()".as_ptr());
    }

    fdpair(readyfds.as_mut_ptr());
    fdpair(wakefds.as_mut_ptr());

    if !thread_mode {
        signal(SIGINT, Some(sig_handler));
        signal(SIGTERM, Some(sig_handler));
    }

    i = 0;
    while i < num_groups {
        total_children += group(
            worker_tab.add(total_children as usize),
            num_fds,
            readyfds[1],
            wakefds[0],
        );
        i += 1;
    }

    /* Wait for everyone to be ready */
    i = 0;
    while i < total_children {
        if read(readyfds[0], &mut dummy as *mut c_char as *mut c_void, 1) != 1 {
            err(EXIT_FAILURE, c"Reading for readyfds".as_ptr());
        }
        i += 1;
    }

    gettimeofday(&mut start, ptr::null_mut());

    /* Kick them off */
    if write(wakefds[1], &dummy as *const c_char as *const c_void, 1) != 1 {
        err(EXIT_FAILURE, c"Writing to start them".as_ptr());
    }

    /* Reap them all */
    i = 0;
    while i < total_children {
        reap_worker(worker_tab.add(i as usize));
        i += 1;
    }

    gettimeofday(&mut stop, ptr::null_mut());

    timersub(&stop, &start, &mut diff);

    if bench_format == BENCH_FORMAT_DEFAULT {
        printf(
            c"# %d sender and receiver %s per group\n".as_ptr(),
            num_fds,
            if thread_mode {
                c"threads".as_ptr()
            } else {
                c"processes".as_ptr()
            },
        );
        printf(
            c"# %d groups == %d %s run\n\n".as_ptr(),
            num_groups,
            num_groups * 2 * num_fds,
            if thread_mode {
                c"threads".as_ptr()
            } else {
                c"processes".as_ptr()
            },
        );
        printf(
            c" %14s: %lu.%03lu [sec]\n".as_ptr(),
            c"Total time".as_ptr(),
            diff.tv_sec as c_long,
            (diff.tv_usec / USEC_PER_MSEC) as c_long,
        );
    } else if bench_format == BENCH_FORMAT_SIMPLE {
        printf(
            c"%lu.%03lu\n".as_ptr(),
            diff.tv_sec as c_long,
            (diff.tv_usec / USEC_PER_MSEC) as c_long,
        );
    } else {
        /* reaching here is something disaster */
        fprintf(stderr, c"Unknown format:%d\n".as_ptr(), bench_format);
        exit(1);
    }

    free(worker_tab as *mut c_void);
    pos = (sender_contexts.next as *mut u8).sub(mem::offset_of!(sender_context, list))
        as *mut sender_context;
    n = ((*pos).list.next as *mut u8).sub(mem::offset_of!(sender_context, list)) as *mut sender_context;
    while ptr::addr_of_mut!((*pos).list) != ptr::addr_of_mut!(sender_contexts) {
        list_del_init(ptr::addr_of_mut!((*pos).list));
        free(pos as *mut c_void);
        pos = n;
        n = ((*pos).list.next as *mut u8).sub(mem::offset_of!(sender_context, list)) as *mut sender_context;
    }

    pos = (receiver_contexts.next as *mut u8).sub(mem::offset_of!(receiver_context, list))
        as *mut sender_context;
    n = ((*pos).list.next as *mut u8).sub(mem::offset_of!(receiver_context, list)) as *mut sender_context;
    while ptr::addr_of_mut!((*pos).list) != ptr::addr_of_mut!(receiver_contexts) {
        list_del_init(ptr::addr_of_mut!((*pos).list));
        free(pos as *mut c_void);
        pos = n;
        n = ((*pos).list.next as *mut u8).sub(mem::offset_of!(receiver_context, list)) as *mut sender_context;
    }
    0
}

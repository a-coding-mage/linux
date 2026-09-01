// SPDX-License-Identifier: GPL-2.0
/*
 * pipe_bench - exercise concurrent pipe operation
 *
 * N writer threads hammer a single pipe with multi-page writes; M reader
 * threads drain it. Each writer records its own write() latency histogram.
 * Multi-page writes (msgsize >= PAGE_SIZE) force the loop in
 * anon_pipe_write() to call alloc_page(GFP_HIGHUSER | __GFP_ACCOUNT) under
 * pipe->mutex, which is the critical section the patch shrinks.
 *
 * By default the benchmark sweeps writers in {1, 2, 5} x readers in
 * {1, 5, 10} and prints one block per configuration so two runs (e.g.
 * baseline vs patched) can be diffed directly. Pass -w and -r to run a
 * single configuration instead. Pass --memory-pressure to spawn stress-ng
 * alongside the sweep so the per-page alloc_page() path under pipe->mutex
 * has to dip into reclaim.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates
 * Copyright (c) 2026 Breno Leitao <leitao@debian.org>
 */

use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

const HIST_BUCKETS: usize = 32;

static mut g_msgsize: libc::size_t = 16 * 4096;
static mut g_duration: libc::c_int = 3;
static mut g_pipe_size: libc::c_int = 1024 * 1024;
static mut g_memory_pressure: libc::c_int = 0;

static g_stop: AtomicI32 = AtomicI32::new(0);
static mut g_pipe: [libc::c_int; 2] = [0; 2];

#[repr(C)]
struct wstats {
    writes: u64,
    bytes: u64,
    lat_sum_ns: u64,
    lat_max_ns: u64,
    lat_hist: [u64; HIST_BUCKETS],
    buf: *mut libc::c_char,
}

#[repr(C)]
struct rstats {
    buf: *mut libc::c_char,
}

#[repr(C)]
struct hist_totals {
    writes: u64,
    bytes: u64,
    lat_sum: u64,
    lat_max: u64,
}

unsafe fn now_ns() -> u64 {
    let mut ts: libc::timespec = mem::zeroed();

    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    ts.tv_sec as u64 * 1000000000u64 + ts.tv_nsec as u64
}

fn log2_bucket(mut v: u64) -> libc::c_int {
    let mut b: libc::c_int = 0;

    if v == 0 {
        return 0;
    }
    while {
        v >>= 1;
        v != 0
    } {
        b += 1;
    }
    if (b as usize) < HIST_BUCKETS {
        b
    } else {
        (HIST_BUCKETS - 1) as libc::c_int
    }
}

unsafe extern "C" fn writer(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = arg as *mut wstats;

    while g_stop.load(Ordering::Relaxed) == 0 {
        let t0 = now_ns();
        let n = libc::write(g_pipe[1], (*s).buf as *const libc::c_void, g_msgsize);
        let dt = now_ns().wrapping_sub(t0);

        if n > 0 {
            (*s).writes = (*s).writes.wrapping_add(1);
            (*s).bytes = (*s).bytes.wrapping_add(n as u64);
            (*s).lat_sum_ns = (*s).lat_sum_ns.wrapping_add(dt);
            if dt > (*s).lat_max_ns {
                (*s).lat_max_ns = dt;
            }
            let b = log2_bucket(dt) as usize;
            (*s).lat_hist[b] = (*s).lat_hist[b].wrapping_add(1);
        } else if n < 0 && (*libc::__errno_location() == libc::EPIPE || *libc::__errno_location() == libc::EBADF) {
            break;
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn reader(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = arg as *mut rstats;

    /*
     * Drain until EOF (write end closed by main). g_stop is not checked
     * here on purpose: writers may be blocked in write() with the pipe
     * full when g_stop is set, so the reader must keep draining until
     * main closes the write end.
     */
    loop {
        let n = libc::read(g_pipe[0], (*s).buf as *mut libc::c_void, g_msgsize);

        if n <= 0 {
            break;
        }
    }
    ptr::null_mut()
}

/* Sum per-writer stats and per-bucket counts into the caller's aggregates. */
unsafe fn aggregate_wstats(
    all: *mut wstats,
    nw: libc::c_int,
    agg: *mut u64,
    t: *mut hist_totals,
) {
    libc::memset(t as *mut libc::c_void, 0, mem::size_of_val(&*t));
    for i in 0..nw {
        let s = all.offset(i as isize);
        (*t).writes = (*t).writes.wrapping_add((*s).writes);
        (*t).bytes = (*t).bytes.wrapping_add((*s).bytes);
        (*t).lat_sum = (*t).lat_sum.wrapping_add((*s).lat_sum_ns);
        if (*s).lat_max_ns > (*t).lat_max {
            (*t).lat_max = (*s).lat_max_ns;
        }
        for b in 0..HIST_BUCKETS {
            *agg.add(b) = (*agg.add(b)).wrapping_add((*s).lat_hist[b]);
        }
    }
}

/*
 * Walk @agg in order, returning the inclusive upper bound (in ns) of the
 * log2 bucket where the running sum first reaches @target.
 *
 * A percentile is undefined with zero samples, and with very low sample
 * counts integer truncation could make @target zero -- then "cum >= 0"
 * would latch on the first (possibly empty) bucket. Callers must pass
 * @target >= 1.
 */
unsafe fn bucket_at(agg: *const u64, target: u64) -> u64 {
    let mut cum: u64 = 0;

    for b in 0..HIST_BUCKETS {
        /* HIST_BUCKETS <= 63, so (b + 1) is always a safe shift. */
        let upper = (1u64 << (b + 1)) - 1;

        cum = cum.wrapping_add(*agg.add(b));
        if cum >= target {
            return upper;
        }
    }
    0
}

unsafe fn compute_p50_p99(agg: *const u64, writes: u64, p50: *mut u64, p99: *mut u64) {
    let mut p50_target: u64;
    let mut p99_target: u64;

    *p50 = 0;
    *p99 = 0;
    if writes == 0 {
        return;
    }

    p50_target = writes.wrapping_mul(50) / 100;
    p99_target = writes.wrapping_mul(99) / 100;
    if p50_target == 0 {
        p50_target = 1;
    }
    if p99_target == 0 {
        p99_target = 1;
    }

    *p50 = bucket_at(agg, p50_target);
    *p99 = bucket_at(agg, p99_target);
}

unsafe fn print_summary(nw: libc::c_int, nr: libc::c_int, t: *const hist_totals, p50: u64, p99: u64) {
    let sec: libc::c_double = g_duration as libc::c_double;
    let avg_ns: u64 = if (*t).writes != 0 {
        (*t).lat_sum / (*t).writes
    } else {
        0
    };

    libc::printf(
        b"config: writers=%d readers=%d msgsize=%zu duration=%d pipe_size=%d memory_pressure=%s\n\0".as_ptr() as *const libc::c_char,
        nw,
        nr,
        g_msgsize,
        g_duration,
        g_pipe_size,
        if g_memory_pressure != 0 { b"yes\0".as_ptr() } else { b"no\0".as_ptr() } as *const libc::c_char,
    );
    libc::printf(
        b"writes: total=%llu rate=%.0f/s\n\0".as_ptr() as *const libc::c_char,
        (*t).writes as libc::c_ulonglong,
        (*t).writes as libc::c_double / sec,
    );
    libc::printf(
        b"throughput_MBps: %.2f\n\0".as_ptr() as *const libc::c_char,
        ((*t).bytes as libc::c_double / sec) / (1024.0 * 1024.0),
    );
    libc::printf(b"lat_avg_ns: %llu\n\0".as_ptr() as *const libc::c_char, avg_ns as libc::c_ulonglong);
    libc::printf(b"lat_p50_ns_upper: %llu\n\0".as_ptr() as *const libc::c_char, p50 as libc::c_ulonglong);
    libc::printf(b"lat_p99_ns_upper: %llu\n\0".as_ptr() as *const libc::c_char, p99 as libc::c_ulonglong);
    libc::printf(b"lat_max_ns: %llu\n\0".as_ptr() as *const libc::c_char, (*t).lat_max as libc::c_ulonglong);
}

unsafe fn summarize(all: *mut wstats, nw: libc::c_int, nr: libc::c_int) {
    let mut agg: [u64; HIST_BUCKETS] = [0; HIST_BUCKETS];
    let mut t: hist_totals = mem::zeroed();
    let mut p50: u64 = 0;
    let mut p99: u64 = 0;

    aggregate_wstats(all, nw, agg.as_mut_ptr(), &mut t);
    compute_p50_p99(agg.as_ptr(), t.writes, &mut p50, &mut p99);
    print_summary(nw, nr, &t, p50, p99);
}

/*
 * Child branch of fork(): restore SIGPIPE to default (parent ignores it),
 * exec stress-ng, and on failure write the reason into @hs_wr before
 * exiting. The parent observes EOF on hs_wr (closed via O_CLOEXEC) when
 * exec succeeds.
 */
unsafe fn stress_ng_child(hs_wr: libc::c_int) -> ! {
    let mut errbuf: [libc::c_char; 256] = [0; 256];

    libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    libc::execlp(
        b"stress-ng\0".as_ptr() as *const libc::c_char,
        b"stress-ng\0".as_ptr() as *const libc::c_char,
        b"--vm\0".as_ptr() as *const libc::c_char,
        b"4\0".as_ptr() as *const libc::c_char,
        b"--vm-bytes\0".as_ptr() as *const libc::c_char,
        b"80%\0".as_ptr() as *const libc::c_char,
        b"--vm-method\0".as_ptr() as *const libc::c_char,
        b"all\0".as_ptr() as *const libc::c_char,
        ptr::null::<libc::c_char>(),
    );
    libc::snprintf(
        errbuf.as_mut_ptr(),
        errbuf.len(),
        b"exec stress-ng failed: %s\n\0".as_ptr() as *const libc::c_char,
        libc::strerror(*libc::__errno_location()),
    );
    let _ = libc::write(
        hs_wr,
        errbuf.as_ptr() as *const libc::c_void,
        libc::strlen(errbuf.as_ptr()),
    );
    libc::_exit(127);
}

/*
 * Read from the O_CLOEXEC handshake pipe. Anything readable means the
 * child wrote an error before exec; EOF (n == 0) means the write-end
 * closed because exec succeeded. Returns 0 on exec success, -1 if the
 * child failed and was reaped.
 */
unsafe fn stress_ng_wait_handshake(hs_rd: libc::c_int, pid: libc::pid_t) -> libc::c_int {
    let mut pfd = libc::pollfd {
        fd: hs_rd,
        events: libc::POLLIN,
        revents: 0,
    };
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    let mut status: libc::c_int = 0;
    let mut ret: libc::c_int;

    ret = libc::poll(&mut pfd, 1, 500);
    if ret <= 0 {
        return 0;
    }

    let n = libc::read(
        hs_rd,
        errbuf.as_mut_ptr() as *mut libc::c_void,
        errbuf.len() - 1,
    );

    if n > 0 {
        errbuf[n as usize] = b'\0' as libc::c_char;
        libc::fputs(errbuf.as_ptr(), libc::stderr);
        libc::waitpid(pid, &mut status, 0);
        return -1;
    }
    0
}

unsafe fn spawn_stress_ng() -> libc::pid_t {
    let mut hs: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::pid_t;

    /*
     * Handshake pipe: child writes one byte and _exit()s on exec
     * failure. On exec success the O_CLOEXEC flag closes the write
     * end, which the parent observes as EOF. This makes the "is
     * stress-ng on $PATH?" check fail fast rather than silently.
     */
    if libc::pipe2(hs.as_mut_ptr(), libc::O_CLOEXEC) < 0 {
        libc::perror(b"pipe2\0".as_ptr() as *const libc::c_char);
        return -1;
    }

    pid = libc::fork();
    if pid < 0 {
        libc::perror(b"fork\0".as_ptr() as *const libc::c_char);
        libc::close(hs[0]);
        libc::close(hs[1]);
        return -1;
    }
    if pid == 0 {
        libc::close(hs[0]);
        stress_ng_child(hs[1]);
    }

    libc::close(hs[1]);
    if stress_ng_wait_handshake(hs[0], pid) < 0 {
        libc::close(hs[0]);
        return -1;
    }
    libc::close(hs[0]);

    /* Give stress-ng a moment to map its VM regions before measuring. */
    libc::sleep(1);
    pid
}

unsafe fn kill_stress_ng(pid: libc::pid_t) {
    let mut status: libc::c_int = 0;

    if pid <= 0 {
        return;
    }
    libc::kill(pid, libc::SIGTERM);
    for _i in 0..20 {
        if libc::waitpid(pid, &mut status, libc::WNOHANG) > 0 {
            return;
        }
        libc::usleep(100 * 1000);
    }
    libc::kill(pid, libc::SIGKILL);
    libc::waitpid(pid, &mut status, 0);
}

/*
 * Allocate per-thread page-aligned buffers in main so a failed
 * aligned_alloc() aborts the run before any thread starts. Workers used
 * to allocate their own buffer and return NULL on failure, which left
 * peers blocked in write()/read() with nobody to unblock them.
 */
unsafe fn alloc_thread_bufs(ws: *mut wstats, nw: libc::c_int, rs: *mut rstats, nr: libc::c_int) -> libc::c_int {
    for i in 0..nw {
        (*ws.offset(i as isize)).buf = libc::aligned_alloc(4096, g_msgsize) as *mut libc::c_char;
        if (*ws.offset(i as isize)).buf.is_null() {
            libc::fprintf(
                libc::stderr,
                b"writer %d: aligned_alloc(%zu) failed\n\0".as_ptr() as *const libc::c_char,
                i,
                g_msgsize,
            );
            return -1;
        }
        libc::memset((*ws.offset(i as isize)).buf as *mut libc::c_void, 0xAA, g_msgsize);
    }
    for i in 0..nr {
        (*rs.offset(i as isize)).buf = libc::aligned_alloc(4096, g_msgsize) as *mut libc::c_char;
        if (*rs.offset(i as isize)).buf.is_null() {
            libc::fprintf(
                libc::stderr,
                b"reader %d: aligned_alloc(%zu) failed\n\0".as_ptr() as *const libc::c_char,
                i,
                g_msgsize,
            );
            return -1;
        }
    }
    0
}

unsafe fn free_thread_bufs(ws: *mut wstats, nw: libc::c_int, rs: *mut rstats, nr: libc::c_int) {
    if !ws.is_null() {
        for i in 0..nw {
            libc::free((*ws.offset(i as isize)).buf as *mut libc::c_void);
        }
    }
    if !rs.is_null() {
        for i in 0..nr {
            libc::free((*rs.offset(i as isize)).buf as *mut libc::c_void);
        }
    }
}

unsafe fn start_readers(
    rt: *mut libc::pthread_t,
    rs: *mut rstats,
    nr: libc::c_int,
    created: *mut libc::c_int,
) -> libc::c_int {
    for i in 0..nr {
        let err = libc::pthread_create(
            rt.offset(i as isize),
            ptr::null(),
            reader,
            rs.offset(i as isize) as *mut libc::c_void,
        );

        if err != 0 {
            libc::fprintf(
                libc::stderr,
                b"pthread_create reader %d: %s\n\0".as_ptr() as *const libc::c_char,
                i,
                libc::strerror(err),
            );
            return -1;
        }
        *created += 1;
    }
    0
}

unsafe fn start_writers(
    wt: *mut libc::pthread_t,
    ws: *mut wstats,
    nw: libc::c_int,
    created: *mut libc::c_int,
) -> libc::c_int {
    for i in 0..nw {
        let err = libc::pthread_create(
            wt.offset(i as isize),
            ptr::null(),
            writer,
            ws.offset(i as isize) as *mut libc::c_void,
        );

        if err != 0 {
            libc::fprintf(
                libc::stderr,
                b"pthread_create writer %d: %s\n\0".as_ptr() as *const libc::c_char,
                i,
                libc::strerror(err),
            );
            return -1;
        }
        *created += 1;
    }
    0
}

unsafe fn open_bench_pipe() -> libc::c_int {
    if libc::pipe(g_pipe.as_mut_ptr()) < 0 {
        libc::perror(b"pipe\0".as_ptr() as *const libc::c_char);
        return -1;
    }
    if libc::fcntl(g_pipe[1], libc::F_SETPIPE_SZ, g_pipe_size) < 0 {
        libc::perror(b"F_SETPIPE_SZ (continuing)\0".as_ptr() as *const libc::c_char);
    }
    0
}

/*
 * Normal termination: g_stop tells writers to leave the loop after the
 * current write() returns. Closing the shared write-end fd means once
 * the in-flight writes drain, readers see EOF and exit. Writers are not
 * unblocked by EPIPE here -- g_pipe[0] stays open so readers can keep
 * draining.
 *
 * Error path: some threads may have been created and others skipped, so
 * writers could be blocked in write() with no reader making progress.
 * Close both ends -- closing the read end is what delivers EPIPE to a
 * blocked writer.
 */
unsafe fn stop_and_join(
    wt: *mut libc::pthread_t,
    nw_created: libc::c_int,
    rt: *mut libc::pthread_t,
    nr_created: libc::c_int,
    rc: libc::c_int,
) {
    g_stop.store(1, Ordering::SeqCst);
    libc::close(g_pipe[1]);
    if rc < 0 {
        libc::close(g_pipe[0]);
    }
    for i in 0..nw_created {
        libc::pthread_join(*wt.offset(i as isize), ptr::null_mut());
    }
    for i in 0..nr_created {
        libc::pthread_join(*rt.offset(i as isize), ptr::null_mut());
    }
    if rc == 0 {
        libc::close(g_pipe[0]);
    }
}

unsafe fn run_one(nw: libc::c_int, nr: libc::c_int) -> libc::c_int {
    let mut wt: *mut libc::pthread_t = ptr::null_mut();
    let mut rt: *mut libc::pthread_t = ptr::null_mut();
    let mut ws: *mut wstats = ptr::null_mut();
    let mut rs: *mut rstats = ptr::null_mut();
    let mut nw_created: libc::c_int = 0;
    let mut nr_created: libc::c_int = 0;
    let mut rc: libc::c_int = 0;

    g_stop.store(0, Ordering::SeqCst);

    if open_bench_pipe() < 0 {
        return -1;
    }

    wt = libc::calloc(nw as libc::size_t, mem::size_of::<libc::pthread_t>()) as *mut libc::pthread_t;
    rt = libc::calloc(nr as libc::size_t, mem::size_of::<libc::pthread_t>()) as *mut libc::pthread_t;
    ws = libc::calloc(nw as libc::size_t, mem::size_of::<wstats>()) as *mut wstats;
    rs = libc::calloc(nr as libc::size_t, mem::size_of::<rstats>()) as *mut rstats;
    if wt.is_null() || rt.is_null() || ws.is_null() || rs.is_null() {
        libc::fprintf(libc::stderr, b"alloc failed\n\0".as_ptr() as *const libc::c_char);
        rc = -1;
        stop_and_join(wt, nw_created, rt, nr_created, rc);
        free_thread_bufs(ws, nw, rs, nr);
        libc::free(wt as *mut libc::c_void);
        libc::free(rt as *mut libc::c_void);
        libc::free(ws as *mut libc::c_void);
        libc::free(rs as *mut libc::c_void);
        return rc;
    }

    if alloc_thread_bufs(ws, nw, rs, nr) < 0 {
        rc = -1;
    } else if start_readers(rt, rs, nr, &mut nr_created) < 0
        || start_writers(wt, ws, nw, &mut nw_created) < 0
    {
        rc = -1;
    } else {
        libc::sleep(g_duration as libc::c_uint);
    }

    stop_and_join(wt, nw_created, rt, nr_created, rc);

    if rc == 0 {
        summarize(ws, nw, nr);
        libc::fflush(libc::stdout);
    }

    free_thread_bufs(ws, nw, rs, nr);
    libc::free(wt as *mut libc::c_void);
    libc::free(rt as *mut libc::c_void);
    libc::free(ws as *mut libc::c_void);
    libc::free(rs as *mut libc::c_void);
    rc
}

unsafe fn usage(prog: *const libc::c_char) {
    libc::fprintf(
        libc::stderr,
        b"usage: %s [-w writers] [-r readers] [-s msgsize] [-d secs] [-p pipe_size] [--memory-pressure]\n  default: sweep writers={1,2,5} x readers={1,5,10}\n  --memory-pressure: spawn stress-ng (--vm 4 --vm-bytes 80%% --vm-method all) for the run\n\0".as_ptr() as *const libc::c_char,
        prog,
    );
}

unsafe fn parse_args(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    writers_override: *mut libc::c_int,
    readers_override: *mut libc::c_int,
) -> libc::c_int {
    let long_opts = [
        libc::option {
            name: b"memory-pressure\0".as_ptr() as *const libc::c_char,
            has_arg: libc::no_argument,
            flag: ptr::null_mut(),
            val: b'M' as libc::c_int,
        },
        libc::option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];
    let mut opt: libc::c_int;

    loop {
        opt = libc::getopt_long(
            argc,
            argv,
            b"w:r:s:d:p:\0".as_ptr() as *const libc::c_char,
            long_opts.as_ptr(),
            ptr::null_mut(),
        );
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'w' as libc::c_int => {
                *writers_override = libc::atoi(libc::optarg);
            }
            x if x == b'r' as libc::c_int => {
                *readers_override = libc::atoi(libc::optarg);
            }
            x if x == b's' as libc::c_int => {
                g_msgsize = libc::atol(libc::optarg) as libc::size_t;
            }
            x if x == b'd' as libc::c_int => {
                g_duration = libc::atoi(libc::optarg);
            }
            x if x == b'p' as libc::c_int => {
                g_pipe_size = libc::atoi(libc::optarg);
            }
            x if x == b'M' as libc::c_int => {
                g_memory_pressure = 1;
            }
            _ => {
                usage(*argv);
                return -1;
            }
        }
    }
    0
}

/*
 * aligned_alloc(4096, size) requires size to be a multiple of the
 * alignment (C11); glibc returns NULL otherwise, which would make
 * writer/reader threads silently exit and the run report zero writes.
 * Validate up front instead.
 */
unsafe fn validate_args() -> libc::c_int {
    if g_msgsize == 0 || g_msgsize % 4096 != 0 {
        libc::fprintf(
            libc::stderr,
            b"msgsize must be a positive multiple of 4096 (got %zu)\n\0".as_ptr() as *const libc::c_char,
            g_msgsize,
        );
        return -1;
    }
    if g_duration <= 0 {
        libc::fprintf(
            libc::stderr,
            b"duration must be > 0 seconds (got %d)\n\0".as_ptr() as *const libc::c_char,
            g_duration,
        );
        return -1;
    }
    if g_pipe_size <= 0 {
        libc::fprintf(
            libc::stderr,
            b"pipe_size must be > 0 bytes (got %d)\n\0".as_ptr() as *const libc::c_char,
            g_pipe_size,
        );
        return -1;
    }
    0
}

unsafe fn run_sweep() -> libc::c_int {
    static writers_sweep: [libc::c_int; 3] = [1, 2, 5];
    static readers_sweep: [libc::c_int; 3] = [1, 5, 10];

    for i in 0..writers_sweep.len() {
        for j in 0..readers_sweep.len() {
            libc::printf(b"---\n\0".as_ptr() as *const libc::c_char);
            if run_one(writers_sweep[i], readers_sweep[j]) < 0 {
                return -1;
            }
        }
    }
    0
}

fn main() {
    unsafe {
        let mut writers_override: libc::c_int = 0;
        let mut readers_override: libc::c_int = 0;
        let mut stress_pid: libc::pid_t = -1;
        let rc: libc::c_int;
        let args: Vec<*mut libc::c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        let mut argv = args;

        if parse_args(
            argv.len() as libc::c_int,
            argv.as_mut_ptr(),
            &mut writers_override,
            &mut readers_override,
        ) < 0
        {
            for arg in argv {
                let _ = std::ffi::CString::from_raw(arg);
            }
            std::process::exit(1);
        }
        if validate_args() < 0 {
            for arg in argv {
                let _ = std::ffi::CString::from_raw(arg);
            }
            std::process::exit(1);
        }

        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
        libc::setvbuf(libc::stdout, ptr::null_mut(), libc::_IOLBF, 0);
        libc::setvbuf(libc::stderr, ptr::null_mut(), libc::_IOLBF, 0);

        libc::fprintf(libc::stderr, b"pid=%d\n\0".as_ptr() as *const libc::c_char, libc::getpid());
        libc::fflush(libc::stderr);

        if g_memory_pressure != 0 {
            stress_pid = spawn_stress_ng();
            if stress_pid < 0 {
                libc::fprintf(
                    libc::stderr,
                    b"memory_pressure requested but stress-ng could not be spawned\n\0".as_ptr() as *const libc::c_char,
                );
                for arg in argv {
                    let _ = std::ffi::CString::from_raw(arg);
                }
                std::process::exit(1);
            }
        }

        if writers_override > 0 || readers_override > 0 {
            let nw = if writers_override > 0 { writers_override } else { 1 };
            let nr = if readers_override > 0 { readers_override } else { 1 };

            rc = if run_one(nw, nr) < 0 { 1 } else { 0 };
        } else {
            rc = if run_sweep() < 0 { 1 } else { 0 };
        }

        kill_stress_ng(stress_pid);
        for arg in argv {
            let _ = std::ffi::CString::from_raw(arg);
        }
        std::process::exit(rc);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

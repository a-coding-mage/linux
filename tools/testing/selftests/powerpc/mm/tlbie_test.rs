// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2019, Nick Piggin, Gautham R. Shenoy, Aneesh Kumar K.V, IBM Corp.
 */

/*
 *
 * Test tlbie/mtpidr race. We have 4 threads doing flush/load/compare/store
 * sequence in a loop. The same threads also rung a context switch task
 * that does sched_yield() in loop.
 *
 * The snapshot thread mark the mmap area PROT_READ in between, make a copy
 * and copy it back to the original area. This helps us to detect if any
 * store continued to happen after we marked the memory PROT_READ.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type time_t = c_long;
type pid_t = c_int;
type key_t = c_int;
type pthread_t = c_ulong;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
struct sched_param {
    sched_priority: c_int,
}

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
union sigaction_handler {
    sa_handler: Option<extern "C" fn(c_int)>,
    sa_sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

#[repr(C)]
struct sigaction {
    handler: sigaction_handler,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;

    fn time(tloc: *mut time_t) -> time_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn ctime(timep: *const time_t) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn perror(s: *const c_char);
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sched_yield() -> c_int;
    fn sched_setscheduler(pid: pid_t, policy: c_int, param: *const sched_param) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn getpagesize() -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> pid_t;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn shmget(key: key_t, size: size_t, shmflg: c_int) -> c_int;
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn fork() -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn alarm(seconds: c_uint) -> c_uint;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

#[repr(C)]
struct pthread_attr_t {
    _private: [u64; 7],
}

const EXIT_FAILURE: c_int = 1;
const SIGSEGV: c_int = 11;
const SIGALRM: c_int = 14;
const SIGKILL: c_int = 9;
const SA_SIGINFO: c_int = 4;
const SCHED_FIFO: c_int = 1;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const IPC_CREAT: c_int = 0o1000;
const PR_SET_PDEATHSIG: c_int = 1;

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        (*set).__bits = [0; 16];
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = 8 * size_of::<c_ulong>();
    unsafe {
        (*set).__bits[cpu / bits_per_word] |= 1_c_ulong << (cpu % bits_per_word);
    }
}

#[inline]
unsafe fn dcbf(addr: *mut c_uint) {
    unsafe {
        asm!(
            "dcbf 0,{0}",
            "sync",
            in(reg) addr as *mut u8,
            options(nostack, preserves_flags)
        );
    }
}

unsafe fn err_msg(msg: *mut c_char) -> ! {
    unsafe {
        let mut now: time_t = 0;
        time(&mut now);
        printf(c"=================================\n".as_ptr());
        printf(c"    Error: %s\n".as_ptr(), msg);
        printf(c"    %s".as_ptr(), ctime(&now));
        printf(c"=================================\n".as_ptr());
        exit(1);
    }
}

static mut map1: *mut c_char = null_mut();
static mut map2: *mut c_char = null_mut();
static mut rim_process_pid: pid_t = 0;

/*
 * A "rim-sequence" is defined to be the sequence of the following
 * operations performed on a memory word:
 *	1) FLUSH the contents of that word.
 *	2) LOAD the contents of that word.
 *	3) COMPARE the contents of that word with the content that was
 *	           previously stored at that word
 *	4) STORE new content into that word.
 *
 * The threads in this test that perform the rim-sequence are termed
 * as rim_threads.
 */

/*
 * A "corruption" is defined to be the failed COMPARE operation in a
 * rim-sequence.
 *
 * A rim_thread that detects a corruption informs about it to all the
 * other rim_threads, and the mem_snapshot thread.
 */
static mut corruption_found: c_uint = 0;

/*
 * This defines the maximum number of rim_threads in this test.
 *
 * The THREAD_ID_BITS denote the number of bits required
 * to represent the thread_ids [0..MAX_THREADS - 1].
 * We are being a bit paranoid here and set it to 8 bits,
 * though 6 bits suffice.
 *
 */
const MAX_THREADS: usize = 64;
const THREAD_ID_BITS: c_uint = 8;
const THREAD_ID_MASK: c_uint = (1 << THREAD_ID_BITS) - 1;
static mut rim_thread_ids: [c_uint; MAX_THREADS] = [0; MAX_THREADS];
static mut rim_threads: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];

/*
 * Each rim_thread works on an exclusive "chunk" of size
 * RIM_CHUNK_SIZE.
 *
 * The ith rim_thread works on the ith chunk.
 *
 * The ith chunk begins at
 * map1 + (i * RIM_CHUNK_SIZE)
 */
const RIM_CHUNK_SIZE: usize = 1024;
const BITS_PER_BYTE: usize = 8;
const WORD_SIZE: usize = size_of::<c_uint>();
const WORD_BITS: c_uint = (WORD_SIZE * BITS_PER_BYTE) as c_uint;
const WORDS_PER_CHUNK: usize = RIM_CHUNK_SIZE / WORD_SIZE;

#[inline]
unsafe fn compute_chunk_start_addr(thread_id: c_uint) -> *mut c_char {
    unsafe { (map1 as c_ulong).wrapping_add((thread_id as usize * RIM_CHUNK_SIZE) as c_ulong) as *mut c_char }
}

/*
 * The "word-offset" of a word-aligned address inside a chunk, is
 * defined to be the number of words that precede the address in that
 * chunk.
 *
 * WORD_OFFSET_BITS denote the number of bits required to represent
 * the word-offsets of all the word-aligned addresses of a chunk.
 */
const WORD_OFFSET_BITS: c_uint = WORDS_PER_CHUNK.trailing_zeros();
const WORD_OFFSET_MASK: c_uint = (1 << WORD_OFFSET_BITS) - 1;

#[inline]
unsafe fn compute_word_offset(start: *mut c_char, addr: *mut c_uint) -> c_uint {
    let delta_bytes: c_uint = (addr as c_ulong).wrapping_sub(start as c_ulong) as c_uint;
    let ret: c_uint = delta_bytes / WORD_SIZE as c_uint;

    ret
}

/*
 * A "sweep" is defined to be the sequential execution of the
 * rim-sequence by a rim_thread on its chunk one word at a time,
 * starting from the first word of its chunk and ending with the last
 * word of its chunk.
 *
 * Each sweep of a rim_thread is uniquely identified by a sweep_id.
 * SWEEP_ID_BITS denote the number of bits required to represent
 * the sweep_ids of rim_threads.
 *
 * As to why SWEEP_ID_BITS are computed as a function of THREAD_ID_BITS,
 * WORD_OFFSET_BITS, and WORD_BITS, see the "store-pattern" below.
 */
const SWEEP_ID_BITS: c_uint = WORD_BITS - (THREAD_ID_BITS + WORD_OFFSET_BITS);
const SWEEP_ID_MASK: c_uint = (1 << SWEEP_ID_BITS) - 1;

/*
 * A "store-pattern" is the word-pattern that is stored into a word
 * location in the 4)STORE step of the rim-sequence.
 *
 * In the store-pattern, we shall encode:
 *
 *      - The thread-id of the rim_thread performing the store
 *        (The most significant THREAD_ID_BITS)
 *
 *      - The word-offset of the address into which the store is being
 *        performed (The next WORD_OFFSET_BITS)
 *
 *      - The sweep_id of the current sweep in which the store is
 *        being performed. (The lower SWEEP_ID_BITS)
 *
 * Store Pattern: 32 bits
 * |------------------|--------------------|---------------------------------|
 * |    Thread id     |  Word offset       |         sweep_id                |
 * |------------------|--------------------|---------------------------------|
 *    THREAD_ID_BITS     WORD_OFFSET_BITS          SWEEP_ID_BITS
 *
 * In the store pattern, the (Thread-id + Word-offset) uniquely identify the
 * address to which the store is being performed i.e,
 *    address == map1 +
 *              (Thread-id * RIM_CHUNK_SIZE) + (Word-offset * WORD_SIZE)
 *
 * And the sweep_id in the store pattern identifies the time when the
 * store was performed by the rim_thread.
 *
 * We shall use this property in the 3)COMPARE step of the
 * rim-sequence.
 */
const SWEEP_ID_SHIFT: c_uint = 0;
const WORD_OFFSET_SHIFT: c_uint = SWEEP_ID_BITS;
const THREAD_ID_SHIFT: c_uint = WORD_OFFSET_BITS + SWEEP_ID_BITS;

/*
 * Compute the store pattern for a given thread with id @tid, at
 * location @addr in the sweep identified by @sweep_id
 */
#[inline]
unsafe fn compute_store_pattern(tid: c_uint, addr: *mut c_uint, sweep_id: c_uint) -> c_uint {
    let mut ret: c_uint = 0;
    let start: *mut c_char = unsafe { compute_chunk_start_addr(tid) };
    let word_offset: c_uint = unsafe { compute_word_offset(start, addr) };

    ret = ret.wrapping_add((tid & THREAD_ID_MASK) << THREAD_ID_SHIFT);
    ret = ret.wrapping_add((word_offset & WORD_OFFSET_MASK) << WORD_OFFSET_SHIFT);
    ret = ret.wrapping_add((sweep_id & SWEEP_ID_MASK) << SWEEP_ID_SHIFT);
    ret
}

/* Extract the thread-id from the given store-pattern */
#[inline]
fn extract_tid(pattern: c_uint) -> c_uint {
    let ret: c_uint;

    ret = (pattern >> THREAD_ID_SHIFT) & THREAD_ID_MASK;
    ret
}

/* Extract the word-offset from the given store-pattern */
#[inline]
fn extract_word_offset(pattern: c_uint) -> c_uint {
    let ret: c_uint;

    ret = (pattern >> WORD_OFFSET_SHIFT) & WORD_OFFSET_MASK;

    ret
}

/* Extract the sweep-id from the given store-pattern */
#[inline]
fn extract_sweep_id(pattern: c_uint) -> c_uint {
    let ret: c_uint;

    ret = (pattern >> SWEEP_ID_SHIFT) & SWEEP_ID_MASK;

    ret
}

/************************************************************
 *                                                          *
 *          Logging the output of the verification          *
 *                                                          *
 ************************************************************/
const LOGDIR_NAME_SIZE: usize = 100;
static mut logdir: [c_char; LOGDIR_NAME_SIZE] = [0; LOGDIR_NAME_SIZE];

static mut fp: [*mut FILE; MAX_THREADS] = [null_mut(); MAX_THREADS];
static logfilename: &[u8] = b"Thread-%02d-Chunk\0";

#[inline]
unsafe fn start_verification_log(
    tid: c_uint,
    addr: *mut c_uint,
    cur_sweep_id: c_uint,
    prev_sweep_id: c_uint,
) {
    unsafe {
        let mut f: *mut FILE;
        let mut logfile: [c_char; 30] = [0; 30];
        let mut path: [c_char; LOGDIR_NAME_SIZE + 30] = [0; LOGDIR_NAME_SIZE + 30];
        let mut separator: [c_char; 2] = [b'/' as c_char, 0];
        let chunk_start: *mut c_char = compute_chunk_start_addr(tid);
        let size: c_uint = RIM_CHUNK_SIZE as c_uint;

        sprintf(logfile.as_mut_ptr(), logfilename.as_ptr() as *const c_char, tid);
        strcpy(path.as_mut_ptr(), logdir.as_ptr());
        strcat(path.as_mut_ptr(), separator.as_mut_ptr());
        strcat(path.as_mut_ptr(), logfile.as_mut_ptr());
        f = fopen(path.as_mut_ptr(), c"w".as_ptr());

        if f.is_null() {
            err_msg(c"Unable to create logfile\n".as_ptr() as *mut c_char);
        }

        fp[tid as usize] = f;

        fprintf(f, c"----------------------------------------------------------\n".as_ptr());
        fprintf(f, c"PID                = %d\n".as_ptr(), rim_process_pid);
        fprintf(f, c"Thread id          = %02d\n".as_ptr(), tid);
        fprintf(f, c"Chunk Start Addr   = 0x%016lx\n".as_ptr(), chunk_start as c_ulong);
        fprintf(f, c"Chunk Size         = %d\n".as_ptr(), size);
        fprintf(f, c"Next Store Addr    = 0x%016lx\n".as_ptr(), addr as c_ulong);
        fprintf(f, c"Current sweep-id   = 0x%08x\n".as_ptr(), cur_sweep_id);
        fprintf(f, c"Previous sweep-id  = 0x%08x\n".as_ptr(), prev_sweep_id);
        fprintf(f, c"----------------------------------------------------------\n".as_ptr());
    }
}

#[inline]
unsafe fn log_anamoly(tid: c_uint, addr: *mut c_uint, expected: c_uint, observed: c_uint) {
    unsafe {
        let f: *mut FILE = fp[tid as usize];

        fprintf(
            f,
            c"Thread %02d: Addr 0x%lx: Expected 0x%x, Observed 0x%x\n".as_ptr(),
            tid,
            addr as c_ulong,
            expected,
            observed,
        );
        fprintf(f, c"Thread %02d: Expected Thread id   = %02d\n".as_ptr(), tid, extract_tid(expected));
        fprintf(f, c"Thread %02d: Observed Thread id   = %02d\n".as_ptr(), tid, extract_tid(observed));
        fprintf(f, c"Thread %02d: Expected Word offset = %03d\n".as_ptr(), tid, extract_word_offset(expected));
        fprintf(f, c"Thread %02d: Observed Word offset = %03d\n".as_ptr(), tid, extract_word_offset(observed));
        fprintf(f, c"Thread %02d: Expected sweep-id    = 0x%x\n".as_ptr(), tid, extract_sweep_id(expected));
        fprintf(f, c"Thread %02d: Observed sweep-id    = 0x%x\n".as_ptr(), tid, extract_sweep_id(observed));
        fprintf(f, c"----------------------------------------------------------\n".as_ptr());
    }
}

#[inline]
unsafe fn end_verification_log(tid: c_uint, nr_anamolies: c_uint) {
    unsafe {
        let f: *mut FILE = fp[tid as usize];
        let mut logfile: [c_char; 30] = [0; 30];
        let mut path: [c_char; LOGDIR_NAME_SIZE + 30] = [0; LOGDIR_NAME_SIZE + 30];
        let mut separator: [c_char; 2] = [b'/' as c_char, 0];

        fclose(f);

        sprintf(logfile.as_mut_ptr(), logfilename.as_ptr() as *const c_char, tid);
        strcpy(path.as_mut_ptr(), logdir.as_ptr());
        strcat(path.as_mut_ptr(), separator.as_mut_ptr());
        strcat(path.as_mut_ptr(), logfile.as_mut_ptr());

        if nr_anamolies == 0 {
            remove(path.as_mut_ptr());
            return;
        }

        printf(
            c"Thread %02d chunk has %d corrupted words. For details check %s\n".as_ptr(),
            tid,
            nr_anamolies,
            path.as_mut_ptr(),
        );
    }
}

/*
 * When a COMPARE step of a rim-sequence fails, the rim_thread informs
 * everyone else via the shared_memory pointed to by
 * corruption_found variable. On seeing this, every thread verifies the
 * content of its chunk as follows.
 *
 * Suppose a thread identified with @tid was about to store (but not
 * yet stored) to @next_store_addr in its current sweep identified
 * @cur_sweep_id. Let @prev_sweep_id indicate the previous sweep_id.
 *
 * This implies that for all the addresses @addr < @next_store_addr,
 * Thread @tid has already performed a store as part of its current
 * sweep. Hence we expect the content of such @addr to be:
 *    |-------------------------------------------------|
 *    | tid   | word_offset(addr) |    cur_sweep_id     |
 *    |-------------------------------------------------|
 *
 * Since Thread @tid is yet to perform stores on address
 * @next_store_addr and above, we expect the content of such an
 * address @addr to be:
 *    |-------------------------------------------------|
 *    | tid   | word_offset(addr) |    prev_sweep_id    |
 *    |-------------------------------------------------|
 *
 * The verifier function @verify_chunk does this verification and logs
 * any anamolies that it finds.
 */
unsafe fn verify_chunk(
    tid: c_uint,
    next_store_addr: *mut c_uint,
    cur_sweep_id: c_uint,
    prev_sweep_id: c_uint,
) {
    unsafe {
        let mut iter_ptr: *mut c_uint;
        let size: c_uint = RIM_CHUNK_SIZE as c_uint;
        let mut expected: c_uint;
        let observed: c_uint;
        let chunk_start: *mut c_char = compute_chunk_start_addr(tid);

        let mut nr_anamolies: c_int = 0;

        start_verification_log(tid, next_store_addr, cur_sweep_id, prev_sweep_id);

        iter_ptr = chunk_start as *mut c_uint;
        while (iter_ptr as c_ulong) < (chunk_start as c_ulong).wrapping_add(size as c_ulong) {
            let expected_sweep_id: c_uint;

            if iter_ptr < next_store_addr {
                expected_sweep_id = cur_sweep_id;
            } else {
                expected_sweep_id = prev_sweep_id;
            }

            expected = compute_store_pattern(tid, iter_ptr, expected_sweep_id);

            dcbf(iter_ptr); //Flush before reading
            observed = *iter_ptr;

            if observed != expected {
                nr_anamolies += 1;
                log_anamoly(tid, iter_ptr, expected, observed);
            }
            iter_ptr = iter_ptr.add(1);
        }

        end_verification_log(tid, nr_anamolies as c_uint);
    }
}

unsafe fn set_pthread_cpu(th: pthread_t, cpu: c_int) {
    unsafe {
        let mut run_cpu_mask: cpu_set_t = zeroed();
        let mut param: sched_param = zeroed();

        CPU_ZERO(&mut run_cpu_mask);
        CPU_SET(cpu, &mut run_cpu_mask);
        pthread_setaffinity_np(th, size_of::<cpu_set_t>(), &run_cpu_mask);

        param.sched_priority = 1;
        if 0 != 0 && sched_setscheduler(0, SCHED_FIFO, &param) == -1 {
            /* haven't reproduced with this setting, it kills random preemption which may be a factor */
            fprintf(stderr, c"could not set SCHED_FIFO, run as root?\n".as_ptr());
        }
    }
}

unsafe fn set_mycpu(cpu: c_int) {
    unsafe {
        let mut run_cpu_mask: cpu_set_t = zeroed();
        let mut param: sched_param = zeroed();

        CPU_ZERO(&mut run_cpu_mask);
        CPU_SET(cpu, &mut run_cpu_mask);
        sched_setaffinity(0, size_of::<cpu_set_t>(), &run_cpu_mask);

        param.sched_priority = 1;
        if 0 != 0 && sched_setscheduler(0, SCHED_FIFO, &param) == -1 {
            fprintf(stderr, c"could not set SCHED_FIFO, run as root?\n".as_ptr());
        }
    }
}

static mut segv_wait: c_int = 0;

extern "C" fn segv_handler(_signo: c_int, _info: *mut siginfo_t, _extra: *mut c_void) {
    unsafe {
        while segv_wait != 0 {
            sched_yield();
        }
    }
}

unsafe fn set_segv_handler() {
    unsafe {
        let mut sa: sigaction = zeroed();

        sa.sa_flags = SA_SIGINFO;
        sa.handler.sa_sigaction = Some(segv_handler);

        if sigaction(SIGSEGV, &sa, null_mut()) == -1 {
            perror(c"sigaction".as_ptr());
            exit(EXIT_FAILURE);
        }
    }
}

static mut timeout: c_int = 0;
/*
 * This function is executed by every rim_thread.
 *
 * This function performs sweeps over the exclusive chunks of the
 * rim_threads executing the rim-sequence one word at a time.
 */
extern "C" fn rim_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let tid: c_uint = *(arg as *mut c_uint);

        let size: c_int = RIM_CHUNK_SIZE as c_int;
        let chunk_start: *mut c_char = compute_chunk_start_addr(tid);

        let mut prev_sweep_id: c_uint;
        let mut cur_sweep_id: c_uint = 0;

        /* word access */
        let mut pattern: c_uint = cur_sweep_id;
        let pattern_ptr: *mut c_uint = &mut pattern;
        let mut w_ptr: *mut c_uint;
        let mut read_data: c_uint;

        set_segv_handler();

        /*
         * Let us initialize the chunk:
         *
         * Each word-aligned address addr in the chunk,
         * is initialized to :
         *    |-------------------------------------------------|
         *    | tid   | word_offset(addr) |         0           |
         *    |-------------------------------------------------|
         */
        w_ptr = chunk_start as *mut c_uint;
        while (w_ptr as c_ulong) < (chunk_start as c_ulong).wrapping_add(size as c_ulong) {
            *pattern_ptr = compute_store_pattern(tid, w_ptr, cur_sweep_id);
            *w_ptr = *pattern_ptr;
            w_ptr = w_ptr.add(1);
        }

        while corruption_found == 0 && timeout == 0 {
            prev_sweep_id = cur_sweep_id;
            cur_sweep_id = cur_sweep_id.wrapping_add(1);

            w_ptr = chunk_start as *mut c_uint;
            while (w_ptr as c_ulong) < (chunk_start as c_ulong).wrapping_add(size as c_ulong) {
                let old_pattern: c_uint;

                /*
                 * Compute the pattern that we would have
                 * stored at this location in the previous
                 * sweep.
                 */
                old_pattern = compute_store_pattern(tid, w_ptr, prev_sweep_id);

                /*
                 * FLUSH:Ensure that we flush the contents of
                 *       the cache before loading
                 */
                dcbf(w_ptr); //Flush

                /* LOAD: Read the value */
                read_data = *w_ptr; //Load

                /*
                 * COMPARE: Is it the same as what we had stored
                 *          in the previous sweep ? It better be!
                 */
                if read_data != old_pattern {
                    /* No it isn't! Tell everyone */
                    corruption_found = 1;
                }

                /*
                 * Before performing a store, let us check if
                 * any rim_thread has found a corruption.
                 */
                if corruption_found != 0 || timeout != 0 {
                    /*
                     * Yes. Someone (including us!) has found
                     * a corruption :(
                     *
                     * Let us verify that our chunk is
                     * correct.
                     */
                    /* But first, let us allow the dust to settle down! */
                    verify_chunk(tid, w_ptr, cur_sweep_id, prev_sweep_id);

                    return null_mut();
                }

                /*
                 * Compute the new pattern that we are going
                 * to write to this location
                 */
                *pattern_ptr = compute_store_pattern(tid, w_ptr, cur_sweep_id);

                /*
                 * STORE: Now let us write this pattern into
                 *        the location
                 */
                *w_ptr = *pattern_ptr;
                w_ptr = w_ptr.add(1);
            }
        }

        null_mut()
    }
}

static mut start_cpu: c_ulong = 0;
static mut nrthreads: c_ulong = 4;

static mut mem_snapshot_thread: pthread_t = 0;

extern "C" fn mem_snapshot_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        let page_size: c_int = getpagesize();
        let size: size_t = page_size as size_t;
        let tmp: *mut c_void = malloc(size);

        while corruption_found == 0 && timeout == 0 {
            /* Stop memory migration once corruption is found */
            segv_wait = 1;

            mprotect(map1 as *mut c_void, size, PROT_READ);

            /*
             * Load from the working alias (map1). Loading from map2
             * also fails.
             */
            memcpy(tmp, map1 as *const c_void, size);

            /*
             * Stores must go via map2 which has write permissions, but
             * the corrupted data tends to be seen in the snapshot buffer,
             * so corruption does not appear to be introduced at the
             * copy-back via map2 alias here.
             */
            memcpy(map2 as *mut c_void, tmp, size);
            /*
             * Before releasing other threads, must ensure the copy
             * back to
             */
            asm!("sync", options(nostack, preserves_flags));
            mprotect(map1 as *mut c_void, size, PROT_READ | PROT_WRITE);
            asm!("sync", options(nostack, preserves_flags));
            segv_wait = 0;

            usleep(1); /* This value makes a big difference */
        }

        null_mut()
    }
}

extern "C" fn alrm_sighandler(_sig: c_int) {
    unsafe {
        timeout = 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut c: c_int;
        let page_size: c_int = getpagesize();
        let mut now: time_t = 0;
        let mut i: c_int;
        let dir_error: c_int;
        let mut attr: pthread_attr_t = zeroed();
        let shm_key: key_t = getpid() as key_t;
        let mut shmid: c_int;
        let mut run_time: c_int = 20 * 60;
        let mut sa_alrm: sigaction = zeroed();

        snprintf(
            logdir.as_mut_ptr(),
            LOGDIR_NAME_SIZE,
            c"/tmp/logdir-%u".as_ptr(),
            getpid() as c_uint,
        );
        loop {
            c = getopt(argc, argv, c"r:hn:l:t:".as_ptr());
            if c == -1 {
                break;
            }
            match c as u8 as char {
                'r' => {
                    start_cpu = strtoul(optarg, null_mut(), 10);
                }
                'h' => {
                    printf(
                        c"%s [-r <start_cpu>] [-n <nrthreads>] [-l <logdir>] [-t <timeout>]\n"
                            .as_ptr(),
                        *argv,
                    );
                    exit(0);
                }
                'n' => {
                    nrthreads = strtoul(optarg, null_mut(), 10);
                }
                'l' => {
                    strncpy(logdir.as_mut_ptr(), optarg, LOGDIR_NAME_SIZE - 1);
                }
                't' => {
                    run_time = strtoul(optarg, null_mut(), 10) as c_int;
                }
                _ => {
                    printf(c"invalid option\n".as_ptr());
                    exit(0);
                }
            }
        }

        if nrthreads > MAX_THREADS as c_ulong {
            nrthreads = MAX_THREADS as c_ulong;
        }

        shmid = shmget(shm_key, page_size as size_t, IPC_CREAT | 0o666);
        if shmid < 0 {
            err_msg(c"Failed shmget\n".as_ptr() as *mut c_char);
        }

        map1 = shmat(shmid, null(), 0) as *mut c_char;
        if map1 == (-1_isize) as *mut c_char {
            err_msg(c"Failed shmat".as_ptr() as *mut c_char);
        }

        map2 = shmat(shmid, null(), 0) as *mut c_char;
        if map2 == (-1_isize) as *mut c_char {
            err_msg(c"Failed shmat".as_ptr() as *mut c_char);
        }

        dir_error = mkdir(logdir.as_ptr(), 0o755);

        if dir_error != 0 {
            err_msg(c"Failed mkdir".as_ptr() as *mut c_char);
        }

        printf(c"start_cpu list:%lu\n".as_ptr(), start_cpu);
        printf(c"number of worker threads:%lu + 1 snapshot thread\n".as_ptr(), nrthreads);
        printf(
            c"Allocated address:0x%016lx + secondary map:0x%016lx\n".as_ptr(),
            map1 as c_ulong,
            map2 as c_ulong,
        );
        printf(c"logdir at : %s\n".as_ptr(), logdir.as_ptr());
        printf(c"Timeout: %d seconds\n".as_ptr(), run_time);

        time(&mut now);
        printf(c"=================================\n".as_ptr());
        printf(c"     Starting Test\n".as_ptr());
        printf(c"     %s".as_ptr(), ctime(&now));
        printf(c"=================================\n".as_ptr());

        i = 0;
        while (i as c_ulong) < nrthreads {
            if 1 != 0 && fork() == 0 {
                prctl(PR_SET_PDEATHSIG, SIGKILL);
                set_mycpu((start_cpu + i as c_ulong) as c_int);
                loop {
                    sched_yield();
                }
            }
            i += 1;
        }

        sa_alrm.handler.sa_handler = Some(alrm_sighandler);
        sigemptyset(&mut sa_alrm.sa_mask);
        sa_alrm.sa_flags = 0;

        if sigaction(SIGALRM, &sa_alrm, null_mut()) == -1 {
            err_msg(c"Failed signal handler registration\n".as_ptr() as *mut c_char);
        }

        alarm(run_time as c_uint);

        pthread_attr_init(&mut attr);
        i = 0;
        while (i as c_ulong) < nrthreads {
            rim_thread_ids[i as usize] = i as c_uint;
            pthread_create(
                &mut rim_threads[i as usize],
                &attr,
                rim_fn,
                &mut rim_thread_ids[i as usize] as *mut c_uint as *mut c_void,
            );
            set_pthread_cpu(rim_threads[i as usize], (start_cpu + i as c_ulong) as c_int);
            i += 1;
        }

        pthread_create(&mut mem_snapshot_thread, &attr, mem_snapshot_fn, map1 as *mut c_void);
        set_pthread_cpu(mem_snapshot_thread, (start_cpu + i as c_ulong) as c_int);

        pthread_join(mem_snapshot_thread, null_mut());
        i = 0;
        while (i as c_ulong) < nrthreads {
            pthread_join(rim_threads[i as usize], null_mut());
            i += 1;
        }

        if timeout == 0 {
            time(&mut now);
            printf(c"=================================\n".as_ptr());
            printf(c"      Data Corruption Detected\n".as_ptr());
            printf(c"      %s".as_ptr(), ctime(&now));
            printf(c"      See logfiles in %s\n".as_ptr(), logdir.as_ptr());
            printf(c"=================================\n".as_ptr());
            return 1;
        }
        0
    }
}

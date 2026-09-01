// SPDX-License-Identifier: LGPL-2.1
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::mem::{size_of, zeroed};
use std::ptr;

type pid_t = c_int;
type pthread_t = usize;
type intptr_t = isize;
type uint64_t = u64;
type size_t = usize;

const __NR_gettid: c_long = 186;
const __NR_membarrier: c_long = 324;
const SIGUSR1: c_int = 10;
const ENXIO: c_int = 6;
const CPU_SETSIZE: usize = 1024;
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: c_int = 0x8;
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: c_int = 0x10;
const MEMBARRIER_CMD_FLAG_CPU: c_int = 0x1;
const __ATOMIC_ACQUIRE: c_int = 2;
const __ATOMIC_RELEASE: c_int = 3;

#[repr(C)]
struct cpu_set_t {
    bits: [u64; CPU_SETSIZE / 64],
}

#[repr(C)]
struct sigset_t {
    bits: [u64; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum rseq_mo {
    RSEQ_MO_RELAXED = 0,
    RSEQ_MO_RELEASE = 1,
}

const RSEQ_MO_RELAXED: rseq_mo = rseq_mo::RSEQ_MO_RELAXED;
const RSEQ_MO_RELEASE: rseq_mo = rseq_mo::RSEQ_MO_RELEASE;
const RSEQ_PERCPU_CPU_ID: c_int = 0;
const RSEQ_PERCPU_MM_CID: c_int = 1;

// From "rseq.h" and C library/kernel headers.
unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn getpid() -> pid_t;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn abort() -> !;
    fn atol(nptr: *const c_char) -> c_long;
    fn atoll(nptr: *const c_char) -> i64;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sched_getaffinity(pid: pid_t, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_yield() -> c_int;
    fn poll(fds: *mut c_void, nfds: c_uint, timeout: c_int) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    static mut stderr: *mut c_void;
    static mut errno: c_int;

    fn __rseq_register_current_thread(no_glibc: bool, legacy: bool) -> c_int;
    fn rseq_unregister_current_thread() -> c_int;
    fn rseq_current_mm_cid() -> c_int;
    fn rseq_mm_cid_available() -> bool;
    fn rseq_cpu_start() -> c_int;
    fn rseq_current_cpu_raw() -> c_int;
    fn rseq_cmpeqv_storev(mo: rseq_mo, percpu: c_int, v: *mut intptr_t,
                          expect: intptr_t, newv: intptr_t, cpu: c_int) -> c_int;
    fn rseq_addv(mo: rseq_mo, percpu: c_int, v: *mut intptr_t,
                 count: intptr_t, cpu: c_int) -> c_int;
    fn rseq_cmpnev_storeoffp_load(mo: rseq_mo, percpu: c_int, v: *mut intptr_t,
                                  expectnot: intptr_t, voffp: c_long,
                                  load: *mut intptr_t, cpu: c_int) -> c_int;
    fn rseq_cmpeqv_trystorev_storev(mo: rseq_mo, percpu: c_int,
                                    v: *mut intptr_t, expect: intptr_t,
                                    v2: *mut intptr_t, newv2: intptr_t,
                                    newv: intptr_t, cpu: c_int) -> c_int;
    fn rseq_cmpeqv_cmpeqv_storev(mo: rseq_mo, percpu: c_int,
                                 v: *mut intptr_t, expect: intptr_t,
                                 v2: *mut intptr_t, expect2: intptr_t,
                                 newv: intptr_t, cpu: c_int) -> c_int;
    fn rseq_cmpeqv_trymemcpy_storev(mo: rseq_mo, percpu: c_int,
                                    v: *mut intptr_t, expect: intptr_t,
                                    dst: *mut c_char, src: *mut c_char,
                                    len: size_t, newv: intptr_t, cpu: c_int) -> c_int;
    fn rseq_offset_deref_addv(mo: rseq_mo, percpu: c_int,
                              ptr: *mut intptr_t, off: size_t,
                              inc: intptr_t, cpu: c_int) -> c_int;
    fn rseq_smp_acquire__after_ctrl_dep();
    fn rseq_smp_store_release(p: *mut intptr_t, v: intptr_t);
    fn rseq_barrier();
}

#[inline]
unsafe fn rseq_gettid() -> pid_t {
    syscall(__NR_gettid) as pid_t
}

const NR_INJECT: usize = 9;
static mut loop_cnt: [c_int; NR_INJECT + 1] = [0; NR_INJECT + 1];

#[unsafe(export_name = "asm_loop_cnt_1")]
static mut loop_cnt_1: c_int = 0;
#[unsafe(export_name = "asm_loop_cnt_2")]
static mut loop_cnt_2: c_int = 0;
#[unsafe(export_name = "asm_loop_cnt_3")]
static mut loop_cnt_3: c_int = 0;
#[unsafe(export_name = "asm_loop_cnt_4")]
static mut loop_cnt_4: c_int = 0;
#[unsafe(export_name = "asm_loop_cnt_5")]
static mut loop_cnt_5: c_int = 0;
#[unsafe(export_name = "asm_loop_cnt_6")]
static mut loop_cnt_6: c_int = 0;

static mut opt_modulo: c_int = 0;
static mut verbose: c_int = 0;
static mut opt_yield: c_int = 0;
static mut opt_signal: c_int = 0;
static mut opt_sleep: c_int = 0;
static mut opt_disable_rseq: c_int = 0;
static mut opt_threads: c_int = 200;
static mut opt_disable_mod: c_int = 0;
static mut opt_test: c_int = b's' as c_int;
static mut opt_rseq_legacy: bool = false;
static mut opt_reps: i64 = 5000;

thread_local! {
    static signals_delivered: std::cell::Cell<c_uint> = const { std::cell::Cell::new(0) };
    static yield_mod_cnt: std::cell::Cell<c_uint> = const { std::cell::Cell::new(0) };
    static nr_abort: std::cell::Cell<c_uint> = const { std::cell::Cell::new(0) };
}

// Architecture-specific RSEQ_INJECT_* assembly macros from C are provided by rseq.h users.
// The C RSEQ_INJECT_FAILED macro increments nr_abort.
// The C RSEQ_INJECT_C macro performs delay loops and optional sleep/yield/signal injection.
const rseq_no_glibc: bool = true;

static mut opt_mo: rseq_mo = RSEQ_MO_RELAXED;

#[inline]
unsafe fn sys_membarrier(cmd: c_int, flags: c_int, cpu_id: c_int) -> c_int {
    syscall(__NR_membarrier, cmd, flags, cpu_id) as c_int
}

// BUILDOPT_RSEQ_PERCPU_MM_CID selects mm_cid in C; otherwise use cpu_id as index.
const RSEQ_PERCPU: c_int = RSEQ_PERCPU_CPU_ID;

#[inline]
unsafe fn get_current_cpu_id() -> c_int {
    rseq_cpu_start()
}

#[inline]
unsafe fn rseq_validate_cpu_id() -> bool {
    rseq_current_cpu_raw() >= 0
}

#[inline]
unsafe fn rseq_use_cpu_index() -> bool {
    true
}

#[inline]
unsafe fn rseq_membarrier_expedited(cpu: c_int) -> c_int {
    sys_membarrier(MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ, MEMBARRIER_CMD_FLAG_CPU, cpu)
}

#[repr(C, align(128))]
struct percpu_lock_entry {
    v: intptr_t,
}

#[repr(C)]
struct percpu_lock {
    c: [percpu_lock_entry; CPU_SETSIZE],
}

#[repr(C, align(128))]
struct test_data_entry {
    count: intptr_t,
}

#[repr(C)]
struct spinlock_test_data {
    lock: percpu_lock,
    c: [test_data_entry; CPU_SETSIZE],
}

#[repr(C)]
struct spinlock_thread_test_data {
    data: *mut spinlock_test_data,
    reps: i64,
    reg: c_int,
}

#[repr(C)]
struct inc_test_data {
    c: [test_data_entry; CPU_SETSIZE],
}

#[repr(C)]
struct inc_thread_test_data {
    data: *mut inc_test_data,
    reps: i64,
    reg: c_int,
}

#[repr(C)]
struct percpu_list_node {
    data: intptr_t,
    next: *mut percpu_list_node,
}

#[repr(C, align(128))]
struct percpu_list_entry {
    head: *mut percpu_list_node,
}

#[repr(C)]
struct percpu_list {
    c: [percpu_list_entry; CPU_SETSIZE],
}

const BUFFER_ITEM_PER_CPU: usize = 100;

#[repr(C)]
struct percpu_buffer_node {
    data: intptr_t,
}

#[repr(C, align(128))]
struct percpu_buffer_entry {
    offset: intptr_t,
    buflen: intptr_t,
    array: *mut *mut percpu_buffer_node,
}

#[repr(C)]
struct percpu_buffer {
    c: [percpu_buffer_entry; CPU_SETSIZE],
}

const MEMCPY_BUFFER_ITEM_PER_CPU: usize = 100;

#[repr(C)]
#[derive(Copy, Clone)]
struct percpu_memcpy_buffer_node {
    data1: intptr_t,
    data2: uint64_t,
}

#[repr(C, align(128))]
struct percpu_memcpy_buffer_entry {
    offset: intptr_t,
    buflen: intptr_t,
    array: *mut percpu_memcpy_buffer_node,
}

#[repr(C)]
struct percpu_memcpy_buffer {
    c: [percpu_memcpy_buffer_entry; CPU_SETSIZE],
}

#[inline]
unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool {
    let cpu = cpu as usize;
    ((*set).bits[cpu / 64] & (1u64 << (cpu % 64))) != 0
}

#[inline]
unsafe fn RSEQ_READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

#[inline]
fn rseq_likely(v: bool) -> bool {
    v
}

#[inline]
fn rseq_unlikely(v: c_int) -> bool {
    v != 0
}

unsafe fn printf_verbose0(s: *const c_char) {
    if verbose != 0 {
        printf(s);
    }
}

/* A simple percpu spinlock. Grabs lock on current cpu. */
unsafe fn rseq_this_cpu_lock(lock: *mut percpu_lock) -> c_int {
    let mut cpu: c_int;

    loop {
        let ret: c_int;

        cpu = get_current_cpu_id();
        if cpu < 0 {
            fprintf(stderr, c"pid: %d: tid: %d, cpu: %d: cid: %d\n".as_ptr(),
                    getpid(), rseq_gettid() as c_int, rseq_current_cpu_raw(), cpu);
            abort();
        }
        ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                 &mut (*lock).c[cpu as usize].v,
                                 0, 1, cpu);
        if rseq_likely(ret == 0) {
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    /*
     * Acquire semantic when taking lock after control dependency.
     * Matches rseq_smp_store_release().
     */
    rseq_smp_acquire__after_ctrl_dep();
    cpu
}

unsafe fn rseq_percpu_unlock(lock: *mut percpu_lock, cpu: c_int) {
    assert!((*lock).c[cpu as usize].v == 1);
    /*
     * Release lock, with release semantic. Matches
     * rseq_smp_acquire__after_ctrl_dep().
     */
    rseq_smp_store_release(&mut (*lock).c[cpu as usize].v, 0);
}

pub unsafe extern "C" fn test_percpu_spinlock_thread(arg: *mut c_void) -> *mut c_void {
    let thread_data = arg as *mut spinlock_thread_test_data;
    let data = (*thread_data).data;
    let mut i: i64;
    let reps: i64;

    if opt_disable_rseq == 0 && (*thread_data).reg != 0 &&
        __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        abort();
    }
    reps = (*thread_data).reps;
    i = 0;
    while i < reps {
        let cpu = rseq_this_cpu_lock(&mut (*data).lock);
        (*data).c[cpu as usize].count += 1;
        rseq_percpu_unlock(&mut (*data).lock, cpu);
        if i != 0 && reps / 10 != 0 && i % (reps / 10) == 0 && verbose != 0 {
            printf(c"tid %d: count %lld\n".as_ptr(), rseq_gettid() as c_int, i);
        }
        i += 1;
    }
    if verbose != 0 {
        nr_abort.with(|n| signals_delivered.with(|s| {
            printf(c"tid %d: number of rseq abort: %d, signals delivered: %u\n".as_ptr(),
                   rseq_gettid() as c_int, n.get(), s.get());
        }));
    }
    if opt_disable_rseq == 0 && (*thread_data).reg != 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }
    ptr::null_mut()
}

/*
 * A simple test which implements a sharded counter using a per-cpu
 * lock.  Obviously real applications might prefer to simply use a
 * per-cpu increment; however, this is reasonable for a test and the
 * lock can be extended to synchronize more complicated operations.
 */
pub unsafe fn test_percpu_spinlock() {
    let num_threads = opt_threads as usize;
    let mut i: usize;
    let mut ret: c_int;
    let mut sum: uint64_t;
    let mut test_threads = vec![0 as pthread_t; num_threads];
    let mut data: spinlock_test_data = zeroed();
    let mut thread_data: Vec<spinlock_thread_test_data> = (0..num_threads).map(|_| zeroed()).collect();

    memset(&mut data as *mut _ as *mut c_void, 0, size_of::<spinlock_test_data>());
    i = 0;
    while i < num_threads {
        thread_data[i].reps = opt_reps;
        thread_data[i].reg = if opt_disable_mod <= 0 || (i as c_int % opt_disable_mod) != 0 { 1 } else { 0 };
        thread_data[i].data = &mut data;
        ret = pthread_create(&mut test_threads[i], ptr::null(),
                             test_percpu_spinlock_thread,
                             &mut thread_data[i] as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(test_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    sum = 0;
    i = 0;
    while i < CPU_SETSIZE {
        sum += data.c[i].count as uint64_t;
        i += 1;
    }

    assert!(sum == opt_reps as uint64_t * num_threads as uint64_t);
}

pub unsafe extern "C" fn test_percpu_inc_thread(arg: *mut c_void) -> *mut c_void {
    let thread_data = arg as *mut inc_thread_test_data;
    let data = (*thread_data).data;
    let mut i: i64;
    let reps: i64;

    if opt_disable_rseq == 0 && (*thread_data).reg != 0 &&
        __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        abort();
    }
    reps = (*thread_data).reps;
    i = 0;
    while i < reps {
        let mut ret: c_int;

        loop {
            let cpu: c_int = get_current_cpu_id();
            ret = rseq_addv(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                            &mut (*data).c[cpu as usize].count, 1, cpu);
            if !rseq_unlikely(ret) {
                break;
            }
        }
        if i != 0 && reps / 10 != 0 && i % (reps / 10) == 0 && verbose != 0 {
            printf(c"tid %d: count %lld\n".as_ptr(), rseq_gettid() as c_int, i);
        }
        i += 1;
    }
    if verbose != 0 {
        nr_abort.with(|n| signals_delivered.with(|s| {
            printf(c"tid %d: number of rseq abort: %d, signals delivered: %u\n".as_ptr(),
                   rseq_gettid() as c_int, n.get(), s.get());
        }));
    }
    if opt_disable_rseq == 0 && (*thread_data).reg != 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }
    ptr::null_mut()
}

pub unsafe fn test_percpu_inc() {
    let num_threads = opt_threads as usize;
    let mut i: usize;
    let mut ret: c_int;
    let mut sum: uint64_t;
    let mut test_threads = vec![0 as pthread_t; num_threads];
    let mut data: inc_test_data = zeroed();
    let mut thread_data: Vec<inc_thread_test_data> = (0..num_threads).map(|_| zeroed()).collect();

    memset(&mut data as *mut _ as *mut c_void, 0, size_of::<inc_test_data>());
    i = 0;
    while i < num_threads {
        thread_data[i].reps = opt_reps;
        thread_data[i].reg = if opt_disable_mod <= 0 || (i as c_int % opt_disable_mod) != 0 { 1 } else { 0 };
        thread_data[i].data = &mut data;
        ret = pthread_create(&mut test_threads[i], ptr::null(), test_percpu_inc_thread,
                             &mut thread_data[i] as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(test_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    sum = 0;
    i = 0;
    while i < CPU_SETSIZE {
        sum += data.c[i].count as uint64_t;
        i += 1;
    }

    assert!(sum == opt_reps as uint64_t * num_threads as uint64_t);
}

pub unsafe fn this_cpu_list_push(list: *mut percpu_list, node: *mut percpu_list_node, _cpu: *mut c_int) {
    let mut cpu: c_int;

    loop {
        let targetptr: *mut intptr_t;
        let newval: intptr_t;
        let expect: intptr_t;
        let ret: c_int;

        cpu = get_current_cpu_id();
        /* Load list->c[cpu].head with single-copy atomicity. */
        expect = RSEQ_READ_ONCE(&(*list).c[cpu as usize].head) as intptr_t;
        newval = node as intptr_t;
        targetptr = &mut (*list).c[cpu as usize].head as *mut _ as *mut intptr_t;
        (*node).next = expect as *mut percpu_list_node;
        ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                 targetptr, expect, newval, cpu);
        if rseq_likely(ret == 0) {
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
}

/*
 * Unlike a traditional lock-less linked list; the availability of a
 * rseq primitive allows us to implement pop without concerns over
 * ABA-type races.
 */
pub unsafe fn this_cpu_list_pop(list: *mut percpu_list, _cpu: *mut c_int) -> *mut percpu_list_node {
    let mut node: *mut percpu_list_node = ptr::null_mut();
    let mut cpu: c_int;

    loop {
        let mut head: *mut percpu_list_node = ptr::null_mut();
        let targetptr: *mut intptr_t;
        let expectnot: intptr_t;
        let load: *mut intptr_t;
        let offset: c_long;
        let ret: c_int;

        cpu = get_current_cpu_id();
        targetptr = &mut (*list).c[cpu as usize].head as *mut _ as *mut intptr_t;
        expectnot = ptr::null_mut::<c_void>() as intptr_t;
        offset = std::mem::offset_of!(percpu_list_node, next) as c_long;
        load = &mut head as *mut _ as *mut intptr_t;
        ret = rseq_cmpnev_storeoffp_load(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                         targetptr, expectnot,
                                         offset, load, cpu);
        if rseq_likely(ret == 0) {
            node = head;
            break;
        }
        if ret > 0 {
            break;
        }
        /* Retry if rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
    node
}

/*
 * __percpu_list_pop is not safe against concurrent accesses. Should
 * only be used on lists that are not concurrently modified.
 */
pub unsafe fn __percpu_list_pop(list: *mut percpu_list, cpu: c_int) -> *mut percpu_list_node {
    let node: *mut percpu_list_node;

    node = (*list).c[cpu as usize].head;
    if node.is_null() {
        return ptr::null_mut();
    }
    (*list).c[cpu as usize].head = (*node).next;
    node
}

pub unsafe extern "C" fn test_percpu_list_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: i64;
    let reps: i64;
    let list = arg as *mut percpu_list;

    if opt_disable_rseq == 0 && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        abort();
    }

    reps = opt_reps;
    i = 0;
    while i < reps {
        let node: *mut percpu_list_node;

        node = this_cpu_list_pop(list, ptr::null_mut());
        if opt_yield != 0 {
            sched_yield();  /* encourage shuffling */
        }
        if !node.is_null() {
            this_cpu_list_push(list, node, ptr::null_mut());
        }
        i += 1;
    }

    if verbose != 0 {
        nr_abort.with(|n| signals_delivered.with(|s| {
            printf(c"tid %d: number of rseq abort: %d, signals delivered: %u\n".as_ptr(),
                   rseq_gettid() as c_int, n.get(), s.get());
        }));
    }
    if opt_disable_rseq == 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }

    ptr::null_mut()
}

/* Simultaneous modification to a per-cpu linked list from many threads.  */
pub unsafe fn test_percpu_list() {
    let num_threads = opt_threads as usize;
    let mut i: usize;
    let mut j: c_int;
    let mut ret: c_int;
    let mut sum: uint64_t = 0;
    let mut expected_sum: uint64_t = 0;
    let mut list: percpu_list = zeroed();
    let mut test_threads = vec![0 as pthread_t; num_threads];
    let mut allowed_cpus: cpu_set_t = zeroed();

    memset(&mut list as *mut _ as *mut c_void, 0, size_of::<percpu_list>());

    /* Generate list entries for every usable cpu. */
    sched_getaffinity(0, size_of::<cpu_set_t>(), &mut allowed_cpus);
    i = 0;
    while i < CPU_SETSIZE {
        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }
        j = 1;
        while j <= 100 {
            let node: *mut percpu_list_node;

            expected_sum += j as uint64_t;

            node = malloc(size_of::<percpu_list_node>()) as *mut percpu_list_node;
            assert!(!node.is_null());
            (*node).data = j as intptr_t;
            (*node).next = list.c[i].head;
            list.c[i].head = node;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_create(&mut test_threads[i], ptr::null(), test_percpu_list_thread,
                             &mut list as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(test_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < CPU_SETSIZE {
        let mut node: *mut percpu_list_node;

        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }

        loop {
            node = __percpu_list_pop(&mut list, i as c_int);
            if node.is_null() {
                break;
            }
            sum += (*node).data as uint64_t;
            free(node as *mut c_void);
        }
        i += 1;
    }

    /*
     * All entries should now be accounted for (unless some external
     * actor is interfering with our allowed affinity while this
     * test is running).
     */
    assert!(sum == expected_sum);
}

pub unsafe fn this_cpu_buffer_push(buffer: *mut percpu_buffer, node: *mut percpu_buffer_node, _cpu: *mut c_int) -> bool {
    let mut result = false;
    let mut cpu: c_int;

    loop {
        let targetptr_spec: *mut intptr_t;
        let newval_spec: intptr_t;
        let targetptr_final: *mut intptr_t;
        let newval_final: intptr_t;
        let offset: intptr_t;
        let ret: c_int;

        cpu = get_current_cpu_id();
        offset = RSEQ_READ_ONCE(&(*buffer).c[cpu as usize].offset);
        if offset == (*buffer).c[cpu as usize].buflen {
            break;
        }
        newval_spec = node as intptr_t;
        targetptr_spec = (*buffer).c[cpu as usize].array.offset(offset) as *mut intptr_t;
        newval_final = offset + 1;
        targetptr_final = &mut (*buffer).c[cpu as usize].offset;
        ret = rseq_cmpeqv_trystorev_storev(opt_mo, RSEQ_PERCPU,
                                           targetptr_final, offset, targetptr_spec,
                                           newval_spec, newval_final, cpu);
        if rseq_likely(ret == 0) {
            result = true;
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
    result
}

pub unsafe fn this_cpu_buffer_pop(buffer: *mut percpu_buffer, _cpu: *mut c_int) -> *mut percpu_buffer_node {
    let mut head: *mut percpu_buffer_node;
    let mut cpu: c_int;

    loop {
        let targetptr: *mut intptr_t;
        let newval: intptr_t;
        let offset: intptr_t;
        let ret: c_int;

        cpu = get_current_cpu_id();
        /* Load offset with single-copy atomicity. */
        offset = RSEQ_READ_ONCE(&(*buffer).c[cpu as usize].offset);
        if offset == 0 {
            head = ptr::null_mut();
            break;
        }
        head = RSEQ_READ_ONCE((*buffer).c[cpu as usize].array.offset(offset - 1));
        newval = offset - 1;
        targetptr = &mut (*buffer).c[cpu as usize].offset;
        ret = rseq_cmpeqv_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                        targetptr, offset,
                                        (*buffer).c[cpu as usize].array.offset(offset - 1) as *mut intptr_t,
                                        head as intptr_t, newval, cpu);
        if rseq_likely(ret == 0) {
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
    head
}

/*
 * __percpu_buffer_pop is not safe against concurrent accesses. Should
 * only be used on buffers that are not concurrently modified.
 */
pub unsafe fn __percpu_buffer_pop(buffer: *mut percpu_buffer, cpu: c_int) -> *mut percpu_buffer_node {
    let head: *mut percpu_buffer_node;
    let offset: intptr_t;

    offset = (*buffer).c[cpu as usize].offset;
    if offset == 0 {
        return ptr::null_mut();
    }
    head = *(*buffer).c[cpu as usize].array.offset(offset - 1);
    (*buffer).c[cpu as usize].offset = offset - 1;
    head
}

pub unsafe extern "C" fn test_percpu_buffer_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: i64;
    let reps: i64;
    let buffer = arg as *mut percpu_buffer;

    if opt_disable_rseq == 0 && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        abort();
    }

    reps = opt_reps;
    i = 0;
    while i < reps {
        let node: *mut percpu_buffer_node;

        node = this_cpu_buffer_pop(buffer, ptr::null_mut());
        if opt_yield != 0 {
            sched_yield();  /* encourage shuffling */
        }
        if !node.is_null() {
            if !this_cpu_buffer_push(buffer, node, ptr::null_mut()) {
                /* Should increase buffer size. */
                abort();
            }
        }
        i += 1;
    }

    if verbose != 0 {
        nr_abort.with(|n| signals_delivered.with(|s| {
            printf(c"tid %d: number of rseq abort: %d, signals delivered: %u\n".as_ptr(),
                   rseq_gettid() as c_int, n.get(), s.get());
        }));
    }
    if opt_disable_rseq == 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }

    ptr::null_mut()
}

/* Simultaneous modification to a per-cpu buffer from many threads.  */
pub unsafe fn test_percpu_buffer() {
    let num_threads = opt_threads as usize;
    let mut i: usize;
    let mut j: c_int;
    let mut ret: c_int;
    let mut sum: uint64_t = 0;
    let mut expected_sum: uint64_t = 0;
    let mut buffer: percpu_buffer = zeroed();
    let mut test_threads = vec![0 as pthread_t; num_threads];
    let mut allowed_cpus: cpu_set_t = zeroed();

    memset(&mut buffer as *mut _ as *mut c_void, 0, size_of::<percpu_buffer>());

    /* Generate list entries for every usable cpu. */
    sched_getaffinity(0, size_of::<cpu_set_t>(), &mut allowed_cpus);
    i = 0;
    while i < CPU_SETSIZE {
        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }
        /* Worse-case is every item in same CPU. */
        buffer.c[i].array =
            malloc(size_of::<*mut percpu_buffer_node>() * CPU_SETSIZE *
                   BUFFER_ITEM_PER_CPU) as *mut *mut percpu_buffer_node;
        assert!(!buffer.c[i].array.is_null());
        buffer.c[i].buflen = (CPU_SETSIZE * BUFFER_ITEM_PER_CPU) as intptr_t;
        j = 1;
        while j <= BUFFER_ITEM_PER_CPU as c_int {
            let node: *mut percpu_buffer_node;

            expected_sum += j as uint64_t;

            /*
             * We could theoretically put the word-sized
             * "data" directly in the buffer. However, we
             * want to model objects that would not fit
             * within a single word, so allocate an object
             * for each node.
             */
            node = malloc(size_of::<percpu_buffer_node>()) as *mut percpu_buffer_node;
            assert!(!node.is_null());
            (*node).data = j as intptr_t;
            *buffer.c[i].array.offset((j - 1) as isize) = node;
            buffer.c[i].offset += 1;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_create(&mut test_threads[i], ptr::null(), test_percpu_buffer_thread,
                             &mut buffer as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(test_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < CPU_SETSIZE {
        let mut node: *mut percpu_buffer_node;

        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }

        loop {
            node = __percpu_buffer_pop(&mut buffer, i as c_int);
            if node.is_null() {
                break;
            }
            sum += (*node).data as uint64_t;
            free(node as *mut c_void);
        }
        free(buffer.c[i].array as *mut c_void);
        i += 1;
    }

    /*
     * All entries should now be accounted for (unless some external
     * actor is interfering with our allowed affinity while this
     * test is running).
     */
    assert!(sum == expected_sum);
}

pub unsafe fn this_cpu_memcpy_buffer_push(buffer: *mut percpu_memcpy_buffer,
                                          item: percpu_memcpy_buffer_node,
                                          _cpu: *mut c_int) -> bool {
    let mut result = false;
    let mut cpu: c_int;

    loop {
        let targetptr_final: *mut intptr_t;
        let newval_final: intptr_t;
        let offset: intptr_t;
        let destptr: *mut c_char;
        let srcptr: *mut c_char;
        let copylen: size_t;
        let ret: c_int;

        cpu = get_current_cpu_id();
        /* Load offset with single-copy atomicity. */
        offset = RSEQ_READ_ONCE(&(*buffer).c[cpu as usize].offset);
        if offset == (*buffer).c[cpu as usize].buflen {
            break;
        }
        destptr = &mut *(*buffer).c[cpu as usize].array.offset(offset) as *mut _ as *mut c_char;
        srcptr = &item as *const _ as *mut c_char;
        /* copylen must be <= 4kB. */
        copylen = size_of::<percpu_memcpy_buffer_node>();
        newval_final = offset + 1;
        targetptr_final = &mut (*buffer).c[cpu as usize].offset;
        ret = rseq_cmpeqv_trymemcpy_storev(
            opt_mo, RSEQ_PERCPU,
            targetptr_final, offset,
            destptr, srcptr, copylen,
            newval_final, cpu);
        if rseq_likely(ret == 0) {
            result = true;
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
    result
}

pub unsafe fn this_cpu_memcpy_buffer_pop(buffer: *mut percpu_memcpy_buffer,
                                         item: *mut percpu_memcpy_buffer_node,
                                         _cpu: *mut c_int) -> bool {
    let mut result = false;
    let mut cpu: c_int;

    loop {
        let targetptr_final: *mut intptr_t;
        let newval_final: intptr_t;
        let offset: intptr_t;
        let destptr: *mut c_char;
        let srcptr: *mut c_char;
        let copylen: size_t;
        let ret: c_int;

        cpu = get_current_cpu_id();
        /* Load offset with single-copy atomicity. */
        offset = RSEQ_READ_ONCE(&(*buffer).c[cpu as usize].offset);
        if offset == 0 {
            break;
        }
        destptr = item as *mut c_char;
        srcptr = &mut *(*buffer).c[cpu as usize].array.offset(offset - 1) as *mut _ as *mut c_char;
        /* copylen must be <= 4kB. */
        copylen = size_of::<percpu_memcpy_buffer_node>();
        newval_final = offset - 1;
        targetptr_final = &mut (*buffer).c[cpu as usize].offset;
        ret = rseq_cmpeqv_trymemcpy_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                           targetptr_final, offset, destptr, srcptr, copylen,
                                           newval_final, cpu);
        if rseq_likely(ret == 0) {
            result = true;
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        *_cpu = cpu;
    }
    result
}

/*
 * __percpu_memcpy_buffer_pop is not safe against concurrent accesses. Should
 * only be used on buffers that are not concurrently modified.
 */
pub unsafe fn __percpu_memcpy_buffer_pop(buffer: *mut percpu_memcpy_buffer,
                                         item: *mut percpu_memcpy_buffer_node,
                                         cpu: c_int) -> bool {
    let offset: intptr_t;

    offset = (*buffer).c[cpu as usize].offset;
    if offset == 0 {
        return false;
    }
    memcpy(item as *mut c_void,
           &mut *(*buffer).c[cpu as usize].array.offset(offset - 1) as *mut _ as *const c_void,
           size_of::<percpu_memcpy_buffer_node>());
    (*buffer).c[cpu as usize].offset = offset - 1;
    true
}

pub unsafe extern "C" fn test_percpu_memcpy_buffer_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: i64;
    let reps: i64;
    let buffer = arg as *mut percpu_memcpy_buffer;

    if opt_disable_rseq == 0 && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        abort();
    }

    reps = opt_reps;
    i = 0;
    while i < reps {
        let mut item: percpu_memcpy_buffer_node = zeroed();
        let result: bool;

        result = this_cpu_memcpy_buffer_pop(buffer, &mut item, ptr::null_mut());
        if opt_yield != 0 {
            sched_yield();  /* encourage shuffling */
        }
        if result {
            if !this_cpu_memcpy_buffer_push(buffer, item, ptr::null_mut()) {
                /* Should increase buffer size. */
                abort();
            }
        }
        i += 1;
    }

    if verbose != 0 {
        nr_abort.with(|n| signals_delivered.with(|s| {
            printf(c"tid %d: number of rseq abort: %d, signals delivered: %u\n".as_ptr(),
                   rseq_gettid() as c_int, n.get(), s.get());
        }));
    }
    if opt_disable_rseq == 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }

    ptr::null_mut()
}

/* Simultaneous modification to a per-cpu buffer from many threads.  */
pub unsafe fn test_percpu_memcpy_buffer() {
    let num_threads = opt_threads as usize;
    let mut i: usize;
    let mut j: c_int;
    let mut ret: c_int;
    let mut sum: uint64_t = 0;
    let mut expected_sum: uint64_t = 0;
    let mut buffer: percpu_memcpy_buffer = zeroed();
    let mut test_threads = vec![0 as pthread_t; num_threads];
    let mut allowed_cpus: cpu_set_t = zeroed();

    memset(&mut buffer as *mut _ as *mut c_void, 0, size_of::<percpu_memcpy_buffer>());

    /* Generate list entries for every usable cpu. */
    sched_getaffinity(0, size_of::<cpu_set_t>(), &mut allowed_cpus);
    i = 0;
    while i < CPU_SETSIZE {
        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }
        /* Worse-case is every item in same CPU. */
        buffer.c[i].array =
            malloc(size_of::<percpu_memcpy_buffer_node>() * CPU_SETSIZE *
                   MEMCPY_BUFFER_ITEM_PER_CPU) as *mut percpu_memcpy_buffer_node;
        assert!(!buffer.c[i].array.is_null());
        buffer.c[i].buflen = (CPU_SETSIZE * MEMCPY_BUFFER_ITEM_PER_CPU) as intptr_t;
        j = 1;
        while j <= MEMCPY_BUFFER_ITEM_PER_CPU as c_int {
            expected_sum += (2 * j + 1) as uint64_t;

            /*
             * We could theoretically put the word-sized
             * "data" directly in the buffer. However, we
             * want to model objects that would not fit
             * within a single word, so allocate an object
             * for each node.
             */
            (*buffer.c[i].array.offset((j - 1) as isize)).data1 = j as intptr_t;
            (*buffer.c[i].array.offset((j - 1) as isize)).data2 = (j + 1) as uint64_t;
            buffer.c[i].offset += 1;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_create(&mut test_threads[i], ptr::null(),
                             test_percpu_memcpy_buffer_thread,
                             &mut buffer as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(test_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < CPU_SETSIZE {
        let mut item: percpu_memcpy_buffer_node = zeroed();

        if rseq_use_cpu_index() && !CPU_ISSET(i as c_int, &allowed_cpus) {
            i += 1;
            continue;
        }

        while __percpu_memcpy_buffer_pop(&mut buffer, &mut item, i as c_int) {
            sum += item.data1 as uint64_t;
            sum += item.data2;
        }
        free(buffer.c[i].array as *mut c_void);
        i += 1;
    }

    /*
     * All entries should now be accounted for (unless some external
     * actor is interfering with our allowed affinity while this
     * test is running).
     */
    assert!(sum == expected_sum);
}

unsafe extern "C" fn test_signal_interrupt_handler(signo: c_int) {
    signals_delivered.with(|s| s.set(s.get() + 1));
}

unsafe fn set_signal_handler() -> c_int {
    let mut ret: c_int = 0;
    let mut sa: sigaction = zeroed();
    let mut sigset: sigset_t = zeroed();

    ret = sigemptyset(&mut sigset);
    if ret < 0 {
        perror(c"sigemptyset".as_ptr());
        return ret;
    }

    sa.sa_handler = Some(test_signal_interrupt_handler);
    sa.sa_mask = sigset;
    sa.sa_flags = 0;
    ret = sigaction(SIGUSR1, &sa, ptr::null_mut());
    if ret < 0 {
        perror(c"sigaction".as_ptr());
        return ret;
    }

    printf_verbose0(c"Signal handler set for SIGUSR1\n".as_ptr());

    ret
}

/* Test MEMBARRIER_CMD_PRIVATE_RESTART_RSEQ_ON_CPU membarrier command. */
#[repr(C)]
struct test_membarrier_thread_args {
    stop: c_int,
    percpu_list_ptr: intptr_t,
}

/* Worker threads modify data in their "active" percpu lists. */
pub unsafe extern "C" fn test_membarrier_worker_thread(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut test_membarrier_thread_args;
    let iters: c_int = opt_reps as c_int;
    let mut i: c_int;

    if __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        fprintf(stderr, c"Error: rseq_register_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno, strerror(errno));
        abort();
    }

    /* Wait for initialization. */
    while ptr::read_volatile(&(*args).percpu_list_ptr) == 0 {}

    i = 0;
    while i < iters {
        let mut ret: c_int;

        loop {
            let cpu = get_current_cpu_id();

            ret = rseq_offset_deref_addv(RSEQ_MO_RELAXED, RSEQ_PERCPU,
                                         &mut (*args).percpu_list_ptr,
                                         size_of::<percpu_list_entry>() * cpu as usize, 1, cpu);
            if !rseq_unlikely(ret) {
                break;
            }
        }
        i += 1;
    }

    if rseq_unregister_current_thread() != 0 {
        fprintf(stderr, c"Error: rseq_unregister_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno, strerror(errno));
        abort();
    }
    ptr::null_mut()
}

pub unsafe fn test_membarrier_init_percpu_list(list: *mut percpu_list) {
    let mut i: usize;

    memset(list as *mut c_void, 0, size_of::<percpu_list>());
    i = 0;
    while i < CPU_SETSIZE {
        let node: *mut percpu_list_node;

        node = malloc(size_of::<percpu_list_node>()) as *mut percpu_list_node;
        assert!(!node.is_null());
        (*node).data = 0;
        (*node).next = ptr::null_mut();
        (*list).c[i].head = node;
        i += 1;
    }
}

pub unsafe fn test_membarrier_free_percpu_list(list: *mut percpu_list) {
    let mut i: usize = 0;

    while i < CPU_SETSIZE {
        free((*list).c[i].head as *mut c_void);
        i += 1;
    }
}

/*
 * The manager thread swaps per-cpu lists that worker threads see,
 * and validates that there are no unexpected modifications.
 */
pub unsafe extern "C" fn test_membarrier_manager_thread(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut test_membarrier_thread_args;
    let mut list_a: percpu_list = zeroed();
    let mut list_b: percpu_list = zeroed();
    let mut expect_a: intptr_t = 0;
    let mut expect_b: intptr_t = 0;
    let mut cpu_a: c_int = 0;
    let mut cpu_b: c_int = 0;

    if __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        fprintf(stderr, c"Error: rseq_register_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno, strerror(errno));
        abort();
    }

    /* Init lists. */
    test_membarrier_init_percpu_list(&mut list_a);
    test_membarrier_init_percpu_list(&mut list_b);

    ptr::write_volatile(&mut (*args).percpu_list_ptr, &mut list_a as *mut _ as intptr_t);

    while ptr::read_volatile(&(*args).stop) == 0 {
        /* list_a is "active". */
        cpu_a = rand() % CPU_SETSIZE as c_int;
        /*
         * As list_b is "inactive", we should never see changes
         * to list_b.
         */
        if expect_b != ptr::read_volatile(&(*list_b.c[cpu_b as usize].head).data) {
            fprintf(stderr, c"Membarrier test failed\n".as_ptr());
            abort();
        }

        /* Make list_b "active". */
        ptr::write_volatile(&mut (*args).percpu_list_ptr, &mut list_b as *mut _ as intptr_t);
        if rseq_membarrier_expedited(cpu_a) != 0 &&
                errno != ENXIO /* missing CPU */ {
            perror(c"sys_membarrier".as_ptr());
            abort();
        }
        /*
         * Cpu A should now only modify list_b, so the values
         * in list_a should be stable.
         */
        expect_a = ptr::read_volatile(&(*list_a.c[cpu_a as usize].head).data);

        cpu_b = rand() % CPU_SETSIZE as c_int;
        /*
         * As list_a is "inactive", we should never see changes
         * to list_a.
         */
        if expect_a != ptr::read_volatile(&(*list_a.c[cpu_a as usize].head).data) {
            fprintf(stderr, c"Membarrier test failed\n".as_ptr());
            abort();
        }

        /* Make list_a "active". */
        ptr::write_volatile(&mut (*args).percpu_list_ptr, &mut list_a as *mut _ as intptr_t);
        if rseq_membarrier_expedited(cpu_b) != 0 &&
                errno != ENXIO /* missing CPU*/ {
            perror(c"sys_membarrier".as_ptr());
            abort();
        }
        /* Remember a value from list_b. */
        expect_b = ptr::read_volatile(&(*list_b.c[cpu_b as usize].head).data);
    }

    test_membarrier_free_percpu_list(&mut list_a);
    test_membarrier_free_percpu_list(&mut list_b);

    if rseq_unregister_current_thread() != 0 {
        fprintf(stderr, c"Error: rseq_unregister_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno, strerror(errno));
        abort();
    }
    ptr::null_mut()
}

unsafe extern "C" {
    fn rand() -> c_int;
}

pub unsafe fn test_membarrier() {
    let num_threads = opt_threads as usize;
    let mut thread_args: test_membarrier_thread_args = zeroed();
    let mut worker_threads = vec![0 as pthread_t; num_threads];
    let mut manager_thread: pthread_t = 0;
    let mut i: usize;
    let mut ret: c_int;

    if sys_membarrier(MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ, 0, 0) != 0 {
        perror(c"sys_membarrier".as_ptr());
        abort();
    }

    thread_args.stop = 0;
    thread_args.percpu_list_ptr = 0;
    ret = pthread_create(&mut manager_thread, ptr::null(),
                         test_membarrier_manager_thread, &mut thread_args as *mut _ as *mut c_void);
    if ret != 0 {
        errno = ret;
        perror(c"pthread_create".as_ptr());
        abort();
    }

    i = 0;
    while i < num_threads {
        ret = pthread_create(&mut worker_threads[i], ptr::null(),
                             test_membarrier_worker_thread, &mut thread_args as *mut _ as *mut c_void);
        if ret != 0 {
            errno = ret;
            perror(c"pthread_create".as_ptr());
            abort();
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        ret = pthread_join(worker_threads[i], ptr::null_mut());
        if ret != 0 {
            errno = ret;
            perror(c"pthread_join".as_ptr());
            abort();
        }
        i += 1;
    }

    ptr::write_volatile(&mut thread_args.stop, 1);
    ret = pthread_join(manager_thread, ptr::null_mut());
    if ret != 0 {
        errno = ret;
        perror(c"pthread_join".as_ptr());
        abort();
    }
}

unsafe fn show_usage(argc: c_int, argv: *mut *mut c_char) {
    printf(c"Usage : %s <OPTIONS>\n".as_ptr(), *argv.offset(0));
    printf(c"OPTIONS:\n".as_ptr());
    printf(c"\t[-1 loops] Number of loops for delay injection 1\n".as_ptr());
    printf(c"\t[-2 loops] Number of loops for delay injection 2\n".as_ptr());
    printf(c"\t[-3 loops] Number of loops for delay injection 3\n".as_ptr());
    printf(c"\t[-4 loops] Number of loops for delay injection 4\n".as_ptr());
    printf(c"\t[-5 loops] Number of loops for delay injection 5\n".as_ptr());
    printf(c"\t[-6 loops] Number of loops for delay injection 6\n".as_ptr());
    printf(c"\t[-7 loops] Number of loops for delay injection 7 (-1 to enable -m)\n".as_ptr());
    printf(c"\t[-8 loops] Number of loops for delay injection 8 (-1 to enable -m)\n".as_ptr());
    printf(c"\t[-9 loops] Number of loops for delay injection 9 (-1 to enable -m)\n".as_ptr());
    printf(c"\t[-m N] Yield/sleep/kill every modulo N (default 0: disabled) (>= 0)\n".as_ptr());
    printf(c"\t[-y] Yield\n".as_ptr());
    printf(c"\t[-k] Kill thread with signal\n".as_ptr());
    printf(c"\t[-s S] S: =0: disabled (default), >0: sleep time (ms)\n".as_ptr());
    printf(c"\t[-t N] Number of threads (default 200)\n".as_ptr());
    printf(c"\t[-r N] Number of repetitions per thread (default 5000)\n".as_ptr());
    printf(c"\t[-d] Disable rseq system call (no initialization)\n".as_ptr());
    printf(c"\t[-D M] Disable rseq for each M threads\n".as_ptr());
    printf(c"\t[-T test] Choose test: (s)pinlock, (l)ist, (b)uffer, (m)emcpy, (i)ncrement, membarrie(r)\n".as_ptr());
    printf(c"\t[-M] Push into buffer and memcpy buffer with memory barriers.\n".as_ptr());
    printf(c"\t[-O] Test with optimized RSEQ\n".as_ptr());
    printf(c"\t[-v] Verbose output.\n".as_ptr());
    printf(c"\t[-h] Show this help.\n".as_ptr());
    printf(c"\n".as_ptr());
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;

    i = 1;
    while i < argc {
        if **argv.offset(i as isize) != b'-' as c_char {
            i += 1;
            continue;
        }
        match *(*argv.offset(i as isize)).offset(1) as u8 {
            b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                let idx = (*(*argv.offset(i as isize)).offset(1) - b'0' as c_char) as usize;
                loop_cnt[idx] = atol(*argv.offset((i + 1) as isize)) as c_int;
                i += 1;
            }
            b'm' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_modulo = atol(*argv.offset((i + 1) as isize)) as c_int;
                if opt_modulo < 0 {
                    show_usage(argc, argv);
                    return -1;
                }
                i += 1;
            }
            b's' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_sleep = atol(*argv.offset((i + 1) as isize)) as c_int;
                if opt_sleep < 0 {
                    show_usage(argc, argv);
                    return -1;
                }
                i += 1;
            }
            b'y' => opt_yield = 1,
            b'k' => opt_signal = 1,
            b'd' => opt_disable_rseq = 1,
            b'D' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_disable_mod = atol(*argv.offset((i + 1) as isize)) as c_int;
                if opt_disable_mod < 0 {
                    show_usage(argc, argv);
                    return -1;
                }
                i += 1;
            }
            b't' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_threads = atol(*argv.offset((i + 1) as isize)) as c_int;
                if opt_threads < 0 {
                    show_usage(argc, argv);
                    return -1;
                }
                i += 1;
            }
            b'r' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_reps = atoll(*argv.offset((i + 1) as isize));
                if opt_reps < 0 {
                    show_usage(argc, argv);
                    return -1;
                }
                i += 1;
            }
            b'h' => {
                show_usage(argc, argv);
                return 0;
            }
            b'T' => {
                if argc < i + 2 {
                    show_usage(argc, argv);
                    return -1;
                }
                opt_test = **argv.offset((i + 1) as isize) as c_int;
                match opt_test as u8 {
                    b's' | b'l' | b'i' | b'b' | b'm' | b'r' => {}
                    _ => {
                        show_usage(argc, argv);
                        return -1;
                    }
                }
                i += 1;
            }
            b'v' => verbose = 1,
            b'M' => opt_mo = RSEQ_MO_RELEASE,
            b'L' => opt_rseq_legacy = true,
            _ => {
                show_usage(argc, argv);
                return -1;
            }
        }
        i += 1;
    }

    loop_cnt_1 = loop_cnt[1];
    loop_cnt_2 = loop_cnt[2];
    loop_cnt_3 = loop_cnt[3];
    loop_cnt_4 = loop_cnt[4];
    loop_cnt_5 = loop_cnt[5];
    loop_cnt_6 = loop_cnt[6];

    if set_signal_handler() != 0 {
        return -1;
    }

    if opt_disable_rseq == 0 && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy) != 0 {
        return -1;
    }
    if opt_disable_rseq == 0 && !rseq_validate_cpu_id() {
        fprintf(stderr, c"Error: cpu id getter unavailable\n".as_ptr());
        return -1;
    }
    match opt_test as u8 {
        b's' => {
            printf_verbose0(c"spinlock\n".as_ptr());
            test_percpu_spinlock();
        }
        b'l' => {
            printf_verbose0(c"linked list\n".as_ptr());
            test_percpu_list();
        }
        b'b' => {
            printf_verbose0(c"buffer\n".as_ptr());
            test_percpu_buffer();
        }
        b'm' => {
            printf_verbose0(c"memcpy buffer\n".as_ptr());
            test_percpu_memcpy_buffer();
        }
        b'i' => {
            printf_verbose0(c"counter increment\n".as_ptr());
            test_percpu_inc();
        }
        b'r' => {
            printf_verbose0(c"membarrier\n".as_ptr());
            test_membarrier();
        }
        _ => {}
    }
    if opt_disable_rseq == 0 && rseq_unregister_current_thread() != 0 {
        abort();
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: LGPL-2.1
// Translated from C source. C includes depended on: assert.h, pthread.h,
// sched.h, stdint.h, stdio.h, stdlib.h, string.h, stddef.h, kselftest.h,
// rseq.h.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type intptr_t = isize;
type uint64_t = u64;
type pthread_t = libc::pthread_t;
type cpu_set_t = libc::cpu_set_t;

const CPU_SETSIZE: usize = libc::CPU_SETSIZE as usize;
const RSEQ_MO_RELAXED: c_int = 0;

// #ifdef BUILDOPT_RSEQ_PERCPU_MM_CID
#[cfg(BUILDOPT_RSEQ_PERCPU_MM_CID)]
unsafe fn RSEQ_PERCPU() -> c_int {
    unsafe { RSEQ_PERCPU_MM_CID }
}
#[cfg(BUILDOPT_RSEQ_PERCPU_MM_CID)]
unsafe fn get_current_cpu_id() -> c_int {
    unsafe { rseq_current_mm_cid() }
}
#[cfg(BUILDOPT_RSEQ_PERCPU_MM_CID)]
unsafe fn rseq_validate_cpu_id() -> bool {
    unsafe { rseq_mm_cid_available() }
}
#[cfg(BUILDOPT_RSEQ_PERCPU_MM_CID)]
fn rseq_use_cpu_index() -> bool {
    false /* Use mm_cid */
}

// #else
#[cfg(not(BUILDOPT_RSEQ_PERCPU_MM_CID))]
unsafe fn RSEQ_PERCPU() -> c_int {
    unsafe { RSEQ_PERCPU_CPU_ID }
}
#[cfg(not(BUILDOPT_RSEQ_PERCPU_MM_CID))]
unsafe fn get_current_cpu_id() -> c_int {
    unsafe { rseq_cpu_start() }
}
#[cfg(not(BUILDOPT_RSEQ_PERCPU_MM_CID))]
unsafe fn rseq_validate_cpu_id() -> bool {
    unsafe { rseq_current_cpu_raw() >= 0 }
}
#[cfg(not(BUILDOPT_RSEQ_PERCPU_MM_CID))]
fn rseq_use_cpu_index() -> bool {
    true /* Use cpu_id as index. */
}
// #endif

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
    reps: c_int,
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

unsafe extern "C" {
    static mut stderr: *mut libc::FILE;
    static RSEQ_PERCPU_MM_CID: c_int;
    static RSEQ_PERCPU_CPU_ID: c_int;

    fn rseq_current_mm_cid() -> c_int;
    fn rseq_mm_cid_available() -> bool;
    fn rseq_cpu_start() -> c_int;
    fn rseq_current_cpu_raw() -> c_int;
    fn rseq_register_current_thread() -> c_int;
    fn rseq_unregister_current_thread() -> c_int;
    fn rseq_cmpeqv_storev(
        mo: c_int,
        percpu: c_int,
        v: *mut intptr_t,
        expect: intptr_t,
        newv: intptr_t,
        cpu: c_int,
    ) -> c_int;
    fn rseq_cmpnev_storeoffp_load(
        mo: c_int,
        percpu: c_int,
        v: *mut intptr_t,
        expectnot: intptr_t,
        voffp: c_long,
        load: *mut intptr_t,
        cpu: c_int,
    ) -> c_int;
    fn rseq_smp_acquire__after_ctrl_dep();
    fn rseq_smp_store_release(p: *mut intptr_t, v: intptr_t);
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn abort() -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sched_yield() -> c_int;
    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn rseq_likely(x: bool) -> bool {
    x
}

unsafe fn RSEQ_READ_ONCE<T: Copy>(p: *mut T) -> T {
    unsafe { ptr::read_volatile(p) }
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool {
    unsafe { libc::CPU_ISSET(cpu as usize, set) }
}

/* A simple percpu spinlock.  Returns the cpu lock was acquired on. */
unsafe extern "C" fn rseq_this_cpu_lock(lock: *mut percpu_lock) -> c_int {
    let mut cpu: c_int;

    loop {
        let ret: c_int;

        cpu = unsafe { get_current_cpu_id() };
        ret = unsafe {
            rseq_cmpeqv_storev(
                RSEQ_MO_RELAXED,
                RSEQ_PERCPU(),
                &mut (*lock).c[cpu as usize].v,
                0,
                1,
                cpu,
            )
        };
        if unsafe { rseq_likely(ret == 0) } {
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    /*
     * Acquire semantic when taking lock after control dependency.
     * Matches rseq_smp_store_release().
     */
    unsafe { rseq_smp_acquire__after_ctrl_dep() };
    cpu
}

unsafe extern "C" fn rseq_percpu_unlock(lock: *mut percpu_lock, cpu: c_int) {
    assert!(unsafe { (*lock).c[cpu as usize].v == 1 });
    /*
     * Release lock, with release semantic. Matches
     * rseq_smp_acquire__after_ctrl_dep().
     */
    unsafe { rseq_smp_store_release(&mut (*lock).c[cpu as usize].v, 0) };
}

unsafe extern "C" fn test_percpu_spinlock_thread(arg: *mut c_void) -> *mut c_void {
    let data: *mut spinlock_test_data = arg as *mut spinlock_test_data;
    let mut i: c_int;
    let mut cpu: c_int;

    if unsafe { rseq_register_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_register_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
            abort();
        }
    }
    i = 0;
    while i < unsafe { (*data).reps } {
        cpu = unsafe { rseq_this_cpu_lock(&mut (*data).lock) };
        unsafe {
            (*data).c[cpu as usize].count += 1;
            rseq_percpu_unlock(&mut (*data).lock, cpu);
        }
        i += 1;
    }
    if unsafe { rseq_unregister_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_unregister_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
            abort();
        }
    }

    ptr::null_mut()
}

/*
 * A simple test which implements a sharded counter using a per-cpu
 * lock.  Obviously real applications might prefer to simply use a
 * per-cpu increment; however, this is reasonable for a test and the
 * lock can be extended to synchronize more complicated operations.
 */
unsafe extern "C" fn test_percpu_spinlock() {
    const num_threads: c_int = 200;
    let mut i: c_int;
    let mut sum: uint64_t;
    let mut test_threads: [pthread_t; num_threads as usize] = unsafe { core::mem::zeroed() };
    let mut data: spinlock_test_data = unsafe { core::mem::zeroed() };

    unsafe {
        memset(
            &mut data as *mut spinlock_test_data as *mut c_void,
            0,
            size_of::<spinlock_test_data>(),
        );
    }
    data.reps = 5000;

    i = 0;
    while i < num_threads {
        unsafe {
            pthread_create(
                &mut test_threads[i as usize],
                ptr::null(),
                test_percpu_spinlock_thread,
                &mut data as *mut spinlock_test_data as *mut c_void,
            );
        }
        i += 1;
    }

    i = 0;
    while i < num_threads {
        unsafe {
            pthread_join(test_threads[i as usize], ptr::null_mut());
        }
        i += 1;
    }

    sum = 0;
    i = 0;
    while (i as usize) < CPU_SETSIZE {
        sum += data.c[i as usize].count as uint64_t;
        i += 1;
    }

    assert!(sum == data.reps as uint64_t * num_threads as uint64_t);
}

unsafe extern "C" fn this_cpu_list_push(
    list: *mut percpu_list,
    node: *mut percpu_list_node,
    _cpu: *mut c_int,
) {
    let mut cpu: c_int;

    loop {
        let targetptr: *mut intptr_t;
        let newval: intptr_t;
        let expect: intptr_t;
        let ret: c_int;

        cpu = unsafe { get_current_cpu_id() };
        /* Load list->c[cpu].head with single-copy atomicity. */
        expect = unsafe { RSEQ_READ_ONCE(&mut (*list).c[cpu as usize].head) } as intptr_t;
        newval = node as intptr_t;
        targetptr = unsafe { &mut (*list).c[cpu as usize].head as *mut *mut percpu_list_node as *mut intptr_t };
        unsafe {
            (*node).next = expect as *mut percpu_list_node;
        }
        ret = unsafe {
            rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU(), targetptr, expect, newval, cpu)
        };
        if unsafe { rseq_likely(ret == 0) } {
            break;
        }
        /* Retry if comparison fails or rseq aborts. */
    }
    if !_cpu.is_null() {
        unsafe {
            *_cpu = cpu;
        }
    }
}

/*
 * Unlike a traditional lock-less linked list; the availability of a
 * rseq primitive allows us to implement pop without concerns over
 * ABA-type races.
 */
unsafe extern "C" fn this_cpu_list_pop(
    list: *mut percpu_list,
    _cpu: *mut c_int,
) -> *mut percpu_list_node {
    loop {
        let mut head: *mut percpu_list_node = ptr::null_mut();
        let targetptr: *mut intptr_t;
        let expectnot: intptr_t;
        let load: *mut intptr_t;
        let offset: c_long;
        let ret: c_int;
        let cpu: c_int;

        cpu = unsafe { get_current_cpu_id() };
        targetptr = unsafe { &mut (*list).c[cpu as usize].head as *mut *mut percpu_list_node as *mut intptr_t };
        expectnot = ptr::null_mut::<c_void>() as intptr_t;
        offset = offset_of!(percpu_list_node, next) as c_long;
        load = &mut head as *mut *mut percpu_list_node as *mut intptr_t;
        ret = unsafe {
            rseq_cmpnev_storeoffp_load(
                RSEQ_MO_RELAXED,
                RSEQ_PERCPU(),
                targetptr,
                expectnot,
                offset,
                load,
                cpu,
            )
        };
        if unsafe { rseq_likely(ret == 0) } {
            if !_cpu.is_null() {
                unsafe {
                    *_cpu = cpu;
                }
            }
            return head;
        }
        if ret > 0 {
            return ptr::null_mut();
        }
        /* Retry if rseq aborts. */
    }
}

/*
 * __percpu_list_pop is not safe against concurrent accesses. Should
 * only be used on lists that are not concurrently modified.
 */
unsafe extern "C" fn __percpu_list_pop(list: *mut percpu_list, cpu: c_int) -> *mut percpu_list_node {
    let node: *mut percpu_list_node;

    node = unsafe { (*list).c[cpu as usize].head };
    if node.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*list).c[cpu as usize].head = (*node).next;
    }
    node
}

unsafe extern "C" fn test_percpu_list_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let list: *mut percpu_list = arg as *mut percpu_list;

    if unsafe { rseq_register_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_register_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
            abort();
        }
    }

    i = 0;
    while i < 100000 {
        let node: *mut percpu_list_node;

        node = unsafe { this_cpu_list_pop(list, ptr::null_mut()) };
        unsafe {
            sched_yield();
        } /* encourage shuffling */
        if !node.is_null() {
            unsafe {
                this_cpu_list_push(list, node, ptr::null_mut());
            }
        }
        i += 1;
    }

    if unsafe { rseq_unregister_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_unregister_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
            abort();
        }
    }

    ptr::null_mut()
}

/* Simultaneous modification to a per-cpu linked list from many threads.  */
unsafe extern "C" fn test_percpu_list() {
    let mut i: c_int;
    let mut j: c_int;
    let mut sum: uint64_t = 0;
    let mut expected_sum: uint64_t = 0;
    let mut list: percpu_list = unsafe { core::mem::zeroed() };
    let mut test_threads: [pthread_t; 200] = unsafe { core::mem::zeroed() };
    let mut allowed_cpus: cpu_set_t = unsafe { core::mem::zeroed() };

    unsafe {
        memset(
            &mut list as *mut percpu_list as *mut c_void,
            0,
            size_of::<percpu_list>(),
        );
    }

    /* Generate list entries for every usable cpu. */
    unsafe {
        sched_getaffinity(
            0,
            size_of::<cpu_set_t>(),
            &mut allowed_cpus as *mut cpu_set_t,
        );
    }
    i = 0;
    while (i as usize) < CPU_SETSIZE {
        if rseq_use_cpu_index()
            && !unsafe { CPU_ISSET(i, &allowed_cpus as *const cpu_set_t) }
        {
            i += 1;
            continue;
        }
        j = 1;
        while j <= 100 {
            let node: *mut percpu_list_node;

            expected_sum += j as uint64_t;

            node = unsafe { malloc(size_of::<percpu_list_node>()) as *mut percpu_list_node };
            assert!(!node.is_null());
            unsafe {
                (*node).data = j as intptr_t;
                (*node).next = list.c[i as usize].head;
                list.c[i as usize].head = node;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < 200 {
        unsafe {
            pthread_create(
                &mut test_threads[i as usize],
                ptr::null(),
                test_percpu_list_thread,
                &mut list as *mut percpu_list as *mut c_void,
            );
        }
        i += 1;
    }

    i = 0;
    while i < 200 {
        unsafe {
            pthread_join(test_threads[i as usize], ptr::null_mut());
        }
        i += 1;
    }

    i = 0;
    while (i as usize) < CPU_SETSIZE {
        let mut node: *mut percpu_list_node;

        if rseq_use_cpu_index()
            && !unsafe { CPU_ISSET(i, &allowed_cpus as *const cpu_set_t) }
        {
            i += 1;
            continue;
        }

        loop {
            node = unsafe { __percpu_list_pop(&mut list, i) };
            if node.is_null() {
                break;
            }
            unsafe {
                sum += (*node).data as uint64_t;
                free(node as *mut c_void);
            }
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

unsafe fn main_0(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if unsafe { rseq_register_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_register_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
        }
        return -1;
    }
    if !unsafe { rseq_validate_cpu_id() } {
        unsafe {
            fprintf(stderr, c"Error: cpu id getter unavailable\n".as_ptr());
        }
        return -1;
    }
    unsafe {
        printf(c"spinlock\n".as_ptr());
    }
    unsafe {
        test_percpu_spinlock();
    }
    unsafe {
        printf(c"percpu_list\n".as_ptr());
    }
    unsafe {
        test_percpu_list();
    }
    if unsafe { rseq_unregister_current_thread() } != 0 {
        unsafe {
            fprintf(
                stderr,
                c"Error: rseq_unregister_current_thread(...) failed(%d): %s\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
        }
        return -1;
    }
    0
}

fn main() {
    let code = unsafe { main_0(0, ptr::null_mut()) };
    if code != 0 {
        std::process::exit(code);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

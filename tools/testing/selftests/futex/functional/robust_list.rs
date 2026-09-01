// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2025 Igalia S.L.
 *
 * Robust list test by Andre Almeida <andrealmeid@igalia.com>
 *
 * The robust list uAPI allows userspace to create "robust" locks, in the sense
 * that if the lock holder thread dies, the remaining threads that are waiting
 * for the lock won't block forever, waiting for a lock that will never be
 * released.
 *
 * This is achieve by userspace setting a list where a thread can enter all the
 * locks (futexes) that it is holding. The robust list is a linked list, and
 * userspace register the start of the list with the syscall set_robust_list().
 * If such thread eventually dies, the kernel will walk this list, waking up one
 * thread waiting for each futex and marking the futex word with the flag
 * FUTEX_OWNER_DIED.
 *
 * See also
 *	man set_robust_list
 *	Documententation/locking/robust-futex-ABI.rst
 *	Documententation/locking/robust-futexes.rst
 */

// C dependencies translated from:
// "futextest.h", "kselftest_harness.h", dlfcn.h, errno.h, pthread.h,
// signal.h, stdatomic.h, stdbool.h, stddef.h, stdint.h, stdlib.h, string.h,
// sys/auxv.h, sys/mman.h, sys/wait.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

const STACK_SIZE: usize = 1024 * 1024;
const FUTEX_TIMEOUT: i64 = 3;
const SLEEP_US: c_uint = 100;

// Original C condition:
// #if __SIZEOF_LONG__ == 8
#[cfg(target_pointer_width = "64")]
const BUILD_64: bool = true;
#[cfg(not(target_pointer_width = "64"))]
const BUILD_64: bool = false;

const CHILD_NR: usize = 10;

type size_t = usize;
type pid_t = c_int;
type atomic_futex_t = AtomicU32;
type futex_t = u32;

#[repr(C)]
pub struct robust_list {
    pub next: *mut robust_list,
}

#[repr(C)]
pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: isize,
    pub list_op_pending: *mut robust_list,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [usize; 4],
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/*
 * Basic lock struct, contains just the futex word and the robust list element
 * Real implementations have also a *prev to easily walk in the list
 */
#[repr(C)]
struct lock_struct {
    futex: atomic_futex_t,
    list: robust_list,
}

#[repr(C)]
struct child_args {
    _metadata: *mut __test_metadata,
    arg: *mut c_void,
}

#[repr(C)]
struct vdso_unlock {
    vdso: Option<unsafe extern "C" fn(*mut AtomicU32, u32, *mut c_void) -> u32>,
}

#[repr(C)]
struct vdso_unlock_variant {
    is_32: bool,
    func_name: *const c_char,
}

#[repr(C)]
struct futex_op {}

#[repr(C)]
struct futex_op_variant {
    op: c_uint,
    val3: c_uint,
}

static mut barrier: pthread_barrier_t = pthread_barrier_t { _private: [0; 4] };
static mut barrier2: pthread_barrier_t = pthread_barrier_t { _private: [0; 4] };

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn gettid() -> pid_t;
    fn futex_wait(uaddr: *mut futex_t, val: u32, timeout: *const timespec, private: c_int) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;

    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
}

type c_long = isize;

extern "C" {
    static SYS_set_robust_list: c_long;
    static SYS_get_robust_list: c_long;
    static SYS_futex: c_long;
    static PROT_READ: c_int;
    static PROT_WRITE: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_ANONYMOUS: c_int;
    static MAP_STACK: c_int;
    static MAP_FAILED: *mut c_void;
    static CLONE_VM: c_int;
    static SIGCHLD: c_int;
    static FUTEX_WAITERS: u32;
    static FUTEX_OWNER_DIED: u32;
    static FUTEX_WAKE: c_uint;
    static FUTEX_WAKE_BITSET: c_uint;
    static FUTEX_UNLOCK_PI: c_uint;
    static FUTEX_ROBUST_LIST32: c_uint;
    static FUTEX_BITSET_MATCH_ANY: c_uint;
    static FUTEX_ROBUST_UNLOCK: c_uint;
    static EINVAL: c_int;
    static RTLD_LAZY: c_int;
    static RTLD_LOCAL: c_int;
    static RTLD_NOLOAD: c_int;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {};
}

fn WEXITSTATUS(wstatus: c_int) -> c_int {
    (wstatus >> 8) & 0xff
}

unsafe fn set_robust_list(head: *mut robust_list_head, len: size_t) -> c_int {
    syscall(SYS_set_robust_list, head, len) as c_int
}

unsafe fn get_robust_list(
    pid: c_int,
    head: *mut *mut robust_list_head,
    len_ptr: *mut size_t,
) -> c_int {
    syscall(SYS_get_robust_list, pid, head, len_ptr) as c_int
}

unsafe fn sys_futex_robust_unlock(
    uaddr: *mut AtomicU32,
    op: c_uint,
    val: c_int,
    list_op_pending: *mut c_void,
    val3: c_uint,
) -> c_int {
    syscall(SYS_futex, uaddr, op, val, ptr::null::<c_void>(), list_op_pending, val3, 0) as c_int
}

/*
 * Helper function to spawn a child thread. Returns -1 on error, pid on success
 */
unsafe fn create_child(
    _metadata: *mut __test_metadata,
    fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
    arg: *mut c_void,
) -> c_int {
    let cargs = malloc(size_of::<child_args>()) as *mut child_args;
    let mut stack: *mut c_char;
    let pid: pid_t;

    if cargs.is_null() {
        return -1;
    }
    (*cargs)._metadata = _metadata;
    (*cargs).arg = arg;

    stack = mmap(
        ptr::null_mut(),
        STACK_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
        -1,
        0,
    ) as *mut c_char;
    if stack == MAP_FAILED as *mut c_char {
        free(cargs as *mut c_void);
        return -1;
    }

    stack = stack.add(STACK_SIZE);

    pid = clone(fn_, stack as *mut c_void, CLONE_VM | SIGCHLD, cargs as *mut c_void);
    if pid == -1 {
        free(cargs as *mut c_void);
        return -1;
    }

    pid
}

/*
 * Helper function to prepare and register a robust list
 */
unsafe fn set_list(head: *mut robust_list_head) -> c_int {
    let ret: c_int;

    ret = set_robust_list(head, size_of::<robust_list_head>());
    if ret != 0 {
        return ret;
    }

    (*head).futex_offset = offset_of!(lock_struct, futex) as isize - offset_of!(lock_struct, list) as isize;
    (*head).list.next = &mut (*head).list;
    (*head).list_op_pending = ptr::null_mut();

    0
}

/*
 * A basic (and incomplete) mutex lock function with robustness
 */
unsafe fn mutex_lock(lock: *mut lock_struct, head: *mut robust_list_head, error_inject: bool) -> c_int {
    let futex: *mut atomic_futex_t = &mut (*lock).futex;
    let mut zero: u32 = 0;
    let mut tid: pid_t = gettid();
    let mut ret: c_int = -1;

    /*
     * Set list_op_pending before starting the lock, so the kernel can catch
     * the case where the thread died during the lock operation
     */
    (*head).list_op_pending = &mut (*lock).list;

    if (*futex)
        .compare_exchange(zero, tid as u32, Ordering::SeqCst, Ordering::SeqCst)
        .map(|_| true)
        .unwrap_or_else(|actual| {
            zero = actual;
            false
        })
    {
        /*
         * We took the lock, insert it in the robust list
         */
        let mut list: *mut robust_list = &mut (*head).list;

        /* Error injection to test list_op_pending */
        if error_inject {
            return 0;
        }

        while (*list).next != &mut (*head).list {
            list = (*list).next;
        }

        (*list).next = &mut (*lock).list;
        (*lock).list.next = &mut (*head).list;

        ret = 0;
    } else {
        /*
         * We didn't take the lock, wait until the owner wakes (or dies)
         */
        let mut to: timespec;

        to = timespec {
            tv_sec: FUTEX_TIMEOUT,
            tv_nsec: 0,
        };

        tid = (*futex).load(Ordering::SeqCst) as pid_t;
        /* Kernel ignores futexes without the waiters flag */
        tid |= FUTEX_WAITERS as pid_t;
        (*futex).store(tid as u32, Ordering::SeqCst);

        ret = futex_wait(futex as *mut futex_t, tid as u32, &to, 0);

        /*
         * A real mutex_lock() implementation would loop here to finally
         * take the lock. We don't care about that, so we stop here.
         */
    }

    (*head).list_op_pending = ptr::null_mut();

    ret
}

/*
 * This child thread will succeed taking the lock, and then will exit holding it
 */
unsafe extern "C" fn child_fn_lock(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let _metadata = (*cargs)._metadata;
    let lock = (*cargs).arg as *mut lock_struct;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;

    free(cargs as *mut c_void);

    ret = set_list(&mut head);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("set_robust_list error");

    ret = mutex_lock(lock, &mut head, false);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("mutex_lock error");

    pthread_barrier_wait(&raw mut barrier);

    /*
     * There's a race here: the parent thread needs to be inside
     * futex_wait() before the child thread dies, otherwise it will miss the
     * wakeup from handle_futex_death() that this child will emit. We wait a
     * little bit just to make sure that this happens.
     */
    usleep(SLEEP_US);

    0
}

/*
 * Spawns a child thread that will set a robust list, take the lock, register it
 * in the robust list and die. The parent thread will wait on this futex, and
 * should be waken up when the child exits.
 */
unsafe fn test_robustness(_metadata: *mut __test_metadata) {
    let mut lock = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let futex: *mut atomic_futex_t = &mut lock.futex;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;
    let mut pid: c_int;
    let mut wstatus: c_int = 0;

    ret = set_list(&mut head);
    ASSERT_EQ!(ret, 0);

    /*
     * Lets use a barrier to ensure that the child thread takes the lock
     * before the parent
     */
    ret = pthread_barrier_init(&raw mut barrier, ptr::null(), 2);
    ASSERT_EQ!(ret, 0);

    pid = create_child(_metadata, child_fn_lock, &mut lock as *mut _ as *mut c_void);
    ASSERT_NE!(pid, -1);

    pthread_barrier_wait(&raw mut barrier);
    ret = mutex_lock(&mut lock, &mut head, false);

    /*
     * futex_wait() should return 0 and the futex word should be marked with
     * FUTEX_OWNER_DIED
     */
    ASSERT_EQ!(ret, 0);

    ASSERT_TRUE!((*futex).load(Ordering::SeqCst) & FUTEX_OWNER_DIED != 0);

    wait(&mut wstatus);
    pthread_barrier_destroy(&raw mut barrier);

    EXPECT_EQ!(WEXITSTATUS(wstatus), 0);
    TH_LOG!("child failed");
}

/*
 * The only valid value for len is sizeof(*head)
 */
unsafe fn test_set_robust_list_invalid_size() {
    let mut head: robust_list_head = core::mem::zeroed();
    let head_size: size_t = size_of_val(&head);
    let mut ret: c_int;

    ret = set_robust_list(&mut head, head_size);
    ASSERT_EQ!(ret, 0);

    ret = set_robust_list(&mut head, head_size * 2);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno, EINVAL);

    ret = set_robust_list(&mut head, head_size - 1);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno, EINVAL);

    ret = set_robust_list(&mut head, 0);
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno, EINVAL);
}

/*
 * Test get_robust_list with pid = 0, getting the list of the running thread
 */
unsafe fn test_get_robust_list_self() {
    let mut head: robust_list_head = core::mem::zeroed();
    let mut head2: robust_list_head = core::mem::zeroed();
    let mut get_head: *mut robust_list_head = ptr::null_mut();
    let head_size: size_t = size_of_val(&head);
    let mut len_ptr: size_t = 0;
    let mut ret: c_int;

    ret = set_robust_list(&mut head, head_size);
    ASSERT_EQ!(ret, 0);

    ret = get_robust_list(0, &mut get_head, &mut len_ptr);
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(get_head, &mut head);
    ASSERT_EQ!(head_size, len_ptr);

    ret = set_robust_list(&mut head2, head_size);
    ASSERT_EQ!(ret, 0);

    ret = get_robust_list(0, &mut get_head, &mut len_ptr);
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(get_head, &mut head2);
    ASSERT_EQ!(head_size, len_ptr);
}

unsafe extern "C" fn child_list(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let _metadata = (*cargs)._metadata;
    let head = (*cargs).arg as *mut robust_list_head;
    let mut ret: c_int;

    free(cargs as *mut c_void);

    ret = set_robust_list(head, size_of::<robust_list_head>());
    ASSERT_EQ!(ret, 0);
    TH_LOG!("set_robust_list error");

    /*
     * After setting the list head, wait until the main thread can call
     * get_robust_list() for this thread before exiting.
     */
    pthread_barrier_wait(&raw mut barrier);
    pthread_barrier_wait(&raw mut barrier2);

    0
}

/*
 * Test get_robust_list from another thread. We use two barriers here to ensure
 * that:
 *   1) the child thread set the list before we try to get it from the
 * parent
 *   2) the child thread still alive when we try to get the list from it
 */
unsafe fn test_get_robust_list_child(_metadata: *mut __test_metadata) {
    let mut head: robust_list_head = core::mem::zeroed();
    let mut get_head: *mut robust_list_head = ptr::null_mut();
    let mut ret: c_int;
    let mut wstatus: c_int = 0;
    let mut len_ptr: size_t = 0;
    let tid: pid_t;

    ret = pthread_barrier_init(&raw mut barrier, ptr::null(), 2);
    ret = pthread_barrier_init(&raw mut barrier2, ptr::null(), 2);
    ASSERT_EQ!(ret, 0);

    tid = create_child(_metadata, child_list, &mut head as *mut _ as *mut c_void);
    ASSERT_NE!(tid, -1);

    pthread_barrier_wait(&raw mut barrier);

    ret = get_robust_list(tid, &mut get_head, &mut len_ptr);
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(&mut head, get_head);

    pthread_barrier_wait(&raw mut barrier2);

    wait(&mut wstatus);
    pthread_barrier_destroy(&raw mut barrier);
    pthread_barrier_destroy(&raw mut barrier2);

    EXPECT_EQ!(WEXITSTATUS(wstatus), 0);
    TH_LOG!("child failed");
}

unsafe extern "C" fn child_fn_lock_with_error(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let _metadata = (*cargs)._metadata;
    let lock = (*cargs).arg as *mut lock_struct;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;

    free(cargs as *mut c_void);

    ret = set_list(&mut head);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("set_robust_list error");

    ret = mutex_lock(lock, &mut head, true);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("mutex_lock error");

    pthread_barrier_wait(&raw mut barrier);

    /* See comment at child_fn_lock() */
    usleep(SLEEP_US);

    0
}

/*
 * Same as robustness test, but inject an error where the mutex_lock() exits
 * earlier, just after setting list_op_pending and taking the lock, to test the
 * list_op_pending mechanism
 */
unsafe fn test_set_list_op_pending(_metadata: *mut __test_metadata) {
    let mut lock = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let futex: *mut atomic_futex_t = &mut lock.futex;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;
    let mut wstatus: c_int = 0;

    ret = set_list(&mut head);
    ASSERT_EQ!(ret, 0);

    ret = pthread_barrier_init(&raw mut barrier, ptr::null(), 2);
    ASSERT_EQ!(ret, 0);

    ret = create_child(_metadata, child_fn_lock_with_error, &mut lock as *mut _ as *mut c_void);
    ASSERT_NE!(ret, -1);

    pthread_barrier_wait(&raw mut barrier);
    ret = mutex_lock(&mut lock, &mut head, false);

    ASSERT_EQ!(ret, 0);

    ASSERT_TRUE!((*futex).load(Ordering::SeqCst) & FUTEX_OWNER_DIED != 0);

    wait(&mut wstatus);
    pthread_barrier_destroy(&raw mut barrier);

    EXPECT_EQ!(WEXITSTATUS(wstatus), 0);
    TH_LOG!("child failed");
}

unsafe extern "C" fn child_lock_holder(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let locks = (*cargs).arg as *mut lock_struct;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut i: c_int;

    free(cargs as *mut c_void);

    set_list(&mut head);

    i = 0;
    while i < CHILD_NR as c_int {
        (*locks.add(i as usize)).futex.store(0, Ordering::SeqCst);
        mutex_lock(locks.add(i as usize), &mut head, false);
        i += 1;
    }

    pthread_barrier_wait(&raw mut barrier);
    pthread_barrier_wait(&raw mut barrier2);

    /* See comment at child_fn_lock() */
    usleep(SLEEP_US);

    0
}

unsafe extern "C" fn child_wait_lock(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let _metadata = (*cargs)._metadata;
    let lock = (*cargs).arg as *mut lock_struct;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;

    free(cargs as *mut c_void);

    pthread_barrier_wait(&raw mut barrier2);
    ret = mutex_lock(lock, &mut head, false);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("mutex_lock error");

    ASSERT_TRUE!((*lock).futex.load(Ordering::SeqCst) & FUTEX_OWNER_DIED != 0);
    TH_LOG!("futex not marked with FUTEX_OWNER_DIED");

    0
}

/*
 * Test a robust list of more than one element. All the waiters should wake when
 * the holder dies
 */
unsafe fn test_robust_list_multiple_elements(_metadata: *mut __test_metadata) {
    let mut locks: [lock_struct; CHILD_NR] = core::array::from_fn(|_| lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    });
    let mut pids: [pid_t; CHILD_NR + 1] = [0; CHILD_NR + 1];
    let mut i: c_int;
    let mut ret: c_int;
    let mut wstatus: c_int = 0;

    ret = pthread_barrier_init(&raw mut barrier, ptr::null(), 2);
    ASSERT_EQ!(ret, 0);
    ret = pthread_barrier_init(&raw mut barrier2, ptr::null(), (CHILD_NR + 1) as c_uint);
    ASSERT_EQ!(ret, 0);

    pids[0] = create_child(_metadata, child_lock_holder, locks.as_mut_ptr() as *mut c_void);
    ASSERT_NE!(pids[0], -1);

    /* Wait until the locker thread takes the look */
    pthread_barrier_wait(&raw mut barrier);

    i = 0;
    while i < CHILD_NR as c_int {
        pids[i as usize + 1] = create_child(
            _metadata,
            child_wait_lock,
            &mut locks[i as usize] as *mut _ as *mut c_void,
        );
        ASSERT_NE!(pids[i as usize + 1], -1);
        i += 1;
    }

    /* Wait for all children to return (holder + all waiters) */
    ret = 0;
    i = 0;
    while i < CHILD_NR as c_int + 1 {
        waitpid(pids[i as usize], &mut wstatus, 0);
        if WEXITSTATUS(wstatus) != 0 {
            ret = -1;
        }
        i += 1;
    }

    pthread_barrier_destroy(&raw mut barrier);
    pthread_barrier_destroy(&raw mut barrier2);

    EXPECT_EQ!(ret, 0);
    TH_LOG!("One or more children failed");
}

unsafe extern "C" fn child_circular_list(arg: *mut c_void) -> c_int {
    let cargs = arg as *mut child_args;
    let _metadata = (*cargs)._metadata;
    static mut A: lock_struct = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    static mut B: lock_struct = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    static mut C: lock_struct = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let mut head: robust_list_head = core::mem::zeroed();
    let mut ret: c_int;

    free(cargs as *mut c_void);

    ret = set_list(&mut head);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("set_list error");

    head.list.next = &raw mut A.list;

    /*
     * The last element should point to head list, but we short circuit it
     */
    A.list.next = &raw mut B.list;
    B.list.next = &raw mut C.list;
    C.list.next = &raw mut A.list;

    0
}

/*
 * Create a circular robust list. The kernel should be able to destroy the list
 * while processing it so it won't be trapped in an infinite loop while handling
 * a process exit
 */
unsafe fn test_circular_list(_metadata: *mut __test_metadata) {
    let mut wstatus: c_int = 0;
    let pid: pid_t;

    pid = create_child(_metadata, child_circular_list, ptr::null_mut());
    ASSERT_NE!(pid, -1);

    wait(&mut wstatus);

    EXPECT_EQ!(WEXITSTATUS(wstatus), 0);
    TH_LOG!("child failed");
}

/*
 * Below are tests for the fix of robust release race condition. Please read the following
 * thread to learn more about the issue in the first place and why the following functions fix it:
 * https://lore.kernel.org/lkml/20260316162316.356674433@kernel.org/
 */

/*
 * Auxiliary code for binding the vDSO functions
 */
unsafe fn get_vdso_func_addr(function: *const c_char) -> *mut c_void {
    let vdso_names: [*const c_char; 4] = [
        c"linux-vdso.so.1".as_ptr(),
        c"linux-gate.so.1".as_ptr(),
        c"linux-vdso32.so.1".as_ptr(),
        c"linux-vdso64.so.1".as_ptr(),
    ];

    let mut i: usize = 0;
    while i < vdso_names.len() {
        let vdso = dlopen(vdso_names[i], RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);

        if !vdso.is_null() {
            return dlsym(vdso, function);
        }
        i += 1;
    }
    ptr::null_mut()
}

/*
 * These are the real vDSO function signatures:
 *
 *	__vdso_futex_robust_list64_try_unlock(__u32 *lock, __u32 tid, __u64 *pop)
 *	__vdso_futex_robust_list32_try_unlock(__u32 *lock, __u32 tid, __u32 *pop)
 *
 * So for the generic entry point we need to use a void pointer as the last argument
 */

unsafe fn vdso_unlock_setup(self_: *mut vdso_unlock, variant: *const vdso_unlock_variant) {
    (*self_).vdso = core::mem::transmute(get_vdso_func_addr((*variant).func_name));
}

unsafe fn vdso_unlock_teardown(_self: *mut vdso_unlock) {}

static VDSO_UNLOCK_32: vdso_unlock_variant = vdso_unlock_variant {
    func_name: c"__vdso_futex_robust_list32_try_unlock".as_ptr(),
    is_32: true,
};

static VDSO_UNLOCK_64: vdso_unlock_variant = vdso_unlock_variant {
    func_name: c"__vdso_futex_robust_list64_try_unlock".as_ptr(),
    is_32: false,
};

/*
 * Test the vDSO robust_listXX_try_unlock() for the uncontended case. The virtual syscall should
 * return the thread ID of the lock owner, the lock word must be 0 and the list_op_pending should
 * be NULL.
 */
unsafe fn test_robust_try_unlock_uncontended(self_: *mut vdso_unlock, variant: *const vdso_unlock_variant) {
    let mut lock = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let futex: *mut AtomicU32 = &mut lock.futex;
    let mut head: robust_list_head = core::mem::zeroed();
    let mut exp: usize = ptr::null::<c_void>() as usize;
    let tid: pid_t = gettid();
    let mut ret: c_int;

    if (*self_).vdso.is_none() {
        ksft_test_result_skip(c"%s not found\n".as_ptr(), (*variant).func_name);
        return;
    }

    (*futex).store(tid as u32, Ordering::SeqCst);

    ret = set_list(&mut head);
    if ret != 0 {
        ksft_test_result_fail(c"set_robust_list error\n".as_ptr());
    }

    head.list_op_pending = &mut lock.list;

    ret = ((*self_).vdso.unwrap())(futex, tid as u32, &mut head.list_op_pending as *mut _ as *mut c_void) as c_int;

    ASSERT_EQ!(ret, tid);
    ASSERT_EQ!((*futex).load(Ordering::SeqCst), 0);

    /* Check only the lower 32 bits for the 32-bit entry point */
    if (*variant).is_32 {
        exp = &mut lock.list as *mut _ as usize;
        exp &= !(0xFFFFFFFF_u64 as usize);
    }

    ASSERT_EQ!(head.list_op_pending as usize, exp);
}

/*
 * If the lock is contended, the operation fails. The return value is the value found at the
 * futex word (tid | FUTEX_WAITERS), the futex word is not modified and the list_op_pending is_32
 * not cleared.
 */
unsafe fn test_robust_try_unlock_contended(self_: *mut vdso_unlock, variant: *const vdso_unlock_variant) {
    let mut lock = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let futex: *mut AtomicU32 = &mut lock.futex;
    let mut head: robust_list_head = core::mem::zeroed();
    let tid: pid_t = gettid();
    let mut ret: c_int;

    if (*self_).vdso.is_none() {
        ksft_test_result_skip(c"%s not found\n".as_ptr(), (*variant).func_name);
        return;
    }

    (*futex).store(tid as u32 | FUTEX_WAITERS, Ordering::SeqCst);

    ret = set_list(&mut head);
    if ret != 0 {
        ksft_test_result_fail(c"set_robust_list error\n".as_ptr());
    }

    head.list_op_pending = &mut lock.list;

    ret = ((*self_).vdso.unwrap())(futex, tid as u32, &mut head.list_op_pending as *mut _ as *mut c_void) as c_int;

    ASSERT_EQ!(ret, (tid as u32 | FUTEX_WAITERS) as c_int);
    ASSERT_EQ!((*futex).load(Ordering::SeqCst), tid as u32 | FUTEX_WAITERS);
    ASSERT_EQ!(head.list_op_pending, &mut lock.list);
}

unsafe fn futex_op_setup(_self: *mut futex_op, _variant: *const futex_op_variant) {}

unsafe fn futex_op_teardown(_self: *mut futex_op) {}

static FUTEX_OP_WAKE: futex_op_variant = futex_op_variant {
    op: FUTEX_WAKE,
    val3: 0,
};

static FUTEX_OP_WAKE_BITSET: futex_op_variant = futex_op_variant {
    op: FUTEX_WAKE_BITSET,
    val3: FUTEX_BITSET_MATCH_ANY,
};

static FUTEX_OP_UNLOCK_PI: futex_op_variant = futex_op_variant {
    op: FUTEX_UNLOCK_PI,
    val3: 0,
};

static FUTEX_OP_WAKE32: futex_op_variant = futex_op_variant {
    op: FUTEX_WAKE | FUTEX_ROBUST_LIST32,
    val3: 0,
};

static FUTEX_OP_WAKE_BITSET32: futex_op_variant = futex_op_variant {
    op: FUTEX_WAKE_BITSET | FUTEX_ROBUST_LIST32,
    val3: FUTEX_BITSET_MATCH_ANY,
};

static FUTEX_OP_UNLOCK_PI32: futex_op_variant = futex_op_variant {
    op: FUTEX_UNLOCK_PI | FUTEX_ROBUST_LIST32,
    val3: 0,
};

/*
 * The syscall should return the number of tasks waken (for this test, 0), clear the futex word and
 * clear list_op_pending
 */
unsafe fn test_futex_robust_unlock(_self: *mut futex_op, variant: *const futex_op_variant) {
    let mut lock = lock_struct {
        futex: AtomicU32::new(0),
        list: robust_list { next: ptr::null_mut() },
    };
    let futex: *mut AtomicU32 = &mut lock.futex;
    let mut exp: usize = ptr::null::<c_void>() as usize;
    let mut head: robust_list_head = core::mem::zeroed();
    let tid: pid_t = gettid();
    let mut ret: c_int;

    if !BUILD_64 {
        if ((*variant).op & FUTEX_ROBUST_LIST32) == 0 {
            ksft_test_result_skip(c"Not supported for 32 bit build\n".as_ptr());
            return;
        }
    }

    (*futex).store(tid as u32 | FUTEX_WAITERS, Ordering::SeqCst);

    ret = set_list(&mut head);
    if ret != 0 {
        ksft_test_result_fail(c"set_robust_list error\n".as_ptr());
    }

    head.list_op_pending = &mut lock.list;

    ret = sys_futex_robust_unlock(
        futex,
        FUTEX_ROBUST_UNLOCK | (*variant).op,
        tid,
        &mut head.list_op_pending as *mut _ as *mut c_void,
        (*variant).val3,
    );

    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!((*futex).load(Ordering::SeqCst), 0);

    if ((*variant).op & FUTEX_ROBUST_LIST32) != 0 {
        exp = &mut lock.list as *mut _ as usize;
        exp &= !(0xFFFFFFFF_u64 as usize);
    }

    ASSERT_EQ!(head.list_op_pending as usize, exp);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

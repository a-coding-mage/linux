// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Various unit tests for the "ntsync" synchronization primitive driver.
 *
 * Copyright (C) 2021-2022 Elizabeth Figura <zfigura@codeweavers.com>
 */

/* Translated from C. Includes from sys/ioctl.h, sys/stat.h, sys/wait.h,
 * fcntl.h, sched.h, time.h, pthread.h, linux/ntsync.h, and
 * kselftest_harness.h are external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u32 = u32;
type __u64 = u64;
type pid_t = c_int;
type pthread_t = c_ulong;
type time_t = c_long;

const CLONE_NEWTIME: c_int = 0x00000080;
const STRESS_LOOPS: __u32 = 10000;
const STRESS_THREADS: usize = 4;

extern "C" {
    static mut errno: c_int;

    static NTSYNC_IOC_SEM_READ: c_ulong;
    static NTSYNC_IOC_SEM_RELEASE: c_ulong;
    static NTSYNC_IOC_MUTEX_READ: c_ulong;
    static NTSYNC_IOC_MUTEX_UNLOCK: c_ulong;
    static NTSYNC_IOC_WAIT_ANY: c_ulong;
    static NTSYNC_IOC_WAIT_ALL: c_ulong;
    static NTSYNC_IOC_CREATE_SEM: c_ulong;
    static NTSYNC_IOC_CREATE_MUTEX: c_ulong;
    static NTSYNC_IOC_MUTEX_KILL: c_ulong;
    static NTSYNC_IOC_CREATE_EVENT: c_ulong;
    static NTSYNC_IOC_EVENT_READ: c_ulong;
    static NTSYNC_IOC_EVENT_SET: c_ulong;
    static NTSYNC_IOC_EVENT_RESET: c_ulong;
    static NTSYNC_IOC_EVENT_PULSE: c_ulong;
    static NTSYNC_MAX_WAIT_COUNT: usize;

    static CLOCK_MONOTONIC: c_int;
    static CLOCK_REALTIME: c_int;
    static O_CLOEXEC: c_int;
    static O_RDONLY: c_int;
    static O_WRONLY: c_int;
    static F_OK: c_int;
    static EINVAL: c_int;
    static EOVERFLOW: c_int;
    static ETIMEDOUT: c_int;
    static EPERM: c_int;
    static EOWNERDEAD: c_int;
    static EBUSY: c_int;
    static UINT64_MAX: __u64;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_timedjoin_np(thread: pthread_t, retval: *mut *mut c_void,
                            abstime: *const timespec) -> c_int;
    fn pthread_tryjoin_np(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn gettid() -> pid_t;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ntsync_sem_args {
    count: __u32,
    max: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ntsync_mutex_args {
    owner: __u32,
    count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ntsync_event_args {
    manual: __u32,
    signaled: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ntsync_wait_args {
    timeout: __u64,
    objs: usize,
    count: __u32,
    owner: __u32,
    index: __u32,
    alert: c_int,
    pad: __u32,
}

macro_rules! EXPECT_EQ { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! EXPECT_LE { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! EXPECT_GE { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! EXPECT_TRUE { ($a:expr) => {{ let _ = $a; }}; }
macro_rules! ASSERT_LE { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! ASSERT_EQ { ($a:expr, $b:expr) => {{ let _ = ($a, $b); }}; }
macro_rules! SKIP { ($ret:expr, $msg:expr) => {{ let _ = $msg; return; }}; }

unsafe fn WIFEXITED(status: c_int) -> bool { (status & 0x7f) == 0 }
unsafe fn WEXITSTATUS(status: c_int) -> c_int { (status & 0xff00) >> 8 }

unsafe fn read_sem_state(sem: c_int, count: *mut __u32, max: *mut __u32) -> c_int {
    let mut args: ntsync_sem_args = zeroed();
    let ret: c_int;

    memset(&mut args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_sem_args>());
    ret = ioctl(sem, NTSYNC_IOC_SEM_READ, &mut args);
    *count = args.count;
    *max = args.max;
    ret
}

macro_rules! check_sem_state {
    ($sem:expr, $count:expr, $max:expr) => {{
        let mut __count: __u32 = 0;
        let mut __max: __u32 = 0;
        let ret = read_sem_state($sem, &mut __count, &mut __max);
        EXPECT_EQ!(0, ret);
        EXPECT_EQ!($count, __count);
        EXPECT_EQ!($max, __max);
    }};
}

unsafe fn release_sem(sem: c_int, count: *mut __u32) -> c_int {
    ioctl(sem, NTSYNC_IOC_SEM_RELEASE, count)
}

unsafe fn read_mutex_state(mutex: c_int, count: *mut __u32, owner: *mut __u32) -> c_int {
    let mut args: ntsync_mutex_args = zeroed();
    let ret: c_int;

    memset(&mut args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_mutex_args>());
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_READ, &mut args);
    *count = args.count;
    *owner = args.owner;
    ret
}

macro_rules! check_mutex_state {
    ($mutex:expr, $count:expr, $owner:expr) => {{
        let mut __count: __u32 = 0;
        let mut __owner: __u32 = 0;
        let ret = read_mutex_state($mutex, &mut __count, &mut __owner);
        EXPECT_EQ!(0, ret);
        EXPECT_EQ!($count, __count);
        EXPECT_EQ!($owner, __owner);
    }};
}

unsafe fn unlock_mutex(mutex: c_int, owner: __u32, count: *mut __u32) -> c_int {
    let mut args: ntsync_mutex_args;
    let ret: c_int;

    args.owner = owner;
    args.count = 0xdeadbeef;
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_UNLOCK, &mut args);
    *count = args.count;
    ret
}

unsafe fn read_event_state(event: c_int, signaled: *mut __u32, manual: *mut __u32) -> c_int {
    let mut args: ntsync_event_args = zeroed();
    let ret: c_int;

    memset(&mut args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_event_args>());
    ret = ioctl(event, NTSYNC_IOC_EVENT_READ, &mut args);
    *signaled = args.signaled;
    *manual = args.manual;
    ret
}

macro_rules! check_event_state {
    ($event:expr, $signaled:expr, $manual:expr) => {{
        let mut __signaled: __u32 = 0;
        let mut __manual: __u32 = 0;
        let ret = read_event_state($event, &mut __signaled, &mut __manual);
        EXPECT_EQ!(0, ret);
        EXPECT_EQ!($signaled, __signaled);
        EXPECT_EQ!($manual, __manual);
    }};
}

unsafe fn wait_objs(fd: c_int, request: c_ulong, count: __u32,
                    objs: *const c_int, owner: __u32, alert: c_int, index: *mut __u32) -> c_int {
    let mut args: ntsync_wait_args = zeroed();
    let mut timeout: timespec = zeroed();
    let ret: c_int;

    clock_gettime(CLOCK_MONOTONIC, &mut timeout);

    args.timeout = (timeout.tv_sec as __u64) * 1000000000 + timeout.tv_nsec as __u64;
    args.count = count;
    args.objs = objs as usize;
    args.owner = owner;
    args.index = 0xdeadbeef;
    args.alert = alert;
    ret = ioctl(fd, request, &mut args);
    *index = args.index;
    ret
}

unsafe fn wait_any(fd: c_int, count: __u32, objs: *const c_int, owner: __u32, index: *mut __u32) -> c_int {
    wait_objs(fd, NTSYNC_IOC_WAIT_ANY, count, objs, owner, 0, index)
}

unsafe fn wait_all(fd: c_int, count: __u32, objs: *const c_int, owner: __u32, index: *mut __u32) -> c_int {
    wait_objs(fd, NTSYNC_IOC_WAIT_ALL, count, objs, owner, 0, index)
}

unsafe fn wait_any_alert(fd: c_int, count: __u32, objs: *const c_int,
                         owner: __u32, alert: c_int, index: *mut __u32) -> c_int {
    wait_objs(fd, NTSYNC_IOC_WAIT_ANY, count, objs, owner, alert, index)
}

unsafe fn wait_all_alert(fd: c_int, count: __u32, objs: *const c_int,
                         owner: __u32, alert: c_int, index: *mut __u32) -> c_int {
    wait_objs(fd, NTSYNC_IOC_WAIT_ALL, count, objs, owner, alert, index)
}

unsafe fn semaphore_state() {
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut timeout: timespec = zeroed();
    let mut count: __u32 = 0;
    let mut index: __u32 = 0;
    let mut ret: c_int;

    clock_gettime(CLOCK_MONOTONIC, &mut timeout);

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 3;
    sem_args.max = 2;
    let mut sem = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_EQ!(-1, sem);
    EXPECT_EQ!(EINVAL, errno);

    sem_args.count = 2;
    sem_args.max = 2;
    sem = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, sem);
    check_sem_state!(sem, 2, 2);

    count = 0;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, count);
    check_sem_state!(sem, 2, 2);

    count = 1;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOVERFLOW, errno);
    check_sem_state!(sem, 2, 2);

    ret = wait_any(fd, 1, &sem, 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(sem, 1, 2);

    ret = wait_any(fd, 1, &sem, 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(sem, 0, 2);

    ret = wait_any(fd, 1, &sem, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    count = 3;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOVERFLOW, errno);
    check_sem_state!(sem, 0, 2);

    count = 2;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);
    check_sem_state!(sem, 2, 2);

    ret = wait_any(fd, 1, &sem, 123, &mut index);
    EXPECT_EQ!(0, ret);
    ret = wait_any(fd, 1, &sem, 123, &mut index);
    EXPECT_EQ!(0, ret);

    count = 1;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);
    check_sem_state!(sem, 1, 2);

    count = !0u32;
    ret = release_sem(sem, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOVERFLOW, errno);
    check_sem_state!(sem, 1, 2);

    close(sem);
    close(fd);
}

unsafe fn mutex_state() {
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut owner: __u32;
    let mut count: __u32 = 0;
    let mut index: __u32 = 0;
    let mut timeout: timespec = zeroed();
    let mut ret: c_int;

    clock_gettime(CLOCK_MONOTONIC, &mut timeout);

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    mutex_args.owner = 123;
    mutex_args.count = 0;
    let mut mutex = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_EQ!(-1, mutex);
    EXPECT_EQ!(EINVAL, errno);

    mutex_args.owner = 0;
    mutex_args.count = 2;
    mutex = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_EQ!(-1, mutex);
    EXPECT_EQ!(EINVAL, errno);

    mutex_args.owner = 123;
    mutex_args.count = 2;
    mutex = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, mutex);
    check_mutex_state!(mutex, 2, 123);

    ret = unlock_mutex(mutex, 0, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    ret = unlock_mutex(mutex, 456, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EPERM, errno);
    check_mutex_state!(mutex, 2, 123);

    ret = unlock_mutex(mutex, 123, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, count);
    check_mutex_state!(mutex, 1, 123);

    ret = unlock_mutex(mutex, 123, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, count);
    check_mutex_state!(mutex, 0, 0);

    ret = unlock_mutex(mutex, 123, &mut count);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EPERM, errno);

    ret = wait_any(fd, 1, &mutex, 456, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_mutex_state!(mutex, 1, 456);

    ret = wait_any(fd, 1, &mutex, 456, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_mutex_state!(mutex, 2, 456);

    ret = unlock_mutex(mutex, 456, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, count);
    check_mutex_state!(mutex, 1, 456);

    ret = wait_any(fd, 1, &mutex, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    owner = 0;
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    owner = 123;
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EPERM, errno);
    check_mutex_state!(mutex, 1, 456);

    owner = 456;
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(0, ret);

    memset(&mut mutex_args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_mutex_args>());
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_READ, &mut mutex_args);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(0, mutex_args.count);
    EXPECT_EQ!(0, mutex_args.owner);

    memset(&mut mutex_args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_mutex_args>());
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_READ, &mut mutex_args);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(0, mutex_args.count);
    EXPECT_EQ!(0, mutex_args.owner);

    ret = wait_any(fd, 1, &mutex, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(0, index);
    check_mutex_state!(mutex, 1, 123);

    owner = 123;
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(0, ret);

    memset(&mut mutex_args as *mut _ as *mut c_void, 0xcc, size_of::<ntsync_mutex_args>());
    ret = ioctl(mutex, NTSYNC_IOC_MUTEX_READ, &mut mutex_args);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(0, mutex_args.count);
    EXPECT_EQ!(0, mutex_args.owner);

    ret = wait_any(fd, 1, &mutex, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(0, index);
    check_mutex_state!(mutex, 1, 123);

    close(mutex);

    mutex_args.owner = 0;
    mutex_args.count = 0;
    mutex = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, mutex);
    check_mutex_state!(mutex, 0, 0);

    ret = wait_any(fd, 1, &mutex, 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_mutex_state!(mutex, 1, 123);

    close(mutex);

    mutex_args.owner = 123;
    mutex_args.count = !0u32;
    mutex = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, mutex);
    check_mutex_state!(mutex, !0u32, 123);

    ret = wait_any(fd, 1, &mutex, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    close(mutex);
    close(fd);
}

unsafe fn manual_event_state() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut index: __u32 = 0;
    let mut signaled: __u32;
    let mut ret: c_int;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    event_args.manual = 1;
    event_args.signaled = 0;
    let event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);
    check_event_state!(event, 0, 1);

    signaled = 0xdeadbeef;
    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(event, 1, 1);

    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);
    check_event_state!(event, 1, 1);

    ret = wait_any(fd, 1, &event, 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_event_state!(event, 1, 1);

    signaled = 0xdeadbeef;
    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);
    check_event_state!(event, 0, 1);

    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(event, 0, 1);

    ret = wait_any(fd, 1, &event, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);

    ret = ioctl(event, NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);
    check_event_state!(event, 0, 1);

    ret = ioctl(event, NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(event, 0, 1);

    close(event);
    close(fd);
}

unsafe fn auto_event_state() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut index: __u32 = 0;
    let mut signaled: __u32;
    let mut ret: c_int;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    event_args.manual = 0;
    event_args.signaled = 1;
    let event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);

    check_event_state!(event, 1, 0);

    signaled = 0xdeadbeef;
    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);
    check_event_state!(event, 1, 0);

    ret = wait_any(fd, 1, &event, 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_event_state!(event, 0, 0);

    signaled = 0xdeadbeef;
    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(event, 0, 0);

    ret = wait_any(fd, 1, &event, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);

    ret = ioctl(event, NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);
    check_event_state!(event, 0, 0);

    ret = ioctl(event, NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(event, 0, 0);

    close(event);
    close(fd);
}

unsafe fn test_wait_any() {
    let mut objs: [c_int; 65] = [0; 65]; /* NTSYNC_MAX_WAIT_COUNT + 1 */
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut owner: __u32;
    let mut index: __u32 = 0;
    let mut count: __u32;
    let mut i: __u32;
    let mut timeout: timespec = zeroed();
    let mut ret: c_int;

    clock_gettime(CLOCK_MONOTONIC, &mut timeout);

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 2;
    sem_args.max = 3;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    mutex_args.owner = 0;
    mutex_args.count = 0;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, objs[1]);

    ret = wait_any(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 1, 3);
    check_mutex_state!(objs[1], 0, 0);

    ret = wait_any(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 0, 0);

    ret = wait_any(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, index);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 1, 123);

    count = 1;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);

    ret = wait_any(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 1, 123);

    ret = wait_any(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, index);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 2, 123);

    ret = wait_any(fd, 2, objs.as_ptr(), 456, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    owner = 123;
    ret = ioctl(objs[1], NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(0, ret);

    ret = wait_any(fd, 2, objs.as_ptr(), 456, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    EXPECT_EQ!(1, index);

    ret = wait_any(fd, 2, objs.as_ptr(), 456, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, index);

    close(objs[1]);

    /* test waiting on the same object twice */

    count = 2;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);

    objs[1] = objs[0];
    ret = wait_any(fd, 2, objs.as_ptr(), 456, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 1, 3);

    ret = wait_any(fd, 0, null(), 456, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    i = 1;
    while (i as usize) < NTSYNC_MAX_WAIT_COUNT + 1 {
        objs[i as usize] = objs[0];
        i += 1;
    }

    ret = wait_any(fd, NTSYNC_MAX_WAIT_COUNT as __u32, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = wait_any(fd, (NTSYNC_MAX_WAIT_COUNT + 1) as __u32, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    ret = wait_any(fd, -1i32 as __u32, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    close(objs[0]);
    close(fd);
}

unsafe fn test_wait_all() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut owner: __u32;
    let mut index: __u32 = 0;
    let mut count: __u32;
    let mut objs: [c_int; 2] = [0; 2];
    let mut ret: c_int;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 2;
    sem_args.max = 3;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    mutex_args.owner = 0;
    mutex_args.count = 0;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, objs[1]);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 1, 3);
    check_mutex_state!(objs[1], 1, 123);

    ret = wait_all(fd, 2, objs.as_ptr(), 456, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);
    check_sem_state!(objs[0], 1, 3);
    check_mutex_state!(objs[1], 1, 123);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 2, 123);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);
    check_sem_state!(objs[0], 0, 3);
    check_mutex_state!(objs[1], 2, 123);

    count = 3;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 2, 3);
    check_mutex_state!(objs[1], 3, 123);

    owner = 123;
    ret = ioctl(objs[1], NTSYNC_IOC_MUTEX_KILL, &mut owner);
    EXPECT_EQ!(0, ret);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EOWNERDEAD, errno);
    check_sem_state!(objs[0], 1, 3);
    check_mutex_state!(objs[1], 1, 123);

    close(objs[1]);

    event_args.manual = 1;
    event_args.signaled = 1;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, objs[1]);

    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);
    check_sem_state!(objs[0], 0, 3);
    check_event_state!(objs[1], 1, 1);

    close(objs[1]);

    /* test waiting on the same object twice */
    objs[1] = objs[0];
    ret = wait_all(fd, 2, objs.as_ptr(), 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    close(objs[0]);
    close(fd);
}

#[repr(C)]
struct wake_args {
    fd: c_int,
    obj: c_int,
}

#[repr(C)]
struct wait_args {
    fd: c_int,
    request: c_ulong,
    args: *mut ntsync_wait_args,
    ret: c_int,
    err: c_int,
}

unsafe extern "C" fn wait_thread(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut wait_args;

    (*args).ret = ioctl((*args).fd, (*args).request, (*args).args);
    (*args).err = errno;
    null_mut()
}

unsafe fn get_abs_timeout(ms: c_uint) -> __u64 {
    let mut timeout: timespec = zeroed();
    clock_gettime(CLOCK_MONOTONIC, &mut timeout);
    (timeout.tv_sec as __u64 * 1000000000) + timeout.tv_nsec as __u64 + (ms as __u64 * 1000000)
}

type c_uint = u32;

unsafe fn wait_for_thread(thread: pthread_t, ms: c_uint) -> c_int {
    let mut timeout: timespec = zeroed();

    clock_gettime(CLOCK_REALTIME, &mut timeout);
    timeout.tv_nsec += (ms * 1000000) as c_long;
    timeout.tv_sec += timeout.tv_nsec / 1000000000;
    timeout.tv_nsec %= 1000000000;
    pthread_timedjoin_np(thread, null_mut(), &timeout)
}

unsafe fn wake_any() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut thread_args: wait_args = zeroed();
    let mut count: __u32;
    let mut index: __u32 = 0;
    let mut signaled: __u32 = 0;
    let mut objs: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut thread: pthread_t = 0;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 0;
    sem_args.max = 3;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    mutex_args.owner = 123;
    mutex_args.count = 1;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, objs[1]);

    /* test waking the semaphore */

    wait_args.timeout = get_abs_timeout(1000);
    wait_args.objs = objs.as_ptr() as usize;
    wait_args.count = 2;
    wait_args.owner = 456;
    wait_args.index = 0xdeadbeef;
    thread_args.fd = fd;
    thread_args.args = &mut wait_args;
    thread_args.request = NTSYNC_IOC_WAIT_ANY;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);

    count = 1;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);
    check_sem_state!(objs[0], 0, 3);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(0, wait_args.index);

    /* test waking the mutex */

    /* first grab it again for owner 123 */
    ret = wait_any(fd, 1, &objs[1], 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    wait_args.timeout = get_abs_timeout(1000);
    wait_args.owner = 456;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);

    ret = unlock_mutex(objs[1], 123, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, count);

    ret = pthread_tryjoin_np(thread, null_mut());
    EXPECT_EQ!(EBUSY, ret);

    ret = unlock_mutex(objs[1], 123, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, mutex_args.count);
    check_mutex_state!(objs[1], 1, 456);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(1, wait_args.index);

    close(objs[1]);

    /* test waking events */

    event_args.manual = 0;
    event_args.signaled = 0;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, objs[1]);

    wait_args.timeout = get_abs_timeout(1000);
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(objs[1], NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(objs[1], 0, 0);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(1, wait_args.index);

    wait_args.timeout = get_abs_timeout(1000);
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(objs[1], NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(objs[1], 0, 0);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(1, wait_args.index);

    close(objs[1]);

    event_args.manual = 1;
    event_args.signaled = 0;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, objs[1]);

    wait_args.timeout = get_abs_timeout(1000);
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(objs[1], NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(objs[1], 1, 1);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(1, wait_args.index);

    ret = ioctl(objs[1], NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);

    wait_args.timeout = get_abs_timeout(1000);
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(objs[1], NTSYNC_IOC_EVENT_PULSE, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);
    check_event_state!(objs[1], 0, 1);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(1, wait_args.index);

    /* delete an object while it's being waited on */
    wait_args.timeout = get_abs_timeout(200);
    wait_args.owner = 123;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    close(objs[0]);
    close(objs[1]);
    ret = wait_for_thread(thread, 200);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(-1, thread_args.ret);
    EXPECT_EQ!(ETIMEDOUT, thread_args.err);

    close(fd);
}

unsafe fn wake_all() {
    let mut manual_event_args: ntsync_event_args = zeroed();
    let mut auto_event_args: ntsync_event_args = zeroed();
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut thread_args: wait_args = zeroed();
    let mut count: __u32;
    let mut index: __u32 = 0;
    let mut signaled: __u32 = 0;
    let mut objs: [c_int; 4] = [0; 4];
    let mut ret: c_int;
    let mut thread: pthread_t = 0;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 0;
    sem_args.max = 3;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    mutex_args.owner = 123;
    mutex_args.count = 1;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, objs[1]);

    manual_event_args.manual = 1;
    manual_event_args.signaled = 1;
    objs[2] = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut manual_event_args);
    EXPECT_LE!(0, objs[2]);

    auto_event_args.manual = 0;
    auto_event_args.signaled = 1;
    objs[3] = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut auto_event_args);
    EXPECT_LE!(0, objs[3]);

    wait_args.timeout = get_abs_timeout(1000);
    wait_args.objs = objs.as_ptr() as usize;
    wait_args.count = 4;
    wait_args.owner = 456;
    thread_args.fd = fd;
    thread_args.args = &mut wait_args;
    thread_args.request = NTSYNC_IOC_WAIT_ALL;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);

    count = 1;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);

    ret = pthread_tryjoin_np(thread, null_mut());
    EXPECT_EQ!(EBUSY, ret);

    check_sem_state!(objs[0], 1, 3);

    ret = wait_any(fd, 1, &objs[0], 123, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = unlock_mutex(objs[1], 123, &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, count);

    ret = pthread_tryjoin_np(thread, null_mut());
    EXPECT_EQ!(EBUSY, ret);

    check_mutex_state!(objs[1], 0, 0);

    ret = ioctl(objs[2], NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);

    count = 2;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, count);
    check_sem_state!(objs[0], 2, 3);

    ret = ioctl(objs[3], NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, signaled);

    ret = ioctl(objs[2], NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);

    ret = ioctl(objs[3], NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, signaled);

    check_sem_state!(objs[0], 1, 3);
    check_mutex_state!(objs[1], 1, 456);
    check_event_state!(objs[2], 1, 1);
    check_event_state!(objs[3], 0, 0);

    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);

    /* delete an object while it's being waited on */
    wait_args.timeout = get_abs_timeout(200);
    wait_args.owner = 123;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    close(objs[0]);
    close(objs[1]);
    close(objs[2]);
    close(objs[3]);
    ret = wait_for_thread(thread, 200);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(-1, thread_args.ret);
    EXPECT_EQ!(ETIMEDOUT, thread_args.err);

    close(fd);
}

unsafe fn alert_any() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut index: __u32 = 0;
    let mut count: __u32;
    let mut signaled: __u32 = 0;
    let mut thread_args: wait_args = zeroed();
    let mut objs: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut thread: pthread_t = 0;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 0;
    sem_args.max = 2;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    sem_args.count = 1;
    sem_args.max = 2;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[1]);

    event_args.manual = 1;
    event_args.signaled = 1;
    let mut event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);

    ret = wait_any_alert(fd, 0, null(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);

    ret = wait_any_alert(fd, 0, null(), 123, event, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);

    ret = wait_any_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(1, index);

    ret = wait_any_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, index);

    /* test wakeup via alert */
    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);

    wait_args.timeout = get_abs_timeout(1000);
    wait_args.objs = objs.as_ptr() as usize;
    wait_args.count = 2;
    wait_args.owner = 123;
    wait_args.index = 0xdeadbeef;
    wait_args.alert = event;
    thread_args.fd = fd;
    thread_args.args = &mut wait_args;
    thread_args.request = NTSYNC_IOC_WAIT_ANY;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(2, wait_args.index);

    close(event);

    /* test with an auto-reset event */
    event_args.manual = 0;
    event_args.signaled = 1;
    event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);

    count = 1;
    ret = release_sem(objs[0], &mut count);
    EXPECT_EQ!(0, ret);

    ret = wait_any_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = wait_any_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, index);

    ret = wait_any_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    close(event);
    close(objs[0]);
    close(objs[1]);
    close(fd);
}

unsafe fn alert_all() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut thread_args: wait_args = zeroed();
    let mut index: __u32 = 0;
    let mut count: __u32;
    let mut signaled: __u32 = 0;
    let mut objs: [c_int; 2] = [0; 2];
    let mut ret: c_int;
    let mut thread: pthread_t = 0;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd);

    sem_args.count = 2;
    sem_args.max = 2;
    objs[0] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[0]);

    sem_args.count = 1;
    sem_args.max = 2;
    objs[1] = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_LE!(0, objs[1]);

    event_args.manual = 1;
    event_args.signaled = 1;
    let mut event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);

    ret = wait_all_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = wait_all_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, index);

    /* test wakeup via alert */
    ret = ioctl(event, NTSYNC_IOC_EVENT_RESET, &mut signaled);
    EXPECT_EQ!(0, ret);

    wait_args.timeout = get_abs_timeout(1000);
    wait_args.objs = objs.as_ptr() as usize;
    wait_args.count = 2;
    wait_args.owner = 123;
    wait_args.index = 0xdeadbeef;
    wait_args.alert = event;
    thread_args.fd = fd;
    thread_args.args = &mut wait_args;
    thread_args.request = NTSYNC_IOC_WAIT_ALL;
    ret = pthread_create(&mut thread, null(), wait_thread, &mut thread_args as *mut _ as *mut c_void);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(ETIMEDOUT, ret);
    ret = ioctl(event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);
    ret = wait_for_thread(thread, 100);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, thread_args.ret);
    EXPECT_EQ!(2, wait_args.index);

    close(event);

    /* test with an auto-reset event */
    event_args.manual = 0;
    event_args.signaled = 1;
    event = ioctl(fd, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, event);

    count = 2;
    ret = release_sem(objs[1], &mut count);
    EXPECT_EQ!(0, ret);

    ret = wait_all_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(0, index);

    ret = wait_all_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(0, ret);
    EXPECT_EQ!(2, index);

    ret = wait_all_alert(fd, 2, objs.as_ptr(), 123, event, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(ETIMEDOUT, errno);

    close(event);
    close(objs[0]);
    close(objs[1]);
    close(fd);
}

static mut stress_counter: c_uint = 0;
static mut stress_device: c_int = 0;
static mut stress_start_event: c_int = 0;
static mut stress_mutex: c_int = 0;

unsafe extern "C" fn stress_thread(_arg: *mut c_void) -> *mut c_void {
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut count: __u32 = 0;
    let mut i: __u32;

    wait_args.timeout = UINT64_MAX;
    wait_args.count = 1;
    wait_args.objs = &raw const stress_start_event as usize;
    wait_args.owner = gettid() as __u32;
    wait_args.index = 0xdeadbeef;

    ioctl(stress_device, NTSYNC_IOC_WAIT_ANY, &mut wait_args);

    wait_args.objs = &raw const stress_mutex as usize;

    i = 0;
    while i < STRESS_LOOPS {
        ioctl(stress_device, NTSYNC_IOC_WAIT_ANY, &mut wait_args);

        stress_counter += 1;

        unlock_mutex(stress_mutex, wait_args.owner, &mut count);
        i += 1;
    }

    null_mut()
}

unsafe fn stress_wait() {
    let mut event_args: ntsync_event_args = zeroed();
    let mut mutex_args: ntsync_mutex_args = zeroed();
    let mut threads: [pthread_t; STRESS_THREADS] = [0; STRESS_THREADS];
    let mut signaled: __u32 = 0;
    let mut i: __u32;
    let mut ret: c_int;

    stress_device = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, stress_device);

    mutex_args.owner = 0;
    mutex_args.count = 0;
    stress_mutex = ioctl(stress_device, NTSYNC_IOC_CREATE_MUTEX, &mut mutex_args);
    EXPECT_LE!(0, stress_mutex);

    event_args.manual = 1;
    event_args.signaled = 0;
    stress_start_event = ioctl(stress_device, NTSYNC_IOC_CREATE_EVENT, &mut event_args);
    EXPECT_LE!(0, stress_start_event);

    i = 0;
    while (i as usize) < STRESS_THREADS {
        pthread_create(&mut threads[i as usize], null(), stress_thread, null_mut());
        i += 1;
    }

    ret = ioctl(stress_start_event, NTSYNC_IOC_EVENT_SET, &mut signaled);
    EXPECT_EQ!(0, ret);

    i = 0;
    while (i as usize) < STRESS_THREADS {
        ret = pthread_join(threads[i as usize], null_mut());
        EXPECT_EQ!(0, ret);
        i += 1;
    }

    EXPECT_EQ!(STRESS_LOOPS * STRESS_THREADS as __u32, stress_counter);

    close(stress_start_event);
    close(stress_mutex);
    close(stress_device);
}

unsafe fn wait_args_validation() {
    let mut sem_args = ntsync_sem_args { count: 1, max: 1 };
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut timeout: timespec = zeroed();
    let mut index: __u32 = 0;
    let mut ret: c_int;

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_GE!(fd, 0);

    let fd2 = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    ASSERT_GE!(fd2, 0);

    let sem = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
    EXPECT_GE!(sem, 0);

    ret = wait_any(fd, 1, &sem, 0, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    ret = wait_all(fd, 1, &sem, 0, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    clock_gettime(CLOCK_MONOTONIC, &mut timeout);
    wait_args.timeout = timeout.tv_sec as __u64 * 1000000000u64 + timeout.tv_nsec as __u64;
    wait_args.count = 0;
    wait_args.objs = 0;
    wait_args.owner = 123;
    wait_args.pad = 1;
    ret = ioctl(fd, NTSYNC_IOC_WAIT_ANY, &mut wait_args);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    ret = wait_any(fd2, 1, &sem, 123, &mut index);
    EXPECT_EQ!(-1, ret);
    EXPECT_EQ!(EINVAL, errno);

    close(sem);
    close(fd2);
    close(fd);
}

/*
 * Absolute MONOTONIC timeouts must honour the caller's time namespace.
 * With a negative monotonic offset, a 100 ms wait must still take ~100 ms
 * of namespace time (not return immediately against the host clock).
 */
unsafe fn wait_any_monotonic_timens() {
    let mut sem_args: ntsync_sem_args = zeroed();
    let mut wait_args: ntsync_wait_args = zeroed();
    let mut start: timespec = zeroed();
    let mut end: timespec = zeroed();
    let mut buf: [c_char; 64] = [0; 64];
    let mut elapsed_ns: __u64;
    let mut ret: c_int;
    let mut status: c_int = 0;

    if access(c"/proc/self/ns/time".as_ptr(), F_OK) != 0 {
        SKIP!(return, "Time namespaces are not supported");
    }

    let fd = open(c"/dev/ntsync".as_ptr(), O_CLOEXEC | O_RDONLY);
    if fd < 0 {
        SKIP!(return, "/dev/ntsync is not available");
    }

    ret = unshare(CLONE_NEWTIME);
    if ret != 0 {
        close(fd);
        if errno == EPERM {
            SKIP!(return, "need CAP_SYS_ADMIN for CLONE_NEWTIME");
        }
        ASSERT_EQ!(0, ret);
    }

    let len = snprintf(buf.as_mut_ptr(), buf.len(), c"%d %d 0".as_ptr(), CLOCK_MONOTONIC, -10);
    let offset_fd = open(c"/proc/self/timens_offsets".as_ptr(), O_WRONLY);
    ASSERT_LE!(0, offset_fd);
    ASSERT_EQ!(len, write(offset_fd, buf.as_ptr() as *const c_void, len as usize) as c_int);
    close(offset_fd);

    let pid = fork();
    ASSERT_LE!(0, pid);
    if pid == 0 {
        let mut obj: c_int;

        sem_args.count = 0;
        sem_args.max = 1;
        let sem = ioctl(fd, NTSYNC_IOC_CREATE_SEM, &mut sem_args);
        if sem < 0 {
            _exit(1);
        }

        obj = sem;
        wait_args.timeout = get_abs_timeout(100);
        wait_args.objs = &mut obj as *mut _ as usize;
        wait_args.count = 1;
        wait_args.owner = 123;
        wait_args.index = 0xdeadbeef;

        if clock_gettime(CLOCK_MONOTONIC, &mut start) != 0 {
            _exit(2);
        }
        ret = ioctl(fd, NTSYNC_IOC_WAIT_ANY, &mut wait_args);
        if clock_gettime(CLOCK_MONOTONIC, &mut end) != 0 {
            _exit(2);
        }

        if ret != -1 || errno != ETIMEDOUT {
            _exit(3);
        }

        elapsed_ns = ((end.tv_sec - start.tv_sec) as __u64) * 1000000000u64
            + (end.tv_nsec - start.tv_nsec) as __u64;
        /* Without timens conversion this returns in ~0 ms. */
        if elapsed_ns < 50 * 1000000u64 {
            _exit(4);
        }
        if elapsed_ns > 1000 * 1000000u64 {
            _exit(5);
        }

        _exit(0);
    }

    ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
    EXPECT_TRUE!(WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));

    close(fd);
}

/* TEST_HARNESS_MAIN */

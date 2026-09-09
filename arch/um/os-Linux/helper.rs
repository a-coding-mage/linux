// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External declarations supplied by the surrounding UML environment.
#[repr(C)]
pub struct pthread_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 128],
}

extern "C" {
    fn alloc_stack(order: c_int, cant_sleep: c_int) -> c_ulong;
    fn free_stack(stack: c_ulong, order: c_int);
    fn __uml_cant_sleep() -> c_int;
    fn socketpair(domain: c_int, ty: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn os_set_exec_close(fd: c_int) -> c_int;
    fn clone(f: unsafe extern "C" fn(*mut c_void) -> c_int, child_stack: *mut c_void,
             flags: c_int, arg: *mut c_void) -> c_int;
    fn execvp_noalloc(buf: *mut c_char, file: *mut c_char, argv: *mut *mut c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      arg: *mut c_void) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn printk(fmt: *const c_char, ...);
    fn panic(fmt: *const c_char, ...);
    fn uml_kmalloc(size: usize, flags: c_int) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ECHILD: c_int = 10;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const CLONE_VM: c_int = 0x00000100;
const __WALL: c_int = 0x40000000;
const SIG_SETMASK: c_int = 2;
const SIGWINCH: c_int = 28;
const SIGPIPE: c_int = 13;
const SIGPROF: c_int = 27;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGCHLD: c_int = 17;
const SIGALRM: c_int = 14;
const SIGIO: c_int = 29;
const SIGUSR1: c_int = 10;
const UM_KERN_PAGE_SIZE: c_ulong = 4096;
const UM_GFP_ATOMIC: c_int = 0;
const UM_GFP_KERNEL: c_int = 0;
const PATH_MAX: usize = 4096;

#[repr(C)]
struct helper_data {
    pre_exec: Option<unsafe extern "C" fn(*mut c_void)>,
    pre_data: *mut c_void,
    argv: *mut *mut c_char,
    fd: c_int,
    buf: *mut c_char,
}

unsafe extern "C" fn helper_child(arg: *mut c_void) -> c_int {
    let data = &mut *(arg as *mut helper_data);
    let argv = data.argv;
    let mut err: c_int;
    let mut ret: isize;

    if let Some(pre_exec) = data.pre_exec {
        pre_exec(data.pre_data);
    }
    err = execvp_noalloc(data.buf, *argv, argv);

    // If the exec succeeds, we don't get here
    ret = write(data.fd, &err as *const c_int as *const c_void, core::mem::size_of::<c_int>());
    while ret < 0 && *libc_errno() == 4 {
        ret = write(data.fd, &err as *const c_int as *const c_void, core::mem::size_of::<c_int>());
    }

    0
}

unsafe fn libc_errno() -> *mut c_int {
    __errno_location()
}

extern "C" {
    fn __errno_location() -> *mut c_int;
}

// Returns either the pid of the child process we run or -E* on failure.
pub unsafe fn run_helper(pre_exec: Option<unsafe extern "C" fn(*mut c_void)>,
                         pre_data: *mut c_void, argv: *mut *mut c_char) -> c_int {
    let mut data: helper_data = core::mem::zeroed();
    let (mut stack, mut sp): (c_ulong, c_ulong);
    let (mut pid, mut fds, mut ret, mut n): (c_int, [c_int; 2], c_int, isize);

    stack = alloc_stack(0, __uml_cant_sleep());
    if stack == 0 { return -ENOMEM; }

    ret = socketpair(AF_UNIX, SOCK_STREAM, 0, fds.as_mut_ptr());
    if ret < 0 { ret = -*libc_errno(); goto_out_free(stack, ret); }

    ret = os_set_exec_close(fds[1]);
    if ret < 0 { return finish_run_helper(data, fds, stack, ret); }

    sp = stack + UM_KERN_PAGE_SIZE;
    data.pre_exec = pre_exec;
    data.pre_data = pre_data;
    data.argv = argv;
    data.fd = fds[1];
    data.buf = uml_kmalloc(PATH_MAX, if __uml_cant_sleep() != 0 { UM_GFP_ATOMIC } else { UM_GFP_KERNEL });
    pid = clone(helper_child, sp as *mut c_void, CLONE_VM, &mut data as *mut _ as *mut c_void);
    if pid < 0 { ret = -*libc_errno(); return finish_run_helper(data, fds, stack, ret); }

    close(fds[1]); fds[1] = -1;
    n = read(fds[0], &mut ret as *mut c_int as *mut c_void, core::mem::size_of::<c_int>());
    if n == 0 { ret = pid; } else {
        if n < 0 { n = -*libc_errno() as isize; ret = n as c_int; }
        waitpid(pid, core::ptr::null_mut(), __WALL);
    }
    kfree(data.buf as *mut c_void);
    if fds[1] != -1 { close(fds[1]); } close(fds[0]); free_stack(stack, 0); ret
}

unsafe fn goto_out_free(_stack: c_ulong, ret: c_int) -> c_int { ret }
unsafe fn finish_run_helper(mut data: helper_data, fds: [c_int; 2], stack: c_ulong, ret: c_int) -> c_int {
    kfree(data.buf as *mut c_void); if fds[1] != -1 { close(fds[1]); } close(fds[0]); free_stack(stack, 0); ret
}

pub unsafe fn run_helper_thread(proc: unsafe extern "C" fn(*mut c_void) -> c_int, arg: *mut c_void,
                                flags: c_uint, stack_out: *mut c_ulong) -> c_int {
    let mut stack = alloc_stack(0, __uml_cant_sleep());
    let mut sp: c_ulong;
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut err: c_int;
    if flags & CLONE_VM as c_uint != 0 { return -EINVAL; }
    if stack == 0 { return -ENOMEM; }
    sp = stack + UM_KERN_PAGE_SIZE;
    pid = clone(proc, sp as *mut c_void, flags as c_int, arg);
    if pid < 0 { err = -*libc_errno(); return err; }
    if stack_out.is_null() {
        pid = waitpid(pid, &mut status, __WALL);
        if pid < 0 { err = -*libc_errno(); pid = err; }
        free_stack(stack, 0);
    } else { *stack_out = stack; }
    pid
}
pub unsafe fn helper_wait(pid: c_int) -> c_int {
    let mut status = 0;
    let ret = waitpid(pid, &mut status, __WALL);
    if ret < 0 { -(*libc_errno()) } else if status != 0 { -ECHILD } else { 0 }
}

#[repr(C)]
pub struct os_helper_thread { handle: pthread_t }

pub unsafe fn os_run_helper_thread(td_out: *mut *mut os_helper_thread,
                                   routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                                   arg: *mut c_void) -> c_int {
    let td = uml_kmalloc(core::mem::size_of::<os_helper_thread>(), UM_GFP_KERNEL) as *mut os_helper_thread;
    if td.is_null() { return -ENOMEM; }
    let mut sigset: sigset_t = core::mem::zeroed();
    let mut oldset: sigset_t = core::mem::zeroed();
    sigfillset(&mut sigset);
    if sigprocmask(SIG_SETMASK, &sigset, &mut oldset) < 0 { kfree(td as *mut c_void); return -*libc_errno(); }
    let err = pthread_create(&mut (*td).handle, core::ptr::null(), routine, arg);
    if sigprocmask(SIG_SETMASK, &oldset, core::ptr::null_mut()) < 0 { panic(core::ptr::null(),); }
    if err != 0 { kfree(td as *mut c_void); } else { *td_out = td; }
    -err
}
pub unsafe fn os_kill_helper_thread(td: *mut os_helper_thread) {
    pthread_cancel((*td).handle); pthread_join((*td).handle, core::ptr::null_mut()); kfree(td as *mut c_void);
}
pub unsafe fn os_fix_helper_thread_signals() {
    let mut sigset: sigset_t = core::mem::zeroed(); sigemptyset(&mut sigset);
    sigaddset(&mut sigset, SIGWINCH); sigaddset(&mut sigset, SIGPIPE); sigaddset(&mut sigset, SIGPROF);
    sigaddset(&mut sigset, SIGINT); sigaddset(&mut sigset, SIGTERM); sigaddset(&mut sigset, SIGCHLD);
    sigaddset(&mut sigset, SIGALRM); sigaddset(&mut sigset, SIGIO); sigaddset(&mut sigset, SIGUSR1);
    pthread_sigmask(SIG_SETMASK, &sigset, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

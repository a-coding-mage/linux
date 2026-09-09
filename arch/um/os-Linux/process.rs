// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use std::os::raw::{c_int, c_uint, c_ulong, c_ulonglong, c_void};

// Declarations supplied by the surrounding UML and libc environment.
extern "C" {
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn ptrace(request: c_int, pid: c_int, ...) -> c_long;
    fn syscall(number: c_long, ...) -> c_long;
    fn mmap64(addr: *mut c_void, length: usize, prot: c_int, flags: c_int,
              fd: c_int, offset: c_ulonglong) -> *mut c_void;
    fn mprotect(addr: *mut c_void, length: usize, prot: c_int) -> c_int;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn prctl(option: c_int, ...) -> c_int;
    fn block_signals();
    fn unblock_signals();
    fn set_handler(signum: c_int);
    fn create_mem_file(size: usize) -> c_int;
    fn printk(fmt: *const u8, ...);
    static using_seccomp: c_int;
}

type c_long = i64;

pub unsafe fn os_alarm_process(pid: c_int) {
    if pid <= 0 { return; }
    kill(pid, SIGALRM);
}

pub unsafe fn os_kill_process(pid: c_int, reap_child: c_int) {
    if pid <= 0 { return; }
    block_signals();
    kill(pid, SIGKILL);
    if reap_child != 0 {
        let mut status = 0;
        catch_eintr_waitpid(pid, &mut status, __WALL);
    }
    unblock_signals();
}

/* Kill off a ptraced child by all means available.  kill it normally first,
 * then PTRACE_KILL it, then PTRACE_CONT it in case it's in a run state from
 * which it can't exit directly.
 */
pub unsafe fn os_kill_ptraced_process(pid: c_int, reap_child: c_int) {
    if pid <= 0 { return; }
    block_signals();
    kill(pid, SIGKILL);
    ptrace(PTRACE_KILL, pid);
    ptrace(PTRACE_CONT, pid);
    if reap_child != 0 {
        let mut status = 0;
        catch_eintr_waitpid(pid, &mut status, __WALL);
    }
    unblock_signals();
}

pub unsafe fn os_reap_child() -> c_int {
    let mut status = 0;
    waitpid(-1, &mut status, WNOHANG)
}

/* Don't use the glibc version, which caches the result in TLS. It misses some
 * syscalls, and also breaks with clone(), which does not unshare the TLS.
 */
pub unsafe fn os_getpid() -> c_int { syscall(__NR_getpid) as c_int }

pub unsafe fn os_map_memory(virt: *mut c_void, fd: c_int, off: c_ulonglong,
                            len: c_ulong, r: c_int, w: c_int, x: c_int) -> c_int {
    let prot = (if r != 0 { PROT_READ } else { 0 }) |
               (if w != 0 { PROT_WRITE } else { 0 }) |
               (if x != 0 { PROT_EXEC } else { 0 });
    let loc = mmap64(virt, len as usize, prot, MAP_SHARED | MAP_FIXED, fd, off);
    if loc == MAP_FAILED { return -errno(); }
    0
}

pub unsafe fn os_protect_memory(addr: *mut c_void, len: c_ulong,
                                r: c_int, w: c_int, x: c_int) -> c_int {
    let prot = (if r != 0 { PROT_READ } else { 0 }) |
               (if w != 0 { PROT_WRITE } else { 0 }) |
               (if x != 0 { PROT_EXEC } else { 0 });
    if mprotect(addr, len as usize, prot) < 0 { return -errno(); }
    0
}

pub unsafe fn os_unmap_memory(addr: *mut c_void, len: c_int) -> c_int {
    if munmap(addr, len as usize) < 0 { return -errno(); }
    0
}

pub unsafe fn os_drop_memory(addr: *mut c_void, length: c_int) -> c_int {
    let mut err = madvise(addr, length as usize, MADV_REMOVE);
    if err < 0 { err = -errno(); }
    err
}

pub unsafe fn can_drop_memory() -> c_int {
    let mut addr: *mut c_void;
    let mut fd: c_int;
    let mut ok = 0;
    printk(b"Checking host MADV_REMOVE support...\0".as_ptr());
    fd = create_mem_file(UM_KERN_PAGE_SIZE);
    if fd < 0 {
        printk(b"Creating test memory file failed, err = %d\n\0".as_ptr(), -fd);
        return ok;
    }
    addr = mmap64(std::ptr::null_mut(), UM_KERN_PAGE_SIZE,
                  PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if addr == MAP_FAILED {
        printk(b"Mapping test memory file failed, err = %d\n\0".as_ptr(), errno());
        close(fd);
        return ok;
    }
    if madvise(addr, UM_KERN_PAGE_SIZE, MADV_REMOVE) != 0 {
        printk(b"MADV_REMOVE failed, err = %d\n\0".as_ptr(), errno());
        munmap(addr, UM_KERN_PAGE_SIZE);
        close(fd);
        return ok;
    }
    printk(b"OK\n\0".as_ptr());
    ok = 1;
    munmap(addr, UM_KERN_PAGE_SIZE);
    close(fd);
    ok
}

pub unsafe fn init_new_thread_signals() {
    set_handler(SIGSEGV); set_handler(SIGTRAP); set_handler(SIGFPE);
    set_handler(SIGILL); set_handler(SIGBUS);
    signal(SIGHUP, SIG_IGN); set_handler(SIGIO);
    if using_seccomp != 0 { set_handler(SIGCHLD); }
    signal(SIGWINCH, SIG_IGN);
}

pub unsafe fn os_set_pdeathsig() { prctl(PR_SET_PDEATHSIG, SIGKILL); }

pub unsafe fn os_futex_wait(uaddr: *mut c_void, val: c_uint) -> c_int {
    let r = syscall(__NR_futex, uaddr, FUTEX_WAIT, val, 0, 0, 0);
    if r < 0 { -errno() } else { r as c_int }
}

pub unsafe fn os_futex_wake(uaddr: *mut c_void) -> c_int {
    let r = syscall(__NR_futex, uaddr, FUTEX_WAKE, INT_MAX, 0, 0, 0);
    if r < 0 { -errno() } else { r as c_int }
}

// These constants and helpers are provided by the target headers/build.
extern "C" { fn errno() -> c_int; fn catch_eintr_waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int; }
const SIGALRM: c_int = 14; const SIGKILL: c_int = 9; const SIGSEGV: c_int = 11;
const SIGTRAP: c_int = 5; const SIGFPE: c_int = 8; const SIGILL: c_int = 4;
const SIGBUS: c_int = 7; const SIGHUP: c_int = 1; const SIGIO: c_int = 29;
const SIGCHLD: c_int = 17; const SIGWINCH: c_int = 28; const SIG_IGN: usize = 1;
const PROT_READ: c_int = 1; const PROT_WRITE: c_int = 2; const PROT_EXEC: c_int = 4;
const MAP_SHARED: c_int = 1; const MAP_FIXED: c_int = 16; const MAP_FAILED: *mut c_void = !0 as *mut c_void;
const WNOHANG: c_int = 1; const __WALL: c_int = 0x40000000;
const PTRACE_KILL: c_int = 8; const PTRACE_CONT: c_int = 7;
const FUTEX_WAIT: c_int = 0; const FUTEX_WAKE: c_int = 1; const MADV_REMOVE: c_int = 9;
const PR_SET_PDEATHSIG: c_int = 1; const __NR_getpid: c_long = 39; const __NR_futex: c_long = 202;
const INT_MAX: c_uint = 0x7fffffff; const UM_KERN_PAGE_SIZE: usize = 4096;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

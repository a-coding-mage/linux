// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependencies from the original C file:
 * _GNU_SOURCE, <sched.h>, <test_progs.h>, <time.h>, <sys/mman.h>,
 * <sys/syscall.h>, and "fexit_sleep.lskel.h".
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type pid_t = c_int;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct fexit_sleep_lskel {
    bss: *mut fexit_sleep_lskel_bss,
    progs: fexit_sleep_lskel_progs,
}

#[repr(C)]
struct fexit_sleep_lskel_bss {
    pid: pid_t,
    fentry_cnt: c_int,
    fexit_cnt: c_int,
}

#[repr(C)]
struct fexit_sleep_lskel_progs {
    nanosleep_fentry: fexit_sleep_lskel_prog,
    nanosleep_fexit: fexit_sleep_lskel_prog,
}

#[repr(C)]
struct fexit_sleep_lskel_prog {
    prog_fd: c_int,
}

const STACK_SIZE: usize = 1024 * 1024;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_STACK: c_int = 0x20000;
const CLONE_FILES: c_int = 0x00000400;
const SIGCHLD: c_int = 17;
const __NR_nanosleep: c_long = 35;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    fn fexit_sleep_lskel__open_and_load() -> *mut fexit_sleep_lskel;
    fn fexit_sleep_lskel__attach(skel: *mut fexit_sleep_lskel) -> c_int;
    fn fexit_sleep_lskel__detach(skel: *mut fexit_sleep_lskel);
    fn fexit_sleep_lskel__destroy(skel: *mut fexit_sleep_lskel);

    fn getpid() -> pid_t;
    fn syscall(num: c_long, ...) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

/* test_progs.h macros used by this file. */
unsafe extern "C" {
    fn CHECK(condition: bool, tag: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_NEQ(actual: *mut c_void, expected: *mut c_void, name: *const c_char) -> bool;
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn do_sleep(skel: *mut c_void) -> c_int {
    let fexit_skel: *mut fexit_sleep_lskel = skel as *mut fexit_sleep_lskel;
    let mut ts1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 1,
    };
    let mut ts2: timespec = timespec {
        tv_sec: 10,
        tv_nsec: 0,
    };

    (*(*fexit_skel).bss).pid = getpid();
    let _ = syscall(
        __NR_nanosleep,
        &mut ts1 as *mut timespec,
        ptr::null_mut::<c_void>(),
    );
    let _ = syscall(
        __NR_nanosleep,
        &mut ts2 as *mut timespec,
        ptr::null_mut::<c_void>(),
    );
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fexit_sleep() {
    let mut fexit_skel: *mut fexit_sleep_lskel = ptr::null_mut();
    let mut wstatus: c_int = 0;
    let duration: c_int = 0;
    let mut cpid: pid_t;
    let mut child_stack: *mut c_char = ptr::null_mut();
    let mut err: c_int;
    let mut fexit_cnt: c_int;

    fexit_skel = fexit_sleep_lskel__open_and_load();
    if CHECK(
        fexit_skel.is_null(),
        c"fexit_skel_load".as_ptr(),
        c"fexit skeleton failed\n".as_ptr(),
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    err = fexit_sleep_lskel__attach(fexit_skel);
    if CHECK(
        err != 0,
        c"fexit_attach".as_ptr(),
        c"fexit attach failed: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    child_stack = mmap(
        ptr::null_mut(),
        STACK_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
        -1,
        0,
    ) as *mut c_char;
    if !ASSERT_NEQ(child_stack as *mut c_void, MAP_FAILED, c"mmap".as_ptr()) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    cpid = clone(
        do_sleep,
        child_stack.add(STACK_SIZE) as *mut c_void,
        CLONE_FILES | SIGCHLD,
        fexit_skel as *mut c_void,
    );
    if CHECK(
        cpid == -1,
        c"clone".as_ptr(),
        c"%s\n".as_ptr(),
        strerror(*__errno_location()),
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    /* wait until first sys_nanosleep ends and second sys_nanosleep starts */
    while ptr::read_volatile(&(*(*fexit_skel).bss).fentry_cnt) != 2 {}
    fexit_cnt = ptr::read_volatile(&(*(*fexit_skel).bss).fexit_cnt);
    if CHECK(
        fexit_cnt != 1,
        c"fexit_cnt".as_ptr(),
        c"%d".as_ptr(),
        fexit_cnt,
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    /* close progs and detach them. That will trigger two nop5->jmp5 rewrites
     * in the trampolines to skip nanosleep_fexit prog.
     * The nanosleep_fentry prog will get detached first.
     * The nanosleep_fexit prog will get detached second.
     * Detaching will trigger freeing of both progs JITed images.
     * There will be two dying bpf_tramp_image-s, but only the initial
     * bpf_tramp_image (with both _fentry and _fexit progs will be stuck
     * waiting for percpu_ref_kill to confirm). The other one
     * will be freed quickly.
     */
    close((*fexit_skel).progs.nanosleep_fentry.prog_fd);
    close((*fexit_skel).progs.nanosleep_fexit.prog_fd);
    fexit_sleep_lskel__detach(fexit_skel);

    /* kill the thread to unwind sys_nanosleep stack through the trampoline */
    kill(cpid, 9);

    if CHECK(
        waitpid(cpid, &mut wstatus as *mut c_int, 0) == -1,
        c"waitpid".as_ptr(),
        c"%s\n".as_ptr(),
        strerror(*__errno_location()),
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }
    if CHECK(
        WEXITSTATUS(wstatus) != 0,
        c"exitstatus".as_ptr(),
        c"failed".as_ptr(),
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    /* The bypassed nanosleep_fexit prog shouldn't have executed.
     * Unlike progs the maps were not freed and directly accessible.
     */
    fexit_cnt = ptr::read_volatile(&(*(*fexit_skel).bss).fexit_cnt);
    if CHECK(
        fexit_cnt != 1,
        c"fexit_cnt".as_ptr(),
        c"%d".as_ptr(),
        fexit_cnt,
    ) {
        goto_cleanup(child_stack, fexit_skel);
        return;
    }

    let _ = duration;
    goto_cleanup(child_stack, fexit_skel);
}

unsafe fn goto_cleanup(child_stack: *mut c_char, fexit_skel: *mut fexit_sleep_lskel) {
    munmap(child_stack as *mut c_void, STACK_SIZE);
    fexit_sleep_lskel__destroy(fexit_skel);
}

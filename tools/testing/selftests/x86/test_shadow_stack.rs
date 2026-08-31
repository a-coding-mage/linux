// SPDX-License-Identifier: GPL-2.0
/*
 * This program test's basic kernel shadow stack support. It enables shadow
 * stack manual via the arch_prctl(), instead of relying on glibc. It's
 * Makefile doesn't compile with shadow stack support, so it doesn't rely on
 * any particular glibc. As a result it can't do any operations that require
 * special glibc shadow stack support (longjmp(), swapcontext(), etc). Just
 * stick to the basics and hope the compiler doesn't do anything strange.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::{asm, global_asm};
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type __u64 = u64;
type uintptr_t = usize;

/*
 * Define the ABI defines if needed, so people can run the tests
 * without building the headers.
 */
const __NR_map_shadow_stack: c_long = 453;

const SHADOW_STACK_SET_TOKEN: c_ulong = 1u64 as c_ulong << 0;

const ARCH_SHSTK_ENABLE: c_long = 0x5001;
const ARCH_SHSTK_DISABLE: c_long = 0x5002;
const ARCH_SHSTK_LOCK: c_long = 0x5003;
const ARCH_SHSTK_UNLOCK: c_long = 0x5004;
const ARCH_SHSTK_STATUS: c_long = 0x5005;

const ARCH_SHSTK_SHSTK: c_ulong = 1u64 as c_ulong << 0;
const ARCH_SHSTK_WRSS: c_ulong = 1u64 as c_ulong << 1;

const NT_X86_SHSTK: c_int = 0x204;

const SS_SIZE: usize = 0x200000;
const PAGE_SIZE: usize = 0x1000;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_32BIT: c_int = 0x40;
const MADV_DONTNEED: c_int = 4;
const O_RDWR: c_int = 0o2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o4000;
const SEEK_SET: c_int = 0;
const SIGSEGV: c_int = 11;
const SIGUSR1: c_int = 10;
const SIGTRAP: c_int = 5;
const SIGKILL: c_int = 9;
const SIG_DFL: usize = 0;
const SA_SIGINFO: c_int = 4;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const ESRCH: c_int = 3;
const ENOENT: c_int = 2;
const EOF_: c_int = -1;
const __NR_arch_prctl: c_long = 158;
const __NR_userfaultfd: c_long = 323;
const __NR_perf_event_open: c_long = 298;
const PERF_FLAG_FD_CLOEXEC: c_ulong = 8;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;
const PTRACE_DETACH: c_int = 17;
const UFFD_API: __u64 = 0xAA;
const UFFDIO_REGISTER_MODE_MISSING: __u64 = 1 << 0;
const UFFDIO_API: c_ulong = 0xC018AA3F;
const UFFDIO_REGISTER: c_ulong = 0xC020AA00;
const UFFDIO_COPY: c_ulong = 0xC028AA03;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub union sighandler_t {
    sa_handler: usize,
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
}

#[repr(C)]
pub struct sigaction {
    sa: sighandler_t,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>,
}

#[repr(C)]
pub struct pthread_t {
    value: c_ulong,
}

#[repr(C)]
pub struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
pub struct uffd_msg_pagefault {
    flags: __u64,
    address: __u64,
    reserved: __u64,
}

#[repr(C)]
pub union uffd_msg_arg {
    pagefault: uffd_msg_pagefault,
}

#[repr(C)]
pub struct uffd_msg {
    event: u8,
    reserved1: u8,
    reserved2: u16,
    reserved3: u32,
    arg: uffd_msg_arg,
}

#[repr(C)]
pub struct uffdio_range {
    start: __u64,
    len: __u64,
}

#[repr(C)]
pub struct uffdio_register {
    range: uffdio_range,
    mode: __u64,
    ioctls: __u64,
}

#[repr(C)]
pub struct uffdio_api {
    api: __u64,
    features: __u64,
    ioctls: __u64,
}

#[repr(C)]
pub struct uffdio_copy {
    dst: __u64,
    src: __u64,
    len: __u64,
    mode: __u64,
    copy: i64,
}

#[repr(C)]
pub struct perf_event_attr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period_or_freq: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
    wakeup_events_or_watermark: u32,
    bp_type: u32,
    config1: u64,
    config2: u64,
}

type sigjmp_buf = [c_long; 32];

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn ptrace(request: c_int, ...) -> c_long;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;
    static mut errno: c_int;
}

#[inline]
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
unsafe fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

unsafe fn write_shstk(addr: *mut c_ulong, val: c_ulong) {
    asm!("wrssq {val}, ({addr})", addr = in(reg) addr, val = in(reg) val, options(nostack));
}

#[inline(always)]
unsafe fn get_ssp() -> c_ulong {
    let mut ret: c_ulong;
    asm!("xor {0}, {0}; rdsspq {0}", out(reg) ret, options(nostack));
    ret
}

/*
 * For use in inline enablement of shadow stack.
 *
 * The program can't return from the point where shadow stack gets enabled
 * because there will be no address on the shadow stack. So it can't use
 * syscall() for enablement, since it is a function.
 *
 * Based on code from nolibc.h. Keep a copy here because this can't pull in all
 * of nolibc.h.
 */
unsafe fn ARCH_PRCTL(arg1: c_long, arg2: c_ulong) -> c_long {
    let ret: c_long;
    asm!(
        "syscall",
        inlateout("rax") __NR_arch_prctl => ret,
        in("rdi") arg1,
        in("rsi") arg2 as c_long,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    ret
}

unsafe fn create_shstk(addr: *mut c_void) -> *mut c_void {
    syscall(__NR_map_shadow_stack, addr, SS_SIZE, SHADOW_STACK_SET_TOKEN) as *mut c_void
}

unsafe fn create_normal_mem(addr: *mut c_void) -> *mut c_void {
    mmap(addr, SS_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, 0, 0)
}

unsafe fn free_shstk(shstk: *mut c_void) {
    munmap(shstk, SS_SIZE);
}

unsafe fn reset_shstk(shstk: *mut c_void) -> c_int {
    madvise(shstk, SS_SIZE, MADV_DONTNEED)
}

unsafe fn try_shstk(new_ssp: c_ulong) {
    printf(b"[INFO]\tnew_ssp = %lx, *new_ssp = %lx\n\0".as_ptr() as *const c_char, new_ssp, *(new_ssp as *mut c_ulong));
    let mut ssp = get_ssp();
    printf(b"[INFO]\tchanging ssp from %lx to %lx\n\0".as_ptr() as *const c_char, ssp, new_ssp);
    asm!("rstorssp ({0})", in(reg) new_ssp, options(nostack));
    asm!("saveprevssp", options(nostack));
    printf(b"[INFO]\tssp is now %lx\n\0".as_ptr() as *const c_char, get_ssp());

    /* Switch back to original shadow stack */
    ssp = ssp.wrapping_sub(8);
    asm!("rstorssp ({0})", in(reg) ssp, options(nostack));
    asm!("saveprevssp", options(nostack));
}

unsafe fn test_shstk_pivot() -> c_int {
    let shstk = create_shstk(null_mut());
    if shstk == MAP_FAILED {
        printf(b"[FAIL]\tError creating shadow stack: %d\n\0".as_ptr() as *const c_char, errno);
        return 1;
    }
    try_shstk(shstk as c_ulong + SS_SIZE as c_ulong - 8);
    free_shstk(shstk);
    printf(b"[OK]\tShadow stack pivot\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn test_shstk_faults() -> c_int {
    let shstk = create_shstk(null_mut()) as *mut c_ulong;
    /* Read shadow stack, test if it's zero to not get read optimized out */
    if *shstk != 0 {
        return 1;
    }
    /* Wrss memory that was already read. */
    write_shstk(shstk, 1);
    if *shstk != 1 {
        return 1;
    }
    /* Page out memory, so we can wrss it again. */
    if reset_shstk(shstk as *mut c_void) != 0 {
        return 1;
    }
    write_shstk(shstk, 1);
    if *shstk != 1 {
        return 1;
    }
    printf(b"[OK]\tShadow stack faults\n\0".as_ptr() as *const c_char);
    0
}

static mut saved_ssp: c_ulong = 0;
static mut saved_ssp_val: c_ulong = 0;
static mut segv_triggered: bool_t = false;

#[inline(never)]
unsafe extern "C" fn violate_ss() {
    saved_ssp = get_ssp();
    saved_ssp_val = *(saved_ssp as *mut c_ulong);
    /* Corrupt shadow stack */
    printf(b"[INFO]\tCorrupting shadow stack\n\0".as_ptr() as *const c_char);
    write_shstk(saved_ssp as *mut c_ulong, 0);
}

unsafe extern "C" fn segv_handler(_signum: c_int, _si: *mut siginfo_t, _uc: *mut c_void) {
    printf(b"[INFO]\tGenerated shadow stack violation successfully\n\0".as_ptr() as *const c_char);
    segv_triggered = true;
    /* Fix shadow stack */
    write_shstk(saved_ssp as *mut c_ulong, saved_ssp_val);
}

unsafe fn test_shstk_violation() -> c_int {
    let mut sa: sigaction = zeroed();
    sa.sa.sa_sigaction = segv_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    segv_triggered = false;
    /* Make sure segv_triggered is set before violate_ss() */
    asm!("", options(nostack, preserves_flags));
    violate_ss();
    signal(SIGSEGV, SIG_DFL);
    printf(b"[OK]\tShadow stack violation test\n\0".as_ptr() as *const c_char);
    (!segv_triggered) as c_int
}

/* Gup test state */
const MAGIC_VAL: c_ulong = 0x12345678;
static mut is_shstk_access: bool_t = false;
static mut shstk_ptr: *mut c_void = null_mut();
static mut fd: c_int = 0;

unsafe fn reset_test_shstk(addr: *mut c_void) {
    if !shstk_ptr.is_null() {
        free_shstk(shstk_ptr);
    }
    shstk_ptr = create_shstk(addr);
}

unsafe extern "C" fn test_access_fix_handler(_signum: c_int, _si: *mut siginfo_t, _uc: *mut c_void) {
    printf(
        b"[INFO]\tViolation from %s\n\0".as_ptr() as *const c_char,
        if is_shstk_access { b"shstk access\0".as_ptr() } else { b"normal write\0".as_ptr() },
    );
    segv_triggered = true;
    /* Fix shadow stack */
    if is_shstk_access {
        reset_test_shstk(shstk_ptr);
        return;
    }
    free_shstk(shstk_ptr);
    create_normal_mem(shstk_ptr);
}

unsafe fn test_shstk_access(ptr: *mut c_void) -> bool_t {
    is_shstk_access = true;
    segv_triggered = false;
    write_shstk(ptr as *mut c_ulong, MAGIC_VAL);
    asm!("", options(nostack, preserves_flags));
    segv_triggered
}

unsafe fn test_write_access(ptr: *mut c_void) -> bool_t {
    is_shstk_access = false;
    segv_triggered = false;
    *(ptr as *mut c_ulong) = MAGIC_VAL;
    asm!("", options(nostack, preserves_flags));
    segv_triggered
}

unsafe fn gup_write(ptr: *mut c_void) -> bool_t {
    let val: c_ulong = 0;
    lseek(fd, ptr as c_long, SEEK_SET);
    if write(fd, &val as *const _ as *const c_void, size_of::<c_ulong>()) < 0 {
        return true;
    }
    false
}

unsafe fn gup_read(ptr: *mut c_void) -> bool_t {
    let mut val: c_ulong = 0;
    lseek(fd, ptr as c_long, SEEK_SET);
    if read(fd, &mut val as *mut _ as *mut c_void, size_of::<c_ulong>()) < 0 {
        return true;
    }
    false
}

unsafe fn test_gup() -> c_int {
    let mut sa: sigaction = zeroed();
    let mut status: c_int = 0;
    let pid: pid_t;
    sa.sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    segv_triggered = false;
    fd = open(b"/proc/self/mem\0".as_ptr() as *const c_char, O_RDWR);
    if fd == -1 {
        return 1;
    }
    reset_test_shstk(null_mut());
    if gup_read(shstk_ptr) || test_shstk_access(shstk_ptr) {
        return 1;
    }
    printf(b"[INFO]\tGup read -> shstk access success\n\0".as_ptr() as *const c_char);
    reset_test_shstk(null_mut());
    if gup_write(shstk_ptr) || test_shstk_access(shstk_ptr) {
        return 1;
    }
    printf(b"[INFO]\tGup write -> shstk access success\n\0".as_ptr() as *const c_char);
    reset_test_shstk(null_mut());
    if gup_read(shstk_ptr) || !test_write_access(shstk_ptr) {
        return 1;
    }
    printf(b"[INFO]\tGup read -> write access success\n\0".as_ptr() as *const c_char);
    reset_test_shstk(null_mut());
    if gup_write(shstk_ptr) || !test_write_access(shstk_ptr) {
        return 1;
    }
    printf(b"[INFO]\tGup write -> write access success\n\0".as_ptr() as *const c_char);
    close(fd);
    /* COW/gup test */
    reset_test_shstk(null_mut());
    pid = fork();
    if pid == 0 {
        fd = open(b"/proc/self/mem\0".as_ptr() as *const c_char, O_RDWR);
        if fd == -1 {
            exit(1);
        }
        if gup_write(shstk_ptr) {
            close(fd);
            exit(1);
        }
        close(fd);
        exit(0);
    }
    waitpid(pid, &mut status, 0);
    if WEXITSTATUS(status) != 0 {
        printf(b"[FAIL]\tWrite in child failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if *(shstk_ptr as *mut c_ulong) == MAGIC_VAL {
        printf(b"[FAIL]\tWrite in child wrote through to shared memory\n\0".as_ptr() as *const c_char);
        return 1;
    }
    printf(b"[INFO]\tCow gup write -> write access success\n\0".as_ptr() as *const c_char);
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    printf(b"[OK]\tShadow gup test\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn test_mprotect() -> c_int {
    let mut sa: sigaction = zeroed();
    sa.sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    segv_triggered = false;
    /* mprotect a shadow stack as read only */
    reset_test_shstk(null_mut());
    if mprotect(shstk_ptr, SS_SIZE, PROT_READ) < 0 {
        printf(b"[FAIL]\tmprotect(PROT_READ) failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    /* try to wrss it and fail */
    if !test_shstk_access(shstk_ptr) {
        printf(b"[FAIL]\tShadow stack access to read-only memory succeeded\n\0".as_ptr() as *const c_char);
        return 1;
    }
    /*
     * The shadow stack was reset above to resolve the fault, make the new one
     * read-only.
     */
    if mprotect(shstk_ptr, SS_SIZE, PROT_READ) < 0 {
        printf(b"[FAIL]\tmprotect(PROT_READ) failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    /* then back to writable */
    if mprotect(shstk_ptr, SS_SIZE, PROT_WRITE | PROT_READ) < 0 {
        printf(b"[FAIL]\tmprotect(PROT_WRITE) failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    /* then wrss to it and succeed */
    if test_shstk_access(shstk_ptr) {
        printf(b"[FAIL]\tShadow stack access to mprotect() writable memory failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    printf(b"[OK]\tmprotect() test\n\0".as_ptr() as *const c_char);
    0
}

static mut zero: [c_char; 4096] = [0; 4096];

unsafe extern "C" fn uffd_thread(arg: *mut c_void) -> *mut c_void {
    let mut req: uffdio_copy = zeroed();
    let uffd = *(arg as *mut c_int);
    let mut msg: uffd_msg = zeroed();
    let mut ret: c_int;
    loop {
        ret = read(uffd, &mut msg as *mut _ as *mut c_void, size_of::<uffd_msg>()) as c_int;
        if ret > 0 {
            break;
        } else if errno == EAGAIN {
            continue;
        }
        return 1usize as *mut c_void;
    }
    req.dst = msg.arg.pagefault.address;
    req.src = zero.as_ptr() as __u64;
    req.len = 4096;
    req.mode = 0;
    if ioctl(uffd, UFFDIO_COPY, &mut req) != 0 {
        return 1usize as *mut c_void;
    }
    null_mut()
}

unsafe fn test_userfaultfd() -> c_int {
    let mut uffdio_register: uffdio_register = zeroed();
    let mut uffdio_api: uffdio_api = zeroed();
    let mut sa: sigaction = zeroed();
    let mut thread: pthread_t = zeroed();
    let mut res: *mut c_void = null_mut();
    let uffd: c_int;
    sa.sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    uffd = syscall(__NR_userfaultfd, O_CLOEXEC | O_NONBLOCK) as c_int;
    if uffd < 0 {
        printf(b"[SKIP]\tUserfaultfd unavailable.\n\0".as_ptr() as *const c_char);
        return 0;
    }
    reset_test_shstk(null_mut());
    uffdio_api.api = UFFD_API;
    uffdio_api.features = 0;
    if ioctl(uffd, UFFDIO_API, &mut uffdio_api) != 0 {
        free_shstk(shstk_ptr);
        close(uffd);
        signal(SIGSEGV, SIG_DFL);
        return 1;
    }
    uffdio_register.range.start = shstk_ptr as __u64;
    uffdio_register.range.len = 4096;
    uffdio_register.mode = UFFDIO_REGISTER_MODE_MISSING;
    if ioctl(uffd, UFFDIO_REGISTER, &mut uffdio_register) != 0 {
        free_shstk(shstk_ptr);
        close(uffd);
        signal(SIGSEGV, SIG_DFL);
        return 1;
    }
    if pthread_create(&mut thread, null(), uffd_thread, &mut (uffd as c_int) as *mut _ as *mut c_void) != 0 {
        free_shstk(shstk_ptr);
        close(uffd);
        signal(SIGSEGV, SIG_DFL);
        return 1;
    }
    reset_shstk(shstk_ptr);
    test_shstk_access(shstk_ptr);
    if pthread_join(thread, &mut res) != 0 {
        free_shstk(shstk_ptr);
        close(uffd);
        signal(SIGSEGV, SIG_DFL);
        return 1;
    }
    if test_shstk_access(shstk_ptr) {
        free_shstk(shstk_ptr);
        close(uffd);
        signal(SIGSEGV, SIG_DFL);
        return 1;
    }
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    if res.is_null() {
        printf(b"[OK]\tUserfaultfd test\n\0".as_ptr() as *const c_char);
    }
    (!res.is_null()) as c_int
}

/* Simple linked list for keeping track of mappings in test_guard_gap() */
#[repr(C)]
struct node {
    next: *mut node,
    mapping: *mut c_void,
}

unsafe fn test_guard_gap_other_gaps() -> c_int {
    let free_area = mmap(null_mut(), SS_SIZE * 3, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    munmap(free_area, SS_SIZE * 3);
    let shstk = create_shstk((free_area as usize + SS_SIZE) as *mut c_void);
    let mut test_map = 0xFFFFFFFFFFFFFFFFusize as *mut c_void;
    let mut head: *mut node = null_mut();
    if shstk == MAP_FAILED {
        return 1;
    }
    while (test_map as usize) > (shstk as usize) {
        test_map = mmap(null_mut(), PAGE_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        if test_map == MAP_FAILED {
            return 1;
        }
        let cur = malloc(size_of::<node>()) as *mut node;
        (*cur).mapping = test_map;
        (*cur).next = head;
        head = cur;
    }
    while !head.is_null() {
        let cur = head;
        head = (*cur).next;
        munmap((*cur).mapping, PAGE_SIZE);
        free(cur as *mut c_void);
    }
    free_shstk(shstk);
    if (shstk as usize).wrapping_sub(test_map as usize).wrapping_sub(PAGE_SIZE) != PAGE_SIZE {
        return 1;
    }
    printf(b"[OK]\tGuard gap test, other mapping's gaps\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn test_guard_gap_new_mappings_gaps() -> c_int {
    let free_area = mmap(null_mut(), PAGE_SIZE * 4, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    munmap(free_area, PAGE_SIZE * 4);
    let mut test_map = 0xFFFFFFFFFFFFFFFFusize as *mut c_void;
    let mut head: *mut node = null_mut();
    let mut ret: c_int = 0;
    /* Test letting map_shadow_stack find a free space */
    let shstk_start = mmap(free_area, PAGE_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if shstk_start == MAP_FAILED || shstk_start != free_area {
        return 1;
    }
    while (test_map as usize) > (shstk_start as usize) {
        test_map = syscall(__NR_map_shadow_stack, 0, PAGE_SIZE, 0) as *mut c_void;
        if test_map == MAP_FAILED {
            printf(b"[INFO]\tmap_shadow_stack MAP_FAILED\n\0".as_ptr() as *const c_char);
            ret = 1;
            break;
        }
        let cur = malloc(size_of::<node>()) as *mut node;
        (*cur).mapping = test_map;
        (*cur).next = head;
        head = cur;
        if test_map == (free_area as usize + PAGE_SIZE) as *mut c_void {
            printf(b"[INFO]\tNew mapping has other mapping in guard gap!\n\0".as_ptr() as *const c_char);
            ret = 1;
            break;
        }
    }
    while !head.is_null() {
        let cur = head;
        head = (*cur).next;
        munmap((*cur).mapping, PAGE_SIZE);
        free(cur as *mut c_void);
    }
    munmap(shstk_start, PAGE_SIZE);
    if ret == 0 {
        printf(b"[OK]\tGuard gap test, placement mapping's gaps\n\0".as_ptr() as *const c_char);
    }
    ret
}

/*
 * Too complicated to pull it out of the 32 bit header, but also get the
 * 64 bit one needed above. Just define a copy here.
 */
const __NR_compat_sigaction: c_long = 67;

unsafe fn sigaction32(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int {
    let ret: c_int;
    asm!(
        "int 0x80",
        inlateout("eax") __NR_compat_sigaction as c_int => ret,
        in("ebx") signum,
        in("ecx") act as c_long,
        in("edx") oldact as c_long,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
    );
    ret
}

static mut jmp_buffer: sigjmp_buf = [0; 32];

unsafe extern "C" fn segv_gp_handler(_signum: c_int, _si: *mut siginfo_t, _uc: *mut c_void) {
    segv_triggered = true;
    /*
     * To work with old glibc, this can't rely on siglongjmp working with
     * shadow stack enabled, so disable shadow stack before siglongjmp().
     */
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    siglongjmp(&mut jmp_buffer, -1);
}

unsafe fn test_32bit() -> c_int {
    let mut sa: sigaction = zeroed();
    /* Create sigaction in 32 bit address range */
    let sa32 = mmap(null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_32BIT | MAP_PRIVATE | MAP_ANONYMOUS, 0, 0) as *mut sigaction;
    (*sa32).sa_flags = SA_SIGINFO;
    sa.sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    segv_triggered = false;
    /* Make sure segv_triggered is set before triggering the #GP */
    asm!("", options(nostack, preserves_flags));
    /*
     * Set handler to somewhere in 32 bit address space
     */
    (*sa32).sa.sa_handler = sa32 as usize;
    if sigaction32(SIGUSR1, sa32, null_mut()) != 0 {
        return 1;
    }
    if sigsetjmp(&mut jmp_buffer, 1) == 0 {
        raise(SIGUSR1);
    }
    if segv_triggered {
        printf(b"[OK]\t32 bit test\n\0".as_ptr() as *const c_char);
    }
    (!segv_triggered) as c_int
}

unsafe fn parse_uint_from_file(file: *const c_char, fmt: *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let f = fopen(file, b"re\0".as_ptr() as *const c_char);
    if f.is_null() {
        let err = -errno;
        printf(b"failed to open '%s': %d\n\0".as_ptr() as *const c_char, file, err);
        return err;
    }
    let mut err = fscanf(f, fmt, &mut ret);
    if err != 1 {
        err = if err == EOF_ { -EIO } else { -errno };
        printf(b"failed to parse '%s': %d\n\0".as_ptr() as *const c_char, file, err);
        fclose(f);
        return err;
    }
    fclose(f);
    ret
}

unsafe fn determine_uprobe_perf_type() -> c_int {
    parse_uint_from_file(
        b"/sys/bus/event_source/devices/uprobe/type\0".as_ptr() as *const c_char,
        b"%d\n\0".as_ptr() as *const c_char,
    )
}

unsafe fn determine_uprobe_retprobe_bit() -> c_int {
    parse_uint_from_file(
        b"/sys/bus/event_source/devices/uprobe/format/retprobe\0".as_ptr() as *const c_char,
        b"config:%d\n\0".as_ptr() as *const c_char,
    )
}

unsafe fn get_uprobe_offset(addr: *const c_void) -> ssize_t {
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut base: size_t = 0;
    let mut buf = [0 as c_char; 256];
    let mut found = false;
    let f = fopen(b"/proc/self/maps\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -errno as ssize_t;
    }
    while fscanf(
        f,
        b"%zx-%zx %s %zx %*[^\n]\n\0".as_ptr() as *const c_char,
        &mut start,
        &mut end,
        buf.as_mut_ptr(),
        &mut base,
    ) == 4
    {
        if buf[2] == b'x' as c_char && addr as uintptr_t >= start && (addr as uintptr_t) < end {
            found = true;
            break;
        }
    }
    fclose(f);
    if !found {
        return -ESRCH as ssize_t;
    }
    (addr as uintptr_t).wrapping_sub(start).wrapping_add(base) as ssize_t
}

#[inline(never)]
unsafe fn uretprobe_trigger() {
    asm!("", options(nostack, preserves_flags));
}

unsafe fn test_uretprobe() -> c_int {
    let attr_sz = size_of::<perf_event_attr>();
    let file = b"/proc/self/exe\0".as_ptr() as *const c_char;
    let mut fd: c_int = 0;
    let mut err: c_int = 1;
    let mut attr: perf_event_attr = zeroed();
    let mut sa: sigaction = zeroed();
    let type_ = determine_uprobe_perf_type();
    if type_ < 0 {
        if type_ == -ENOENT {
            printf(b"[SKIP]\tUretprobe test, uprobes are not available\n\0".as_ptr() as *const c_char);
        }
        return 0;
    }
    let offset = get_uprobe_offset(uretprobe_trigger as *const c_void);
    if offset < 0 {
        return 1;
    }
    let bit = determine_uprobe_retprobe_bit();
    if bit < 0 {
        return 1;
    }
    sa.sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    /* Setup return uprobe through perf event interface. */
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.size = attr_sz as u32;
    attr.type_ = type_ as u32;
    attr.config = 1u64 << bit;
    attr.config1 = file as __u64;
    attr.config2 = offset as u64;
    fd = syscall(__NR_perf_event_open, &mut attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC) as c_int;
    if fd >= 0 {
        if sigsetjmp(&mut jmp_buffer, 1) == 0 {
            ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK);
            /*
             * This either segfaults and goes through sigsetjmp above
             * or succeeds and we're good.
             */
            uretprobe_trigger();
            printf(b"[OK]\tUretprobe test\n\0".as_ptr() as *const c_char);
            err = 0;
        }
    }
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    signal(SIGSEGV, SIG_DFL);
    if fd != 0 {
        close(fd);
    }
    err
}

/* Keep the CALL first so the function address is exactly the probed CALL. */
unsafe extern "C" {
    fn uprobe_call_trigger();
}

global_asm!(
    ".pushsection .text",
    ".global uprobe_call_target",
    ".type uprobe_call_target, @function",
    "uprobe_call_target:",
    "	ret",
    ".size uprobe_call_target, .-uprobe_call_target",
    ".global uprobe_call_trigger",
    ".type uprobe_call_trigger, @function",
    "uprobe_call_trigger:",
    "	call uprobe_call_target",
    "	ret",
    ".size uprobe_call_trigger, .-uprobe_call_trigger",
    ".popsection",
);

unsafe fn test_uprobe_call() -> c_int {
    let attr_sz = size_of::<perf_event_attr>();
    let file = b"/proc/self/exe\0".as_ptr() as *const c_char;
    let mut fd: c_int = -1;
    let mut err: c_int = 1;
    let mut attr: perf_event_attr = zeroed();
    let mut sa: sigaction = zeroed();
    let type_ = determine_uprobe_perf_type();
    if type_ < 0 {
        if type_ == -ENOENT {
            printf(b"[SKIP]\tUprobe on CALL test, uprobes are not available\n\0".as_ptr() as *const c_char);
        }
        return 0;
    }
    let offset = get_uprobe_offset(uprobe_call_trigger as *const c_void);
    if offset < 0 {
        return 1;
    }
    sa.sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
        return 1;
    }
    /* Setup entry uprobe through perf event interface. */
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.size = attr_sz as u32;
    attr.type_ = type_ as u32;
    attr.config = 0;
    attr.config1 = file as __u64;
    attr.config2 = offset as u64;
    fd = syscall(__NR_perf_event_open, &mut attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC) as c_int;
    if fd >= 0 {
        if sigsetjmp(&mut jmp_buffer, 1) == 0 {
            if ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK) == 0 {
                /*
                 * This either segfaults and goes through sigsetjmp above
                 * or succeeds and we're good.
                 */
                uprobe_call_trigger();
                printf(b"[OK]\tUprobe on CALL test\n\0".as_ptr() as *const c_char);
                err = 0;
            }
        }
    }
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    signal(SIGSEGV, SIG_DFL);
    if fd >= 0 {
        close(fd);
    }
    err
}

unsafe extern "C" fn segv_handler_ptrace(_signum: c_int, _si: *mut siginfo_t, _uc: *mut c_void) {
    /* The SSP adjustment caused a segfault. */
    exit(0);
}

unsafe fn test_ptrace() -> c_int {
    let mut saved_ssp_local: c_ulong;
    let mut ssp: c_ulong = 0;
    let mut sa: sigaction = zeroed();
    let mut iov = iovec { iov_base: &mut ssp as *mut _ as *mut c_void, iov_len: size_of::<c_ulong>() };
    let mut status: c_int = 0;
    let pid = fork();
    if pid == 0 {
        ssp = get_ssp();
        sa.sa.sa_sigaction = segv_handler_ptrace;
        sa.sa_flags = SA_SIGINFO;
        if sigaction(SIGSEGV, &sa, null_mut()) != 0 {
            return 1;
        }
        ptrace(PTRACE_TRACEME, null::<c_void>(), null::<c_void>(), null::<c_void>());
        /*
         * The parent will tweak the SSP and return from this function
         * will #CP.
         */
        raise(SIGTRAP);
        exit(1);
    }
    while waitpid(pid, &mut status, 0) != -1 && WSTOPSIG(status) != SIGTRAP {}
    if ptrace(PTRACE_GETREGSET, pid, NT_X86_SHSTK, &mut iov) != 0 {
        printf(b"[INFO]\tFailed to PTRACE_GETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    if ssp == 0 {
        printf(b"[INFO]\tPtrace child SSP was 0\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    saved_ssp_local = ssp;
    iov.iov_len = 0;
    if ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &mut iov) == 0 {
        printf(b"[INFO]\tToo small size accepted via PTRACE_SETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    iov.iov_len = size_of::<c_ulong>() + 1;
    if ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &mut iov) == 0 {
        printf(b"[INFO]\tToo large size accepted via PTRACE_SETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    ssp = ssp.wrapping_add(1);
    if ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &mut iov) == 0 {
        printf(b"[INFO]\tUnaligned SSP written via PTRACE_SETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    ssp = 0xFFFFFFFFFFFF0000;
    if ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &mut iov) == 0 {
        printf(b"[INFO]\tKernel range SSP written via PTRACE_SETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    /*
     * Tweak the SSP so the child with #CP when it resumes and returns
     * from raise()
     */
    ssp = saved_ssp_local.wrapping_add(8);
    iov.iov_len = size_of::<c_ulong>();
    if ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &mut iov) != 0 {
        printf(b"[INFO]\tFailed to PTRACE_SETREGS\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    if ptrace(PTRACE_DETACH, pid, null::<c_void>(), null::<c_void>()) != 0 {
        printf(b"[INFO]\tFailed to PTRACE_DETACH\n\0".as_ptr() as *const c_char);
        kill(pid, SIGKILL);
        return 1;
    }
    waitpid(pid, &mut status, 0);
    if WEXITSTATUS(status) != 0 {
        return 1;
    }
    printf(b"[OK]\tPtrace test\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn real_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    if ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK) != 0 {
        printf(b"[SKIP]\tCould not enable Shadow stack\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK) != 0 {
        ret = 1;
        printf(b"[FAIL]\tDisabling shadow stack failed\n\0".as_ptr() as *const c_char);
    }
    if ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK) != 0 {
        printf(b"[SKIP]\tCould not re-enable Shadow stack\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_WRSS) != 0 {
        printf(b"[SKIP]\tCould not enable WRSS\n\0".as_ptr() as *const c_char);
        ret = 1;
    } else if get_ssp() == 0 {
        /* Should have succeeded if here, but this is a test, so double check. */
        printf(b"[FAIL]\tShadow stack disabled\n\0".as_ptr() as *const c_char);
        return 1;
    } else if test_shstk_pivot() != 0 {
        ret = 1;
        printf(b"[FAIL]\tShadow stack pivot\n\0".as_ptr() as *const c_char);
    } else if test_shstk_faults() != 0 {
        ret = 1;
        printf(b"[FAIL]\tShadow stack fault test\n\0".as_ptr() as *const c_char);
    } else if test_shstk_violation() != 0 {
        ret = 1;
        printf(b"[FAIL]\tShadow stack violation test\n\0".as_ptr() as *const c_char);
    } else if test_gup() != 0 {
        ret = 1;
        printf(b"[FAIL]\tShadow shadow stack gup\n\0".as_ptr() as *const c_char);
    } else if test_mprotect() != 0 {
        ret = 1;
        printf(b"[FAIL]\tShadow shadow mprotect test\n\0".as_ptr() as *const c_char);
    } else if test_userfaultfd() != 0 {
        ret = 1;
        printf(b"[FAIL]\tUserfaultfd test\n\0".as_ptr() as *const c_char);
    } else if test_guard_gap_other_gaps() != 0 {
        ret = 1;
        printf(b"[FAIL]\tGuard gap test, other mappings' gaps\n\0".as_ptr() as *const c_char);
    } else if test_guard_gap_new_mappings_gaps() != 0 {
        ret = 1;
        printf(b"[FAIL]\tGuard gap test, placement mapping's gaps\n\0".as_ptr() as *const c_char);
    } else if test_ptrace() != 0 {
        ret = 1;
        printf(b"[FAIL]\tptrace test\n\0".as_ptr() as *const c_char);
    } else if test_32bit() != 0 {
        ret = 1;
        printf(b"[FAIL]\t32 bit test\n\0".as_ptr() as *const c_char);
    } else if test_uretprobe() != 0 {
        ret = 1;
        printf(b"[FAIL]\turetprobe test\n\0".as_ptr() as *const c_char);
    } else if test_uprobe_call() != 0 {
        ret = 1;
        printf(b"[FAIL]\tuprobe on CALL test\n\0".as_ptr() as *const c_char);
    } else {
        return ret;
    }

    /*
     * Disable shadow stack before the function returns, or there will be a
     * shadow stack violation.
     */
    if ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK) != 0 {
        ret = 1;
        printf(b"[FAIL]\tDisabling shadow stack failed\n\0".as_ptr() as *const c_char);
    }
    ret
}

fn main() {
    unsafe {
        let code = real_main(0, null_mut());
        if code != 0 {
            exit(code);
        }
    }
}

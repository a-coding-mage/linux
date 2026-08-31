/*
 * Copyright (c) 2019 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Fork and exec tiny 1 page executable which precisely controls its VM.
 * Test /proc/$PID/maps
 * Test /proc/$PID/smaps
 * Test /proc/$PID/smaps_rollup
 * Test /proc/$PID/statm
 *
 * FIXME require CONFIG_TMPFS which can be disabled
 * FIXME test other values from "smaps"
 * FIXME support other archs
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type __u64 = u64;

const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const SIGTERM: c_int = 15;
const SIGSEGV: c_int = 11;
const SA_SIGINFO: c_int = 4;
const RLIMIT_CORE: c_int = 4;
const CLONE_NEWNS: c_int = 0x0002_0000;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_EXCL: c_int = 0o200;
const O_DIRECTORY: c_int = 0o200000;
const O_CLOEXEC: c_int = 0o2000000;
const O_PATH: c_int = 0o10000000;
const O_TMPFILE: c_int = 0o20000000 | O_DIRECTORY;
const AT_FDCWD: c_int = -100;
const AT_EMPTY_PATH: c_int = 0x1000;
const SYS_execveat: c_long = 322;
const PROCMAP_QUERY: c_ulong = 0xc068_6611;
const PROCMAP_QUERY_VMA_READABLE: __u64 = 0x01;
const PROCMAP_QUERY_VMA_WRITABLE: __u64 = 0x02;
const PROCMAP_QUERY_VMA_EXECUTABLE: __u64 = 0x04;
const PROCMAP_QUERY_COVERING_OR_NEXT_VMA: __u64 = 0x10;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn abort() -> !;
    fn atexit(function: extern "C" fn()) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memmem(
        haystack: *const c_void,
        haystacklen: size_t,
        needle: *const c_void,
        needlelen: size_t,
    ) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn syscall(number: c_long, ...) -> c_long;
    fn unshare(flags: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;

    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct rlimit {
    rlim_cur: c_ulong,
    rlim_max: c_ulong,
}

#[repr(C)]
struct sigaction {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 128],
}

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: c_long,
    st_atime_nsec: c_long,
    st_mtime: c_long,
    st_mtime_nsec: c_long,
    st_ctime: c_long,
    st_ctime_nsec: c_long,
    __glibc_reserved: [c_long; 3],
}

#[repr(C)]
struct procmap_query {
    size: __u64,
    query_flags: __u64,
    query_addr: __u64,
    vma_start: __u64,
    vma_end: __u64,
    vma_flags: __u64,
    vma_page_size: __u64,
    vma_offset: __u64,
    inode: __u64,
    dev_major: __u64,
    dev_minor: __u64,
    vma_name_addr: __u64,
    vma_name_size: __u64,
    build_id_addr: __u64,
    build_id_size: __u64,
    reserved: [__u64; 4],
}

#[repr(C)]
struct elf64_hdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct elf64_phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn MAJOR(dev: c_ulong) -> c_ulong {
    ((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff)
}

fn MINOR(dev: c_ulong) -> c_ulong {
    (dev & 0xff) | ((dev >> 12) & !0xff)
}

unsafe fn sys_execveat(
    dirfd: c_int,
    pathname: *const c_char,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
    flags: c_int,
) -> c_long {
    syscall(SYS_execveat, dirfd, pathname, argv, envp, flags)
}

unsafe fn make_private_tmp() {
    if unshare(CLONE_NEWNS) == -1 {
        if errno() == ENOSYS || errno() == EPERM {
            exit(4);
        }
        exit(1);
    }
    if mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_PRIVATE | MS_REC, ptr::null()) == -1 {
        exit(1);
    }
    if mount(ptr::null(), c"/tmp".as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()) == -1 {
        exit(1);
    }
}

static mut pid: pid_t = -1;

extern "C" fn ate() {
    unsafe {
        if pid > 0 {
            kill(pid, SIGTERM);
        }
    }
}

#[cfg(target_arch = "x86_64")]
const PAGE_SIZE: u64 = 4096;
#[cfg(target_arch = "x86_64")]
const VADDR: u64 = 1u64 << 32;
#[cfg(target_arch = "x86_64")]
const MAPS_OFFSET: usize = 73;

#[cfg(target_arch = "x86_64")]
fn mov_rdi(x: u64) -> [u8; 10] {
    [
        0x48,
        0xbf,
        (x & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 56) & 0xff) as u8,
    ]
}

#[cfg(target_arch = "x86_64")]
fn mov_rsi(x: u64) -> [u8; 10] {
    [
        0x48,
        0xbe,
        (x & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 56) & 0xff) as u8,
    ]
}

#[cfg(target_arch = "x86_64")]
fn mov_eax(x: u32) -> [u8; 5] {
    [
        0xb8,
        (x & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
    ]
}

#[cfg(target_arch = "x86_64")]
fn payload() -> Vec<u8> {
    let mut payload = Vec::new();
    /* Casually unmap stack, vDSO and everything else. */
    /* munmap */
    payload.extend_from_slice(&mov_rdi(VADDR + 4096));
    payload.extend_from_slice(&mov_rsi((1u64 << 47) - 4096 - VADDR - 4096));
    payload.extend_from_slice(&mov_eax(11));
    payload.extend_from_slice(&[0x0f, 0x05]);

    /* Ping parent. */
    /* write(0, &c, 1); */
    payload.extend_from_slice(&[
        0x31, 0xff, /* xor edi, edi */
        0x48, 0x8d, 0x35, 0x00, 0x00, 0x00, 0x00, /* lea rsi, [rip] */
        0xba, 0x01, 0x00, 0x00, 0x00, /* mov edx, 1 */
    ]);
    payload.extend_from_slice(&mov_eax(1));
    payload.extend_from_slice(&[0x0f, 0x05]);

    /* 1: pause(); */
    payload.extend_from_slice(&mov_eax(34));
    payload.extend_from_slice(&[0x0f, 0x05]);

    payload.extend_from_slice(&[0xeb, 0xf7]); /* jmp 1b */
    payload
}

#[cfg(target_arch = "x86_64")]
unsafe fn make_exe(payload: *const u8, len: size_t) -> c_int {
    let mut h: elf64_hdr = zeroed();
    let mut ph: elf64_phdr = zeroed();

    let mut iov = [
        iovec {
            iov_base: &mut h as *mut _ as *mut c_void,
            iov_len: size_of::<elf64_hdr>(),
        },
        iovec {
            iov_base: &mut ph as *mut _ as *mut c_void,
            iov_len: size_of::<elf64_phdr>(),
        },
        iovec {
            iov_base: payload as *mut c_void,
            iov_len: len,
        },
    ];
    let mut fd: c_int;
    let mut fd1: c_int;
    let mut buf = [0 as c_char; 64];

    memset(&mut h as *mut _ as *mut c_void, 0, size_of::<elf64_hdr>());
    h.e_ident[0] = 0x7f;
    h.e_ident[1] = b'E';
    h.e_ident[2] = b'L';
    h.e_ident[3] = b'F';
    h.e_ident[4] = 2;
    h.e_ident[5] = 1;
    h.e_ident[6] = 1;
    h.e_ident[7] = 0;
    h.e_type = 2;
    h.e_machine = 0x3e;
    h.e_version = 1;
    h.e_entry = VADDR + size_of::<elf64_hdr>() as u64 + size_of::<elf64_phdr>() as u64;
    h.e_phoff = size_of::<elf64_hdr>() as u64;
    h.e_shoff = 0;
    h.e_flags = 0;
    h.e_ehsize = size_of::<elf64_hdr>() as u16;
    h.e_phentsize = size_of::<elf64_phdr>() as u16;
    h.e_phnum = 1;
    h.e_shentsize = 0;
    h.e_shnum = 0;
    h.e_shstrndx = 0;

    memset(&mut ph as *mut _ as *mut c_void, 0, size_of::<elf64_phdr>());
    ph.p_type = 1;
    ph.p_flags = (1 << 2) | 1;
    ph.p_offset = 0;
    ph.p_vaddr = VADDR;
    ph.p_paddr = 0;
    ph.p_filesz = (size_of::<elf64_hdr>() + size_of::<elf64_phdr>() + len) as u64;
    ph.p_memsz = (size_of::<elf64_hdr>() + size_of::<elf64_phdr>() + len) as u64;
    ph.p_align = 4096;

    fd = openat(AT_FDCWD, c"/tmp".as_ptr(), O_WRONLY | O_EXCL | O_TMPFILE, 0o700);
    if fd == -1 {
        exit(1);
    }

    if writev(fd, iov.as_ptr(), 3) != (size_of::<elf64_hdr>() + size_of::<elf64_phdr>() + len) as ssize_t {
        exit(1);
    }

    /* Avoid ETXTBSY on exec. */
    snprintf(buf.as_mut_ptr(), buf.len(), c"/proc/self/fd/%u".as_ptr(), fd);
    fd1 = open(buf.as_ptr(), O_RDONLY | O_CLOEXEC);
    close(fd);

    fd1
}

/*
 * 0: vsyscall VMA doesn't exist	vsyscall=none
 * 1: vsyscall VMA is --xp		vsyscall=xonly
 * 2: vsyscall VMA is r-xp		vsyscall=emulate
 */
static mut g_vsyscall: c_int = 0;
static mut str_vsyscall: *const c_char = ptr::null();

static str_vsyscall_0: &[u8] = b"\0";
static str_vsyscall_1: &[u8] =
    b"ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n\0";
static str_vsyscall_2: &[u8] =
    b"ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n\0";

#[cfg(target_arch = "x86_64")]
extern "C" fn sigaction_SIGSEGV(_: c_int, __: *mut siginfo_t, ___: *mut c_void) {
    unsafe {
        _exit(g_vsyscall);
    }
}

/*
 * vsyscall page can't be unmapped, probe it directly.
 */
#[cfg(target_arch = "x86_64")]
unsafe fn vsyscall() {
    let mut child_pid: pid_t;
    let mut wstatus: c_int = 0;

    child_pid = fork();
    if child_pid < 0 {
        fprintf(stderr, c"fork, errno %d\n".as_ptr(), errno());
        exit(1);
    }
    if child_pid == 0 {
        let rlim = rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        let _ = setrlimit(RLIMIT_CORE, &rlim);

        /* Hide "segfault at ffffffffff600000" messages. */
        let mut act: sigaction = zeroed();
        memset(&mut act as *mut _ as *mut c_void, 0, size_of::<sigaction>());
        act.sa_flags = SA_SIGINFO;
        act.sa_sigaction = sigaction_SIGSEGV;
        let _ = sigaction(SIGSEGV, &act, ptr::null_mut());

        g_vsyscall = 0;
        /* gettimeofday(NULL, NULL); */
        let mut rax: u64 = 0xffffffffff600000;
        asm!(
            "call *{rax}",
            rax = inout(reg) rax,
            in("rdi") ptr::null::<c_void>(),
            in("rsi") ptr::null::<c_void>(),
            lateout("rcx") _,
            lateout("r11") _,
        );

        g_vsyscall = 1;
        ptr::read_volatile(0xffffffffff600000u64 as *const c_int);

        g_vsyscall = 2;
        exit(g_vsyscall);
    }
    waitpid(child_pid, &mut wstatus, 0);
    if WIFEXITED(wstatus) {
        g_vsyscall = WEXITSTATUS(wstatus);
    } else {
        fprintf(stderr, c"error: wstatus %08x\n".as_ptr(), wstatus);
        exit(1);
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn c_assert(cond: bool) {
    assert!(cond);
}

#[cfg(target_arch = "x86_64")]
unsafe fn main_x86_64() -> c_int {
    let mut pipefd = [0 as c_int; 2];
    let mut exec_fd: c_int;

    vsyscall();
    match g_vsyscall {
        0 => str_vsyscall = str_vsyscall_0.as_ptr() as *const c_char,
        1 => str_vsyscall = str_vsyscall_1.as_ptr() as *const c_char,
        2 => str_vsyscall = str_vsyscall_2.as_ptr() as *const c_char,
        _ => abort(),
    }

    atexit(ate);

    make_private_tmp();

    /* Reserve fd 0 for 1-byte pipe ping from child. */
    close(0);
    if open(c"/".as_ptr(), O_RDONLY | O_DIRECTORY | O_PATH) != 0 {
        return 1;
    }

    let payload = payload();
    exec_fd = make_exe(payload.as_ptr(), payload.len());

    if pipe(pipefd.as_mut_ptr()) == -1 {
        return 1;
    }
    if dup2(pipefd[1], 0) != 0 {
        return 1;
    }

    pid = fork();
    if pid == -1 {
        return 1;
    }
    if pid == 0 {
        sys_execveat(exec_fd, c"".as_ptr(), ptr::null_mut(), ptr::null_mut(), AT_EMPTY_PATH);
        return 1;
    }

    let mut ch: c_char = 0;
    if read(pipefd[0], &mut ch as *mut _ as *mut c_void, 1) != 1 {
        return 1;
    }

    let mut st: stat = zeroed();
    if fstat(exec_fd, &mut st) == -1 {
        return 1;
    }

    /* Generate "head -n1 /proc/$PID/maps" */
    let mut buf0 = [0 as c_char; 256];
    memset(buf0.as_mut_ptr() as *mut c_void, b' ' as c_int, buf0.len());
    let mut len = snprintf(
        buf0.as_mut_ptr(),
        buf0.len(),
        c"%08lx-%08lx r-xp 00000000 %02lx:%02lx %llu".as_ptr(),
        VADDR as c_ulong,
        (VADDR + PAGE_SIZE) as c_ulong,
        MAJOR(st.st_dev),
        MINOR(st.st_dev),
        st.st_ino as c_ulong,
    );
    buf0[len as usize] = b' ' as c_char;
    snprintf(
        buf0.as_mut_ptr().add(MAPS_OFFSET),
        buf0.len() - MAPS_OFFSET,
        c"/tmp/#%llu (deleted)\n".as_ptr(),
        st.st_ino as c_ulong,
    );

    /* Test /proc/$PID/maps */
    {
        let expected_len = strlen(buf0.as_ptr()) + strlen(str_vsyscall);
        let mut buf = [0 as c_char; 256];
        let mut rv: ssize_t;
        let mut fd: c_int;

        snprintf(buf.as_mut_ptr(), buf.len(), c"/proc/%u/maps".as_ptr(), pid as c_uint);
        fd = open(buf.as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }
        rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        c_assert(rv == expected_len as ssize_t);
        c_assert(memcmp(buf.as_ptr() as *const c_void, buf0.as_ptr() as *const c_void, strlen(buf0.as_ptr())) == 0);
        if g_vsyscall > 0 {
            c_assert(
                memcmp(
                    buf.as_ptr().add(strlen(buf0.as_ptr())) as *const c_void,
                    str_vsyscall as *const c_void,
                    strlen(str_vsyscall),
                ) == 0,
            );
        }
    }

    const RSS1: &[u8] = b"Rss:                   4 kB\n\0";
    const RSS2: &[u8] = b"Rss:                   0 kB\n\0";
    const PSS1: &[u8] = b"Pss:                   4 kB\n\0";
    const PSS2: &[u8] = b"Pss:                   0 kB\n\0";

    /* Test /proc/$PID/smaps */
    {
        let mut buf = [0 as c_char; 4096];
        let mut rv: ssize_t;
        let mut fd: c_int;

        snprintf(buf.as_mut_ptr(), buf.len(), c"/proc/%u/smaps".as_ptr(), pid as c_uint);
        fd = open(buf.as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }
        rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        c_assert(0 <= rv && rv <= buf.len() as ssize_t);

        c_assert(rv >= strlen(buf0.as_ptr()) as ssize_t);
        c_assert(memcmp(buf.as_ptr() as *const c_void, buf0.as_ptr() as *const c_void, strlen(buf0.as_ptr())) == 0);

        c_assert(
            !memmem(buf.as_ptr() as *const c_void, rv as size_t, RSS1.as_ptr() as *const c_void, strlen(RSS1.as_ptr() as *const c_char)).is_null()
                || !memmem(buf.as_ptr() as *const c_void, rv as size_t, RSS2.as_ptr() as *const c_void, strlen(RSS2.as_ptr() as *const c_char)).is_null(),
        );
        c_assert(
            !memmem(buf.as_ptr() as *const c_void, rv as size_t, PSS1.as_ptr() as *const c_void, strlen(PSS1.as_ptr() as *const c_char)).is_null()
                || !memmem(buf.as_ptr() as *const c_void, rv as size_t, PSS2.as_ptr() as *const c_void, strlen(PSS2.as_ptr() as *const c_char)).is_null(),
        );

        let S: [*const c_char; 8] = [
            c"Size:                  4 kB\n".as_ptr(),
            c"KernelPageSize:        4 kB\n".as_ptr(),
            c"MMUPageSize:           4 kB\n".as_ptr(),
            c"Anonymous:             0 kB\n".as_ptr(),
            c"AnonHugePages:         0 kB\n".as_ptr(),
            c"Shared_Hugetlb:        0 kB\n".as_ptr(),
            c"Private_Hugetlb:       0 kB\n".as_ptr(),
            c"Locked:                0 kB\n".as_ptr(),
        ];

        for i in 0..S.len() {
            c_assert(!memmem(buf.as_ptr() as *const c_void, rv as size_t, S[i] as *const c_void, strlen(S[i])).is_null());
        }

        if g_vsyscall > 0 {
            c_assert(!memmem(buf.as_ptr() as *const c_void, rv as size_t, str_vsyscall as *const c_void, strlen(str_vsyscall)).is_null());
        }
    }

    /* Test /proc/$PID/smaps_rollup */
    {
        let mut bufr = [0 as c_char; 256];
        memset(bufr.as_mut_ptr() as *mut c_void, b' ' as c_int, bufr.len());
        len = snprintf(
            bufr.as_mut_ptr(),
            bufr.len(),
            c"%08lx-%08lx ---p 00000000 00:00 0".as_ptr(),
            VADDR as c_ulong,
            (VADDR + PAGE_SIZE) as c_ulong,
        );
        bufr[len as usize] = b' ' as c_char;
        snprintf(
            bufr.as_mut_ptr().add(MAPS_OFFSET),
            bufr.len() - MAPS_OFFSET,
            c"[rollup]\n".as_ptr(),
        );

        let mut buf = [0 as c_char; 1024];
        let mut rv: ssize_t;
        let mut fd: c_int;

        snprintf(buf.as_mut_ptr(), buf.len(), c"/proc/%u/smaps_rollup".as_ptr(), pid as c_uint);
        fd = open(buf.as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }
        rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        c_assert(0 <= rv && rv <= buf.len() as ssize_t);

        c_assert(rv >= strlen(bufr.as_ptr()) as ssize_t);
        c_assert(memcmp(buf.as_ptr() as *const c_void, bufr.as_ptr() as *const c_void, strlen(bufr.as_ptr())) == 0);

        c_assert(
            !memmem(buf.as_ptr() as *const c_void, rv as size_t, RSS1.as_ptr() as *const c_void, strlen(RSS1.as_ptr() as *const c_char)).is_null()
                || !memmem(buf.as_ptr() as *const c_void, rv as size_t, RSS2.as_ptr() as *const c_void, strlen(RSS2.as_ptr() as *const c_char)).is_null(),
        );
        c_assert(
            !memmem(buf.as_ptr() as *const c_void, rv as size_t, PSS1.as_ptr() as *const c_void, strlen(PSS1.as_ptr() as *const c_char)).is_null()
                || !memmem(buf.as_ptr() as *const c_void, rv as size_t, PSS2.as_ptr() as *const c_void, strlen(PSS2.as_ptr() as *const c_char)).is_null(),
        );

        let S: [*const c_char; 6] = [
            c"Anonymous:             0 kB\n".as_ptr(),
            c"AnonHugePages:         0 kB\n".as_ptr(),
            c"Shared_Hugetlb:        0 kB\n".as_ptr(),
            c"Private_Hugetlb:       0 kB\n".as_ptr(),
            c"Locked:                0 kB\n".as_ptr(),
        ];

        for i in 0..S.len() {
            c_assert(!memmem(buf.as_ptr() as *const c_void, rv as size_t, S[i] as *const c_void, strlen(S[i])).is_null());
        }
    }

    /* Test /proc/$PID/statm */
    {
        let mut buf = [0 as c_char; 64];
        let mut rv: ssize_t;
        let mut fd: c_int;

        snprintf(buf.as_mut_ptr(), buf.len(), c"/proc/%u/statm".as_ptr(), pid as c_uint);
        fd = open(buf.as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }
        rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        c_assert(rv == 7 * 2);

        c_assert(buf[0] == b'1' as c_char); /* ->total_vm */
        c_assert(buf[1] == b' ' as c_char);
        c_assert(buf[2] == b'0' as c_char || buf[2] == b'1' as c_char); /* rss */
        c_assert(buf[3] == b' ' as c_char);
        c_assert(buf[4] == b'0' as c_char || buf[2] == b'1' as c_char); /* file rss */
        c_assert(buf[5] == b' ' as c_char);
        c_assert(buf[6] == b'1' as c_char); /* ELF executable segments */
        c_assert(buf[7] == b' ' as c_char);
        c_assert(buf[8] == b'0' as c_char);
        c_assert(buf[9] == b' ' as c_char);
        c_assert(buf[10] == b'0' as c_char); /* ->data_vm + ->stack_vm */
        c_assert(buf[11] == b' ' as c_char);
        c_assert(buf[12] == b'0' as c_char);
        c_assert(buf[13] == b'\n' as c_char);
    }

    /* Test PROCMAP_QUERY ioctl() for /proc/$PID/maps */
    {
        let mut path_buf = [0 as c_char; 256];
        let mut exp_path_buf = [0 as c_char; 256];
        let mut q: procmap_query = zeroed();
        let mut fd: c_int;
        let mut err: c_int;

        snprintf(path_buf.as_mut_ptr(), path_buf.len(), c"/proc/%u/maps".as_ptr(), pid as c_uint);
        fd = open(path_buf.as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }

        /* CASE 1: exact MATCH at VADDR */
        memset(&mut q as *mut _ as *mut c_void, 0, size_of::<procmap_query>());
        q.size = size_of::<procmap_query>() as __u64;
        q.query_addr = VADDR;
        q.query_flags = 0;
        q.vma_name_addr = path_buf.as_mut_ptr() as c_ulong as __u64;
        q.vma_name_size = path_buf.len() as __u64;

        err = ioctl(fd, PROCMAP_QUERY, &mut q);
        c_assert(err == 0);

        c_assert(q.query_addr == VADDR);
        c_assert(q.query_flags == 0);

        c_assert(q.vma_flags == (PROCMAP_QUERY_VMA_READABLE | PROCMAP_QUERY_VMA_EXECUTABLE));
        c_assert(q.vma_start == VADDR);
        c_assert(q.vma_end == VADDR + PAGE_SIZE);
        c_assert(q.vma_page_size == PAGE_SIZE);

        c_assert(q.vma_offset == 0);
        c_assert(q.inode == st.st_ino as __u64);
        c_assert(q.dev_major == MAJOR(st.st_dev) as __u64);
        c_assert(q.dev_minor == MINOR(st.st_dev) as __u64);

        snprintf(
            exp_path_buf.as_mut_ptr(),
            exp_path_buf.len(),
            c"/tmp/#%llu (deleted)".as_ptr(),
            st.st_ino as c_ulong,
        );
        c_assert(q.vma_name_size == strlen(exp_path_buf.as_ptr()) as __u64 + 1);
        c_assert(strcmp(path_buf.as_ptr(), exp_path_buf.as_ptr()) == 0);

        /* CASE 2: NO MATCH at VADDR-1 */
        memset(&mut q as *mut _ as *mut c_void, 0, size_of::<procmap_query>());
        q.size = size_of::<procmap_query>() as __u64;
        q.query_addr = VADDR - 1;
        q.query_flags = 0; /* exact match */

        err = ioctl(fd, PROCMAP_QUERY, &mut q);
        err = if err < 0 { -errno() } else { 0 };
        c_assert(err == -ENOENT);

        /* CASE 3: MATCH COVERING_OR_NEXT_VMA at VADDR - 1 */
        memset(&mut q as *mut _ as *mut c_void, 0, size_of::<procmap_query>());
        q.size = size_of::<procmap_query>() as __u64;
        q.query_addr = VADDR - 1;
        q.query_flags = PROCMAP_QUERY_COVERING_OR_NEXT_VMA;

        err = ioctl(fd, PROCMAP_QUERY, &mut q);
        c_assert(err == 0);

        c_assert(q.query_addr == VADDR - 1);
        c_assert(q.query_flags == PROCMAP_QUERY_COVERING_OR_NEXT_VMA);
        c_assert(q.vma_start == VADDR);
        c_assert(q.vma_end == VADDR + PAGE_SIZE);

        /* CASE 4: NO MATCH at VADDR + PAGE_SIZE */
        memset(&mut q as *mut _ as *mut c_void, 0, size_of::<procmap_query>());
        q.size = size_of::<procmap_query>() as __u64;
        q.query_addr = VADDR + PAGE_SIZE; /* point right after the VMA */
        q.query_flags = PROCMAP_QUERY_COVERING_OR_NEXT_VMA;

        err = ioctl(fd, PROCMAP_QUERY, &mut q);
        err = if err < 0 { -errno() } else { 0 };
        c_assert(err == -ENOENT);

        /* CASE 5: NO MATCH WRITABLE at VADDR */
        memset(&mut q as *mut _ as *mut c_void, 0, size_of::<procmap_query>());
        q.size = size_of::<procmap_query>() as __u64;
        q.query_addr = VADDR;
        q.query_flags = PROCMAP_QUERY_VMA_WRITABLE;

        err = ioctl(fd, PROCMAP_QUERY, &mut q);
        err = if err < 0 { -errno() } else { 0 };
        c_assert(err == -ENOENT);
    }

    0
}

#[cfg(target_arch = "x86_64")]
fn main() {
    unsafe {
        std::process::exit(main_x86_64());
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn main() {
    std::process::exit(4);
}

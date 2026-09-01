/* SPDX-License-Identifier: GPL-2.0 */
/* _GNU_SOURCE */

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use std::ptr;

const MEMCG_SOCKSTAT_WAIT_RETRIES: c_int = 30;

const BUF_SIZE: usize = 4096;
const PATH_MAX: usize = 4096;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const O_TMPFILE: c_int = 0o20000000;
const O_RDWR: c_int = 0o2;
const O_EXCL: c_int = 0o200;
const O_WRONLY: c_int = 0o1;
const O_APPEND: c_int = 0o2000;
const O_CLOEXEC: c_int = 0o2000000;
const ENOENT: c_int = 2;
const EAGAIN: c_int = 11;
const EADDRINUSE: c_int = 98;
const EAFNOSUPPORT: c_int = 97;
const ECONNRESET: c_int = 104;
const S_IWUSR: c_uint = 0o200;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANON: c_int = 0x20;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SIGKILL: c_int = 9;
const OOM_SCORE_ADJ_MIN: c_int = -1000;
const IN_DELETE_SELF: c_int = 0x00000400;
const IN_IGNORED: c_int = 0x00008000;
const DEFAULT_WAIT_INTERVAL_US: c_long = 100000;
const _SC_PAGE_SIZE: c_int = 30;

static mut HAS_LOCALEVENTS: bool = false;
static mut HAS_RECURSIVEPROT: bool = false;
static mut PAGE_SIZE: c_int = 0;

#[repr(C)]
struct Stat {
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
    __unused: [c_long; 3],
}

#[repr(C)]
struct In6Addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct Sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct SockaddrIn6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: In6Addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct Addrinfo {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: u32,
    ai_addr: *mut Sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut Addrinfo,
}

#[repr(C)]
struct InotifyEvent {
    wd: c_int,
    mask: u32,
    cookie: u32,
    len: u32,
}

type SocklenT = u32;
type SsizeT = isize;
type SizeT = usize;
type CgRunFn = unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int;
type TestFn = unsafe fn(*const c_char) -> c_int;

unsafe extern "C" {
    static mut errno: c_int;
    static in6addr_any: In6Addr;
    static MAP_FAILED: *mut c_void;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, st: *mut Stat) -> c_int;
    fn ftruncate(fd: c_int, length: c_long) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SsizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SsizeT;
    fn close(fd: c_int) -> c_int;
    fn dprintf(fd: c_int, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: SizeT, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn malloc(size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn getppid() -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: SizeT,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn mlock(addr: *const c_void, len: SizeT) -> c_int;
    fn munmap(addr: *mut c_void, length: SizeT) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn rand() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: SocklenT,
    ) -> c_int;
    fn bind(socket: c_int, address: *const Sockaddr, address_len: SocklenT) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut Sockaddr, address_len: *mut SocklenT) -> c_int;
    fn connect(socket: c_int, address: *const Sockaddr, address_len: SocklenT) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const Addrinfo,
        res: *mut *mut Addrinfo,
    ) -> c_int;
    fn freeaddrinfo(res: *mut Addrinfo);
    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;

    fn read_text(path: *const c_char, buf: *mut c_char, size: SizeT) -> c_int;
    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_name_indexed(root: *const c_char, name: *const c_char, index: c_int) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_read(cgroup: *const c_char, control: *const c_char, buf: *mut c_char, len: SizeT) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
    fn cg_read_strcmp(cgroup: *const c_char, control: *const c_char, expected: *const c_char) -> c_int;
    fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
    fn cg_read_long_fd(fd: c_int) -> c_long;
    fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long;
    fn cg_read_key_long_poll(
        cgroup: *const c_char,
        control: *const c_char,
        key: *const c_char,
        expected: c_long,
        retries: c_int,
        interval: c_long,
    ) -> c_long;
    fn cg_open(cgroup: *const c_char, control: *const c_char, flags: c_int) -> c_int;
    fn cg_run(cgroup: *const c_char, fn_: CgRunFn, arg: *mut c_void) -> c_int;
    fn cg_run_nowait(cgroup: *const c_char, fn_: CgRunFn, arg: *mut c_void) -> c_int;
    fn cg_control(cgroup: *const c_char, control: *const c_char) -> *mut c_char;
    fn cg_find_unified_root(root: *mut c_char, len: SizeT, mount: *mut c_void) -> c_int;
    fn memcg_prepare_for_wait(cgroup: *const c_char) -> c_int;
    fn cg_wait_for(fd: c_int);
    fn proc_mount_contains(option: *const c_char) -> c_int;
    fn values_close(a: c_long, b: c_long, err: c_int) -> bool;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished() -> !;
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn mb(x: c_long) -> c_long {
    x * 1024 * 1024
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn get_temp_fd() -> c_int {
    open(c!("."), O_TMPFILE | O_RDWR | O_EXCL, 0)
}

unsafe fn alloc_pagecache(fd: c_int, mut size: SizeT) -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];
    let mut st: Stat = mem::zeroed();
    let mut i: SizeT;

    if fstat(fd, &mut st) != 0 {
        return -1;
    }

    size += st.st_size as SizeT;

    if ftruncate(fd, size as c_long) != 0 {
        return -1;
    }

    i = 0;
    while i < size {
        read(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf));
        i += mem::size_of_val(&buf);
    }

    0
}

unsafe fn alloc_and_populate_anon(size: SizeT) -> *mut c_char {
    let buf: *mut c_char;
    let mut ptr_: *mut c_char;

    buf = malloc(size) as *mut c_char;
    if buf.is_null() {
        fprintf(stderr, c!("malloc() failed\n"));
        return ptr::null_mut();
    }

    ptr_ = buf;
    while ptr_ < buf.add(size) {
        *ptr_ = 0;
        ptr_ = ptr_.add(PAGE_SIZE as usize);
    }

    buf
}

unsafe extern "C" fn alloc_anon(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let size = arg as c_ulong as SizeT;
    let buf = alloc_and_populate_anon(size);

    if buf.is_null() {
        return -1;
    }

    free(buf as *mut c_void);
    0
}

unsafe fn is_swap_enabled() -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];
    let delim = c!("\n");
    let mut cnt = 0;
    let mut line: *mut c_char;

    if read_text(c!("/proc/swaps"), buf.as_mut_ptr(), mem::size_of_val(&buf)) <= 0 {
        return -1;
    }

    line = strtok(buf.as_mut_ptr(), delim);
    while !line.is_null() {
        cnt += 1;
        line = strtok(ptr::null_mut(), delim);
    }

    (cnt > 1) as c_int
}

unsafe fn set_oom_adj_score(pid: c_int, score: c_int) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let fd: c_int;
    let len: c_int;

    sprintf(path.as_mut_ptr(), c!("/proc/%d/oom_score_adj"), pid);

    fd = open(path.as_ptr(), O_WRONLY | O_APPEND, 0);
    if fd < 0 {
        return fd;
    }

    len = dprintf(fd, c!("%d"), score);
    if len < 0 {
        close(fd);
        return len;
    }

    close(fd);
    0
}

/*
 * This test creates two nested cgroups with and without enabling
 * the memory controller.
 */
unsafe fn test_memcg_subtree_control(root: *const c_char) -> c_int {
    let mut parent: *mut c_char;
    let mut child: *mut c_char;
    let mut parent2: *mut c_char = ptr::null_mut();
    let mut child2: *mut c_char = ptr::null_mut();
    let mut ret = KSFT_FAIL;
    let mut buf = [0 as c_char; BUF_SIZE];

    /* Create two nested cgroups with the memory controller enabled */
    parent = cg_name(root, c!("memcg_test_0"));
    child = cg_name(root, c!("memcg_test_0/memcg_test_1"));
    if parent.is_null() || child.is_null() {
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_create(parent) != 0 {
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_write(parent, c!("cgroup.subtree_control"), c!("+memory")) != 0 {
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_create(child) != 0 {
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_read_strstr(child, c!("cgroup.controllers"), c!("memory")) != 0 {
        cg_destroy(child);
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    /* Create two nested cgroups without enabling memory controller */
    parent2 = cg_name(root, c!("memcg_test_1"));
    child2 = cg_name(root, c!("memcg_test_1/memcg_test_1"));
    if parent2.is_null() || child2.is_null() {
        free(parent2 as *mut c_void);
        free(child2 as *mut c_void);
        cg_destroy(child);
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_create(parent2) != 0 {
        free(parent2 as *mut c_void);
        free(child2 as *mut c_void);
        cg_destroy(child);
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_create(child2) != 0 {
        cg_destroy(parent2);
        free(parent2 as *mut c_void);
        free(child2 as *mut c_void);
        cg_destroy(child);
        cg_destroy(parent);
        free(parent as *mut c_void);
        free(child as *mut c_void);
        return ret;
    }

    if cg_read(child2, c!("cgroup.controllers"), buf.as_mut_ptr(), mem::size_of_val(&buf)) == 0
        && cg_read_strstr(child2, c!("cgroup.controllers"), c!("memory")) != 0
    {
        ret = KSFT_PASS;
    }

    cg_destroy(child2);
    cg_destroy(parent2);
    free(parent2 as *mut c_void);
    free(child2 as *mut c_void);
    cg_destroy(child);
    cg_destroy(parent);
    free(parent as *mut c_void);
    free(child as *mut c_void);

    ret
}

unsafe extern "C" fn alloc_anon_50M_check(cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    let size = mb(50) as SizeT;
    let buf: *mut c_char;
    let anon: c_long;
    let current: c_long;
    let mut ret = -1;

    buf = alloc_and_populate_anon(size);
    if buf.is_null() {
        return -1;
    }

    current = cg_read_long(cgroup, c!("memory.current"));
    if current >= size as c_long
        && values_close(size as c_long, current, 3)
        && {
            anon = cg_read_key_long(cgroup, c!("memory.stat"), c!("anon "));
            anon >= 0 && values_close(anon, current, 3)
        }
    {
        ret = 0;
    }

    free(buf as *mut c_void);
    ret
}

unsafe extern "C" fn alloc_pagecache_50M_check(cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    let size = mb(50) as SizeT;
    let mut ret = -1;
    let current: c_long;
    let file: c_long;
    let fd: c_int;

    fd = get_temp_fd();
    if fd < 0 {
        return -1;
    }

    if alloc_pagecache(fd, size) == 0 {
        current = cg_read_long(cgroup, c!("memory.current"));
        if current >= size as c_long {
            file = cg_read_key_long(cgroup, c!("memory.stat"), c!("file "));
            if file >= 0 && values_close(file, current, 10) {
                ret = 0;
            }
        }
    }

    close(fd);
    ret
}

/*
 * This test create a memory cgroup, allocates
 * some anonymous memory and some pagecache
 * and checks memory.current, memory.peak, and some memory.stat values.
 */
unsafe fn test_memcg_current_peak(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let current: c_long;
    let mut peak: c_long;
    let mut peak_reset: c_long;
    let memcg: *mut c_char;
    let mut fd2_closed = false;
    let mut fd3_closed = false;
    let mut fd4_closed = false;
    let mut peak_fd = -1;
    let mut peak_fd2 = -1;
    let mut peak_fd3 = -1;
    let mut peak_fd4 = -1;
    let mut ss: Stat = mem::zeroed();
    static RESET_STRING: &[u8] = b"reset\n\0";

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0 {
            break;
        }
        current = cg_read_long(memcg, c!("memory.current"));
        if current != 0 {
            break;
        }
        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak != 0 {
            break;
        }
        if cg_run(memcg, alloc_anon_50M_check, ptr::null_mut()) != 0 {
            break;
        }
        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak < mb(50) {
            break;
        }

        /*
         * We'll open a few FDs for the same memory.peak file to exercise the free-path
         * We need at least three to be closed in a different order than writes occurred to test
         * the linked-list handling.
         */
        peak_fd = cg_open(memcg, c!("memory.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if peak_fd == -1 {
            if errno == ENOENT {
                ret = KSFT_SKIP;
            }
            break;
        }

        /*
         * Before we try to use memory.peak's fd, try to figure out whether
         * this kernel supports writing to that file in the first place. (by
         * checking the writable bit on the file's st_mode)
         */
        if fstat(peak_fd, &mut ss) != 0 {
            break;
        }
        if (ss.st_mode & S_IWUSR) == 0 {
            ret = KSFT_SKIP;
            break;
        }

        peak_fd2 = cg_open(memcg, c!("memory.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if peak_fd2 == -1 {
            break;
        }
        peak_fd3 = cg_open(memcg, c!("memory.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if peak_fd3 == -1 {
            break;
        }

        /* any non-empty string resets, but make it clear */
        peak_reset = write(peak_fd, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_long;
        if peak_reset != RESET_STRING.len() as c_long {
            break;
        }
        peak_reset = write(peak_fd2, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_long;
        if peak_reset != RESET_STRING.len() as c_long {
            break;
        }
        peak_reset = write(peak_fd3, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_long;
        if peak_reset != RESET_STRING.len() as c_long {
            break;
        }

        /* Make sure a completely independent read isn't affected by our  FD-local reset above*/
        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak < mb(50) {
            break;
        }

        fd2_closed = true;
        if close(peak_fd2) != 0 {
            break;
        }

        peak_fd4 = cg_open(memcg, c!("memory.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if peak_fd4 == -1 {
            break;
        }

        peak_reset = write(peak_fd4, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_long;
        if peak_reset != RESET_STRING.len() as c_long {
            break;
        }

        peak = cg_read_long_fd(peak_fd);
        if peak > mb(30) || peak < 0 {
            break;
        }

        if cg_run(memcg, alloc_pagecache_50M_check, ptr::null_mut()) != 0 {
            break;
        }

        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak < mb(50) {
            break;
        }

        /* Make sure everything is back to normal */
        peak = cg_read_long_fd(peak_fd);
        if peak < mb(50) {
            break;
        }
        peak = cg_read_long_fd(peak_fd4);
        if peak < mb(50) {
            break;
        }

        fd3_closed = true;
        if close(peak_fd3) != 0 {
            break;
        }
        fd4_closed = true;
        if close(peak_fd4) != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    close(peak_fd);
    if !fd2_closed {
        close(peak_fd2);
    }
    if !fd3_closed {
        close(peak_fd3);
    }
    if !fd4_closed {
        close(peak_fd4);
    }
    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

unsafe extern "C" fn alloc_pagecache_50M_noexit(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let fd = arg as c_long as c_int;
    let ppid = getppid();

    if alloc_pagecache(fd, mb(50) as SizeT) != 0 {
        return -1;
    }

    while getppid() == ppid {
        sleep(1);
    }

    0
}

unsafe extern "C" fn alloc_anon_noexit(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let ppid = getppid();
    let size = arg as c_ulong as SizeT;
    let buf: *mut c_char;

    buf = alloc_and_populate_anon(size);
    if buf.is_null() {
        return -1;
    }

    while getppid() == ppid {
        sleep(1);
    }

    free(buf as *mut c_void);
    0
}

/*
 * Wait until processes are killed asynchronously by the OOM killer
 * If we exceed a timeout, fail.
 */
unsafe fn cg_test_proc_killed(cgroup: *const c_char) -> c_int {
    let mut limit: c_int;

    limit = 10;
    while limit > 0 {
        if cg_read_strcmp(cgroup, c!("cgroup.procs"), c!("")) == 0 {
            return 0;
        }

        usleep(100000);
        limit -= 1;
    }
    -1
}

unsafe fn reclaim_until(memcg: *const c_char, goal: c_long) -> bool;

/*
 * First, this test creates the following hierarchy:
 * A       memory.min = 0,    memory.max = 200M
 * A/B     memory.min = 50M
 * A/B/C   memory.min = 75M,  memory.current = 50M
 * A/B/D   memory.min = 25M,  memory.current = 50M
 * A/B/E   memory.min = 0,    memory.current = 50M
 * A/B/F   memory.min = 500M, memory.current = 0
 *
 * (or memory.low if we test soft protection)
 *
 * Usages are pagecache and the test keeps a running
 * process in every leaf cgroup.
 * Then it creates A/G and creates a significant
 * memory pressure in A.
 *
 * Then it checks actual memory usages and expects that:
 * A/B    memory.current ~= 50M
 * A/B/C  memory.current ~= 29M [memory.events:low > 0]
 * A/B/D  memory.current ~= 21M [memory.events:low > 0]
 * A/B/E  memory.current ~= 0   [memory.events:low == 0 if !memory_recursiveprot,
 *				 undefined otherwise]
 * A/B/F  memory.current  = 0   [memory.events:low == 0]
 * (for origin of the numbers, see model in memcg_protection.m.)
 *
 * After that it tries to allocate more than there is
 * unprotected memory in A available, and checks that:
 * a) memory.min protects pagecache even in this case,
 * b) memory.low allows reclaiming page cache with low events.
 *
 * Then we try to reclaim from A/B/C using memory.reclaim until its
 * usage reaches 10M.
 * This makes sure that:
 * (a) We ignore the protection of the reclaim target memcg.
 * (b) The previously calculated emin value (~29M) should be dismissed.
 */
unsafe fn test_memcg_protection(root: *const c_char, min: bool) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut rc: c_int;
    let mut parent = [ptr::null_mut::<c_char>(); 3];
    let mut children = [ptr::null_mut::<c_char>(); 4];
    let attribute = if min { c!("memory.min") } else { c!("memory.low") };
    let mut cvals = [0 as c_long; 4];
    let current: c_long;
    let mut i: c_int;
    let mut attempts: c_int;
    let fd: c_int;

    fd = get_temp_fd();
    if fd < 0 {
        return ret;
    }

    loop {
        parent[0] = cg_name(root, c!("memcg_test_0"));
        if parent[0].is_null() {
            break;
        }
        parent[1] = cg_name(parent[0], c!("memcg_test_1"));
        if parent[1].is_null() {
            break;
        }
        parent[2] = cg_name(parent[0], c!("memcg_test_2"));
        if parent[2].is_null() {
            break;
        }

        if cg_create(parent[0]) != 0 {
            break;
        }

        if cg_read_long(parent[0], attribute) != 0 {
            /* No memory.min on older kernels is fine */
            if min {
                ret = KSFT_SKIP;
            }
            break;
        }

        if cg_write(parent[0], c!("cgroup.subtree_control"), c!("+memory")) != 0
            || cg_write(parent[0], c!("memory.max"), c!("200M")) != 0
            || cg_write(parent[0], c!("memory.swap.max"), c!("0")) != 0
            || cg_create(parent[1]) != 0
            || cg_write(parent[1], c!("cgroup.subtree_control"), c!("+memory")) != 0
            || cg_create(parent[2]) != 0
        {
            break;
        }

        i = 0;
        while (i as usize) < children.len() {
            children[i as usize] = cg_name_indexed(parent[1], c!("child_memcg"), i);
            if children[i as usize].is_null() || cg_create(children[i as usize]) != 0 {
                break;
            }
            if i <= 2 {
                cg_run_nowait(children[i as usize], alloc_pagecache_50M_noexit, fd as c_long as *mut c_void);
            }
            i += 1;
        }
        if (i as usize) < children.len() {
            break;
        }

        if cg_write(parent[1], attribute, c!("50M")) != 0
            || cg_write(children[0], attribute, c!("75M")) != 0
            || cg_write(children[1], attribute, c!("25M")) != 0
            || cg_write(children[2], attribute, c!("0")) != 0
            || cg_write(children[3], attribute, c!("500M")) != 0
        {
            break;
        }

        attempts = 0;
        while !values_close(cg_read_long(parent[1], c!("memory.current")), mb(150), 3) {
            if attempts > 5 {
                break;
            }
            attempts += 1;
            sleep(1);
        }

        if cg_run(parent[2], alloc_anon, mb(148) as *mut c_void) != 0 {
            break;
        }

        if !values_close(cg_read_long(parent[1], c!("memory.current")), mb(50), 3) {
            break;
        }

        i = 0;
        while (i as usize) < children.len() {
            cvals[i as usize] = cg_read_long(children[i as usize], c!("memory.current"));
            i += 1;
        }

        if !values_close(cvals[0], mb(29), 15) {
            break;
        }
        if !values_close(cvals[1], mb(21), 20) {
            break;
        }
        if cvals[3] != 0 {
            break;
        }

        rc = cg_run(parent[2], alloc_anon, mb(170) as *mut c_void);
        if min && rc == 0 {
            break;
        } else if !min && rc != 0 {
            fprintf(stderr, c!("memory.low prevents from allocating anon memory\n"));
            break;
        }

        current = if min { mb(50) } else { mb(30) };
        if !values_close(cg_read_long(parent[1], c!("memory.current")), current, 3) {
            break;
        }

        if !reclaim_until(children[0], mb(10)) {
            break;
        }

        if min {
            ret = KSFT_PASS;
            break;
        }

        /*
         * Child 2 has memory.low=0, but some low protection may still be
         * distributed down from its parent with memory.low=50M if cgroup2
         * memory_recursiveprot mount option is enabled. Ignore the low
         * event count in this case.
         */
        i = 0;
        while (i as usize) < children.len() {
            let ignore_low_events_index = if HAS_RECURSIVEPROT { 2 } else { -1 };
            let no_low_events_index = 1;
            let low: c_long;
            let oom: c_long;

            oom = cg_read_key_long(children[i as usize], c!("memory.events"), c!("oom "));
            low = cg_read_key_long(children[i as usize], c!("memory.events"), c!("low "));

            if oom != 0 {
                break;
            }
            if i == ignore_low_events_index {
                i += 1;
                continue;
            }
            if i <= no_low_events_index && low <= 0 {
                break;
            }
            if i > no_low_events_index && low != 0 {
                break;
            }
            i += 1;
        }
        if (i as usize) < children.len() {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    i = children.len() as c_int - 1;
    while i >= 0 {
        if !children[i as usize].is_null() {
            cg_destroy(children[i as usize]);
            free(children[i as usize] as *mut c_void);
        }
        i -= 1;
    }

    i = parent.len() as c_int - 1;
    while i >= 0 {
        if !parent[i as usize].is_null() {
            cg_destroy(parent[i as usize]);
            free(parent[i as usize] as *mut c_void);
        }
        i -= 1;
    }
    close(fd);
    ret
}

unsafe fn test_memcg_min(root: *const c_char) -> c_int {
    test_memcg_protection(root, true)
}

unsafe fn test_memcg_low(root: *const c_char) -> c_int {
    test_memcg_protection(root, false)
}

unsafe extern "C" fn alloc_pagecache_max_30M(cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    let size = mb(50) as SizeT;
    let mut ret = -1;
    let current: c_long;
    let high: c_long;
    let max: c_long;
    let fd: c_int;

    high = cg_read_long(cgroup, c!("memory.high"));
    max = cg_read_long(cgroup, c!("memory.max"));
    if high != mb(30) && max != mb(30) {
        return -1;
    }

    fd = get_temp_fd();
    if fd < 0 {
        return -1;
    }

    if alloc_pagecache(fd, size) == 0 {
        current = cg_read_long(cgroup, c!("memory.current"));
        if values_close(current, mb(30), 5) {
            ret = 0;
        }
    }

    close(fd);
    ret
}

/*
 * This test checks that memory.high limits the amount of
 * memory which can be consumed by either anonymous memory
 * or pagecache.
 */
unsafe fn test_memcg_high(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let memcg: *mut c_char;
    let high: c_long;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0
            || cg_read_strcmp(memcg, c!("memory.high"), c!("max\n")) != 0
            || cg_write(memcg, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(memcg, c!("memory.high"), c!("30M")) != 0
            || cg_run(memcg, alloc_anon, mb(31) as *mut c_void) != 0
            || cg_run(memcg, alloc_pagecache_50M_check, ptr::null_mut()) == 0
            || cg_run(memcg, alloc_pagecache_max_30M, ptr::null_mut()) != 0
        {
            break;
        }

        high = cg_read_key_long(memcg, c!("memory.events"), c!("high "));
        if high <= 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

unsafe extern "C" fn alloc_anon_mlock(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let size = arg as SizeT;
    let buf: *mut c_void;

    buf = mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, 0, 0);
    if buf == MAP_FAILED {
        return -1;
    }

    mlock(buf, size);
    munmap(buf, size);
    0
}

/*
 * This test checks that memory.high is able to throttle big single shot
 * allocation i.e. large allocation within one kernel entry.
 */
unsafe fn test_memcg_high_sync(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let pid: c_int;
    let mut fd = -1;
    let memcg: *mut c_char;
    let pre_high: c_long;
    let pre_max: c_long;
    let post_high: c_long;
    let post_max: c_long;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0 {
            break;
        }
        pre_high = cg_read_key_long(memcg, c!("memory.events"), c!("high "));
        pre_max = cg_read_key_long(memcg, c!("memory.events"), c!("max "));
        if pre_high < 0 || pre_max < 0 {
            break;
        }

        if cg_write(memcg, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(memcg, c!("memory.high"), c!("30M")) != 0
            || cg_write(memcg, c!("memory.max"), c!("140M")) != 0
        {
            break;
        }

        fd = memcg_prepare_for_wait(memcg);
        if fd < 0 {
            break;
        }

        pid = cg_run_nowait(memcg, alloc_anon_mlock, mb(200) as *mut c_void);
        if pid < 0 {
            break;
        }

        cg_wait_for(fd);

        post_high = cg_read_key_long(memcg, c!("memory.events"), c!("high "));
        post_max = cg_read_key_long(memcg, c!("memory.events"), c!("max "));
        if post_high < 0 || post_max < 0 {
            break;
        }

        if pre_high == post_high || pre_max != post_max {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if fd >= 0 {
        close(fd);
    }
    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

/*
 * This test checks that memory.max limits the amount of
 * memory which can be consumed by either anonymous memory
 * or pagecache.
 */
unsafe fn test_memcg_max(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let memcg: *mut c_char;
    let current: c_long;
    let max: c_long;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0
            || cg_read_strcmp(memcg, c!("memory.max"), c!("max\n")) != 0
            || cg_write(memcg, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(memcg, c!("memory.max"), c!("30M")) != 0
        {
            break;
        }

        /* Should be killed by OOM killer */
        if cg_run(memcg, alloc_anon, mb(100) as *mut c_void) == 0 {
            break;
        }

        if cg_run(memcg, alloc_pagecache_max_30M, ptr::null_mut()) != 0 {
            break;
        }

        current = cg_read_long(memcg, c!("memory.current"));
        if current > mb(30) || current == 0 {
            break;
        }

        max = cg_read_key_long(memcg, c!("memory.events"), c!("max "));
        if max <= 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

/*
 * Reclaim from @memcg until usage reaches @goal by writing to
 * memory.reclaim.
 *
 * This function will return false if the usage is already below the
 * goal.
 *
 * This function assumes that writing to memory.reclaim is the only
 * source of change in memory.current (no concurrent allocations or
 * reclaim).
 *
 * This function makes sure memory.reclaim is sane. It will return
 * false if memory.reclaim's error codes do not make sense, even if
 * the usage goal was satisfied.
 */
unsafe fn reclaim_until(memcg: *const c_char, goal: c_long) -> bool {
    let mut buf = [0 as c_char; 64];
    let mut retries: c_int;
    let err: c_int;
    let current: c_long;
    let to_reclaim: c_long;
    let mut reclaimed = false;

    retries = 5;
    while retries > 0 {
        current = cg_read_long(memcg, c!("memory.current"));

        if current < goal || values_close(current, goal, 3) {
            break;
        /* Did memory.reclaim return 0 incorrectly? */
        } else if reclaimed {
            return false;
        }

        to_reclaim = current - goal;
        snprintf(buf.as_mut_ptr(), mem::size_of_val(&buf), c!("%ld"), to_reclaim);
        err = cg_write(memcg, c!("memory.reclaim"), buf.as_ptr());
        if err == 0 {
            reclaimed = true;
        } else if err != -EAGAIN {
            return false;
        }
        retries -= 1;
    }
    reclaimed
}

/*
 * This test checks that memory.reclaim reclaims the given
 * amount of memory (from both anon and file, if possible).
 */
unsafe fn test_memcg_reclaim(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut fd = -1;
    let mut retries: c_int;
    let memcg: *mut c_char;
    let current: c_long;
    let expected_usage: c_long;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0 {
            break;
        }

        current = cg_read_long(memcg, c!("memory.current"));
        if current != 0 {
            break;
        }

        fd = get_temp_fd();
        if fd < 0 {
            break;
        }

        cg_run_nowait(memcg, alloc_pagecache_50M_noexit, fd as c_long as *mut c_void);

        /*
         * If swap is enabled, try to reclaim from both anon and file, else try
         * to reclaim from file only.
         */
        expected_usage = if is_swap_enabled() != 0 {
            cg_run_nowait(memcg, alloc_anon_noexit, mb(50) as *mut c_void);
            mb(100)
        } else {
            mb(50)
        };

        /*
         * Wait until current usage reaches the expected usage (or we run out of
         * retries).
         */
        retries = 5;
        while !values_close(cg_read_long(memcg, c!("memory.current")), expected_usage, 10) {
            if retries != 0 {
                retries -= 1;
                sleep(1);
                continue;
            } else {
                fprintf(
                    stderr,
                    c!("failed to allocate %ld for memcg reclaim test\n"),
                    expected_usage,
                );
                break;
            }
        }
        if !values_close(cg_read_long(memcg, c!("memory.current")), expected_usage, 10) {
            break;
        }

        /*
         * Reclaim until current reaches 30M, this makes sure we hit both anon
         * and file if swap is enabled.
         */
        if !reclaim_until(memcg, mb(30)) {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    cg_destroy(memcg);
    free(memcg as *mut c_void);
    close(fd);

    ret
}

unsafe extern "C" fn alloc_anon_50M_check_swap(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let mem_max = arg as c_long;
    let size = mb(50) as SizeT;
    let buf: *mut c_char;
    let mem_current: c_long;
    let swap_current: c_long;
    let mut ret = -1;

    buf = alloc_and_populate_anon(size);
    if buf.is_null() {
        return -1;
    }

    mem_current = cg_read_long(cgroup, c!("memory.current"));
    if mem_current != 0 && values_close(mem_current, mem_max, 3) {
        swap_current = cg_read_long(cgroup, c!("memory.swap.current"));
        if swap_current != 0 && values_close(mem_current + swap_current, size as c_long, 3) {
            ret = 0;
        }
    }

    free(buf as *mut c_void);
    ret
}

/*
 * This test checks that memory.swap.max limits the amount of
 * anonymous memory which can be swapped out. Additionally, it verifies that
 * memory.swap.peak reflects the high watermark and can be reset.
 */
unsafe fn test_memcg_swap_max_peak(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let memcg: *mut c_char;
    let max: c_long;
    let mut peak: c_long;
    let mut ss: Stat = mem::zeroed();
    let mut swap_peak_fd = -1;
    let mut mem_peak_fd = -1;
    static RESET_STRING: &[u8] = b"foobarbaz\0";
    let mut peak_reset: c_int;

    /* any non-empty string resets */
    if is_swap_enabled() == 0 {
        return KSFT_SKIP;
    }

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0 {
            break;
        }

        if cg_read_long(memcg, c!("memory.swap.current")) != 0 {
            ret = KSFT_SKIP;
            break;
        }

        swap_peak_fd = cg_open(memcg, c!("memory.swap.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if swap_peak_fd == -1 {
            if errno == ENOENT {
                ret = KSFT_SKIP;
            }
            break;
        }

        /*
         * Before we try to use memory.swap.peak's fd, try to figure out
         * whether this kernel supports writing to that file in the first
         * place. (by checking the writable bit on the file's st_mode)
         */
        if fstat(swap_peak_fd, &mut ss) != 0 {
            break;
        }

        if (ss.st_mode & S_IWUSR) == 0 {
            ret = KSFT_SKIP;
            break;
        }

        mem_peak_fd = cg_open(memcg, c!("memory.peak"), O_RDWR | O_APPEND | O_CLOEXEC);
        if mem_peak_fd == -1
            || cg_read_long(memcg, c!("memory.swap.peak")) != 0
            || cg_read_long_fd(swap_peak_fd) != 0
        {
            break;
        }

        /* switch the swap and mem fds into local-peak tracking mode*/
        peak_reset = write(swap_peak_fd, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_int;
        if peak_reset != RESET_STRING.len() as c_int
            || cg_read_long_fd(swap_peak_fd) != 0
            || cg_read_long(memcg, c!("memory.peak")) != 0
            || cg_read_long_fd(mem_peak_fd) != 0
        {
            break;
        }

        peak_reset = write(mem_peak_fd, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_int;
        if peak_reset != RESET_STRING.len() as c_int
            || cg_read_long_fd(mem_peak_fd) != 0
            || cg_read_strcmp(memcg, c!("memory.max"), c!("max\n")) != 0
            || cg_read_strcmp(memcg, c!("memory.swap.max"), c!("max\n")) != 0
            || cg_write(memcg, c!("memory.swap.max"), c!("30M")) != 0
            || cg_write(memcg, c!("memory.max"), c!("30M")) != 0
        {
            break;
        }

        /* Should be killed by OOM killer */
        if cg_run(memcg, alloc_anon, mb(100) as *mut c_void) == 0
            || cg_read_key_long(memcg, c!("memory.events"), c!("oom ")) != 1
            || cg_read_key_long(memcg, c!("memory.events"), c!("oom_kill ")) != 1
        {
            break;
        }

        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long(memcg, c!("memory.swap.peak"));
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long_fd(mem_peak_fd);
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long_fd(swap_peak_fd);
        if peak < mb(29) {
            break;
        }

        /*
         * open, reset and close the peak swap on another FD to make sure
         * multiple extant fds don't corrupt the linked-list
         */
        peak_reset = cg_write(memcg, c!("memory.swap.peak"), RESET_STRING.as_ptr() as *const c_char);
        if peak_reset != 0 {
            break;
        }
        peak_reset = cg_write(memcg, c!("memory.peak"), RESET_STRING.as_ptr() as *const c_char);
        if peak_reset != 0 {
            break;
        }

        /* actually reset on the fds */
        peak_reset = write(swap_peak_fd, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_int;
        if peak_reset != RESET_STRING.len() as c_int {
            break;
        }
        peak_reset = write(mem_peak_fd, RESET_STRING.as_ptr() as *const c_void, RESET_STRING.len()) as c_int;
        if peak_reset != RESET_STRING.len() as c_int {
            break;
        }

        peak = cg_read_long_fd(swap_peak_fd);
        if peak > mb(10) {
            break;
        }

        /*
         * The cgroup is now empty, but there may be a page or two associated
         * with the open FD accounted to it.
         */
        peak = cg_read_long_fd(mem_peak_fd);
        if peak > mb(1)
            || cg_read_long(memcg, c!("memory.peak")) < mb(29)
            || cg_read_long(memcg, c!("memory.swap.peak")) < mb(29)
            || cg_run(memcg, alloc_anon_50M_check_swap, mb(30) as *mut c_void) != 0
        {
            break;
        }

        max = cg_read_key_long(memcg, c!("memory.events"), c!("max "));
        if max <= 0 {
            break;
        }

        peak = cg_read_long(memcg, c!("memory.peak"));
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long(memcg, c!("memory.swap.peak"));
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long_fd(mem_peak_fd);
        if peak < mb(29) {
            break;
        }
        peak = cg_read_long_fd(swap_peak_fd);
        if peak < mb(19) {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if mem_peak_fd != -1 && close(mem_peak_fd) != 0 {
        ret = KSFT_FAIL;
    }
    if swap_peak_fd != -1 && close(swap_peak_fd) != 0 {
        ret = KSFT_FAIL;
    }
    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

/*
 * This test disables swapping and tries to allocate anonymous memory
 * up to OOM. Then it checks for oom and oom_kill events in
 * memory.events.
 */
unsafe fn test_memcg_oom_events(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let memcg: *mut c_char;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    if cg_create(memcg) == 0
        && cg_write(memcg, c!("memory.max"), c!("30M")) == 0
        && cg_write(memcg, c!("memory.swap.max"), c!("0")) == 0
        && cg_run(memcg, alloc_anon, mb(100) as *mut c_void) != 0
        && cg_read_strcmp(memcg, c!("cgroup.procs"), c!("")) == 0
        && cg_read_key_long(memcg, c!("memory.events"), c!("oom ")) == 1
        && cg_read_key_long(memcg, c!("memory.events"), c!("oom_kill ")) == 1
    {
        ret = KSFT_PASS;
    }

    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

#[repr(C)]
struct TcpServerArgs {
    port: u16,
    ctl: [c_int; 2],
}

unsafe extern "C" fn tcp_server(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let srv_args = arg as *mut TcpServerArgs;
    let mut saddr: SockaddrIn6 = mem::zeroed();
    let slen = mem::size_of_val(&saddr) as SocklenT;
    let sk: c_int;
    let client_sk: c_int;
    let ctl_fd: c_int;
    let yes = 1;
    let mut ret = -1;

    close((*srv_args).ctl[0]);
    ctl_fd = (*srv_args).ctl[1];

    saddr.sin6_family = AF_INET6 as u16;
    saddr.sin6_addr = in6addr_any;
    saddr.sin6_port = htons((*srv_args).port);

    sk = socket(AF_INET6, SOCK_STREAM, 0);
    if sk < 0 {
        /* Pass back errno to the ctl_fd */
        write(ctl_fd, &errno as *const c_int as *const c_void, mem::size_of_val(&errno));
        return ret;
    }

    if setsockopt(
        sk,
        SOL_SOCKET,
        SO_REUSEADDR,
        &yes as *const c_int as *const c_void,
        mem::size_of_val(&yes) as SocklenT,
    ) < 0
    {
        close(sk);
        return ret;
    }

    if bind(sk, &saddr as *const SockaddrIn6 as *const Sockaddr, slen) != 0 {
        write(ctl_fd, &errno as *const c_int as *const c_void, mem::size_of_val(&errno));
        close(sk);
        return ret;
    }

    if listen(sk, 1) != 0 {
        close(sk);
        return ret;
    }

    ret = 0;
    if write(ctl_fd, &ret as *const c_int as *const c_void, mem::size_of_val(&ret)) != mem::size_of_val(&ret) as isize {
        ret = -1;
        close(sk);
        return ret;
    }

    client_sk = accept(sk, ptr::null_mut(), ptr::null_mut());
    if client_sk < 0 {
        close(sk);
        return ret;
    }

    ret = -1;
    loop {
        let buf = [0u8; 0x100000];

        if write(client_sk, buf.as_ptr() as *const c_void, mem::size_of_val(&buf)) <= 0 {
            if errno == ECONNRESET {
                ret = 0;
            }
            break;
        }
    }

    close(client_sk);
    close(sk);
    ret
}

unsafe fn tcp_client(cgroup: *const c_char, port: u16) -> c_int {
    let server = c!("localhost");
    let mut ai: *mut Addrinfo = ptr::null_mut();
    let mut servport = [0 as c_char; 6];
    let mut retries = 0x10; /* nice round number */
    let sk: c_int;
    let mut ret: c_int;
    let allocated: c_long;

    allocated = cg_read_long(cgroup, c!("memory.current"));
    snprintf(servport.as_mut_ptr(), mem::size_of_val(&servport), c!("%hd"), port as c_int);
    ret = getaddrinfo(server, servport.as_ptr(), ptr::null(), &mut ai);
    if ret != 0 {
        return ret;
    }

    sk = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
    if sk < 0 {
        freeaddrinfo(ai);
        return ret;
    }

    ret = connect(sk, (*ai).ai_addr, (*ai).ai_addrlen);
    if ret < 0 {
        close(sk);
        freeaddrinfo(ai);
        return ret;
    }

    ret = KSFT_FAIL;
    while retries != 0 {
        let mut buf = [0u8; 0x100000];
        let current: c_long;
        let sock: c_long;

        retries -= 1;
        if read(sk, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf)) <= 0 {
            break;
        }

        current = cg_read_long(cgroup, c!("memory.current"));
        sock = cg_read_key_long(cgroup, c!("memory.stat"), c!("sock "));

        if current < 0 || sock < 0 {
            break;
        }

        /* exclude the memory not related to socket connection */
        if values_close(current - allocated, sock, 10) {
            ret = KSFT_PASS;
            break;
        }
    }

    close(sk);
    freeaddrinfo(ai);
    ret
}

/*
 * This test checks socket memory accounting.
 * The test forks a TCP server listens on a random port between 1000
 * and 61000. Once it gets a client connection, it starts writing to
 * its socket.
 * The TCP client interleaves reads from the socket with check whether
 * memory.current and memory.stat.sock are similar.
 */
unsafe fn test_memcg_sock(root: *const c_char) -> c_int {
    let mut bind_retries = 5;
    let mut ret = KSFT_FAIL;
    let mut pid: c_int = -1;
    let mut err: c_int = 0;
    let mut port: u16 = 0;
    let memcg: *mut c_char;
    let mut sock_post = -1;

    memcg = cg_name(root, c!("memcg_test"));
    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0 {
            break;
        }

        while bind_retries != 0 {
            let mut args = TcpServerArgs { port: 0, ctl: [0; 2] };

            bind_retries -= 1;
            if pipe(args.ctl.as_mut_ptr()) != 0 {
                break;
            }

            args.port = (1000 + rand() % 60000) as u16;
            port = args.port;

            pid = cg_run_nowait(memcg, tcp_server, &mut args as *mut TcpServerArgs as *mut c_void);
            if pid < 0 {
                break;
            }

            close(args.ctl[1]);
            if read(args.ctl[0], &mut err as *mut c_int as *mut c_void, mem::size_of_val(&err)) != mem::size_of_val(&err) as isize {
                break;
            }
            close(args.ctl[0]);

            /* Skip if address family not supported by protocol */
            if err == EAFNOSUPPORT {
                ret = KSFT_SKIP;
                break;
            }

            if err == 0 {
                break;
            }
            if err != EADDRINUSE {
                break;
            }

            waitpid(pid, ptr::null_mut(), 0);
        }

        if ret == KSFT_SKIP {
            break;
        }
        if err == EADDRINUSE {
            ret = KSFT_SKIP;
            break;
        }
        if err != 0 || pid < 0 {
            break;
        }

        if tcp_client(memcg, port) != KSFT_PASS {
            break;
        }

        waitpid(pid, &mut err, 0);
        if wexitstatus(err) != 0 {
            break;
        }

        if cg_read_long(memcg, c!("memory.current")) < 0 {
            break;
        }

        /*
         * memory.stat is updated asynchronously via the memcg rstat
         * flushing worker, which runs periodically (every 2 seconds,
         * see FLUSH_TIME). On a busy system, the "sock " counter may
         * stay non-zero for a short period of time after the TCP
         * connection is closed and all socket memory has been
         * uncharged.
         *
         * Poll memory.stat for up to 3 seconds (~FLUSH_TIME plus some
         * scheduling slack) and require that the "sock " counter
         * eventually drops to zero.
         */
        sock_post = cg_read_key_long_poll(
            memcg,
            c!("memory.stat"),
            c!("sock "),
            0,
            MEMCG_SOCKSTAT_WAIT_RETRIES,
            DEFAULT_WAIT_INTERVAL_US,
        );
        if sock_post != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    cg_destroy(memcg);
    free(memcg as *mut c_void);

    ret
}

/*
 * This test disables swapping and tries to allocate anonymous memory
 * up to OOM with memory.group.oom set. Then it checks that all
 * processes in the leaf were killed. It also checks that oom_events
 * were propagated to the parent level.
 */
unsafe fn test_memcg_oom_group_leaf_events(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let parent: *mut c_char;
    let child: *mut c_char;
    let parent_oom_events: c_long;

    parent = cg_name(root, c!("memcg_test_0"));
    child = cg_name(root, c!("memcg_test_0/memcg_test_1"));

    loop {
        if parent.is_null() || child.is_null() {
            break;
        }

        if cg_create(parent) != 0
            || cg_create(child) != 0
            || cg_write(parent, c!("cgroup.subtree_control"), c!("+memory")) != 0
            || cg_write(child, c!("memory.max"), c!("50M")) != 0
            || cg_write(child, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(child, c!("memory.oom.group"), c!("1")) != 0
        {
            break;
        }

        cg_run_nowait(parent, alloc_anon_noexit, mb(60) as *mut c_void);
        cg_run_nowait(child, alloc_anon_noexit, mb(1) as *mut c_void);
        cg_run_nowait(child, alloc_anon_noexit, mb(1) as *mut c_void);
        if cg_run(child, alloc_anon, mb(100) as *mut c_void) == 0 {
            break;
        }

        if cg_test_proc_killed(child) != 0 {
            break;
        }

        if cg_read_key_long(child, c!("memory.events"), c!("oom_kill ")) <= 0 {
            break;
        }

        parent_oom_events = cg_read_key_long(parent, c!("memory.events"), c!("oom_kill "));
        /*
         * If memory_localevents is not enabled (the default), the parent should
         * count OOM events in its children groups. Otherwise, it should not
         * have observed any events.
         */
        if HAS_LOCALEVENTS && parent_oom_events != 0 {
            break;
        } else if !HAS_LOCALEVENTS && parent_oom_events <= 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if !child.is_null() {
        cg_destroy(child);
    }
    if !parent.is_null() {
        cg_destroy(parent);
    }
    free(child as *mut c_void);
    free(parent as *mut c_void);

    ret
}

/*
 * This test disables swapping and tries to allocate anonymous memory
 * up to OOM with memory.group.oom set. Then it checks that all
 * processes in the parent and leaf were killed.
 */
unsafe fn test_memcg_oom_group_parent_events(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let parent: *mut c_char;
    let child: *mut c_char;

    parent = cg_name(root, c!("memcg_test_0"));
    child = cg_name(root, c!("memcg_test_0/memcg_test_1"));

    loop {
        if parent.is_null() || child.is_null() {
            break;
        }

        if cg_create(parent) != 0
            || cg_create(child) != 0
            || cg_write(parent, c!("memory.max"), c!("80M")) != 0
            || cg_write(parent, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(parent, c!("memory.oom.group"), c!("1")) != 0
        {
            break;
        }

        cg_run_nowait(parent, alloc_anon_noexit, mb(60) as *mut c_void);
        cg_run_nowait(child, alloc_anon_noexit, mb(1) as *mut c_void);
        cg_run_nowait(child, alloc_anon_noexit, mb(1) as *mut c_void);

        if cg_run(child, alloc_anon, mb(100) as *mut c_void) == 0 {
            break;
        }

        if cg_test_proc_killed(child) != 0 {
            break;
        }
        if cg_test_proc_killed(parent) != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if !child.is_null() {
        cg_destroy(child);
    }
    if !parent.is_null() {
        cg_destroy(parent);
    }
    free(child as *mut c_void);
    free(parent as *mut c_void);

    ret
}

/*
 * This test disables swapping and tries to allocate anonymous memory
 * up to OOM with memory.group.oom set. Then it checks that all
 * processes were killed except those set with OOM_SCORE_ADJ_MIN
 */
unsafe fn test_memcg_oom_group_score_events(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let memcg: *mut c_char;
    let safe_pid: c_int;

    memcg = cg_name(root, c!("memcg_test_0"));

    if memcg.is_null() {
        return ret;
    }

    loop {
        if cg_create(memcg) != 0
            || cg_write(memcg, c!("memory.max"), c!("50M")) != 0
            || cg_write(memcg, c!("memory.swap.max"), c!("0")) != 0
            || cg_write(memcg, c!("memory.oom.group"), c!("1")) != 0
        {
            break;
        }

        safe_pid = cg_run_nowait(memcg, alloc_anon_noexit, mb(1) as *mut c_void);
        if set_oom_adj_score(safe_pid, OOM_SCORE_ADJ_MIN) != 0 {
            break;
        }

        cg_run_nowait(memcg, alloc_anon_noexit, mb(1) as *mut c_void);
        if cg_run(memcg, alloc_anon, mb(100) as *mut c_void) == 0 {
            break;
        }

        if cg_read_key_long(memcg, c!("memory.events"), c!("oom_kill ")) != 3 {
            break;
        }

        if kill(safe_pid, SIGKILL) != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if !memcg.is_null() {
        cg_destroy(memcg);
    }
    free(memcg as *mut c_void);

    ret
}

unsafe fn read_event(inotify_fd: c_int, expected_event: c_int, expected_wd: c_int) -> c_int {
    let mut event: InotifyEvent = mem::zeroed();
    let len: SsizeT;

    len = read(
        inotify_fd,
        &mut event as *mut InotifyEvent as *mut c_void,
        mem::size_of_val(&event),
    );
    if len < mem::size_of_val(&event) as SsizeT {
        return -1;
    }

    if event.mask != expected_event as u32 || event.wd != expected_wd {
        fprintf(
            stderr,
            c!("event does not match expected values: mask %d (expected %d) wd %d (expected %d)\n"),
            event.mask,
            expected_event,
            event.wd,
            expected_wd,
        );
        return -1;
    }

    0
}

unsafe fn test_memcg_inotify_delete_file(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut memcg: *mut c_char = ptr::null_mut();
    let mut fd: c_int = -1;
    let wd: c_int;

    memcg = cg_name(root, c!("memcg_test_0"));

    loop {
        if memcg.is_null() {
            break;
        }

        if cg_create(memcg) != 0 {
            break;
        }

        fd = inotify_init1(0);
        if fd == -1 {
            break;
        }

        wd = inotify_add_watch(fd, cg_control(memcg, c!("memory.events")), IN_DELETE_SELF);
        if wd == -1 {
            break;
        }

        if cg_destroy(memcg) != 0 {
            break;
        }
        free(memcg as *mut c_void);
        memcg = ptr::null_mut();

        if read_event(fd, IN_DELETE_SELF as c_int, wd) != 0 {
            break;
        }

        if read_event(fd, IN_IGNORED as c_int, wd) != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if fd >= 0 {
        close(fd);
    }
    if !memcg.is_null() {
        cg_destroy(memcg);
    }
    free(memcg as *mut c_void);

    ret
}

unsafe fn test_memcg_inotify_delete_dir(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut memcg: *mut c_char = ptr::null_mut();
    let mut fd: c_int = -1;
    let wd: c_int;

    memcg = cg_name(root, c!("memcg_test_0"));

    loop {
        if memcg.is_null() {
            break;
        }

        if cg_create(memcg) != 0 {
            break;
        }

        fd = inotify_init1(0);
        if fd == -1 {
            break;
        }

        wd = inotify_add_watch(fd, memcg, IN_DELETE_SELF);
        if wd == -1 {
            break;
        }

        if cg_destroy(memcg) != 0 {
            break;
        }
        free(memcg as *mut c_void);
        memcg = ptr::null_mut();

        if read_event(fd, IN_DELETE_SELF as c_int, wd) != 0 {
            break;
        }

        if read_event(fd, IN_IGNORED as c_int, wd) != 0 {
            break;
        }

        ret = KSFT_PASS;
        break;
    }

    if fd >= 0 {
        close(fd);
    }
    if !memcg.is_null() {
        cg_destroy(memcg);
    }
    free(memcg as *mut c_void);

    ret
}

#[repr(C)]
struct MemcgTest {
    fn_: TestFn,
    name: *const c_char,
}

static TESTS: &[MemcgTest] = &[
    MemcgTest { fn_: test_memcg_subtree_control, name: c!("test_memcg_subtree_control") },
    MemcgTest { fn_: test_memcg_current_peak, name: c!("test_memcg_current_peak") },
    MemcgTest { fn_: test_memcg_min, name: c!("test_memcg_min") },
    MemcgTest { fn_: test_memcg_low, name: c!("test_memcg_low") },
    MemcgTest { fn_: test_memcg_high, name: c!("test_memcg_high") },
    MemcgTest { fn_: test_memcg_high_sync, name: c!("test_memcg_high_sync") },
    MemcgTest { fn_: test_memcg_max, name: c!("test_memcg_max") },
    MemcgTest { fn_: test_memcg_reclaim, name: c!("test_memcg_reclaim") },
    MemcgTest { fn_: test_memcg_oom_events, name: c!("test_memcg_oom_events") },
    MemcgTest { fn_: test_memcg_swap_max_peak, name: c!("test_memcg_swap_max_peak") },
    MemcgTest { fn_: test_memcg_sock, name: c!("test_memcg_sock") },
    MemcgTest { fn_: test_memcg_oom_group_leaf_events, name: c!("test_memcg_oom_group_leaf_events") },
    MemcgTest { fn_: test_memcg_oom_group_parent_events, name: c!("test_memcg_oom_group_parent_events") },
    MemcgTest { fn_: test_memcg_oom_group_score_events, name: c!("test_memcg_oom_group_score_events") },
    MemcgTest { fn_: test_memcg_inotify_delete_file, name: c!("test_memcg_inotify_delete_file") },
    MemcgTest { fn_: test_memcg_inotify_delete_dir, name: c!("test_memcg_inotify_delete_dir") },
];

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut root = [0 as c_char; PATH_MAX];
    let mut i: usize;
    let mut proc_status: c_int;

    PAGE_SIZE = sysconf(_SC_PAGE_SIZE) as c_int;
    if PAGE_SIZE <= 0 {
        PAGE_SIZE = BUF_SIZE as c_int;
    }

    ksft_print_header();
    ksft_set_plan(TESTS.len() as c_uint);
    if cg_find_unified_root(root.as_mut_ptr(), mem::size_of_val(&root), ptr::null_mut()) != 0 {
        ksft_exit_skip(c!("cgroup v2 isn't mounted\n"));
    }

    /*
     * Check that memory controller is available:
     * memory is listed in cgroup.controllers
     */
    if cg_read_strstr(root.as_ptr(), c!("cgroup.controllers"), c!("memory")) != 0 {
        ksft_exit_skip(c!("memory controller isn't available\n"));
    }

    if cg_read_strstr(root.as_ptr(), c!("cgroup.subtree_control"), c!("memory")) != 0 {
        if cg_write(root.as_ptr(), c!("cgroup.subtree_control"), c!("+memory")) != 0 {
            ksft_exit_skip(c!("Failed to set memory controller\n"));
        }
    }

    proc_status = proc_mount_contains(c!("memory_recursiveprot"));
    if proc_status < 0 {
        ksft_exit_skip(c!("Failed to query cgroup mount option\n"));
    }
    HAS_RECURSIVEPROT = proc_status != 0;

    proc_status = proc_mount_contains(c!("memory_localevents"));
    if proc_status < 0 {
        ksft_exit_skip(c!("Failed to query cgroup mount option\n"));
    }
    HAS_LOCALEVENTS = proc_status != 0;

    i = 0;
    while i < TESTS.len() {
        match (TESTS[i].fn_)(root.as_ptr()) {
            KSFT_PASS => {
                ksft_test_result_pass(c!("%s\n"), TESTS[i].name);
            }
            KSFT_SKIP => {
                ksft_test_result_skip(c!("%s\n"), TESTS[i].name);
            }
            _ => {
                ksft_test_result_fail(c!("%s\n"), TESTS[i].name);
            }
        }
        i += 1;
    }

    ksft_finished();
}

fn main() {
    unsafe {
        main_impl(0, ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

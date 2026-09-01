// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/d_path.c.
// C dependencies: test_progs.h, sys/stat.h, linux/sched.h, sys/syscall.h,
// test_d_path.skel.h, test_d_path_check_rdonly_mem.skel.h,
// test_d_path_check_types.skel.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const MAX_PATH_LEN: usize = 128;
const MAX_FILES: usize = 7;

/* sys_close_range is not around for long time, so let's
 * make sure we can call it on systems with older glibc
 */
#[cfg(target_arch = "alpha")]
const __NR_close_range: c_long = 546;
#[cfg(not(target_arch = "alpha"))]
const __NR_close_range: c_long = 436;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const O_RDONLY: c_int = 0;
const O_CREAT: c_int = 0o100;
const O_PATH: c_int = 0o10000000;

type __u32 = u32;
type pid_t = c_int;

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_d_path {
    pub bss: *mut test_d_path__bss,
}

#[repr(C)]
pub struct test_d_path__bss {
    pub my_pid: pid_t,
    pub called_stat: c_int,
    pub called_close: c_int,
    pub paths_stat: [[c_char; MAX_PATH_LEN]; MAX_FILES],
    pub paths_close: [[c_char; MAX_PATH_LEN]; MAX_FILES],
    pub rets_stat: [c_int; MAX_FILES],
    pub rets_close: [c_int; MAX_FILES],
    pub path_match_fallocate: c_int,
}

#[repr(C)]
pub struct test_d_path_check_rdonly_mem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_d_path_check_types {
    _private: [u8; 0],
}

#[repr(C)]
struct Src {
    cnt: __u32,
    paths: [[c_char; MAX_PATH_LEN]; MAX_FILES],
}

static mut duration: c_int = 0;

static mut src: Src = Src {
    cnt: 0,
    paths: [[0; MAX_PATH_LEN]; MAX_FILES],
};

unsafe extern "C" {
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn syscall(number: c_long, ...) -> c_long;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn getpid() -> pid_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn fallocate(fd: c_int, mode: c_int, offset: i64, len: i64) -> c_int;

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn test_d_path__open_and_load() -> *mut test_d_path;
    fn test_d_path__attach(skel: *mut test_d_path) -> c_int;
    fn test_d_path__destroy(skel: *mut test_d_path);

    fn test_d_path_check_rdonly_mem__open_and_load() -> *mut test_d_path_check_rdonly_mem;
    fn test_d_path_check_rdonly_mem__destroy(skel: *mut test_d_path_check_rdonly_mem);

    fn test_d_path_check_types__open_and_load() -> *mut test_d_path_check_types;
    fn test_d_path_check_types__destroy(skel: *mut test_d_path_check_types);
}

unsafe fn set_pathname(fd: c_int, pid: pid_t) -> c_int {
    let mut buf: [c_char; MAX_PATH_LEN] = [0; MAX_PATH_LEN];

    snprintf(
        buf.as_mut_ptr(),
        MAX_PATH_LEN,
        c"/proc/%d/fd/%d".as_ptr(),
        pid,
        fd,
    );
    let cnt = src.cnt as usize;
    src.cnt = src.cnt.wrapping_add(1);
    readlink(buf.as_ptr(), src.paths[cnt].as_mut_ptr(), MAX_PATH_LEN) as c_int
}

#[inline]
unsafe fn syscall_close(fd: c_int) -> c_long {
    syscall(
        __NR_close_range,
        fd as c_uint,
        fd as c_uint,
        0u32,
    )
}

unsafe fn trigger_fstat_events(pid: pid_t) -> c_int {
    let mut sockfd: c_int = -1;
    let mut procfd: c_int = -1;
    let mut devfd: c_int = -1;
    let mut localfd: c_int = -1;
    let mut indicatorfd: c_int = -1;
    let mut pipefd: [c_int; 2] = [-1, -1];
    let mut fileStat: stat = core::mem::zeroed();
    let mut ret: c_int = -1;

    /* unmountable pseudo-filesystems */
    if CHECK(pipe(pipefd.as_mut_ptr()) < 0, c"trigger".as_ptr(), c"pipe failed\n".as_ptr()) {
        return ret;
    }
    /* unmountable pseudo-filesystems */
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if CHECK(sockfd < 0, c"trigger".as_ptr(), c"socket failed\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    /* mountable pseudo-filesystems */
    procfd = open(c"/proc/self/comm".as_ptr(), O_RDONLY);
    if CHECK(procfd < 0, c"trigger".as_ptr(), c"open /proc/self/comm failed\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    devfd = open(c"/dev/urandom".as_ptr(), O_RDONLY);
    if CHECK(devfd < 0, c"trigger".as_ptr(), c"open /dev/urandom failed\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    localfd = open(c"/tmp/d_path_loadgen.txt".as_ptr(), O_CREAT | O_RDONLY, 0o644);
    if CHECK(localfd < 0, c"trigger".as_ptr(), c"open /tmp/d_path_loadgen.txt failed\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    /* bpf_d_path will return path with (deleted) */
    remove(c"/tmp/d_path_loadgen.txt".as_ptr());
    indicatorfd = open(c"/tmp/".as_ptr(), O_PATH);
    if CHECK(indicatorfd < 0, c"trigger".as_ptr(), c"open /tmp/ failed\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }

    ret = set_pathname(pipefd[0], pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for pipe[0]\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(pipefd[1], pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for pipe[1]\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(sockfd, pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for socket\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(procfd, pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for proc\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(devfd, pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for dev\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(localfd, pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for file\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }
    ret = set_pathname(indicatorfd, pid);
    if CHECK(ret < 0, c"trigger".as_ptr(), c"set_pathname failed for dir\n".as_ptr()) {
        goto_out_close(pipefd, sockfd, procfd, devfd, localfd, indicatorfd);
        return ret;
    }

    /* triggers vfs_getattr */
    fstat(pipefd[0], &mut fileStat);
    fstat(pipefd[1], &mut fileStat);
    fstat(sockfd, &mut fileStat);
    fstat(procfd, &mut fileStat);
    fstat(devfd, &mut fileStat);
    fstat(localfd, &mut fileStat);
    fstat(indicatorfd, &mut fileStat);

    /* sys_close no longer triggers filp_close, but we can
     * call sys_close_range instead which still does
     */
    syscall_close(pipefd[0]);
    syscall_close(pipefd[1]);
    syscall_close(sockfd);
    syscall_close(procfd);
    syscall_close(devfd);
    syscall_close(localfd);
    syscall_close(indicatorfd);
    ret
}

unsafe fn goto_out_close(
    pipefd: [c_int; 2],
    sockfd: c_int,
    procfd: c_int,
    devfd: c_int,
    localfd: c_int,
    indicatorfd: c_int,
) {
    /* sys_close no longer triggers filp_close, but we can
     * call sys_close_range instead which still does
     */
    syscall_close(pipefd[0]);
    syscall_close(pipefd[1]);
    syscall_close(sockfd);
    syscall_close(procfd);
    syscall_close(devfd);
    syscall_close(localfd);
    syscall_close(indicatorfd);
}

unsafe fn attach_and_load(skel: *mut *mut test_d_path) {
    let err: c_int;

    *skel = test_d_path__open_and_load();
    if CHECK((*skel).is_null(), c"setup".as_ptr(), c"d_path skeleton failed\n".as_ptr()) {
        test_d_path__destroy(*skel);
        *skel = ptr::null_mut();
        return;
    }

    err = test_d_path__attach(*skel);
    if CHECK(err != 0, c"setup".as_ptr(), c"attach failed: %d\n".as_ptr(), err) {
        test_d_path__destroy(*skel);
        *skel = ptr::null_mut();
        return;
    }

    (*(*skel).as_mut().unwrap().bss).my_pid = getpid();
}

unsafe fn test_d_path_basic() {
    let bss: *mut test_d_path__bss;
    let mut skel: *mut test_d_path = ptr::null_mut();
    let err: c_int;

    attach_and_load(&mut skel);
    if skel.is_null() {
        test_d_path__destroy(skel);
        return;
    }

    bss = (*skel).bss;

    err = trigger_fstat_events((*bss).my_pid);
    if err < 0 {
        test_d_path__destroy(skel);
        return;
    }

    if CHECK(
        (*bss).called_stat == 0,
        c"stat".as_ptr(),
        c"trampoline for security_inode_getattr was not called\n".as_ptr(),
    ) {
        test_d_path__destroy(skel);
        return;
    }

    if CHECK(
        (*bss).called_close == 0,
        c"close".as_ptr(),
        c"trampoline for filp_close was not called\n".as_ptr(),
    ) {
        test_d_path__destroy(skel);
        return;
    }

    for i in 0..MAX_FILES {
        CHECK(
            strncmp(src.paths[i].as_ptr(), (*bss).paths_stat[i].as_ptr(), MAX_PATH_LEN) != 0,
            c"check".as_ptr(),
            c"failed to get stat path[%d]: %s vs %s\n".as_ptr(),
            i as c_int,
            src.paths[i].as_ptr(),
            (*bss).paths_stat[i].as_ptr(),
        );
        CHECK(
            strncmp(src.paths[i].as_ptr(), (*bss).paths_close[i].as_ptr(), MAX_PATH_LEN) != 0,
            c"check".as_ptr(),
            c"failed to get close path[%d]: %s vs %s\n".as_ptr(),
            i as c_int,
            src.paths[i].as_ptr(),
            (*bss).paths_close[i].as_ptr(),
        );
        /* The d_path helper returns size plus NUL char, hence + 1 */
        CHECK(
            (*bss).rets_stat[i] != (strlen((*bss).paths_stat[i].as_ptr()) + 1) as c_int,
            c"check".as_ptr(),
            c"failed to match stat return [%d]: %d vs %zd [%s]\n".as_ptr(),
            i as c_int,
            (*bss).rets_stat[i],
            strlen((*bss).paths_stat[i].as_ptr()) + 1,
            (*bss).paths_stat[i].as_ptr(),
        );
        CHECK(
            (*bss).rets_close[i] != (strlen((*bss).paths_stat[i].as_ptr()) + 1) as c_int,
            c"check".as_ptr(),
            c"failed to match stat return [%d]: %d vs %zd [%s]\n".as_ptr(),
            i as c_int,
            (*bss).rets_close[i],
            strlen((*bss).paths_close[i].as_ptr()) + 1,
            (*bss).paths_stat[i].as_ptr(),
        );
    }

    test_d_path__destroy(skel);
}

unsafe fn test_d_path_check_rdonly_mem() {
    let skel: *mut test_d_path_check_rdonly_mem;

    skel = test_d_path_check_rdonly_mem__open_and_load();
    ASSERT_ERR_PTR(skel as *const c_void, c"unexpected_load_overwriting_rdonly_mem".as_ptr());

    test_d_path_check_rdonly_mem__destroy(skel);
}

unsafe fn test_d_path_check_types() {
    let skel: *mut test_d_path_check_types;

    skel = test_d_path_check_types__open_and_load();
    ASSERT_ERR_PTR(skel as *const c_void, c"unexpected_load_passing_wrong_type".as_ptr());

    test_d_path_check_types__destroy(skel);
}

/* Check if the verifier correctly generates code for
 * accessing the memory modified by d_path helper.
 */
unsafe fn test_d_path_mem_access() {
    let mut localfd: c_int = -1;
    let mut path_template: [c_char; 30] = *b"/dev/shm/d_path_loadgen.XXXXXX\0";
    let bss: *mut test_d_path__bss;
    let mut skel: *mut test_d_path = ptr::null_mut();

    attach_and_load(&mut skel);
    if skel.is_null() {
        syscall_close(localfd);
        test_d_path__destroy(skel);
        return;
    }

    bss = (*skel).bss;

    localfd = mkstemp(path_template.as_mut_ptr());
    if CHECK(localfd < 0, c"trigger".as_ptr(), c"mkstemp failed\n".as_ptr()) {
        syscall_close(localfd);
        test_d_path__destroy(skel);
        return;
    }

    if CHECK(
        fallocate(localfd, 0, 0, 1024) < 0,
        c"trigger".as_ptr(),
        c"fallocate failed\n".as_ptr(),
    ) {
        syscall_close(localfd);
        test_d_path__destroy(skel);
        return;
    }
    remove(path_template.as_ptr());

    if CHECK(
        (*bss).path_match_fallocate == 0,
        c"check".as_ptr(),
        c"failed to read fallocate path".as_ptr(),
    ) {
        syscall_close(localfd);
        test_d_path__destroy(skel);
        return;
    }

    syscall_close(localfd);
    test_d_path__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_d_path() {
    if test__start_subtest(c"basic".as_ptr()) {
        test_d_path_basic();
    }

    if test__start_subtest(c"check_rdonly_mem".as_ptr()) {
        test_d_path_check_rdonly_mem();
    }

    if test__start_subtest(c"check_alloc_mem".as_ptr()) {
        test_d_path_check_types();
    }

    if test__start_subtest(c"check_mem_access".as_ptr()) {
        test_d_path_mem_access();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

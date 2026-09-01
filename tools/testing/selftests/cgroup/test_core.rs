/* SPDX-License-Identifier: GPL-2.0 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type uid_t = c_uint;
type pthread_t = c_ulong;
type c_ulong = u64;

const PATH_MAX: usize = 4096;
const BUF_SIZE: usize = 4096;
const KSFT_FAIL: c_int = 1;
const KSFT_PASS: c_int = 0;
const KSFT_SKIP: c_int = 4;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANON: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const EINTR: c_int = 4;
const EBADF: c_int = 9;
const EACCES: c_int = 13;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const ENOSYS: c_int = 38;
const EOPNOTSUPP: c_int = 95;
const SIGSTOP: c_int = 19;
const SIGCONT: c_int = 18;
const SIGCHLD: c_int = 17;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const WSTOPPED: c_int = 2;
const WEXITED: c_int = 4;
const CLONE_FILES: c_int = 0x00000400;
const CLONE_VM: c_int = 0x00000100;
const CLONE_NEWCGROUP: c_int = 0x02000000;
const CG_THREADS_FILE: *const c_char = b"cgroup.threads\0".as_ptr() as *const c_char;
const CG_NAMED_NAME: *const c_char = b"selftest\0".as_ptr() as *const c_char;
const TEST_UID: uid_t = 65534;

static mut nsdelegate: bool = false;

unsafe extern "C" {
    static mut errno: c_int;
    static mut cg_test_v1_named: bool;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn getppid() -> pid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pause() -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn chown(path: *const c_char, owner: uid_t, group: uid_t) -> c_int;
    fn geteuid() -> uid_t;
    fn seteuid(euid: uid_t) -> c_int;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount(target: *const c_char) -> c_int;

    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
    fn cg_killall(cgroup: *const c_char) -> c_int;
    fn cg_read(cgroup: *const c_char, control: *const c_char, buf: *mut c_char, len: size_t) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_enter_current(cgroup: *const c_char) -> c_int;
    fn cg_read_strcmp(cgroup: *const c_char, control: *const c_char, expected: *const c_char) -> c_int;
    fn cg_read_strcmp_wait(cgroup: *const c_char, control: *const c_char, expected: *const c_char) -> c_int;
    fn dirfd_open_opath(path: *const c_char) -> c_int;
    fn clone_into_cgroup(cgroup_fd: c_int) -> pid_t;
    fn clone_reap(pid: pid_t, options: c_int) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, buf: *const c_char) -> c_int;
    fn clone_into_cgroup_run_wait(cgroup: *const c_char) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
    fn cg_read_lc(cgroup: *const c_char, control: *const c_char) -> c_int;
    fn cg_enter_current_thread(cgroup: *const c_char) -> c_int;
    fn proc_read_strstr(pid: c_int, tid: c_int, file: *const c_char, needle: *const c_char) -> c_int;
    fn cg_find_unified_root(root: *mut c_char, len: size_t, nsdelegate: *mut bool) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished() -> !;
}

fn MB(x: usize) -> usize {
    x * 1024 * 1024
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe extern "C" fn touch_anon(buf: *mut c_char, mut size: size_t) -> c_int {
    let fd: c_int;
    let mut pos: *mut c_char = buf;

    fd = open(b"/dev/urandom\0".as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        return -1;
    }

    while size > 0 {
        let ret: ssize_t = read(fd, pos as *mut c_void, size);

        if ret < 0 {
            if errno != EINTR {
                close(fd);
                return -1;
            }
        } else {
            pos = pos.add(ret as usize);
            size -= ret as usize;
        }
    }
    close(fd);

    0
}

unsafe extern "C" fn alloc_and_touch_anon_noexit(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let ppid: c_int = getppid();
    let size: size_t = arg as size_t;
    let buf: *mut c_void;

    buf = mmap(
        core::ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANON,
        0,
        0,
    );
    if buf == MAP_FAILED {
        return -1;
    }

    if touch_anon(buf as *mut c_char, size) != 0 {
        munmap(buf, size);
        return -1;
    }

    while getppid() == ppid {
        sleep(1);
    }

    munmap(buf, size);
    0
}

/*
 * Create a child process that allocates and touches 100MB, then waits to be
 * killed. Wait until the child is attached to the cgroup, kill all processes
 * in that cgroup and wait until "cgroup.procs" is empty. At this point try to
 * destroy the empty cgroup. The test helps detect race conditions between
 * dying processes leaving the cgroup and cgroup destruction path.
 */
unsafe extern "C" fn test_cgcore_destroy(root: *const c_char) -> c_int {
    let mut ret: c_int = KSFT_FAIL;
    let mut cg_test: *mut c_char = core::ptr::null_mut();
    let mut child_pid: c_int;
    let mut buf: [c_char; BUF_SIZE] = [0; BUF_SIZE];

    cg_test = cg_name(root, b"cg_test\0".as_ptr() as *const c_char);

    if cg_test.is_null() {
        goto_cleanup_destroy(cg_test, ret);
        return ret;
    }

    for _i in 0..10 {
        if cg_create(cg_test) != 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }

        child_pid = cg_run_nowait(cg_test, alloc_and_touch_anon_noexit, MB(100) as *mut c_void);

        if child_pid < 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }

        /* wait for the child to enter cgroup */
        if cg_wait_for_proc_count(cg_test, 1) != 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }

        if cg_killall(cg_test) != 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }

        /* wait for cgroup to be empty */
        loop {
            if cg_read(
                cg_test,
                b"cgroup.procs\0".as_ptr() as *const c_char,
                buf.as_mut_ptr(),
                buf.len(),
            ) != 0
            {
                goto_cleanup_destroy(cg_test, ret);
                return ret;
            }
            if buf[0] == 0 {
                break;
            }
            usleep(1000);
        }

        if rmdir(cg_test) != 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }

        if waitpid(child_pid, core::ptr::null_mut(), 0) < 0 {
            goto_cleanup_destroy(cg_test, ret);
            return ret;
        }
    }
    ret = KSFT_PASS;
    goto_cleanup_destroy(cg_test, ret)
}

unsafe fn goto_cleanup_destroy(cg_test: *mut c_char, ret: c_int) -> c_int {
    if !cg_test.is_null() {
        cg_destroy(cg_test);
    }
    free(cg_test as *mut c_void);
    ret
}

/*
 * A(0) - B(0) - C(1)
 *        \ D(0)
 *
 * A, B and C's "populated" fields would be 1 while D's 0.
 * test that after the one process in C is moved to root,
 * A,B and C's "populated" fields would flip to "0" and file
 * modified events will be generated on the
 * "cgroup.events" files of both cgroups.
 */
unsafe extern "C" fn test_cgcore_populated(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut err: c_int;
    let mut cg_test_a: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b: *mut c_char = core::ptr::null_mut();
    let mut cg_test_c: *mut c_char = core::ptr::null_mut();
    let mut cg_test_d: *mut c_char = core::ptr::null_mut();
    let mut cgroup_fd: c_int = -EBADF;
    let mut pid: pid_t;

    if cg_test_v1_named {
        return KSFT_SKIP;
    }

    cg_test_a = cg_name(root, b"cg_test_a\0".as_ptr() as *const c_char);
    cg_test_b = cg_name(root, b"cg_test_a/cg_test_b\0".as_ptr() as *const c_char);
    cg_test_c = cg_name(root, b"cg_test_a/cg_test_b/cg_test_c\0".as_ptr() as *const c_char);
    cg_test_d = cg_name(root, b"cg_test_a/cg_test_b/cg_test_d\0".as_ptr() as *const c_char);

    'cleanup: loop {
        if cg_test_a.is_null() || cg_test_b.is_null() || cg_test_c.is_null() || cg_test_d.is_null() {
            break 'cleanup;
        }
        if cg_create(cg_test_a) != 0 { break 'cleanup; }
        if cg_create(cg_test_b) != 0 { break 'cleanup; }
        if cg_create(cg_test_c) != 0 { break 'cleanup; }
        if cg_create(cg_test_d) != 0 { break 'cleanup; }
        if cg_enter_current(cg_test_c) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_a, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 1\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_b, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 1\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_c, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 1\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_d, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_enter_current(root) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_a, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_b, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_c, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(cg_test_d, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }

        /* Test that we can directly clone into a new cgroup. */
        cgroup_fd = dirfd_open_opath(cg_test_d);
        if cgroup_fd < 0 { break 'cleanup; }

        pid = clone_into_cgroup(cgroup_fd);
        if pid < 0 {
            if errno == ENOSYS {
                ret = KSFT_PASS;
            }
            break 'cleanup;
        }

        if pid == 0 {
            if raise(SIGSTOP) != 0 {
                exit(EXIT_FAILURE);
            }
            exit(EXIT_SUCCESS);
        }

        err = cg_read_strcmp(cg_test_d, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 1\n\0".as_ptr() as *const c_char);
        clone_reap(pid, WSTOPPED);
        kill(pid, SIGCONT);
        clone_reap(pid, WEXITED);
        if err != 0 { break 'cleanup; }
        if cg_read_strcmp_wait(cg_test_d, b"cgroup.events\0".as_ptr() as *const c_char, b"populated 0\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }

        /* Remove cgroup. */
        if !cg_test_d.is_null() {
            cg_destroy(cg_test_d);
            free(cg_test_d as *mut c_void);
            cg_test_d = core::ptr::null_mut();
        }

        pid = clone_into_cgroup(cgroup_fd);
        if pid < 0 {
            ret = KSFT_PASS;
            break 'cleanup;
        }
        if pid == 0 {
            exit(EXIT_SUCCESS);
        }
        clone_reap(pid, WEXITED);
        break 'cleanup;
    }

    if !cg_test_d.is_null() { cg_destroy(cg_test_d); }
    if !cg_test_c.is_null() { cg_destroy(cg_test_c); }
    if !cg_test_b.is_null() { cg_destroy(cg_test_b); }
    if !cg_test_a.is_null() { cg_destroy(cg_test_a); }
    free(cg_test_d as *mut c_void);
    free(cg_test_c as *mut c_void);
    free(cg_test_b as *mut c_void);
    free(cg_test_a as *mut c_void);
    if cgroup_fd >= 0 { close(cgroup_fd); }
    ret
}

/*
 * A (domain threaded) - B (threaded) - C (domain)
 *
 * test that C can't be used until it is turned into a
 * threaded cgroup.  "cgroup.type" file will report "domain (invalid)" in
 * these cases. Operations which fail due to invalid topology use
 * EOPNOTSUPP as the errno.
 */
unsafe extern "C" fn test_cgcore_invalid_domain(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut grandparent: *mut c_char = core::ptr::null_mut();
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();
    if cg_test_v1_named { return KSFT_SKIP; }
    grandparent = cg_name(root, b"cg_test_grandparent\0".as_ptr() as *const c_char);
    parent = cg_name(root, b"cg_test_grandparent/cg_test_parent\0".as_ptr() as *const c_char);
    child = cg_name(root, b"cg_test_grandparent/cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if parent.is_null() || child.is_null() || grandparent.is_null() { break 'cleanup; }
        if cg_create(grandparent) != 0 { break 'cleanup; }
        if cg_create(parent) != 0 { break 'cleanup; }
        if cg_create(child) != 0 { break 'cleanup; }
        if cg_write(parent, b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(child, b"cgroup.type\0".as_ptr() as *const c_char, b"domain invalid\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_enter_current(child) == 0 { break 'cleanup; }
        if errno != EOPNOTSUPP { break 'cleanup; }
        if clone_into_cgroup_run_wait(child) == 0 { break 'cleanup; }
        if errno == ENOSYS { ret = KSFT_PASS; break 'cleanup; }
        if errno != EOPNOTSUPP { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    cg_enter_current(root);
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    if !grandparent.is_null() { cg_destroy(grandparent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    free(grandparent as *mut c_void);
    ret
}

/*
 * Test that when a child becomes threaded
 * the parent type becomes domain threaded.
 */
unsafe extern "C" fn test_cgcore_parent_becomes_threaded(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut parent = core::ptr::null_mut();
    let mut child = core::ptr::null_mut();
    if cg_test_v1_named { return KSFT_SKIP; }
    parent = cg_name(root, b"cg_test_parent\0".as_ptr() as *const c_char);
    child = cg_name(root, b"cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if parent.is_null() || child.is_null() { break 'cleanup; }
        if cg_create(parent) != 0 { break 'cleanup; }
        if cg_create(child) != 0 { break 'cleanup; }
        if cg_write(child, b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_read_strcmp(parent, b"cgroup.type\0".as_ptr() as *const c_char, b"domain threaded\n\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    ret
}

/*
 * Test that there's no internal process constrain on threaded cgroups.
 * You can add threads/processes on a parent with a controller enabled.
 */
unsafe extern "C" fn test_cgcore_no_internal_process_constraint_on_threads(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();

    if cg_test_v1_named
        || cg_read_strstr(root, b"cgroup.controllers\0".as_ptr() as *const c_char, b"cpu\0".as_ptr() as *const c_char) != 0
        || cg_write(root, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+cpu\0".as_ptr() as *const c_char) != 0
    {
        ret = KSFT_SKIP;
    } else {
        parent = cg_name(root, b"cg_test_parent\0".as_ptr() as *const c_char);
        child = cg_name(root, b"cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
        'cleanup: loop {
            if parent.is_null() || child.is_null() { break 'cleanup; }
            if cg_create(parent) != 0 { break 'cleanup; }
            if cg_create(child) != 0 { break 'cleanup; }
            if cg_write(parent, b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
            if cg_write(child, b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
            if cg_write(parent, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+cpu\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
            if cg_enter_current(parent) != 0 { break 'cleanup; }
            ret = KSFT_PASS;
            break 'cleanup;
        }
    }

    cg_enter_current(root);
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    ret
}

/*
 * Test that you can't enable a controller on a child if it's not enabled
 * on the parent.
 */
unsafe extern "C" fn test_cgcore_top_down_constraint_enable(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();
    if cg_test_v1_named { return KSFT_SKIP; }
    parent = cg_name(root, b"cg_test_parent\0".as_ptr() as *const c_char);
    child = cg_name(root, b"cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if parent.is_null() || child.is_null() { break 'cleanup; }
        if cg_create(parent) != 0 { break 'cleanup; }
        if cg_create(child) != 0 { break 'cleanup; }
        if cg_write(child, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+memory\0".as_ptr() as *const c_char) == 0 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    ret
}

/*
 * Test that you can't disable a controller on a parent
 * if it's enabled in a child.
 */
unsafe extern "C" fn test_cgcore_top_down_constraint_disable(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();
    if cg_test_v1_named { return KSFT_SKIP; }
    parent = cg_name(root, b"cg_test_parent\0".as_ptr() as *const c_char);
    child = cg_name(root, b"cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if parent.is_null() || child.is_null() { break 'cleanup; }
        if cg_create(parent) != 0 { break 'cleanup; }
        if cg_create(child) != 0 { break 'cleanup; }
        if cg_write(parent, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+memory\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_write(child, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+memory\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_write(parent, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"-memory\0".as_ptr() as *const c_char) == 0 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    ret
}

/*
 * Test internal process constraint.
 * You can't add a pid to a domain parent if a controller is enabled.
 */
unsafe extern "C" fn test_cgcore_internal_process_constraint(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();
    if cg_test_v1_named { return KSFT_SKIP; }
    parent = cg_name(root, b"cg_test_parent\0".as_ptr() as *const c_char);
    child = cg_name(root, b"cg_test_parent/cg_test_child\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if parent.is_null() || child.is_null() { break 'cleanup; }
        if cg_create(parent) != 0 { break 'cleanup; }
        if cg_create(child) != 0 { break 'cleanup; }
        if cg_write(parent, b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+memory\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        if cg_enter_current(parent) == 0 { break 'cleanup; }
        if clone_into_cgroup_run_wait(parent) == 0 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    if !child.is_null() { cg_destroy(child); }
    if !parent.is_null() { cg_destroy(parent); }
    free(child as *mut c_void);
    free(parent as *mut c_void);
    ret
}

unsafe extern "C" fn dummy_thread_fn(_arg: *mut c_void) -> *mut c_void {
    pause() as size_t as *mut c_void
}

/*
 * Test threadgroup migration.
 * All threads of a process are migrated together.
 */
unsafe extern "C" fn test_cgcore_proc_migration(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut t: c_int;
    let mut c_threads: c_int = 0;
    let n_threads: c_int = 13;
    let mut src: *mut c_char = core::ptr::null_mut();
    let mut dst: *mut c_char = core::ptr::null_mut();
    let mut threads: [pthread_t; 13] = [0; 13];

    src = cg_name(root, b"cg_src\0".as_ptr() as *const c_char);
    dst = cg_name(root, b"cg_dst\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if src.is_null() || dst.is_null() { break 'cleanup; }
        if cg_create(src) != 0 { break 'cleanup; }
        if cg_create(dst) != 0 { break 'cleanup; }
        if cg_enter_current(src) != 0 { break 'cleanup; }
        while c_threads < n_threads {
            if pthread_create(&mut threads[c_threads as usize], core::ptr::null(), dummy_thread_fn, core::ptr::null_mut()) != 0 {
                break 'cleanup;
            }
            c_threads += 1;
        }
        cg_enter_current(dst);
        if cg_read_lc(dst, CG_THREADS_FILE) != n_threads + 1 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }

    t = 0;
    while t < c_threads {
        pthread_cancel(threads[t as usize]);
        t += 1;
    }
    t = 0;
    while t < c_threads {
        pthread_join(threads[t as usize], core::ptr::null_mut());
        t += 1;
    }
    cg_enter_current(root);
    if !dst.is_null() { cg_destroy(dst); }
    if !src.is_null() { cg_destroy(src); }
    free(dst as *mut c_void);
    free(src as *mut c_void);
    ret
}

unsafe extern "C" fn migrating_thread_fn(arg: *mut c_void) -> *mut c_void {
    let mut g: c_int;
    let mut i: c_int;
    let n_iterations: c_int = 1000;
    let grps: *mut *mut c_char = arg as *mut *mut c_char;
    let mut lines: [[c_char; PATH_MAX]; 3] = [[0; PATH_MAX]; 3];

    g = 1;
    while g < 3 {
        snprintf(
            lines[g as usize].as_mut_ptr(),
            lines[g as usize].len(),
            b"0::%s\n\0".as_ptr() as *const c_char,
            (*grps.add(g as usize)).add(strlen(*grps.add(0))),
        );
        g += 1;
    }

    i = 0;
    while i < n_iterations {
        cg_enter_current_thread(*grps.add(((i % 2) + 1) as usize));
        if proc_read_strstr(
            0,
            1,
            b"cgroup\0".as_ptr() as *const c_char,
            lines[((i % 2) + 1) as usize].as_mut_ptr(),
        ) != 0
        {
            return (-1isize) as *mut c_void;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

/*
 * Test single thread migration.
 * Threaded cgroups allow successful migration of a thread.
 */
unsafe extern "C" fn test_cgcore_thread_migration(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut dom: *mut c_char = core::ptr::null_mut();
    let mut line: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut grps: [*mut c_char; 3] = [root as *mut c_char, core::ptr::null_mut(), core::ptr::null_mut()];
    let mut thr: pthread_t = 0;
    let mut retval: *mut c_void = core::ptr::null_mut();

    dom = cg_name(root, b"cg_dom\0".as_ptr() as *const c_char);
    grps[1] = cg_name(root, b"cg_dom/cg_src\0".as_ptr() as *const c_char);
    grps[2] = cg_name(root, b"cg_dom/cg_dst\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if grps[1].is_null() || grps[2].is_null() || dom.is_null() { break 'cleanup; }
        if cg_create(dom) != 0 { break 'cleanup; }
        if cg_create(grps[1]) != 0 { break 'cleanup; }
        if cg_create(grps[2]) != 0 { break 'cleanup; }
        if !cg_test_v1_named {
            if cg_write(grps[1], b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
            if cg_write(grps[2], b"cgroup.type\0".as_ptr() as *const c_char, b"threaded\0".as_ptr() as *const c_char) != 0 { break 'cleanup; }
        }
        if cg_enter_current(grps[1]) != 0 { break 'cleanup; }
        if pthread_create(&mut thr, core::ptr::null(), migrating_thread_fn, grps.as_mut_ptr() as *mut c_void) != 0 { break 'cleanup; }
        if pthread_join(thr, &mut retval) != 0 { break 'cleanup; }
        if !retval.is_null() { break 'cleanup; }
        snprintf(line.as_mut_ptr(), line.len(), b"0::%s\n\0".as_ptr() as *const c_char, grps[1].add(strlen(grps[0])));
        if proc_read_strstr(0, 1, b"cgroup\0".as_ptr() as *const c_char, line.as_mut_ptr()) != 0 { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }
    cg_enter_current(root);
    if !grps[2].is_null() { cg_destroy(grps[2]); }
    if !grps[1].is_null() { cg_destroy(grps[1]); }
    if !dom.is_null() { cg_destroy(dom); }
    free(grps[2] as *mut c_void);
    free(grps[1] as *mut c_void);
    free(dom as *mut c_void);
    ret
}

/*
 * cgroup migration permission check should be performed based on the
 * credentials at the time of open instead of write.
 */
unsafe extern "C" fn test_cgcore_lesser_euid_open(root: *const c_char) -> c_int {
    let test_euid: uid_t = TEST_UID;
    let mut ret = KSFT_FAIL;
    let mut cg_test_a: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b: *mut c_char = core::ptr::null_mut();
    let mut cg_test_a_procs: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b_procs: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b_procs_fd: c_int = -1;
    let saved_uid: uid_t;

    cg_test_a = cg_name(root, b"cg_test_a\0".as_ptr() as *const c_char);
    cg_test_b = cg_name(root, b"cg_test_b\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if cg_test_a.is_null() || cg_test_b.is_null() { break 'cleanup; }
        cg_test_a_procs = cg_name(cg_test_a, b"cgroup.procs\0".as_ptr() as *const c_char);
        cg_test_b_procs = cg_name(cg_test_b, b"cgroup.procs\0".as_ptr() as *const c_char);
        if cg_test_a_procs.is_null() || cg_test_b_procs.is_null() { break 'cleanup; }
        if cg_create(cg_test_a) != 0 || cg_create(cg_test_b) != 0 { break 'cleanup; }
        if cg_enter_current(cg_test_a) != 0 { break 'cleanup; }
        if chown(cg_test_a_procs, test_euid, -1i32 as uid_t) != 0
            || chown(cg_test_b_procs, test_euid, -1i32 as uid_t) != 0
        {
            break 'cleanup;
        }
        saved_uid = geteuid();
        if seteuid(test_euid) != 0 { break 'cleanup; }
        cg_test_b_procs_fd = open(cg_test_b_procs, O_RDWR);
        if seteuid(saved_uid) != 0 { break 'cleanup; }
        if cg_test_b_procs_fd < 0 { break 'cleanup; }
        if write(cg_test_b_procs_fd, b"0\0".as_ptr() as *const c_void, 1) >= 0 || errno != EACCES { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }

    cg_enter_current(root);
    if cg_test_b_procs_fd >= 0 { close(cg_test_b_procs_fd); }
    if !cg_test_b.is_null() { cg_destroy(cg_test_b); }
    if !cg_test_a.is_null() { cg_destroy(cg_test_a); }
    free(cg_test_b_procs as *mut c_void);
    free(cg_test_a_procs as *mut c_void);
    free(cg_test_b as *mut c_void);
    free(cg_test_a as *mut c_void);
    ret
}

#[repr(C)]
struct lesser_ns_open_thread_arg {
    path: *const c_char,
    fd: c_int,
    err: c_int,
}

unsafe extern "C" fn lesser_ns_open_thread_fn(arg: *mut c_void) -> c_int {
    let targ: *mut lesser_ns_open_thread_arg = arg as *mut lesser_ns_open_thread_arg;

    (*targ).fd = open((*targ).path, O_RDWR);
    (*targ).err = errno;
    0
}

/*
 * cgroup migration permission check should be performed based on the cgroup
 * namespace at the time of open instead of write.
 */
unsafe extern "C" fn test_cgcore_lesser_ns_open(root: *const c_char) -> c_int {
    static mut STACK: [c_char; 65536] = [0; 65536];
    let mut ret = KSFT_FAIL;
    let mut cg_test_a: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b_procs: *mut c_char = core::ptr::null_mut();
    let mut cg_test_b_procs_fd: c_int = -1;
    let mut targ = lesser_ns_open_thread_arg {
        path: core::ptr::null(),
        fd: -1,
        err: 0,
    };
    let mut pid: pid_t;
    let mut status: c_int = 0;

    if !nsdelegate {
        return KSFT_SKIP;
    }

    cg_test_a = cg_name(root, b"cg_test_a\0".as_ptr() as *const c_char);
    cg_test_b = cg_name(root, b"cg_test_b\0".as_ptr() as *const c_char);
    'cleanup: loop {
        if cg_test_a.is_null() || cg_test_b.is_null() { break 'cleanup; }
        cg_test_b_procs = cg_name(cg_test_b, b"cgroup.procs\0".as_ptr() as *const c_char);
        if cg_test_b_procs.is_null() { break 'cleanup; }
        if cg_create(cg_test_a) != 0 || cg_create(cg_test_b) != 0 { break 'cleanup; }
        if cg_enter_current(cg_test_b) != 0 { break 'cleanup; }
        targ.path = cg_test_b_procs;
        pid = clone(
            lesser_ns_open_thread_fn,
            STACK.as_mut_ptr().add(STACK.len()) as *mut c_void,
            CLONE_NEWCGROUP | CLONE_FILES | CLONE_VM | SIGCHLD,
            &mut targ as *mut lesser_ns_open_thread_arg as *mut c_void,
        );
        if pid < 0 { break 'cleanup; }
        if waitpid(pid, &mut status, 0) < 0 { break 'cleanup; }
        if !WIFEXITED(status) { break 'cleanup; }
        cg_test_b_procs_fd = targ.fd;
        if cg_test_b_procs_fd < 0 { break 'cleanup; }
        if cg_enter_current(cg_test_a) != 0 { break 'cleanup; }
        status = write(cg_test_b_procs_fd, b"0\0".as_ptr() as *const c_void, 1) as c_int;
        if status >= 0 || errno != ENOENT { break 'cleanup; }
        ret = KSFT_PASS;
        break 'cleanup;
    }

    cg_enter_current(root);
    if cg_test_b_procs_fd >= 0 { close(cg_test_b_procs_fd); }
    if !cg_test_b.is_null() { cg_destroy(cg_test_b); }
    if !cg_test_a.is_null() { cg_destroy(cg_test_a); }
    free(cg_test_b_procs as *mut c_void);
    free(cg_test_b as *mut c_void);
    free(cg_test_a as *mut c_void);
    ret
}

unsafe extern "C" fn setup_named_v1_root(root: *mut c_char, len: size_t, name: *const c_char) -> c_int {
    let mut options: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut r: c_int;

    r = snprintf(root, len, b"/mnt/cg_selftest\0".as_ptr() as *const c_char);
    if r < 0 {
        return r;
    }

    r = snprintf(options.as_mut_ptr(), options.len(), b"none,name=%s\0".as_ptr() as *const c_char, name);
    if r < 0 {
        return r;
    }

    r = mkdir(root, 0o755);
    if r < 0 && errno != EEXIST {
        return r;
    }

    r = mount(
        b"none\0".as_ptr() as *const c_char,
        root,
        b"cgroup\0".as_ptr() as *const c_char,
        0,
        options.as_ptr() as *const c_void,
    );
    if r < 0 {
        return r;
    }

    0
}

unsafe extern "C" fn cleanup_named_v1_root(root: *mut c_char) {
    if !cg_test_v1_named {
        return;
    }
    umount(root);
    rmdir(root);
}

#[repr(C)]
struct corecg_test {
    fn_: unsafe extern "C" fn(*const c_char) -> c_int,
    name: *const c_char,
}

static tests: [corecg_test; 12] = [
    corecg_test { fn_: test_cgcore_internal_process_constraint, name: b"test_cgcore_internal_process_constraint\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_top_down_constraint_enable, name: b"test_cgcore_top_down_constraint_enable\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_top_down_constraint_disable, name: b"test_cgcore_top_down_constraint_disable\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_no_internal_process_constraint_on_threads, name: b"test_cgcore_no_internal_process_constraint_on_threads\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_parent_becomes_threaded, name: b"test_cgcore_parent_becomes_threaded\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_invalid_domain, name: b"test_cgcore_invalid_domain\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_populated, name: b"test_cgcore_populated\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_proc_migration, name: b"test_cgcore_proc_migration\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_thread_migration, name: b"test_cgcore_thread_migration\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_destroy, name: b"test_cgcore_destroy\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_lesser_euid_open, name: b"test_cgcore_lesser_euid_open\0".as_ptr() as *const c_char },
    corecg_test { fn_: test_cgcore_lesser_ns_open, name: b"test_cgcore_lesser_ns_open\0".as_ptr() as *const c_char },
];

unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn main(argc: c_int, argv: *mut *mut c_char) {
    let mut root: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut i: c_int;
    let _ = argc;
    let _ = argv;

    ksft_print_header();
    ksft_set_plan(ARRAY_SIZE(&tests) as c_int);
    if cg_find_unified_root(root.as_mut_ptr(), root.len(), &mut nsdelegate) != 0 {
        if setup_named_v1_root(root.as_mut_ptr(), root.len(), CG_NAMED_NAME) != 0 {
            ksft_exit_skip(b"cgroup v2 isn't mounted and could not setup named v1 hierarchy\n\0".as_ptr() as *const c_char);
        }
        cg_test_v1_named = true;
    } else {
        if cg_read_strstr(root.as_mut_ptr(), b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"memory\0".as_ptr() as *const c_char) != 0 {
            if cg_write(root.as_mut_ptr(), b"cgroup.subtree_control\0".as_ptr() as *const c_char, b"+memory\0".as_ptr() as *const c_char) != 0 {
                ksft_exit_skip(b"Failed to set memory controller\n\0".as_ptr() as *const c_char);
            }
        }
    }

    i = 0;
    while (i as usize) < ARRAY_SIZE(&tests) {
        match (tests[i as usize].fn_)(root.as_mut_ptr()) {
            KSFT_PASS => {
                ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, tests[i as usize].name);
            }
            KSFT_SKIP => {
                ksft_test_result_skip(b"%s\n\0".as_ptr() as *const c_char, tests[i as usize].name);
            }
            _ => {
                ksft_test_result_fail(b"%s\n\0".as_ptr() as *const c_char, tests[i as usize].name);
            }
        }
        i += 1;
    }

    cleanup_named_v1_root(root.as_mut_ptr());
    ksft_finished();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// _GNU_SOURCE, assert.h, errno.h, fcntl.h, linux/types.h, sched.h, signal.h,
// stdio.h, stdlib.h, string.h, syscall.h, sys/wait.h, sys/mman.h, sys/mount.h,
// "pidfd.h", and "kselftest.h".

use std::ffi::{c_char, c_int, c_ulong, c_void, CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr;

const PIDFD_PASS: c_int = 0;
const PIDFD_ERROR: c_int = 1;
const PIDFD_FAIL: c_int = 2;
const PIDFD_SKIP: c_int = 3;
const PIDFD_XFAIL: c_int = 4;

const CHILD_STACK_SIZE: usize = 8192;

const CLONE_PIDFD: c_int = 0x00001000;
const CLONE_NEWPID: c_int = 0x20000000;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;
const SIGCHLD: c_int = 17;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_STACK: c_int = 0x20000;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const MNT_DETACH: c_int = 2;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
struct error {
    code: c_int,
    msg: [c_char; 512],
}

#[repr(C)]
struct child {
    stack: *mut c_char,
    pid: pid_t,
    fd: c_int,
}

#[allow(non_camel_case_types)]
type pid_t = c_int;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn assert_perror(errnum: c_int);
    fn clone(
        fn_: extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn geteuid() -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut c_void) -> isize;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;

    fn ksft_exit_fail_msg(msg: *const c_char, ...);
    fn ksft_exit_pass() -> !;
    fn ksft_print_header();
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_error(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_test_result_skip(msg: *const c_char, ...);
    fn wait_for_pid(pid: pid_t) -> c_int;
}

fn c_string(s: String) -> CString {
    CString::new(s).unwrap()
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn set_msg(err: *mut error, msg: String) {
    let bytes = msg.as_bytes();
    assert!(bytes.len() < unsafe { (*err).msg.len() });
    unsafe {
        ptr::write_bytes((*err).msg.as_mut_ptr(), 0, (*err).msg.len());
        ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, (*err).msg.as_mut_ptr(), bytes.len());
    }
}

unsafe fn error_set(err: *mut error, code: c_int, msg: String) -> c_int {
    if code == PIDFD_PASS || err.is_null() || unsafe { (*err).code } != PIDFD_PASS {
        return code;
    }

    unsafe {
        (*err).code = code;
        set_msg(err, msg);
    }

    code
}

unsafe fn error_report(err: *mut error, test_name: *const c_char) {
    unsafe {
        match (*err).code {
            PIDFD_ERROR => {
                ksft_exit_fail_msg(c"%s test: Fatal: %s\n".as_ptr(), test_name, (*err).msg.as_ptr());
            }

            PIDFD_FAIL => {
                /* will be: not ok %d # error %s test: %s */
                ksft_test_result_error(c"%s test: %s\n".as_ptr(), test_name, (*err).msg.as_ptr());
            }

            PIDFD_SKIP => {
                /* will be: not ok %d # SKIP %s test: %s */
                ksft_test_result_skip(c"%s test: %s\n".as_ptr(), test_name, (*err).msg.as_ptr());
            }

            PIDFD_XFAIL => {
                ksft_test_result_pass(
                    c"%s test: Expected failure: %s\n".as_ptr(),
                    test_name,
                    (*err).msg.as_ptr(),
                );
            }

            PIDFD_PASS => {
                ksft_test_result_pass(c"%s test: Passed\n".as_ptr(), test_name);
            }

            _ => {
                ksft_exit_fail_msg(
                    c"%s test: Unknown code: %d %s\n".as_ptr(),
                    test_name,
                    (*err).code,
                    (*err).msg.as_ptr(),
                );
            }
        }
    }
}

unsafe fn error_check(err: *mut error, test_name: *const c_char) -> c_int {
    /* In case of error we bail out and terminate the test program */
    if unsafe { (*err).code } == PIDFD_ERROR {
        unsafe { error_report(err, test_name) };
    }

    unsafe { (*err).code }
}

unsafe fn clone_newns(fn_: extern "C" fn(*mut c_void) -> c_int, args: *mut c_void, err: *mut error) -> child {
    static mut FLAGS: c_int = CLONE_PIDFD | CLONE_NEWPID | CLONE_NEWNS | SIGCHLD;
    let mut ret = child {
        stack: ptr::null_mut(),
        pid: 0,
        fd: 0,
    };

    unsafe {
        if (FLAGS & CLONE_NEWUSER) == 0 && geteuid() != 0 {
            FLAGS |= CLONE_NEWUSER;
        }

        ret.stack = mmap(
            ptr::null_mut(),
            CHILD_STACK_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
            -1,
            0,
        ) as *mut c_char;
        if ret.stack as *mut c_void == MAP_FAILED {
            error_set(err, -1, format!("mmap of stack failed (errno {})", errno()));
            return ret;
        }

        // The original C has an __ia64__ conditional using __clone2. Non-ia64
        // uses clone(fn, stack + CHILD_STACK_SIZE, flags, args, &ret.fd).
        ret.pid = clone(
            fn_,
            ret.stack.add(CHILD_STACK_SIZE) as *mut c_void,
            FLAGS,
            args,
            &mut ret.fd as *mut c_int,
        );

        if ret.pid < 0 {
            error_set(
                err,
                PIDFD_ERROR,
                format!("clone failed (ret {}, errno {})", ret.fd, errno()),
            );
            return ret;
        }

        ksft_print_msg(c"New child: %d, fd: %d\n".as_ptr(), ret.pid, ret.fd);
    }

    ret
}

unsafe fn child_close(child: *mut child) {
    unsafe {
        close((*child).fd);
    }
}

unsafe fn child_join(child: *mut child, err: *mut error) -> c_int {
    let mut r: c_int;

    unsafe {
        r = wait_for_pid((*child).pid);
        if r < 0 {
            error_set(
                err,
                PIDFD_ERROR,
                format!("waitpid failed (ret {}, errno {})", r, errno()),
            );
        } else if r > 0 {
            error_set(err, r, format!("child {} reported: {}", (*child).pid, r));
        }

        if munmap((*child).stack as *mut c_void, CHILD_STACK_SIZE) != 0 {
            error_set(err, -1, format!("munmap of child stack failed (errno {})", errno()));
            r = -1;
        }

        ksft_print_msg(c"waitpid WEXITSTATUS=%d\n".as_ptr(), r);
    }
    r
}

unsafe fn child_join_close(child: *mut child, err: *mut error) -> c_int {
    unsafe {
        child_close(child);
        child_join(child, err)
    }
}

unsafe fn trim_newline(str_: *mut c_char) {
    let pos = unsafe { strrchr(str_, '\n' as c_int) };

    if !pos.is_null() {
        unsafe {
            *pos = '\0' as c_char;
        }
    }
}

unsafe fn verify_fdinfo(
    pidfd: c_int,
    err: *mut error,
    prefix: *const c_char,
    prefix_len: usize,
    expect: String,
) -> c_int {
    let mut buffer = [0 as c_char; 512];
    let mut path = [0 as c_char; 512];
    let mut line: *mut c_char = ptr::null_mut();
    let mut n: usize = 0;
    let mut found: c_int = 0;
    let mut r: c_int;

    unsafe {
        assert!(expect.as_bytes().len() < buffer.len());
        ptr::copy_nonoverlapping(expect.as_ptr() as *const c_char, buffer.as_mut_ptr(), expect.as_bytes().len());

        let path_string = c_string(format!("/proc/self/fdinfo/{}", pidfd));
        assert!(strlen(path_string.as_ptr()) < path.len());
        ptr::copy_nonoverlapping(path_string.as_ptr(), path.as_mut_ptr(), strlen(path_string.as_ptr()) + 1);
        let f = fopen(path.as_ptr(), c"re".as_ptr());
        if f.is_null() {
            return error_set(err, PIDFD_ERROR, format!("fdinfo open failed for {}", pidfd));
        }

        while getline(&mut line as *mut *mut c_char, &mut n as *mut usize, f) != -1 {
            let val: *mut c_char;

            if strncmp(line, prefix, prefix_len) != 0 {
                continue;
            }

            found = 1;

            val = line.add(prefix_len);
            r = strcmp(val, buffer.as_ptr());
            if r != 0 {
                trim_newline(line);
                trim_newline(buffer.as_mut_ptr());
                error_set(
                    err,
                    PIDFD_FAIL,
                    format!(
                        "{} '{}' != '{}'",
                        CStr::from_ptr(prefix).to_string_lossy(),
                        CStr::from_ptr(val).to_string_lossy(),
                        CStr::from_ptr(buffer.as_ptr()).to_string_lossy()
                    ),
                );
            }
            break;
        }

        free(line as *mut c_void);
        fclose(f);

        if found == 0 {
            return error_set(
                err,
                PIDFD_FAIL,
                format!(
                    "{} not found for fd {}",
                    CStr::from_ptr(prefix).to_string_lossy(),
                    pidfd
                ),
            );
        }
    }

    PIDFD_PASS
}

extern "C" fn child_fdinfo_nspid_test(args: *mut c_void) -> c_int {
    unsafe {
        let mut err = error {
            code: 0,
            msg: [0; 512],
        };
        let pidfd: c_int;
        let mut r: c_int;

        /* if we got no fd for the sibling, we are done */
        if args.is_null() {
            return PIDFD_PASS;
        }

        /* verify that we can not resolve the pidfd for a process
         * in a sibling pid namespace, i.e. a pid namespace it is
         * not in our or a descended namespace
         */
        r = mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null());
        if r < 0 {
            ksft_print_msg(c"Failed to remount / private\n".as_ptr());
            return PIDFD_ERROR;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);
        r = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, ptr::null());
        if r < 0 {
            ksft_print_msg(c"Failed to remount /proc\n".as_ptr());
            return PIDFD_ERROR;
        }

        pidfd = *(args as *mut c_int);
        r = verify_fdinfo(pidfd, &mut err, c"NSpid:".as_ptr(), 6, "\t0\n".to_string());

        if r != PIDFD_PASS {
            ksft_print_msg(c"NSpid fdinfo check failed: %s\n".as_ptr(), err.msg.as_ptr());
        }

        r
    }
}

unsafe fn test_pidfd_fdinfo_nspid() {
    unsafe {
        let mut a: child;
        let mut b: child;
        let mut err = error {
            code: 0,
            msg: [0; 512],
        };
        let test_name = c"pidfd check for NSpid in fdinfo";

        /* Create a new child in a new pid and mount namespace */
        a = clone_newns(child_fdinfo_nspid_test, ptr::null_mut(), &mut err);
        error_check(&mut err, test_name.as_ptr());

        /* Pass the pidfd representing the first child to the
         * second child, which will be in a sibling pid namespace,
         * which means that the fdinfo NSpid entry for the pidfd
         * should only contain '0'.
         */
        b = clone_newns(child_fdinfo_nspid_test, &mut a.fd as *mut c_int as *mut c_void, &mut err);
        error_check(&mut err, test_name.as_ptr());

        /* The children will have pid 1 in the new pid namespace,
         * so the line must be 'NSPid:\t<pid>\t1'.
         */
        verify_fdinfo(a.fd, &mut err, c"NSpid:".as_ptr(), 6, format!("\t{}\t{}\n", a.pid, 1));
        verify_fdinfo(b.fd, &mut err, c"NSpid:".as_ptr(), 6, format!("\t{}\t{}\n", b.pid, 1));

        /* wait for the process, check the exit status and set
         * 'err' accordingly, if it is not already set.
         */
        child_join_close(&mut a, &mut err);
        child_join_close(&mut b, &mut err);

        error_report(&mut err, test_name.as_ptr());
    }
}

unsafe fn test_pidfd_dead_fdinfo() {
    unsafe {
        let mut a: child;
        let mut err = error {
            code: 0,
            msg: [0; 512],
        };
        let test_name = c"pidfd check fdinfo for dead process";

        /* Create a new child in a new pid and mount namespace */
        a = clone_newns(child_fdinfo_nspid_test, ptr::null_mut(), &mut err);
        error_check(&mut err, test_name.as_ptr());
        child_join(&mut a, &mut err);

        verify_fdinfo(a.fd, &mut err, c"Pid:".as_ptr(), 4, "\t-1\n".to_string());
        verify_fdinfo(a.fd, &mut err, c"NSpid:".as_ptr(), 6, "\t-1\n".to_string());
        child_close(&mut a);
        error_report(&mut err, test_name.as_ptr());
    }
}

fn main() {
    unsafe {
        ksft_print_header();
        ksft_set_plan(2);

        test_pidfd_fdinfo_nspid();
        test_pidfd_dead_fdinfo();

        ksft_exit_pass();
    }
}

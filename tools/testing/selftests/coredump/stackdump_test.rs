// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// assert.h, fcntl.h, inttypes.h, libgen.h, limits.h, linux/coredump.h,
// linux/fs.h, linux/limits.h, pthread.h, string.h, sys/mount.h, poll.h,
// sys/epoll.h, sys/resource.h, sys/stat.h, sys/socket.h, sys/un.h, unistd.h
//
// Local dependencies:
// kselftest_harness.h, ../filesystems/wrappers.h, ../pidfd/pidfd.h,
// coredump_test.h

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const STACKDUMP_FILE: &[u8] = b"stack_values\0";
const STACKDUMP_SCRIPT: &[u8] = b"stackdump\0";

// #ifndef PAGE_SIZE
const PAGE_SIZE: usize = 4096;

type FILE = c_void;
type pid_t = c_int;
type size_t = usize;

const ESRCH: c_int = 3;
const SIGTERM: c_int = 15;
const NUM_THREAD_SPAWN: c_int = 16;

#[repr(C)]
pub struct CoredumpFixture {
    pub pid_coredump_server: pid_t,
    pub fd_tmpfs_detached: c_int,
    pub original_core_pattern: [c_char; PAGE_SIZE],
}

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> isize;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn fork() -> pid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn free(ptr: *mut c_void);

    static mut stderr: *mut FILE;

    fn create_detached_tmpfs() -> c_int;
    fn crashing_child() -> !;
}

type c_uint = u32;
type c_ulonglong = u64;

#[inline]
fn wifsignaled(status: c_int) -> bool {
    ((status & 0x7f) + 1) >= 2
}

#[inline]
fn wcoredump(status: c_int) -> bool {
    (status & 0x80) != 0
}

#[inline]
unsafe fn coredump_fixture_setup(self_: *mut CoredumpFixture) {
    let mut file: *mut FILE;
    let mut ret: c_int;

    unsafe {
        (*self_).pid_coredump_server = -ESRCH;
        (*self_).fd_tmpfs_detached = -1;
        file = fopen(c"/proc/sys/kernel/core_pattern".as_ptr(), c"r".as_ptr());
        assert_ne!(core::ptr::null_mut::<FILE>(), file);

        ret = fread(
            (*self_).original_core_pattern.as_mut_ptr() as *mut c_void,
            1,
            (*self_).original_core_pattern.len(),
            file,
        ) as c_int;
        assert!(ret != 0 || feof(file) != 0);
        assert!(ret < (*self_).original_core_pattern.len() as c_int);

        (*self_).original_core_pattern[ret as usize] = 0;
        (*self_).fd_tmpfs_detached = create_detached_tmpfs();
        assert!((*self_).fd_tmpfs_detached >= 0);

        ret = fclose(file);
        assert_eq!(0, ret);
    }
}

#[inline]
unsafe fn coredump_fixture_teardown(self_: *mut CoredumpFixture) {
    let mut reason: *const c_char;
    let mut file: *mut FILE;
    let mut ret: c_int;
    let mut status: c_int = 0;

    unsafe {
        unlink(STACKDUMP_FILE.as_ptr() as *const c_char);

        if (*self_).pid_coredump_server > 0 {
            kill((*self_).pid_coredump_server, SIGTERM);
            waitpid((*self_).pid_coredump_server, &mut status, 0);
        }
        unlink(c"/tmp/coredump.file".as_ptr());
        unlink(c"/tmp/coredump.socket".as_ptr());

        file = fopen(c"/proc/sys/kernel/core_pattern".as_ptr(), c"w".as_ptr());
        if file.is_null() {
            reason = c"Unable to open core_pattern".as_ptr();
            goto_fail(reason);
            return;
        }

        ret = fprintf(file, c"%s".as_ptr(), (*self_).original_core_pattern.as_ptr());
        if ret < 0 {
            reason = c"Unable to write to core_pattern".as_ptr();
            goto_fail(reason);
            return;
        }

        ret = fclose(file);
        if ret != 0 {
            reason = c"Unable to close core_pattern".as_ptr();
            goto_fail(reason);
            return;
        }

        if (*self_).fd_tmpfs_detached >= 0 {
            ret = close((*self_).fd_tmpfs_detached);
            if ret < 0 {
                reason = c"Unable to close detached tmpfs".as_ptr();
                goto_fail(reason);
                return;
            }
            (*self_).fd_tmpfs_detached = -1;
        }
    }
}

#[inline]
unsafe fn goto_fail(reason: *const c_char) {
    // This should never happen
    unsafe {
        fprintf(
            stderr,
            c"Failed to cleanup stackdump test: %s\n".as_ptr(),
            reason,
        );
    }
}

// TEST_F_TIMEOUT(coredump, stackdump, 120)
unsafe fn coredump_stackdump_test() {
    let mut stack: c_ulonglong;
    let mut test_dir: *mut c_char;
    let mut line: *mut c_char;
    let mut line_length: size_t = 0;
    let mut buf: [c_char; PAGE_SIZE] = [0; PAGE_SIZE];
    let mut ret: c_int;
    let mut i: c_int;
    let mut status: c_int = 0;
    let mut file: *mut FILE;
    let mut pid: pid_t;

    unsafe {
        /*
         * Step 1: Setup core_pattern so that the stackdump script is executed when the child
         * process crashes
         */
        ret = readlink(
            c"/proc/self/exe".as_ptr(),
            buf.as_mut_ptr(),
            buf.len(),
        ) as c_int;
        assert_ne!(-1, ret);
        assert!(ret < buf.len() as c_int);
        buf[ret as usize] = 0;

        test_dir = dirname(buf.as_mut_ptr());

        file = fopen(c"/proc/sys/kernel/core_pattern".as_ptr(), c"w".as_ptr());
        assert_ne!(core::ptr::null_mut::<FILE>(), file);

        ret = fprintf(
            file,
            c"|%1$s/%2$s %%P %1$s/%3$s".as_ptr(),
            test_dir,
            STACKDUMP_SCRIPT.as_ptr() as *const c_char,
            STACKDUMP_FILE.as_ptr() as *const c_char,
        );
        assert!(0 < ret);

        ret = fclose(file);
        assert_eq!(0, ret);

        /* Step 2: Create a process who spawns some threads then crashes */
        pid = fork();
        assert!(pid >= 0);
        if pid == 0 {
            crashing_child();
        }

        /*
         * Step 3: Wait for the stackdump script to write the stack pointers to the stackdump file
         */
        waitpid(pid, &mut status, 0);
        assert!(wifsignaled(status));
        assert!(wcoredump(status));

        i = 0;
        loop {
            if !(i < 10) {
                break;
            }
            file = fopen(STACKDUMP_FILE.as_ptr() as *const c_char, c"r".as_ptr());
            if !file.is_null() {
                break;
            }
            sleep(1);
            i += 1;
        }
        assert_ne!(file, core::ptr::null_mut());

        /* Step 4: Make sure all stack pointer values are non-zero */
        line = core::ptr::null_mut();
        i = 0;
        while getline(&mut line, &mut line_length, file) != -1 {
            stack = strtoull(line, core::ptr::null_mut(), 10);
            assert_ne!(stack, 0);
            i += 1;
        }
        free(line as *mut c_void);

        assert_eq!(i, 1 + NUM_THREAD_SPAWN);

        fclose(file);
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

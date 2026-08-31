/* SPDX-License-Identifier: GPL-2.0 */
/* _GNU_SOURCE */
/* Dependencies translated from C includes:
 * assert.h, errno.h, fcntl.h, linux/types.h, sched.h, signal.h, stdio.h,
 * stdlib.h, string.h, syscall.h, sys/mount.h, sys/wait.h, unistd.h,
 * "kselftest_harness.h", "../pidfd/pidfd.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

const ENOMEM: c_int = 12;
const O_RDWR: c_int = 0o2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NOCTTY: c_int = 0o400;
const SIGCHLD: c_int = 17;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const MNT_DETACH: c_int = 2;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWPID: c_int = 0x20000000;
const EXIT_SUCCESS: c_int = 0;

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn clone(
        fn_: Option<extern "C" fn(*mut c_void) -> c_int>,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;
    #[cfg(target_arch = "ia64")]
    fn __clone2(
        fn_: Option<extern "C" fn(*mut c_void) -> c_int>,
        child_stack_base: *mut c_void,
        stack_size: size_t,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;

    fn wait_for_pid(pid: pid_t) -> c_int;
}

/*
 * The kernel computes the minimum allowed pid_max as:
 *   max(RESERVED_PIDS + 1, PIDS_PER_CPU_MIN * num_possible_cpus())
 * Mirror that here so the test values are always valid.
 *
 * Note: glibc's get_nprocs_conf() returns the number of *configured*
 * (present) CPUs, not *possible* CPUs.  The kernel uses
 * num_possible_cpus() which corresponds to /sys/devices/system/cpu/possible.
 * These can differ significantly (e.g. 16 configured vs 128 possible).
 */
const RESERVED_PIDS: c_int = 300;
const PIDS_PER_CPU_MIN: c_int = 8;

/* Count CPUs from a range list like "0-31" or "0-15,32-47". */
unsafe fn num_possible_cpus() -> c_int {
    let mut count: c_int = 0;
    let mut lo: c_int = 0;
    let mut hi: c_int = 0;

    let f = unsafe { fopen(c"/sys/devices/system/cpu/possible".as_ptr(), c"r".as_ptr()) };
    if f.is_null() {
        return 0;
    }

    while unsafe { fscanf(f, c"%d".as_ptr(), &mut lo as *mut c_int) } == 1 {
        if unsafe { fscanf(f, c"-%d".as_ptr(), &mut hi as *mut c_int) } == 1 {
            count += hi - lo + 1;
        } else {
            count += 1;
        }
        /* skip comma separator */
        unsafe {
            fscanf(f, c",".as_ptr());
        }
    }

    unsafe {
        fclose(f);
    }
    count
}

unsafe fn pid_min() -> c_int {
    let cpu_min: c_int = PIDS_PER_CPU_MIN * unsafe { num_possible_cpus() };

    if cpu_min > (RESERVED_PIDS + 1) {
        cpu_min
    } else {
        RESERVED_PIDS + 1
    }
}

/*
 * Outer and inner pid_max limits used by the tests.  The outer limit is
 * the more restrictive ancestor; the inner limit is set higher in a
 * nested namespace but must still be capped by the outer limit.
 * Both are derived from the kernel's minimum so they are always writable.
 *
 * Global so that clone callbacks can access them without parameter plumbing.
 */
static mut outer_limit: c_int = 0;
static mut inner_limit: c_int = 0;

unsafe fn write_int_to_fd(fd: c_int, val: c_int) -> c_int {
    let mut buf = [0 as c_char; 12];
    let len: c_int = unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%d".as_ptr(),
            val,
        )
    };

    unsafe { write(fd, buf.as_ptr() as *const c_void, len as size_t) as c_int }
}

const __STACK_SIZE: usize = 8 * 1024 * 1024;

unsafe fn do_clone(
    fn_: extern "C" fn(*mut c_void) -> c_int,
    arg: *mut c_void,
    flags: c_int,
) -> pid_t {
    let stack = unsafe { malloc(__STACK_SIZE) as *mut c_char };
    if stack.is_null() {
        return -ENOMEM;
    }

    #[cfg(target_arch = "ia64")]
    let ret = unsafe {
        __clone2(
            Some(fn_),
            stack as *mut c_void,
            __STACK_SIZE,
            flags | SIGCHLD,
            arg,
        )
    };

    #[cfg(not(target_arch = "ia64"))]
    let ret = unsafe {
        clone(
            Some(fn_),
            stack.add(__STACK_SIZE) as *mut c_void,
            flags | SIGCHLD,
            arg,
        )
    };

    unsafe {
        free(stack as *mut c_void);
    }
    ret
}

extern "C" fn pid_max_cb(_data: *mut c_void) -> c_int {
    unsafe {
        let mut ret: c_int;
        let mut pid: pid_t;

        ret = mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_PRIVATE | MS_REC, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to make rootfs private mount\n".as_ptr());
            return -1;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);

        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to mount proc\n".as_ptr());
            return -1;
        }

        let fd = open(c"/proc/sys/kernel/pid_max".as_ptr(), O_RDWR | O_CLOEXEC | O_NOCTTY);
        if fd < 0 {
            fprintf(stderr, c"%m - Failed to open pid_max\n".as_ptr());
            return -1;
        }

        ret = write_int_to_fd(fd, inner_limit);
        if ret < 0 {
            fprintf(stderr, c"%m - Failed to write pid_max\n".as_ptr());
            return -1;
        }

        for _i in 0..(inner_limit + 1) {
            pid = fork();
            if pid == 0 {
                exit(EXIT_SUCCESS);
            }
            wait_for_pid(pid);
            if pid > inner_limit {
                fprintf(stderr, c"Managed to create pid number beyond limit\n".as_ptr());
                return -1;
            }
        }

        0
    }
}

extern "C" fn pid_max_nested_inner(_data: *mut c_void) -> c_int {
    unsafe {
        let fret: c_int = -1;
        let mut pids: [pid_t; 2] = [0; 2];
        let mut ret: c_int;

        ret = mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_PRIVATE | MS_REC, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to make rootfs private mount\n".as_ptr());
            return fret;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);

        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to mount proc\n".as_ptr());
            return fret;
        }

        let fd = open(c"/proc/sys/kernel/pid_max".as_ptr(), O_RDWR | O_CLOEXEC | O_NOCTTY);
        if fd < 0 {
            fprintf(stderr, c"%m - Failed to open pid_max\n".as_ptr());
            return fret;
        }

        ret = write_int_to_fd(fd, inner_limit);
        close(fd);
        if ret < 0 {
            fprintf(stderr, c"%m - Failed to write pid_max\n".as_ptr());
            return fret;
        }

        pids[0] = fork();
        if pids[0] < 0 {
            fprintf(stderr, c"Failed to create first new process\n".as_ptr());
            return fret;
        }

        if pids[0] == 0 {
            exit(EXIT_SUCCESS);
        }

        pids[1] = fork();
        wait_for_pid(pids[0]);
        if pids[1] >= 0 {
            if pids[1] == 0 {
                exit(EXIT_SUCCESS);
            }
            wait_for_pid(pids[1]);

            fprintf(stderr, c"Managed to create process even though ancestor pid namespace had a limit\n".as_ptr());
            return fret;
        }

        /* Now make sure that we wrap pids at outer_limit. */
        for _i in 0..(inner_limit + 10) {
            let pid: pid_t = fork();
            if pid < 0 {
                return fret;
            }

            if pid == 0 {
                exit(EXIT_SUCCESS);
            }

            wait_for_pid(pid);
            if pid >= inner_limit {
                fprintf(
                    stderr,
                    c"Managed to create process with pid %d beyond configured limit\n".as_ptr(),
                    pid,
                );
                return fret;
            }
        }

        0
    }
}

extern "C" fn pid_max_nested_outer(_data: *mut c_void) -> c_int {
    unsafe {
        let mut fret: c_int = -1;
        let mut nr_procs: c_int = 0;
        let mut ret: c_int;
        let mut pid: pid_t;

        let pids = malloc((outer_limit as size_t) * core::mem::size_of::<pid_t>()) as *mut pid_t;
        if pids.is_null() {
            return -1;
        }

        ret = mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_PRIVATE | MS_REC, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to make rootfs private mount\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);

        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to mount proc\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        let fd = open(c"/proc/sys/kernel/pid_max".as_ptr(), O_RDWR | O_CLOEXEC | O_NOCTTY);
        if fd < 0 {
            fprintf(stderr, c"%m - Failed to open pid_max\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        ret = write_int_to_fd(fd, outer_limit);
        close(fd);
        if ret < 0 {
            fprintf(stderr, c"%m - Failed to write pid_max\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        /*
         * Create (outer_limit - 4) processes. This leaves room for
         * do_clone() and one more. So creating another process needs
         * to fail.
         */
        while nr_procs < outer_limit - 4 {
            pid = fork();
            if pid < 0 {
                break;
            }

            if pid == 0 {
                exit(EXIT_SUCCESS);
            }

            *pids.add(nr_procs as usize) = pid;
            nr_procs += 1;
        }

        if nr_procs == outer_limit - 4 {
            pid = do_clone(pid_max_nested_inner, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
            if pid < 0 {
                fprintf(stderr, c"%m - Failed to clone nested pidns\n".as_ptr());
            } else if wait_for_pid(pid) != 0 {
                fprintf(stderr, c"%m - Nested pid_max failed\n".as_ptr());
            } else {
                fret = 0;
            }
        }

        for i in 0..nr_procs {
            wait_for_pid(*pids.add(i as usize));
        }

        free(pids as *mut c_void);
        fret
    }
}

extern "C" fn pid_max_nested_limit_inner(_data: *mut c_void) -> c_int {
    unsafe {
        let mut fret: c_int = -1;
        let mut nr_procs: c_int = 0;
        let mut ret: c_int;
        let mut pid: pid_t;

        let pids = malloc((inner_limit as size_t) * core::mem::size_of::<pid_t>()) as *mut pid_t;
        if pids.is_null() {
            return -1;
        }

        ret = mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_PRIVATE | MS_REC, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to make rootfs private mount\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);

        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to mount proc\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        let fd = open(c"/proc/sys/kernel/pid_max".as_ptr(), O_RDWR | O_CLOEXEC | O_NOCTTY);
        if fd < 0 {
            fprintf(stderr, c"%m - Failed to open pid_max\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        ret = write_int_to_fd(fd, inner_limit);
        close(fd);
        if ret < 0 {
            fprintf(stderr, c"%m - Failed to write pid_max\n".as_ptr());
            free(pids as *mut c_void);
            return fret;
        }

        while nr_procs < inner_limit {
            pid = fork();
            if pid < 0 {
                break;
            }

            if pid == 0 {
                exit(EXIT_SUCCESS);
            }

            *pids.add(nr_procs as usize) = pid;
            nr_procs += 1;
        }

        if nr_procs >= outer_limit {
            fprintf(stderr, c"Managed to create processes beyond the configured outer limit\n".as_ptr());
        } else {
            fret = 0;
        }

        for i in 0..nr_procs {
            wait_for_pid(*pids.add(i as usize));
        }

        free(pids as *mut c_void);
        fret
    }
}

extern "C" fn pid_max_nested_limit_outer(_data: *mut c_void) -> c_int {
    unsafe {
        let mut ret: c_int;
        let pid: pid_t;

        ret = mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_PRIVATE | MS_REC, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to make rootfs private mount\n".as_ptr());
            return -1;
        }

        umount2(c"/proc".as_ptr(), MNT_DETACH);

        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, core::ptr::null());
        if ret != 0 {
            fprintf(stderr, c"%m - Failed to mount proc\n".as_ptr());
            return -1;
        }

        let fd = open(c"/proc/sys/kernel/pid_max".as_ptr(), O_RDWR | O_CLOEXEC | O_NOCTTY);
        if fd < 0 {
            fprintf(stderr, c"%m - Failed to open pid_max\n".as_ptr());
            return -1;
        }

        ret = write_int_to_fd(fd, outer_limit);
        close(fd);
        if ret < 0 {
            fprintf(stderr, c"%m - Failed to write pid_max\n".as_ptr());
            return -1;
        }

        pid = do_clone(pid_max_nested_limit_inner, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
        if pid < 0 {
            fprintf(stderr, c"%m - Failed to clone nested pidns\n".as_ptr());
            return -1;
        }

        if wait_for_pid(pid) != 0 {
            fprintf(stderr, c"%m - Nested pid_max failed\n".as_ptr());
            return -1;
        }

        0
    }
}

/* FIXTURE(pid_max) { int dummy; }; */
#[repr(C)]
struct pid_max {
    dummy: c_int,
}

/* FIXTURE_SETUP(pid_max) */
unsafe fn pid_max_setup(_self: *mut pid_max) {
    let min: c_int = unsafe { pid_min() };

    unsafe {
        outer_limit = min + 100;
        inner_limit = min + 200;
    }
}

/* FIXTURE_TEARDOWN(pid_max) */
unsafe fn pid_max_teardown(_self: *mut pid_max) {}

/* TEST_F(pid_max, simple) */
unsafe fn pid_max_simple(_self: *mut pid_max) {
    let pid: pid_t = unsafe {
        do_clone(
            pid_max_cb,
            core::ptr::null_mut(),
            CLONE_NEWPID | CLONE_NEWNS,
        )
    };
    assert!(pid > 0);
    assert_eq!(0, unsafe { wait_for_pid(pid) });
}

/* TEST_F(pid_max, nested_limit) */
unsafe fn pid_max_nested_limit(_self: *mut pid_max) {
    let pid: pid_t = unsafe {
        do_clone(
            pid_max_nested_limit_outer,
            core::ptr::null_mut(),
            CLONE_NEWPID | CLONE_NEWNS,
        )
    };
    assert!(pid > 0);
    assert_eq!(0, unsafe { wait_for_pid(pid) });
}

/* TEST_F(pid_max, nested) */
unsafe fn pid_max_nested(_self: *mut pid_max) {
    let pid: pid_t = unsafe {
        do_clone(
            pid_max_nested_outer,
            core::ptr::null_mut(),
            CLONE_NEWPID | CLONE_NEWNS,
        )
    };
    assert!(pid > 0);
    assert_eq!(0, unsafe { wait_for_pid(pid) });
}

/* TEST_HARNESS_MAIN */

// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/send_signal.c.
// Dependencies originally came from:
// <test_progs.h>, <sys/time.h>, <sys/resource.h>,
// "test_send_signal_kern.skel.h", and "io_helpers.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const SIGUSR1: c_int = 10;
const SIGKILL: c_int = 9;
const SIG_ERR: sighandler_t = unsafe { core::mem::transmute::<isize, sighandler_t>(-1) };
const SA_RESTART: c_int = 0x10000000;
const SA_SIGINFO: c_int = 4;
const PRIO_PROCESS: c_int = 0;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const __NR_perf_event_open: c_long = 298;
const EAGAIN: c_int = 11;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_value: sigval,
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: c_ulong,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct test_send_signal_kern {
    pub links: test_send_signal_kern_links,
    pub progs: test_send_signal_kern_progs,
    pub bss: *mut test_send_signal_kern_bss,
}

#[repr(C)]
pub struct test_send_signal_kern_links {
    pub send_signal_perf: *mut c_void,
}

#[repr(C)]
pub struct test_send_signal_kern_progs {
    pub send_signal_perf: *mut c_void,
}

#[repr(C)]
pub struct test_send_signal_kern_bss {
    pub signal_thread: bool,
    pub sig: c_int,
    pub target_pid: pid_t,
    pub pid: pid_t,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn pipe(pipefd: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn getpriority(which: c_int, who: c_int) -> c_int;
    fn setpriority(which: c_int, who: c_int, prio: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn sleep(seconds: c_int) -> c_int;
    fn usleep(usec: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn test_send_signal_kern__open_and_load() -> *mut test_send_signal_kern;
    fn test_send_signal_kern__attach(skel: *mut test_send_signal_kern) -> c_int;
    fn test_send_signal_kern__destroy(skel: *mut test_send_signal_kern);
    fn bpf_program__attach_perf_event(prog: *mut c_void, pfd: c_int) -> *mut c_void;

    fn read_with_timeout(fd: c_int, buf: *mut c_void, count: usize, timeout_ms: c_int) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(res: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_GT(res: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_EQ(res: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_NEQ(res: c_long, expected: c_long, name: *const c_char) -> bool;
}

static mut sigusr1_received: c_int = 0;

unsafe extern "C" fn sigusr1_handler(_signum: c_int) {
    unsafe {
        sigusr1_received = 8;
    }
}

unsafe extern "C" fn sigusr1_siginfo_handler(_s: c_int, i: *mut siginfo_t, _v: *mut c_void) {
    unsafe {
        sigusr1_received = (*i).si_value.sival_ptr as isize as c_int;
    }
}

unsafe fn test_send_signal_common(
    attr: *mut perf_event_attr,
    signal_thread: bool,
    remote: bool,
) {
    unsafe {
        let mut skel: *mut test_send_signal_kern;
        let mut sa: sigaction = core::mem::zeroed();
        let mut pipe_c2p: [c_int; 2] = [0; 2];
        let mut pipe_p2c: [c_int; 2] = [0; 2];
        let mut err: c_int = -1;
        let mut pmu_fd: c_int = -1;
        let mut j: c_int = 0;
        let mut retry_count: c_int;
        let mut buf: [c_char; 256] = [0; 256];
        let pid: pid_t;
        let mut old_prio: c_int = 0;

        if !ASSERT_OK(pipe(pipe_c2p.as_mut_ptr()), c"pipe_c2p".as_ptr()) {
            return;
        }

        if !ASSERT_OK(pipe(pipe_p2c.as_mut_ptr()), c"pipe_p2c".as_ptr()) {
            close(pipe_c2p[0]);
            close(pipe_c2p[1]);
            return;
        }

        pid = fork();
        if !ASSERT_GE(pid as c_long, 0, c"fork".as_ptr()) {
            close(pipe_c2p[0]);
            close(pipe_c2p[1]);
            close(pipe_p2c[0]);
            close(pipe_p2c[1]);
            return;
        }

        if pid == 0 {
            /* install signal handler and notify parent */
            if remote {
                sa.sa_sigaction = Some(sigusr1_siginfo_handler);
                sa.sa_flags = SA_RESTART | SA_SIGINFO;
                ASSERT_NEQ(
                    sigaction(SIGUSR1, &sa, ptr::null_mut()) as c_long,
                    -1,
                    c"sigaction".as_ptr(),
                );
            } else {
                ASSERT_NEQ(
                    signal(SIGUSR1, Some(sigusr1_handler)) as isize as c_long,
                    SIG_ERR as isize as c_long,
                    c"signal".as_ptr(),
                );
            }

            close(pipe_c2p[0]); /* close read */
            close(pipe_p2c[1]); /* close write */

            /* boost with a high priority so we got a higher chance
             * that if an interrupt happens, the underlying task
             * is this process.
             */
            if !remote {
                errno = 0;
                old_prio = getpriority(PRIO_PROCESS, 0);
                ASSERT_OK(errno, c"getpriority".as_ptr());
                ASSERT_OK(setpriority(PRIO_PROCESS, 0, -20), c"setpriority".as_ptr());
            }

            /* notify parent signal handler is installed */
            ASSERT_EQ(
                write(pipe_c2p[1], buf.as_ptr() as *const c_void, 1) as c_long,
                1,
                c"pipe_write".as_ptr(),
            );

            /* make sure parent enabled bpf program to send_signal */
            ASSERT_EQ(
                read(pipe_p2c[0], buf.as_mut_ptr() as *mut c_void, 1) as c_long,
                1,
                c"pipe_read".as_ptr(),
            );

            /* wait a little for signal handler */
            let mut i: c_int = 0;
            while i < 1000000000 && sigusr1_received == 0 {
                let cur_j = core::ptr::read_volatile(&j);
                core::ptr::write_volatile(&mut j, cur_j / (i + cur_j + 1));
                if remote {
                    sleep(1);
                } else if attr.is_null() {
                    /* trigger the nanosleep tracepoint program. */
                    usleep(1);
                }
                i += 1;
            }

            buf[0] = sigusr1_received as c_char;

            ASSERT_EQ(sigusr1_received as c_long, 8, c"sigusr1_received".as_ptr());
            ASSERT_EQ(
                write(pipe_c2p[1], buf.as_ptr() as *const c_void, 1) as c_long,
                1,
                c"pipe_write".as_ptr(),
            );

            /* wait for parent notification and exit */
            ASSERT_EQ(
                read(pipe_p2c[0], buf.as_mut_ptr() as *mut c_void, 1) as c_long,
                1,
                c"pipe_read".as_ptr(),
            );

            /* restore the old priority */
            if !remote {
                ASSERT_OK(setpriority(PRIO_PROCESS, 0, old_prio), c"setpriority".as_ptr());
            }

            close(pipe_c2p[1]);
            close(pipe_p2c[0]);
            exit(0);
        }

        close(pipe_c2p[1]); /* close write */
        close(pipe_p2c[0]); /* close read */

        skel = test_send_signal_kern__open_and_load();
        if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
            close(pipe_c2p[0]);
            close(pipe_p2c[1]);
            /*
             * Child is either about to exit cleanly or stuck in case of errors.
             * Nudge it to exit.
             */
            kill(pid, SIGKILL);
            wait(ptr::null_mut());
            return;
        }

        'destroy_skel: {
            /* boost with a high priority so we got a higher chance
             * that if an interrupt happens, the underlying task
             * is this process.
             */
            if remote {
                errno = 0;
                old_prio = getpriority(PRIO_PROCESS, 0);
                ASSERT_OK(errno, c"getpriority".as_ptr());
                ASSERT_OK(setpriority(PRIO_PROCESS, 0, -20), c"setpriority".as_ptr());
            }

            if attr.is_null() {
                err = test_send_signal_kern__attach(skel);
                if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
                    err = -1;
                    break 'destroy_skel;
                }
            } else {
                if !remote {
                    pmu_fd = syscall(
                        __NR_perf_event_open,
                        attr,
                        pid,
                        -1i32, /* cpu */
                        -1i32, /* group id */
                        0i32,  /* flags */
                    ) as c_int;
                } else {
                    pmu_fd = syscall(
                        __NR_perf_event_open,
                        attr,
                        getpid(),
                        -1i32, /* cpu */
                        -1i32, /* group id */
                        0i32,  /* flags */
                    ) as c_int;
                }
                if !ASSERT_GE(pmu_fd as c_long, 0, c"perf_event_open".as_ptr()) {
                    err = -1;
                    break 'destroy_skel;
                }

                (*skel).links.send_signal_perf =
                    bpf_program__attach_perf_event((*skel).progs.send_signal_perf, pmu_fd);
                if !ASSERT_OK_PTR(
                    (*skel).links.send_signal_perf,
                    c"attach_perf_event".as_ptr(),
                ) {
                    close(pmu_fd);
                    break 'destroy_skel;
                }
            }

            'disable_pmu: {
                /* wait until child signal handler installed */
                ASSERT_EQ(
                    read(pipe_c2p[0], buf.as_mut_ptr() as *mut c_void, 1) as c_long,
                    1,
                    c"pipe_read".as_ptr(),
                );

                /* trigger the bpf send_signal */
                (*(*skel).bss).signal_thread = signal_thread;
                (*(*skel).bss).sig = SIGUSR1;
                if !remote {
                    (*(*skel).bss).target_pid = 0;
                    (*(*skel).bss).pid = pid;
                } else {
                    (*(*skel).bss).target_pid = pid;
                    (*(*skel).bss).pid = getpid();
                }

                /* notify child that bpf program can send_signal now */
                ASSERT_EQ(
                    write(pipe_p2c[1], buf.as_ptr() as *const c_void, 1) as c_long,
                    1,
                    c"pipe_write".as_ptr(),
                );

                retry_count = 0;
                loop {
                    /* For the remote test, the BPF program is triggered from this
                     * process but the other process/thread is signaled.
                     */
                    if remote {
                        if attr.is_null() {
                            let mut i: c_int = 0;
                            while i < 10 {
                                usleep(1);
                                i += 1;
                            }
                        } else {
                            let mut i: c_int = 0;
                            while i < 100000000 {
                                let cur_j = core::ptr::read_volatile(&j);
                                core::ptr::write_volatile(&mut j, cur_j / (i + 1));
                                i += 1;
                            }
                        }
                    }
                    /* wait for result */
                    err = read_with_timeout(pipe_c2p[0], buf.as_mut_ptr() as *mut c_void, 1, 100);
                    if err == -EAGAIN && {
                        let old = retry_count;
                        retry_count += 1;
                        old < 10000
                    } {
                        continue;
                    }
                    break;
                }
                if !ASSERT_GE(err as c_long, 0, c"reading pipe".as_ptr()) {
                    break 'disable_pmu;
                }
                if !ASSERT_GT(err as c_long, 0, c"reading pipe error: size 0".as_ptr()) {
                    err = -1;
                    break 'disable_pmu;
                }

                ASSERT_EQ(buf[0] as c_long, 8, c"incorrect result".as_ptr());

                /* notify child safe to exit */
                ASSERT_EQ(
                    write(pipe_p2c[1], buf.as_ptr() as *const c_void, 1) as c_long,
                    1,
                    c"pipe_write".as_ptr(),
                );
            }

            close(pmu_fd);
        }

        test_send_signal_kern__destroy(skel);
        /* restore the old priority */
        if remote {
            ASSERT_OK(setpriority(PRIO_PROCESS, 0, old_prio), c"setpriority".as_ptr());
        }
        close(pipe_c2p[0]);
        close(pipe_p2c[1]);
        /*
         * Child is either about to exit cleanly or stuck in case of errors.
         * Nudge it to exit.
         */
        kill(pid, SIGKILL);
        wait(ptr::null_mut());
    }
}

unsafe fn test_send_signal_tracepoint(signal_thread: bool, remote: bool) {
    unsafe {
        test_send_signal_common(ptr::null_mut(), signal_thread, remote);
    }
}

unsafe fn test_send_signal_perf(signal_thread: bool, remote: bool) {
    unsafe {
        let mut attr = perf_event_attr {
            type_: PERF_TYPE_SOFTWARE,
            size: 0,
            config: PERF_COUNT_SW_CPU_CLOCK,
            sample_period_or_freq: 1000,
            sample_type: 0,
            read_format: 0,
            flags: 1,
        };

        test_send_signal_common(&mut attr, signal_thread, remote);
    }
}

unsafe fn test_send_signal_nmi(signal_thread: bool, remote: bool) {
    unsafe {
        let mut attr = perf_event_attr {
            type_: PERF_TYPE_HARDWARE,
            size: 0,
            config: PERF_COUNT_HW_CPU_CYCLES,
            sample_period_or_freq: 1000,
            sample_type: 0,
            read_format: 0,
            flags: 1,
        };
        let pmu_fd: c_int;

        /* Some setups (e.g. virtual machines) might run with hardware
         * perf events disabled. If this is the case, skip this test.
         */
        pmu_fd = syscall(
            __NR_perf_event_open,
            &mut attr as *mut perf_event_attr,
            0i32,  /* pid */
            -1i32, /* cpu */
            -1i32, /* group_fd */
            0i32,  /* flags */
        ) as c_int;
        if pmu_fd == -1 {
            if errno == ENOENT || errno == EOPNOTSUPP {
                printf(
                    c"%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n".as_ptr(),
                    c"test_send_signal_nmi".as_ptr(),
                );
                test__skip();
                return;
            }
            /* Let the test fail with a more informative message */
        } else {
            close(pmu_fd);
        }

        test_send_signal_common(&mut attr, signal_thread, remote);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_send_signal() {
    unsafe {
        if test__start_subtest(c"send_signal_tracepoint".as_ptr()) {
            test_send_signal_tracepoint(false, false);
        }
        if test__start_subtest(c"send_signal_perf".as_ptr()) {
            test_send_signal_perf(false, false);
        }
        if test__start_subtest(c"send_signal_nmi".as_ptr()) {
            test_send_signal_nmi(false, false);
        }
        if test__start_subtest(c"send_signal_tracepoint_thread".as_ptr()) {
            test_send_signal_tracepoint(true, false);
        }
        if test__start_subtest(c"send_signal_perf_thread".as_ptr()) {
            test_send_signal_perf(true, false);
        }
        if test__start_subtest(c"send_signal_nmi_thread".as_ptr()) {
            test_send_signal_nmi(true, false);
        }

        /* Signal remote thread and thread group */
        if test__start_subtest(c"send_signal_tracepoint_remote".as_ptr()) {
            test_send_signal_tracepoint(false, true);
        }
        if test__start_subtest(c"send_signal_perf_remote".as_ptr()) {
            test_send_signal_perf(false, true);
        }
        if test__start_subtest(c"send_signal_nmi_remote".as_ptr()) {
            test_send_signal_nmi(false, true);
        }
        if test__start_subtest(c"send_signal_tracepoint_thread_remote".as_ptr()) {
            test_send_signal_tracepoint(true, true);
        }
        if test__start_subtest(c"send_signal_perf_thread_remote".as_ptr()) {
            test_send_signal_perf(true, true);
        }
        if test__start_subtest(c"send_signal_nmi_thread_remote".as_ptr()) {
            test_send_signal_nmi(true, true);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

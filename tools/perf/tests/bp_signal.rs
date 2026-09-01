// SPDX-License-Identifier: GPL-2.0
/*
 * Inspired by breakpoint overflow test done by
 * Vince Weaver <vincent.weaver@maine.edu> for perf_event_tests
 * (git://github.com/deater/perf_event_tests)
 */

/*
 * Powerpc C source needs __SANE_USERSPACE_TYPES__ before <linux/types.h> to
 * select 'int-ll64.h' and avoid compile warnings when printing __u64 with %llu.
 */

/* Dependencies from the original C includes:
 * stdlib.h, stdio.h, unistd.h, string.h, sys/ioctl.h, time.h, fcntl.h,
 * signal.h, sys/mman.h, linux/compiler.h, linux/hw_breakpoint.h,
 * tests.h, debug.h, event.h, parse-events.h, perf-sys.h, cloexec.h.
 */

use core::ffi::{c_int, c_long, c_void};
use core::mem;
use core::ptr;

static mut fd1: c_int = 0;
static mut fd2: c_int = 0;
static mut fd3: c_int = 0;
static mut overflows: c_int = 0;
static mut overflows_2: c_int = 0;

static mut the_var: c_long = 0;

/*
 * Use ASM to ensure watchpoint and breakpoint can be triggered
 * at one instruction.
 */
#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".pushsection .text;",
    ".globl __test_function",
    ".type __test_function, @function;",
    "__test_function:",
    "incq (%rdi)",
    "ret",
    ".popsection",
);

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn __test_function(ptr: *mut c_long);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe extern "C" fn __test_function(ptr: *mut c_long) {
    unsafe {
        *ptr = 0x1234;
    }
}

#[inline(never)]
unsafe extern "C" fn test_function() -> c_int {
    unsafe {
        __test_function(&raw mut the_var);
        the_var += 1;
        time(ptr::null_mut()) as c_int
    }
}

unsafe extern "C" fn sig_handler_2(
    _signum: c_int,
    _oh: *mut siginfo_t,
    _uc: *mut c_void,
) {
    unsafe {
        overflows_2 += 1;
        if overflows_2 > 10 {
            ioctl(fd1, PERF_EVENT_IOC_DISABLE, 0);
            ioctl(fd2, PERF_EVENT_IOC_DISABLE, 0);
            ioctl(fd3, PERF_EVENT_IOC_DISABLE, 0);
        }
    }
}

unsafe extern "C" fn sig_handler(
    _signum: c_int,
    _oh: *mut siginfo_t,
    _uc: *mut c_void,
) {
    unsafe {
        overflows += 1;

        if overflows > 10 {
            /*
             * This should be executed only once during
             * this test, if we are here for the 10th
             * time, consider this the recursive issue.
             *
             * We can get out of here by disable events,
             * so no new SIGIO is delivered.
             */
            ioctl(fd1, PERF_EVENT_IOC_DISABLE, 0);
            ioctl(fd2, PERF_EVENT_IOC_DISABLE, 0);
            ioctl(fd3, PERF_EVENT_IOC_DISABLE, 0);
        }
    }
}

unsafe fn __event(is_x: bool, addr: *mut c_void, sig: c_int) -> c_int {
    unsafe {
        let mut pe: perf_event_attr = mem::zeroed();
        let fd: c_int;

        pe.type_ = PERF_TYPE_BREAKPOINT;
        pe.size = mem::size_of::<perf_event_attr>() as _;

        pe.config = 0;
        pe.bp_type = if is_x {
            HW_BREAKPOINT_X
        } else {
            HW_BREAKPOINT_W
        };
        pe.bp_addr = addr as libc::c_ulong;
        pe.bp_len = if is_x {
            default_breakpoint_len()
        } else {
            mem::size_of::<c_long>() as _
        };

        pe.sample_period = 1;
        pe.sample_type = PERF_SAMPLE_IP;
        pe.wakeup_events = 1;

        pe.disabled = 1;
        pe.exclude_kernel = 1;
        pe.exclude_hv = 1;

        fd = sys_perf_event_open(
            &mut pe,
            0,
            -1,
            -1,
            perf_event_open_cloexec_flag(),
        );
        if fd < 0 {
            pr_debug!("failed opening event %llx\n", pe.config);
            return TEST_FAIL;
        }

        fcntl(fd, F_SETFL, O_RDWR | O_NONBLOCK | O_ASYNC);
        fcntl(fd, F_SETSIG, sig);
        fcntl(fd, F_SETOWN, getpid());

        ioctl(fd, PERF_EVENT_IOC_RESET, 0);

        fd
    }
}

unsafe fn bp_event(addr: *mut c_void, sig: c_int) -> c_int {
    unsafe { __event(true, addr, sig) }
}

unsafe fn wp_event(addr: *mut c_void, sig: c_int) -> c_int {
    unsafe { __event(false, addr, sig) }
}

unsafe fn bp_count(fd: c_int) -> i64 {
    unsafe {
        let mut count: i64 = 0;
        let ret: c_int;

        ret = read(
            fd,
            &mut count as *mut i64 as *mut c_void,
            mem::size_of::<i64>(),
        ) as c_int;
        if ret != mem::size_of::<i64>() as c_int {
            pr_debug!("failed to read: %d\n", ret);
            return TEST_FAIL as i64;
        }

        count
    }
}

unsafe extern "C" fn test__bp_signal(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    unsafe {
        let mut sa: sigaction = mem::zeroed();
        let count1: i64;
        let count2: i64;
        let count3: i64;

        if !BP_SIGNAL_IS_SUPPORTED {
            pr_debug!("Test not supported on this architecture");
            return TEST_SKIP;
        }

        /* setup SIGIO signal handler */
        ptr::write_bytes(
            &mut sa as *mut sigaction as *mut u8,
            0,
            mem::size_of::<sigaction>(),
        );
        sa.sa_sigaction = sig_handler as usize;
        sa.sa_flags = SA_SIGINFO;

        if sigaction(SIGIO, &sa, ptr::null_mut()) < 0 {
            pr_debug!("failed setting up signal handler\n");
            return TEST_FAIL;
        }

        sa.sa_sigaction = sig_handler_2 as usize;
        if sigaction(SIGUSR1, &sa, ptr::null_mut()) < 0 {
            pr_debug!("failed setting up signal handler 2\n");
            return TEST_FAIL;
        }

        /*
         * We create following events:
         *
         * fd1 - breakpoint event on __test_function with SIGIO
         *       signal configured. We should get signal
         *       notification each time the breakpoint is hit
         *
         * fd2 - breakpoint event on sig_handler with SIGUSR1
         *       configured. We should get SIGUSR1 each time when
         *       breakpoint is hit
         *
         * fd3 - watchpoint event on __test_function with SIGIO
         *       configured.
         *
         * Following processing should happen:
         *   Exec:               Action:                       Result:
         *   incq (%rdi)       - fd1 event breakpoint hit   -> count1 == 1
         *                     - SIGIO is delivered
         *   sig_handler       - fd2 event breakpoint hit   -> count2 == 1
         *                     - SIGUSR1 is delivered
         *   sig_handler_2                                  -> overflows_2 == 1  (nested signal)
         *   sys_rt_sigreturn  - return from sig_handler_2
         *   overflows++                                    -> overflows = 1
         *   sys_rt_sigreturn  - return from sig_handler
         *   incq (%rdi)       - fd3 event watchpoint hit   -> count3 == 1       (wp and bp in one insn)
         *                     - SIGIO is delivered
         *   sig_handler       - fd2 event breakpoint hit   -> count2 == 2
         *                     - SIGUSR1 is delivered
         *   sig_handler_2                                  -> overflows_2 == 2  (nested signal)
         *   sys_rt_sigreturn  - return from sig_handler_2
         *   overflows++                                    -> overflows = 2
         *   sys_rt_sigreturn  - return from sig_handler
         *   the_var++         - fd3 event watchpoint hit   -> count3 == 2       (standalone watchpoint)
         *                     - SIGIO is delivered
         *   sig_handler       - fd2 event breakpoint hit   -> count2 == 3
         *                     - SIGUSR1 is delivered
         *   sig_handler_2                                  -> overflows_2 == 3  (nested signal)
         *   sys_rt_sigreturn  - return from sig_handler_2
         *   overflows++                                    -> overflows == 3
         *   sys_rt_sigreturn  - return from sig_handler
         *
         * The test case check following error conditions:
         * - we get stuck in signal handler because of debug
         *   exception being triggered recursively due to
         *   the wrong RF EFLAG management
         *
         * - we never trigger the sig_handler breakpoint due
         *   to the wrong RF EFLAG management
         *
         */

        fd1 = bp_event(__test_function as *mut c_void, SIGIO);
        fd2 = bp_event(sig_handler as *mut c_void, SIGUSR1);
        fd3 = wp_event(&raw mut the_var as *mut c_void, SIGIO);

        ioctl(fd1, PERF_EVENT_IOC_ENABLE, 0);
        ioctl(fd2, PERF_EVENT_IOC_ENABLE, 0);
        ioctl(fd3, PERF_EVENT_IOC_ENABLE, 0);

        /*
         * Kick off the test by triggering 'fd1'
         * breakpoint.
         */
        test_function();

        ioctl(fd1, PERF_EVENT_IOC_DISABLE, 0);
        ioctl(fd2, PERF_EVENT_IOC_DISABLE, 0);
        ioctl(fd3, PERF_EVENT_IOC_DISABLE, 0);

        count1 = bp_count(fd1);
        count2 = bp_count(fd2);
        count3 = bp_count(fd3);

        close(fd1);
        close(fd2);
        close(fd3);

        pr_debug!(
            "count1 %lld, count2 %lld, count3 %lld, overflow %d, overflows_2 %d\n",
            count1,
            count2,
            count3,
            overflows,
            overflows_2
        );

        if count1 != 1 {
            if count1 == 11 {
                pr_debug!("failed: RF EFLAG recursion issue detected\n");
            } else {
                pr_debug!("failed: wrong count for bp1: %lld, expected 1\n", count1);
            }
        }

        if overflows != 3 {
            pr_debug!("failed: wrong overflow (%d) hit, expected 3\n", overflows);
        }

        if overflows_2 != 3 {
            pr_debug!(
                "failed: wrong overflow_2 (%d) hit, expected 3\n",
                overflows_2
            );
        }

        if count2 != 3 {
            pr_debug!("failed: wrong count for bp2 (%lld), expected 3\n", count2);
        }

        if count3 != 2 {
            pr_debug!("failed: wrong count for bp3 (%lld), expected 2\n", count3);
        }

        if count1 == 1
            && overflows == 3
            && count2 == 3
            && overflows_2 == 3
            && count3 == 2
        {
            TEST_OK
        } else {
            TEST_FAIL
        }
    }
}

DEFINE_SUITE!("Breakpoint overflow signal handler", bp_signal);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

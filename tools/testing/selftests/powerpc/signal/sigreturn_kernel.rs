// SPDX-License-Identifier: GPL-2.0
/*
 * Test that we can't sigreturn to kernel addresses, or to kernel mode.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type c_int = i32;
type c_ulong = u64;
type c_ulonglong = u64;
type pid_t = i32;

const SIGUSR1: c_int = 10;
const SA_SIGINFO: c_int = 4;
const MSR_PR: c_ulong = 1u64 << 14;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut core::ffi::c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

unsafe extern "C" {
    fn fork() -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn printf(format: *const u8, ...) -> c_int;

    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const u8) -> c_int;
    fn UCONTEXT_NIA(uc: *mut ucontext_t) -> *mut c_ulonglong;
    fn UCONTEXT_MSR(uc: *mut ucontext_t) -> *mut c_ulonglong;
    fn FAIL_IF(condition: c_int);
}

static mut sigreturn_addr: c_ulonglong = 0;
static mut sigreturn_msr_mask: c_ulonglong = 0;

unsafe fn WIFEXITED(status: c_int) -> c_int {
    (((status) & 0x7f) == 0) as c_int
}

unsafe fn WIFSIGNALED(status: c_int) -> c_int {
    ((((status) & 0x7f) + 1) >> 1 > 0) as c_int
}

unsafe fn WTERMSIG(status: c_int) -> c_int {
    (status) & 0x7f
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    ((status) & 0xff00) >> 8
}

unsafe extern "C" fn sigusr1_handler(
    _signo: c_int,
    _si: *mut siginfo_t,
    uc_ptr: *mut core::ffi::c_void,
) {
    let uc: *mut ucontext_t = uc_ptr as *mut ucontext_t;

    if sigreturn_addr != 0 {
        *UCONTEXT_NIA(uc) = sigreturn_addr;
    }

    if sigreturn_msr_mask != 0 {
        *UCONTEXT_MSR(uc) &= sigreturn_msr_mask;
    }
}

unsafe fn fork_child() -> pid_t {
    let pid: pid_t;

    pid = fork();
    if pid == 0 {
        raise(SIGUSR1);
        exit(0);
    }

    pid
}

unsafe fn expect_segv(pid: pid_t) -> c_int {
    let mut child_ret: c_int = 0;

    waitpid(pid, &mut child_ret, 0);
    FAIL_IF(WIFEXITED(child_ret));
    FAIL_IF((!WIFSIGNALED(child_ret) != 0) as c_int);
    FAIL_IF((WTERMSIG(child_ret) != 11) as c_int);

    0
}

pub extern "C" fn test_sigreturn_kernel() -> c_int {
    unsafe {
        let mut act: sigaction = core::mem::zeroed();
        let mut child_ret: c_int = 0;
        let mut i: c_int;
        let mut pid: pid_t;

        act.sa_sigaction = Some(sigusr1_handler);
        act.sa_flags = SA_SIGINFO;
        sigemptyset(&mut act.sa_mask);

        FAIL_IF(sigaction(SIGUSR1, &act, core::ptr::null_mut()));

        i = 0;
        while i < 2 {
            // Return to kernel
            sigreturn_addr = 0xcu64 << 60;
            pid = fork_child();
            expect_segv(pid);

            // Return to kernel virtual
            sigreturn_addr = 0xc008u64 << 48;
            pid = fork_child();
            expect_segv(pid);

            // Return out of range
            sigreturn_addr = 0xc010u64 << 48;
            pid = fork_child();
            expect_segv(pid);

            // Return to no-man's land, just below PAGE_OFFSET
            sigreturn_addr = (0xcu64 << 60).wrapping_sub(64 * 1024);
            pid = fork_child();
            expect_segv(pid);

            // Return to no-man's land, above TASK_SIZE_4PB
            sigreturn_addr = 0x1u64 << 52;
            pid = fork_child();
            expect_segv(pid);

            // Return to 0xd space
            sigreturn_addr = 0xdu64 << 60;
            pid = fork_child();
            expect_segv(pid);

            // Return to 0xe space
            sigreturn_addr = 0xeu64 << 60;
            pid = fork_child();
            expect_segv(pid);

            // Return to 0xf space
            sigreturn_addr = 0xfu64 << 60;
            pid = fork_child();
            expect_segv(pid);

            // Attempt to set PR=0 for 2nd loop (should be blocked by kernel)
            sigreturn_msr_mask = !MSR_PR;

            i += 1;
        }

        printf(c"All children killed as expected\n".as_ptr() as *const u8);

        // Don't change address, just MSR, should return to user as normal
        sigreturn_addr = 0;
        sigreturn_msr_mask = !MSR_PR;
        pid = fork_child();
        waitpid(pid, &mut child_ret, 0);
        FAIL_IF((!WIFEXITED(child_ret) != 0) as c_int);
        FAIL_IF(WIFSIGNALED(child_ret));
        FAIL_IF((WEXITSTATUS(child_ret) != 0) as c_int);

        0
    }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test_sigreturn_kernel,
            c"sigreturn_kernel".as_ptr() as *const u8,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and depended on kselftest_harness.h, stdio.h,
// string.h, errno.h, sys/wait.h, sys/syscall.h, sys/prctl.h, and
// linux/ptrace.h.

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_int, c_long};
use std::process;

type pid_t = c_int;

const SYS_ptrace: c_long = 101;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: c_int = 0x4210;
const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: c_int = 0x4211;
const PR_SYS_DISPATCH_OFF: u64 = 0;
const PR_SYS_DISPATCH_ON: u64 = 1;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;

#[repr(C)]
struct ptrace_sud_config {
    mode: u64,
    selector: u64,
    offset: u64,
    len: u64,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn _exit(status: c_int) -> !;
}

unsafe fn sys_ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_int {
    syscall(SYS_ptrace, request, pid, addr, data) as c_int
}

unsafe fn get_set_sud() {
    let mut config: ptrace_sud_config = std::mem::zeroed();
    let child: pid_t;
    let mut ret: c_int = 0;
    let mut status: c_int = 0;

    child = fork();
    assert!(child >= 0);
    if child == 0 {
        if sys_ptrace(
            PTRACE_TRACEME,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ) != 0
        {
            eprintln!("PTRACE_TRACEME: {}", std::io::Error::last_os_error());
            panic!("assertion failed: 0 == sys_ptrace(PTRACE_TRACEME, 0, 0, 0)");
        }
        kill(getpid(), SIGSTOP);
        _exit(1);
    }

    waitpid(child, &mut status, 0);

    memset(
        &mut config as *mut ptrace_sud_config as *mut c_void,
        0xff,
        size_of::<ptrace_sud_config>(),
    );
    config.mode = PR_SYS_DISPATCH_ON;

    ret = sys_ptrace(
        PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG,
        child,
        size_of::<ptrace_sud_config>() as *mut c_void,
        &mut config as *mut ptrace_sud_config as *mut c_void,
    );

    assert_eq!(ret, 0);
    assert_eq!(config.mode, PR_SYS_DISPATCH_OFF);
    assert_eq!(config.selector, 0);
    assert_eq!(config.offset, 0);
    assert_eq!(config.len, 0);

    config.mode = PR_SYS_DISPATCH_ON;
    config.selector = 0;
    config.offset = 0x400000;
    config.len = 0x1000;

    ret = sys_ptrace(
        PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG,
        child,
        size_of::<ptrace_sud_config>() as *mut c_void,
        &mut config as *mut ptrace_sud_config as *mut c_void,
    );

    assert_eq!(ret, 0);

    memset(
        &mut config as *mut ptrace_sud_config as *mut c_void,
        1,
        size_of::<ptrace_sud_config>(),
    );
    ret = sys_ptrace(
        PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG,
        child,
        size_of::<ptrace_sud_config>() as *mut c_void,
        &mut config as *mut ptrace_sud_config as *mut c_void,
    );

    assert_eq!(ret, 0);
    assert_eq!(config.mode, PR_SYS_DISPATCH_ON);
    assert_eq!(config.selector, 0);
    assert_eq!(config.offset, 0x400000);
    assert_eq!(config.len, 0x1000);

    kill(child, SIGKILL);
}

fn main() {
    unsafe {
        get_set_sud();
    }
    process::exit(0);
}

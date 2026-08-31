// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2018 Dmitry V. Levin <ldv@altlinux.org>
 * All rights reserved.
 *
 * Check whether PTRACE_GET_SYSCALL_INFO semantics implemented in the kernel
 * matches userspace expectations.
 */

// C dependencies: "kselftest_harness.h", <err.h>, <signal.h>,
// <asm/unistd.h>, and "linux/ptrace.h".

use core::mem::size_of;
use core::ptr::addr_of;

type pid_t = i32;

const SIGKILL: i32 = 9;
const SIGSTOP: i32 = 19;
const SIGTRAP: i32 = 5;
const ENOENT: i32 = 2;

const __NR_ptrace: libc::c_long = libc::SYS_ptrace as libc::c_long;
const __NR_chdir: libc::c_long = libc::SYS_chdir as libc::c_long;
const __NR_gettid: libc::c_long = libc::SYS_gettid as libc::c_long;
const __NR_exit_group: libc::c_long = libc::SYS_exit_group as libc::c_long;

extern "C" {
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: i32) -> i32;
    fn wait(status: *mut i32) -> pid_t;
    fn syscall(num: libc::c_long, ...) -> libc::c_long;
    fn _exit(status: i32) -> !;
    fn abort() -> !;
    fn __errno_location() -> *mut i32;
}

#[repr(C)]
struct ptrace_syscall_info_entry {
    nr: u64,
    args: [u64; 6],
}

#[repr(C)]
struct ptrace_syscall_info_exit {
    rval: i64,
    is_error: u8,
}

#[repr(C)]
union ptrace_syscall_info_data {
    entry: ptrace_syscall_info_entry,
    exit: ptrace_syscall_info_exit,
}

#[repr(C)]
struct ptrace_syscall_info {
    op: u8,
    pad: [u8; 3],
    arch: u32,
    instruction_pointer: u64,
    stack_pointer: u64,
    data: ptrace_syscall_info_data,
}

impl ptrace_syscall_info {
    unsafe fn entry(&self) -> &ptrace_syscall_info_entry {
        &self.data.entry
    }

    unsafe fn exit(&self) -> &ptrace_syscall_info_exit {
        &self.data.exit
    }
}

const PTRACE_TRACEME: i32 = 0;
const PTRACE_SYSCALL: i32 = 24;
const PTRACE_SETOPTIONS: i32 = 0x4200;
const PTRACE_GET_SYSCALL_INFO: i32 = 0x420e;
const PTRACE_O_TRACESYSGOOD: libc::c_ulong = 0x0000_0001;

const PTRACE_SYSCALL_INFO_NONE: u8 = 0;
const PTRACE_SYSCALL_INFO_ENTRY: u8 = 1;
const PTRACE_SYSCALL_INFO_EXIT: u8 = 2;

macro_rules! TH_LOG {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr, $body:block) => {
        if !($left <= $right) {
            $body
            panic!("ASSERT_LE({}, {}) failed", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr, $body:block) => {
        if !($left < $right) {
            $body
            panic!("ASSERT_LT({}, {}) failed", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $body:block) => {
        if !($left == $right) {
            $body
            panic!("ASSERT_EQ({}, {}) failed", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! ASSERT_FALSE {
    ($expr:expr, $body:block) => {
        if $expr {
            $body
            panic!("ASSERT_FALSE({}) failed", stringify!($expr));
        }
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr, $body:block) => {
        if !$expr {
            $body
            panic!("ASSERT_TRUE({}) failed", stringify!($expr));
        }
    };
}

fn WIFEXITED(status: i32) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: i32) -> i32 {
    (status & 0xff00) >> 8
}

fn WIFSIGNALED(status: i32) -> bool {
    ((status & 0x7f) + 1) >> 1 > 0
}

fn WTERMSIG(status: i32) -> i32 {
    status & 0x7f
}

fn WIFSTOPPED(status: i32) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: i32) -> i32 {
    (status & 0xff00) >> 8
}

unsafe fn kill_tracee(pid: pid_t) -> i32 {
    if pid == 0 {
        return 0;
    }

    let saved_errno = *__errno_location();

    let rc = kill(pid, SIGKILL);

    *__errno_location() = saved_errno;
    rc
}

unsafe fn sys_ptrace(
    request: i32,
    pid: pid_t,
    addr: libc::c_ulong,
    data: libc::c_ulong,
) -> libc::c_long {
    syscall(__NR_ptrace, request, pid, addr, data)
}

macro_rules! LOG_KILL_TRACEE {
    ($pid:expr, $ptrace_stop:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        kill_tracee($pid);
        TH_LOG!(concat!("wait #{}: ", $fmt), $ptrace_stop $(, $arg)*);
    }};
}

unsafe fn get_syscall_info() {
    static ARGS: [[libc::c_ulong; 7]; 3] = [
        /* a sequence of architecture-agnostic syscalls */
        [
            __NR_chdir as libc::c_ulong,
            b"\0".as_ptr() as libc::c_ulong,
            0xbad1fed1,
            0xbad2fed2,
            0xbad3fed3,
            0xbad4fed4,
            0xbad5fed5,
        ],
        [
            __NR_gettid as libc::c_ulong,
            0xcaf0bea0,
            0xcaf1bea1,
            0xcaf2bea2,
            0xcaf3bea3,
            0xcaf4bea4,
            0xcaf5bea5,
        ],
        [
            __NR_exit_group as libc::c_ulong,
            0,
            0xfac1c0d1,
            0xfac2c0d2,
            0xfac3c0d3,
            0xfac4c0d4,
            0xfac5c0d5,
        ],
    ];
    let mut exp_args: *const libc::c_ulong;

    let mut pid = fork();

    ASSERT_LE!(0, pid, {
        TH_LOG!("fork: %m");
    });

    if pid == 0 {
        /* get the pid before PTRACE_TRACEME */
        pid = getpid();
        ASSERT_EQ!(
            0,
            sys_ptrace(PTRACE_TRACEME, 0, 0, 0) as i32,
            {
                TH_LOG!("PTRACE_TRACEME: %m");
            }
        );
        ASSERT_EQ!(0, kill(pid, SIGSTOP), {
            /* cannot happen */
            TH_LOG!("kill SIGSTOP: %m");
        });
        for i in 0..ARGS.len() {
            syscall(
                ARGS[i][0] as libc::c_long,
                ARGS[i][1],
                ARGS[i][2],
                ARGS[i][3],
                ARGS[i][4],
                ARGS[i][5],
                ARGS[i][6],
            );
        }
        /* unreachable */
        _exit(1);
    }

    #[repr(C)]
    struct exit_param_t {
        is_error: u32,
        rval: i32,
    }

    let exit_param: [exit_param_t; 2] = [
        exit_param_t {
            is_error: 1,
            rval: -ENOENT,
        }, /* chdir */
        exit_param_t {
            is_error: 0,
            rval: pid,
        }, /* gettid */
    ];
    let mut exp_param: *const exit_param_t;

    let mut ptrace_stop: u32 = 0;

    loop {
        let mut info = ptrace_syscall_info {
            op: 0xff, /* invalid PTRACE_SYSCALL_INFO_* op */
            pad: [0; 3],
            arch: 0,
            instruction_pointer: 0,
            stack_pointer: 0,
            data: ptrace_syscall_info_data {
                entry: ptrace_syscall_info_entry {
                    nr: 0,
                    args: [0; 6],
                },
            },
        };
        let size = size_of::<ptrace_syscall_info>();
        let expected_none_size =
            (addr_of!(info.data.entry) as *const libc::c_void as isize
                - addr_of!(info) as *const libc::c_void as isize) as i32;
        let expected_entry_size =
            (addr_of!(info.entry().args[6]) as *const libc::c_void as isize
                - addr_of!(info) as *const libc::c_void as isize) as i32;
        let expected_exit_size =
            (addr_of!(info.exit().is_error).add(1) as *const libc::c_void as isize
                - addr_of!(info) as *const libc::c_void as isize) as i32;
        let mut status: i32 = 0;
        let mut rc: libc::c_long;

        ASSERT_EQ!(pid, wait(&mut status), {
            /* cannot happen */
            LOG_KILL_TRACEE!(pid, ptrace_stop, "wait: %m");
        });
        if WIFEXITED(status) {
            pid = 0; /* the tracee is no more */
            ASSERT_EQ!(0, WEXITSTATUS(status));
            break;
        }
        ASSERT_FALSE!(WIFSIGNALED(status), {
            pid = 0; /* the tracee is no more */
            LOG_KILL_TRACEE!(pid, ptrace_stop, "unexpected signal {}", WTERMSIG(status));
        });
        ASSERT_TRUE!(WIFSTOPPED(status), {
            /* cannot happen */
            LOG_KILL_TRACEE!(pid, ptrace_stop, "unexpected wait status {:#x}", status);
        });

        match WSTOPSIG(status) {
            SIGSTOP => {
                ASSERT_EQ!(0, ptrace_stop, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "unexpected signal stop");
                });
                ASSERT_EQ!(
                    0,
                    sys_ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACESYSGOOD) as i32,
                    {
                        LOG_KILL_TRACEE!(pid, ptrace_stop, "PTRACE_SETOPTIONS: %m");
                    }
                );
                rc = sys_ptrace(
                    PTRACE_GET_SYSCALL_INFO,
                    pid,
                    size as libc::c_ulong,
                    &mut info as *mut ptrace_syscall_info as libc::c_ulong,
                );
                ASSERT_LT!(0, rc, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "PTRACE_GET_SYSCALL_INFO: %m");
                });
                ASSERT_EQ!(expected_none_size, rc as i32, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "signal stop mismatch");
                });
                ASSERT_EQ!(PTRACE_SYSCALL_INFO_NONE, info.op, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "signal stop mismatch");
                });
                ASSERT_TRUE!(info.arch != 0, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "signal stop mismatch");
                });
                ASSERT_TRUE!(info.instruction_pointer != 0, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "signal stop mismatch");
                });
                ASSERT_TRUE!(info.stack_pointer != 0, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "signal stop mismatch");
                });
            }

            sig if sig == (SIGTRAP | 0x80) => {
                rc = sys_ptrace(
                    PTRACE_GET_SYSCALL_INFO,
                    pid,
                    size as libc::c_ulong,
                    &mut info as *mut ptrace_syscall_info as libc::c_ulong,
                );
                ASSERT_LT!(0, rc, {
                    LOG_KILL_TRACEE!(pid, ptrace_stop, "PTRACE_GET_SYSCALL_INFO: %m");
                });
                match ptrace_stop {
                    1 | 3 | 5 => {
                        /* entering chdir, entering gettid, entering exit_group */
                        exp_args = ARGS[(ptrace_stop / 2) as usize].as_ptr();
                        ASSERT_EQ!(expected_entry_size, rc as i32, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(PTRACE_SYSCALL_INFO_ENTRY, info.op, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_TRUE!(info.arch != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_TRUE!(info.instruction_pointer != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_TRUE!(info.stack_pointer != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(0), info.entry().nr as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(1), info.entry().args[0] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(2), info.entry().args[1] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(3), info.entry().args[2] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(4), info.entry().args[3] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(5), info.entry().args[4] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                        ASSERT_EQ!(*exp_args.add(6), info.entry().args[5] as libc::c_ulong, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "entry stop mismatch");
                        });
                    }
                    2 | 4 => {
                        /* exiting chdir, exiting gettid */
                        exp_param = &exit_param[(ptrace_stop / 2 - 1) as usize];
                        ASSERT_EQ!(expected_exit_size, rc as i32, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_EQ!(PTRACE_SYSCALL_INFO_EXIT, info.op, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_TRUE!(info.arch != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_TRUE!(info.instruction_pointer != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_TRUE!(info.stack_pointer != 0, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_EQ!((*exp_param).is_error as u8, info.exit().is_error, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                        ASSERT_EQ!((*exp_param).rval as i64, info.exit().rval, {
                            LOG_KILL_TRACEE!(pid, ptrace_stop, "exit stop mismatch");
                        });
                    }
                    _ => {
                        LOG_KILL_TRACEE!(pid, ptrace_stop, "unexpected syscall stop");
                        abort();
                    }
                }
            }

            _ => {
                LOG_KILL_TRACEE!(
                    pid,
                    ptrace_stop,
                    "unexpected stop signal {:#x}",
                    WSTOPSIG(status)
                );
                abort();
            }
        }

        ASSERT_EQ!(0, sys_ptrace(PTRACE_SYSCALL, pid, 0, 0) as i32, {
            LOG_KILL_TRACEE!(pid, ptrace_stop, "PTRACE_SYSCALL: %m");
        });

        ptrace_stop += 1;
    }

    ASSERT_EQ!((ARGS.len() * 2) as u32, ptrace_stop);
}

fn main() {
    unsafe {
        get_syscall_info();
    }
}

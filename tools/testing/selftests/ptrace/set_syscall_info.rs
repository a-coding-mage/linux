// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2018-2025 Dmitry V. Levin <ldv@strace.io>
 * All rights reserved.
 *
 * Check whether PTRACE_SET_SYSCALL_INFO semantics implemented in the kernel
 * matches userspace expectations.
 */

// Original C dependencies:
// "kselftest_harness.h", <err.h>, <fcntl.h>, <signal.h>, <asm/unistd.h>,
// <linux/types.h>, and <linux/ptrace.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type size_t = usize;

// MIPS N32 is the only architecture where __kernel_ulong_t
// does not match the bitness of syscall arguments.
#[cfg(all(any(target_arch = "mips", target_arch = "mips64"), target_pointer_width = "32"))]
type kernel_ulong_t = u64;
#[cfg(not(all(any(target_arch = "mips", target_arch = "mips64"), target_pointer_width = "32")))]
type kernel_ulong_t = c_ulong;

#[repr(C)]
struct __test_metadata {
    _unused: [u8; 0],
}

#[repr(C)]
struct ptrace_syscall_info_entry {
    nr: c_long,
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

#[repr(C)]
struct si_entry {
    nr: c_int,
    args: [kernel_ulong_t; 6],
}

#[repr(C)]
struct si_exit {
    is_error: c_uint,
    rval: c_int,
}

#[repr(C)]
struct si_pair {
    entry: [si_entry; 2],
    exit: [si_exit; 2],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn abort() -> !;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn _exit(status: c_int) -> !;

    fn TH_LOG(fmt: *const c_char, ...);
}

const SIGKILL: c_int = 9;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;

const ENOSYS: c_int = 38;
const EISDIR: c_int = 21;

const SPLICE_F_NONBLOCK: c_int = 2;

const PTRACE_TRACEME: c_int = 0;
const PTRACE_SYSCALL: c_int = 24;
const PTRACE_SETOPTIONS: c_int = 0x4200;
const PTRACE_GET_SYSCALL_INFO: c_int = 0x420e;
const PTRACE_SET_SYSCALL_INFO: c_int = 0x4210;
const PTRACE_O_TRACESYSGOOD: c_ulong = 0x00000001;

const PTRACE_SYSCALL_INFO_ENTRY: u8 = 1;
const PTRACE_SYSCALL_INFO_EXIT: u8 = 2;

const __NR_ptrace: c_long = 101;
const __NR_gettid: c_int = 186;
const __NR_getppid: c_int = 110;
const __NR_chdir: c_int = 80;
const __NR_splice: c_int = 275;
const __NR_exit_group: c_int = 231;

static mut ptrace_stop: c_uint = 0;
static mut tracee_pid: pid_t = 0;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr, $body:block) => {
        if $left != $right $body
    };
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_TRUE {
    ($cond:expr, $body:block) => {
        if !$cond $body
    };
}

macro_rules! ASSERT_FALSE {
    ($cond:expr, $body:block) => {
        if $cond $body
    };
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr, $body:block) => {
        if !($left < $right) $body
    };
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr, $body:block) => {
        if !($left <= $right) $body
    };
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) as i8 >> 1) > 0
}

unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

unsafe fn LOG_KILL_TRACEE(fmt: *const c_char) {
    kill_tracee(tracee_pid);
    TH_LOG(cstr!("wait #%d: %s"), ptrace_stop, fmt);
}

unsafe fn LOG_KILL_TRACEE_1s(fmt: *const c_char, s: *const c_char) {
    kill_tracee(tracee_pid);
    TH_LOG(cstr!("wait #%d: "), ptrace_stop);
    TH_LOG(fmt, s);
}

unsafe fn LOG_KILL_TRACEE_1u(fmt: *const c_char, u: c_uint) {
    kill_tracee(tracee_pid);
    TH_LOG(cstr!("wait #%d: "), ptrace_stop);
    TH_LOG(fmt, u);
}

unsafe fn kill_tracee(pid: pid_t) -> c_int {
    if pid == 0 {
        return 0;
    }

    let saved_errno = errno;

    let rc = kill(pid, SIGKILL);

    errno = saved_errno;
    rc
}

unsafe fn sys_ptrace(request: c_int, pid: pid_t, addr: c_ulong, data: c_ulong) -> c_long {
    syscall(__NR_ptrace, request, pid, addr, data)
}

unsafe fn check_psi_entry(
    _metadata: *mut __test_metadata,
    info: *const ptrace_syscall_info,
    exp_entry: *const si_entry,
    text: *const c_char,
) {
    let mut i: c_uint;
    let mut exp_nr = (*exp_entry).nr;
    #[cfg(any(target_arch = "s390x", target_arch = "s390"))]
    {
        // s390 is the only architecture that has 16-bit syscall numbers.
        exp_nr &= 0xffff;
    }

    ASSERT_EQ!(PTRACE_SYSCALL_INFO_ENTRY, (*info).op, {
        LOG_KILL_TRACEE_1s(cstr!("%s: entry stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).arch != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: entry stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).instruction_pointer != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: entry stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).stack_pointer != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: entry stop mismatch"), text);
    });
    ASSERT_EQ!(exp_nr as c_long, (*info).data.entry.nr, {
        LOG_KILL_TRACEE_1s(cstr!("%s: syscall nr mismatch"), text);
    });
    i = 0;
    while (i as usize) < (*exp_entry).args.len() {
        ASSERT_EQ!((*exp_entry).args[i as usize] as u64, (*info).data.entry.args[i as usize], {
            kill_tracee(tracee_pid);
            TH_LOG(
                cstr!("wait #%d: %s: syscall arg #%u mismatch"),
                ptrace_stop,
                text,
                i,
            );
        });
        i += 1;
    }
}

unsafe fn check_psi_exit(
    _metadata: *mut __test_metadata,
    info: *const ptrace_syscall_info,
    exp_exit: *const si_exit,
    text: *const c_char,
) {
    ASSERT_EQ!(PTRACE_SYSCALL_INFO_EXIT, (*info).op, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).arch != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).instruction_pointer != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
    ASSERT_TRUE!((*info).stack_pointer != 0, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
    ASSERT_EQ!((*exp_exit).is_error as u8, (*info).data.exit.is_error, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
    ASSERT_EQ!((*exp_exit).rval as i64, (*info).data.exit.rval, {
        LOG_KILL_TRACEE_1s(cstr!("%s: exit stop mismatch"), text);
    });
}

unsafe fn set_syscall_info(_metadata: *mut __test_metadata) {
    let tracer_pid: pid_t = getpid();
    let dummy: [kernel_ulong_t; 6] = [
        0xdad0bef0bad0fed0_u64 as kernel_ulong_t,
        0xdad1bef1bad1fed1_u64 as kernel_ulong_t,
        0xdad2bef2bad2fed2_u64 as kernel_ulong_t,
        0xdad3bef3bad3fed3_u64 as kernel_ulong_t,
        0xdad4bef4bad4fed4_u64 as kernel_ulong_t,
        0xdad5bef5bad5fed5_u64 as kernel_ulong_t,
    ];
    let mut splice_in: [c_int; 2] = [0; 2];
    let mut splice_out: [c_int; 2] = [0; 2];

    ASSERT_EQ!(0, pipe(splice_in.as_mut_ptr()));
    ASSERT_EQ!(0, pipe(splice_out.as_mut_ptr()));
    ASSERT_EQ!(
        size_of::<[kernel_ulong_t; 6]>() as isize,
        write(
            splice_in[1],
            dummy.as_ptr() as *const c_void,
            size_of::<[kernel_ulong_t; 6]>()
        )
    );

    let si: [si_pair; 8] = [
        // change scno, keep non-error rval
        si_pair {
            entry: [
                si_entry { nr: __NR_gettid, args: dummy },
                si_entry { nr: __NR_getppid, args: dummy },
            ],
            exit: [
                si_exit { is_error: 0, rval: tracer_pid },
                si_exit { is_error: 0, rval: tracer_pid },
            ],
        },
        // set scno to -1, keep error rval
        si_pair {
            entry: [
                si_entry {
                    nr: __NR_chdir,
                    args: [cstr!(".") as usize as kernel_ulong_t, dummy[1], dummy[2], dummy[3], dummy[4], dummy[5]],
                },
                si_entry {
                    nr: -1,
                    args: [cstr!(".") as usize as kernel_ulong_t, dummy[1], dummy[2], dummy[3], dummy[4], dummy[5]],
                },
            ],
            exit: [
                si_exit { is_error: 1, rval: -ENOSYS },
                si_exit { is_error: 1, rval: -ENOSYS },
            ],
        },
        // keep scno, change non-error rval
        si_pair {
            entry: [
                si_entry { nr: __NR_getppid, args: dummy },
                si_entry { nr: __NR_getppid, args: dummy },
            ],
            exit: [
                si_exit { is_error: 0, rval: tracer_pid },
                si_exit { is_error: 0, rval: tracer_pid + 1 },
            ],
        },
        // change arg1, keep non-error rval
        si_pair {
            entry: [
                si_entry {
                    nr: __NR_chdir,
                    args: [cstr!("") as usize as kernel_ulong_t, dummy[1], dummy[2], dummy[3], dummy[4], dummy[5]],
                },
                si_entry {
                    nr: __NR_chdir,
                    args: [cstr!(".") as usize as kernel_ulong_t, dummy[1], dummy[2], dummy[3], dummy[4], dummy[5]],
                },
            ],
            exit: [
                si_exit { is_error: 0, rval: 0 },
                si_exit { is_error: 0, rval: 0 },
            ],
        },
        // set scno to -1, change error rval to non-error
        si_pair {
            entry: [
                si_entry { nr: __NR_gettid, args: dummy },
                si_entry { nr: -1, args: dummy },
            ],
            exit: [
                si_exit { is_error: 1, rval: -ENOSYS },
                si_exit { is_error: 0, rval: tracer_pid },
            ],
        },
        // change scno, change non-error rval to error
        si_pair {
            entry: [
                si_entry { nr: __NR_chdir, args: dummy },
                si_entry { nr: __NR_getppid, args: dummy },
            ],
            exit: [
                si_exit { is_error: 0, rval: tracer_pid },
                si_exit { is_error: 1, rval: -EISDIR },
            ],
        },
        // change scno and all args, change non-error rval
        si_pair {
            entry: [
                si_entry { nr: __NR_gettid, args: dummy },
                si_entry {
                    nr: __NR_splice,
                    args: [
                        splice_in[0] as kernel_ulong_t,
                        0,
                        splice_out[1] as kernel_ulong_t,
                        0,
                        size_of::<[kernel_ulong_t; 6]>() as kernel_ulong_t,
                        SPLICE_F_NONBLOCK as kernel_ulong_t,
                    ],
                },
            ],
            exit: [
                si_exit { is_error: 0, rval: size_of::<[kernel_ulong_t; 6]>() as c_int },
                si_exit { is_error: 0, rval: size_of::<[kernel_ulong_t; 6]>() as c_int + 1 },
            ],
        },
        // change arg1, no exit stop
        si_pair {
            entry: [
                si_entry { nr: __NR_exit_group, args: dummy },
                si_entry { nr: __NR_exit_group, args: [0, dummy[1], dummy[2], dummy[3], dummy[4], dummy[5]] },
            ],
            exit: [
                si_exit { is_error: 0, rval: 0 },
                si_exit { is_error: 0, rval: 0 },
            ],
        },
    ];

    let mut rc: c_long;
    let mut i: c_uint;

    tracee_pid = fork();

    ASSERT_LE!(0, tracee_pid, {
        TH_LOG(cstr!("fork: %m"));
    });

    if tracee_pid == 0 {
        // get the pid before PTRACE_TRACEME
        tracee_pid = getpid();
        ASSERT_EQ!(0, sys_ptrace(PTRACE_TRACEME, 0, 0, 0), {
            TH_LOG(cstr!("PTRACE_TRACEME: %m"));
        });
        ASSERT_EQ!(0, kill(tracee_pid, SIGSTOP), {
            // cannot happen
            TH_LOG(cstr!("kill SIGSTOP: %m"));
        });
        i = 0;
        while (i as usize) < si.len() {
            rc = syscall(
                si[i as usize].entry[0].nr as c_long,
                si[i as usize].entry[0].args[0],
                si[i as usize].entry[0].args[1],
                si[i as usize].entry[0].args[2],
                si[i as usize].entry[0].args[3],
                si[i as usize].entry[0].args[4],
                si[i as usize].entry[0].args[5],
            );
            if si[i as usize].exit[1].is_error != 0 {
                if rc != -1 || errno != -si[i as usize].exit[1].rval {
                    break;
                }
            } else if rc != si[i as usize].exit[1].rval as c_long {
                break;
            }
            i += 1;
        }
        /*
         * Something went wrong, but in this state tracee
         * cannot reliably issue syscalls, so just crash.
         */
        ptr::write_volatile(i as usize as *mut u8, 42);
        // unreachable
        _exit(i as c_int + 1);
    }

    ptrace_stop = 0;
    loop {
        let mut info = ptrace_syscall_info {
            op: 0xff, // invalid PTRACE_SYSCALL_INFO_* op
            pad: [0; 3],
            arch: 0,
            instruction_pointer: 0,
            stack_pointer: 0,
            data: ptrace_syscall_info_data {
                entry: ptrace_syscall_info_entry { nr: 0, args: [0; 6] },
            },
        };
        let size: size_t = size_of::<ptrace_syscall_info>();
        let expected_entry_size: c_int = ((&info.data.entry.args[6] as *const u64 as *const c_void as usize)
            - (&info as *const ptrace_syscall_info as *const c_void as usize)) as c_int;
        let expected_exit_size: c_int = (((&info.data.exit.is_error as *const u8).add(1) as *const c_void as usize)
            - (&info as *const ptrace_syscall_info as *const c_void as usize)) as c_int;
        let mut status: c_int = 0;

        ASSERT_EQ!(tracee_pid, wait(&mut status), {
            // cannot happen
            LOG_KILL_TRACEE(cstr!("wait: %m"));
        });
        if WIFEXITED(status) {
            tracee_pid = 0; // the tracee is no more
            ASSERT_EQ!(0, WEXITSTATUS(status), {
                kill_tracee(tracee_pid);
                TH_LOG(
                    cstr!("wait #%d: unexpected exit status %u"),
                    ptrace_stop,
                    WEXITSTATUS(status) as c_uint,
                );
            });
            break;
        }
        ASSERT_FALSE!(WIFSIGNALED(status), {
            tracee_pid = 0; // the tracee is no more
            kill_tracee(tracee_pid);
            TH_LOG(
                cstr!("wait #%d: unexpected signal %u"),
                ptrace_stop,
                WTERMSIG(status) as c_uint,
            );
        });
        ASSERT_TRUE!(WIFSTOPPED(status), {
            // cannot happen
            kill_tracee(tracee_pid);
            TH_LOG(cstr!("wait #%d: unexpected wait status %#x"), ptrace_stop, status);
        });

        ASSERT_LT!(ptrace_stop as usize, si.len() * 2, {
            LOG_KILL_TRACEE(cstr!("ptrace stop overflow"));
        });

        match WSTOPSIG(status) {
            SIGSTOP => {
                ASSERT_EQ!(0, ptrace_stop, {
                    LOG_KILL_TRACEE(cstr!("unexpected signal stop"));
                });
                ASSERT_EQ!(
                    0,
                    sys_ptrace(PTRACE_SETOPTIONS, tracee_pid, 0, PTRACE_O_TRACESYSGOOD),
                    {
                        LOG_KILL_TRACEE(cstr!("PTRACE_SETOPTIONS: %m"));
                    }
                );
            }
            x if x == (SIGTRAP | 0x80) => {
                ASSERT_LT!(0, ptrace_stop, {
                    LOG_KILL_TRACEE(cstr!("unexpected syscall stop"));
                });
                rc = sys_ptrace(
                    PTRACE_GET_SYSCALL_INFO,
                    tracee_pid,
                    size as c_ulong,
                    &mut info as *mut ptrace_syscall_info as usize as c_ulong,
                );
                ASSERT_LT!(0, rc, {
                    LOG_KILL_TRACEE(cstr!("PTRACE_GET_SYSCALL_INFO #1: %m"));
                });
                if (ptrace_stop & 1) != 0 {
                    // entering syscall
                    let exp_entry: *const si_entry = &si[ptrace_stop as usize / 2].entry[0];
                    let set_entry: *const si_entry = &si[ptrace_stop as usize / 2].entry[1];

                    // check ptrace_syscall_info before the changes
                    ASSERT_EQ!(expected_entry_size as c_long, rc, {
                        LOG_KILL_TRACEE(cstr!(
                            "PTRACE_GET_SYSCALL_INFO #1: entry stop mismatch"
                        ));
                    });
                    check_psi_entry(
                        _metadata,
                        &info,
                        exp_entry,
                        cstr!("PTRACE_GET_SYSCALL_INFO #1"),
                    );

                    // apply the changes
                    info.data.entry.nr = (*set_entry).nr as c_long;
                    i = 0;
                    while (i as usize) < (*set_entry).args.len() {
                        info.data.entry.args[i as usize] = (*set_entry).args[i as usize] as u64;
                        i += 1;
                    }
                    ASSERT_EQ!(
                        0,
                        sys_ptrace(
                            PTRACE_SET_SYSCALL_INFO,
                            tracee_pid,
                            size as c_ulong,
                            &mut info as *mut ptrace_syscall_info as usize as c_ulong,
                        ),
                        {
                            LOG_KILL_TRACEE(cstr!("PTRACE_SET_SYSCALL_INFO: %m"));
                        }
                    );

                    // check ptrace_syscall_info after the changes
                    memset(
                        &mut info as *mut ptrace_syscall_info as *mut c_void,
                        0,
                        size_of::<ptrace_syscall_info>(),
                    );
                    info.op = 0xff;
                    rc = sys_ptrace(
                        PTRACE_GET_SYSCALL_INFO,
                        tracee_pid,
                        size as c_ulong,
                        &mut info as *mut ptrace_syscall_info as usize as c_ulong,
                    );
                    ASSERT_LT!(0, rc, {
                        LOG_KILL_TRACEE(cstr!("PTRACE_GET_SYSCALL_INFO: %m"));
                    });
                    ASSERT_EQ!(expected_entry_size as c_long, rc, {
                        LOG_KILL_TRACEE(cstr!(
                            "PTRACE_GET_SYSCALL_INFO #2: entry stop mismatch"
                        ));
                    });
                    check_psi_entry(
                        _metadata,
                        &info,
                        set_entry,
                        cstr!("PTRACE_GET_SYSCALL_INFO #2"),
                    );
                } else {
                    // exiting syscall
                    let exp_exit: *const si_exit = &si[ptrace_stop as usize / 2 - 1].exit[0];
                    let set_exit: *const si_exit = &si[ptrace_stop as usize / 2 - 1].exit[1];

                    // check ptrace_syscall_info before the changes
                    ASSERT_EQ!(expected_exit_size as c_long, rc, {
                        LOG_KILL_TRACEE(cstr!(
                            "PTRACE_GET_SYSCALL_INFO #1: exit stop mismatch"
                        ));
                    });
                    check_psi_exit(
                        _metadata,
                        &info,
                        exp_exit,
                        cstr!("PTRACE_GET_SYSCALL_INFO #1"),
                    );

                    // apply the changes
                    info.data.exit.is_error = (*set_exit).is_error as u8;
                    info.data.exit.rval = (*set_exit).rval as i64;
                    ASSERT_EQ!(
                        0,
                        sys_ptrace(
                            PTRACE_SET_SYSCALL_INFO,
                            tracee_pid,
                            size as c_ulong,
                            &mut info as *mut ptrace_syscall_info as usize as c_ulong,
                        ),
                        {
                            LOG_KILL_TRACEE(cstr!("PTRACE_SET_SYSCALL_INFO: %m"));
                        }
                    );

                    // check ptrace_syscall_info after the changes
                    memset(
                        &mut info as *mut ptrace_syscall_info as *mut c_void,
                        0,
                        size_of::<ptrace_syscall_info>(),
                    );
                    info.op = 0xff;
                    rc = sys_ptrace(
                        PTRACE_GET_SYSCALL_INFO,
                        tracee_pid,
                        size as c_ulong,
                        &mut info as *mut ptrace_syscall_info as usize as c_ulong,
                    );
                    ASSERT_LT!(0, rc, {
                        LOG_KILL_TRACEE(cstr!("PTRACE_GET_SYSCALL_INFO #2: %m"));
                    });
                    ASSERT_EQ!(expected_exit_size as c_long, rc, {
                        LOG_KILL_TRACEE(cstr!(
                            "PTRACE_GET_SYSCALL_INFO #2: exit stop mismatch"
                        ));
                    });
                    check_psi_exit(
                        _metadata,
                        &info,
                        set_exit,
                        cstr!("PTRACE_GET_SYSCALL_INFO #2"),
                    );
                }
            }
            _ => {
                LOG_KILL_TRACEE_1u(cstr!("unexpected stop signal %u"), WSTOPSIG(status) as c_uint);
                abort();
            }
        }

        ASSERT_EQ!(0, sys_ptrace(PTRACE_SYSCALL, tracee_pid, 0, 0), {
            LOG_KILL_TRACEE(cstr!("PTRACE_SYSCALL: %m"));
        });

        ptrace_stop += 1;
    }

    ASSERT_EQ!(ptrace_stop as usize, si.len() * 2);
}

fn main() {
    unsafe {
        let mut metadata = __test_metadata { _unused: [] };
        set_syscall_info(&mut metadata);
    }
}

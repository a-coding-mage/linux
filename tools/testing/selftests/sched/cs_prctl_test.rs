// SPDX-License-Identifier: GPL-2.0-only
/*
 * Use the core scheduling prctl() to test core scheduling cookies control.
 *
 * Copyright (c) 2021 Oracle and/or its affiliates.
 * Author: Chris Hyser <chris.hyser@oracle.com>
 *
 *
 * This library is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License as
 * published by the Free Software Foundation.
 *
 * This library is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library; if not, see <http://www.gnu.org/licenses>.
 */

use libc::{
    c_char, c_int, c_long, c_ulong, c_void, pid_t, size_t, EINVAL, EXIT_FAILURE, EXIT_SUCCESS,
    SIGCHLD,
};

// C source included GNU/Linux and libc headers. Symbols below are expected from libc.

const PR_SCHED_CORE: c_int = 62;
const PR_SCHED_CORE_GET: c_ulong = 0;
const PR_SCHED_CORE_CREATE: c_ulong = 1; /* create unique core_sched cookie */
const PR_SCHED_CORE_SHARE_TO: c_ulong = 2; /* push core_sched cookie to pid */
const PR_SCHED_CORE_SHARE_FROM: c_ulong = 3; /* pull core_sched cookie to pid */
const PR_SCHED_CORE_MAX: c_ulong = 4;

const MAX_PROCESSES: usize = 128;
const MAX_THREADS: usize = 128;

static USAGE: &[u8] = b"cs_prctl_test [options]\n\
    options:\n\
\t-P  : number of processes to create.\n\
\t-T  : number of threads per process to create.\n\
\t-d  : delay time to keep tasks alive.\n\
\t-k  : keep tasks alive until keypress.\n\0";

#[repr(C)]
enum pid_type {
    PIDTYPE_PID = 0,
    PIDTYPE_TGID,
    PIDTYPE_PGID,
}

const THREAD_CLONE_FLAGS: c_int =
    libc::CLONE_THREAD | libc::CLONE_SIGHAND | libc::CLONE_FS | libc::CLONE_VM | libc::CLONE_FILES;

#[repr(C)]
#[derive(Copy, Clone)]
struct child_args {
    num_threads: c_int,
    pfd: [c_int; 2],
    cpid: c_int,
    thr_tids: [c_int; MAX_THREADS],
}

static mut procs: [child_args; MAX_PROCESSES] = [child_args {
    num_threads: 0,
    pfd: [0; 2],
    cpid: 0,
    thr_tids: [0; MAX_THREADS],
}; MAX_PROCESSES];
static mut num_processes: c_int = 2;
static mut need_cleanup: c_int = 0;

unsafe fn gettid() -> pid_t {
    libc::syscall(libc::SYS_gettid) as pid_t
}

unsafe fn _prctl(
    option: c_int,
    arg2: c_ulong,
    arg3: c_ulong,
    arg4: c_ulong,
    arg5: c_ulong,
) -> c_int {
    let res: c_int;

    res = libc::prctl(option, arg2, arg3, arg4, arg5);
    libc::printf(
        b"%d = prctl(%d, %ld, %ld, %ld, %lx)\n\0".as_ptr() as *const c_char,
        res,
        option,
        arg2 as c_long,
        arg3 as c_long,
        arg4 as c_long,
        arg5,
    );
    res
}

const STACK_SIZE: usize = 1024 * 1024;

macro_rules! handle_error {
    ($msg:expr) => {
        __handle_error(
            concat!(file!(), "\0").as_ptr() as *mut c_char,
            line!() as c_int,
            concat!($msg, "\0").as_ptr() as *mut c_char,
        )
    };
}

unsafe fn __handle_error(fn_: *mut c_char, ln: c_int, msg: *mut c_char) -> ! {
    let mut pidx: c_int;
    libc::printf(b"(%s:%d) - \0".as_ptr() as *const c_char, fn_, ln);
    libc::perror(msg);
    if need_cleanup != 0 {
        pidx = 0;
        while pidx < num_processes {
            libc::kill(procs[pidx as usize].cpid, 15);
            pidx += 1;
        }
        need_cleanup = 0;
    }
    libc::exit(EXIT_FAILURE);
}

unsafe fn handle_usage(rc: c_int, msg: *mut c_char) -> ! {
    libc::puts(USAGE.as_ptr() as *const c_char);
    libc::puts(msg);
    libc::putchar('\n' as c_int);
    libc::exit(rc);
}

unsafe fn get_cs_cookie(pid: c_int) -> c_ulong {
    let mut cookie: libc::c_ulonglong = 0;
    let ret: c_int;

    ret = libc::prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_GET,
        pid as c_ulong,
        pid_type::PIDTYPE_PID as c_ulong,
        &mut cookie as *mut libc::c_ulonglong as c_ulong,
    );
    if ret != 0 {
        libc::printf(b"Not a core sched system\n\0".as_ptr() as *const c_char);
        return !0 as c_ulong;
    }

    cookie as c_ulong
}

extern "C" fn child_func_thread(_arg: *mut c_void) -> c_int {
    unsafe {
        loop {
            libc::usleep(20000);
        }
    }
}

unsafe fn create_threads(num_threads: c_int, thr_tids: *mut c_int) {
    let mut child_stack: *mut c_void;
    let mut tid: pid_t;
    let mut i: c_int;

    i = 0;
    while i < num_threads {
        child_stack = libc::malloc(STACK_SIZE as size_t);
        if child_stack.is_null() {
            handle_error!("child stack allocate");
        }

        tid = libc::clone(
            child_func_thread,
            (child_stack as *mut u8).add(STACK_SIZE) as *mut c_void,
            THREAD_CLONE_FLAGS,
            std::ptr::null_mut(),
        );
        if tid == -1 {
            handle_error!("clone thread");
        }
        *thr_tids.add(i as usize) = tid;
        i += 1;
    }
}

extern "C" fn child_func_process(arg: *mut c_void) -> c_int {
    unsafe {
        let ca: *mut child_args = arg as *mut child_args;
        let ret: c_int;

        libc::close((*ca).pfd[0]);

        create_threads((*ca).num_threads, (*ca).thr_tids.as_mut_ptr());

        ret = libc::write(
            (*ca).pfd[1],
            (*ca).thr_tids.as_ptr() as *const c_void,
            (std::mem::size_of::<c_int>() as c_int * (*ca).num_threads) as size_t,
        ) as c_int;
        if ret == -1 {
            libc::printf(
                b"write failed on pfd[%d] - error (%s)\n\0".as_ptr() as *const c_char,
                (*ca).pfd[1],
                libc::strerror(*libc::__errno_location()),
            );
        }

        libc::close((*ca).pfd[1]);

        loop {
            libc::usleep(20000);
        }
    }
}

static mut child_func_process_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

unsafe fn create_processes(num_processes_: c_int, num_threads: c_int, proc_: *mut child_args) {
    let mut cpid: pid_t;
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < num_processes_ {
        (*proc_.add(i as usize)).num_threads = num_threads;

        if libc::pipe((*proc_.add(i as usize)).pfd.as_mut_ptr()) == -1 {
            handle_error!("pipe() failed");
        }

        cpid = libc::clone(
            child_func_process,
            child_func_process_stack.as_mut_ptr().add(STACK_SIZE) as *mut c_void,
            SIGCHLD,
            proc_.add(i as usize) as *mut c_void,
        );
        (*proc_.add(i as usize)).cpid = cpid;
        libc::close((*proc_.add(i as usize)).pfd[1]);
        i += 1;
    }

    i = 0;
    while i < num_processes_ {
        ret = libc::read(
            (*proc_.add(i as usize)).pfd[0],
            (*proc_.add(i as usize)).thr_tids.as_mut_ptr() as *mut c_void,
            (std::mem::size_of::<c_int>() as c_int * (*proc_.add(i as usize)).num_threads)
                as size_t,
        ) as c_int;
        if ret == -1 {
            libc::printf(
                b"read failed on proc[%d].pfd[0] error (%s)\n\0".as_ptr() as *const c_char,
                i,
                libc::strerror(*libc::__errno_location()),
            );
        }
        libc::close((*proc_.add(i as usize)).pfd[0]);
        i += 1;
    }
}

unsafe fn disp_processes(num_processes_: c_int, proc_: *mut child_args) {
    let mut i: c_int;
    let mut j: c_int;

    libc::printf(
        b"tid=%d, / tgid=%d / pgid=%d: %lx\n\0".as_ptr() as *const c_char,
        gettid(),
        libc::getpid(),
        libc::getpgid(0),
        get_cs_cookie(libc::getpid()),
    );

    i = 0;
    while i < num_processes_ {
        libc::printf(
            b"    tid=%d, / tgid=%d / pgid=%d: %lx\n\0".as_ptr() as *const c_char,
            (*proc_.add(i as usize)).cpid,
            (*proc_.add(i as usize)).cpid,
            libc::getpgid((*proc_.add(i as usize)).cpid),
            get_cs_cookie((*proc_.add(i as usize)).cpid),
        );
        j = 0;
        while j < (*proc_.add(i as usize)).num_threads {
            libc::printf(
                b"        tid=%d, / tgid=%d / pgid=%d: %lx\n\0".as_ptr() as *const c_char,
                (*proc_.add(i as usize)).thr_tids[j as usize],
                (*proc_.add(i as usize)).cpid,
                libc::getpgid(0),
                get_cs_cookie((*proc_.add(i as usize)).thr_tids[j as usize]),
            );
            j += 1;
        }
        i += 1;
    }
    libc::puts(b"\n\0".as_ptr() as *const c_char);
}

static mut errors: c_int = 0;

macro_rules! validate {
    ($v:expr) => {
        _validate(
            line!() as c_int,
            if $v { 1 } else { 0 },
            concat!(stringify!($v), "\0").as_ptr() as *mut c_char,
        )
    };
}

unsafe fn _validate(line: c_int, val: c_int, msg: *mut c_char) {
    if val == 0 {
        errors += 1;
        libc::printf(b"(%d) FAILED: %s\n\0".as_ptr() as *const c_char, line, msg);
    } else {
        libc::printf(b"(%d) PASSED: %s\n\0".as_ptr() as *const c_char, line, msg);
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut keypress: c_int = 0;
    let mut num_threads: c_int = 3;
    let mut delay: c_int = 0;
    let mut res: c_int = 0;
    let mut pidx: c_int;
    let mut pid: c_int;
    let mut opt: c_int;

    loop {
        opt = libc::getopt(argc, argv, b":hkT:P:d:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt {
            x if x == 'P' as c_int => {
                num_processes =
                    libc::strtol(libc::optarg, std::ptr::null_mut(), 10) as c_int;
            }
            x if x == 'T' as c_int => {
                num_threads = libc::strtoul(libc::optarg, std::ptr::null_mut(), 10) as c_int;
            }
            x if x == 'd' as c_int => {
                delay = libc::strtol(libc::optarg, std::ptr::null_mut(), 10) as c_int;
            }
            x if x == 'k' as c_int => {
                keypress = 1;
            }
            x if x == 'h' as c_int => {
                libc::printf(USAGE.as_ptr() as *const c_char);
                libc::exit(EXIT_SUCCESS);
            }
            _ => {
                handle_usage(20, b"unknown option\0".as_ptr() as *mut c_char);
            }
        }
    }

    if num_processes < 1 || num_processes > MAX_PROCESSES as c_int {
        handle_usage(1, b"Bad processes value\0".as_ptr() as *mut c_char);
    }

    if num_threads < 1 || num_threads > MAX_THREADS as c_int {
        handle_usage(2, b"Bad thread value\0".as_ptr() as *mut c_char);
    }

    if keypress != 0 {
        delay = -1;
    }

    libc::srand(libc::time(std::ptr::null_mut()) as libc::c_uint);

    /* put into separate process group */
    if libc::setpgid(0, 0) != 0 {
        handle_error!("process group");
    }

    libc::printf(b"\n## Create a thread/process/process group hierarchy\n\0".as_ptr() as *const c_char);
    create_processes(num_processes, num_threads, procs.as_mut_ptr());
    need_cleanup = 1;
    disp_processes(num_processes, procs.as_mut_ptr());
    validate!(get_cs_cookie(0) == 0);

    libc::printf(b"\n## Set a cookie on entire process group\n\0".as_ptr() as *const c_char);
    if _prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_CREATE,
        0,
        pid_type::PIDTYPE_PGID as c_ulong,
        0,
    ) < 0
    {
        handle_error!("core_sched create failed -- PGID");
    }
    disp_processes(num_processes, procs.as_mut_ptr());

    validate!(get_cs_cookie(0) != 0);

    /* get a random process pid */
    pidx = libc::rand() % num_processes;
    pid = procs[pidx as usize].cpid;

    validate!(get_cs_cookie(0) == get_cs_cookie(pid));
    validate!(get_cs_cookie(0) == get_cs_cookie(procs[pidx as usize].thr_tids[0]));

    libc::printf(
        b"\n## Set a new cookie on entire process/TGID [%d]\n\0".as_ptr() as *const c_char,
        pid,
    );
    if _prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_CREATE,
        pid as c_ulong,
        pid_type::PIDTYPE_TGID as c_ulong,
        0,
    ) < 0
    {
        handle_error!("core_sched create failed -- TGID");
    }
    disp_processes(num_processes, procs.as_mut_ptr());

    validate!(get_cs_cookie(0) != get_cs_cookie(pid));
    validate!(get_cs_cookie(pid) != 0);
    validate!(get_cs_cookie(pid) == get_cs_cookie(procs[pidx as usize].thr_tids[0]));

    libc::printf(
        b"\n## Copy the cookie of current/PGID[%d], to pid [%d] as PIDTYPE_PID\n\0".as_ptr()
            as *const c_char,
        libc::getpid(),
        pid,
    );
    if _prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_SHARE_TO,
        pid as c_ulong,
        pid_type::PIDTYPE_PID as c_ulong,
        0,
    ) < 0
    {
        handle_error!("core_sched share to itself failed -- PID");
    }
    disp_processes(num_processes, procs.as_mut_ptr());

    validate!(get_cs_cookie(0) == get_cs_cookie(pid));
    validate!(get_cs_cookie(pid) != 0);
    validate!(get_cs_cookie(pid) != get_cs_cookie(procs[pidx as usize].thr_tids[0]));

    libc::printf(
        b"\n## Copy cookie from a thread [%d] to current/PGID [%d] as PIDTYPE_PID\n\0".as_ptr()
            as *const c_char,
        procs[pidx as usize].thr_tids[0],
        libc::getpid(),
    );
    if _prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_SHARE_FROM,
        procs[pidx as usize].thr_tids[0] as c_ulong,
        pid_type::PIDTYPE_PID as c_ulong,
        0,
    ) < 0
    {
        handle_error!("core_sched share from thread failed -- PID");
    }
    disp_processes(num_processes, procs.as_mut_ptr());

    validate!(get_cs_cookie(0) == get_cs_cookie(procs[pidx as usize].thr_tids[0]));
    validate!(get_cs_cookie(pid) != get_cs_cookie(procs[pidx as usize].thr_tids[0]));

    libc::printf(
        b"\n## Copy cookie from current [%d] to current as pidtype PGID\n\0".as_ptr()
            as *const c_char,
        libc::getpid(),
    );
    if _prctl(
        PR_SCHED_CORE,
        PR_SCHED_CORE_SHARE_TO,
        0,
        pid_type::PIDTYPE_PGID as c_ulong,
        0,
    ) < 0
    {
        handle_error!("core_sched share to self failed -- PGID");
    }
    disp_processes(num_processes, procs.as_mut_ptr());

    validate!(get_cs_cookie(0) == get_cs_cookie(pid));
    validate!(get_cs_cookie(pid) != 0);
    validate!(get_cs_cookie(pid) == get_cs_cookie(procs[pidx as usize].thr_tids[0]));

    validate!(
        _prctl(
            PR_SCHED_CORE,
            PR_SCHED_CORE_MAX,
            0,
            pid_type::PIDTYPE_PGID as c_ulong,
            0,
        ) < 0
            && *libc::__errno_location() == EINVAL
    );

    validate!(
        _prctl(
            PR_SCHED_CORE,
            PR_SCHED_CORE_SHARE_TO,
            0,
            pid_type::PIDTYPE_PGID as c_ulong,
            1,
        ) < 0
            && *libc::__errno_location() == EINVAL
    );

    if errors != 0 {
        libc::printf(
            b"TESTS FAILED. errors: %d\n\0".as_ptr() as *const c_char,
            errors,
        );
        res = 10;
    } else {
        libc::printf(b"SUCCESS !!!\n\0".as_ptr() as *const c_char);
    }

    if keypress != 0 {
        libc::getchar();
    } else {
        libc::sleep(delay as libc::c_uint);
    }

    pidx = 0;
    while pidx < num_processes {
        libc::kill(procs[pidx as usize].cpid, 15);
        pidx += 1;
    }

    res
}

fn main() {
    unsafe {
        let code = main_0(
            std::env::args().count() as c_int,
            libc::__libc_argv,
        );
        std::process::exit(code);
    }
}

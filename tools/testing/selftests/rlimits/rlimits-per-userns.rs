// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Alexey Gladkov <gladkov.alexey@gmail.com>
 */
// C source used _GNU_SOURCE and included system headers for process, rlimit,
// namespace, signal, errno, and err(3) interfaces.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

type pid_t = c_int;
type uid_t = c_uint;
type gid_t = c_uint;
type rlim_t = u64;

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const NR_CHILDS: usize = 2;

const RLIMIT_NPROC: c_int = 6;
const PR_SET_PDEATHSIG: c_int = 1;
const SIGKILL: c_int = 9;
const SIGUSR1: c_int = 10;
const SIG_DFL: usize = 0;
const CLONE_NEWUSER: c_int = 0x10000000;
const WNOHANG: c_int = 1;
const ECHILD: c_int = 10;

static mut service_prog: *mut c_char = ptr::null_mut();
static mut user: uid_t = 60000;
static mut group: gid_t = 60000;

unsafe extern "C" {
    fn getpid() -> pid_t;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn fork() -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn setgid(gid: gid_t) -> c_int;
    fn setuid(uid: uid_t) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn pause() -> c_int;
    fn exit(status: c_int) -> !;
    fn usleep(usec: c_uint) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn warnx(fmt: *const c_char, ...);
    fn warn(fmt: *const c_char, ...);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn __errno_location() -> *mut c_int;
}

unsafe fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn errno_location() -> *mut c_int {
    __errno_location()
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifsignaled(status: c_int) -> bool {
    let term_sig = status & 0x7f;
    term_sig != 0 && term_sig != 0x7f
}

fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn setrlimit_nproc(n: rlim_t) {
    let pid: pid_t = getpid();
    let limit = rlimit {
        rlim_cur: n,
        rlim_max: n,
    };

    warnx(
        cstr("(pid=%d): Setting RLIMIT_NPROC=%ld").as_ptr(),
        pid,
        n as c_long,
    );

    if setrlimit(RLIMIT_NPROC, &limit) < 0 {
        err(
            EXIT_FAILURE,
            cstr("(pid=%d): setrlimit(RLIMIT_NPROC)").as_ptr(),
            pid,
        );
    }
}

unsafe fn fork_child() -> pid_t {
    let mut pid: pid_t = fork();

    if pid < 0 {
        err(EXIT_FAILURE, cstr("fork").as_ptr());
    }

    if pid > 0 {
        return pid;
    }

    pid = getpid();

    warnx(cstr("(pid=%d): New process starting ...").as_ptr(), pid);

    if prctl(PR_SET_PDEATHSIG, SIGKILL) < 0 {
        err(
            EXIT_FAILURE,
            cstr("(pid=%d): prctl(PR_SET_PDEATHSIG)").as_ptr(),
            pid,
        );
    }

    signal(SIGUSR1, SIG_DFL);

    warnx(
        cstr("(pid=%d): Changing to uid=%d, gid=%d").as_ptr(),
        pid,
        user,
        group,
    );

    if setgid(group) < 0 {
        err(
            EXIT_FAILURE,
            cstr("(pid=%d): setgid(%d)").as_ptr(),
            pid,
            group,
        );
    }
    if setuid(user) < 0 {
        err(
            EXIT_FAILURE,
            cstr("(pid=%d): setuid(%d)").as_ptr(),
            pid,
            user,
        );
    }

    warnx(cstr("(pid=%d): Service running ...").as_ptr(), pid);

    warnx(cstr("(pid=%d): Unshare user namespace").as_ptr(), pid);
    if unshare(CLONE_NEWUSER) < 0 {
        err(EXIT_FAILURE, cstr("unshare(CLONE_NEWUSER)").as_ptr());
    }

    let service = cstr("service");
    let env = cstr("I_AM_SERVICE=1");
    let argv: [*mut c_char; 2] = [service.as_ptr() as *mut c_char, ptr::null_mut()];
    let envp: [*mut c_char; 2] = [env.as_ptr() as *mut c_char, ptr::null_mut()];

    warnx(cstr("(pid=%d): Executing real service ...").as_ptr(), pid);

    execve(service_prog, argv.as_ptr(), envp.as_ptr());
    err(EXIT_FAILURE, cstr("(pid=%d): execve").as_ptr(), pid);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: usize;
    let mut child: [pid_t; NR_CHILDS] = [0; NR_CHILDS];
    let mut wstatus: [c_int; NR_CHILDS] = [0; NR_CHILDS];
    let mut childs: c_int = NR_CHILDS as c_int;
    let pid: pid_t;

    let service_env = cstr("I_AM_SERVICE");
    if !getenv(service_env.as_ptr()).is_null() {
        pause();
        exit(EXIT_SUCCESS);
    }

    service_prog = *argv.offset(0);
    pid = getpid();

    warnx(cstr("(pid=%d) Starting testcase").as_ptr(), pid);

    /*
     * This rlimit is not a problem for root because it can be exceeded.
     */
    setrlimit_nproc(1);

    i = 0;
    while i < NR_CHILDS {
        child[i] = fork_child();
        wstatus[i] = 0;
        usleep(250000);
        i += 1;
    }

    loop {
        i = 0;
        while i < NR_CHILDS {
            if child[i] <= 0 {
                i += 1;
                continue;
            }

            *errno_location() = 0;
            let ret: pid_t = waitpid(child[i], &mut wstatus[i], WNOHANG);

            if ret == 0 || (!wifexited(wstatus[i]) && !wifsignaled(wstatus[i])) {
                i += 1;
                continue;
            }

            if ret < 0 && *errno_location() != ECHILD {
                warn(cstr("(pid=%d): waitpid(%d)").as_ptr(), pid, child[i]);
            }

            child[i] *= -1;
            childs -= 1;
            i += 1;
        }

        if childs == 0 {
            break;
        }

        usleep(250000);

        i = 0;
        while i < NR_CHILDS {
            if child[i] <= 0 {
                i += 1;
                continue;
            }
            kill(child[i], SIGUSR1);
            i += 1;
        }
    }

    i = 0;
    while i < NR_CHILDS {
        if wifexited(wstatus[i]) {
            warnx(
                cstr("(pid=%d): pid %d exited, status=%d").as_ptr(),
                pid,
                -child[i],
                wexitstatus(wstatus[i]),
            );
        } else if wifsignaled(wstatus[i]) {
            warnx(
                cstr("(pid=%d): pid %d killed by signal %d").as_ptr(),
                pid,
                -child[i],
                wtermsig(wstatus[i]),
            );
        }

        if wifsignaled(wstatus[i]) && wtermsig(wstatus[i]) == SIGUSR1 {
            i += 1;
            continue;
        }

        warnx(cstr("(pid=%d): Test failed").as_ptr(), pid);
        exit(EXIT_FAILURE);
    }

    warnx(cstr("(pid=%d): Test passed").as_ptr(), pid);
    exit(EXIT_SUCCESS);

    #[allow(unreachable_code)]
    {
        let _ = argc;
        EXIT_SUCCESS
    }
}

fn main() {
    let mut args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter_mut()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    argv.push(ptr::null_mut());

    unsafe {
        main_impl((argv.len() - 1) as c_int, argv.as_mut_ptr());
    }
}

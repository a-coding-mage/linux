// SPDX-License-Identifier: GPL-2.0
// C dependencies in the original: sched.h, unistd.h, stdio.h, stdlib.h,
// signal.h, errno.h, sys/types.h, sys/stat.h, fcntl.h, sys/ioctl.h,
// sys/prctl.h, sys/wait.h.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

const NSIO: c_uint = 0xb7;
const NS_GET_USERNS: c_ulong = ((NSIO as c_ulong) << 8) | 0x1;

const O_RDONLY: c_int = 0;
const SIGKILL: c_int = 9;
const PR_SET_PDEATHSIG: c_int = 1;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const EPERM: c_int = 1;

type PidT = c_int;
type ModeT = c_uint;
type OffT = c_long;
type BlkcntT = c_long;
type BlksizeT = c_long;
type DevT = c_ulong;
type InoT = c_ulong;
type NlinkT = c_ulong;
type UidT = c_uint;
type GidT = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat {
    st_dev: DevT,
    st_ino: InoT,
    st_nlink: NlinkT,
    st_mode: ModeT,
    st_uid: UidT,
    st_gid: GidT,
    __pad0: c_int,
    st_rdev: DevT,
    st_size: OffT,
    st_blksize: BlksizeT,
    st_blocks: BlkcntT,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [c_long; 3],
}

unsafe extern "C" {
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> PidT;
    fn prctl(option: c_int, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut Stat) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut Stat) -> c_int;
    fn kill(pid: PidT, sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> PidT;
    fn __errno_location() -> *mut c_int;
}

fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

fn pr_err(message: String, line: u32) -> c_int {
    eprintln!(
        "main:{}:{}: {}",
        line,
        message,
        std::io::Error::last_os_error()
    );
    1
}

fn path_to_string(path: &[c_char]) -> String {
    unsafe { CStr::from_ptr(path.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

fn main() {
    let mut pfd: [c_int; 2] = [0; 2];
    let mut st1 = Stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut st2 = st1;
    let mut path: [c_char; 128] = [0; 128];
    let pid: PidT;
    let mut c: c_char = 0;

    unsafe {
        if pipe(pfd.as_mut_ptr()) != 0 {
            std::process::exit(1);
        }

        pid = fork();
        if pid < 0 {
            std::process::exit(pr_err("fork".to_string(), line!()));
        }
        if pid == 0 {
            prctl(PR_SET_PDEATHSIG, SIGKILL);
            if unshare(CLONE_NEWUTS | CLONE_NEWUSER) != 0 {
                std::process::exit(pr_err("unshare".to_string(), line!()));
            }
            close(pfd[0]);
            close(pfd[1]);
            loop {
                sleep(1);
            }
        }
        close(pfd[1]);
        if read(pfd[0], &mut c as *mut c_char as *mut c_void, 1) != 0 {
            std::process::exit(pr_err("Unable to read from pipe".to_string(), line!()));
        }
        close(pfd[0]);

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"/proc/%d/ns/uts".as_ptr(),
            pid,
        );
        let ns = open(path.as_ptr(), O_RDONLY);
        if ns < 0 {
            std::process::exit(pr_err(
                format!("Unable to open {}", path_to_string(&path)),
                line!(),
            ));
        }

        let uns = ioctl(ns, NS_GET_USERNS);
        if uns < 0 {
            std::process::exit(pr_err(
                "Unable to get an owning user namespace".to_string(),
                line!(),
            ));
        }

        if fstat(uns, &mut st1) != 0 {
            std::process::exit(pr_err("fstat".to_string(), line!()));
        }

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"/proc/%d/ns/user".as_ptr(),
            pid,
        );
        if stat(path.as_ptr(), &mut st2) != 0 {
            std::process::exit(pr_err("stat".to_string(), line!()));
        }

        if st1.st_ino != st2.st_ino {
            std::process::exit(pr_err(
                "NS_GET_USERNS returned a wrong namespace".to_string(),
                line!(),
            ));
        }

        let init_uns = ioctl(uns, NS_GET_USERNS);
        if uns < 0 {
            std::process::exit(pr_err(
                "Unable to get an owning user namespace".to_string(),
                line!(),
            ));
        }

        if ioctl(init_uns, NS_GET_USERNS) >= 0 || errno_value() != EPERM {
            std::process::exit(pr_err("Don't get EPERM".to_string(), line!()));
        }

        if unshare(CLONE_NEWUSER) != 0 {
            std::process::exit(pr_err("unshare".to_string(), line!()));
        }

        if ioctl(ns, NS_GET_USERNS) >= 0 || errno_value() != EPERM {
            std::process::exit(pr_err("Don't get EPERM".to_string(), line!()));
        }
        if ioctl(init_uns, NS_GET_USERNS) >= 0 || errno_value() != EPERM {
            std::process::exit(pr_err("Don't get EPERM".to_string(), line!()));
        }

        kill(pid, SIGKILL);
        wait(ptr::null_mut());
    }
}

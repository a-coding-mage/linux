// SPDX-License-Identifier: GPL-2.0
// C dependencies: sched.h, unistd.h, stdio.h, stdlib.h, signal.h, errno.h,
// sys/types.h, sys/stat.h, fcntl.h, sys/ioctl.h, sys/prctl.h, sys/wait.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const NSIO: c_uint = 0xb7;
const NS_GET_USERNS: c_ulong = ior_none(NSIO, 0x1);
const NS_GET_PARENT: c_ulong = ior_none(NSIO, 0x2);

const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const EPERM: c_int = 1;
const O_RDONLY: c_int = 0;
const PR_SET_PDEATHSIG: c_int = 1;
const SIGCHLD: c_int = 17;
const SIGKILL: c_int = 9;

const fn ior_none(type_: c_uint, nr: c_uint) -> c_ulong {
    ((type_ << 8) | nr) as c_ulong
}

type PidT = c_int;

#[repr(C)]
struct Stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: c_long,
    st_atime_nsec: c_long,
    st_mtime: c_long,
    st_mtime_nsec: c_long,
    st_ctime: c_long,
    st_ctime_nsec: c_long,
    __glibc_reserved: [c_long; 3],
}

#[repr(C, align(16))]
struct AlignedStack {
    stack: [c_char; 128],
}

#[repr(C)]
struct CrCloneArg {
    stack: AlignedStack,
    // C flexible array member: char stack_ptr[];
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn clone(
        fn_: extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fstat(fd: c_int, buf: *mut Stat) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn kill(pid: PidT, sig: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut Stat) -> c_int;
    fn wait(wstatus: *mut c_int) -> PidT;

    static mut stderr: *mut c_void;
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn pr_err(msg: *const c_char) -> c_int {
    fprintf(stderr, c"%s:%d:%s: ".as_ptr(), c"main".as_ptr(), line!(), msg);
    perror(ptr::null());
    1
}

unsafe fn pr_err_path(msg: *const c_char, path: *const c_char) -> c_int {
    fprintf(stderr, c"%s:%d:".as_ptr(), c"main".as_ptr(), line!());
    fprintf(stderr, msg, path);
    fprintf(stderr, c": ".as_ptr());
    perror(ptr::null());
    1
}

extern "C" fn child(_args: *mut c_void) -> c_int {
    unsafe {
        prctl(PR_SET_PDEATHSIG, SIGKILL);
        loop {
            sleep(1);
        }
    }
}

unsafe fn real_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let ns_strs: [*const c_char; 2] = [c"pid".as_ptr(), c"user".as_ptr()];
    let mut path = *b"/proc/0123456789/ns/pid\0";
    let ca = MaybeUninit::<CrCloneArg>::uninit();
    let mut st1 = MaybeUninit::<Stat>::uninit();
    let mut st2 = MaybeUninit::<Stat>::uninit();
    let mut ns: c_int;
    let mut pns: c_int;
    let mut i: c_int;
    let pid: PidT;

    pid = clone(
        child,
        (ca.as_ptr() as *mut c_char).add(core::mem::size_of::<CrCloneArg>()) as *mut c_void,
        CLONE_NEWUSER | CLONE_NEWPID | SIGCHLD,
        ptr::null_mut(),
    );
    if pid < 0 {
        return pr_err(c"clone".as_ptr());
    }

    i = 0;
    while i < 2 {
        snprintf(
            path.as_mut_ptr() as *mut c_char,
            path.len(),
            c"/proc/%d/ns/%s".as_ptr(),
            pid,
            ns_strs[i as usize],
        );
        ns = open(path.as_ptr() as *const c_char, O_RDONLY);
        if ns < 0 {
            return pr_err_path(c"Unable to open %s".as_ptr(), path.as_ptr() as *const c_char);
        }

        pns = ioctl(ns, NS_GET_PARENT);
        if pns < 0 {
            return pr_err(c"Unable to get a parent pidns".as_ptr());
        }

        snprintf(
            path.as_mut_ptr() as *mut c_char,
            path.len(),
            c"/proc/self/ns/%s".as_ptr(),
            ns_strs[i as usize],
        );
        if stat(path.as_ptr() as *const c_char, st2.as_mut_ptr()) != 0 {
            return pr_err_path(c"Unable to stat %s".as_ptr(), path.as_ptr() as *const c_char);
        }
        if fstat(pns, st1.as_mut_ptr()) != 0 {
            return pr_err(c"Unable to stat the parent pidns".as_ptr());
        }
        if (*st1.as_ptr()).st_ino != (*st2.as_ptr()).st_ino {
            return pr_err(c"NS_GET_PARENT returned a wrong namespace".as_ptr());
        }

        if ioctl(pns, NS_GET_PARENT) >= 0 || errno() != EPERM {
            return pr_err(c"Don't get EPERM".as_ptr());
        }

        i += 1;
    }

    kill(pid, SIGKILL);
    wait(ptr::null_mut());
    0
}

fn main() {
    unsafe {
        let status = real_main(0, ptr::null_mut());
        if status != 0 {
            exit(status);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

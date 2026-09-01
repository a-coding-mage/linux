// SPDX-License-Identifier: GPL-2.0
// C dependencies: _GNU_SOURCE, errno, fcntl, sched, stdbool, stdio, stdlib,
// string, unistd, asm/ioctls, sys/mount, sys/wait, kselftest.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const KSFT_SKIP: c_int = 4;

const EINTR: c_int = 4;
const EINVAL: c_int = 22;

const O_RDWR: c_int = 0o00000002;
const O_NOCTTY: c_int = 0o00000400;
const O_CLOEXEC: c_int = 0o2000000;

const TIOCSCTTY: c_ulong = 0x540E;
const TIOCGPTPEER: c_ulong = 0x5441;

const MS_RDONLY: c_ulong = 1;
const MS_NOSUID: c_ulong = 2;
const MS_NODEV: c_ulong = 4;
const MS_NOEXEC: c_ulong = 8;
const MS_SYNCHRONOUS: c_ulong = 16;
const MS_REMOUNT: c_ulong = 32;
const MS_MANDLOCK: c_ulong = 64;
const MS_DIRSYNC: c_ulong = 128;
const MS_NOATIME: c_ulong = 1024;
const MS_NODIRATIME: c_ulong = 2048;
const MS_BIND: c_ulong = 4096;
const MS_MOVE: c_ulong = 8192;
const MS_REC: c_ulong = 16384;
const MS_SILENT: c_ulong = 32768;
const MS_POSIXACL: c_ulong = 1 << 16;
const MS_UNBINDABLE: c_ulong = 1 << 17;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_SLAVE: c_ulong = 1 << 19;
const MS_SHARED: c_ulong = 1 << 20;
const MS_RELATIME: c_ulong = 1 << 21;
const MS_KERNMOUNT: c_ulong = 1 << 22;
const MS_I_VERSION: c_ulong = 1 << 23;
const MS_STRICTATIME: c_ulong = 1 << 24;
const MS_LAZYTIME: c_ulong = 1 << 25;

const CLONE_NEWNS: c_int = 0x00020000;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn setsid() -> pid_t;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn unlockpt(fd: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn terminal_dup2(duplicate: c_int, original: c_int) -> bool {
    let ret: c_int;

    ret = dup2(duplicate, original);
    if ret < 0 {
        return false;
    }

    true
}

unsafe fn terminal_set_stdfds(fd: c_int) -> c_int {
    let mut i: c_int;
    let stdfds = [STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO];

    if fd < 0 {
        return 0;
    }

    i = 0;
    while i < 3 {
        if !terminal_dup2(fd, stdfds[i as usize]) {
            return -1;
        }
        i += 1;
    }

    0
}

unsafe fn login_pty(fd: c_int) -> c_int {
    let mut ret: c_int;

    setsid();

    ret = ioctl(fd, TIOCSCTTY, ptr::null_mut::<c_void>());
    if ret < 0 {
        return -1;
    }

    ret = terminal_set_stdfds(fd);
    if ret < 0 {
        return -1;
    }

    if fd > STDERR_FILENO {
        close(fd);
    }

    0
}

unsafe fn wait_for_pid(pid: pid_t) -> c_int {
    let mut status: c_int = 0;
    let mut ret: c_int;

    loop {
        ret = waitpid(pid, &mut status, 0);
        if ret == -1 {
            if errno == EINTR {
                continue;
            }
            return -1;
        }
        if ret != pid {
            continue;
        }

        break;
    }

    if !wifexited(status) || wexitstatus(status) != 0 {
        return -1;
    }

    0
}

unsafe fn resolve_procfd_symlink(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    let mut ret: c_int;
    let mut procfd = [0 as c_char; 4096];

    ret = snprintf(
        procfd.as_mut_ptr(),
        4096,
        c"/proc/self/fd/%d".as_ptr(),
        fd,
    );
    if ret < 0 || ret >= 4096 {
        return -1;
    }

    ret = readlink(procfd.as_ptr(), buf, buflen) as c_int;
    if ret < 0 || ret as size_t >= buflen {
        return -1;
    }

    *buf.add(ret as usize) = b'\0' as c_char;

    0
}

unsafe fn do_tiocgptpeer(ptmx: *mut c_char, expected_procfd_contents: *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut master: c_int = -1;
    let mut slave: c_int = -1;
    let mut fret: c_int = -1;

    master = open(ptmx, O_RDWR | O_NOCTTY | O_CLOEXEC);
    if master < 0 {
        fprintf(
            stderr,
            c"Failed to open \"%s\": %s\n".as_ptr(),
            ptmx,
            strerror(errno),
        );
        return -1;
    }

    /*
     * grantpt() makes assumptions about /dev/pts/ so ignore it. It's also
     * not really needed.
     */
    ret = unlockpt(master);
    if ret < 0 {
        fprintf(stderr, c"Failed to unlock terminal\n".as_ptr());
        return cleanup(master, slave, fret);
    }

    slave = ioctl(master, TIOCGPTPEER, O_RDWR | O_NOCTTY | O_CLOEXEC);
    if slave < 0 {
        if errno == EINVAL {
            fprintf(
                stderr,
                c"TIOCGPTPEER is not supported. Skipping test.\n".as_ptr(),
            );
            fret = KSFT_SKIP;
        } else {
            fprintf(stderr, c"Failed to perform TIOCGPTPEER ioctl\n".as_ptr());
            fret = EXIT_FAILURE;
        }
        return cleanup(master, slave, fret);
    }

    let pid: pid_t = fork();
    if pid < 0 {
        return cleanup(master, slave, fret);
    }

    if pid == 0 {
        let mut buf = [0 as c_char; 4096];

        ret = login_pty(slave);
        if ret < 0 {
            fprintf(stderr, c"Failed to setup terminal\n".as_ptr());
            _exit(EXIT_FAILURE);
        }

        ret = resolve_procfd_symlink(STDIN_FILENO, buf.as_mut_ptr(), buf.len());
        if ret < 0 {
            fprintf(
                stderr,
                c"Failed to retrieve pathname of pts slave file descriptor\n".as_ptr(),
            );
            _exit(EXIT_FAILURE);
        }

        if strncmp(
            expected_procfd_contents,
            buf.as_ptr(),
            strlen(expected_procfd_contents),
        ) != 0
        {
            fprintf(
                stderr,
                c"Received invalid contents for \"/proc/<pid>/fd/%d\" symlink: %s\n".as_ptr(),
                STDIN_FILENO,
                buf.as_ptr(),
            );
            _exit(-1);
        }

        fprintf(
            stderr,
            c"Contents of \"/proc/<pid>/fd/%d\" symlink are valid: %s\n".as_ptr(),
            STDIN_FILENO,
            buf.as_ptr(),
        );

        _exit(EXIT_SUCCESS);
    }

    ret = wait_for_pid(pid);
    if ret < 0 {
        return cleanup(master, slave, fret);
    }

    fret = EXIT_SUCCESS;

    cleanup(master, slave, fret)
}

unsafe fn cleanup(master: c_int, slave: c_int, fret: c_int) -> c_int {
    if master >= 0 {
        close(master);
    }
    if slave >= 0 {
        close(slave);
    }

    fret
}

unsafe fn verify_non_standard_devpts_mount() -> c_int {
    let mut mntpoint: *mut c_char;
    let mut ret: c_int = -1;
    let mut devpts = *b"/tmp/devpts_fs_XXXXXX\0";
    let mut ptmx = *b"/tmp/devpts_fs_XXXXXX/ptmx\0";

    ret = umount(c"/dev/pts".as_ptr());
    if ret < 0 {
        fprintf(
            stderr,
            c"Failed to unmount \"/dev/pts\": %s\n".as_ptr(),
            strerror(errno),
        );
        return -1;
    }

    umount(c"/dev/ptmx".as_ptr());

    mntpoint = mkdtemp(devpts.as_mut_ptr() as *mut c_char);
    if mntpoint.is_null() {
        fprintf(
            stderr,
            c"Failed to create temporary mountpoint: %s\n".as_ptr(),
            strerror(errno),
        );
        return -1;
    }

    ret = mount(
        c"devpts".as_ptr(),
        mntpoint,
        c"devpts".as_ptr(),
        MS_NOSUID | MS_NOEXEC,
        c"newinstance,ptmxmode=0666,mode=0620,gid=5".as_ptr() as *const c_void,
    );
    if ret < 0 {
        fprintf(
            stderr,
            c"Failed to mount devpts fs to \"%s\" in new mount namespace: %s\n".as_ptr(),
            mntpoint,
            strerror(errno),
        );
        unlink(mntpoint);
        return -1;
    }

    ret = snprintf(
        ptmx.as_mut_ptr() as *mut c_char,
        ptmx.len(),
        c"%s/ptmx".as_ptr(),
        devpts.as_ptr() as *const c_char,
    );
    if ret < 0 || ret as size_t >= ptmx.len() {
        unlink(mntpoint);
        return -1;
    }

    ret = do_tiocgptpeer(ptmx.as_mut_ptr() as *mut c_char, mntpoint);
    unlink(mntpoint);
    if ret < 0 {
        return -1;
    }

    0
}

unsafe fn verify_ptmx_bind_mount() -> c_int {
    let mut ret: c_int;

    ret = mount(
        c"/dev/pts/ptmx".as_ptr(),
        c"/dev/ptmx".as_ptr(),
        ptr::null(),
        MS_BIND,
        ptr::null(),
    );
    if ret < 0 {
        fprintf(
            stderr,
            c"Failed to bind mount \"/dev/pts/ptmx\" to \"/dev/ptmx\" mount namespace\n".as_ptr(),
        );
        return -1;
    }

    ret = do_tiocgptpeer(
        c"/dev/ptmx".as_ptr() as *mut c_char,
        c"/dev/pts/".as_ptr() as *mut c_char,
    );
    if ret < 0 {
        return -1;
    }

    0
}

unsafe fn verify_invalid_ptmx_bind_mount() -> c_int {
    let mut ret: c_int;
    let mntpoint_fd: c_char;
    let mut ptmx = *b"/tmp/devpts_ptmx_XXXXXX\0";

    mntpoint_fd = mkstemp(ptmx.as_mut_ptr() as *mut c_char) as c_char;
    if mntpoint_fd < 0 {
        fprintf(
            stderr,
            c"Failed to create temporary directory: %s\n".as_ptr(),
            strerror(errno),
        );
        return -1;
    }

    ret = mount(
        c"/dev/pts/ptmx".as_ptr(),
        ptmx.as_ptr() as *const c_char,
        ptr::null(),
        MS_BIND,
        ptr::null(),
    );
    close(mntpoint_fd as c_int);
    if ret < 0 {
        fprintf(
            stderr,
            c"Failed to bind mount \"/dev/pts/ptmx\" to \"%s\" mount namespace\n".as_ptr(),
            ptmx.as_ptr() as *const c_char,
        );
        return -1;
    }

    ret = do_tiocgptpeer(
        ptmx.as_mut_ptr() as *mut c_char,
        c"/dev/pts/".as_ptr() as *mut c_char,
    );
    if ret == 0 {
        return -1;
    }

    0
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;

    if isatty(STDIN_FILENO) == 0 {
        fprintf(
            stderr,
            c"Standard input file descriptor is not attached to a terminal. Skipping test\n"
                .as_ptr(),
        );
        exit(KSFT_SKIP);
    }

    ret = unshare(CLONE_NEWNS);
    if ret < 0 {
        fprintf(stderr, c"Failed to unshare mount namespace\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    ret = mount(
        c"".as_ptr(),
        c"/".as_ptr(),
        ptr::null(),
        MS_PRIVATE | MS_REC,
        ptr::null(),
    );
    if ret < 0 {
        fprintf(
            stderr,
            c"Failed to make \"/\" MS_PRIVATE in new mount namespace\n".as_ptr(),
        );
        exit(EXIT_FAILURE);
    }

    ret = verify_ptmx_bind_mount();
    if ret < 0 {
        exit(EXIT_FAILURE);
    }

    ret = verify_invalid_ptmx_bind_mount();
    if ret < 0 {
        exit(EXIT_FAILURE);
    }

    ret = verify_non_standard_devpts_mount();
    if ret < 0 {
        exit(EXIT_FAILURE);
    }

    exit(EXIT_SUCCESS);
}

fn main() {
    unsafe {
        let argv: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        c_main(argv.len() as c_int, argv.as_ptr() as *mut *mut c_char);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

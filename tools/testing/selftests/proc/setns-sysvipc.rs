/*
 * Copyright © 2019 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Test that setns(CLONE_NEWIPC) points to new /proc/sysvipc content even
 * if old one is in dcache.
 */

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

type pid_t = c_int;
type key_t = c_int;
type size_t = usize;
type ssize_t = isize;

const SIGTERM: c_int = 15;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const O_RDONLY: c_int = 0;
const CLONE_NEWIPC: c_int = 0x08000000;
const IPC_PRIVATE: key_t = 0;
const IPC_CREAT: c_int = 0o1000;

static mut PID: pid_t = -1;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn atexit(function: extern "C" fn()) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn shmget(key: key_t, size: size_t, shmflg: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn pause() -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
}

extern "C" fn f() {
    unsafe {
        if PID > 0 {
            kill(PID, SIGTERM);
        }
    }
}

fn c_string(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn main() {
    let mut fd: [c_int; 2] = [0; 2];
    let mut underscore: c_char = 0;
    let nsfd: c_int;

    unsafe {
        atexit(f);

        /* Check for priviledges and syscall availability straight away. */
        if unshare(CLONE_NEWIPC) == -1 {
            let errno = *__errno_location();
            if errno == ENOSYS || errno == EPERM {
                std::process::exit(4);
            }
            std::process::exit(1);
        }
        /* Distinguisher between two otherwise empty IPC namespaces. */
        if shmget(IPC_PRIVATE, 1, IPC_CREAT) == -1 {
            std::process::exit(1);
        }

        if pipe(fd.as_mut_ptr()) == -1 {
            std::process::exit(1);
        }

        PID = fork();
        if PID == -1 {
            std::process::exit(1);
        }

        if PID == 0 {
            if unshare(CLONE_NEWIPC) == -1 {
                std::process::exit(1);
            }

            if write(fd[1], &underscore as *const c_char as *const c_void, 1) != 1 {
                std::process::exit(1);
            }

            pause();

            std::process::exit(0);
        }

        if read(fd[0], &mut underscore as *mut c_char as *mut c_void, 1) != 1 {
            std::process::exit(1);
        }

        {
            let mut buf: [c_char; 64] = [0; 64];
            let format = c_string("/proc/%u/ns/ipc");
            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                format.as_ptr(),
                PID as c_uint,
            );
            nsfd = open(buf.as_ptr(), O_RDONLY);
            if nsfd == -1 {
                std::process::exit(1);
            }
        }

        /* Reliably pin dentry into dcache. */
        let proc_sysvipc_shm = c_string("/proc/sysvipc/shm");
        open(proc_sysvipc_shm.as_ptr(), O_RDONLY);

        if setns(nsfd, CLONE_NEWIPC) == -1 {
            std::process::exit(1);
        }

        kill(PID, SIGTERM);
        PID = 0;

        {
            let mut buf: [c_char; 4096] = [0; 4096];
            let rv: ssize_t;
            let fd: c_int;

            fd = open(proc_sysvipc_shm.as_ptr(), O_RDONLY);
            if fd == -1 {
                std::process::exit(1);
            }

            const S32: &[u8] = b"       key      shmid perms       size  cpid  lpid nattch   uid   gid  cuid  cgid      atime      dtime      ctime        rss       swap\n";
            const S64: &[u8] = b"       key      shmid perms                  size  cpid  lpid nattch   uid   gid  cuid  cgid      atime      dtime      ctime                   rss                  swap\n";
            rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
            if rv == S32.len() as ssize_t {
                assert!(
                    memcmp(
                        buf.as_ptr() as *const c_void,
                        S32.as_ptr() as *const c_void,
                        S32.len(),
                    ) == 0
                );
            } else if rv == S64.len() as ssize_t {
                assert!(
                    memcmp(
                        buf.as_ptr() as *const c_void,
                        S64.as_ptr() as *const c_void,
                        S64.len(),
                    ) == 0
                );
            } else {
                assert!(false);
            }
        }
    }

    let _ = ptr::null::<c_void>();
}

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
 * Test that setns(CLONE_NEWNET) points to new /proc/net content even
 * if old one is in dcache.
 *
 * FIXME /proc/net/unix is under CONFIG_UNIX which can be disabled.
 */

use std::mem;

static mut PID: libc::pid_t = -1;

extern "C" fn f() {
    unsafe {
        if PID > 0 {
            libc::kill(PID, libc::SIGTERM);
        }
    }
}

fn main() {
    std::process::exit(unsafe { main_0() });
}

unsafe fn main_0() -> i32 {
    let mut fd: [libc::c_int; 2] = [0; 2];
    let mut byte: libc::c_char = 0;
    let nsfd: libc::c_int;

    libc::atexit(f);

    /* Check for priviledges and syscall availability straight away. */
    if libc::unshare(libc::CLONE_NEWNET) == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::ENOSYS || errno == libc::EPERM {
            return 4;
        }
        return 1;
    }
    /* Distinguisher between two otherwise empty net namespaces. */
    if libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0) == -1 {
        return 1;
    }

    if libc::pipe(fd.as_mut_ptr()) == -1 {
        return 1;
    }

    PID = libc::fork();
    if PID == -1 {
        return 1;
    }

    if PID == 0 {
        if libc::unshare(libc::CLONE_NEWNET) == -1 {
            return 1;
        }

        if libc::write(fd[1], &mut byte as *mut libc::c_char as *const libc::c_void, 1) != 1 {
            return 1;
        }

        libc::pause();

        return 0;
    }

    if libc::read(fd[0], &mut byte as *mut libc::c_char as *mut libc::c_void, 1) != 1 {
        return 1;
    }

    {
        let mut buf: [libc::c_char; 64] = [0; 64];
        libc::snprintf(
            buf.as_mut_ptr(),
            mem::size_of_val(&buf),
            c"/proc/%u/ns/net".as_ptr(),
            PID as libc::c_uint,
        );
        nsfd = libc::open(buf.as_ptr(), libc::O_RDONLY);
        if nsfd == -1 {
            return 1;
        }
    }

    /* Reliably pin dentry into dcache. */
    libc::open(c"/proc/net/unix".as_ptr(), libc::O_RDONLY);

    if libc::setns(nsfd, libc::CLONE_NEWNET) == -1 {
        return 1;
    }

    libc::kill(PID, libc::SIGTERM);
    PID = 0;

    {
        let mut buf: [libc::c_char; 4096] = [0; 4096];
        let rv: libc::ssize_t;
        let fd: libc::c_int;

        fd = libc::open(c"/proc/net/unix".as_ptr(), libc::O_RDONLY);
        if fd == -1 {
            return 1;
        }

        const S: &str = "Num       RefCount Protocol Flags    Type St Inode Path\n";
        rv = libc::read(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            mem::size_of_val(&buf),
        );

        assert!(rv == S.len() as libc::ssize_t);
        assert!(
            libc::memcmp(
                buf.as_ptr() as *const libc::c_void,
                S.as_ptr() as *const libc::c_void,
                S.len(),
            ) == 0
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

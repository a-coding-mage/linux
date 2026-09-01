/*
 * Copyright (c) 2021 Alexey Dobriyan <adobriyan@gmail.com>
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
// Test that /proc/*/task never contains "0".
// C dependencies: sys/types.h, dirent.h, signal.h, stdio.h, stdlib.h,
// string.h, unistd.h, pthread.h.

use libc::{
    alarm, atexit, c_char, c_int, c_void, closedir, dirent, exit, fork, kill, opendir, perror,
    pthread_create, pthread_join, pthread_t, readdir, signal, snprintf, strcmp, DIR, NULL, SIGALRM,
    SIGKILL,
};

static mut pid: libc::pid_t = -1;

extern "C" fn atexit_hook() {
    unsafe {
        if pid > 0 {
            kill(pid, SIGKILL);
        }
    }
}

extern "C" fn f(_: *mut c_void) -> *mut c_void {
    NULL
}

extern "C" fn sigalrm(_: c_int) {
    unsafe {
        exit(0);
    }
}

fn main() -> c_int {
    unsafe {
        pid = fork();
        if pid == 0 {
            /* child */
            loop {
                let mut pth: pthread_t = std::mem::zeroed();
                pthread_create(&mut pth, NULL, f, NULL);
                pthread_join(pth, NULL);
            }
        } else if pid > 0 {
            /* parent */
            atexit(atexit_hook);

            let mut buf: [c_char; 64] = [0; 64];
            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"/proc/%u/task\0".as_ptr() as *const c_char,
                pid,
            );

            signal(SIGALRM, sigalrm as usize);
            alarm(1);

            loop {
                let d: *mut DIR = opendir(buf.as_ptr());
                let mut de: *mut dirent;
                loop {
                    de = readdir(d);
                    if de.is_null() {
                        break;
                    }
                    if strcmp((*de).d_name.as_ptr(), b"0\0".as_ptr() as *const c_char) == 0 {
                        exit(1);
                    }
                }
                closedir(d);
            }
        } else {
            perror(b"fork\0".as_ptr() as *const c_char);
            return 1;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

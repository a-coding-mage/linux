/* Copyright (C) 2006 by Paolo Giarrusso - modified from glibc' execvp.c.
   Original copyright notice follows:

   Copyright (C) 1991,92,1995-99,2002,2004 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, write to the Free
   Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
   02111-1307 USA.  */

// C dependencies: unistd.h, stdlib.h, string.h, errno.h, limits.h, and os.h.
// The TEST build condition from the C source is preserved below.

extern "C" {
    fn execv(file: *const libc::c_char, argv: *const *mut libc::c_char) -> libc::c_int;
    fn getenv(name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(s: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
}

/* Execute FILE, searching in the `PATH' environment variable if it contains
   no slashes, with arguments ARGV and environment from `environ'.  */
pub unsafe fn execvp_noalloc(
    buf: *mut libc::c_char,
    file: *const libc::c_char,
    argv: *const *mut libc::c_char,
) -> libc::c_int {
    if *file == 0 {
        return -libc::ENOENT;
    }

    if !strchr(file, b'/' as libc::c_int).is_null() {
        /* Don't search when it contains a slash.  */
        execv(file, argv);
    } else {
        let mut got_eacces: libc::c_int;
        let len: usize;
        let pathlen: usize;
        let name: *mut libc::c_char;
        let mut p: *mut libc::c_char;
        let mut path = getenv(b"PATH\0".as_ptr() as *const libc::c_char);
        if path.is_null() {
            path = b":/bin:/usr/bin\0".as_ptr() as *mut libc::c_char;
        }

        len = libc::strlen(file) + 1;
        pathlen = libc::strlen(path);
        /* Copy the file name at the top.  */
        libc::memcpy(
            buf.add(pathlen + 1) as *mut libc::c_void,
            file as *const libc::c_void,
            len,
        );
        name = buf.add(pathlen + 1);
        /* And add the slash.  */
        let name = name.sub(1);
        *name = b'/' as libc::c_char;

        got_eacces = 0;
        p = path;
        loop {
            let startp: *mut libc::c_char;

            path = p;
            // Let's avoid this GNU extension.
            // p = strchrnul(path, ':');
            p = strchr(path, b':' as libc::c_int);
            if p.is_null() {
                p = strchr(path, 0);
            }

            if p == path {
                /* Two adjacent colons, or a colon at the beginning or the end
                   of `PATH' means to search the current directory.  */
                startp = name.add(1);
            } else {
                let offset = p.offset_from(path) as usize;
                startp = name.sub(offset);
                libc::memcpy(
                    startp as *mut libc::c_void,
                    path as *const libc::c_void,
                    offset,
                );
            }

            /* Try to execute this name.  If it works, execv will not return.  */
            execv(startp, argv);

            /*
            if (errno == ENOEXEC) {
            }
            */

            match *libc::__errno_location() {
                libc::EACCES => {
                    /* Record the we got a `Permission denied' error.  If we end
                       up finding no executable we can use, we want to diagnose
                       that we did find one but were denied access.  */
                    got_eacces = 1;
                }
                libc::ENOENT | libc::ESTALE | libc::ENOTDIR => {
                    /* Those errors indicate the file is missing or not executable
                       by us, in which case we want to just try the next path
                       directory.  */
                }
                libc::ENODEV | libc::ETIMEDOUT => {
                    /* Some strange filesystems like AFS return even
                       stranger error numbers.  They cannot reasonably mean
                       anything else so ignore those, too.  */
                }
                libc::ENOEXEC => {
                    /* We won't go searching for the shell
                     * if it is not executable - the Linux
                     * kernel already handles this enough,
                     * for us. */
                }
                _ => {
                    /* Some other error means we found an executable file, but
                       something went wrong executing it; return the error to our
                       caller.  */
                    return -*libc::__errno_location();
                }
            }

            if *p == 0 {
                break;
            }
            p = p.add(1);
        }

        /* We tried every element and none of them worked.  */
        if got_eacces != 0 {
            /* At least one failure was due to permissions, so report that
               error.  */
            return -libc::EACCES;
        }
    }

    /* Return the error from the last attempt (probably ENOENT).  */
    -(*libc::__errno_location())
}

// The C source includes this test harness only when TEST is defined.
#[cfg(feature = "TEST")]
extern "C" {
    fn os_warn(message: *const libc::c_char);
}

#[cfg(feature = "TEST")]
pub unsafe fn main_c(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut buf = [0 as libc::c_char; libc::PATH_MAX as usize];
    let mut ret: libc::c_int;
    argc -= 1;
    if argc == 0 {
        os_warn(b"Not enough arguments\n\0".as_ptr() as *const libc::c_char);
        return 1;
    }
    argv = argv.add(1);
    ret = execvp_noalloc(buf.as_mut_ptr(), *argv, argv);
    if ret != 0 {
        *libc::__errno_location() = -ret;
        libc::perror(b"execvp_noalloc\0".as_ptr() as *const libc::c_char);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

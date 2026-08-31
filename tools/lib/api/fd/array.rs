// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */
// C dependencies: "array.h", <errno.h>, <fcntl.h>, <poll.h>, <stdlib.h>,
// <unistd.h>, <string.h>

use core::ffi::{c_char, c_int, c_short, c_void};

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

pub type fdarray_flags = c_int;

// Value supplied by array.h in the original C build.
unsafe extern "C" {
    pub static fdarray_flag__nonfilterable: fdarray_flags;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv_ {
    pub flags: fdarray_flags,
}

#[repr(C)]
pub struct fdarray {
    pub entries: *mut pollfd,
    pub priv_: *mut priv_,
    pub nr: c_int,
    pub nr_alloc: c_int,
    pub nr_autogrow: c_int,
}

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__init(fda: *mut fdarray, nr_autogrow: c_int) {
    unsafe {
        (*fda).entries = core::ptr::null_mut();
        (*fda).priv_ = core::ptr::null_mut();
        (*fda).nr = 0;
        (*fda).nr_alloc = 0;
        (*fda).nr_autogrow = nr_autogrow;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__grow(fda: *mut fdarray, nr: c_int) -> c_int {
    unsafe {
        let mut priv_ptr: *mut priv_;
        let nr_alloc: c_int = (*fda).nr_alloc + nr;
        let psize: usize = core::mem::size_of::<priv_>() * nr_alloc as usize;
        let size: usize = core::mem::size_of::<pollfd>() * nr_alloc as usize;
        let entries: *mut pollfd = realloc((*fda).entries as *mut c_void, size) as *mut pollfd;

        if entries.is_null() {
            return -ENOMEM;
        }

        priv_ptr = realloc((*fda).priv_ as *mut c_void, psize) as *mut priv_;
        if priv_ptr.is_null() {
            /* this will be freed by fdarray__exit() */
            (*fda).entries = entries;
            return -ENOMEM;
        }

        memset(
            entries.add((*fda).nr_alloc as usize) as *mut c_void,
            0,
            core::mem::size_of::<pollfd>() * nr as usize,
        );
        memset(
            priv_ptr.add((*fda).nr_alloc as usize) as *mut c_void,
            0,
            core::mem::size_of::<priv_>() * nr as usize,
        );

        (*fda).nr_alloc = nr_alloc;
        (*fda).entries = entries;
        (*fda).priv_ = priv_ptr;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__new(
    nr_alloc: c_int,
    nr_autogrow: c_int,
) -> *mut fdarray {
    unsafe {
        let mut fda: *mut fdarray = calloc(1, core::mem::size_of::<fdarray>()) as *mut fdarray;

        if !fda.is_null() {
            if fdarray__grow(fda, nr_alloc) != 0 {
                fdarray__delete(fda);
                fda = core::ptr::null_mut();
            } else {
                (*fda).nr_autogrow = nr_autogrow;
            }
        }

        fda
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__exit(fda: *mut fdarray) {
    unsafe {
        free((*fda).entries as *mut c_void);
        free((*fda).priv_ as *mut c_void);
        fdarray__init(fda, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__delete(fda: *mut fdarray) {
    unsafe {
        fdarray__exit(fda);
        free(fda as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__add(
    fda: *mut fdarray,
    fd: c_int,
    revents: c_short,
    flags: fdarray_flags,
) -> c_int {
    unsafe {
        let pos: c_int = (*fda).nr;

        if (*fda).nr == (*fda).nr_alloc && fdarray__grow(fda, (*fda).nr_autogrow) < 0 {
            return -ENOMEM;
        }

        (*(*fda).entries.add((*fda).nr as usize)).fd = fd;
        (*(*fda).entries.add((*fda).nr as usize)).events = revents;
        (*(*fda).priv_.add((*fda).nr as usize)).flags = flags;
        (*fda).nr += 1;
        pos
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__dup_entry_from(
    fda: *mut fdarray,
    pos: c_int,
    from: *mut fdarray,
) -> c_int {
    unsafe {
        let entry: *mut pollfd;
        let npos: c_int;

        if pos >= (*from).nr {
            return -EINVAL;
        }

        entry = (*from).entries.add(pos as usize);

        npos = fdarray__add(
            fda,
            (*entry).fd,
            (*entry).events,
            (*(*from).priv_.add(pos as usize)).flags,
        );
        if npos >= 0 {
            *(*fda).priv_.add(npos as usize) = *(*from).priv_.add(pos as usize);
        }

        npos
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__filter(
    fda: *mut fdarray,
    revents: c_short,
    entry_destructor: Option<unsafe extern "C" fn(*mut fdarray, c_int, *mut c_void)>,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        let mut fd: c_int;
        let mut nr: c_int = 0;

        if (*fda).nr == 0 {
            return 0;
        }

        fd = 0;
        while fd < (*fda).nr {
            if (*(*fda).priv_.add(fd as usize)).flags & fdarray_flag__nonfilterable != 0 {
                fd += 1;
                continue;
            }

            if (*(*fda).entries.add(fd as usize)).events == 0 {
                fd += 1;
                continue;
            }

            if (*(*fda).entries.add(fd as usize)).revents & revents != 0 {
                if let Some(entry_destructor_fn) = entry_destructor {
                    entry_destructor_fn(fda, fd, arg);
                }

                /*
                 * Set fd to -1 so poll() ignores this entry; otherwise
                 * POLLHUP/POLLERR are still reported for events=0 fds
                 * (POSIX: always checked), causing a poll storm.
                 */
                (*(*fda).entries.add(fd as usize)).fd = -1;
                (*(*fda).entries.add(fd as usize)).events = 0;
                (*(*fda).entries.add(fd as usize)).revents = (*(*fda).entries.add(fd as usize)).events;
                fd += 1;
                continue;
            }

            nr += 1;
            fd += 1;
        }

        nr
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__poll(fda: *mut fdarray, timeout: c_int) -> c_int {
    unsafe { poll((*fda).entries, (*fda).nr as usize, timeout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdarray__fprintf(fda: *mut fdarray, fp: *mut FILE) -> c_int {
    unsafe {
        let mut fd: c_int;
        let mut printed: c_int = fprintf(fp, c"%d [ ".as_ptr(), (*fda).nr);

        fd = 0;
        while fd < (*fda).nr {
            printed += fprintf(
                fp,
                c"%s%d".as_ptr(),
                if fd != 0 {
                    c", ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*fda).entries.add(fd as usize)).fd,
            );
            fd += 1;
        }

        printed + fprintf(fp, c" ]".as_ptr())
    }
}

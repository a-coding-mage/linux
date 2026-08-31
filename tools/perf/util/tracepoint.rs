// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/tracepoint.c.
// C includes:
// - "tracepoint.h"
// - <errno.h>
// - <fcntl.h>
// - <stdio.h>
// - <stdlib.h>
// - <sys/param.h>
// - <unistd.h>
// - <api/fs/tracing_path.h>
// - "fncache.h"

use core::ffi::{c_char, c_int, c_void};

pub const MAXPATHLEN: usize = 4096;
pub const O_RDONLY: c_int = 0;
pub const EINVAL: c_int = 22;

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn get_events_file(file: *const c_char) -> *mut c_char;
    fn file_available(path: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_event_has_id(
    dir_path: *const c_char,
    evt_dir: *mut dirent,
) -> c_int {
    let mut evt_path: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];
    let fd: c_int;

    unsafe {
        snprintf(
            evt_path.as_mut_ptr(),
            MAXPATHLEN,
            c"%s/%s/id".as_ptr(),
            dir_path,
            (*evt_dir).d_name.as_ptr(),
        );
        fd = open(evt_path.as_ptr(), O_RDONLY);
        if fd < 0 {
            return -EINVAL;
        }
        close(fd);
    }

    0
}

/*
 * Check whether event is in <debugfs_mount_point>/tracing/events
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_valid_tracepoint(event_string: *const c_char) -> bool {
    let mut dst: *mut c_char;
    let path: *mut c_char;
    let mut have_file: bool = false; /* Conservatively return false if memory allocation failed. */
    let mut src: *const c_char;

    unsafe {
        path = malloc(strlen(event_string) + 4) as *mut c_char; /* Space for "/id\0". */
        if path.is_null() {
            return false;
        }

        /* Copy event_string replacing the ':' with '/'. */
        src = event_string;
        dst = path;
        while *src != 0 {
            *dst = if *src == b':' as c_char {
                b'/' as c_char
            } else {
                *src
            };
            src = src.add(1);
            dst = dst.add(1);
        }
        /* Add "/id\0". */
        memcpy(dst as *mut c_void, c"/id".as_ptr() as *const c_void, 4);

        dst = get_events_file(path);
        if !dst.is_null() {
            have_file = file_available(dst);
        }
        free(dst as *mut c_void);
        free(path as *mut c_void);
    }

    have_file
}

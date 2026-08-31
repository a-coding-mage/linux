// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated as external declarations:
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type size_t = usize;
type ssize_t = isize;

extern "C" {
    static mut stderr: *mut FILE;

    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn free(ptr: *mut c_void);
    fn fclose(stream: *mut FILE) -> c_int;
}

unsafe fn find_map(start: *mut *mut c_void, end: *mut *mut c_void, name: *const c_char) -> c_int {
    let maps: *mut FILE;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut len: size_t = 0;
    let mut found: c_int = 0;

    maps = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if maps.is_null() {
        fprintf(stderr, c"cannot open maps\n".as_ptr());
        return -1;
    }

    while found == 0 && getline(&mut line, &mut len, maps) != -1 {
        let mut m: c_int = -1;

        /* We care only about private r-x mappings. */
        if 2
            != sscanf(
                line,
                c"%p-%p r-xp %*x %*x:%*x %*u %n".as_ptr(),
                start,
                end,
                &mut m,
            )
        {
            continue;
        }
        if m < 0 {
            continue;
        }

        if strncmp(line.offset(m as isize), name, strlen(name)) == 0 {
            found = 1;
        }
    }

    free(line as *mut c_void);
    fclose(maps);
    (found == 0) as c_int
}

// SPDX-License-Identifier: GPL-2.0
/*
 * fusectl test file-system
 * Creates a simple FUSE filesystem with a single read-write file (/test)
 */

/* C dependency intent:
 * #define FUSE_USE_VERSION 26
 * #include <fuse.h>
 * #include <stdio.h>
 * #include <string.h>
 * #include <errno.h>
 * #include <fcntl.h>
 * #include <stdlib.h>
 * #include <unistd.h>
 */

use core::mem;
use core::ptr;
use std::os::raw::{c_char, c_int, c_void};

type SizeT = usize;
type OffT = libc::off_t;

const S_IFDIR: libc::mode_t = 0o040000;
const S_IFREG: libc::mode_t = 0o100000;

fn MAX(a: SizeT, b: SizeT) -> SizeT {
    if a > b {
        a
    } else {
        b
    }
}

static mut content: *mut c_char = ptr::null_mut();
static mut content_size: SizeT = 0;
static test_path: &[u8; 6] = b"/test\0";

#[repr(C)]
pub struct fuse_file_info {
    _private: [u8; 0],
}

type fuse_fill_dir_t = Option<
    unsafe extern "C" fn(
        buf: *mut c_void,
        name: *const c_char,
        stbuf: *const libc::stat,
        off: OffT,
    ) -> c_int,
>;

#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option<unsafe extern "C" fn(path: *const c_char, st: *mut libc::stat) -> c_int>,
    pub readdir: Option<
        unsafe extern "C" fn(
            path: *const c_char,
            buf: *mut c_void,
            filler: fuse_fill_dir_t,
            offset: OffT,
            fi: *mut fuse_file_info,
        ) -> c_int,
    >,
    pub open: Option<unsafe extern "C" fn(path: *const c_char, fi: *mut fuse_file_info) -> c_int>,
    pub read: Option<
        unsafe extern "C" fn(
            path: *const c_char,
            buf: *mut c_char,
            size: SizeT,
            offset: OffT,
            fi: *mut fuse_file_info,
        ) -> c_int,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            path: *const c_char,
            buf: *const c_char,
            size: SizeT,
            offset: OffT,
            fi: *mut fuse_file_info,
        ) -> c_int,
    >,
    pub truncate: Option<unsafe extern "C" fn(path: *const c_char, size: OffT) -> c_int>,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fuse_main(
        argc: c_int,
        argv: *mut *mut c_char,
        op: *const fuse_operations,
        user_data: *mut c_void,
    ) -> c_int;
}

unsafe extern "C" fn test_getattr(path: *const c_char, st: *mut libc::stat) -> c_int {
    unsafe {
        memset(
            st as *mut c_void,
            0,
            mem::size_of_val(&*st) as SizeT,
        );

        if strcmp(path, c"/".as_ptr()) == 0 {
            (*st).st_mode = S_IFDIR | 0o755;
            (*st).st_nlink = 2;
            return 0;
        }

        if strcmp(path, test_path.as_ptr() as *const c_char) == 0 {
            (*st).st_mode = S_IFREG | 0o664;
            (*st).st_nlink = 1;
            (*st).st_size = content_size as _;
            return 0;
        }

        -libc::ENOENT
    }
}

unsafe extern "C" fn test_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: OffT,
    fi: *mut fuse_file_info,
) -> c_int {
    unsafe {
        let _ = offset;
        let _ = fi;

        if strcmp(path, c"/".as_ptr()) != 0 {
            return -libc::ENOENT;
        }

        if let Some(filler_fn) = filler {
            filler_fn(buf, c".".as_ptr(), ptr::null(), 0);
            filler_fn(buf, c"..".as_ptr(), ptr::null(), 0);
            filler_fn(buf, test_path.as_ptr().add(1) as *const c_char, ptr::null(), 0);
        }

        0
    }
}

unsafe extern "C" fn test_open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    unsafe {
        let _ = fi;

        if strcmp(path, test_path.as_ptr() as *const c_char) != 0 {
            return -libc::ENOENT;
        }

        0
    }
}

unsafe extern "C" fn test_read(
    path: *const c_char,
    buf: *mut c_char,
    mut size: SizeT,
    offset: OffT,
    fi: *mut fuse_file_info,
) -> c_int {
    unsafe {
        let _ = fi;

        if strcmp(path, test_path.as_ptr() as *const c_char) != 0 {
            return -libc::ENOENT;
        }

        if content.is_null() || content_size == 0 {
            return 0;
        }

        if offset as SizeT >= content_size {
            return 0;
        }

        if offset as SizeT + size > content_size {
            size = content_size - offset as SizeT;
        }

        memcpy(
            buf as *mut c_void,
            content.add(offset as SizeT) as *const c_void,
            size,
        );

        size as c_int
    }
}

unsafe extern "C" fn test_write(
    path: *const c_char,
    buf: *const c_char,
    size: SizeT,
    offset: OffT,
    fi: *mut fuse_file_info,
) -> c_int {
    unsafe {
        let _ = fi;
        let new_size: SizeT;

        if strcmp(path, test_path.as_ptr() as *const c_char) != 0 {
            return -libc::ENOENT;
        }

        if offset as SizeT > content_size {
            return -libc::EINVAL;
        }

        new_size = MAX(offset as SizeT + size, content_size);

        if new_size > content_size {
            content = realloc(content as *mut c_void, new_size) as *mut c_char;
        }

        content_size = new_size;

        if content.is_null() {
            return -libc::ENOMEM;
        }

        memcpy(
            content.add(offset as SizeT) as *mut c_void,
            buf as *const c_void,
            size,
        );

        size as c_int
    }
}

unsafe extern "C" fn test_truncate(path: *const c_char, size: OffT) -> c_int {
    unsafe {
        if strcmp(path, test_path.as_ptr() as *const c_char) != 0 {
            return -libc::ENOENT;
        }

        if size == 0 {
            free(content as *mut c_void);
            content = ptr::null_mut();
            content_size = 0;
            return 0;
        }

        content = realloc(content as *mut c_void, size as SizeT) as *mut c_char;

        if content.is_null() {
            return -libc::ENOMEM;
        }

        if size as SizeT > content_size {
            memset(
                content.add(content_size) as *mut c_void,
                0,
                size as SizeT - content_size,
            );
        }

        content_size = size as SizeT;
        0
    }
}

static mut memfd_ops: fuse_operations = fuse_operations {
    getattr: Some(test_getattr),
    readdir: Some(test_readdir),
    open: Some(test_open),
    read: Some(test_read),
    write: Some(test_write),
    truncate: Some(test_truncate),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe { fuse_main(argc, argv, &raw const memfd_ops, ptr::null_mut()) }
}

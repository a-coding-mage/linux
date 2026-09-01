// SPDX-License-Identifier: GPL-2.0
/*
 * memfd test file-system
 * This file uses FUSE to create a dummy file-system with only one file /memfd.
 * This file is read-only and takes 1s per read.
 *
 * This file-system is used by the memfd test-cases to force the kernel to pin
 * pages during reads(). Due to the 1s delay of this file-system, this is a
 * nice way to test race-conditions against get_user_pages() in the kernel.
 *
 * We use direct_io==1 to force the kernel to use direct-IO for this
 * file-system.
 */

/* C source defined FUSE_USE_VERSION as 26 before including <fuse.h>. */

use libc::{
    c_char, c_int, c_void, memcpy, memset, off_t, size_t, sleep, stat, strcmp, strlen, EACCES,
    ENOENT, O_RDONLY, S_IFDIR, S_IFREG,
};

type fuse_fill_dir_t =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, *const stat, off_t) -> c_int>;

#[repr(C)]
pub struct fuse_file_info {
    pub flags: c_int,
    pub direct_io: u32,
}

#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
    pub readdir: Option<
        unsafe extern "C" fn(
            *const c_char,
            *mut c_void,
            fuse_fill_dir_t,
            off_t,
            *mut fuse_file_info,
        ) -> c_int,
    >,
    pub open: Option<unsafe extern "C" fn(*const c_char, *mut fuse_file_info) -> c_int>,
    pub read: Option<
        unsafe extern "C" fn(
            *const c_char,
            *mut c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    fn fuse_main(
        argc: c_int,
        argv: *mut *mut c_char,
        op: *const fuse_operations,
        user_data: *mut c_void,
    ) -> c_int;
}

static MEMFD_CONTENT: &[u8; 22] = b"memfd-example-content\0";
static MEMFD_PATH: &[u8; 7] = b"/memfd\0";

unsafe extern "C" fn memfd_getattr(path: *const c_char, st: *mut stat) -> c_int {
    unsafe {
        memset(
            st as *mut c_void,
            0,
            ::core::mem::size_of_val(&*st) as size_t,
        );

        if strcmp(path, b"/\0".as_ptr() as *const c_char) == 0 {
            (*st).st_mode = S_IFDIR | 0o755;
            (*st).st_nlink = 2;
        } else if strcmp(path, MEMFD_PATH.as_ptr() as *const c_char) == 0 {
            (*st).st_mode = S_IFREG | 0o444;
            (*st).st_nlink = 1;
            (*st).st_size = strlen(MEMFD_CONTENT.as_ptr() as *const c_char) as _;
        } else {
            return -ENOENT;
        }

        0
    }
}

unsafe extern "C" fn memfd_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    unsafe {
        let _ = offset;
        let _ = fi;

        if strcmp(path, b"/\0".as_ptr() as *const c_char) != 0 {
            return -ENOENT;
        }

        if let Some(filler_fn) = filler {
            filler_fn(buf, b".\0".as_ptr() as *const c_char, ::core::ptr::null(), 0);
            filler_fn(buf, b"..\0".as_ptr() as *const c_char, ::core::ptr::null(), 0);
            filler_fn(
                buf,
                MEMFD_PATH.as_ptr().add(1) as *const c_char,
                ::core::ptr::null(),
                0,
            );
        }

        0
    }
}

unsafe extern "C" fn memfd_open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    unsafe {
        if strcmp(path, MEMFD_PATH.as_ptr() as *const c_char) != 0 {
            return -ENOENT;
        }

        if ((*fi).flags & 3) != O_RDONLY {
            return -EACCES;
        }

        /* force direct-IO */
        (*fi).direct_io = 1;

        0
    }
}

unsafe extern "C" fn memfd_read(
    path: *const c_char,
    buf: *mut c_char,
    mut size: size_t,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    unsafe {
        let _ = fi;
        let len: size_t;

        if strcmp(path, MEMFD_PATH.as_ptr() as *const c_char) != 0 {
            return -ENOENT;
        }

        sleep(1);

        len = strlen(MEMFD_CONTENT.as_ptr() as *const c_char);
        if (offset as size_t) < len {
            if (offset as size_t).wrapping_add(size) > len {
                size = len.wrapping_sub(offset as size_t);
            }

            memcpy(
                buf as *mut c_void,
                MEMFD_CONTENT.as_ptr().add(offset as usize) as *const c_void,
                size,
            );
        } else {
            size = 0;
        }

        size as c_int
    }
}

static mut MEMFD_OPS: fuse_operations = fuse_operations {
    getattr: Some(memfd_getattr),
    readdir: Some(memfd_readdir),
    open: Some(memfd_open),
    read: Some(memfd_read),
};

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe { fuse_main(argc, argv, &raw const MEMFD_OPS, ::core::ptr::null_mut()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

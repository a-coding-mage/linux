/*
 *  sync / sw_sync abstraction
 *  Copyright 2015-2016 Collabora Ltd.
 *
 *  Based on the implementation from the Android Open Source Project,
 *
 *  Copyright 2012 Google, Inc
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;

const O_RDWR: c_int = 0o2;
const F_GETFD: c_int = 1;
const POLLIN: i16 = 0x0001;
const POLLERR: i16 = 0x0008;

const IOC_NRBITS: c_ulong = 8;
const IOC_TYPEBITS: c_ulong = 8;
const IOC_SIZEBITS: c_ulong = 14;
const IOC_DIRBITS: c_ulong = 2;

const IOC_NRSHIFT: c_ulong = 0;
const IOC_TYPESHIFT: c_ulong = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_ulong = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_ulong = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_WRITE: c_ulong = 1;
const IOC_READ: c_ulong = 2;

const fn _ioc(dir: c_ulong, type_: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn _iow<T>(type_: c_ulong, nr: c_ulong) -> c_ulong {
    _ioc(IOC_WRITE, type_, nr, core::mem::size_of::<T>() as c_ulong)
}

const fn _iowr<T>(type_: c_ulong, nr: c_ulong) -> c_ulong {
    _ioc(IOC_READ | IOC_WRITE, type_, nr, core::mem::size_of::<T>() as c_ulong)
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: i16,
    pub revents: i16,
}

#[repr(C)]
pub struct sync_merge_data {
    pub name: [c_char; 32],
    pub fd2: __s32,
    pub fence: __s32,
    pub flags: __u32,
    pub pad: __u32,
}

#[repr(C)]
pub struct sync_fence_info {
    pub obj_name: [c_char; 32],
    pub driver_name: [c_char; 32],
    pub status: __s32,
    pub flags: __u32,
    pub timestamp_ns: __u64,
}

#[repr(C)]
pub struct sync_file_info {
    pub name: [c_char; 32],
    pub status: __s32,
    pub flags: __u32,
    pub num_fences: __u32,
    pub pad: __u32,
    pub sync_fence_info: __u64,
}

/* SW_SYNC ioctls */
#[repr(C)]
pub struct sw_sync_create_fence_data {
    pub value: __u32,
    pub name: [c_char; 32],
    pub fence: __s32,
}

const SYNC_IOC_MAGIC: c_ulong = b'>' as c_ulong;
const SYNC_IOC_MERGE: c_ulong = _iowr::<sync_merge_data>(SYNC_IOC_MAGIC, 3);
const SYNC_IOC_FILE_INFO: c_ulong = _iowr::<sync_file_info>(SYNC_IOC_MAGIC, 4);

const SW_SYNC_IOC_MAGIC: c_ulong = b'W' as c_ulong;
const SW_SYNC_IOC_CREATE_FENCE: c_ulong =
    _iowr::<sw_sync_create_fence_data>(SW_SYNC_IOC_MAGIC, 0);
const SW_SYNC_IOC_INC: c_ulong = _iow::<__u32>(SW_SYNC_IOC_MAGIC, 1);

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sync_wait(fd: c_int, timeout: c_int) -> c_int {
    let mut fds = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };

    fds.fd = fd;
    fds.events = POLLIN | POLLERR;

    unsafe { poll(&mut fds, 1, timeout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sync_merge(name: *const c_char, fd1: c_int, fd2: c_int) -> c_int {
    let mut data: sync_merge_data = unsafe { core::mem::zeroed() };
    let err: c_int;

    data.fd2 = fd2;
    unsafe {
        strncpy(data.name.as_mut_ptr(), name, core::mem::size_of_val(&data.name) - 1);
    }
    data.name[core::mem::size_of_val(&data.name) - 1] = b'\0' as c_char;

    err = unsafe { ioctl(fd1, SYNC_IOC_MERGE, &mut data) };
    if err < 0 {
        return err;
    }

    data.fence
}

unsafe fn sync_file_info(fd: c_int) -> *mut sync_file_info {
    let info: *mut sync_file_info;
    let mut fence_info: *mut sync_fence_info;
    let mut err: c_int;
    let num_fences: c_int;

    info = unsafe { calloc(1, core::mem::size_of::<sync_file_info>()) as *mut sync_file_info };
    if info.is_null() {
        return core::ptr::null_mut();
    }

    err = unsafe { ioctl(fd, SYNC_IOC_FILE_INFO, info) };
    if err < 0 {
        unsafe {
            free(info as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    num_fences = unsafe { (*info).num_fences as c_int };

    if num_fences != 0 {
        unsafe {
            (*info).flags = 0;
            (*info).num_fences = num_fences as __u32;
        }

        fence_info = unsafe {
            calloc(
                num_fences as usize,
                core::mem::size_of::<sync_fence_info>(),
            ) as *mut sync_fence_info
        };
        if fence_info.is_null() {
            unsafe {
                free(info as *mut c_void);
            }
            return core::ptr::null_mut();
        }

        unsafe {
            (*info).sync_fence_info = fence_info as c_ulong as __u64;
        }

        err = unsafe { ioctl(fd, SYNC_IOC_FILE_INFO, info) };
        if err < 0 {
            unsafe {
                free(fence_info as *mut c_void);
                free(info as *mut c_void);
            }
            return core::ptr::null_mut();
        }
    }

    info
}

unsafe fn sync_file_info_free(info: *mut sync_file_info) {
    unsafe {
        free((*info).sync_fence_info as c_ulong as *mut c_void);
        free(info as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sync_fence_size(fd: c_int) -> c_int {
    let count: c_int;
    let info: *mut sync_file_info = unsafe { sync_file_info(fd) };

    if info.is_null() {
        return 0;
    }

    count = unsafe { (*info).num_fences as c_int };

    unsafe {
        sync_file_info_free(info);
    }

    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sync_fence_count_with_status(fd: c_int, status: c_int) -> c_int {
    let mut i: __u32;
    let mut count: __u32 = 0;
    let mut fence_info: *mut sync_fence_info = core::ptr::null_mut();
    let info: *mut sync_file_info = unsafe { sync_file_info(fd) };

    if info.is_null() {
        return -1;
    }

    fence_info = unsafe { (*info).sync_fence_info as c_ulong as *mut sync_fence_info };
    i = 0;
    while i < unsafe { (*info).num_fences } {
        if unsafe { (*fence_info.add(i as usize)).status == status } {
            count += 1;
        }
        i += 1;
    }

    unsafe {
        sync_file_info_free(info);
    }

    count as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_timeline_create() -> c_int {
    unsafe { open(c"/sys/kernel/debug/sync/sw_sync".as_ptr(), O_RDWR) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_timeline_inc(fd: c_int, count: __u32) -> c_int {
    let mut arg: __u32 = count;

    unsafe { ioctl(fd, SW_SYNC_IOC_INC, &mut arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_timeline_is_valid(fd: c_int) -> c_int {
    let status: c_int;

    if fd == -1 {
        return 0;
    }

    status = unsafe { fcntl(fd, F_GETFD, 0) };
    (status >= 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_timeline_destroy(fd: c_int) {
    if unsafe { sw_sync_timeline_is_valid(fd) } != 0 {
        unsafe {
            close(fd);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_fence_create(
    fd: c_int,
    name: *const c_char,
    value: __u32,
) -> c_int {
    let mut data: sw_sync_create_fence_data = unsafe { core::mem::zeroed() };
    let err: c_int;

    data.value = value;
    unsafe {
        strncpy(data.name.as_mut_ptr(), name, core::mem::size_of_val(&data.name) - 1);
    }
    data.name[core::mem::size_of_val(&data.name) - 1] = b'\0' as c_char;

    err = unsafe { ioctl(fd, SW_SYNC_IOC_CREATE_FENCE, &mut data) };
    if err < 0 {
        return err;
    }

    data.fence
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_fence_is_valid(fd: c_int) -> c_int {
    /* Same code! */
    unsafe { sw_sync_timeline_is_valid(fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sw_sync_fence_destroy(fd: c_int) {
    if unsafe { sw_sync_fence_is_valid(fd) } != 0 {
        unsafe {
            close(fd);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

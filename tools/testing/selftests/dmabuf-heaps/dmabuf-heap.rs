// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type __u32 = u32;
type __u64 = u64;

const DEVPATH: &[u8] = b"/dev/dma_heap\0";
const ONE_MEG: size_t = 1024 * 1024;

const O_RDWR: c_int = 0o00000002;
const O_CLOEXEC: c_int = 0o2000000;
const EINVAL: c_int = 22;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const KSFT_SKIP: c_int = 4;

const DMA_BUF_SYNC_READ: __u64 = 1 << 0;
const DMA_BUF_SYNC_WRITE: __u64 = 2 << 0;
const DMA_BUF_SYNC_RW: __u64 = DMA_BUF_SYNC_READ | DMA_BUF_SYNC_WRITE;
const DMA_BUF_SYNC_START: c_int = 0 << 2;
const DMA_BUF_SYNC_END: c_int = 1 << 2;

const DMA_HEAP_IOC_MAGIC: c_uint = b'H' as c_uint;
const DRM_IOCTL_BASE: c_uint = b'd' as c_uint;
const DRM_COMMAND_BASE: c_uint = 0x40;

const IOC_NRBITS: c_uint = 8;
const IOC_TYPEBITS: c_uint = 8;
const IOC_SIZEBITS: c_uint = 14;
const IOC_DIRBITS: c_uint = 2;
const IOC_NRSHIFT: c_uint = 0;
const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: c_uint = 1;
const IOC_READ: c_uint = 2;

const fn _IOC(dir: c_uint, typ: c_uint, nr: c_uint, size: c_uint) -> c_uint {
    (dir << IOC_DIRSHIFT) | (typ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn _IOWR<T>(typ: c_uint, nr: c_uint) -> c_uint {
    _IOC(IOC_READ | IOC_WRITE, typ, nr, size_of::<T>() as c_uint)
}

#[repr(C)]
struct drm_version_t {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
    name_len: size_t,
    name: *mut c_char,
    date_len: size_t,
    date: *mut c_char,
    desc_len: size_t,
    desc: *mut c_char,
}

#[repr(C)]
struct drm_prime_handle {
    handle: __u32,
    flags: __u32,
    fd: c_int,
}

#[repr(C)]
struct drm_gem_close {
    handle: __u32,
    pad: __u32,
}

#[repr(C)]
struct dma_heap_allocation_data {
    len: __u64,
    fd: __u32,
    fd_flags: __u32,
    heap_flags: __u64,
}

#[repr(C)]
struct dma_buf_sync {
    flags: __u64,
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: c_ulong,
    d_off: c_long,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

const DRM_IOCTL_VERSION: c_ulong = _IOWR::<drm_version_t>(DRM_IOCTL_BASE, 0x00) as c_ulong;
const DRM_IOCTL_GEM_CLOSE: c_ulong =
    _IOWR::<drm_gem_close>(DRM_IOCTL_BASE, 0x09) as c_ulong;
const DRM_IOCTL_PRIME_FD_TO_HANDLE: c_ulong =
    _IOWR::<drm_prime_handle>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x2e) as c_ulong;
const DMA_HEAP_IOCTL_ALLOC: c_ulong =
    _IOWR::<dma_heap_allocation_data>(DMA_HEAP_IOC_MAGIC, 0x0) as c_ulong;
const DMA_BUF_IOCTL_SYNC: c_ulong = _IOWR::<dma_buf_sync>(b'b' as c_uint, 0) as c_ulong;

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;

    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result(condition: c_int, format: *const c_char, ...);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_finished();
}

fn MAP_FAILED() -> *mut c_void {
    (-1isize) as *mut c_void
}

unsafe fn check_vgem(fd: c_int) -> c_int {
    let mut version: drm_version_t = core::mem::zeroed();
    let mut name: [c_char; 5] = [0; 5];
    let ret: c_int;

    version.name_len = 4;
    version.name = name.as_mut_ptr();

    ret = ioctl(fd, DRM_IOCTL_VERSION, &mut version);
    if ret != 0 || version.name_len != 4 {
        return 0;
    }

    name[4] = b'\0' as c_char;

    (strcmp(name.as_ptr(), c"vgem".as_ptr()) == 0) as c_int
}

unsafe fn open_vgem() -> c_int {
    let mut i: c_int;
    let mut fd: c_int;
    let drmstr = c"/dev/dri/card";

    fd = -1;
    i = 0;
    while i < 16 {
        let mut name: [c_char; 80] = [0; 80];

        snprintf(name.as_mut_ptr(), 80, c"%s%u".as_ptr(), drmstr.as_ptr(), i as c_uint);

        fd = open(name.as_ptr(), O_RDWR);
        if fd < 0 {
            i += 1;
            continue;
        }

        if check_vgem(fd) == 0 {
            close(fd);
            fd = -1;
            i += 1;
            continue;
        } else {
            break;
        }
    }
    fd
}

unsafe fn import_vgem_fd(vgem_fd: c_int, dma_buf_fd: c_int, handle: *mut u32) -> c_int {
    let mut import_handle = drm_prime_handle {
        fd: dma_buf_fd,
        flags: 0,
        handle: 0,
    };
    let ret: c_int;

    ret = ioctl(
        vgem_fd,
        DRM_IOCTL_PRIME_FD_TO_HANDLE,
        &mut import_handle,
    );
    if ret == 0 {
        *handle = import_handle.handle;
    }
    ret
}

unsafe fn close_handle(vgem_fd: c_int, handle: u32) {
    let mut close_data = drm_gem_close { handle, pad: 0 };

    ioctl(vgem_fd, DRM_IOCTL_GEM_CLOSE, &mut close_data);
}

unsafe fn dmabuf_heap_open(name: *mut c_char) -> c_int {
    let ret: c_int;
    let fd: c_int;
    let mut buf: [c_char; 256] = [0; 256];

    ret = snprintf(
        buf.as_mut_ptr(),
        256,
        c"%s/%s".as_ptr(),
        DEVPATH.as_ptr() as *const c_char,
        name,
    );
    if ret < 0 {
        ksft_exit_fail_msg(c"snprintf failed! %d\n".as_ptr(), ret);
    }

    fd = open(buf.as_ptr(), O_RDWR);
    if fd < 0 {
        ksft_exit_fail_msg(
            c"open %s failed: %s\n".as_ptr(),
            buf.as_ptr(),
            strerror(errno),
        );
    }

    fd
}

unsafe fn dmabuf_heap_alloc_fdflags(
    fd: c_int,
    len: size_t,
    fd_flags: c_uint,
    heap_flags: c_uint,
    dmabuf_fd: *mut c_int,
) -> c_int {
    let mut data = dma_heap_allocation_data {
        len: len as __u64,
        fd: 0,
        fd_flags,
        heap_flags: heap_flags as __u64,
    };
    let ret: c_int;

    if dmabuf_fd.is_null() {
        return -EINVAL;
    }

    ret = ioctl(fd, DMA_HEAP_IOCTL_ALLOC, &mut data);
    if ret < 0 {
        return ret;
    }
    *dmabuf_fd = data.fd as c_int;
    ret
}

unsafe fn dmabuf_heap_alloc(fd: c_int, len: size_t, flags: c_uint, dmabuf_fd: *mut c_int) -> c_int {
    dmabuf_heap_alloc_fdflags(fd, len, (O_RDWR | O_CLOEXEC) as c_uint, flags, dmabuf_fd)
}

unsafe fn dmabuf_sync(fd: c_int, start_stop: c_int) -> c_int {
    let mut sync = dma_buf_sync {
        flags: (start_stop as __u64) | DMA_BUF_SYNC_RW,
    };

    ioctl(fd, DMA_BUF_IOCTL_SYNC, &mut sync)
}

unsafe fn test_alloc_and_import(heap_name: *mut c_char) {
    let mut heap_fd: c_int = -1;
    let mut dmabuf_fd: c_int = -1;
    let mut importer_fd: c_int = -1;
    let mut handle: u32 = 0;
    let mut p: *mut c_void = ptr::null_mut();
    let mut ret: c_int;

    heap_fd = dmabuf_heap_open(heap_name);

    ksft_print_msg(c"Testing allocation and importing:\n".as_ptr());
    ret = dmabuf_heap_alloc(heap_fd, ONE_MEG, 0, &mut dmabuf_fd);
    if ret != 0 {
        ksft_test_result_fail(c"FAIL (Allocation Failed!) %d\n".as_ptr(), ret);
        return;
    }

    /* mmap and write a simple pattern */
    p = mmap(
        ptr::null_mut(),
        ONE_MEG,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        dmabuf_fd,
        0,
    );
    if p == MAP_FAILED() {
        ksft_test_result_fail(
            c"FAIL (mmap() failed): %s\n".as_ptr(),
            strerror(errno),
        );
        close(dmabuf_fd);
        close(heap_fd);
        return;
    }

    dmabuf_sync(dmabuf_fd, DMA_BUF_SYNC_START);
    memset(p, 1, ONE_MEG / 2);
    memset((p as *mut c_char).add(ONE_MEG / 2) as *mut c_void, 0, ONE_MEG / 2);
    dmabuf_sync(dmabuf_fd, DMA_BUF_SYNC_END);

    importer_fd = open_vgem();
    if importer_fd < 0 {
        ksft_test_result_skip(c"Could not open vgem %d\n".as_ptr(), importer_fd);
    } else {
        ret = import_vgem_fd(importer_fd, dmabuf_fd, &mut handle);
        ksft_test_result((ret >= 0) as c_int, c"Import buffer %d\n".as_ptr(), ret);
    }

    ret = dmabuf_sync(dmabuf_fd, DMA_BUF_SYNC_START);
    if ret < 0 {
        ksft_print_msg(c"FAIL (DMA_BUF_SYNC_START failed!) %d\n".as_ptr(), ret);
        ksft_test_result_fail(c"%s dmabuf sync failed\n".as_ptr(), c"test_alloc_and_import".as_ptr());
        munmap(p, ONE_MEG);
        close(importer_fd);
        close(dmabuf_fd);
        close(heap_fd);
        return;
    }

    memset(p, 0xff, ONE_MEG);
    ret = dmabuf_sync(dmabuf_fd, DMA_BUF_SYNC_END);
    if ret < 0 {
        ksft_print_msg(c"FAIL (DMA_BUF_SYNC_END failed!) %d\n".as_ptr(), ret);
        ksft_test_result_fail(c"%s dmabuf sync failed\n".as_ptr(), c"test_alloc_and_import".as_ptr());
        munmap(p, ONE_MEG);
        close(importer_fd);
        close(dmabuf_fd);
        close(heap_fd);
        return;
    }

    close_handle(importer_fd, handle);
    ksft_test_result_pass(c"%s dmabuf sync succeeded\n".as_ptr(), c"test_alloc_and_import".as_ptr());
}

unsafe fn test_alloc_zeroed(heap_name: *mut c_char, size: size_t) {
    let mut heap_fd: c_int = -1;
    let mut dmabuf_fd: [c_int; 32] = [0; 32];
    let mut i: c_int = 0;
    let mut j: c_int;
    let mut k: c_int;
    let mut ret: c_int;
    let mut p: *mut c_void = ptr::null_mut();
    let mut c: *mut c_char;

    ksft_print_msg(c"Testing alloced %ldk buffers are zeroed:\n".as_ptr(), size / 1024);
    heap_fd = dmabuf_heap_open(heap_name);

    /* Allocate and fill a bunch of buffers */
    while i < 32 {
        ret = dmabuf_heap_alloc(heap_fd, size, 0, &mut dmabuf_fd[i as usize]);
        if ret != 0 {
            ksft_test_result_fail(c"FAIL (Allocation (%i) failed) %d\n".as_ptr(), i, ret);
            k = 0;
            while k < i {
                close(dmabuf_fd[k as usize]);
                k += 1;
            }
            close(heap_fd);
            return;
        }

        /* mmap and fill with simple pattern */
        p = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            dmabuf_fd[i as usize],
            0,
        );
        if p == MAP_FAILED() {
            ksft_test_result_fail(c"FAIL (mmap() failed!): %s\n".as_ptr(), strerror(errno));
            k = 0;
            while k < i {
                close(dmabuf_fd[k as usize]);
                k += 1;
            }
            close(heap_fd);
            return;
        }

        dmabuf_sync(dmabuf_fd[i as usize], DMA_BUF_SYNC_START);
        memset(p, 0xff, size);
        dmabuf_sync(dmabuf_fd[i as usize], DMA_BUF_SYNC_END);
        munmap(p, size);
        i += 1;
    }
    /* close them all */
    i = 0;
    while i < 32 {
        close(dmabuf_fd[i as usize]);
        i += 1;
    }
    ksft_test_result_pass(c"Allocate and fill a bunch of buffers\n".as_ptr());

    /* Allocate and validate all buffers are zeroed */
    i = 0;
    while i < 32 {
        ret = dmabuf_heap_alloc(heap_fd, size, 0, &mut dmabuf_fd[i as usize]);
        if ret < 0 {
            ksft_test_result_fail(c"FAIL (Allocation (%i) failed) %d\n".as_ptr(), i, ret);
            k = 0;
            while k < i {
                close(dmabuf_fd[k as usize]);
                k += 1;
            }
            close(heap_fd);
            return;
        }

        /* mmap and validate everything is zero */
        p = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            dmabuf_fd[i as usize],
            0,
        );
        if p == MAP_FAILED() {
            ksft_test_result_fail(c"FAIL (mmap() failed!): %s\n".as_ptr(), strerror(errno));
            k = 0;
            while k < i {
                close(dmabuf_fd[k as usize]);
                k += 1;
            }
            close(heap_fd);
            return;
        }

        dmabuf_sync(dmabuf_fd[i as usize], DMA_BUF_SYNC_START);
        c = p as *mut c_char;
        j = 0;
        while (j as size_t) < size {
            if *c.add(j as usize) != 0 {
                ksft_print_msg(c"FAIL (Allocated buffer not zeroed @ %i)\n".as_ptr(), j);
                dmabuf_sync(dmabuf_fd[i as usize], DMA_BUF_SYNC_END);
                munmap(p, size);
                break;
            }
            j += 1;
        }
        if (j as size_t) < size {
            break;
        }
        dmabuf_sync(dmabuf_fd[i as usize], DMA_BUF_SYNC_END);
        munmap(p, size);
        i += 1;
    }

    ksft_test_result((i == 32) as c_int, c"Allocate and validate all buffers are zeroed\n".as_ptr());

    /* close them all */
    k = 0;
    while k < i {
        close(dmabuf_fd[k as usize]);
        k += 1;
    }

    close(heap_fd);
}

/* Test the ioctl version compatibility w/ a smaller structure then expected */
#[repr(C)]
struct dma_heap_allocation_data_smaller {
    len: __u64,
    fd: __u32,
    fd_flags: __u32,
}

unsafe fn dmabuf_heap_alloc_older(
    fd: c_int,
    len: size_t,
    _flags: c_uint,
    dmabuf_fd: *mut c_int,
) -> c_int {
    let ret: c_int;
    let older_alloc_ioctl: c_uint;
    let mut data = dma_heap_allocation_data_smaller {
        len: len as __u64,
        fd: 0,
        fd_flags: (O_RDWR | O_CLOEXEC) as __u32,
    };

    older_alloc_ioctl = _IOWR::<dma_heap_allocation_data_smaller>(DMA_HEAP_IOC_MAGIC, 0x0);
    if dmabuf_fd.is_null() {
        return -EINVAL;
    }

    ret = ioctl(fd, older_alloc_ioctl as c_ulong, &mut data);
    if ret < 0 {
        return ret;
    }
    *dmabuf_fd = data.fd as c_int;
    ret
}

/* Test the ioctl version compatibility w/ a larger structure then expected */
#[repr(C)]
struct dma_heap_allocation_data_bigger {
    len: __u64,
    fd: __u32,
    fd_flags: __u32,
    heap_flags: __u64,
    garbage1: __u64,
    garbage2: __u64,
    garbage3: __u64,
}

unsafe fn dmabuf_heap_alloc_newer(
    fd: c_int,
    len: size_t,
    flags: c_uint,
    dmabuf_fd: *mut c_int,
) -> c_int {
    let ret: c_int;
    let newer_alloc_ioctl: c_uint;
    let mut data = dma_heap_allocation_data_bigger {
        len: len as __u64,
        fd: 0,
        fd_flags: (O_RDWR | O_CLOEXEC) as __u32,
        heap_flags: flags as __u64,
        garbage1: 0xffffffff,
        garbage2: 0x88888888,
        garbage3: 0x11111111,
    };

    newer_alloc_ioctl = _IOWR::<dma_heap_allocation_data_bigger>(DMA_HEAP_IOC_MAGIC, 0x0);
    if dmabuf_fd.is_null() {
        return -EINVAL;
    }

    ret = ioctl(fd, newer_alloc_ioctl as c_ulong, &mut data);
    if ret < 0 {
        return ret;
    }

    *dmabuf_fd = data.fd as c_int;
    ret
}

unsafe fn test_alloc_compat(heap_name: *mut c_char) {
    let mut ret: c_int;
    let mut heap_fd: c_int = -1;
    let mut dmabuf_fd: c_int = -1;

    heap_fd = dmabuf_heap_open(heap_name);

    ksft_print_msg(c"Testing (theoretical) older alloc compat:\n".as_ptr());
    ret = dmabuf_heap_alloc_older(heap_fd, ONE_MEG, 0, &mut dmabuf_fd);
    if dmabuf_fd >= 0 {
        close(dmabuf_fd);
    }
    ksft_test_result((ret == 0) as c_int, c"dmabuf_heap_alloc_older\n".as_ptr());

    ksft_print_msg(c"Testing (theoretical) newer alloc compat:\n".as_ptr());
    ret = dmabuf_heap_alloc_newer(heap_fd, ONE_MEG, 0, &mut dmabuf_fd);
    if dmabuf_fd >= 0 {
        close(dmabuf_fd);
    }
    ksft_test_result((ret == 0) as c_int, c"dmabuf_heap_alloc_newer\n".as_ptr());

    close(heap_fd);
}

unsafe fn test_alloc_errors(heap_name: *mut c_char) {
    let mut heap_fd: c_int = -1;
    let mut dmabuf_fd: c_int = -1;
    let mut ret: c_int;

    heap_fd = dmabuf_heap_open(heap_name);

    ksft_print_msg(c"Testing expected error cases:\n".as_ptr());
    ret = dmabuf_heap_alloc(0, ONE_MEG, 0x111111, &mut dmabuf_fd);
    ksft_test_result((ret != 0) as c_int, c"Error expected on invalid fd %d\n".as_ptr(), ret);

    ret = dmabuf_heap_alloc(heap_fd, ONE_MEG, 0x111111, &mut dmabuf_fd);
    ksft_test_result((ret != 0) as c_int, c"Error expected on invalid heap flags %d\n".as_ptr(), ret);

    ret = dmabuf_heap_alloc_fdflags(
        heap_fd,
        ONE_MEG,
        !(O_RDWR | O_CLOEXEC) as c_uint,
        0,
        &mut dmabuf_fd,
    );
    ksft_test_result((ret != 0) as c_int, c"Error expected on invalid heap flags %d\n".as_ptr(), ret);

    if dmabuf_fd >= 0 {
        close(dmabuf_fd);
    }
    close(heap_fd);
}

unsafe fn numer_of_heaps() -> c_int {
    let d: *mut DIR = opendir(DEVPATH.as_ptr() as *const c_char);
    let mut dir: *mut dirent;
    let mut heaps: c_int = 0;

    loop {
        dir = readdir(d);
        if dir.is_null() {
            break;
        }
        if strncmp((*dir).d_name.as_ptr(), c".".as_ptr(), 2) == 0 {
            continue;
        }
        if strncmp((*dir).d_name.as_ptr(), c"..".as_ptr(), 3) == 0 {
            continue;
        }
        heaps += 1;
    }

    heaps
}

pub unsafe fn main() -> c_int {
    let mut dir: *mut dirent;
    let d: *mut DIR;

    ksft_print_header();

    d = opendir(DEVPATH.as_ptr() as *const c_char);
    if d.is_null() {
        ksft_print_msg(c"No %s directory?\n".as_ptr(), DEVPATH.as_ptr() as *const c_char);
        return KSFT_SKIP;
    }

    ksft_set_plan(11 * numer_of_heaps());

    loop {
        dir = readdir(d);
        if dir.is_null() {
            break;
        }
        if strncmp((*dir).d_name.as_ptr(), c".".as_ptr(), 2) == 0 {
            continue;
        }
        if strncmp((*dir).d_name.as_ptr(), c"..".as_ptr(), 3) == 0 {
            continue;
        }

        ksft_print_msg(c"Testing heap: %s\n".as_ptr(), (*dir).d_name.as_ptr());
        ksft_print_msg(c"=======================================\n".as_ptr());
        test_alloc_and_import((*dir).d_name.as_mut_ptr());
        test_alloc_zeroed((*dir).d_name.as_mut_ptr(), 4 * 1024);
        test_alloc_zeroed((*dir).d_name.as_mut_ptr(), ONE_MEG);
        test_alloc_compat((*dir).d_name.as_mut_ptr());
        test_alloc_errors((*dir).d_name.as_mut_ptr());
    }
    closedir(d);

    ksft_finished();
    0
}

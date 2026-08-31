// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and __EXPORTED_HEADERS__ before including
// libc, Linux udmabuf, and kselftest headers.

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::mem;
use std::ptr;

type off64_t = i64;
type size_t = usize;

const TEST_PREFIX: &[u8] = b"drivers/dma-buf/udmabuf\0";
const NUM_PAGES: c_int = 4;
const NUM_ENTRIES: c_int = 4;
const MEMFD_SIZE: off64_t = 1024; /* in pages */

const MFD_ALLOW_SEALING: c_uint = 0x0002;
const MFD_HUGETLB: c_uint = 0x0004;
const F_ADD_SEALS: c_int = 1033;
const F_SEAL_SHRINK: c_int = 0x0002;
const O_RDWR: c_int = 0o2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

// ioctl request values are supplied by <linux/udmabuf.h> in the original C
// translation unit.
const UDMABUF_CREATE: c_ulong = 0;
const UDMABUF_CREATE_LIST: c_ulong = 0;
const UDMABUF_FLAGS_CLOEXEC: u32 = 0x01;

#[repr(C)]
struct udmabuf_create {
    memfd: u32,
    flags: u32,
    offset: u64,
    size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct udmabuf_create_item {
    memfd: u32,
    __pad: u32,
    offset: u64,
    size: u64,
}

#[repr(C)]
struct udmabuf_create_list {
    count: u32,
    flags: u32,
    list: [udmabuf_create_item; 0],
}

static mut page_size: c_uint = 0;

unsafe extern "C" {
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn ftruncate(fd: c_int, length: off64_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn getpagesize() -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off64_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_cnts();
}

unsafe fn create_memfd_with_seals(size: off64_t, hpage: bool) -> c_int {
    let memfd: c_int;
    let mut ret: c_int;
    let mut flags: c_uint = MFD_ALLOW_SEALING;

    if hpage {
        flags |= MFD_HUGETLB;
    }

    memfd = memfd_create(c"udmabuf-test".as_ptr(), flags);
    if memfd < 0 {
        ksft_print_msg(c"%s: [skip,no-memfd]\n".as_ptr(), TEST_PREFIX.as_ptr());
        exit(KSFT_SKIP);
    }

    ret = fcntl(memfd, F_ADD_SEALS, F_SEAL_SHRINK);
    if ret < 0 {
        ksft_print_msg(
            c"%s: [skip,fcntl-add-seals]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_SKIP);
    }

    ret = ftruncate(memfd, size);
    if ret == -1 {
        ksft_print_msg(
            c"%s: [FAIL,memfd-truncate]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_FAIL);
    }

    memfd
}

unsafe fn create_udmabuf_list(devfd: c_int, memfd: c_int, memfd_size: off64_t) -> c_int {
    let list: *mut udmabuf_create_list;
    let ubuf_fd: c_int;
    let mut i: c_int;

    list = malloc(
        mem::size_of::<udmabuf_create_list>()
            + mem::size_of::<udmabuf_create_item>() * NUM_ENTRIES as usize,
    ) as *mut udmabuf_create_list;
    if list.is_null() {
        ksft_print_msg(
            c"%s: [FAIL, udmabuf-malloc]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_FAIL);
    }

    i = 0;
    while i < NUM_ENTRIES {
        let item = ((*list).list.as_mut_ptr()).add(i as usize);
        (*item).memfd = memfd as u32;
        (*item).offset = (i as off64_t * (memfd_size / NUM_ENTRIES as off64_t)) as u64;
        (*item).size = (getpagesize() * NUM_PAGES) as u64;
        i += 1;
    }

    (*list).count = NUM_ENTRIES as u32;
    (*list).flags = UDMABUF_FLAGS_CLOEXEC;
    ubuf_fd = ioctl(devfd, UDMABUF_CREATE_LIST, list);
    free(list as *mut c_void);
    if ubuf_fd < 0 {
        ksft_print_msg(c"%s: [FAIL, udmabuf-create]\n".as_ptr(), TEST_PREFIX.as_ptr());
        exit(KSFT_FAIL);
    }

    ubuf_fd
}

unsafe fn write_to_memfd(addr: *mut c_void, size: off64_t, chr: c_char) {
    let mut i: c_int;

    i = 0;
    while (i as off64_t) < size / page_size as off64_t {
        *((addr as *mut c_char).add(i as usize * page_size as usize)) = chr;
        i += 1;
    }
}

unsafe fn mmap_fd(fd: c_int, size: off64_t) -> *mut c_void {
    let addr: *mut c_void;

    addr = mmap(
        ptr::null_mut(),
        size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0,
    );
    if addr == (-1isize) as *mut c_void {
        ksft_print_msg(c"%s: ubuf_fd mmap fail\n".as_ptr(), TEST_PREFIX.as_ptr());
        exit(KSFT_FAIL);
    }

    addr
}

unsafe fn compare_chunks(addr1: *mut c_void, addr2: *mut c_void, memfd_size: off64_t) -> c_int {
    let mut off: off64_t;
    let mut i: c_int = 0;
    let mut j: c_int;
    let mut k: c_int = 0;
    let mut ret: c_int = 0;
    let char1: c_char;
    let char2: c_char;

    'err: loop {
        while i < NUM_ENTRIES {
            off = i as off64_t * (memfd_size / NUM_ENTRIES as off64_t);
            j = 0;
            while j < NUM_PAGES {
                char1 = *((addr1 as *mut c_char)
                    .offset(off as isize + (j * getpagesize()) as isize));
                char2 = *((addr2 as *mut c_char).offset((k * getpagesize()) as isize));
                if char1 != char2 {
                    ret = -1;
                    break 'err;
                }
                j += 1;
                k += 1;
            }
            i += 1;
        }
        break;
    }

    munmap(addr1, memfd_size as size_t);
    munmap(
        addr2,
        (NUM_ENTRIES * NUM_PAGES * getpagesize()) as size_t,
    );
    ret
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut create: udmabuf_create = mem::zeroed();
    let mut devfd: c_int;
    let mut memfd: c_int;
    let mut buf: c_int;
    let mut ret: c_int;
    let mut size: off64_t;
    let mut addr1: *mut c_void;
    let mut addr2: *mut c_void;

    ksft_print_header();
    ksft_set_plan(7);

    devfd = open(c"/dev/udmabuf".as_ptr(), O_RDWR);
    if devfd < 0 {
        ksft_print_msg(
            c"%s: [skip,no-udmabuf: Unable to access DMA buffer device file]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_SKIP);
    }

    memfd = memfd_create(c"udmabuf-test".as_ptr(), MFD_ALLOW_SEALING);
    if memfd < 0 {
        ksft_print_msg(c"%s: [skip,no-memfd]\n".as_ptr(), TEST_PREFIX.as_ptr());
        exit(KSFT_SKIP);
    }

    ret = fcntl(memfd, F_ADD_SEALS, F_SEAL_SHRINK);
    if ret < 0 {
        ksft_print_msg(
            c"%s: [skip,fcntl-add-seals]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_SKIP);
    }

    size = getpagesize() as off64_t * NUM_PAGES as off64_t;
    ret = ftruncate(memfd, size);
    if ret == -1 {
        ksft_print_msg(
            c"%s: [FAIL,memfd-truncate]\n".as_ptr(),
            TEST_PREFIX.as_ptr(),
        );
        exit(KSFT_FAIL);
    }

    memset(
        &mut create as *mut udmabuf_create as *mut c_void,
        0,
        mem::size_of_val(&create),
    );

    /* should fail (offset not page aligned) */
    create.memfd = memfd as u32;
    create.offset = (getpagesize() / 2) as u64;
    create.size = getpagesize() as u64;
    buf = ioctl(devfd, UDMABUF_CREATE, &mut create);
    if buf >= 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-1]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-1]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    /* should fail (size not multiple of page) */
    create.memfd = memfd as u32;
    create.offset = 0;
    create.size = (getpagesize() / 2) as u64;
    buf = ioctl(devfd, UDMABUF_CREATE, &mut create);
    if buf >= 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-2]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-2]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    /* should fail (not memfd) */
    create.memfd = 0; /* stdin */
    create.offset = 0;
    create.size = size as u64;
    buf = ioctl(devfd, UDMABUF_CREATE, &mut create);
    if buf >= 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-3]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-3]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    /* should work */
    page_size = getpagesize() as c_uint;
    addr1 = mmap_fd(memfd, size);
    write_to_memfd(addr1, size, b'a' as c_char);
    create.memfd = memfd as u32;
    create.offset = 0;
    create.size = size as u64;
    buf = ioctl(devfd, UDMABUF_CREATE, &mut create);
    if buf < 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-4]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-4]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    munmap(addr1, size as size_t);
    close(buf);
    close(memfd);

    /* should work (migration of 4k size pages)*/
    size = MEMFD_SIZE * page_size as off64_t;
    memfd = create_memfd_with_seals(size, false);
    addr1 = mmap_fd(memfd, size);
    write_to_memfd(addr1, size, b'a' as c_char);
    buf = create_udmabuf_list(devfd, memfd, size);
    addr2 = mmap_fd(
        buf,
        (NUM_PAGES * NUM_ENTRIES * getpagesize()) as off64_t,
    );
    write_to_memfd(addr1, size, b'b' as c_char);
    ret = compare_chunks(addr1, addr2, size);
    if ret < 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-5]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-5]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    close(buf);
    close(memfd);

    /* should work (migration of 2MB size huge pages)*/
    page_size = (getpagesize() * 512) as c_uint; /* 2 MB */
    size = MEMFD_SIZE * page_size as off64_t;
    memfd = create_memfd_with_seals(size, true);
    addr1 = mmap_fd(memfd, size);
    write_to_memfd(addr1, size, b'a' as c_char);
    buf = create_udmabuf_list(devfd, memfd, size);
    addr2 = mmap_fd(
        buf,
        (NUM_PAGES * NUM_ENTRIES * getpagesize()) as off64_t,
    );
    write_to_memfd(addr1, size, b'b' as c_char);
    ret = compare_chunks(addr1, addr2, size);
    if ret < 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-6]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-6]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    close(buf);
    close(memfd);

    /* same test as above but we pin first before writing to memfd */
    page_size = (getpagesize() * 512) as c_uint; /* 2 MB */
    size = MEMFD_SIZE * page_size as off64_t;
    memfd = create_memfd_with_seals(size, true);
    buf = create_udmabuf_list(devfd, memfd, size);
    addr2 = mmap_fd(
        buf,
        (NUM_PAGES * NUM_ENTRIES * getpagesize()) as off64_t,
    );
    addr1 = mmap_fd(memfd, size);
    write_to_memfd(addr1, size, b'a' as c_char);
    write_to_memfd(addr1, size, b'b' as c_char);
    ret = compare_chunks(addr1, addr2, size);
    if ret < 0 {
        ksft_test_result_fail(c"%s: [FAIL,test-7]\n".as_ptr(), TEST_PREFIX.as_ptr());
    } else {
        ksft_test_result_pass(c"%s: [PASS,test-7]\n".as_ptr(), TEST_PREFIX.as_ptr());
    }

    close(buf);
    close(memfd);
    close(devfd);

    ksft_print_msg(c"%s: ok\n".as_ptr(), TEST_PREFIX.as_ptr());
    ksft_print_cnts();

    0
}

fn main() {
    unsafe {
        std::process::exit(main_impl(0, ptr::null_mut()));
    }
}
